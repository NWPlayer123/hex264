use ::core::mem::size_of;
use core::ffi::{c_char, c_float, c_int, c_uint, c_ulonglong, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::assert_h::__assert_fail;
use crate::base_h::{
    x264_clip3, x264_free, x264_malloc, x264_scan8, x264_union16_t, x264_union32_t, x264_union64_t,
    CHROMA_420, CHROMA_422, CHROMA_444, SLICE_TYPE_B, SLICE_TYPE_I, SLICE_TYPE_P, X264_SCAN8_0,
};
use crate::bitstream_h::{bs_size_te, bs_size_ue};
use crate::common_h::{
    dctcoef, pixel, pixel4, udctcoef, x264_10_log, x264_t, FDEC_STRIDE, FENC_STRIDE, QP_MAX,
    QP_MAX_SPEC, X264_LOOKAHEAD_QP,
};
use crate::config_h::HAVE_INTERLACED;
use crate::encoder_macroblock_h::{
    x264_10_macroblock_encode, x264_10_macroblock_probe_skip, x264_10_predict_lossless_16x16,
    x264_10_predict_lossless_4x4, x264_10_predict_lossless_8x8, x264_10_predict_lossless_chroma,
    x264_mb_encode_i4x4, x264_mb_encode_i8x8,
};
use crate::frame_h::{
    x264_10_frame_cond_wait, x264_10_weight_scale_plane, x264_frame_t, PADH, PADV,
};
use crate::limits_h::INT_MAX;
use crate::macroblock_h::{
    block_idx_x, block_idx_xy_fdec, block_idx_xy_fenc, block_idx_y, x264_10_mb_mc,
    x264_10_mb_predict_mv, x264_10_mb_predict_mv_16x16, x264_10_mb_predict_mv_direct16x16,
    x264_10_mb_predict_mv_ref16x16, x264_mb_partition_listX_table, x264_mb_predict_intra4x4_mode,
    x264_mb_transform_8x8_allowed, x264_transform_allowed, B_8x8, D_16x16, D_16x8, D_8x16, D_8x8,
    D_BI_8x8, D_DIRECT_8x8, D_L0_4x4, D_L0_4x8, D_L0_8x4, D_L0_8x8, D_L1_8x8, I_16x16, I_4x4,
    I_8x8, P_8x8, ALL_NEIGHBORS, B_BI_BI, B_DIRECT, B_L0_L0, B_L1_L1, B_SKIP, I_PCM, MB_LEFT,
    MB_TOP, MB_TOPLEFT, MB_TOPRIGHT, P_L0, P_SKIP,
};
use crate::mathcalls_h::log2f;
use crate::mc_h::x264_weight_t;
use crate::me_h::{
    x264_10_me_refine_bidir_rd, x264_10_me_refine_bidir_satd, x264_10_me_refine_qpel,
    x264_10_me_refine_qpel_rd, x264_10_me_refine_qpel_refdupe, x264_10_me_search_ref, x264_me_t,
    COST_MAX, COST_MAX64,
};
use crate::pixel_h::{
    x264_pixel_cmp_t, PIXEL_16x16, PIXEL_16x8, PIXEL_4x4, PIXEL_4x8, PIXEL_8x16, PIXEL_8x4,
    PIXEL_8x8,
};
use crate::predict_h::{
    x264_mb_chroma_pred_mode_fix, x264_mb_pred_mode16x16_fix, x264_mb_pred_mode4x4_fix,
    I_PRED_16x16_DC, I_PRED_16x16_DC_128, I_PRED_16x16_DC_LEFT, I_PRED_16x16_DC_TOP,
    I_PRED_16x16_H, I_PRED_16x16_P, I_PRED_16x16_V, I_PRED_4x4_DC, I_PRED_4x4_DC_128,
    I_PRED_4x4_DC_LEFT, I_PRED_4x4_DC_TOP, I_PRED_4x4_DDL, I_PRED_4x4_DDR, I_PRED_4x4_H,
    I_PRED_4x4_HD, I_PRED_4x4_HU, I_PRED_4x4_V, I_PRED_4x4_VL, I_PRED_4x4_VR, I_PRED_CHROMA_DC,
    I_PRED_CHROMA_DC_128, I_PRED_CHROMA_DC_LEFT, I_PRED_CHROMA_DC_TOP, I_PRED_CHROMA_H,
    I_PRED_CHROMA_P, I_PRED_CHROMA_V,
};
use crate::ratecontrol_h::x264_10_ratecontrol_mb_qp;
use crate::rdo_c::{
    rd_cost_chroma, rd_cost_i4x4, rd_cost_i8x8, rd_cost_mb, ssd_mb, x264_10_rd_cost_part,
};
use crate::rectangle_h::{
    x264_macroblock_cache_intra8x8_pred, x264_macroblock_cache_mv, x264_macroblock_cache_mvd,
    x264_macroblock_cache_ref, x264_macroblock_cache_skip,
};
use crate::stdint_h::intptr_t;
use crate::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::stdlib_h::abs;
use crate::tables_h::{
    x264_chroma_lambda2_offset_tab, x264_lambda2_tab, x264_lambda_tab, x264_trellis_lambda2_tab,
    x264_zero,
};
use crate::util_h::{x264_union128_sse_t, M128_ZERO};
use crate::x264_h::{
    X264_ANALYSE_BSUB16x16, X264_ANALYSE_I4x4, X264_ANALYSE_I8x8, X264_ANALYSE_PSUB16x16,
    X264_ANALYSE_PSUB8x8, X264_LOG_DEBUG, X264_LOG_ERROR, X264_LOG_WARNING, X264_MBINFO_CONSTANT,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "56:9"]
pub struct x264_mb_analysis_t {
    pub i_lambda: c_int,
    pub i_lambda2: c_int,
    pub i_qp: c_int,
    pub p_cost_mv: *mut uint16_t,
    pub p_cost_ref: [*mut uint16_t; 2],
    pub i_mbrd: c_int,
    pub b_fast_intra: c_int,
    pub b_force_intra: c_int,
    pub b_avoid_topright: c_int,
    pub b_try_skip: c_int,
    pub i_satd_i16x16: c_int,
    pub i_satd_i16x16_dir: [c_int; 7],
    pub i_predict16x16: c_int,
    pub i_satd_i8x8: c_int,
    pub i_cbp_i8x8_luma: c_int,
    pub i_satd_i8x8_dir: [[uint16_t; 16]; 4],
    pub i_predict8x8: [c_int; 4],
    pub i_satd_i4x4: c_int,
    pub i_predict4x4: [c_int; 16],
    pub i_satd_pcm: c_int,
    pub i_satd_chroma: c_int,
    pub i_satd_chroma_dir: [c_int; 7],
    pub i_predict8x8chroma: c_int,
    pub l0: x264_mb_analysis_list_t,
    pub l1: x264_mb_analysis_list_t,
    pub i_cost16x16bi: c_int,
    pub i_cost16x16direct: c_int,
    pub i_cost8x8bi: c_int,
    pub i_cost8x8direct: [c_int; 4],
    pub i_satd8x8: [[c_int; 4]; 3],
    pub i_cost_est16x8: [c_int; 2],
    pub i_cost_est8x16: [c_int; 2],
    pub i_cost16x8bi: c_int,
    pub i_cost8x16bi: c_int,
    pub i_rd16x16bi: c_int,
    pub i_rd16x16direct: c_int,
    pub i_rd16x8bi: c_int,
    pub i_rd8x16bi: c_int,
    pub i_rd8x8bi: c_int,
    pub i_mb_partition16x8: [c_int; 2],
    pub i_mb_partition8x16: [c_int; 2],
    pub i_mb_type16x8: c_int,
    pub i_mb_type8x16: c_int,
    pub b_direct_available: c_int,
    pub b_early_terminate: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "35:9"]
pub struct x264_mb_analysis_list_t {
    pub me16x16: x264_me_t,
    pub bi16x16: x264_me_t,
    pub me8x8: [x264_me_t; 4],
    pub me4x4: [[x264_me_t; 4]; 4],
    pub me8x4: [[x264_me_t; 2]; 4],
    pub me4x8: [[x264_me_t; 2]; 4],
    pub me16x8: [x264_me_t; 2],
    pub me8x16: [x264_me_t; 2],
    pub i_rd16x16: c_int,
    pub i_cost8x8: c_int,
    pub i_cost4x4: [c_int; 4],
    pub i_cost8x4: [c_int; 4],
    pub i_cost4x8: [c_int; 4],
    pub i_cost16x8: c_int,
    pub i_cost8x16: c_int,
    pub mvc: [[[int16_t; 2]; 6]; 32],
}
#[c2rust::src_loc = "124:22"]
static mut i_mb_b_cost_table: [uint8_t; 19] = [
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    1 as c_int as uint8_t,
    3 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    3 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    5 as c_int as uint8_t,
    9 as c_int as uint8_t,
    0 as c_int as uint8_t,
];
#[c2rust::src_loc = "128:22"]
static mut i_mb_b16x8_cost_table: [uint8_t; 17] = [
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    5 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    5 as c_int as uint8_t,
    7 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
];
#[c2rust::src_loc = "132:22"]
static mut i_sub_mb_b_cost_table: [uint8_t; 13] = [
    7 as c_int as uint8_t,
    5 as c_int as uint8_t,
    5 as c_int as uint8_t,
    3 as c_int as uint8_t,
    7 as c_int as uint8_t,
    5 as c_int as uint8_t,
    7 as c_int as uint8_t,
    3 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    5 as c_int as uint8_t,
    1 as c_int as uint8_t,
];
#[c2rust::src_loc = "136:22"]
static mut i_sub_mb_p_cost_table: [uint8_t; 4] = [
    5 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    1 as c_int as uint8_t,
];
#[c2rust::src_loc = "143:1"]
unsafe extern "C" fn init_costs(
    mut h: *mut x264_t,
    mut logs: *mut c_float,
    mut qp: c_int,
) -> c_int {
    let mut cost_i4x4_mode: *mut uint16_t = 0 as *mut uint16_t;
    let mut current_block: u64;
    if !(*h).cost_mv[qp as usize].is_null() {
        return 0 as c_int;
    }
    let mut mv_range: c_int = (*h).param.analyse.i_mv_range << (*h).param.b_interlaced;
    let mut lambda: c_int = x264_lambda_tab[qp as usize] as c_int;
    (*h).cost_mv[qp as usize] = x264_malloc(
        ((4 as c_int * 4 as c_int * mv_range + 1 as c_int) as usize)
            .wrapping_mul(size_of::<uint16_t>() as usize) as int64_t,
    ) as *mut uint16_t;
    if !(*h).cost_mv[qp as usize].is_null() {
        (*h).cost_mv[qp as usize] =
            (*h).cost_mv[qp as usize].offset((2 as c_int * 4 as c_int * mv_range) as isize);
        let mut i: c_int = 0 as c_int;
        while i <= 2 as c_int * 4 as c_int * mv_range {
            let ref mut fresh2 = *(*h).cost_mv[qp as usize].offset(i as isize);
            *fresh2 = (if ((lambda as c_float * *logs.offset(i as isize) + 0.5f32) as c_int)
                < 65535 as c_int
            {
                (lambda as c_float * *logs.offset(i as isize) + 0.5f32) as c_int
            } else {
                65535 as c_int
            }) as uint16_t;
            *(*h).cost_mv[qp as usize].offset(-i as isize) = *fresh2;
            i += 1;
        }
        let mut i_0: c_int = 0 as c_int;
        while i_0 < 3 as c_int {
            let mut j: c_int = 0 as c_int;
            while j < 33 as c_int {
                (*(*h).cost_table).ref_0[qp as usize][i_0 as usize][j as usize] = (if i_0 != 0 {
                    if lambda * bs_size_te(i_0, j) < 65535 as c_int {
                        lambda * bs_size_te(i_0, j)
                    } else {
                        65535 as c_int
                    }
                } else {
                    0 as c_int
                })
                    as uint16_t;
                j += 1;
            }
            i_0 += 1;
        }
        if (*h).param.analyse.me_method.exhaustive_search()
            && (*h).cost_mv_fpel[qp as usize][0].is_null()
        {
            let mut j_0: c_int = 0 as c_int;
            loop {
                if !(j_0 < 4 as c_int) {
                    current_block = 11194104282611034094;
                    break;
                }
                (*h).cost_mv_fpel[qp as usize][j_0 as usize] = x264_malloc(
                    ((4 as c_int * mv_range + 1 as c_int) as usize)
                        .wrapping_mul(size_of::<uint16_t>() as usize)
                        as int64_t,
                ) as *mut uint16_t;
                if (*h).cost_mv_fpel[qp as usize][j_0 as usize].is_null() {
                    current_block = 7965308608183717162;
                    break;
                }
                (*h).cost_mv_fpel[qp as usize][j_0 as usize] = (*h).cost_mv_fpel[qp as usize]
                    [j_0 as usize]
                    .offset((2 as c_int * mv_range) as isize);
                let mut i_1: c_int = -(2 as c_int) * mv_range;
                while i_1 < 2 as c_int * mv_range {
                    *(*h).cost_mv_fpel[qp as usize][j_0 as usize].offset(i_1 as isize) =
                        *(*h).cost_mv[qp as usize].offset((i_1 * 4 as c_int + j_0) as isize);
                    i_1 += 1;
                }
                j_0 += 1;
            }
        } else {
            current_block = 11194104282611034094;
        }
        match current_block {
            7965308608183717162 => {}
            _ => {
                cost_i4x4_mode = (*(*(*h).cost_table)
                    .i4x4_mode
                    .as_mut_ptr()
                    .offset(qp as isize))
                .as_mut_ptr();
                let mut i_2: c_int = 0 as c_int;
                while i_2 < 17 as c_int {
                    *cost_i4x4_mode.offset(i_2 as isize) =
                        (3 as c_int * lambda * (i_2 != 8 as c_int) as c_int) as uint16_t;
                    i_2 += 1;
                }
                return 0 as c_int;
            }
        }
    }
    return -1;
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn x264_10_analyse_init_costs(mut h: *mut x264_t) -> c_int {
    let mut current_block: u64;
    let mut mv_range: c_int = (*h).param.analyse.i_mv_range << (*h).param.b_interlaced;
    let mut logs: *mut c_float = x264_malloc(
        ((2 as c_int * 4 as c_int * mv_range + 1 as c_int) as usize)
            .wrapping_mul(size_of::<c_float>() as usize) as int64_t,
    ) as *mut c_float;
    if logs.is_null() {
        return -1;
    }
    *logs.offset(0) = 0.718f32;
    let mut i: c_int = 1 as c_int;
    while i <= 2 as c_int * 4 as c_int * mv_range {
        *logs.offset(i as isize) = log2f((i + 1 as c_int) as c_float) * 2.0f32 + 1.718f32;
        i += 1;
    }
    let mut qp: c_int =
        if (*h).param.rc.i_qp_min < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) {
            (*h).param.rc.i_qp_min
        } else {
            51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
        };
    loop {
        if !(qp <= (*h).param.rc.i_qp_max) {
            current_block = 10886091980245723256;
            break;
        }
        if init_costs(h, logs, qp) != 0 {
            current_block = 15663036285462993308;
            break;
        }
        qp += 1;
    }
    match current_block {
        10886091980245723256 => {
            if !(init_costs(h, logs, X264_LOOKAHEAD_QP) != 0) {
                x264_free(logs as *mut c_void);
                return 0 as c_int;
            }
        }
        _ => {}
    }
    x264_free(logs as *mut c_void);
    return -1;
}
#[no_mangle]
#[c2rust::src_loc = "204:1"]
pub unsafe extern "C" fn x264_10_analyse_free_costs(mut h: *mut x264_t) {
    let mut mv_range: c_int = (*h).param.analyse.i_mv_range << (*h).param.b_interlaced;
    let mut i: c_int = 0 as c_int;
    while i < QP_MAX + 1 as c_int {
        if !(*h).cost_mv[i as usize].is_null() {
            x264_free(
                (*h).cost_mv[i as usize].offset(-((2 as c_int * 4 as c_int * mv_range) as isize))
                    as *mut c_void,
            );
        }
        let mut j: c_int = 0 as c_int;
        while j < 4 as c_int {
            if !(*h).cost_mv_fpel[i as usize][j as usize].is_null() {
                x264_free(
                    (*h).cost_mv_fpel[i as usize][j as usize]
                        .offset(-((2 as c_int * mv_range) as isize))
                        as *mut c_void,
                );
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "219:1"]
pub unsafe extern "C" fn x264_10_analyse_weight_frame(mut h: *mut x264_t, mut end: c_int) {
    let mut j: c_int = 0 as c_int;
    while j < (*h).i_ref[0] {
        if !(*h).sh.weight[j as usize][0].weightfn.is_null() {
            let mut frame: *mut x264_frame_t = (*h).fref[0][j as usize];
            let mut width: c_int = (*frame).i_width[0]
                + ((if 32 as c_int > 64 as c_int / size_of::<pixel>() as c_int {
                    32 as c_int
                } else {
                    64 as c_int / size_of::<pixel>() as c_int
                }) + PADH);
            let mut i_padv: c_int = PADV << (*h).param.b_interlaced;
            let mut offset: c_int = 0;
            let mut height: c_int = 0;
            let mut src: *mut pixel = (*frame).filtered[0][0]
                .offset(-(((*frame).i_stride[0] * i_padv) as isize))
                .offset(
                    -((if 32 as c_int > 64 as c_int / size_of::<pixel>() as c_int {
                        32 as c_int
                    } else {
                        64 as c_int / size_of::<pixel>() as c_int
                    }) as isize),
                );
            height = (if 16 as c_int + end + i_padv
                < (*(*h).fref[0][j as usize]).i_lines[0] + i_padv * 2 as c_int
            {
                16 as c_int + end + i_padv
            } else {
                (*(*h).fref[0][j as usize]).i_lines[0] + i_padv * 2 as c_int
            }) - (*(*h).fenc).i_lines_weighted;
            offset = (*(*h).fenc).i_lines_weighted * (*frame).i_stride[0];
            (*(*h).fenc).i_lines_weighted += height;
            if height != 0 {
                let mut k: c_int = j;
                while k < (*h).i_ref[0] {
                    if !(*h).sh.weight[k as usize][0].weightfn.is_null() {
                        let mut dst: *mut pixel = (*(*h).fenc).weighted[k as usize]
                            .offset(-(((*(*h).fenc).i_stride[0] * i_padv) as isize))
                            .offset(
                                -((if 32 as c_int > 64 as c_int / size_of::<pixel>() as c_int {
                                    32 as c_int
                                } else {
                                    64 as c_int / size_of::<pixel>() as c_int
                                }) as isize),
                            );
                        x264_10_weight_scale_plane(
                            h,
                            dst.offset(offset as isize),
                            (*frame).i_stride[0] as intptr_t,
                            src.offset(offset as isize),
                            (*frame).i_stride[0] as intptr_t,
                            width,
                            height,
                            &mut *(*(*h).sh.weight.as_mut_ptr().offset(k as isize))
                                .as_mut_ptr()
                                .offset(0),
                        );
                    }
                    k += 1;
                }
            }
            break;
        } else {
            j += 1;
        }
    }
}
#[c2rust::src_loc = "248:1"]
pub unsafe extern "C" fn mb_analyse_load_costs(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    (*a).p_cost_mv = (*h).cost_mv[(*a).i_qp as usize];
    (*a).p_cost_ref[0] = (*(*(*(*h).cost_table)
        .ref_0
        .as_mut_ptr()
        .offset((*a).i_qp as isize))
    .as_mut_ptr()
    .offset(
        (x264_clip3 as unsafe extern "C" fn(c_int, c_int, c_int) -> c_int)(
            (*h).sh.i_num_ref_idx_l0_active - 1 as c_int,
            0 as c_int,
            2 as c_int,
        ) as isize,
    ))
    .as_mut_ptr();
    (*a).p_cost_ref[1] = (*(*(*(*h).cost_table)
        .ref_0
        .as_mut_ptr()
        .offset((*a).i_qp as isize))
    .as_mut_ptr()
    .offset(
        (x264_clip3 as unsafe extern "C" fn(c_int, c_int, c_int) -> c_int)(
            (*h).sh.i_num_ref_idx_l1_active - 1 as c_int,
            0 as c_int,
            2 as c_int,
        ) as isize,
    ))
    .as_mut_ptr();
}
#[c2rust::src_loc = "255:1"]
unsafe extern "C" fn mb_analyse_init_qp(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut qp: c_int,
) {
    let mut effective_chroma_qp: c_int = *(*h).chroma_qp_table.offset(
        (if qp < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) {
            qp
        } else {
            51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
        }) as isize,
    ) as c_int
        + (if qp - (51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)) > 0 as c_int {
            qp - (51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int))
        } else {
            0 as c_int
        });
    (*a).i_lambda = x264_lambda_tab[qp as usize] as c_int;
    (*a).i_lambda2 = x264_lambda2_tab[qp as usize];
    (*h).mb.b_trellis = ((*h).param.analyse.i_trellis > 1 as c_int && (*a).i_mbrd != 0) as c_int;
    if (*h).param.analyse.i_trellis != 0 {
        (*h).mb.i_trellis_lambda2[0][0] = x264_trellis_lambda2_tab[0][qp as usize];
        (*h).mb.i_trellis_lambda2[0][1] = x264_trellis_lambda2_tab[1][qp as usize];
        (*h).mb.i_trellis_lambda2[1][0] = x264_trellis_lambda2_tab[0][effective_chroma_qp as usize];
        (*h).mb.i_trellis_lambda2[1][1] = x264_trellis_lambda2_tab[1][effective_chroma_qp as usize];
    }
    (*h).mb.i_psy_rd_lambda = (*a).i_lambda;
    let mut chroma_offset_idx: c_int = if (qp - effective_chroma_qp + 12 as c_int) < 36 as c_int {
        qp - effective_chroma_qp + 12 as c_int
    } else {
        36 as c_int
    };
    (*h).mb.i_chroma_lambda2_offset = if (*h).param.analyse.b_psy != 0 {
        x264_chroma_lambda2_offset_tab[chroma_offset_idx as usize] as c_int
    } else {
        256 as c_int
    };
    if qp > QP_MAX_SPEC {
        (*h).nr_offset = (*(*h)
            .nr_offset_emergency
            .offset((qp - QP_MAX_SPEC - 1 as c_int) as isize))
        .as_mut_ptr() as *mut [udctcoef; 64];
        (*h).nr_residual_sum =
            (*(*h).nr_residual_sum_buf.as_mut_ptr().offset(1)).as_mut_ptr() as *mut [uint32_t; 64];
        (*h).nr_count = (*(*h).nr_count_buf.as_mut_ptr().offset(1)).as_mut_ptr();
        (*h).mb.b_noise_reduction = 1 as c_int;
        qp = QP_MAX_SPEC;
    } else {
        (*h).nr_offset = (*h).nr_offset_denoise.as_mut_ptr() as *mut [udctcoef; 64];
        (*h).nr_residual_sum =
            (*(*h).nr_residual_sum_buf.as_mut_ptr().offset(0)).as_mut_ptr() as *mut [uint32_t; 64];
        (*h).nr_count = (*(*h).nr_count_buf.as_mut_ptr().offset(0)).as_mut_ptr();
        (*h).mb.b_noise_reduction = 0 as c_int;
    }
    (*h).mb.i_qp = qp;
    (*a).i_qp = (*h).mb.i_qp;
    (*h).mb.i_chroma_qp = *(*h).chroma_qp_table.offset(qp as isize) as c_int;
}
#[c2rust::src_loc = "294:1"]
unsafe extern "C" fn mb_analyse_init(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut qp: c_int,
) {
    let mut subme: c_int =
        (*h).param.analyse.i_subpel_refine - ((*h).sh.i_type == SLICE_TYPE_B as c_int) as c_int;
    (*a).i_mbrd = (subme >= 6 as c_int) as c_int
        + (subme >= 8 as c_int) as c_int
        + ((*h).param.analyse.i_subpel_refine >= 10 as c_int) as c_int;
    (*h).mb.b_deblock_rdo = ((*h).param.analyse.i_subpel_refine >= 9 as c_int
        && (*h).sh.i_disable_deblocking_filter_idc != 1 as c_int)
        as c_int;
    (*a).b_early_terminate = ((*h).param.analyse.i_subpel_refine < 11 as c_int) as c_int;
    mb_analyse_init_qp(h, a, qp);
    (*h).mb.b_transform_8x8 = 0 as c_int;
    (*a).i_satd_i4x4 = COST_MAX;
    (*a).i_satd_i8x8 = (*a).i_satd_i4x4;
    (*a).i_satd_i16x16 = (*a).i_satd_i8x8;
    (*a).i_satd_chroma = if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        COST_MAX
    } else {
        0 as c_int
    };
    let mut pcm_cost: uint64_t = ((256 as c_int * 10 as c_int
        + 2 as c_int
            * (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                256 as c_int * 10 as c_int >> (*h).mb.chroma_h_shift + (*h).mb.chroma_v_shift
            } else {
                0 as c_int
            })
        + 16 as c_int) as uint64_t)
        .wrapping_mul((*a).i_lambda2 as uint64_t)
        .wrapping_add(128 as uint64_t)
        >> 8 as c_int;
    (*a).i_satd_pcm = (if (*h).param.i_avcintra_class == 0
        && (*h).mb.i_psy_rd == 0
        && (*a).i_mbrd != 0
        && pcm_cost < COST_MAX as uint64_t
    {
        pcm_cost
    } else {
        COST_MAX as uint64_t
    }) as c_int;
    (*a).b_fast_intra = 0 as c_int;
    (*a).b_avoid_topright = 0 as c_int;
    (*h).mb.i_skip_intra = if (*h).mb.b_lossless != 0 {
        0 as c_int
    } else if (*a).i_mbrd != 0 {
        2 as c_int
    } else {
        ((*h).param.analyse.i_trellis == 0 && (*h).param.analyse.i_noise_reduction == 0) as c_int
    };
    if (*h).sh.i_type != SLICE_TYPE_I as c_int {
        let mut i_fmv_range: c_int = 4 as c_int * (*h).param.analyse.i_mv_range;
        let mut i_fpel_border: c_int = 6 as c_int;
        (*h).mb.mv_min[0] = 4 as c_int * (-(16 as c_int) * (*h).mb.i_mb_x - 24 as c_int);
        (*h).mb.mv_max[0] = 4 as c_int
            * (16 as c_int * ((*h).mb.i_mb_width - (*h).mb.i_mb_x - 1 as c_int) + 24 as c_int);
        (*h).mb.mv_min_spel[0] = if (*h).mb.mv_min[0] > -i_fmv_range {
            (*h).mb.mv_min[0]
        } else {
            -i_fmv_range
        };
        (*h).mb.mv_max_spel[0] = if (*h).mb.mv_max[0] < i_fmv_range - 1 as c_int {
            (*h).mb.mv_max[0]
        } else {
            i_fmv_range - 1 as c_int
        };
        if (*h).param.b_intra_refresh != 0 && (*h).sh.i_type == SLICE_TYPE_P as c_int {
            let mut max_x: c_int =
                ((*(*h).fref[0][0]).i_pir_end_col * 16 as c_int - 3 as c_int) * 4 as c_int;
            let mut max_mv: c_int = max_x - 4 as c_int * 16 as c_int * (*h).mb.i_mb_x;
            if max_mv > 0 as c_int && (*h).mb.i_mb_x < (*(*h).fdec).i_pir_start_col {
                (*h).mb.mv_max_spel[0] = if (*h).mb.mv_max_spel[0] < max_mv {
                    (*h).mb.mv_max_spel[0]
                } else {
                    max_mv
                };
            }
        }
        (*h).mb.mv_limit_fpel[0][0] =
            (((*h).mb.mv_min_spel[0] >> 2 as c_int) + i_fpel_border) as int16_t;
        (*h).mb.mv_limit_fpel[1][0] =
            (((*h).mb.mv_max_spel[0] >> 2 as c_int) - i_fpel_border) as int16_t;
        if (*h).mb.i_mb_x == 0 as c_int && (*h).mb.i_mb_y & (*h).param.b_interlaced == 0 {
            let mut mb_y: c_int = (*h).mb.i_mb_y >> (*h).sh.b_mbaff;
            let mut thread_mvy_range: c_int = i_fmv_range;
            if (*h).i_thread_frames > 1 as c_int {
                let mut pix_y: c_int = ((*h).mb.i_mb_y | (*h).param.b_interlaced) * 16 as c_int;
                let mut thresh: c_int = pix_y + (*h).param.analyse.i_mv_range_thread;
                let mut i: c_int = ((*h).sh.i_type == SLICE_TYPE_B as c_int) as c_int;
                while i >= 0 as c_int {
                    let mut j: c_int = 0 as c_int;
                    while j < (*h).i_ref[i as usize] {
                        let mut completed: c_int = x264_10_frame_cond_wait(
                            (*(*h).fref[i as usize][j as usize]).orig as *mut x264_frame_t,
                            thresh,
                        );
                        thread_mvy_range = if thread_mvy_range < completed - pix_y {
                            thread_mvy_range
                        } else {
                            completed - pix_y
                        };
                        j += 1;
                    }
                    i -= 1;
                }
                if (*h).param.b_deterministic != 0 {
                    thread_mvy_range = (*h).param.analyse.i_mv_range_thread;
                }
                if (*h).param.b_interlaced != 0 {
                    thread_mvy_range >>= 1 as c_int;
                }
                x264_10_analyse_weight_frame(h, pix_y + thread_mvy_range);
            }
            if (*h).param.b_interlaced != 0 {
                let mut i_0: c_int = 0 as c_int;
                while i_0 < 3 as c_int {
                    let mut j_0: c_int = (i_0 == 2 as c_int) as c_int;
                    mb_y = ((*h).mb.i_mb_y >> j_0) + (i_0 == 1 as c_int) as c_int;
                    (*h).mb.mv_miny_row[i_0 as usize] =
                        4 as c_int * (-(16 as c_int) * mb_y - 24 as c_int);
                    (*h).mb.mv_maxy_row[i_0 as usize] = 4 as c_int
                        * (16 as c_int * (((*h).mb.i_mb_height >> j_0) - mb_y - 1 as c_int)
                            + 24 as c_int);
                    (*h).mb.mv_miny_spel_row[i_0 as usize] =
                        if (*h).mb.mv_miny_row[i_0 as usize] > -i_fmv_range {
                            (*h).mb.mv_miny_row[i_0 as usize]
                        } else {
                            -i_fmv_range
                        };
                    (*h).mb.mv_maxy_spel_row[i_0 as usize] = if (*h).mb.mv_maxy_row[i_0 as usize]
                        < (if (i_fmv_range - 1 as c_int) < 4 as c_int * thread_mvy_range {
                            i_fmv_range - 1 as c_int
                        } else {
                            4 as c_int * thread_mvy_range
                        }) {
                        (*h).mb.mv_maxy_row[i_0 as usize]
                    } else if (i_fmv_range - 1 as c_int) < 4 as c_int * thread_mvy_range {
                        i_fmv_range - 1 as c_int
                    } else {
                        4 as c_int * thread_mvy_range
                    };
                    (*h).mb.mv_miny_fpel_row[i_0 as usize] =
                        ((*h).mb.mv_miny_spel_row[i_0 as usize] >> 2 as c_int) + i_fpel_border;
                    (*h).mb.mv_maxy_fpel_row[i_0 as usize] =
                        ((*h).mb.mv_maxy_spel_row[i_0 as usize] >> 2 as c_int) - i_fpel_border;
                    i_0 += 1;
                }
            } else {
                (*h).mb.mv_min[1] = 4 as c_int * (-(16 as c_int) * mb_y - 24 as c_int);
                (*h).mb.mv_max[1] = 4 as c_int
                    * (16 as c_int * ((*h).mb.i_mb_height - mb_y - 1 as c_int) + 24 as c_int);
                (*h).mb.mv_min_spel[1] = if (*h).mb.mv_min[1] > -i_fmv_range {
                    (*h).mb.mv_min[1]
                } else {
                    -i_fmv_range
                };
                (*h).mb.mv_max_spel[1] = if (*h).mb.mv_max[1]
                    < (if (i_fmv_range - 1 as c_int) < 4 as c_int * thread_mvy_range {
                        i_fmv_range - 1 as c_int
                    } else {
                        4 as c_int * thread_mvy_range
                    }) {
                    (*h).mb.mv_max[1]
                } else if (i_fmv_range - 1 as c_int) < 4 as c_int * thread_mvy_range {
                    i_fmv_range - 1 as c_int
                } else {
                    4 as c_int * thread_mvy_range
                };
                (*h).mb.mv_limit_fpel[0][1] =
                    (((*h).mb.mv_min_spel[1] >> 2 as c_int) + i_fpel_border) as int16_t;
                (*h).mb.mv_limit_fpel[1][1] =
                    (((*h).mb.mv_max_spel[1] >> 2 as c_int) - i_fpel_border) as int16_t;
            }
        }
        if (*h).param.b_interlaced != 0 {
            let mut i_1: c_int = if (*h).mb.b_interlaced != 0 {
                2 as c_int
            } else {
                (*h).mb.i_mb_y & 1 as c_int
            };
            (*h).mb.mv_min[1] = (*h).mb.mv_miny_row[i_1 as usize];
            (*h).mb.mv_max[1] = (*h).mb.mv_maxy_row[i_1 as usize];
            (*h).mb.mv_min_spel[1] = (*h).mb.mv_miny_spel_row[i_1 as usize];
            (*h).mb.mv_max_spel[1] = (*h).mb.mv_maxy_spel_row[i_1 as usize];
            (*h).mb.mv_limit_fpel[0][1] = (*h).mb.mv_miny_fpel_row[i_1 as usize] as int16_t;
            (*h).mb.mv_limit_fpel[1][1] = (*h).mb.mv_maxy_fpel_row[i_1 as usize] as int16_t;
        }
        (*a).l0.i_cost8x16 = COST_MAX;
        (*a).l0.i_cost16x8 = (*a).l0.i_cost8x16;
        (*a).l0.i_cost8x8 = (*a).l0.i_cost16x8;
        (*a).l0.i_rd16x16 = (*a).l0.i_cost8x8;
        (*a).l0.me16x16.cost = (*a).l0.i_rd16x16;
        if (*h).sh.i_type == SLICE_TYPE_B as c_int {
            (*a).i_cost8x16bi = COST_MAX;
            (*a).i_cost16x8bi = (*a).i_cost8x16bi;
            (*a).i_cost8x8bi = (*a).i_cost16x8bi;
            (*a).i_cost16x16direct = (*a).i_cost8x8bi;
            (*a).i_cost16x16bi = (*a).i_cost16x16direct;
            (*a).i_rd8x16bi = (*a).i_cost16x16bi;
            (*a).i_rd16x8bi = (*a).i_rd8x16bi;
            (*a).i_rd8x8bi = (*a).i_rd16x8bi;
            (*a).i_rd16x16direct = (*a).i_rd8x8bi;
            (*a).i_rd16x16bi = (*a).i_rd16x16direct;
            (*a).l1.i_cost8x16 = (*a).i_rd16x16bi;
            (*a).l1.i_cost16x8 = (*a).l1.i_cost8x16;
            (*a).i_cost8x8direct[3] = (*a).l1.i_cost16x8;
            (*a).i_cost8x8direct[2] = (*a).i_cost8x8direct[3];
            (*a).i_cost8x8direct[1] = (*a).i_cost8x8direct[2];
            (*a).i_cost8x8direct[0] = (*a).i_cost8x8direct[1];
            (*a).l1.i_cost8x8 = (*a).i_cost8x8direct[0];
            (*a).l1.i_rd16x16 = (*a).l1.i_cost8x8;
            (*a).l1.me16x16.cost = (*a).l1.i_rd16x16;
        } else if (*h).param.analyse.inter & X264_ANALYSE_PSUB8x8 != 0 {
            let mut i_2: c_int = 0 as c_int;
            while i_2 < 4 as c_int {
                (*a).l0.i_cost4x8[i_2 as usize] = COST_MAX;
                (*a).l0.i_cost8x4[i_2 as usize] = (*a).l0.i_cost4x8[i_2 as usize];
                (*a).l0.i_cost4x4[i_2 as usize] = (*a).l0.i_cost8x4[i_2 as usize];
                i_2 += 1;
            }
        }
        if (*a).b_early_terminate != 0 && (*h).mb.i_mb_xy - (*h).sh.i_first_mb > 4 as c_int {
            if !((*h).mb.i_mb_type_left[0] == I_4x4 as c_int
                || (*h).mb.i_mb_type_left[0] == I_8x8 as c_int
                || (*h).mb.i_mb_type_left[0] == I_16x16 as c_int
                || (*h).mb.i_mb_type_left[0] == I_PCM as c_int
                || ((*h).mb.i_mb_type_top == I_4x4 as c_int
                    || (*h).mb.i_mb_type_top == I_8x8 as c_int
                    || (*h).mb.i_mb_type_top == I_16x16 as c_int
                    || (*h).mb.i_mb_type_top == I_PCM as c_int)
                || ((*h).mb.i_mb_type_topleft == I_4x4 as c_int
                    || (*h).mb.i_mb_type_topleft == I_8x8 as c_int
                    || (*h).mb.i_mb_type_topleft == I_16x16 as c_int
                    || (*h).mb.i_mb_type_topleft == I_PCM as c_int)
                || ((*h).mb.i_mb_type_topright == I_4x4 as c_int
                    || (*h).mb.i_mb_type_topright == I_8x8 as c_int
                    || (*h).mb.i_mb_type_topright == I_16x16 as c_int
                    || (*h).mb.i_mb_type_topright == I_PCM as c_int)
                || (*h).sh.i_type == SLICE_TYPE_P as c_int
                    && (*(*(*h).fref[0][0]).mb_type.offset((*h).mb.i_mb_xy as isize) as c_int
                        == I_4x4 as c_int
                        || *(*(*h).fref[0][0]).mb_type.offset((*h).mb.i_mb_xy as isize) as c_int
                            == I_8x8 as c_int
                        || *(*(*h).fref[0][0]).mb_type.offset((*h).mb.i_mb_xy as isize) as c_int
                            == I_16x16 as c_int
                        || *(*(*h).fref[0][0]).mb_type.offset((*h).mb.i_mb_xy as isize) as c_int
                            == I_PCM as c_int)
                || (*h).mb.i_mb_xy - (*h).sh.i_first_mb
                    < 3 as c_int
                        * ((*h).stat.frame.i_mb_count[I_4x4 as c_int as usize]
                            + (*h).stat.frame.i_mb_count[I_8x8 as c_int as usize]
                            + (*h).stat.frame.i_mb_count[I_16x16 as c_int as usize]
                            + (*h).stat.frame.i_mb_count[I_PCM as c_int as usize]))
            {
                (*a).b_fast_intra = 1 as c_int;
            }
        }
        (*h).mb.b_skip_mc = 0 as c_int;
        if (*h).param.b_intra_refresh != 0
            && (*h).sh.i_type == SLICE_TYPE_P as c_int
            && (*h).mb.i_mb_x >= (*(*h).fdec).i_pir_start_col
            && (*h).mb.i_mb_x <= (*(*h).fdec).i_pir_end_col
        {
            (*a).b_force_intra = 1 as c_int;
            (*a).b_fast_intra = 0 as c_int;
            (*a).b_avoid_topright = ((*h).mb.i_mb_x == (*(*h).fdec).i_pir_end_col) as c_int;
        } else {
            (*a).b_force_intra = 0 as c_int;
        }
    }
}
#[c2rust::src_loc = "476:21"]
static mut i16x16_mode_available: [[int8_t; 5]; 5] = [
    [
        I_PRED_16x16_DC_128 as c_int as int8_t,
        -1 as int8_t,
        -1 as int8_t,
        -1 as int8_t,
        -1 as int8_t,
    ],
    [
        I_PRED_16x16_DC_LEFT as c_int as int8_t,
        I_PRED_16x16_H as c_int as int8_t,
        -1 as int8_t,
        -1 as int8_t,
        -1 as int8_t,
    ],
    [
        I_PRED_16x16_DC_TOP as c_int as int8_t,
        I_PRED_16x16_V as c_int as int8_t,
        -1 as int8_t,
        -1 as int8_t,
        -1 as int8_t,
    ],
    [
        I_PRED_16x16_V as c_int as int8_t,
        I_PRED_16x16_H as c_int as int8_t,
        I_PRED_16x16_DC as c_int as int8_t,
        -1 as int8_t,
        -1 as int8_t,
    ],
    [
        I_PRED_16x16_V as c_int as int8_t,
        I_PRED_16x16_H as c_int as int8_t,
        I_PRED_16x16_DC as c_int as int8_t,
        I_PRED_16x16_P as c_int as int8_t,
        -1 as int8_t,
    ],
];
#[c2rust::src_loc = "485:21"]
static mut chroma_mode_available: [[int8_t; 5]; 5] = [
    [
        I_PRED_CHROMA_DC_128 as c_int as int8_t,
        -1 as int8_t,
        -1 as int8_t,
        -1 as int8_t,
        -1 as int8_t,
    ],
    [
        I_PRED_CHROMA_DC_LEFT as c_int as int8_t,
        I_PRED_CHROMA_H as c_int as int8_t,
        -1 as int8_t,
        -1 as int8_t,
        -1 as int8_t,
    ],
    [
        I_PRED_CHROMA_DC_TOP as c_int as int8_t,
        I_PRED_CHROMA_V as c_int as int8_t,
        -1 as int8_t,
        -1 as int8_t,
        -1 as int8_t,
    ],
    [
        I_PRED_CHROMA_V as c_int as int8_t,
        I_PRED_CHROMA_H as c_int as int8_t,
        I_PRED_CHROMA_DC as c_int as int8_t,
        -1 as int8_t,
        -1 as int8_t,
    ],
    [
        I_PRED_CHROMA_V as c_int as int8_t,
        I_PRED_CHROMA_H as c_int as int8_t,
        I_PRED_CHROMA_DC as c_int as int8_t,
        I_PRED_CHROMA_P as c_int as int8_t,
        -1 as int8_t,
    ],
];
#[c2rust::src_loc = "494:21"]
static mut i8x8_mode_available: [[[int8_t; 10]; 5]; 2] = [
    [
        [
            I_PRED_4x4_DC_128 as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC_LEFT as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC_TOP as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            I_PRED_4x4_DDL as c_int as int8_t,
            I_PRED_4x4_VL as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            I_PRED_4x4_DDL as c_int as int8_t,
            I_PRED_4x4_VL as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            I_PRED_4x4_DDL as c_int as int8_t,
            I_PRED_4x4_DDR as c_int as int8_t,
            I_PRED_4x4_VR as c_int as int8_t,
            I_PRED_4x4_HD as c_int as int8_t,
            I_PRED_4x4_VL as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
        ],
    ],
    [
        [
            I_PRED_4x4_DC_128 as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC_LEFT as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_HD as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
    ],
];
#[c2rust::src_loc = "512:21"]
static mut i4x4_mode_available: [[[int8_t; 10]; 5]; 2] = [
    [
        [
            I_PRED_4x4_DC_128 as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC_LEFT as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC_TOP as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            I_PRED_4x4_DDL as c_int as int8_t,
            I_PRED_4x4_VL as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            I_PRED_4x4_DDL as c_int as int8_t,
            I_PRED_4x4_VL as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            I_PRED_4x4_DDL as c_int as int8_t,
            I_PRED_4x4_DDR as c_int as int8_t,
            I_PRED_4x4_VR as c_int as int8_t,
            I_PRED_4x4_HD as c_int as int8_t,
            I_PRED_4x4_VL as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
        ],
    ],
    [
        [
            I_PRED_4x4_DC_128 as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC_LEFT as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC_TOP as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
        [
            I_PRED_4x4_DC as c_int as int8_t,
            I_PRED_4x4_H as c_int as int8_t,
            I_PRED_4x4_V as c_int as int8_t,
            I_PRED_4x4_DDR as c_int as int8_t,
            I_PRED_4x4_VR as c_int as int8_t,
            I_PRED_4x4_HD as c_int as int8_t,
            I_PRED_4x4_HU as c_int as int8_t,
            -1 as int8_t,
            -1 as int8_t,
            -1 as int8_t,
        ],
    ],
];
#[inline(always)]
#[c2rust::src_loc = "530:1"]
unsafe extern "C" fn predict_16x16_mode_available(mut i_neighbour: c_int) -> *const int8_t {
    let mut idx: c_int = i_neighbour & (MB_TOP as c_int | MB_LEFT as c_int | MB_TOPLEFT as c_int);
    idx = if idx == MB_TOP as c_int | MB_LEFT as c_int | MB_TOPLEFT as c_int {
        4 as c_int
    } else {
        idx & (MB_TOP as c_int | MB_LEFT as c_int)
    };
    return (*i16x16_mode_available.as_ptr().offset(idx as isize)).as_ptr();
}
#[inline(always)]
#[c2rust::src_loc = "537:1"]
unsafe extern "C" fn predict_chroma_mode_available(mut i_neighbour: c_int) -> *const int8_t {
    let mut idx: c_int = i_neighbour & (MB_TOP as c_int | MB_LEFT as c_int | MB_TOPLEFT as c_int);
    idx = if idx == MB_TOP as c_int | MB_LEFT as c_int | MB_TOPLEFT as c_int {
        4 as c_int
    } else {
        idx & (MB_TOP as c_int | MB_LEFT as c_int)
    };
    return (*chroma_mode_available.as_ptr().offset(idx as isize)).as_ptr();
}
#[inline(always)]
#[c2rust::src_loc = "544:1"]
unsafe extern "C" fn predict_8x8_mode_available(
    mut force_intra: c_int,
    mut i_neighbour: c_int,
    mut i: c_int,
) -> *const int8_t {
    let mut avoid_topright: c_int = (force_intra != 0 && i & 1 as c_int != 0) as c_int;
    let mut idx: c_int = i_neighbour & (MB_TOP as c_int | MB_LEFT as c_int | MB_TOPLEFT as c_int);
    idx = if idx == MB_TOP as c_int | MB_LEFT as c_int | MB_TOPLEFT as c_int {
        4 as c_int
    } else {
        idx & (MB_TOP as c_int | MB_LEFT as c_int)
    };
    return (*(*i8x8_mode_available.as_ptr().offset(avoid_topright as isize))
        .as_ptr()
        .offset(idx as isize))
    .as_ptr();
}
#[inline(always)]
#[c2rust::src_loc = "552:1"]
unsafe extern "C" fn predict_4x4_mode_available(
    mut force_intra: c_int,
    mut i_neighbour: c_int,
    mut i: c_int,
) -> *const int8_t {
    let mut avoid_topright: c_int = (force_intra != 0 && i & 5 as c_int == 5 as c_int) as c_int;
    let mut idx: c_int = i_neighbour & (MB_TOP as c_int | MB_LEFT as c_int | MB_TOPLEFT as c_int);
    idx = if idx == MB_TOP as c_int | MB_LEFT as c_int | MB_TOPLEFT as c_int {
        4 as c_int
    } else {
        idx & (MB_TOP as c_int | MB_LEFT as c_int)
    };
    return (*(*i4x4_mode_available.as_ptr().offset(avoid_topright as isize))
        .as_ptr()
        .offset(idx as isize))
    .as_ptr();
}
#[inline]
#[c2rust::src_loc = "561:1"]
unsafe extern "C" fn psy_trellis_init(mut h: *mut x264_t, mut do_both_dct: c_int) {
    if do_both_dct != 0 || (*h).mb.b_transform_8x8 != 0 {
        (*h).dctf.sub16x16_dct8.expect("non-null function pointer")(
            (*h).mb.pic.fenc_dct8.as_mut_ptr(),
            (*h).mb.pic.p_fenc[0],
            x264_zero.as_mut_ptr() as *mut pixel,
        );
    }
    if do_both_dct != 0 || (*h).mb.b_transform_8x8 == 0 {
        (*h).dctf.sub16x16_dct.expect("non-null function pointer")(
            (*h).mb.pic.fenc_dct4.as_mut_ptr(),
            (*h).mb.pic.p_fenc[0],
            x264_zero.as_mut_ptr() as *mut pixel,
        );
    }
}
#[inline]
#[c2rust::src_loc = "570:1"]
unsafe extern "C" fn mb_init_fenc_cache(mut h: *mut x264_t, mut b_satd: c_int) {
    if (*h).param.analyse.i_trellis == 2 as c_int && (*h).mb.i_psy_trellis != 0 {
        psy_trellis_init(h, (*h).param.analyse.b_transform_8x8);
    }
    if (*h).mb.i_psy_rd == 0 {
        return;
    }
    (*(&mut *(*h).mb.pic.fenc_hadamard_cache.as_mut_ptr().offset(0) as *mut uint64_t
        as *mut x264_union128_sse_t))
        .i = M128_ZERO;
    (*(&mut *(*h).mb.pic.fenc_hadamard_cache.as_mut_ptr().offset(2) as *mut uint64_t
        as *mut x264_union128_sse_t))
        .i = M128_ZERO;
    (*(&mut *(*h).mb.pic.fenc_hadamard_cache.as_mut_ptr().offset(4) as *mut uint64_t
        as *mut x264_union128_sse_t))
        .i = M128_ZERO;
    (*(&mut *(*h).mb.pic.fenc_hadamard_cache.as_mut_ptr().offset(6) as *mut uint64_t
        as *mut x264_union128_sse_t))
        .i = M128_ZERO;
    (*h).mb.pic.fenc_hadamard_cache[8] = 0 as uint64_t;
    if b_satd != 0 {
        (*h).mc.memzero_aligned.expect("non-null function pointer")(
            (*h).mb.pic.fenc_satd_cache.as_mut_ptr() as *mut c_void,
            size_of::<[uint32_t; 32]>() as size_t,
        );
    }
}
#[c2rust::src_loc = "586:1"]
unsafe extern "C" fn mb_analyse_intra_chroma(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    if (*a).i_satd_chroma < COST_MAX {
        return;
    }
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
        if (*h).mb.b_chroma_me == 0 {
            (*a).i_satd_chroma = 0 as c_int;
            return;
        }
        if (*h).mb.b_lossless != 0 {
            x264_10_predict_lossless_16x16(h, 1 as c_int, (*a).i_predict16x16);
            x264_10_predict_lossless_16x16(h, 2 as c_int, (*a).i_predict16x16);
        } else {
            (*h).predict_16x16[(*a).i_predict16x16 as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[1],
            );
            (*h).predict_16x16[(*a).i_predict16x16 as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[2],
            );
        }
        (*a).i_satd_chroma =
            (*h).pixf.mbcmp[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[1],
                FENC_STRIDE as intptr_t,
                (*h).mb.pic.p_fdec[1],
                FDEC_STRIDE as intptr_t,
            ) + (*h).pixf.mbcmp[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[2],
                FENC_STRIDE as intptr_t,
                (*h).mb.pic.p_fdec[2],
                FDEC_STRIDE as intptr_t,
            );
        return;
    }
    let mut predict_mode: *const int8_t =
        predict_chroma_mode_available((*h).mb.i_neighbour_intra as c_int);
    let mut chromapix: c_int = (*h).luma2chroma_pixel[PIXEL_16x16 as c_int as usize] as c_int;
    if *predict_mode.offset(3) as c_int >= 0 as c_int && (*h).mb.b_lossless == 0 {
        let mut satdu: [c_int; 4] = [0; 4];
        let mut satdv: [c_int; 4] = [0; 4];
        (*h).pixf
            .intra_mbcmp_x3_chroma
            .expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[1],
            (*h).mb.pic.p_fdec[1],
            satdu.as_mut_ptr(),
        );
        (*h).pixf
            .intra_mbcmp_x3_chroma
            .expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[2],
            (*h).mb.pic.p_fdec[2],
            satdv.as_mut_ptr(),
        );
        (*h).predict_chroma[I_PRED_CHROMA_P as c_int as usize].expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[1],
        );
        (*h).predict_chroma[I_PRED_CHROMA_P as c_int as usize].expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[2],
        );
        satdu[I_PRED_CHROMA_P as c_int as usize] = (*h).pixf.mbcmp[chromapix as usize]
            .expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[1],
            FENC_STRIDE as intptr_t,
            (*h).mb.pic.p_fdec[1],
            FDEC_STRIDE as intptr_t,
        );
        satdv[I_PRED_CHROMA_P as c_int as usize] = (*h).pixf.mbcmp[chromapix as usize]
            .expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[2],
            FENC_STRIDE as intptr_t,
            (*h).mb.pic.p_fdec[2],
            FDEC_STRIDE as intptr_t,
        );
        while *predict_mode as c_int >= 0 as c_int {
            let mut i_mode: c_int = *predict_mode as c_int;
            let mut i_satd: c_int = satdu[i_mode as usize]
                + satdv[i_mode as usize]
                + (*a).i_lambda * bs_size_ue(i_mode as c_uint);
            (*a).i_satd_chroma_dir[i_mode as usize] = i_satd;
            if i_satd < (*a).i_satd_chroma {
                (*a).i_satd_chroma = i_satd;
                (*a).i_predict8x8chroma = i_mode;
            }
            predict_mode = predict_mode.offset(1);
        }
    } else {
        while *predict_mode as c_int >= 0 as c_int {
            let mut i_satd_0: c_int = 0;
            let mut i_mode_0: c_int = *predict_mode as c_int;
            if (*h).mb.b_lossless != 0 {
                x264_10_predict_lossless_chroma(h, i_mode_0);
            } else {
                (*h).predict_chroma[i_mode_0 as usize].expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[1],
                );
                (*h).predict_chroma[i_mode_0 as usize].expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[2],
                );
            }
            i_satd_0 = (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[1],
                FENC_STRIDE as intptr_t,
                (*h).mb.pic.p_fdec[1],
                FDEC_STRIDE as intptr_t,
            ) + (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[2],
                FENC_STRIDE as intptr_t,
                (*h).mb.pic.p_fdec[2],
                FDEC_STRIDE as intptr_t,
            ) + (*a).i_lambda
                * bs_size_ue(x264_mb_chroma_pred_mode_fix[i_mode_0 as usize] as c_uint);
            (*a).i_satd_chroma_dir[i_mode_0 as usize] = i_satd_0;
            if i_satd_0 < (*a).i_satd_chroma {
                (*a).i_satd_chroma = i_satd_0;
                (*a).i_predict8x8chroma = i_mode_0;
            }
            predict_mode = predict_mode.offset(1);
        }
    }
    (*h).mb.i_chroma_pred_mode = (*a).i_predict8x8chroma;
}
#[c2rust::src_loc = "668:1"]
unsafe extern "C" fn mb_analyse_intra(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_satd_inter: c_int,
) {
    let flags: c_uint = if (*h).sh.i_type == SLICE_TYPE_I as c_int {
        (*h).param.analyse.intra
    } else {
        (*h).param.analyse.inter
    };
    let mut p_src: *mut pixel = (*h).mb.pic.p_fenc[0];
    let mut p_dst: *mut pixel = (*h).mb.pic.p_fdec[0];
    static mut intra_analysis_shortcut: [[[[int8_t; 5]; 2]; 2]; 2] = [
        [
            [
                [
                    I_PRED_4x4_HU as c_int as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                ],
                [
                    I_PRED_4x4_DDL as c_int as int8_t,
                    I_PRED_4x4_VL as c_int as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                ],
            ],
            [
                [
                    I_PRED_4x4_DDR as c_int as int8_t,
                    I_PRED_4x4_HD as c_int as int8_t,
                    I_PRED_4x4_HU as c_int as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                ],
                [
                    I_PRED_4x4_DDL as c_int as int8_t,
                    I_PRED_4x4_DDR as c_int as int8_t,
                    I_PRED_4x4_VR as c_int as int8_t,
                    I_PRED_4x4_VL as c_int as int8_t,
                    -1 as int8_t,
                ],
            ],
        ],
        [
            [
                [
                    I_PRED_4x4_HU as c_int as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                ],
                [
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                ],
            ],
            [
                [
                    I_PRED_4x4_DDR as c_int as int8_t,
                    I_PRED_4x4_HD as c_int as int8_t,
                    I_PRED_4x4_HU as c_int as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                ],
                [
                    I_PRED_4x4_DDR as c_int as int8_t,
                    I_PRED_4x4_VR as c_int as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                    -1 as int8_t,
                ],
            ],
        ],
    ];
    let mut idx: c_int = 0;
    let mut lambda: c_int = (*a).i_lambda;
    if (*h).param.i_avcintra_class == 0 {
        let mut predict_mode: *const int8_t =
            predict_16x16_mode_available((*h).mb.i_neighbour_intra as c_int);
        static mut i16x16_thresh_lut: [uint8_t; 11] = [
            2 as c_int as uint8_t,
            2 as c_int as uint8_t,
            2 as c_int as uint8_t,
            3 as c_int as uint8_t,
            3 as c_int as uint8_t,
            4 as c_int as uint8_t,
            4 as c_int as uint8_t,
            4 as c_int as uint8_t,
            4 as c_int as uint8_t,
            4 as c_int as uint8_t,
            4 as c_int as uint8_t,
        ];
        let mut i16x16_thresh: c_int = if (*a).b_fast_intra != 0 {
            i16x16_thresh_lut[(*h).mb.i_subpel_refine as usize] as c_int * i_satd_inter
                >> 1 as c_int
        } else {
            COST_MAX
        };
        if (*h).mb.b_lossless == 0 && *predict_mode.offset(3) as c_int >= 0 as c_int {
            (*h).pixf
                .intra_mbcmp_x3_16x16
                .expect("non-null function pointer")(
                p_src,
                p_dst,
                (*a).i_satd_i16x16_dir.as_mut_ptr(),
            );
            (*a).i_satd_i16x16_dir[0] += lambda * bs_size_ue(0 as c_uint);
            (*a).i_satd_i16x16_dir[1] += lambda * bs_size_ue(1 as c_uint);
            (*a).i_satd_i16x16_dir[2] += lambda * bs_size_ue(2 as c_uint);
            if (*a).i_satd_i16x16_dir[0] < (*a).i_satd_i16x16 {
                (*a).i_satd_i16x16 = (*a).i_satd_i16x16_dir[0];
                (*a).i_predict16x16 = 0 as c_int;
            }
            if (*a).i_satd_i16x16_dir[1] < (*a).i_satd_i16x16 {
                (*a).i_satd_i16x16 = (*a).i_satd_i16x16_dir[1];
                (*a).i_predict16x16 = 1 as c_int;
            }
            if (*a).i_satd_i16x16_dir[2] < (*a).i_satd_i16x16 {
                (*a).i_satd_i16x16 = (*a).i_satd_i16x16_dir[2];
                (*a).i_predict16x16 = 2 as c_int;
            }
            if (*a).i_satd_i16x16 <= i16x16_thresh {
                (*h).predict_16x16[I_PRED_16x16_P as c_int as usize]
                    .expect("non-null function pointer")(p_dst);
                (*a).i_satd_i16x16_dir[I_PRED_16x16_P as c_int as usize] = (*h).pixf.mbcmp
                    [PIXEL_16x16 as c_int as usize]
                    .expect("non-null function pointer")(
                    p_src,
                    FENC_STRIDE as intptr_t,
                    p_dst,
                    FDEC_STRIDE as intptr_t,
                );
                (*a).i_satd_i16x16_dir[I_PRED_16x16_P as c_int as usize] +=
                    lambda * bs_size_ue(3 as c_uint);
                if (*a).i_satd_i16x16_dir[I_PRED_16x16_P as c_int as usize] < (*a).i_satd_i16x16 {
                    (*a).i_satd_i16x16 = (*a).i_satd_i16x16_dir[I_PRED_16x16_P as c_int as usize];
                    (*a).i_predict16x16 = 3 as c_int;
                }
            }
        } else {
            while *predict_mode as c_int >= 0 as c_int {
                let mut i_satd: c_int = 0;
                let mut i_mode: c_int = *predict_mode as c_int;
                if (*h).mb.b_lossless != 0 {
                    x264_10_predict_lossless_16x16(h, 0 as c_int, i_mode);
                } else {
                    (*h).predict_16x16[i_mode as usize].expect("non-null function pointer")(p_dst);
                }
                i_satd = (*h).pixf.mbcmp[PIXEL_16x16 as c_int as usize]
                    .expect("non-null function pointer")(
                    p_src,
                    FENC_STRIDE as intptr_t,
                    p_dst,
                    FDEC_STRIDE as intptr_t,
                ) + lambda
                    * bs_size_ue(x264_mb_pred_mode16x16_fix[i_mode as usize] as c_uint);
                if i_satd < (*a).i_satd_i16x16 {
                    (*a).i_satd_i16x16 = i_satd;
                    (*a).i_predict16x16 = i_mode;
                }
                (*a).i_satd_i16x16_dir[i_mode as usize] = i_satd;
                predict_mode = predict_mode.offset(1);
            }
        }
        if (*h).sh.i_type == SLICE_TYPE_B as c_int {
            (*a).i_satd_i16x16 += lambda * i_mb_b_cost_table[I_16x16 as c_int as usize] as c_int;
        }
        if (*a).i_satd_i16x16 > i16x16_thresh {
            return;
        }
    }
    let mut cost_i4x4_mode: *mut uint16_t = (*(*(*h).cost_table)
        .i4x4_mode
        .as_mut_ptr()
        .offset((*a).i_qp as isize))
    .as_mut_ptr()
    .offset(8);
    if flags & X264_ANALYSE_I8x8 != 0 {
        let mut edge: [pixel; 36] = [0; 36];
        let mut sa8d: x264_pixel_cmp_t = if (*h).pixf.mbcmp[0] == (*h).pixf.satd[0] {
            (*h).pixf.sa8d[PIXEL_8x8 as c_int as usize]
        } else {
            (*h).pixf.mbcmp[PIXEL_8x8 as c_int as usize]
        };
        let mut i_satd_thresh: c_int = if (*a).i_mbrd != 0 {
            COST_MAX
        } else if i_satd_inter < (*a).i_satd_i16x16 {
            i_satd_inter
        } else {
            (*a).i_satd_i16x16
        };
        let mut i_cost: c_int = lambda * 4 as c_int;
        (*h).mb.i_cbp_luma = 0 as c_int;
        if (*h).sh.i_type == SLICE_TYPE_B as c_int {
            i_cost += lambda * i_mb_b_cost_table[I_8x8 as c_int as usize] as c_int;
        }
        idx = 0 as c_int;
        loop {
            let mut x: c_int = idx & 1 as c_int;
            let mut y: c_int = idx >> 1 as c_int;
            let mut p_src_by: *mut pixel = p_src
                .offset((8 as c_int * x) as isize)
                .offset((8 as c_int * y * FENC_STRIDE) as isize);
            let mut p_dst_by: *mut pixel = p_dst
                .offset((8 as c_int * x) as isize)
                .offset((8 as c_int * y * FDEC_STRIDE) as isize);
            let mut i_best: c_int = COST_MAX;
            let mut i_pred_mode: c_int = x264_mb_predict_intra4x4_mode(h, 4 as c_int * idx);
            let mut predict_mode_0: *const int8_t = predict_8x8_mode_available(
                (*a).b_avoid_topright,
                (*h).mb.i_neighbour8[idx as usize] as c_int,
                idx,
            );
            (*h).predict_8x8_filter.expect("non-null function pointer")(
                p_dst_by,
                edge.as_mut_ptr(),
                (*h).mb.i_neighbour8[idx as usize] as c_int,
                ALL_NEIGHBORS as c_int,
            );
            if (*h).pixf.intra_mbcmp_x9_8x8.is_some()
                && *predict_mode_0.offset(8) as c_int >= 0 as c_int
            {
                i_best = (*h)
                    .pixf
                    .intra_mbcmp_x9_8x8
                    .expect("non-null function pointer")(
                    p_src_by,
                    p_dst_by,
                    edge.as_mut_ptr(),
                    cost_i4x4_mode.offset(-(i_pred_mode as isize)),
                    (*(*a).i_satd_i8x8_dir.as_mut_ptr().offset(idx as isize)).as_mut_ptr(),
                );
                i_cost += i_best & 0xffff as c_int;
                i_best >>= 16 as c_int;
                (*a).i_predict8x8[idx as usize] = i_best;
                if idx == 3 as c_int || i_cost > i_satd_thresh {
                    break;
                }
                x264_macroblock_cache_intra8x8_pred(h, 2 as c_int * x, 2 as c_int * y, i_best);
            } else {
                if (*h).mb.b_lossless == 0 && *predict_mode_0.offset(5) as c_int >= 0 as c_int {
                    let mut satd: [int32_t; 4] = [0; 4];
                    (*h).pixf
                        .intra_mbcmp_x3_8x8
                        .expect("non-null function pointer")(
                        p_src_by,
                        edge.as_mut_ptr(),
                        satd.as_mut_ptr() as *mut c_int,
                    );
                    let mut favor_vertical: c_int = (satd[I_PRED_4x4_H as c_int as usize]
                        > satd[I_PRED_4x4_V as c_int as usize])
                        as c_int;
                    if i_pred_mode < 3 as c_int {
                        satd[i_pred_mode as usize] =
                            (satd[i_pred_mode as usize] as c_int - 3 as c_int * lambda) as int32_t;
                    }
                    let mut i: c_int = 2 as c_int;
                    while i >= 0 as c_int {
                        let mut cost: c_int = satd[i as usize];
                        (*a).i_satd_i8x8_dir[idx as usize][i as usize] =
                            (cost + 4 as c_int * lambda) as uint16_t;
                        if cost < i_best {
                            i_best = cost;
                            (*a).i_predict8x8[idx as usize] = i;
                        }
                        i -= 1;
                    }
                    if (*a).i_mbrd < 1 as c_int + (*a).b_fast_intra {
                        predict_mode_0 = (*(*(*intra_analysis_shortcut
                            .as_ptr()
                            .offset((*a).b_avoid_topright as isize))
                        .as_ptr()
                        .offset(
                            (*predict_mode_0.offset(8) as c_int >= 0 as c_int) as c_int as isize,
                        ))
                        .as_ptr()
                        .offset(favor_vertical as isize))
                        .as_ptr();
                    } else {
                        predict_mode_0 = predict_mode_0.offset(3);
                    }
                }
                while *predict_mode_0 as c_int >= 0 as c_int
                    && (i_best >= 0 as c_int || (*a).i_mbrd >= 2 as c_int)
                {
                    let mut i_satd_0: c_int = 0;
                    let mut i_mode_0: c_int = *predict_mode_0 as c_int;
                    if (*h).mb.b_lossless != 0 {
                        x264_10_predict_lossless_8x8(
                            h,
                            p_dst_by,
                            0 as c_int,
                            idx,
                            i_mode_0,
                            edge.as_mut_ptr(),
                        );
                    } else {
                        (*h).predict_8x8[i_mode_0 as usize].expect("non-null function pointer")(
                            p_dst_by,
                            edge.as_mut_ptr(),
                        );
                    }
                    i_satd_0 = sa8d.expect("non-null function pointer")(
                        p_dst_by,
                        FDEC_STRIDE as intptr_t,
                        p_src_by,
                        FENC_STRIDE as intptr_t,
                    );
                    if i_pred_mode
                        == x264_mb_pred_mode4x4_fix[(i_mode_0 + 1 as c_int) as usize] as c_int
                    {
                        i_satd_0 -= 3 as c_int * lambda;
                    }
                    if i_satd_0 < i_best {
                        i_best = i_satd_0;
                        (*a).i_predict8x8[idx as usize] = i_mode_0;
                    }
                    (*a).i_satd_i8x8_dir[idx as usize][i_mode_0 as usize] =
                        (i_satd_0 + 4 as c_int * lambda) as uint16_t;
                    predict_mode_0 = predict_mode_0.offset(1);
                }
                i_cost += i_best + 3 as c_int * lambda;
                if idx == 3 as c_int || i_cost > i_satd_thresh {
                    break;
                }
                if (*h).mb.b_lossless != 0 {
                    x264_10_predict_lossless_8x8(
                        h,
                        p_dst_by,
                        0 as c_int,
                        idx,
                        (*a).i_predict8x8[idx as usize],
                        edge.as_mut_ptr(),
                    );
                } else {
                    (*h).predict_8x8[(*a).i_predict8x8[idx as usize] as usize]
                        .expect("non-null function pointer")(
                        p_dst_by, edge.as_mut_ptr()
                    );
                }
                x264_macroblock_cache_intra8x8_pred(
                    h,
                    2 as c_int * x,
                    2 as c_int * y,
                    (*a).i_predict8x8[idx as usize],
                );
            }
            x264_mb_encode_i8x8(
                h,
                0 as c_int,
                idx,
                (*a).i_qp,
                (*a).i_predict8x8[idx as usize],
                edge.as_mut_ptr(),
                0 as c_int,
            );
            idx += 1;
        }
        if idx == 3 as c_int {
            (*a).i_satd_i8x8 = i_cost;
            if (*h).mb.i_skip_intra != 0 {
                (*h).mc.copy[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
                    (*h).mb.pic.i8x8_fdec_buf.as_mut_ptr(),
                    16 as intptr_t,
                    p_dst,
                    FDEC_STRIDE as intptr_t,
                    16 as c_int,
                );
                (*h).mb.pic.i8x8_nnz_buf[0] = (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(0) as isize)
                    as *mut uint8_t
                    as *mut x264_union32_t))
                    .i;
                (*h).mb.pic.i8x8_nnz_buf[1] = (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(2) as isize)
                    as *mut uint8_t
                    as *mut x264_union32_t))
                    .i;
                (*h).mb.pic.i8x8_nnz_buf[2] = (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(8) as isize)
                    as *mut uint8_t
                    as *mut x264_union32_t))
                    .i;
                (*h).mb.pic.i8x8_nnz_buf[3] = (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(10 as c_int as isize) as isize)
                    as *mut uint8_t
                    as *mut x264_union32_t))
                    .i;
                (*h).mb.pic.i8x8_cbp = (*h).mb.i_cbp_luma;
                if (*h).mb.i_skip_intra == 2 as c_int {
                    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                        (*h).mb.pic.i8x8_dct_buf.as_mut_ptr() as *mut c_void,
                        (*h).dct.luma8x8.as_mut_ptr() as *const c_void,
                        size_of::<[[dctcoef; 64]; 3]>() as size_t,
                    );
                }
            }
        } else {
            static mut cost_div_fix8: [uint16_t; 3] = [
                1024 as c_int as uint16_t,
                512 as c_int as uint16_t,
                341 as c_int as uint16_t,
            ];
            (*a).i_satd_i8x8 = COST_MAX;
            i_cost = i_cost * cost_div_fix8[idx as usize] as c_int >> 8 as c_int;
        }
        static mut i8x8_thresh: [uint8_t; 11] = [
            4 as c_int as uint8_t,
            4 as c_int as uint8_t,
            4 as c_int as uint8_t,
            5 as c_int as uint8_t,
            5 as c_int as uint8_t,
            5 as c_int as uint8_t,
            6 as c_int as uint8_t,
            6 as c_int as uint8_t,
            6 as c_int as uint8_t,
            6 as c_int as uint8_t,
            6 as c_int as uint8_t,
        ];
        if (*a).b_early_terminate != 0
            && (if i_cost < (*a).i_satd_i16x16 {
                i_cost
            } else {
                (*a).i_satd_i16x16
            }) > i_satd_inter * i8x8_thresh[(*h).mb.i_subpel_refine as usize] as c_int
                >> 2 as c_int
        {
            return;
        }
    }
    if flags & X264_ANALYSE_I4x4 != 0 {
        let mut i_cost_0: c_int = lambda * (24 as c_int + 16 as c_int);
        let mut i_satd_thresh_0: c_int = if (*a).b_early_terminate != 0 {
            if i_satd_inter
                < (if (*a).i_satd_i16x16 < (*a).i_satd_i8x8 {
                    (*a).i_satd_i16x16
                } else {
                    (*a).i_satd_i8x8
                })
            {
                i_satd_inter
            } else if (*a).i_satd_i16x16 < (*a).i_satd_i8x8 {
                (*a).i_satd_i16x16
            } else {
                (*a).i_satd_i8x8
            }
        } else {
            COST_MAX
        };
        (*h).mb.i_cbp_luma = 0 as c_int;
        if (*a).b_early_terminate != 0 && (*a).i_mbrd != 0 {
            i_satd_thresh_0 = i_satd_thresh_0 * (10 as c_int - (*a).b_fast_intra) / 8 as c_int;
        }
        if (*h).sh.i_type == SLICE_TYPE_B as c_int {
            i_cost_0 += lambda * i_mb_b_cost_table[I_4x4 as c_int as usize] as c_int;
        }
        idx = 0 as c_int;
        loop {
            let mut p_src_by_0: *mut pixel =
                p_src.offset(block_idx_xy_fenc[idx as usize] as c_int as isize);
            let mut p_dst_by_0: *mut pixel =
                p_dst.offset(block_idx_xy_fdec[idx as usize] as c_int as isize);
            let mut i_best_0: c_int = COST_MAX;
            let mut i_pred_mode_0: c_int = x264_mb_predict_intra4x4_mode(h, idx);
            let mut predict_mode_1: *const int8_t = predict_4x4_mode_available(
                (*a).b_avoid_topright,
                (*h).mb.i_neighbour4[idx as usize] as c_int,
                idx,
            );
            if (*h).mb.i_neighbour4[idx as usize]
                & (MB_TOPRIGHT as c_int | MB_TOP as c_int) as c_uint
                == MB_TOP as c_int as c_uint
            {
                (*(&mut *p_dst_by_0.offset((4 as c_int - 32 as c_int) as isize) as *mut pixel
                    as *mut x264_union64_t))
                    .i = (*p_dst_by_0.offset((3 as c_int - 32 as c_int) as isize) as c_ulonglong)
                    .wrapping_mul(0x1000100010001 as c_ulonglong)
                    as uint64_t;
            }
            if (*h).pixf.intra_mbcmp_x9_4x4.is_some()
                && *predict_mode_1.offset(8) as c_int >= 0 as c_int
            {
                i_best_0 = (*h)
                    .pixf
                    .intra_mbcmp_x9_4x4
                    .expect("non-null function pointer")(
                    p_src_by_0,
                    p_dst_by_0,
                    cost_i4x4_mode.offset(-(i_pred_mode_0 as isize)),
                );
                i_cost_0 += i_best_0 & 0xffff as c_int;
                i_best_0 >>= 16 as c_int;
                (*a).i_predict4x4[idx as usize] = i_best_0;
                if i_cost_0 > i_satd_thresh_0 || idx == 15 as c_int {
                    break;
                }
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[idx as usize] as usize] =
                    i_best_0 as int8_t;
            } else {
                if (*h).mb.b_lossless == 0 && *predict_mode_1.offset(5) as c_int >= 0 as c_int {
                    let mut satd_0: [int32_t; 4] = [0; 4];
                    (*h).pixf
                        .intra_mbcmp_x3_4x4
                        .expect("non-null function pointer")(
                        p_src_by_0,
                        p_dst_by_0,
                        satd_0.as_mut_ptr() as *mut c_int,
                    );
                    let mut favor_vertical_0: c_int = (satd_0[I_PRED_4x4_H as c_int as usize]
                        > satd_0[I_PRED_4x4_V as c_int as usize])
                        as c_int;
                    if i_pred_mode_0 < 3 as c_int {
                        satd_0[i_pred_mode_0 as usize] = (satd_0[i_pred_mode_0 as usize] as c_int
                            - 3 as c_int * lambda)
                            as int32_t;
                    }
                    i_best_0 = satd_0[I_PRED_4x4_DC as c_int as usize] as c_int;
                    (*a).i_predict4x4[idx as usize] = I_PRED_4x4_DC as c_int;
                    if satd_0[I_PRED_4x4_H as c_int as usize] < i_best_0 as int32_t {
                        i_best_0 = satd_0[I_PRED_4x4_H as c_int as usize] as c_int;
                        (*a).i_predict4x4[idx as usize] = I_PRED_4x4_H as c_int;
                    }
                    if satd_0[I_PRED_4x4_V as c_int as usize] < i_best_0 as int32_t {
                        i_best_0 = satd_0[I_PRED_4x4_V as c_int as usize] as c_int;
                        (*a).i_predict4x4[idx as usize] = I_PRED_4x4_V as c_int;
                    }
                    if (*a).i_mbrd < 1 as c_int + (*a).b_fast_intra {
                        predict_mode_1 = (*(*(*intra_analysis_shortcut
                            .as_ptr()
                            .offset((*a).b_avoid_topright as isize))
                        .as_ptr()
                        .offset(
                            (*predict_mode_1.offset(8) as c_int >= 0 as c_int) as c_int as isize,
                        ))
                        .as_ptr()
                        .offset(favor_vertical_0 as isize))
                        .as_ptr();
                    } else {
                        predict_mode_1 = predict_mode_1.offset(3);
                    }
                }
                if i_best_0 > 0 as c_int {
                    while *predict_mode_1 as c_int >= 0 as c_int {
                        let mut i_satd_1: c_int = 0;
                        let mut i_mode_1: c_int = *predict_mode_1 as c_int;
                        if (*h).mb.b_lossless != 0 {
                            x264_10_predict_lossless_4x4(h, p_dst_by_0, 0 as c_int, idx, i_mode_1);
                        } else {
                            (*h).predict_4x4[i_mode_1 as usize].expect("non-null function pointer")(
                                p_dst_by_0,
                            );
                        }
                        i_satd_1 = (*h).pixf.mbcmp[PIXEL_4x4 as c_int as usize]
                            .expect("non-null function pointer")(
                            p_src_by_0,
                            FENC_STRIDE as intptr_t,
                            p_dst_by_0,
                            FDEC_STRIDE as intptr_t,
                        );
                        if i_pred_mode_0
                            == x264_mb_pred_mode4x4_fix[(i_mode_1 + 1 as c_int) as usize] as c_int
                        {
                            i_satd_1 -= lambda * 3 as c_int;
                            if i_satd_1 <= 0 as c_int {
                                i_best_0 = i_satd_1;
                                (*a).i_predict4x4[idx as usize] = i_mode_1;
                                break;
                            }
                        }
                        if i_satd_1 < i_best_0 {
                            i_best_0 = i_satd_1;
                            (*a).i_predict4x4[idx as usize] = i_mode_1;
                        }
                        predict_mode_1 = predict_mode_1.offset(1);
                    }
                }
                i_cost_0 += i_best_0 + 3 as c_int * lambda;
                if i_cost_0 > i_satd_thresh_0 || idx == 15 as c_int {
                    break;
                }
                if (*h).mb.b_lossless != 0 {
                    x264_10_predict_lossless_4x4(
                        h,
                        p_dst_by_0,
                        0 as c_int,
                        idx,
                        (*a).i_predict4x4[idx as usize],
                    );
                } else {
                    (*h).predict_4x4[(*a).i_predict4x4[idx as usize] as usize]
                        .expect("non-null function pointer")(p_dst_by_0);
                }
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[idx as usize] as usize] =
                    (*a).i_predict4x4[idx as usize] as int8_t;
            }
            x264_mb_encode_i4x4(
                h,
                0 as c_int,
                idx,
                (*a).i_qp,
                (*a).i_predict4x4[idx as usize],
                0 as c_int,
            );
            idx += 1;
        }
        if idx == 15 as c_int {
            (*a).i_satd_i4x4 = i_cost_0;
            if (*h).mb.i_skip_intra != 0 {
                (*h).mc.copy[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
                    (*h).mb.pic.i4x4_fdec_buf.as_mut_ptr(),
                    16 as intptr_t,
                    p_dst,
                    FDEC_STRIDE as intptr_t,
                    16 as c_int,
                );
                (*h).mb.pic.i4x4_nnz_buf[0] = (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(0) as isize)
                    as *mut uint8_t
                    as *mut x264_union32_t))
                    .i;
                (*h).mb.pic.i4x4_nnz_buf[1] = (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(2) as isize)
                    as *mut uint8_t
                    as *mut x264_union32_t))
                    .i;
                (*h).mb.pic.i4x4_nnz_buf[2] = (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(8) as isize)
                    as *mut uint8_t
                    as *mut x264_union32_t))
                    .i;
                (*h).mb.pic.i4x4_nnz_buf[3] = (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(10 as c_int as isize) as isize)
                    as *mut uint8_t
                    as *mut x264_union32_t))
                    .i;
                (*h).mb.pic.i4x4_cbp = (*h).mb.i_cbp_luma;
                if (*h).mb.i_skip_intra == 2 as c_int {
                    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                        (*h).mb.pic.i4x4_dct_buf.as_mut_ptr() as *mut c_void,
                        (*h).dct.luma4x4.as_mut_ptr() as *const c_void,
                        size_of::<[[dctcoef; 16]; 15]>() as size_t,
                    );
                }
            }
        } else {
            (*a).i_satd_i4x4 = COST_MAX;
        }
    }
}
#[c2rust::src_loc = "982:1"]
unsafe extern "C" fn intra_rd(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_satd_thresh: c_int,
) {
    if (*a).b_early_terminate == 0 {
        i_satd_thresh = COST_MAX;
    }
    if (*a).i_satd_i16x16 < i_satd_thresh {
        (*h).mb.i_type = I_16x16 as c_int;
        analyse_update_cache(h, a);
        (*a).i_satd_i16x16 = rd_cost_mb(h, (*a).i_lambda2);
    } else {
        (*a).i_satd_i16x16 = COST_MAX;
    }
    if (*a).i_satd_i4x4 < i_satd_thresh {
        (*h).mb.i_type = I_4x4 as c_int;
        analyse_update_cache(h, a);
        (*a).i_satd_i4x4 = rd_cost_mb(h, (*a).i_lambda2);
    } else {
        (*a).i_satd_i4x4 = COST_MAX;
    }
    if (*a).i_satd_i8x8 < i_satd_thresh {
        (*h).mb.i_type = I_8x8 as c_int;
        analyse_update_cache(h, a);
        (*a).i_satd_i8x8 = rd_cost_mb(h, (*a).i_lambda2);
        (*a).i_cbp_i8x8_luma = (*h).mb.i_cbp_luma;
    } else {
        (*a).i_satd_i8x8 = COST_MAX;
    };
}
#[c2rust::src_loc = "1016:1"]
unsafe extern "C" fn intra_rd_refine(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    let mut i_satd: uint64_t = 0;
    let mut i_best: uint64_t = 0;
    let mut plane_count: c_int =
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            3 as c_int
        } else {
            1 as c_int
        };
    (*h).mb.i_skip_intra = 0 as c_int;
    if (*h).mb.i_type == I_16x16 as c_int {
        let mut old_pred_mode: c_int = (*a).i_predict16x16;
        let mut predict_mode: *const int8_t =
            predict_16x16_mode_available((*h).mb.i_neighbour_intra as c_int);
        let mut i_thresh: c_int = if (*a).b_early_terminate != 0 {
            (*a).i_satd_i16x16_dir[old_pred_mode as usize] * 9 as c_int / 8 as c_int
        } else {
            COST_MAX
        };
        i_best = (*a).i_satd_i16x16 as uint64_t;
        while *predict_mode as c_int >= 0 as c_int {
            let mut i_mode: c_int = *predict_mode as c_int;
            if !(i_mode == old_pred_mode || (*a).i_satd_i16x16_dir[i_mode as usize] > i_thresh) {
                (*h).mb.i_intra16x16_pred_mode = i_mode;
                i_satd = rd_cost_mb(h, (*a).i_lambda2) as uint64_t;
                if i_satd < i_best {
                    i_best = i_satd;
                    (*a).i_predict16x16 = i_mode;
                }
            }
            predict_mode = predict_mode.offset(1);
        }
    }
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as c_int
        || (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as c_int
    {
        let mut predict_mode_0: *const int8_t =
            predict_chroma_mode_available((*h).mb.i_neighbour_intra as c_int);
        if *predict_mode_0.offset(1) as c_int >= 0 as c_int {
            let mut predict_mode_sorted: [int8_t; 4] = [0; 4];
            let mut i_max: c_int = 0;
            let mut i_thresh_0: c_int = if (*a).b_early_terminate != 0 {
                (*a).i_satd_chroma * 5 as c_int / 4 as c_int
            } else {
                COST_MAX
            };
            i_max = 0 as c_int;
            while *predict_mode_0 as c_int >= 0 as c_int {
                let mut i_mode_0: c_int = *predict_mode_0 as c_int;
                if (*a).i_satd_chroma_dir[i_mode_0 as usize] < i_thresh_0
                    && i_mode_0 != (*a).i_predict8x8chroma
                {
                    let fresh4 = i_max;
                    i_max = i_max + 1;
                    predict_mode_sorted[fresh4 as usize] = i_mode_0 as int8_t;
                }
                predict_mode_0 = predict_mode_0.offset(1);
            }
            if i_max > 0 as c_int {
                let mut i_cbp_chroma_best: c_int = (*h).mb.i_cbp_chroma;
                let mut i_chroma_lambda: c_int = x264_lambda2_tab[(*h).mb.i_chroma_qp as usize];
                i_best = rd_cost_chroma(h, i_chroma_lambda, (*a).i_predict8x8chroma, 0 as c_int);
                let mut i: c_int = 0 as c_int;
                while i < i_max {
                    let mut i_mode_1: c_int = predict_mode_sorted[i as usize] as c_int;
                    if (*h).mb.b_lossless != 0 {
                        x264_10_predict_lossless_chroma(h, i_mode_1);
                    } else {
                        (*h).predict_chroma[i_mode_1 as usize].expect("non-null function pointer")(
                            (*h).mb.pic.p_fdec[1],
                        );
                        (*h).predict_chroma[i_mode_1 as usize].expect("non-null function pointer")(
                            (*h).mb.pic.p_fdec[2],
                        );
                    }
                    i_satd = rd_cost_chroma(
                        h,
                        i_chroma_lambda,
                        i_mode_1,
                        ((*h).mb.i_cbp_chroma != 0 as c_int) as c_int,
                    );
                    if i_satd < i_best {
                        i_best = i_satd;
                        (*a).i_predict8x8chroma = i_mode_1;
                        i_cbp_chroma_best = (*h).mb.i_cbp_chroma;
                    }
                    i += 1;
                }
                (*h).mb.i_chroma_pred_mode = (*a).i_predict8x8chroma;
                (*h).mb.i_cbp_chroma = i_cbp_chroma_best;
            }
        }
    }
    if (*h).mb.i_type == I_4x4 as c_int {
        let mut pels: [[pixel4; 4]; 3] = [[0 as c_int as pixel4, 0, 0, 0], [0; 4], [0; 4]];
        let mut nnz: [c_int; 3] = [0 as c_int; 3];
        let mut idx: c_int = 0 as c_int;
        while idx < 16 as c_int {
            let mut dst: [*mut pixel; 3] = [
                (*h).mb.pic.p_fdec[0].offset(block_idx_xy_fdec[idx as usize] as c_int as isize),
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                    (*h).mb.pic.p_fdec[1].offset(block_idx_xy_fdec[idx as usize] as c_int as isize)
                } else {
                    0 as *mut pixel
                },
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                    (*h).mb.pic.p_fdec[2].offset(block_idx_xy_fdec[idx as usize] as c_int as isize)
                } else {
                    0 as *mut pixel
                },
            ];
            i_best = COST_MAX64 as uint64_t;
            let mut predict_mode_1: *const int8_t = predict_4x4_mode_available(
                (*a).b_avoid_topright,
                (*h).mb.i_neighbour4[idx as usize] as c_int,
                idx,
            );
            if (*h).mb.i_neighbour4[idx as usize]
                & (MB_TOPRIGHT as c_int | MB_TOP as c_int) as c_uint
                == MB_TOP as c_int as c_uint
            {
                let mut p: c_int = 0 as c_int;
                while p < plane_count {
                    (*(dst[p as usize].offset(4).offset(-(32 as c_int as isize))
                        as *mut x264_union64_t))
                        .i = (*dst[p as usize].offset((3 as c_int - 32 as c_int) as isize)
                        as c_ulonglong)
                        .wrapping_mul(0x1000100010001 as c_ulonglong)
                        as uint64_t;
                    p += 1;
                }
            }
            while *predict_mode_1 as c_int >= 0 as c_int {
                let mut i_mode_2: c_int = *predict_mode_1 as c_int;
                i_satd = rd_cost_i4x4(h, (*a).i_lambda2, idx, i_mode_2);
                if i_best > i_satd {
                    (*a).i_predict4x4[idx as usize] = i_mode_2;
                    i_best = i_satd;
                    let mut p_0: c_int = 0 as c_int;
                    while p_0 < plane_count {
                        pels[p_0 as usize][0] = (*(dst[p_0 as usize]
                            .offset((0 as c_int * 32 as c_int) as isize)
                            as *mut x264_union64_t))
                            .i as pixel4;
                        pels[p_0 as usize][1] = (*(dst[p_0 as usize]
                            .offset((1 as c_int * 32 as c_int) as isize)
                            as *mut x264_union64_t))
                            .i as pixel4;
                        pels[p_0 as usize][2] = (*(dst[p_0 as usize]
                            .offset((2 as c_int * 32 as c_int) as isize)
                            as *mut x264_union64_t))
                            .i as pixel4;
                        pels[p_0 as usize][3] = (*(dst[p_0 as usize]
                            .offset((3 as c_int * 32 as c_int) as isize)
                            as *mut x264_union64_t))
                            .i as pixel4;
                        nnz[p_0 as usize] = (*h).mb.cache.non_zero_count
                            [x264_scan8[(idx + p_0 * 16 as c_int) as usize] as usize]
                            as c_int;
                        p_0 += 1;
                    }
                }
                predict_mode_1 = predict_mode_1.offset(1);
            }
            let mut p_1: c_int = 0 as c_int;
            while p_1 < plane_count {
                (*(dst[p_1 as usize].offset((0 as c_int * 32 as c_int) as isize)
                    as *mut x264_union64_t))
                    .i = pels[p_1 as usize][0] as uint64_t;
                (*(dst[p_1 as usize].offset((1 as c_int * 32 as c_int) as isize)
                    as *mut x264_union64_t))
                    .i = pels[p_1 as usize][1] as uint64_t;
                (*(dst[p_1 as usize].offset((2 as c_int * 32 as c_int) as isize)
                    as *mut x264_union64_t))
                    .i = pels[p_1 as usize][2] as uint64_t;
                (*(dst[p_1 as usize].offset((3 as c_int * 32 as c_int) as isize)
                    as *mut x264_union64_t))
                    .i = pels[p_1 as usize][3] as uint64_t;
                (*h).mb.cache.non_zero_count
                    [x264_scan8[(idx + p_1 * 16 as c_int) as usize] as usize] =
                    nnz[p_1 as usize] as uint8_t;
                p_1 += 1;
            }
            (*h).mb.cache.intra4x4_pred_mode[x264_scan8[idx as usize] as usize] =
                (*a).i_predict4x4[idx as usize] as int8_t;
            idx += 1;
        }
    } else if (*h).mb.i_type == I_8x8 as c_int {
        let mut edge: [[pixel; 32]; 4] = [[0; 32]; 4];
        let mut pels_h: [[pixel4; 2]; 3] = [[0 as c_int as pixel4, 0], [0; 2], [0; 2]];
        let mut pels_v: [[pixel; 7]; 3] = [[0 as c_int as pixel, 0, 0, 0, 0, 0, 0], [0; 7], [0; 7]];
        let mut nnz_0: [[uint16_t; 2]; 3] = [[0 as c_int as uint16_t, 0], [0; 2], [0; 2]];
        let mut idx_0: c_int = 0 as c_int;
        while idx_0 < 4 as c_int {
            let mut x: c_int = idx_0 & 1 as c_int;
            let mut y: c_int = idx_0 >> 1 as c_int;
            let mut s8: c_int = X264_SCAN8_0 + 2 as c_int * x + 16 as c_int * y;
            let mut dst_0: [*mut pixel; 3] = [
                (*h).mb.pic.p_fdec[0]
                    .offset((8 as c_int * x) as isize)
                    .offset((8 as c_int * y * FDEC_STRIDE) as isize),
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                    (*h).mb.pic.p_fdec[1]
                        .offset((8 as c_int * x) as isize)
                        .offset((8 as c_int * y * FDEC_STRIDE) as isize)
                } else {
                    0 as *mut pixel
                },
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                    (*h).mb.pic.p_fdec[2]
                        .offset((8 as c_int * x) as isize)
                        .offset((8 as c_int * y * FDEC_STRIDE) as isize)
                } else {
                    0 as *mut pixel
                },
            ];
            let mut cbp_luma_new: c_int = 0 as c_int;
            let mut i_thresh_1: c_int = if (*a).b_early_terminate != 0 {
                (*a).i_satd_i8x8_dir[idx_0 as usize][(*a).i_predict8x8[idx_0 as usize] as usize]
                    as c_int
                    * 11 as c_int
                    / 8 as c_int
            } else {
                COST_MAX
            };
            i_best = COST_MAX64 as uint64_t;
            let mut predict_mode_2: *const int8_t = predict_8x8_mode_available(
                (*a).b_avoid_topright,
                (*h).mb.i_neighbour8[idx_0 as usize] as c_int,
                idx_0,
            );
            let mut p_2: c_int = 0 as c_int;
            while p_2 < plane_count {
                (*h).predict_8x8_filter.expect("non-null function pointer")(
                    dst_0[p_2 as usize],
                    (*edge.as_mut_ptr().offset(p_2 as isize)).as_mut_ptr(),
                    (*h).mb.i_neighbour8[idx_0 as usize] as c_int,
                    ALL_NEIGHBORS as c_int,
                );
                p_2 += 1;
            }
            while *predict_mode_2 as c_int >= 0 as c_int {
                let mut i_mode_3: c_int = *predict_mode_2 as c_int;
                if !((*a).i_satd_i8x8_dir[idx_0 as usize][i_mode_3 as usize] as c_int > i_thresh_1)
                {
                    (*h).mb.i_cbp_luma = (*a).i_cbp_i8x8_luma;
                    i_satd = rd_cost_i8x8(h, (*a).i_lambda2, idx_0, i_mode_3, edge.as_mut_ptr());
                    if i_best > i_satd {
                        (*a).i_predict8x8[idx_0 as usize] = i_mode_3;
                        cbp_luma_new = (*h).mb.i_cbp_luma;
                        i_best = i_satd;
                        let mut p_3: c_int = 0 as c_int;
                        while p_3 < plane_count {
                            pels_h[p_3 as usize][0] = (*(dst_0[p_3 as usize]
                                .offset((7 as c_int * 32 as c_int) as isize)
                                .offset(0)
                                as *mut x264_union64_t))
                                .i as pixel4;
                            pels_h[p_3 as usize][1] = (*(dst_0[p_3 as usize]
                                .offset((7 as c_int * 32 as c_int) as isize)
                                .offset(4)
                                as *mut x264_union64_t))
                                .i as pixel4;
                            if idx_0 & 1 as c_int == 0 {
                                let mut j: c_int = 0 as c_int;
                                while j < 7 as c_int {
                                    pels_v[p_3 as usize][j as usize] = *dst_0[p_3 as usize]
                                        .offset((7 as c_int + j * FDEC_STRIDE) as isize);
                                    j += 1;
                                }
                            }
                            nnz_0[p_3 as usize][0] =
                                (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                                    (s8 + 0 as c_int * 8 as c_int + p_3 * 16 as c_int) as isize,
                                ) as *mut uint8_t
                                    as *mut x264_union16_t))
                                    .i;
                            nnz_0[p_3 as usize][1] =
                                (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                                    (s8 + 1 as c_int * 8 as c_int + p_3 * 16 as c_int) as isize,
                                ) as *mut uint8_t
                                    as *mut x264_union16_t))
                                    .i;
                            p_3 += 1;
                        }
                    }
                }
                predict_mode_2 = predict_mode_2.offset(1);
            }
            (*a).i_cbp_i8x8_luma = cbp_luma_new;
            let mut p_4: c_int = 0 as c_int;
            while p_4 < plane_count {
                (*(dst_0[p_4 as usize]
                    .offset((7 as c_int * 32 as c_int) as isize)
                    .offset(0) as *mut x264_union64_t))
                    .i = pels_h[p_4 as usize][0] as uint64_t;
                (*(dst_0[p_4 as usize]
                    .offset((7 as c_int * 32 as c_int) as isize)
                    .offset(4) as *mut x264_union64_t))
                    .i = pels_h[p_4 as usize][1] as uint64_t;
                if idx_0 & 1 as c_int == 0 {
                    let mut j_0: c_int = 0 as c_int;
                    while j_0 < 7 as c_int {
                        *dst_0[p_4 as usize].offset((7 as c_int + j_0 * FDEC_STRIDE) as isize) =
                            pels_v[p_4 as usize][j_0 as usize];
                        j_0 += 1;
                    }
                }
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset((s8 + 0 as c_int * 8 as c_int + p_4 * 16 as c_int) as isize)
                    as *mut uint8_t as *mut x264_union16_t))
                    .i = nnz_0[p_4 as usize][0];
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset((s8 + 1 as c_int * 8 as c_int + p_4 * 16 as c_int) as isize)
                    as *mut uint8_t as *mut x264_union16_t))
                    .i = nnz_0[p_4 as usize][1];
                p_4 += 1;
            }
            x264_macroblock_cache_intra8x8_pred(
                h,
                2 as c_int * x,
                2 as c_int * y,
                (*a).i_predict8x8[idx_0 as usize],
            );
            idx_0 += 1;
        }
    }
}
#[c2rust::src_loc = "1255:1"]
unsafe extern "C" fn mb_analyse_inter_p16x16(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    let mut m: x264_me_t = x264_me_t {
        i_pixel: 0,
        p_cost_mv: 0 as *mut uint16_t,
        i_ref_cost: 0,
        i_ref: 0,
        weight: 0 as *const x264_weight_t,
        p_fref: [0 as *mut pixel; 12],
        p_fref_w: 0 as *mut pixel,
        p_fenc: [0 as *mut pixel; 3],
        integral: 0 as *mut uint16_t,
        i_stride: [0; 3],
        mvp: [0; 2],
        cost_mv: 0,
        cost: 0,
        mv: [0; 2],
    };
    let mut i_mvc: c_int = 0;
    let mut mvc: [[int16_t; 2]; 8] = [[0; 2]; 8];
    let mut i_halfpel_thresh: c_int = INT_MAX;
    let mut p_halfpel_thresh: *mut c_int =
        if (*a).b_early_terminate != 0 && (*h).mb.pic.i_fref[0] > 1 as c_int {
            &mut i_halfpel_thresh
        } else {
            0 as *mut c_int
        };
    m.i_pixel = PIXEL_16x16 as c_int;
    m.p_cost_mv = (*a).p_cost_mv;
    m.i_stride[0] = (*h).mb.pic.i_stride[0];
    m.i_stride[1] = (*h).mb.pic.i_stride[1];
    m.i_stride[2] = (*h).mb.pic.i_stride[2];
    m.p_fenc[0] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(0))
        .offset((0 as c_int + 0 as c_int * FENC_STRIDE) as isize) as *mut pixel;
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        m.p_fenc[1] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(1)).offset(
            ((0 as c_int >> (*h).mb.chroma_h_shift)
                + (0 as c_int >> (*h).mb.chroma_v_shift) * FENC_STRIDE) as isize,
        ) as *mut pixel;
        m.p_fenc[2] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(2)).offset(
            ((0 as c_int >> (*h).mb.chroma_h_shift)
                + (0 as c_int >> (*h).mb.chroma_v_shift) * FENC_STRIDE) as isize,
        ) as *mut pixel;
    }
    (*a).l0.me16x16.cost = INT_MAX;
    let mut i_ref: c_int = 0 as c_int;
    while i_ref < (*h).mb.pic.i_fref[0] {
        m.i_ref_cost = *(*a).p_cost_ref[0].offset(i_ref as isize) as c_int;
        i_halfpel_thresh -= m.i_ref_cost;
        m.p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset(i_ref as isize))
        .as_mut_ptr()
        .offset(0))
        .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
            as *mut pixel;
        m.p_fref_w = m.p_fref[0];
        if (*h).param.analyse.i_subpel_refine != 0 {
            m.p_fref[1] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(1))
            .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                as *mut pixel;
            m.p_fref[2] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(2))
            .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                as *mut pixel;
            m.p_fref[3] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(3))
            .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                as *mut pixel;
        }
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(4))
            .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize)
                as *mut pixel;
            m.p_fref[8] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(8))
            .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize)
                as *mut pixel;
            if (*h).param.analyse.i_subpel_refine != 0 {
                m.p_fref[5] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(5))
                .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize)
                    as *mut pixel;
                m.p_fref[6] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(6))
                .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize)
                    as *mut pixel;
                m.p_fref[7] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(7))
                .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize)
                    as *mut pixel;
                m.p_fref[9] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(9 as c_int as isize))
                .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize)
                    as *mut pixel;
                m.p_fref[10] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(10 as c_int as isize))
                .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize)
                    as *mut pixel;
                m.p_fref[11] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(11 as c_int as isize))
                .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize)
                    as *mut pixel;
            }
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(4))
            .offset(
                (0 as c_int
                    + (0 as c_int >> (*h).mb.chroma_v_shift) * *m.i_stride.as_mut_ptr().offset(1))
                    as isize,
            ) as *mut pixel;
        }
        if (*h).param.analyse.me_method.exhaustive_search() {
            m.integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                as *mut uint16_t;
        }
        m.weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
        m.i_ref = i_ref;
        m.p_fref_w = &mut *(*(*h).mb.pic.p_fref_w.as_mut_ptr().offset(i_ref as isize))
            .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
            as *mut pixel;
        m.weight = (*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr();
        x264_10_mb_predict_mv_16x16(h, 0 as c_int, i_ref, m.mvp.as_mut_ptr());
        if (*h).mb.ref_blind_dupe == i_ref {
            (*(m.mv.as_mut_ptr() as *mut x264_union32_t)).i =
                (*((*(*(*a).l0.mvc.as_mut_ptr().offset(0)).as_mut_ptr().offset(0)).as_mut_ptr()
                    as *mut x264_union32_t))
                    .i;
            x264_10_me_refine_qpel_refdupe(h, &mut m, p_halfpel_thresh);
        } else {
            x264_10_mb_predict_mv_ref16x16(
                h,
                0 as c_int,
                i_ref,
                mvc.as_mut_ptr() as *mut [int16_t; 2],
                &mut i_mvc,
            );
            x264_10_me_search_ref(
                h,
                &mut m,
                mvc.as_mut_ptr() as *mut [int16_t; 2],
                i_mvc,
                p_halfpel_thresh,
            );
        }
        (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset(i_ref as isize))
        .offset((*h).mb.i_mb_xy as isize))
        .as_mut_ptr() as *mut x264_union32_t))
            .i = (*(m.mv.as_mut_ptr() as *mut x264_union32_t)).i;
        (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize))
            .as_mut_ptr()
            .offset(0))
        .as_mut_ptr() as *mut x264_union32_t))
            .i = (*(m.mv.as_mut_ptr() as *mut x264_union32_t)).i;
        if i_ref == 0 as c_int
            && (*a).b_try_skip != 0
            && m.cost - m.cost_mv < 300 as c_int * (*a).i_lambda
            && abs(m.mv[0] as c_int - (*h).mb.cache.pskip_mv[0] as c_int)
                + abs(m.mv[1] as c_int - (*h).mb.cache.pskip_mv[1] as c_int)
                <= 1 as c_int
            && x264_10_macroblock_probe_skip(h, 0 as c_int) != 0
        {
            (*h).mb.i_type = P_SKIP as c_int;
            analyse_update_cache(h, a);
            if (*h).mb.cache.pskip_mv[1] as c_int <= (*h).mb.mv_max_spel[1]
                || (*h).i_thread_frames == 1 as c_int
            {
            } else {
                __assert_fail(
                    b"h->mb.cache.pskip_mv[1] <= h->mb.mv_max_spel[1] || h->i_thread_frames == 1\0"
                        as *const u8 as *const c_char,
                    b"encoder/analyse.c\0" as *const u8 as *const c_char,
                    1305 as c_uint,
                    ::core::mem::transmute::<[u8; 61], [c_char; 61]>(
                        *b"void mb_analyse_inter_p16x16(x264_t *, x264_mb_analysis_t *)\0",
                    )
                    .as_ptr(),
                );
            }
            return;
        }
        m.cost += m.i_ref_cost;
        i_halfpel_thresh += m.i_ref_cost;
        if m.cost < (*a).l0.me16x16.cost {
            (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                &mut (*a).l0.me16x16 as *mut x264_me_t as *mut c_void,
                &mut m as *mut x264_me_t as *const c_void,
                size_of::<x264_me_t>() as size_t,
            );
        }
        i_ref += 1;
    }
    x264_macroblock_cache_ref(
        h,
        0 as c_int,
        0 as c_int,
        4 as c_int,
        4 as c_int,
        0 as c_int,
        (*a).l0.me16x16.i_ref as int8_t,
    );
    if (*a).l0.me16x16.mv[1] as c_int <= (*h).mb.mv_max_spel[1]
        || (*h).i_thread_frames == 1 as c_int
    {
    } else {
        __assert_fail(
            b"a->l0.me16x16.mv[1] <= h->mb.mv_max_spel[1] || h->i_thread_frames == 1\0" as *const u8
                as *const c_char,
            b"encoder/analyse.c\0" as *const u8 as *const c_char,
            1317 as c_uint,
            ::core::mem::transmute::<[u8; 61], [c_char; 61]>(
                *b"void mb_analyse_inter_p16x16(x264_t *, x264_mb_analysis_t *)\0",
            )
            .as_ptr(),
        );
    }
    (*h).mb.i_type = P_L0 as c_int;
    if (*a).i_mbrd != 0 {
        mb_init_fenc_cache(
            h,
            ((*a).i_mbrd >= 2 as c_int || (*h).param.analyse.inter & X264_ANALYSE_PSUB8x8 != 0)
                as c_int,
        );
        if (*a).l0.me16x16.i_ref == 0 as c_int
            && (*((*a).l0.me16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i
                == (*((*h).mb.cache.pskip_mv.as_mut_ptr() as *mut x264_union32_t)).i
            && (*a).b_force_intra == 0
        {
            (*h).mb.i_partition = D_16x16 as c_int;
            x264_macroblock_cache_mv(
                h,
                0 as c_int,
                0 as c_int,
                4 as c_int,
                4 as c_int,
                0 as c_int,
                (*((*a).l0.me16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i,
            );
            (*a).l0.i_rd16x16 = rd_cost_mb(h, (*a).i_lambda2);
            if (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0 {
                (*h).mb.i_type = P_SKIP as c_int;
            }
        }
    }
}
#[c2rust::src_loc = "1334:1"]
unsafe extern "C" fn mb_analyse_inter_p8x8_mixed_ref(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
) {
    let mut m: x264_me_t = x264_me_t {
        i_pixel: 0,
        p_cost_mv: 0 as *mut uint16_t,
        i_ref_cost: 0,
        i_ref: 0,
        weight: 0 as *const x264_weight_t,
        p_fref: [0 as *mut pixel; 12],
        p_fref_w: 0 as *mut pixel,
        p_fenc: [0 as *mut pixel; 3],
        integral: 0 as *mut uint16_t,
        i_stride: [0; 3],
        mvp: [0; 2],
        cost_mv: 0,
        cost: 0,
        mv: [0; 2],
    };
    let mut p_fenc: *mut *mut pixel = (*h).mb.pic.p_fenc.as_mut_ptr();
    let mut i_maxref: c_int = (*h).mb.pic.i_fref[0] - 1 as c_int;
    (*h).mb.i_partition = D_8x8 as c_int;
    if (*a).b_early_terminate != 0
        && (i_maxref > 0 as c_int
            && ((*a).l0.me16x16.i_ref == 0 as c_int
                || (*a).l0.me16x16.i_ref == (*h).mb.ref_blind_dupe)
            && (*h).mb.i_mb_type_top > 0 as c_int
            && (*h).mb.i_mb_type_left[0] > 0 as c_int)
    {
        i_maxref = 0 as c_int;
        let mut ref_0: c_int =
            (*h).mb.cache.ref_0[0][(X264_SCAN8_0 + -(8 as c_int) - 1 as c_int) as usize] as c_int;
        if ref_0 > i_maxref && ref_0 != (*h).mb.ref_blind_dupe {
            i_maxref = ref_0;
        }
        let mut ref_1: c_int =
            (*h).mb.cache.ref_0[0][(X264_SCAN8_0 + -(8 as c_int) + 0 as c_int) as usize] as c_int;
        if ref_1 > i_maxref && ref_1 != (*h).mb.ref_blind_dupe {
            i_maxref = ref_1;
        }
        let mut ref_2: c_int =
            (*h).mb.cache.ref_0[0][(X264_SCAN8_0 + -(8 as c_int) + 2 as c_int) as usize] as c_int;
        if ref_2 > i_maxref && ref_2 != (*h).mb.ref_blind_dupe {
            i_maxref = ref_2;
        }
        let mut ref_3: c_int =
            (*h).mb.cache.ref_0[0][(X264_SCAN8_0 + -(8 as c_int) + 4 as c_int) as usize] as c_int;
        if ref_3 > i_maxref && ref_3 != (*h).mb.ref_blind_dupe {
            i_maxref = ref_3;
        }
        let mut ref_4: c_int =
            (*h).mb.cache.ref_0[0][(X264_SCAN8_0 + 0 as c_int - 1 as c_int) as usize] as c_int;
        if ref_4 > i_maxref && ref_4 != (*h).mb.ref_blind_dupe {
            i_maxref = ref_4;
        }
        let mut ref_5: c_int = (*h).mb.cache.ref_0[0]
            [(X264_SCAN8_0 + 2 as c_int * 8 as c_int - 1 as c_int) as usize]
            as c_int;
        if ref_5 > i_maxref && ref_5 != (*h).mb.ref_blind_dupe {
            i_maxref = ref_5;
        }
    }
    let mut i_ref: c_int = 0 as c_int;
    while i_ref <= i_maxref {
        (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize))
            .as_mut_ptr()
            .offset(0))
        .as_mut_ptr() as *mut x264_union32_t))
            .i = (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset(i_ref as isize))
        .offset((*h).mb.i_mb_xy as isize))
        .as_mut_ptr() as *mut x264_union32_t))
            .i;
        i_ref += 1;
    }
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        let mut l0m: *mut x264_me_t =
            &mut *(*a).l0.me8x8.as_mut_ptr().offset(i as isize) as *mut x264_me_t;
        let mut x8: c_int = i & 1 as c_int;
        let mut y8: c_int = i >> 1 as c_int;
        m.i_pixel = PIXEL_8x8 as c_int;
        m.p_cost_mv = (*a).p_cost_mv;
        m.i_stride[0] = (*h).mb.pic.i_stride[0];
        m.i_stride[1] = (*h).mb.pic.i_stride[1];
        m.i_stride[2] = (*h).mb.pic.i_stride[2];
        m.p_fenc[0] = &mut *(*p_fenc.offset(0))
            .offset((8 as c_int * x8 + 8 as c_int * y8 * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            m.p_fenc[1] = &mut *(*p_fenc.offset(1)).offset(
                ((8 as c_int * x8 >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * y8 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            m.p_fenc[2] = &mut *(*p_fenc.offset(2)).offset(
                ((8 as c_int * x8 >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * y8 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        (*l0m).cost = INT_MAX;
        let mut i_ref_0: c_int = 0 as c_int;
        while i_ref_0 <= i_maxref || i_ref_0 == (*h).mb.ref_blind_dupe {
            m.i_ref_cost = *(*a).p_cost_ref[0].offset(i_ref_0 as isize) as c_int;
            m.p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref_0 as isize))
            .as_mut_ptr()
            .offset(0))
            .offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0)) as isize,
            ) as *mut pixel;
            m.p_fref_w = m.p_fref[0];
            if (*h).param.analyse.i_subpel_refine != 0 {
                m.p_fref[1] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref_0 as isize))
                .as_mut_ptr()
                .offset(1))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                        as isize,
                ) as *mut pixel;
                m.p_fref[2] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref_0 as isize))
                .as_mut_ptr()
                .offset(2))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                        as isize,
                ) as *mut pixel;
                m.p_fref[3] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref_0 as isize))
                .as_mut_ptr()
                .offset(3))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                        as isize,
                ) as *mut pixel;
            }
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref_0 as isize))
                .as_mut_ptr()
                .offset(4))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                m.p_fref[8] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref_0 as isize))
                .as_mut_ptr()
                .offset(8))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                if (*h).param.analyse.i_subpel_refine != 0 {
                    m.p_fref[5] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref_0 as isize))
                    .as_mut_ptr()
                    .offset(5))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(1))
                            as isize,
                    ) as *mut pixel;
                    m.p_fref[6] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref_0 as isize))
                    .as_mut_ptr()
                    .offset(6))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(1))
                            as isize,
                    ) as *mut pixel;
                    m.p_fref[7] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref_0 as isize))
                    .as_mut_ptr()
                    .offset(7))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(1))
                            as isize,
                    ) as *mut pixel;
                    m.p_fref[9] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref_0 as isize))
                    .as_mut_ptr()
                    .offset(9 as c_int as isize))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(2))
                            as isize,
                    ) as *mut pixel;
                    m.p_fref[10] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref_0 as isize))
                    .as_mut_ptr()
                    .offset(10 as c_int as isize))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(2))
                            as isize,
                    ) as *mut pixel;
                    m.p_fref[11] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref_0 as isize))
                    .as_mut_ptr()
                    .offset(11 as c_int as isize))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(2))
                            as isize,
                    ) as *mut pixel;
                }
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref_0 as isize))
                .as_mut_ptr()
                .offset(4))
                .offset(
                    (8 as c_int * x8
                        + (8 as c_int * y8 >> (*h).mb.chroma_v_shift)
                            * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                ) as *mut pixel;
            }
            if (*h).param.analyse.me_method.exhaustive_search() {
                m.integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref_0 as isize))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                        as isize,
                ) as *mut uint16_t;
            }
            m.weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
            m.i_ref = i_ref_0;
            m.p_fref_w = &mut *(*(*h).mb.pic.p_fref_w.as_mut_ptr().offset(i_ref_0 as isize)).offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0)) as isize,
            ) as *mut pixel;
            m.weight = (*(*h).sh.weight.as_mut_ptr().offset(i_ref_0 as isize)).as_mut_ptr();
            x264_macroblock_cache_ref(
                h,
                2 as c_int * x8,
                2 as c_int * y8,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                i_ref_0 as int8_t,
            );
            x264_10_mb_predict_mv(
                h,
                0 as c_int,
                4 as c_int * i,
                2 as c_int,
                m.mvp.as_mut_ptr(),
            );
            if (*h).mb.ref_blind_dupe == i_ref_0 {
                (*(m.mv.as_mut_ptr() as *mut x264_union32_t)).i =
                    (*((*(*(*a).l0.mvc.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset((i + 1 as c_int) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                        .i;
                x264_10_me_refine_qpel_refdupe(h, &mut m, 0 as *mut c_int);
            } else {
                x264_10_me_search_ref(
                    h,
                    &mut m,
                    (*(*a).l0.mvc.as_mut_ptr().offset(i_ref_0 as isize)).as_mut_ptr()
                        as *mut [int16_t; 2],
                    i + 1 as c_int,
                    0 as *mut c_int,
                );
            }
            m.cost += m.i_ref_cost;
            (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref_0 as isize))
                .as_mut_ptr()
                .offset((i + 1 as c_int) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*(m.mv.as_mut_ptr() as *mut x264_union32_t)).i;
            if m.cost < (*l0m).cost {
                (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                    l0m as *mut c_void,
                    &mut m as *mut x264_me_t as *const c_void,
                    size_of::<x264_me_t>() as size_t,
                );
            }
            if i_ref_0 == i_maxref && i_maxref < (*h).mb.ref_blind_dupe {
                i_ref_0 = (*h).mb.ref_blind_dupe;
            } else {
                i_ref_0 += 1;
            }
        }
        x264_macroblock_cache_mv(
            h,
            2 as c_int * x8,
            2 as c_int * y8,
            2 as c_int,
            2 as c_int,
            0 as c_int,
            (*((*l0m).mv.as_mut_ptr() as *mut x264_union32_t)).i,
        );
        x264_macroblock_cache_ref(
            h,
            2 as c_int * x8,
            2 as c_int * y8,
            2 as c_int,
            2 as c_int,
            0 as c_int,
            (*l0m).i_ref as int8_t,
        );
        (*a).i_satd8x8[0][i as usize] = (*l0m).cost - ((*l0m).cost_mv + (*l0m).i_ref_cost);
        if (*h).param.b_cabac == 0 || (*h).param.analyse.inter & X264_ANALYSE_PSUB8x8 != 0 {
            (*l0m).cost +=
                (*a).i_lambda * i_sub_mb_p_cost_table[D_L0_8x8 as c_int as usize] as c_int;
        }
        i += 1;
    }
    (*a).l0.i_cost8x8 = (*a).l0.me8x8[0].cost
        + (*a).l0.me8x8[1].cost
        + (*a).l0.me8x8[2].cost
        + (*a).l0.me8x8[3].cost;
    if (*h).param.b_cabac == 0
        && (*a).l0.me8x8[0].i_ref
            | (*a).l0.me8x8[1].i_ref
            | (*a).l0.me8x8[2].i_ref
            | (*a).l0.me8x8[3].i_ref
            == 0
    {
        (*a).l0.i_cost8x8 -= *(*a).p_cost_ref[0].offset(0) as c_int * 4 as c_int;
    }
    (*((*h).mb.i_sub_partition.as_mut_ptr() as *mut x264_union32_t)).i =
        (D_L0_8x8 as c_int * 0x1010101 as c_int) as uint32_t;
}
#[c2rust::src_loc = "1425:1"]
unsafe extern "C" fn mb_analyse_inter_p8x8(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    let i_ref: c_int = if (*h).mb.ref_blind_dupe == (*a).l0.me16x16.i_ref {
        0 as c_int
    } else {
        (*a).l0.me16x16.i_ref
    };
    let i_ref_cost: c_int = if (*h).param.b_cabac != 0 || i_ref != 0 {
        *(*a).p_cost_ref[0].offset(i_ref as isize) as c_int
    } else {
        0 as c_int
    };
    let mut p_fenc: *mut *mut pixel = (*h).mb.pic.p_fenc.as_mut_ptr();
    let mut i_mvc: c_int = 0;
    let mut mvc: *mut [int16_t; 2] =
        (*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr() as *mut [int16_t; 2];
    (*h).mb.i_partition = D_8x8 as c_int;
    i_mvc = 1 as c_int;
    (*((*mvc.offset(0)).as_mut_ptr() as *mut x264_union32_t)).i =
        (*((*a).l0.me16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i;
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        let mut m: *mut x264_me_t =
            &mut *(*a).l0.me8x8.as_mut_ptr().offset(i as isize) as *mut x264_me_t;
        let mut x8: c_int = i & 1 as c_int;
        let mut y8: c_int = i >> 1 as c_int;
        (*m).i_pixel = PIXEL_8x8 as c_int;
        (*m).i_ref_cost = i_ref_cost;
        (*m).p_cost_mv = (*a).p_cost_mv;
        (*m).i_stride[0] = (*h).mb.pic.i_stride[0];
        (*m).i_stride[1] = (*h).mb.pic.i_stride[1];
        (*m).i_stride[2] = (*h).mb.pic.i_stride[2];
        (*m).p_fenc[0] = &mut *(*p_fenc.offset(0))
            .offset((8 as c_int * x8 + 8 as c_int * y8 * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*m).p_fenc[1] = &mut *(*p_fenc.offset(1)).offset(
                ((8 as c_int * x8 >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * y8 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            (*m).p_fenc[2] = &mut *(*p_fenc.offset(2)).offset(
                ((8 as c_int * x8 >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * y8 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        (*m).p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset(i_ref as isize))
        .as_mut_ptr()
        .offset(0))
        .offset(
            (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0)) as isize,
        ) as *mut pixel;
        (*m).p_fref_w = (*m).p_fref[0];
        if (*h).param.analyse.i_subpel_refine != 0 {
            (*m).p_fref[1] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(1))
            .offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[2] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(2))
            .offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[3] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(3))
            .offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
        }
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            (*m).p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(4))
            .offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(1))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[8] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(8))
            .offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(2))
                    as isize,
            ) as *mut pixel;
            if (*h).param.analyse.i_subpel_refine != 0 {
                (*m).p_fref[5] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(5))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[6] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(6))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[7] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(7))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[9] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(9 as c_int as isize))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[10] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(10 as c_int as isize))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[11] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(11 as c_int as isize))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
            }
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*m).p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(4))
            .offset(
                (8 as c_int * x8
                    + (8 as c_int * y8 >> (*h).mb.chroma_v_shift)
                        * *(*m).i_stride.as_mut_ptr().offset(1)) as isize,
            ) as *mut pixel;
        }
        if (*h).param.analyse.me_method.exhaustive_search() {
            (*m).integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut uint16_t;
        }
        (*m).weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
        (*m).i_ref = i_ref;
        (*m).p_fref_w = &mut *(*(*h).mb.pic.p_fref_w.as_mut_ptr().offset(i_ref as isize)).offset(
            (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0)) as isize,
        ) as *mut pixel;
        (*m).weight = (*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr();
        x264_10_mb_predict_mv(
            h,
            0 as c_int,
            4 as c_int * i,
            2 as c_int,
            (*m).mvp.as_mut_ptr(),
        );
        x264_10_me_search_ref(h, m, mvc, i_mvc, 0 as *mut c_int);
        x264_macroblock_cache_mv(
            h,
            2 as c_int * x8,
            2 as c_int * y8,
            2 as c_int,
            2 as c_int,
            0 as c_int,
            (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i,
        );
        (*((*mvc.offset(i_mvc as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i;
        i_mvc += 1;
        (*a).i_satd8x8[0][i as usize] = (*m).cost - (*m).cost_mv;
        (*m).cost += i_ref_cost;
        if (*h).param.b_cabac == 0 || (*h).param.analyse.inter & X264_ANALYSE_PSUB8x8 != 0 {
            (*m).cost += (*a).i_lambda * i_sub_mb_p_cost_table[D_L0_8x8 as c_int as usize] as c_int;
        }
        i += 1;
    }
    (*a).l0.i_cost8x8 = (*a).l0.me8x8[0].cost
        + (*a).l0.me8x8[1].cost
        + (*a).l0.me8x8[2].cost
        + (*a).l0.me8x8[3].cost;
    if (*h).param.b_cabac != 0 {
        (*a).l0.i_cost8x8 -= i_ref_cost;
    }
    (*((*h).mb.i_sub_partition.as_mut_ptr() as *mut x264_union32_t)).i =
        (D_L0_8x8 as c_int * 0x1010101 as c_int) as uint32_t;
}
#[c2rust::src_loc = "1480:1"]
unsafe extern "C" fn mb_analyse_inter_p16x8(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_best_satd: c_int,
) {
    let mut m: x264_me_t = x264_me_t {
        i_pixel: 0,
        p_cost_mv: 0 as *mut uint16_t,
        i_ref_cost: 0,
        i_ref: 0,
        weight: 0 as *const x264_weight_t,
        p_fref: [0 as *mut pixel; 12],
        p_fref_w: 0 as *mut pixel,
        p_fenc: [0 as *mut pixel; 3],
        integral: 0 as *mut uint16_t,
        i_stride: [0; 3],
        mvp: [0; 2],
        cost_mv: 0,
        cost: 0,
        mv: [0; 2],
    };
    let mut p_fenc: *mut *mut pixel = (*h).mb.pic.p_fenc.as_mut_ptr();
    let mut mvc: [[int16_t; 2]; 3] = [[0; 2]; 3];
    (*h).mb.i_partition = D_16x8 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 2 as c_int {
        let mut l0m: *mut x264_me_t =
            &mut *(*a).l0.me16x8.as_mut_ptr().offset(i as isize) as *mut x264_me_t;
        let minref: c_int = if (*a).l0.me8x8[(2 as c_int * i) as usize].i_ref
            < (*a).l0.me8x8[(2 as c_int * i + 1 as c_int) as usize].i_ref
        {
            (*a).l0.me8x8[(2 as c_int * i) as usize].i_ref
        } else {
            (*a).l0.me8x8[(2 as c_int * i + 1 as c_int) as usize].i_ref
        };
        let maxref: c_int = if (*a).l0.me8x8[(2 as c_int * i) as usize].i_ref
            > (*a).l0.me8x8[(2 as c_int * i + 1 as c_int) as usize].i_ref
        {
            (*a).l0.me8x8[(2 as c_int * i) as usize].i_ref
        } else {
            (*a).l0.me8x8[(2 as c_int * i + 1 as c_int) as usize].i_ref
        };
        let ref8: [c_int; 2] = [minref, maxref];
        let i_ref8s: c_int = if ref8[0] == ref8[1] {
            1 as c_int
        } else {
            2 as c_int
        };
        m.i_pixel = PIXEL_16x8 as c_int;
        m.p_cost_mv = (*a).p_cost_mv;
        m.i_stride[0] = (*h).mb.pic.i_stride[0];
        m.i_stride[1] = (*h).mb.pic.i_stride[1];
        m.i_stride[2] = (*h).mb.pic.i_stride[2];
        m.p_fenc[0] = &mut *(*p_fenc.offset(0))
            .offset((0 as c_int + 8 as c_int * i * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            m.p_fenc[1] = &mut *(*p_fenc.offset(1)).offset(
                ((0 as c_int >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * i >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            m.p_fenc[2] = &mut *(*p_fenc.offset(2)).offset(
                ((0 as c_int >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * i >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        (*l0m).cost = INT_MAX;
        let mut j: c_int = 0 as c_int;
        while j < i_ref8s {
            let i_ref: c_int = ref8[j as usize];
            m.i_ref_cost = *(*a).p_cost_ref[0].offset(i_ref as isize) as c_int;
            (*((*mvc.as_mut_ptr().offset(0)).as_mut_ptr() as *mut x264_union32_t)).i =
                (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(0))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            (*((*mvc.as_mut_ptr().offset(1)).as_mut_ptr() as *mut x264_union32_t)).i =
                (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset((2 as c_int * i + 1 as c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            (*((*mvc.as_mut_ptr().offset(2)).as_mut_ptr() as *mut x264_union32_t)).i =
                (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset((2 as c_int * i + 2 as c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            m.p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(0))
            .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                as *mut pixel;
            m.p_fref_w = m.p_fref[0];
            if (*h).param.analyse.i_subpel_refine != 0 {
                m.p_fref[1] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(1))
                .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
                m.p_fref[2] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(2))
                .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
                m.p_fref[3] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(3))
                .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
            }
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4))
                .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(1)) as isize)
                    as *mut pixel;
                m.p_fref[8] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8))
                .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(2)) as isize)
                    as *mut pixel;
                if (*h).param.analyse.i_subpel_refine != 0 {
                    m.p_fref[5] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(5))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                    ) as *mut pixel;
                    m.p_fref[6] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(6))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                    ) as *mut pixel;
                    m.p_fref[7] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(7))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                    ) as *mut pixel;
                    m.p_fref[9] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(9 as c_int as isize))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(2)) as isize,
                    ) as *mut pixel;
                    m.p_fref[10] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(10 as c_int as isize))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(2)) as isize,
                    ) as *mut pixel;
                    m.p_fref[11] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(11 as c_int as isize))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(2)) as isize,
                    ) as *mut pixel;
                }
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4))
                .offset(
                    (0 as c_int
                        + (8 as c_int * i >> (*h).mb.chroma_v_shift)
                            * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                ) as *mut pixel;
            }
            if (*h).param.analyse.me_method.exhaustive_search() {
                m.integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut uint16_t;
            }
            m.weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
            m.i_ref = i_ref;
            m.p_fref_w = &mut *(*(*h).mb.pic.p_fref_w.as_mut_ptr().offset(i_ref as isize))
                .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                as *mut pixel;
            m.weight = (*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr();
            x264_macroblock_cache_ref(
                h,
                0 as c_int,
                2 as c_int * i,
                4 as c_int,
                2 as c_int,
                0 as c_int,
                i_ref as int8_t,
            );
            x264_10_mb_predict_mv(
                h,
                0 as c_int,
                8 as c_int * i,
                4 as c_int,
                m.mvp.as_mut_ptr(),
            );
            if (*h).mb.ref_blind_dupe == i_ref && ref8[0] == 0 {
                x264_10_me_refine_qpel_refdupe(h, &mut m, 0 as *mut c_int);
            } else {
                x264_10_me_search_ref(
                    h,
                    &mut m,
                    mvc.as_mut_ptr() as *mut [int16_t; 2],
                    3 as c_int,
                    0 as *mut c_int,
                );
            }
            m.cost += m.i_ref_cost;
            if m.cost < (*l0m).cost {
                (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                    l0m as *mut c_void,
                    &mut m as *mut x264_me_t as *const c_void,
                    size_of::<x264_me_t>() as size_t,
                );
            }
            j += 1;
        }
        if (*a).b_early_terminate != 0
            && (i == 0
                && (*l0m).cost + (*a).i_cost_est16x8[1]
                    > i_best_satd * (4 as c_int + ((*a).i_mbrd != 0) as c_int) / 4 as c_int)
        {
            (*a).l0.i_cost16x8 = COST_MAX;
            return;
        }
        x264_macroblock_cache_mv(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            0 as c_int,
            (*((*l0m).mv.as_mut_ptr() as *mut x264_union32_t)).i,
        );
        x264_macroblock_cache_ref(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            0 as c_int,
            (*l0m).i_ref as int8_t,
        );
        i += 1;
    }
    (*a).l0.i_cost16x8 = (*a).l0.me16x8[0].cost + (*a).l0.me16x8[1].cost;
}
#[c2rust::src_loc = "1546:1"]
unsafe extern "C" fn mb_analyse_inter_p8x16(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_best_satd: c_int,
) {
    let mut m: x264_me_t = x264_me_t {
        i_pixel: 0,
        p_cost_mv: 0 as *mut uint16_t,
        i_ref_cost: 0,
        i_ref: 0,
        weight: 0 as *const x264_weight_t,
        p_fref: [0 as *mut pixel; 12],
        p_fref_w: 0 as *mut pixel,
        p_fenc: [0 as *mut pixel; 3],
        integral: 0 as *mut uint16_t,
        i_stride: [0; 3],
        mvp: [0; 2],
        cost_mv: 0,
        cost: 0,
        mv: [0; 2],
    };
    let mut p_fenc: *mut *mut pixel = (*h).mb.pic.p_fenc.as_mut_ptr();
    let mut mvc: [[int16_t; 2]; 3] = [[0; 2]; 3];
    (*h).mb.i_partition = D_8x16 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 2 as c_int {
        let mut l0m: *mut x264_me_t =
            &mut *(*a).l0.me8x16.as_mut_ptr().offset(i as isize) as *mut x264_me_t;
        let minref: c_int =
            if (*a).l0.me8x8[i as usize].i_ref < (*a).l0.me8x8[(i + 2 as c_int) as usize].i_ref {
                (*a).l0.me8x8[i as usize].i_ref
            } else {
                (*a).l0.me8x8[(i + 2 as c_int) as usize].i_ref
            };
        let maxref: c_int =
            if (*a).l0.me8x8[i as usize].i_ref > (*a).l0.me8x8[(i + 2 as c_int) as usize].i_ref {
                (*a).l0.me8x8[i as usize].i_ref
            } else {
                (*a).l0.me8x8[(i + 2 as c_int) as usize].i_ref
            };
        let ref8: [c_int; 2] = [minref, maxref];
        let i_ref8s: c_int = if ref8[0] == ref8[1] {
            1 as c_int
        } else {
            2 as c_int
        };
        m.i_pixel = PIXEL_8x16 as c_int;
        m.p_cost_mv = (*a).p_cost_mv;
        m.i_stride[0] = (*h).mb.pic.i_stride[0];
        m.i_stride[1] = (*h).mb.pic.i_stride[1];
        m.i_stride[2] = (*h).mb.pic.i_stride[2];
        m.p_fenc[0] = &mut *(*p_fenc.offset(0))
            .offset((8 as c_int * i + 0 as c_int * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            m.p_fenc[1] = &mut *(*p_fenc.offset(1)).offset(
                ((8 as c_int * i >> (*h).mb.chroma_h_shift)
                    + (0 as c_int >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            m.p_fenc[2] = &mut *(*p_fenc.offset(2)).offset(
                ((8 as c_int * i >> (*h).mb.chroma_h_shift)
                    + (0 as c_int >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        (*l0m).cost = INT_MAX;
        let mut j: c_int = 0 as c_int;
        while j < i_ref8s {
            let i_ref: c_int = ref8[j as usize];
            m.i_ref_cost = *(*a).p_cost_ref[0].offset(i_ref as isize) as c_int;
            (*((*mvc.as_mut_ptr().offset(0)).as_mut_ptr() as *mut x264_union32_t)).i =
                (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(0))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            (*((*mvc.as_mut_ptr().offset(1)).as_mut_ptr() as *mut x264_union32_t)).i =
                (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset((i + 1 as c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            (*((*mvc.as_mut_ptr().offset(2)).as_mut_ptr() as *mut x264_union32_t)).i =
                (*((*(*(*a).l0.mvc.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset((i + 3 as c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            m.p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .as_mut_ptr()
            .offset(0))
            .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                as *mut pixel;
            m.p_fref_w = m.p_fref[0];
            if (*h).param.analyse.i_subpel_refine != 0 {
                m.p_fref[1] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(1))
                .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
                m.p_fref[2] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(2))
                .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
                m.p_fref[3] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(3))
                .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
            }
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4))
                .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize)
                    as *mut pixel;
                m.p_fref[8] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8))
                .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize)
                    as *mut pixel;
                if (*h).param.analyse.i_subpel_refine != 0 {
                    m.p_fref[5] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(5))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                    ) as *mut pixel;
                    m.p_fref[6] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(6))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                    ) as *mut pixel;
                    m.p_fref[7] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(7))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                    ) as *mut pixel;
                    m.p_fref[9] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(9 as c_int as isize))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize,
                    ) as *mut pixel;
                    m.p_fref[10] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(10 as c_int as isize))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize,
                    ) as *mut pixel;
                    m.p_fref[11] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(11 as c_int as isize))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize,
                    ) as *mut pixel;
                }
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4))
                .offset(
                    (8 as c_int * i
                        + (0 as c_int >> (*h).mb.chroma_v_shift)
                            * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                ) as *mut pixel;
            }
            if (*h).param.analyse.me_method.exhaustive_search() {
                m.integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut uint16_t;
            }
            m.weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
            m.i_ref = i_ref;
            m.p_fref_w = &mut *(*(*h).mb.pic.p_fref_w.as_mut_ptr().offset(i_ref as isize))
                .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                as *mut pixel;
            m.weight = (*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr();
            x264_macroblock_cache_ref(
                h,
                2 as c_int * i,
                0 as c_int,
                2 as c_int,
                4 as c_int,
                0 as c_int,
                i_ref as int8_t,
            );
            x264_10_mb_predict_mv(
                h,
                0 as c_int,
                4 as c_int * i,
                2 as c_int,
                m.mvp.as_mut_ptr(),
            );
            if (*h).mb.ref_blind_dupe == i_ref && ref8[0] == 0 {
                x264_10_me_refine_qpel_refdupe(h, &mut m, 0 as *mut c_int);
            } else {
                x264_10_me_search_ref(
                    h,
                    &mut m,
                    mvc.as_mut_ptr() as *mut [int16_t; 2],
                    3 as c_int,
                    0 as *mut c_int,
                );
            }
            m.cost += m.i_ref_cost;
            if m.cost < (*l0m).cost {
                (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                    l0m as *mut c_void,
                    &mut m as *mut x264_me_t as *const c_void,
                    size_of::<x264_me_t>() as size_t,
                );
            }
            j += 1;
        }
        if (*a).b_early_terminate != 0
            && (i == 0
                && (*l0m).cost + (*a).i_cost_est8x16[1]
                    > i_best_satd * (4 as c_int + ((*a).i_mbrd != 0) as c_int) / 4 as c_int)
        {
            (*a).l0.i_cost8x16 = COST_MAX;
            return;
        }
        x264_macroblock_cache_mv(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            0 as c_int,
            (*((*l0m).mv.as_mut_ptr() as *mut x264_union32_t)).i,
        );
        x264_macroblock_cache_ref(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            0 as c_int,
            (*l0m).i_ref as int8_t,
        );
        i += 1;
    }
    (*a).l0.i_cost8x16 = (*a).l0.me8x16[0].cost + (*a).l0.me8x16[1].cost;
}
#[inline(always)]
#[c2rust::src_loc = "1611:1"]
unsafe extern "C" fn mb_analyse_inter_p4x4_chroma_internal(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut p_fref: *mut *mut pixel,
    mut i8x8: c_int,
    mut size: c_int,
    mut chroma: c_int,
) -> c_int {
    let mut pix1: [pixel; 256] = [0; 256];
    let mut pix2: *mut pixel = pix1.as_mut_ptr().offset(8);
    let mut i_stride: c_int = (*h).mb.pic.i_stride[1];
    let mut chroma_h_shift: c_int = (chroma <= CHROMA_422 as c_int) as c_int;
    let mut chroma_v_shift: c_int = (chroma == CHROMA_420 as c_int) as c_int;
    let mut or: c_int = 8 as c_int * (i8x8 & 1 as c_int)
        + (4 as c_int >> chroma_v_shift) * (i8x8 & 2 as c_int) * i_stride;
    let mut i_ref: c_int = (*a).l0.me8x8[i8x8 as usize].i_ref;
    let mut mvy_offset: c_int = if chroma_v_shift != 0 && (*h).mb.b_interlaced & i_ref != 0 {
        ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
    } else {
        0 as c_int
    };
    let mut weight: *mut x264_weight_t =
        (*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr();
    if size == PIXEL_4x4 as c_int {
        let mut m: *mut x264_me_t =
            (*(*a).l0.me4x4.as_mut_ptr().offset(i8x8 as isize)).as_mut_ptr();
        if chroma == CHROMA_444 as c_int {
            let mut mvx: c_int =
                (*m.offset(0)).mv[0] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            let mut mvy: c_int =
                (*m.offset(0)).mv[1] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4),
                i_stride as intptr_t,
                mvx,
                mvy,
                2 as c_int * 2 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix2.offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8),
                i_stride as intptr_t,
                mvx,
                mvy,
                2 as c_int * 2 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2),
            );
        } else {
            let mut offset: c_int =
                0 as c_int + (2 as c_int >> chroma_v_shift) * 16 as c_int * 0 as c_int;
            let mut chroma_height: c_int = (2 as c_int >> chroma_v_shift) * 2 as c_int;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(offset as isize),
                &mut *pix2.offset(offset as isize),
                16 as intptr_t,
                &mut *(*p_fref.offset(4)).offset(
                    (or + 2 as c_int * 0 as c_int
                        + (2 as c_int >> chroma_v_shift) * 0 as c_int * i_stride)
                        as isize,
                ),
                i_stride as intptr_t,
                (*m.offset(0)).mv[0] as c_int,
                (2 as c_int >> chroma_v_shift) * ((*m.offset(0)).mv[1] as c_int + mvy_offset),
                2 as c_int,
                chroma_height,
            );
            if !(*weight.offset(1)).weightfn.is_null() {
                (*(*weight.offset(1))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix1.as_mut_ptr().offset(offset as isize),
                    16 as intptr_t,
                    &mut *pix1.as_mut_ptr().offset(offset as isize),
                    16 as intptr_t,
                    &mut *weight.offset(1),
                    chroma_height,
                );
            }
            if !(*weight.offset(2)).weightfn.is_null() {
                (*(*weight.offset(2))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix2.offset(offset as isize),
                    16 as intptr_t,
                    &mut *pix2.offset(offset as isize),
                    16 as intptr_t,
                    &mut *weight.offset(2),
                    chroma_height,
                );
            }
        }
        if chroma == CHROMA_444 as c_int {
            let mut mvx_0: c_int =
                (*m.offset(1)).mv[0] as c_int + 4 as c_int * 2 as c_int * 2 as c_int;
            let mut mvy_0: c_int =
                (*m.offset(1)).mv[1] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(
                    (2 as c_int * 2 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4),
                i_stride as intptr_t,
                mvx_0,
                mvy_0,
                2 as c_int * 2 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix2.offset(
                    (2 as c_int * 2 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8),
                i_stride as intptr_t,
                mvx_0,
                mvy_0,
                2 as c_int * 2 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2),
            );
        } else {
            let mut offset_0: c_int =
                2 as c_int + (2 as c_int >> chroma_v_shift) * 16 as c_int * 0 as c_int;
            let mut chroma_height_0: c_int = (2 as c_int >> chroma_v_shift) * 2 as c_int;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(offset_0 as isize),
                &mut *pix2.offset(offset_0 as isize),
                16 as intptr_t,
                &mut *(*p_fref.offset(4)).offset(
                    (or + 2 as c_int * 2 as c_int
                        + (2 as c_int >> chroma_v_shift) * 0 as c_int * i_stride)
                        as isize,
                ),
                i_stride as intptr_t,
                (*m.offset(1)).mv[0] as c_int,
                (2 as c_int >> chroma_v_shift) * ((*m.offset(1)).mv[1] as c_int + mvy_offset),
                2 as c_int,
                chroma_height_0,
            );
            if !(*weight.offset(1)).weightfn.is_null() {
                (*(*weight.offset(1))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix1.as_mut_ptr().offset(offset_0 as isize),
                    16 as intptr_t,
                    &mut *pix1.as_mut_ptr().offset(offset_0 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(1),
                    chroma_height_0,
                );
            }
            if !(*weight.offset(2)).weightfn.is_null() {
                (*(*weight.offset(2))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix2.offset(offset_0 as isize),
                    16 as intptr_t,
                    &mut *pix2.offset(offset_0 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(2),
                    chroma_height_0,
                );
            }
        }
        if chroma == CHROMA_444 as c_int {
            let mut mvx_1: c_int =
                (*m.offset(2)).mv[0] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            let mut mvy_1: c_int =
                (*m.offset(2)).mv[1] as c_int + 4 as c_int * 2 as c_int * 2 as c_int;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 2 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4),
                i_stride as intptr_t,
                mvx_1,
                mvy_1,
                2 as c_int * 2 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix2.offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 2 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8),
                i_stride as intptr_t,
                mvx_1,
                mvy_1,
                2 as c_int * 2 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2),
            );
        } else {
            let mut offset_1: c_int =
                0 as c_int + (2 as c_int >> chroma_v_shift) * 16 as c_int * 2 as c_int;
            let mut chroma_height_1: c_int = (2 as c_int >> chroma_v_shift) * 2 as c_int;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(offset_1 as isize),
                &mut *pix2.offset(offset_1 as isize),
                16 as intptr_t,
                &mut *(*p_fref.offset(4)).offset(
                    (or + 2 as c_int * 0 as c_int
                        + (2 as c_int >> chroma_v_shift) * 2 as c_int * i_stride)
                        as isize,
                ),
                i_stride as intptr_t,
                (*m.offset(2)).mv[0] as c_int,
                (2 as c_int >> chroma_v_shift) * ((*m.offset(2)).mv[1] as c_int + mvy_offset),
                2 as c_int,
                chroma_height_1,
            );
            if !(*weight.offset(1)).weightfn.is_null() {
                (*(*weight.offset(1))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix1.as_mut_ptr().offset(offset_1 as isize),
                    16 as intptr_t,
                    &mut *pix1.as_mut_ptr().offset(offset_1 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(1),
                    chroma_height_1,
                );
            }
            if !(*weight.offset(2)).weightfn.is_null() {
                (*(*weight.offset(2))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix2.offset(offset_1 as isize),
                    16 as intptr_t,
                    &mut *pix2.offset(offset_1 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(2),
                    chroma_height_1,
                );
            }
        }
        if chroma == CHROMA_444 as c_int {
            let mut mvx_2: c_int =
                (*m.offset(3)).mv[0] as c_int + 4 as c_int * 2 as c_int * 2 as c_int;
            let mut mvy_2: c_int =
                (*m.offset(3)).mv[1] as c_int + 4 as c_int * 2 as c_int * 2 as c_int;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(
                    (2 as c_int * 2 as c_int + 2 as c_int * 2 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4),
                i_stride as intptr_t,
                mvx_2,
                mvy_2,
                2 as c_int * 2 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix2.offset(
                    (2 as c_int * 2 as c_int + 2 as c_int * 2 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8),
                i_stride as intptr_t,
                mvx_2,
                mvy_2,
                2 as c_int * 2 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2),
            );
        } else {
            let mut offset_2: c_int =
                2 as c_int + (2 as c_int >> chroma_v_shift) * 16 as c_int * 2 as c_int;
            let mut chroma_height_2: c_int = (2 as c_int >> chroma_v_shift) * 2 as c_int;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(offset_2 as isize),
                &mut *pix2.offset(offset_2 as isize),
                16 as intptr_t,
                &mut *(*p_fref.offset(4)).offset(
                    (or + 2 as c_int * 2 as c_int
                        + (2 as c_int >> chroma_v_shift) * 2 as c_int * i_stride)
                        as isize,
                ),
                i_stride as intptr_t,
                (*m.offset(3)).mv[0] as c_int,
                (2 as c_int >> chroma_v_shift) * ((*m.offset(3)).mv[1] as c_int + mvy_offset),
                2 as c_int,
                chroma_height_2,
            );
            if !(*weight.offset(1)).weightfn.is_null() {
                (*(*weight.offset(1))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix1.as_mut_ptr().offset(offset_2 as isize),
                    16 as intptr_t,
                    &mut *pix1.as_mut_ptr().offset(offset_2 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(1),
                    chroma_height_2,
                );
            }
            if !(*weight.offset(2)).weightfn.is_null() {
                (*(*weight.offset(2))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix2.offset(offset_2 as isize),
                    16 as intptr_t,
                    &mut *pix2.offset(offset_2 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(2),
                    chroma_height_2,
                );
            }
        }
    } else if size == PIXEL_8x4 as c_int {
        let mut m_0: *mut x264_me_t =
            (*(*a).l0.me8x4.as_mut_ptr().offset(i8x8 as isize)).as_mut_ptr();
        if chroma == CHROMA_444 as c_int {
            let mut mvx_3: c_int =
                (*m_0.offset(0)).mv[0] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            let mut mvy_3: c_int =
                (*m_0.offset(0)).mv[1] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4),
                i_stride as intptr_t,
                mvx_3,
                mvy_3,
                2 as c_int * 4 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix2.offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8),
                i_stride as intptr_t,
                mvx_3,
                mvy_3,
                2 as c_int * 4 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2),
            );
        } else {
            let mut offset_3: c_int =
                0 as c_int + (2 as c_int >> chroma_v_shift) * 16 as c_int * 0 as c_int;
            let mut chroma_height_3: c_int = (2 as c_int >> chroma_v_shift) * 2 as c_int;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(offset_3 as isize),
                &mut *pix2.offset(offset_3 as isize),
                16 as intptr_t,
                &mut *(*p_fref.offset(4)).offset(
                    (or + 2 as c_int * 0 as c_int
                        + (2 as c_int >> chroma_v_shift) * 0 as c_int * i_stride)
                        as isize,
                ),
                i_stride as intptr_t,
                (*m_0.offset(0)).mv[0] as c_int,
                (2 as c_int >> chroma_v_shift) * ((*m_0.offset(0)).mv[1] as c_int + mvy_offset),
                4 as c_int,
                chroma_height_3,
            );
            if !(*weight.offset(1)).weightfn.is_null() {
                (*(*weight.offset(1))
                    .weightfn
                    .offset((4 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix1.as_mut_ptr().offset(offset_3 as isize),
                    16 as intptr_t,
                    &mut *pix1.as_mut_ptr().offset(offset_3 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(1),
                    chroma_height_3,
                );
            }
            if !(*weight.offset(2)).weightfn.is_null() {
                (*(*weight.offset(2))
                    .weightfn
                    .offset((4 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix2.offset(offset_3 as isize),
                    16 as intptr_t,
                    &mut *pix2.offset(offset_3 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(2),
                    chroma_height_3,
                );
            }
        }
        if chroma == CHROMA_444 as c_int {
            let mut mvx_4: c_int =
                (*m_0.offset(1)).mv[0] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            let mut mvy_4: c_int =
                (*m_0.offset(1)).mv[1] as c_int + 4 as c_int * 2 as c_int * 2 as c_int;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 2 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4),
                i_stride as intptr_t,
                mvx_4,
                mvy_4,
                2 as c_int * 4 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix2.offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 2 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8),
                i_stride as intptr_t,
                mvx_4,
                mvy_4,
                2 as c_int * 4 as c_int,
                2 as c_int * 2 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2),
            );
        } else {
            let mut offset_4: c_int =
                0 as c_int + (2 as c_int >> chroma_v_shift) * 16 as c_int * 2 as c_int;
            let mut chroma_height_4: c_int = (2 as c_int >> chroma_v_shift) * 2 as c_int;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(offset_4 as isize),
                &mut *pix2.offset(offset_4 as isize),
                16 as intptr_t,
                &mut *(*p_fref.offset(4)).offset(
                    (or + 2 as c_int * 0 as c_int
                        + (2 as c_int >> chroma_v_shift) * 2 as c_int * i_stride)
                        as isize,
                ),
                i_stride as intptr_t,
                (*m_0.offset(1)).mv[0] as c_int,
                (2 as c_int >> chroma_v_shift) * ((*m_0.offset(1)).mv[1] as c_int + mvy_offset),
                4 as c_int,
                chroma_height_4,
            );
            if !(*weight.offset(1)).weightfn.is_null() {
                (*(*weight.offset(1))
                    .weightfn
                    .offset((4 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix1.as_mut_ptr().offset(offset_4 as isize),
                    16 as intptr_t,
                    &mut *pix1.as_mut_ptr().offset(offset_4 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(1),
                    chroma_height_4,
                );
            }
            if !(*weight.offset(2)).weightfn.is_null() {
                (*(*weight.offset(2))
                    .weightfn
                    .offset((4 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix2.offset(offset_4 as isize),
                    16 as intptr_t,
                    &mut *pix2.offset(offset_4 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(2),
                    chroma_height_4,
                );
            }
        }
    } else {
        let mut m_1: *mut x264_me_t =
            (*(*a).l0.me4x8.as_mut_ptr().offset(i8x8 as isize)).as_mut_ptr();
        if chroma == CHROMA_444 as c_int {
            let mut mvx_5: c_int =
                (*m_1.offset(0)).mv[0] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            let mut mvy_5: c_int =
                (*m_1.offset(0)).mv[1] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4),
                i_stride as intptr_t,
                mvx_5,
                mvy_5,
                2 as c_int * 2 as c_int,
                2 as c_int * 4 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix2.offset(
                    (2 as c_int * 0 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8),
                i_stride as intptr_t,
                mvx_5,
                mvy_5,
                2 as c_int * 2 as c_int,
                2 as c_int * 4 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2),
            );
        } else {
            let mut offset_5: c_int =
                0 as c_int + (2 as c_int >> chroma_v_shift) * 16 as c_int * 0 as c_int;
            let mut chroma_height_5: c_int = (2 as c_int >> chroma_v_shift) * 4 as c_int;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(offset_5 as isize),
                &mut *pix2.offset(offset_5 as isize),
                16 as intptr_t,
                &mut *(*p_fref.offset(4)).offset(
                    (or + 2 as c_int * 0 as c_int
                        + (2 as c_int >> chroma_v_shift) * 0 as c_int * i_stride)
                        as isize,
                ),
                i_stride as intptr_t,
                (*m_1.offset(0)).mv[0] as c_int,
                (2 as c_int >> chroma_v_shift) * ((*m_1.offset(0)).mv[1] as c_int + mvy_offset),
                2 as c_int,
                chroma_height_5,
            );
            if !(*weight.offset(1)).weightfn.is_null() {
                (*(*weight.offset(1))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix1.as_mut_ptr().offset(offset_5 as isize),
                    16 as intptr_t,
                    &mut *pix1.as_mut_ptr().offset(offset_5 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(1),
                    chroma_height_5,
                );
            }
            if !(*weight.offset(2)).weightfn.is_null() {
                (*(*weight.offset(2))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix2.offset(offset_5 as isize),
                    16 as intptr_t,
                    &mut *pix2.offset(offset_5 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(2),
                    chroma_height_5,
                );
            }
        }
        if chroma == CHROMA_444 as c_int {
            let mut mvx_6: c_int =
                (*m_1.offset(1)).mv[0] as c_int + 4 as c_int * 2 as c_int * 2 as c_int;
            let mut mvy_6: c_int =
                (*m_1.offset(1)).mv[1] as c_int + 4 as c_int * 2 as c_int * 0 as c_int;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(
                    (2 as c_int * 2 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(4),
                i_stride as intptr_t,
                mvx_6,
                mvy_6,
                2 as c_int * 2 as c_int,
                2 as c_int * 4 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &mut *pix2.offset(
                    (2 as c_int * 2 as c_int + 2 as c_int * 0 as c_int * 16 as c_int) as isize,
                ),
                16 as intptr_t,
                &mut *(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(8),
                i_stride as intptr_t,
                mvx_6,
                mvy_6,
                2 as c_int * 2 as c_int,
                2 as c_int * 4 as c_int,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2),
            );
        } else {
            let mut offset_6: c_int =
                2 as c_int + (2 as c_int >> chroma_v_shift) * 16 as c_int * 0 as c_int;
            let mut chroma_height_6: c_int = (2 as c_int >> chroma_v_shift) * 4 as c_int;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &mut *pix1.as_mut_ptr().offset(offset_6 as isize),
                &mut *pix2.offset(offset_6 as isize),
                16 as intptr_t,
                &mut *(*p_fref.offset(4)).offset(
                    (or + 2 as c_int * 2 as c_int
                        + (2 as c_int >> chroma_v_shift) * 0 as c_int * i_stride)
                        as isize,
                ),
                i_stride as intptr_t,
                (*m_1.offset(1)).mv[0] as c_int,
                (2 as c_int >> chroma_v_shift) * ((*m_1.offset(1)).mv[1] as c_int + mvy_offset),
                2 as c_int,
                chroma_height_6,
            );
            if !(*weight.offset(1)).weightfn.is_null() {
                (*(*weight.offset(1))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix1.as_mut_ptr().offset(offset_6 as isize),
                    16 as intptr_t,
                    &mut *pix1.as_mut_ptr().offset(offset_6 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(1),
                    chroma_height_6,
                );
            }
            if !(*weight.offset(2)).weightfn.is_null() {
                (*(*weight.offset(2))
                    .weightfn
                    .offset((2 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                    &mut *pix2.offset(offset_6 as isize),
                    16 as intptr_t,
                    &mut *pix2.offset(offset_6 as isize),
                    16 as intptr_t,
                    &mut *weight.offset(2),
                    chroma_height_6,
                );
            }
        }
    }
    let mut oe: c_int = (8 as c_int >> chroma_h_shift) * (i8x8 & 1 as c_int)
        + (4 as c_int >> chroma_v_shift) * (i8x8 & 2 as c_int) * FENC_STRIDE;
    let mut chromapix: c_int = if chroma == CHROMA_444 as c_int {
        PIXEL_8x8 as c_int
    } else if chroma == CHROMA_422 as c_int {
        PIXEL_4x8 as c_int
    } else {
        PIXEL_4x4 as c_int
    };
    return (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
        &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(1)).offset(oe as isize),
        FENC_STRIDE as intptr_t,
        pix1.as_mut_ptr(),
        16 as intptr_t,
    ) + (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
        &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(2)).offset(oe as isize),
        FENC_STRIDE as intptr_t,
        pix2,
        16 as intptr_t,
    );
}
#[c2rust::src_loc = "1675:1"]
unsafe extern "C" fn mb_analyse_inter_p4x4_chroma(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut p_fref: *mut *mut pixel,
    mut i8x8: c_int,
    mut size: c_int,
) -> c_int {
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
        return mb_analyse_inter_p4x4_chroma_internal(
            h,
            a,
            p_fref,
            i8x8,
            size,
            CHROMA_444 as c_int,
        );
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as c_int {
        return mb_analyse_inter_p4x4_chroma_internal(
            h,
            a,
            p_fref,
            i8x8,
            size,
            CHROMA_422 as c_int,
        );
    } else {
        return mb_analyse_inter_p4x4_chroma_internal(
            h,
            a,
            p_fref,
            i8x8,
            size,
            CHROMA_420 as c_int,
        );
    };
}
#[c2rust::src_loc = "1685:1"]
unsafe extern "C" fn mb_analyse_inter_p4x4(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i8x8: c_int,
) {
    let mut p_fref: *mut *mut pixel = (*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
        .as_mut_ptr()
        .offset((*(*a).l0.me8x8.as_mut_ptr().offset(i8x8 as isize)).i_ref as isize))
    .as_mut_ptr();
    let mut p_fenc: *mut *mut pixel = (*h).mb.pic.p_fenc.as_mut_ptr();
    let i_ref: c_int = (*a).l0.me8x8[i8x8 as usize].i_ref;
    (*h).mb.i_partition = D_8x8 as c_int;
    let mut i4x4: c_int = 0 as c_int;
    while i4x4 < 4 as c_int {
        let idx: c_int = 4 as c_int * i8x8 + i4x4;
        let x4: c_int = block_idx_x[idx as usize] as c_int;
        let y4: c_int = block_idx_y[idx as usize] as c_int;
        let i_mvc: c_int = (i4x4 == 0 as c_int) as c_int;
        let mut m: *mut x264_me_t = &mut *(*(*a).l0.me4x4.as_mut_ptr().offset(i8x8 as isize))
            .as_mut_ptr()
            .offset(i4x4 as isize) as *mut x264_me_t;
        (*m).i_pixel = PIXEL_4x4 as c_int;
        (*m).p_cost_mv = (*a).p_cost_mv;
        (*m).i_stride[0] = (*h).mb.pic.i_stride[0];
        (*m).i_stride[1] = (*h).mb.pic.i_stride[1];
        (*m).i_stride[2] = (*h).mb.pic.i_stride[2];
        (*m).p_fenc[0] = &mut *(*p_fenc.offset(0))
            .offset((4 as c_int * x4 + 4 as c_int * y4 * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*m).p_fenc[1] = &mut *(*p_fenc.offset(1)).offset(
                ((4 as c_int * x4 >> (*h).mb.chroma_h_shift)
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            (*m).p_fenc[2] = &mut *(*p_fenc.offset(2)).offset(
                ((4 as c_int * x4 >> (*h).mb.chroma_h_shift)
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        (*m).p_fref[0] = &mut *(*p_fref.offset(0)).offset(
            (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0)) as isize,
        ) as *mut pixel;
        (*m).p_fref_w = (*m).p_fref[0];
        if (*h).param.analyse.i_subpel_refine != 0 {
            (*m).p_fref[1] = &mut *(*p_fref.offset(1)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[2] = &mut *(*p_fref.offset(2)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[3] = &mut *(*p_fref.offset(3)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
        }
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            (*m).p_fref[4] = &mut *(*p_fref.offset(4)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[8] = &mut *(*p_fref.offset(8)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                    as isize,
            ) as *mut pixel;
            if (*h).param.analyse.i_subpel_refine != 0 {
                (*m).p_fref[5] = &mut *(*p_fref.offset(5)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[6] = &mut *(*p_fref.offset(6)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[7] = &mut *(*p_fref.offset(7)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[9] = &mut *(*p_fref.offset(9 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[10] = &mut *(*p_fref.offset(10 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[11] = &mut *(*p_fref.offset(11 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
            }
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*m).p_fref[4] = &mut *(*p_fref.offset(4)).offset(
                (4 as c_int * x4
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift)
                        * *(*m).i_stride.as_mut_ptr().offset(1)) as isize,
            ) as *mut pixel;
        }
        if (*h).param.analyse.me_method.exhaustive_search() {
            (*m).integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut uint16_t;
        }
        (*m).weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
        (*m).i_ref = i_ref;
        (*m).p_fref_w = &mut *(*(*h).mb.pic.p_fref_w.as_mut_ptr().offset(i_ref as isize)).offset(
            (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0)) as isize,
        ) as *mut pixel;
        (*m).weight = (*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr();
        x264_10_mb_predict_mv(h, 0 as c_int, idx, 1 as c_int, (*m).mvp.as_mut_ptr());
        x264_10_me_search_ref(
            h,
            m,
            &mut (*(*a).l0.me8x8.as_mut_ptr().offset(i8x8 as isize)).mv,
            i_mvc,
            0 as *mut c_int,
        );
        x264_macroblock_cache_mv(
            h,
            x4,
            y4,
            1 as c_int,
            1 as c_int,
            0 as c_int,
            (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i,
        );
        i4x4 += 1;
    }
    (*a).l0.i_cost4x4[i8x8 as usize] = (*a).l0.me4x4[i8x8 as usize][0].cost
        + (*a).l0.me4x4[i8x8 as usize][1].cost
        + (*a).l0.me4x4[i8x8 as usize][2].cost
        + (*a).l0.me4x4[i8x8 as usize][3].cost
        + *(*a).p_cost_ref[0].offset(i_ref as isize) as c_int
        + (*a).i_lambda * i_sub_mb_p_cost_table[D_L0_4x4 as c_int as usize] as c_int;
    if (*h).mb.b_chroma_me != 0
        && !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int)
    {
        (*a).l0.i_cost4x4[i8x8 as usize] +=
            mb_analyse_inter_p4x4_chroma(h, a, p_fref, i8x8, PIXEL_4x4 as c_int);
    }
}
#[c2rust::src_loc = "1724:1"]
unsafe extern "C" fn mb_analyse_inter_p8x4(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i8x8: c_int,
) {
    let mut p_fref: *mut *mut pixel = (*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
        .as_mut_ptr()
        .offset((*(*a).l0.me8x8.as_mut_ptr().offset(i8x8 as isize)).i_ref as isize))
    .as_mut_ptr();
    let mut p_fenc: *mut *mut pixel = (*h).mb.pic.p_fenc.as_mut_ptr();
    let i_ref: c_int = (*a).l0.me8x8[i8x8 as usize].i_ref;
    (*h).mb.i_partition = D_8x8 as c_int;
    let mut i8x4: c_int = 0 as c_int;
    while i8x4 < 2 as c_int {
        let idx: c_int = 4 as c_int * i8x8 + 2 as c_int * i8x4;
        let x4: c_int = block_idx_x[idx as usize] as c_int;
        let y4: c_int = block_idx_y[idx as usize] as c_int;
        let i_mvc: c_int = (i8x4 == 0 as c_int) as c_int;
        let mut m: *mut x264_me_t = &mut *(*(*a).l0.me8x4.as_mut_ptr().offset(i8x8 as isize))
            .as_mut_ptr()
            .offset(i8x4 as isize) as *mut x264_me_t;
        (*m).i_pixel = PIXEL_8x4 as c_int;
        (*m).p_cost_mv = (*a).p_cost_mv;
        (*m).i_stride[0] = (*h).mb.pic.i_stride[0];
        (*m).i_stride[1] = (*h).mb.pic.i_stride[1];
        (*m).i_stride[2] = (*h).mb.pic.i_stride[2];
        (*m).p_fenc[0] = &mut *(*p_fenc.offset(0))
            .offset((4 as c_int * x4 + 4 as c_int * y4 * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*m).p_fenc[1] = &mut *(*p_fenc.offset(1)).offset(
                ((4 as c_int * x4 >> (*h).mb.chroma_h_shift)
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            (*m).p_fenc[2] = &mut *(*p_fenc.offset(2)).offset(
                ((4 as c_int * x4 >> (*h).mb.chroma_h_shift)
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        (*m).p_fref[0] = &mut *(*p_fref.offset(0)).offset(
            (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0)) as isize,
        ) as *mut pixel;
        (*m).p_fref_w = (*m).p_fref[0];
        if (*h).param.analyse.i_subpel_refine != 0 {
            (*m).p_fref[1] = &mut *(*p_fref.offset(1)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[2] = &mut *(*p_fref.offset(2)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[3] = &mut *(*p_fref.offset(3)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
        }
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            (*m).p_fref[4] = &mut *(*p_fref.offset(4)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[8] = &mut *(*p_fref.offset(8)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                    as isize,
            ) as *mut pixel;
            if (*h).param.analyse.i_subpel_refine != 0 {
                (*m).p_fref[5] = &mut *(*p_fref.offset(5)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[6] = &mut *(*p_fref.offset(6)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[7] = &mut *(*p_fref.offset(7)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[9] = &mut *(*p_fref.offset(9 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[10] = &mut *(*p_fref.offset(10 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[11] = &mut *(*p_fref.offset(11 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
            }
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*m).p_fref[4] = &mut *(*p_fref.offset(4)).offset(
                (4 as c_int * x4
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift)
                        * *(*m).i_stride.as_mut_ptr().offset(1)) as isize,
            ) as *mut pixel;
        }
        if (*h).param.analyse.me_method.exhaustive_search() {
            (*m).integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut uint16_t;
        }
        (*m).weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
        (*m).i_ref = i_ref;
        (*m).p_fref_w = &mut *(*(*h).mb.pic.p_fref_w.as_mut_ptr().offset(i_ref as isize)).offset(
            (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0)) as isize,
        ) as *mut pixel;
        (*m).weight = (*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr();
        x264_10_mb_predict_mv(h, 0 as c_int, idx, 2 as c_int, (*m).mvp.as_mut_ptr());
        x264_10_me_search_ref(
            h,
            m,
            &mut (*(*(*a).l0.me4x4.as_mut_ptr().offset(i8x8 as isize))
                .as_mut_ptr()
                .offset(0))
            .mv,
            i_mvc,
            0 as *mut c_int,
        );
        x264_macroblock_cache_mv(
            h,
            x4,
            y4,
            2 as c_int,
            1 as c_int,
            0 as c_int,
            (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i,
        );
        i8x4 += 1;
    }
    (*a).l0.i_cost8x4[i8x8 as usize] = (*a).l0.me8x4[i8x8 as usize][0].cost
        + (*a).l0.me8x4[i8x8 as usize][1].cost
        + *(*a).p_cost_ref[0].offset(i_ref as isize) as c_int
        + (*a).i_lambda * i_sub_mb_p_cost_table[D_L0_8x4 as c_int as usize] as c_int;
    if (*h).mb.b_chroma_me != 0
        && !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int)
    {
        (*a).l0.i_cost8x4[i8x8 as usize] +=
            mb_analyse_inter_p4x4_chroma(h, a, p_fref, i8x8, PIXEL_8x4 as c_int);
    }
}
#[c2rust::src_loc = "1760:1"]
unsafe extern "C" fn mb_analyse_inter_p4x8(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i8x8: c_int,
) {
    let mut p_fref: *mut *mut pixel = (*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
        .as_mut_ptr()
        .offset((*(*a).l0.me8x8.as_mut_ptr().offset(i8x8 as isize)).i_ref as isize))
    .as_mut_ptr();
    let mut p_fenc: *mut *mut pixel = (*h).mb.pic.p_fenc.as_mut_ptr();
    let i_ref: c_int = (*a).l0.me8x8[i8x8 as usize].i_ref;
    (*h).mb.i_partition = D_8x8 as c_int;
    let mut i4x8: c_int = 0 as c_int;
    while i4x8 < 2 as c_int {
        let idx: c_int = 4 as c_int * i8x8 + i4x8;
        let x4: c_int = block_idx_x[idx as usize] as c_int;
        let y4: c_int = block_idx_y[idx as usize] as c_int;
        let i_mvc: c_int = (i4x8 == 0 as c_int) as c_int;
        let mut m: *mut x264_me_t = &mut *(*(*a).l0.me4x8.as_mut_ptr().offset(i8x8 as isize))
            .as_mut_ptr()
            .offset(i4x8 as isize) as *mut x264_me_t;
        (*m).i_pixel = PIXEL_4x8 as c_int;
        (*m).p_cost_mv = (*a).p_cost_mv;
        (*m).i_stride[0] = (*h).mb.pic.i_stride[0];
        (*m).i_stride[1] = (*h).mb.pic.i_stride[1];
        (*m).i_stride[2] = (*h).mb.pic.i_stride[2];
        (*m).p_fenc[0] = &mut *(*p_fenc.offset(0))
            .offset((4 as c_int * x4 + 4 as c_int * y4 * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*m).p_fenc[1] = &mut *(*p_fenc.offset(1)).offset(
                ((4 as c_int * x4 >> (*h).mb.chroma_h_shift)
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            (*m).p_fenc[2] = &mut *(*p_fenc.offset(2)).offset(
                ((4 as c_int * x4 >> (*h).mb.chroma_h_shift)
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        (*m).p_fref[0] = &mut *(*p_fref.offset(0)).offset(
            (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0)) as isize,
        ) as *mut pixel;
        (*m).p_fref_w = (*m).p_fref[0];
        if (*h).param.analyse.i_subpel_refine != 0 {
            (*m).p_fref[1] = &mut *(*p_fref.offset(1)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[2] = &mut *(*p_fref.offset(2)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[3] = &mut *(*p_fref.offset(3)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
        }
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            (*m).p_fref[4] = &mut *(*p_fref.offset(4)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref[8] = &mut *(*p_fref.offset(8)).offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                    as isize,
            ) as *mut pixel;
            if (*h).param.analyse.i_subpel_refine != 0 {
                (*m).p_fref[5] = &mut *(*p_fref.offset(5)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[6] = &mut *(*p_fref.offset(6)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[7] = &mut *(*p_fref.offset(7)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(1))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[9] = &mut *(*p_fref.offset(9 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[10] = &mut *(*p_fref.offset(10 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
                (*m).p_fref[11] = &mut *(*p_fref.offset(11 as c_int as isize)).offset(
                    (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(2))
                        as isize,
                ) as *mut pixel;
            }
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*m).p_fref[4] = &mut *(*p_fref.offset(4)).offset(
                (4 as c_int * x4
                    + (4 as c_int * y4 >> (*h).mb.chroma_v_shift)
                        * *(*m).i_stride.as_mut_ptr().offset(1)) as isize,
            ) as *mut pixel;
        }
        if (*h).param.analyse.me_method.exhaustive_search() {
            (*m).integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(i_ref as isize))
            .offset(
                (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut uint16_t;
        }
        (*m).weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
        (*m).i_ref = i_ref;
        (*m).p_fref_w = &mut *(*(*h).mb.pic.p_fref_w.as_mut_ptr().offset(i_ref as isize)).offset(
            (4 as c_int * x4 + 4 as c_int * y4 * *(*m).i_stride.as_mut_ptr().offset(0)) as isize,
        ) as *mut pixel;
        (*m).weight = (*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr();
        x264_10_mb_predict_mv(h, 0 as c_int, idx, 1 as c_int, (*m).mvp.as_mut_ptr());
        x264_10_me_search_ref(
            h,
            m,
            &mut (*(*(*a).l0.me4x4.as_mut_ptr().offset(i8x8 as isize))
                .as_mut_ptr()
                .offset(0))
            .mv,
            i_mvc,
            0 as *mut c_int,
        );
        x264_macroblock_cache_mv(
            h,
            x4,
            y4,
            1 as c_int,
            2 as c_int,
            0 as c_int,
            (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i,
        );
        i4x8 += 1;
    }
    (*a).l0.i_cost4x8[i8x8 as usize] = (*a).l0.me4x8[i8x8 as usize][0].cost
        + (*a).l0.me4x8[i8x8 as usize][1].cost
        + *(*a).p_cost_ref[0].offset(i_ref as isize) as c_int
        + (*a).i_lambda * i_sub_mb_p_cost_table[D_L0_4x8 as c_int as usize] as c_int;
    if (*h).mb.b_chroma_me != 0
        && !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int)
    {
        (*a).l0.i_cost4x8[i8x8 as usize] +=
            mb_analyse_inter_p4x4_chroma(h, a, p_fref, i8x8, PIXEL_4x8 as c_int);
    }
}
#[inline(always)]
#[c2rust::src_loc = "1796:1"]
unsafe extern "C" fn analyse_bi_chroma(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut idx: c_int,
    mut i_pixel: c_int,
) -> c_int {
    let mut pix: [[pixel; 256]; 4] = [[0; 256]; 4];
    let mut bi: [[pixel; 256]; 2] = [[0; 256]; 2];
    let mut i_chroma_cost: c_int = 0 as c_int;
    let mut chromapix: c_int = (*h).luma2chroma_pixel[i_pixel as usize] as c_int;
    if i_pixel == PIXEL_16x16 as c_int {
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*a).l0.bi16x16.p_fref.as_mut_ptr().offset(4),
                (*a).l0.bi16x16.i_stride[1] as intptr_t,
                (*a).l0.bi16x16.mv[0] as c_int,
                (*a).l0.bi16x16.mv[1] as c_int,
                16 as c_int,
                16 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*a).l0.bi16x16.p_fref.as_mut_ptr().offset(8),
                (*a).l0.bi16x16.i_stride[2] as intptr_t,
                (*a).l0.bi16x16.mv[0] as c_int,
                (*a).l0.bi16x16.mv[1] as c_int,
                16 as c_int,
                16 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*a).l1.bi16x16.p_fref.as_mut_ptr().offset(4),
                (*a).l1.bi16x16.i_stride[1] as intptr_t,
                (*a).l1.bi16x16.mv[0] as c_int,
                (*a).l1.bi16x16.mv[1] as c_int,
                16 as c_int,
                16 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*a).l1.bi16x16.p_fref.as_mut_ptr().offset(8),
                (*a).l1.bi16x16.i_stride[2] as intptr_t,
                (*a).l1.bi16x16.mv[0] as c_int,
                (*a).l1.bi16x16.mv[1] as c_int,
                16 as c_int,
                16 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
        } else {
            let mut v_shift: c_int = (*h).mb.chroma_v_shift;
            let mut l0_mvy_offset: c_int =
                if v_shift & (*h).mb.b_interlaced & (*a).l0.bi16x16.i_ref != 0 {
                    ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
                } else {
                    0 as c_int
                };
            let mut l1_mvy_offset: c_int =
                if v_shift & (*h).mb.b_interlaced & (*a).l1.bi16x16.i_ref != 0 {
                    ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
                } else {
                    0 as c_int
                };
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
                (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
                16 as intptr_t,
                (*a).l0.bi16x16.p_fref[4],
                (*a).l0.bi16x16.i_stride[1] as intptr_t,
                (*a).l0.bi16x16.mv[0] as c_int,
                2 as c_int * ((*a).l0.bi16x16.mv[1] as c_int + l0_mvy_offset) >> v_shift,
                16 as c_int >> 1 as c_int,
                16 as c_int >> v_shift,
            );
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
                (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
                16 as intptr_t,
                (*a).l1.bi16x16.p_fref[4],
                (*a).l1.bi16x16.i_stride[1] as intptr_t,
                (*a).l1.bi16x16.mv[0] as c_int,
                2 as c_int * ((*a).l1.bi16x16.mv[1] as c_int + l1_mvy_offset) >> v_shift,
                16 as c_int >> 1 as c_int,
                16 as c_int >> v_shift,
            );
        }
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            (*bi.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
            16 as intptr_t,
            (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
                [(*a).l1.bi16x16.i_ref as usize] as c_int,
        );
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            (*bi.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
            16 as intptr_t,
            (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
                [(*a).l1.bi16x16.i_ref as usize] as c_int,
        );
        i_chroma_cost = (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
            (*a).l0.bi16x16.p_fenc[1],
            FENC_STRIDE as intptr_t,
            (*bi.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
        ) + (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
            (*a).l0.bi16x16.p_fenc[2],
            FENC_STRIDE as intptr_t,
            (*bi.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
        );
    } else if i_pixel == PIXEL_16x8 as c_int {
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l0.me16x8.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(4),
                (*a).l0.me16x8[idx as usize].i_stride[1] as intptr_t,
                (*a).l0.me16x8[idx as usize].mv[0] as c_int,
                (*a).l0.me16x8[idx as usize].mv[1] as c_int,
                16 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l0.me16x8.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(8),
                (*a).l0.me16x8[idx as usize].i_stride[2] as intptr_t,
                (*a).l0.me16x8[idx as usize].mv[0] as c_int,
                (*a).l0.me16x8[idx as usize].mv[1] as c_int,
                16 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l1.me16x8.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(4),
                (*a).l1.me16x8[idx as usize].i_stride[1] as intptr_t,
                (*a).l1.me16x8[idx as usize].mv[0] as c_int,
                (*a).l1.me16x8[idx as usize].mv[1] as c_int,
                16 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l1.me16x8.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(8),
                (*a).l1.me16x8[idx as usize].i_stride[2] as intptr_t,
                (*a).l1.me16x8[idx as usize].mv[0] as c_int,
                (*a).l1.me16x8[idx as usize].mv[1] as c_int,
                16 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
        } else {
            let mut v_shift_0: c_int = (*h).mb.chroma_v_shift;
            let mut l0_mvy_offset_0: c_int =
                if v_shift_0 & (*h).mb.b_interlaced & (*a).l0.me16x8[idx as usize].i_ref != 0 {
                    ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
                } else {
                    0 as c_int
                };
            let mut l1_mvy_offset_0: c_int =
                if v_shift_0 & (*h).mb.b_interlaced & (*a).l1.me16x8[idx as usize].i_ref != 0 {
                    ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
                } else {
                    0 as c_int
                };
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
                (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
                16 as intptr_t,
                (*a).l0.me16x8[idx as usize].p_fref[4],
                (*a).l0.me16x8[idx as usize].i_stride[1] as intptr_t,
                (*a).l0.me16x8[idx as usize].mv[0] as c_int,
                2 as c_int * ((*a).l0.me16x8[idx as usize].mv[1] as c_int + l0_mvy_offset_0)
                    >> v_shift_0,
                16 as c_int >> 1 as c_int,
                8 as c_int >> v_shift_0,
            );
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
                (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
                16 as intptr_t,
                (*a).l1.me16x8[idx as usize].p_fref[4],
                (*a).l1.me16x8[idx as usize].i_stride[1] as intptr_t,
                (*a).l1.me16x8[idx as usize].mv[0] as c_int,
                2 as c_int * ((*a).l1.me16x8[idx as usize].mv[1] as c_int + l1_mvy_offset_0)
                    >> v_shift_0,
                16 as c_int >> 1 as c_int,
                8 as c_int >> v_shift_0,
            );
        }
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            (*bi.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
            16 as intptr_t,
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me16x8[idx as usize].i_ref as isize))
                [(*a).l1.me16x8[idx as usize].i_ref as usize] as c_int,
        );
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            (*bi.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
            16 as intptr_t,
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me16x8[idx as usize].i_ref as isize))
                [(*a).l1.me16x8[idx as usize].i_ref as usize] as c_int,
        );
        i_chroma_cost = (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
            (*a).l0.me16x8[idx as usize].p_fenc[1],
            FENC_STRIDE as intptr_t,
            (*bi.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
        ) + (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
            (*a).l0.me16x8[idx as usize].p_fenc[2],
            FENC_STRIDE as intptr_t,
            (*bi.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
        );
    } else if i_pixel == PIXEL_8x16 as c_int {
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l0.me8x16.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(4),
                (*a).l0.me8x16[idx as usize].i_stride[1] as intptr_t,
                (*a).l0.me8x16[idx as usize].mv[0] as c_int,
                (*a).l0.me8x16[idx as usize].mv[1] as c_int,
                8 as c_int,
                16 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l0.me8x16.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(8),
                (*a).l0.me8x16[idx as usize].i_stride[2] as intptr_t,
                (*a).l0.me8x16[idx as usize].mv[0] as c_int,
                (*a).l0.me8x16[idx as usize].mv[1] as c_int,
                8 as c_int,
                16 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l1.me8x16.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(4),
                (*a).l1.me8x16[idx as usize].i_stride[1] as intptr_t,
                (*a).l1.me8x16[idx as usize].mv[0] as c_int,
                (*a).l1.me8x16[idx as usize].mv[1] as c_int,
                8 as c_int,
                16 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l1.me8x16.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(8),
                (*a).l1.me8x16[idx as usize].i_stride[2] as intptr_t,
                (*a).l1.me8x16[idx as usize].mv[0] as c_int,
                (*a).l1.me8x16[idx as usize].mv[1] as c_int,
                8 as c_int,
                16 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
        } else {
            let mut v_shift_1: c_int = (*h).mb.chroma_v_shift;
            let mut l0_mvy_offset_1: c_int =
                if v_shift_1 & (*h).mb.b_interlaced & (*a).l0.me8x16[idx as usize].i_ref != 0 {
                    ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
                } else {
                    0 as c_int
                };
            let mut l1_mvy_offset_1: c_int =
                if v_shift_1 & (*h).mb.b_interlaced & (*a).l1.me8x16[idx as usize].i_ref != 0 {
                    ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
                } else {
                    0 as c_int
                };
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
                (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
                16 as intptr_t,
                (*a).l0.me8x16[idx as usize].p_fref[4],
                (*a).l0.me8x16[idx as usize].i_stride[1] as intptr_t,
                (*a).l0.me8x16[idx as usize].mv[0] as c_int,
                2 as c_int * ((*a).l0.me8x16[idx as usize].mv[1] as c_int + l0_mvy_offset_1)
                    >> v_shift_1,
                8 as c_int >> 1 as c_int,
                16 as c_int >> v_shift_1,
            );
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
                (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
                16 as intptr_t,
                (*a).l1.me8x16[idx as usize].p_fref[4],
                (*a).l1.me8x16[idx as usize].i_stride[1] as intptr_t,
                (*a).l1.me8x16[idx as usize].mv[0] as c_int,
                2 as c_int * ((*a).l1.me8x16[idx as usize].mv[1] as c_int + l1_mvy_offset_1)
                    >> v_shift_1,
                8 as c_int >> 1 as c_int,
                16 as c_int >> v_shift_1,
            );
        }
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            (*bi.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
            16 as intptr_t,
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me8x16[idx as usize].i_ref as isize))
                [(*a).l1.me8x16[idx as usize].i_ref as usize] as c_int,
        );
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            (*bi.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
            16 as intptr_t,
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me8x16[idx as usize].i_ref as isize))
                [(*a).l1.me8x16[idx as usize].i_ref as usize] as c_int,
        );
        i_chroma_cost = (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
            (*a).l0.me8x16[idx as usize].p_fenc[1],
            FENC_STRIDE as intptr_t,
            (*bi.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
        ) + (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
            (*a).l0.me8x16[idx as usize].p_fenc[2],
            FENC_STRIDE as intptr_t,
            (*bi.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
        );
    } else {
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l0.me8x8.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(4),
                (*a).l0.me8x8[idx as usize].i_stride[1] as intptr_t,
                (*a).l0.me8x8[idx as usize].mv[0] as c_int,
                (*a).l0.me8x8[idx as usize].mv[1] as c_int,
                8 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l0.me8x8.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(8),
                (*a).l0.me8x8[idx as usize].i_stride[2] as intptr_t,
                (*a).l0.me8x8[idx as usize].mv[0] as c_int,
                (*a).l0.me8x8[idx as usize].mv[1] as c_int,
                8 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l1.me8x8.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(4),
                (*a).l1.me8x8[idx as usize].i_stride[1] as intptr_t,
                (*a).l1.me8x8[idx as usize].mv[0] as c_int,
                (*a).l1.me8x8[idx as usize].mv[1] as c_int,
                8 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
                16 as intptr_t,
                &mut *(*(*a).l1.me8x8.as_mut_ptr().offset(idx as isize))
                    .p_fref
                    .as_mut_ptr()
                    .offset(8),
                (*a).l1.me8x8[idx as usize].i_stride[2] as intptr_t,
                (*a).l1.me8x8[idx as usize].mv[0] as c_int,
                (*a).l1.me8x8[idx as usize].mv[1] as c_int,
                8 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
        } else {
            let mut v_shift_2: c_int = (*h).mb.chroma_v_shift;
            let mut l0_mvy_offset_2: c_int =
                if v_shift_2 & (*h).mb.b_interlaced & (*a).l0.me8x8[idx as usize].i_ref != 0 {
                    ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
                } else {
                    0 as c_int
                };
            let mut l1_mvy_offset_2: c_int =
                if v_shift_2 & (*h).mb.b_interlaced & (*a).l1.me8x8[idx as usize].i_ref != 0 {
                    ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int
                } else {
                    0 as c_int
                };
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
                (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
                16 as intptr_t,
                (*a).l0.me8x8[idx as usize].p_fref[4],
                (*a).l0.me8x8[idx as usize].i_stride[1] as intptr_t,
                (*a).l0.me8x8[idx as usize].mv[0] as c_int,
                2 as c_int * ((*a).l0.me8x8[idx as usize].mv[1] as c_int + l0_mvy_offset_2)
                    >> v_shift_2,
                8 as c_int >> 1 as c_int,
                8 as c_int >> v_shift_2,
            );
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
                (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
                16 as intptr_t,
                (*a).l1.me8x8[idx as usize].p_fref[4],
                (*a).l1.me8x8[idx as usize].i_stride[1] as intptr_t,
                (*a).l1.me8x8[idx as usize].mv[0] as c_int,
                2 as c_int * ((*a).l1.me8x8[idx as usize].mv[1] as c_int + l1_mvy_offset_2)
                    >> v_shift_2,
                8 as c_int >> 1 as c_int,
                8 as c_int >> v_shift_2,
            );
        }
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            (*bi.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(2)).as_mut_ptr(),
            16 as intptr_t,
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me8x8[idx as usize].i_ref as isize))
                [(*a).l1.me8x8[idx as usize].i_ref as usize] as c_int,
        );
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            (*bi.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
            (*pix.as_mut_ptr().offset(3)).as_mut_ptr(),
            16 as intptr_t,
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me8x8[idx as usize].i_ref as isize))
                [(*a).l1.me8x8[idx as usize].i_ref as usize] as c_int,
        );
        i_chroma_cost = (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
            (*a).l0.me8x8[idx as usize].p_fenc[1],
            FENC_STRIDE as intptr_t,
            (*bi.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
        ) + (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
            (*a).l0.me8x8[idx as usize].p_fenc[2],
            FENC_STRIDE as intptr_t,
            (*bi.as_mut_ptr().offset(1)).as_mut_ptr(),
            16 as intptr_t,
        );
    }
    return i_chroma_cost;
}
#[c2rust::src_loc = "1844:1"]
unsafe extern "C" fn mb_analyse_inter_direct(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    let mut p_fenc: *mut pixel = (*h).mb.pic.p_fenc[0];
    let mut p_fdec: *mut pixel = (*h).mb.pic.p_fdec[0];
    (*a).i_cost16x16direct = (*a).i_lambda * i_mb_b_cost_table[B_DIRECT as c_int as usize] as c_int;
    if (*h).param.analyse.inter & X264_ANALYSE_BSUB16x16 != 0 {
        let mut chromapix: c_int = (*h).luma2chroma_pixel[PIXEL_8x8 as c_int as usize] as c_int;
        let mut i: c_int = 0 as c_int;
        while i < 4 as c_int {
            let x: c_int = (i & 1 as c_int) * 8 as c_int;
            let y: c_int = (i >> 1 as c_int) * 8 as c_int;
            (*a).i_cost8x8direct[i as usize] = (*h).pixf.mbcmp[PIXEL_8x8 as c_int as usize]
                .expect("non-null function pointer")(
                &mut *p_fenc.offset((x + y * FENC_STRIDE) as isize),
                FENC_STRIDE as intptr_t,
                &mut *p_fdec.offset((x + y * FDEC_STRIDE) as isize),
                FDEC_STRIDE as intptr_t,
            );
            if (*h).mb.b_chroma_me != 0 {
                let mut fenc_offset: c_int =
                    (x >> (*h).mb.chroma_h_shift) + (y >> (*h).mb.chroma_v_shift) * FENC_STRIDE;
                let mut fdec_offset: c_int =
                    (x >> (*h).mb.chroma_h_shift) + (y >> (*h).mb.chroma_v_shift) * FDEC_STRIDE;
                (*a).i_cost8x8direct[i as usize] += (*h).pixf.mbcmp[chromapix as usize]
                    .expect("non-null function pointer")(
                    &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(1)).offset(fenc_offset as isize),
                    FENC_STRIDE as intptr_t,
                    &mut *(*(*h).mb.pic.p_fdec.as_mut_ptr().offset(1)).offset(fdec_offset as isize),
                    FDEC_STRIDE as intptr_t,
                ) + (*h).pixf.mbcmp[chromapix as usize]
                    .expect("non-null function pointer")(
                    &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(2)).offset(fenc_offset as isize),
                    FENC_STRIDE as intptr_t,
                    &mut *(*(*h).mb.pic.p_fdec.as_mut_ptr().offset(2)).offset(fdec_offset as isize),
                    FDEC_STRIDE as intptr_t,
                );
            }
            (*a).i_cost16x16direct += (*a).i_cost8x8direct[i as usize];
            (*a).i_cost8x8direct[i as usize] +=
                (*a).i_lambda * i_sub_mb_b_cost_table[D_DIRECT_8x8 as c_int as usize] as c_int;
            i += 1;
        }
    } else {
        (*a).i_cost16x16direct += (*h).pixf.mbcmp[PIXEL_16x16 as c_int as usize]
            .expect("non-null function pointer")(
            p_fenc,
            FENC_STRIDE as intptr_t,
            p_fdec,
            FDEC_STRIDE as intptr_t,
        );
        if (*h).mb.b_chroma_me != 0 {
            let mut chromapix_0: c_int =
                (*h).luma2chroma_pixel[PIXEL_16x16 as c_int as usize] as c_int;
            (*a).i_cost16x16direct +=
                (*h).pixf.mbcmp[chromapix_0 as usize].expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[1],
                    FENC_STRIDE as intptr_t,
                    (*h).mb.pic.p_fdec[1],
                    FDEC_STRIDE as intptr_t,
                ) + (*h).pixf.mbcmp[chromapix_0 as usize].expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[2],
                    FENC_STRIDE as intptr_t,
                    (*h).mb.pic.p_fdec[2],
                    FDEC_STRIDE as intptr_t,
                );
        }
    };
}
#[c2rust::src_loc = "1890:1"]
unsafe extern "C" fn mb_analyse_inter_b16x16(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    let mut pix0: [pixel; 256] = [0; 256];
    let mut pix1: [pixel; 256] = [0; 256];
    let mut src0: *mut pixel = 0 as *mut pixel;
    let mut src1: *mut pixel = 0 as *mut pixel;
    let mut stride0: intptr_t = 16 as intptr_t;
    let mut stride1: intptr_t = 16 as intptr_t;
    let mut i_ref: c_int = 0;
    let mut i_mvc: c_int = 0;
    let mut mvc: [[int16_t; 2]; 9] = [[0; 2]; 9];
    let mut try_skip: c_int = (*a).b_try_skip;
    let mut list1_skipped: c_int = 0 as c_int;
    let mut i_halfpel_thresh: [c_int; 2] = [INT_MAX, INT_MAX];
    let mut p_halfpel_thresh: [*mut c_int; 2] = [
        if (*a).b_early_terminate != 0 && (*h).mb.pic.i_fref[0] > 1 as c_int {
            &mut *i_halfpel_thresh.as_mut_ptr().offset(0) as *mut c_int
        } else {
            0 as *mut c_int
        },
        if (*a).b_early_terminate != 0 && (*h).mb.pic.i_fref[1] > 1 as c_int {
            &mut *i_halfpel_thresh.as_mut_ptr().offset(1) as *mut c_int
        } else {
            0 as *mut c_int
        },
    ];
    let mut m: x264_me_t = x264_me_t {
        i_pixel: 0,
        p_cost_mv: 0 as *mut uint16_t,
        i_ref_cost: 0,
        i_ref: 0,
        weight: 0 as *const x264_weight_t,
        p_fref: [0 as *mut pixel; 12],
        p_fref_w: 0 as *mut pixel,
        p_fenc: [0 as *mut pixel; 3],
        integral: 0 as *mut uint16_t,
        i_stride: [0; 3],
        mvp: [0; 2],
        cost_mv: 0,
        cost: 0,
        mv: [0; 2],
    };
    m.i_pixel = PIXEL_16x16 as c_int;
    m.p_cost_mv = (*a).p_cost_mv;
    m.i_stride[0] = (*h).mb.pic.i_stride[0];
    m.i_stride[1] = (*h).mb.pic.i_stride[1];
    m.i_stride[2] = (*h).mb.pic.i_stride[2];
    m.p_fenc[0] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(0))
        .offset((0 as c_int + 0 as c_int * FENC_STRIDE) as isize) as *mut pixel;
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        m.p_fenc[1] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(1)).offset(
            ((0 as c_int >> (*h).mb.chroma_h_shift)
                + (0 as c_int >> (*h).mb.chroma_v_shift) * FENC_STRIDE) as isize,
        ) as *mut pixel;
        m.p_fenc[2] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(2)).offset(
            ((0 as c_int >> (*h).mb.chroma_h_shift)
                + (0 as c_int >> (*h).mb.chroma_v_shift) * FENC_STRIDE) as isize,
        ) as *mut pixel;
    }
    (*a).l0.me16x16.cost = INT_MAX;
    (*a).l1.me16x16.cost = INT_MAX;
    let mut l: c_int = 1 as c_int;
    while l >= 0 as c_int {
        let mut lX: *mut x264_mb_analysis_list_t = if l != 0 { &mut (*a).l1 } else { &mut (*a).l0 };
        i_ref = if list1_skipped != 0 && l == 1 as c_int {
            1 as c_int
        } else {
            0 as c_int
        };
        while i_ref < (*h).mb.pic.i_fref[l as usize] {
            if try_skip != 0 && l == 1 as c_int && i_ref > 0 as c_int {
                list1_skipped = 1 as c_int;
                break;
            } else {
                m.i_ref_cost = *(*a).p_cost_ref[l as usize].offset(i_ref as isize) as c_int;
                m.p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0))
                .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
                m.p_fref_w = m.p_fref[0];
                if (*h).param.analyse.i_subpel_refine != 0 {
                    m.p_fref[1] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1))
                    .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                        as *mut pixel;
                    m.p_fref[2] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2))
                    .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                        as *mut pixel;
                    m.p_fref[3] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(3))
                    .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                        as *mut pixel;
                }
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                    m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(4))
                    .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize)
                        as *mut pixel;
                    m.p_fref[8] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(8))
                    .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize)
                        as *mut pixel;
                    if (*h).param.analyse.i_subpel_refine != 0 {
                        m.p_fref[5] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(5))
                            .offset(
                                (0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[6] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(6))
                            .offset(
                                (0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[7] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(7))
                            .offset(
                                (0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[9] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(9 as c_int as isize))
                            .offset(
                                (0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[10] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(10 as c_int as isize))
                            .offset(
                                (0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[11] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(11 as c_int as isize))
                            .offset(
                                (0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                    }
                } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                    m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(4))
                    .offset(
                        (0 as c_int
                            + (0 as c_int >> (*h).mb.chroma_v_shift)
                                * *m.i_stride.as_mut_ptr().offset(1))
                            as isize,
                    ) as *mut pixel;
                }
                if (*h).param.analyse.me_method.exhaustive_search() {
                    m.integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .offset((0 as c_int + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                        as *mut uint16_t;
                }
                m.weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
                m.i_ref = i_ref;
                x264_10_mb_predict_mv_16x16(h, l, i_ref, m.mvp.as_mut_ptr());
                x264_10_mb_predict_mv_ref16x16(
                    h,
                    l,
                    i_ref,
                    mvc.as_mut_ptr() as *mut [int16_t; 2],
                    &mut i_mvc,
                );
                x264_10_me_search_ref(
                    h,
                    &mut m,
                    mvc.as_mut_ptr() as *mut [int16_t; 2],
                    i_mvc,
                    p_halfpel_thresh[l as usize],
                );
                m.cost += m.i_ref_cost;
                if m.cost < (*lX).me16x16.cost {
                    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                        &mut (*lX).me16x16 as *mut x264_me_t as *mut c_void,
                        &mut m as *mut x264_me_t as *const c_void,
                        size_of::<x264_me_t>() as size_t,
                    );
                }
                (*((*(*(*lX).mvc.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(0))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*(m.mv.as_mut_ptr() as *mut x264_union32_t)).i;
                (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .offset((*h).mb.i_mb_xy as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*(m.mv.as_mut_ptr() as *mut x264_union32_t)).i;
                if i_ref == 0 as c_int && try_skip != 0 {
                    if abs((*lX).me16x16.mv[0] as c_int
                        - (*h).mb.cache.direct_mv[l as usize][0][0] as c_int)
                        + abs((*lX).me16x16.mv[1] as c_int
                            - (*h).mb.cache.direct_mv[l as usize][0][1] as c_int)
                        > 1 as c_int
                    {
                        try_skip = 0 as c_int;
                    } else if l == 0 {
                        (*h).mb.i_type = B_SKIP as c_int;
                        analyse_update_cache(h, a);
                        return;
                    }
                }
                i_ref += 1;
            }
        }
        if list1_skipped != 0 && l == 1 as c_int && i_ref == (*h).mb.pic.i_fref[1] {
            break;
        }
        if list1_skipped != 0 && l == 0 as c_int {
            l = 1 as c_int;
        } else {
            l -= 1;
        }
    }
    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
        &mut (*a).l0.bi16x16 as *mut x264_me_t as *mut c_void,
        &mut (*a).l0.me16x16 as *mut x264_me_t as *const c_void,
        size_of::<x264_me_t>() as size_t,
    );
    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
        &mut (*a).l1.bi16x16 as *mut x264_me_t as *mut c_void,
        &mut (*a).l1.me16x16 as *mut x264_me_t as *const c_void,
        size_of::<x264_me_t>() as size_t,
    );
    let mut ref_costs: c_int = *(*a).p_cost_ref[0].offset((*a).l0.bi16x16.i_ref as isize) as c_int
        + *(*a).p_cost_ref[1].offset((*a).l1.bi16x16.i_ref as isize) as c_int;
    src0 = (*h).mc.get_ref.expect("non-null function pointer")(
        pix0.as_mut_ptr(),
        &mut stride0,
        (*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset((*a).l0.bi16x16.i_ref as isize))
        .as_mut_ptr(),
        (*h).mb.pic.i_stride[0] as intptr_t,
        (*a).l0.bi16x16.mv[0] as c_int,
        (*a).l0.bi16x16.mv[1] as c_int,
        16 as c_int,
        16 as c_int,
        x264_zero.as_mut_ptr() as *const x264_weight_t,
    );
    src1 = (*h).mc.get_ref.expect("non-null function pointer")(
        pix1.as_mut_ptr(),
        &mut stride1,
        (*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(1))
            .as_mut_ptr()
            .offset((*a).l1.bi16x16.i_ref as isize))
        .as_mut_ptr(),
        (*h).mb.pic.i_stride[0] as intptr_t,
        (*a).l1.bi16x16.mv[0] as c_int,
        (*a).l1.bi16x16.mv[1] as c_int,
        16 as c_int,
        16 as c_int,
        x264_zero.as_mut_ptr() as *const x264_weight_t,
    );
    (*h).mc.avg[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
        pix0.as_mut_ptr(),
        16 as intptr_t,
        src0,
        stride0,
        src1,
        stride1,
        (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
            [(*a).l1.bi16x16.i_ref as usize] as c_int,
    );
    (*a).i_cost16x16bi = (*h).pixf.mbcmp[PIXEL_16x16 as c_int as usize]
        .expect("non-null function pointer")(
        (*h).mb.pic.p_fenc[0],
        FENC_STRIDE as intptr_t,
        pix0.as_mut_ptr(),
        16 as intptr_t,
    ) + ref_costs
        + (*a).l0.bi16x16.cost_mv
        + (*a).l1.bi16x16.cost_mv;
    if (*h).mb.b_chroma_me != 0 {
        (*a).i_cost16x16bi += analyse_bi_chroma(h, a, 0 as c_int, PIXEL_16x16 as c_int);
    }
    if (*((*a).l0.bi16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i
        | (*((*a).l1.bi16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i
        != 0
    {
        let mut l0_mv_cost: c_int = *(*a)
            .l0
            .bi16x16
            .p_cost_mv
            .offset(-((*a).l0.bi16x16.mvp[0] as c_int) as isize)
            as c_int
            + *(*a)
                .l0
                .bi16x16
                .p_cost_mv
                .offset(-((*a).l0.bi16x16.mvp[1] as c_int) as isize) as c_int;
        let mut l1_mv_cost: c_int = *(*a)
            .l1
            .bi16x16
            .p_cost_mv
            .offset(-((*a).l1.bi16x16.mvp[0] as c_int) as isize)
            as c_int
            + *(*a)
                .l1
                .bi16x16
                .p_cost_mv
                .offset(-((*a).l1.bi16x16.mvp[1] as c_int) as isize) as c_int;
        (*h).mc.avg[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
            pix0.as_mut_ptr(),
            16 as intptr_t,
            (*h).mb.pic.p_fref[0][(*a).l0.bi16x16.i_ref as usize][0],
            (*h).mb.pic.i_stride[0] as intptr_t,
            (*h).mb.pic.p_fref[1][(*a).l1.bi16x16.i_ref as usize][0],
            (*h).mb.pic.i_stride[0] as intptr_t,
            (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
                [(*a).l1.bi16x16.i_ref as usize] as c_int,
        );
        let mut cost00: c_int = (*h).pixf.mbcmp[PIXEL_16x16 as c_int as usize]
            .expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[0],
            FENC_STRIDE as intptr_t,
            pix0.as_mut_ptr(),
            16 as intptr_t,
        ) + ref_costs
            + l0_mv_cost
            + l1_mv_cost;
        if (*h).mb.b_chroma_me != 0 && cost00 < (*a).i_cost16x16bi {
            let mut bi: [pixel; 256] = [0; 256];
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                (*h).mc.avg[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
                    bi.as_mut_ptr(),
                    FENC_STRIDE as intptr_t,
                    (*h).mb.pic.p_fref[0][(*a).l0.bi16x16.i_ref as usize][4],
                    (*h).mb.pic.i_stride[1] as intptr_t,
                    (*h).mb.pic.p_fref[1][(*a).l1.bi16x16.i_ref as usize][4],
                    (*h).mb.pic.i_stride[1] as intptr_t,
                    (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
                        [(*a).l1.bi16x16.i_ref as usize] as c_int,
                );
                cost00 += (*h).pixf.mbcmp[PIXEL_16x16 as c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[1],
                    FENC_STRIDE as intptr_t,
                    bi.as_mut_ptr(),
                    FENC_STRIDE as intptr_t,
                );
                (*h).mc.avg[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
                    bi.as_mut_ptr(),
                    FENC_STRIDE as intptr_t,
                    (*h).mb.pic.p_fref[0][(*a).l0.bi16x16.i_ref as usize][8],
                    (*h).mb.pic.i_stride[2] as intptr_t,
                    (*h).mb.pic.p_fref[1][(*a).l1.bi16x16.i_ref as usize][8],
                    (*h).mb.pic.i_stride[2] as intptr_t,
                    (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
                        [(*a).l1.bi16x16.i_ref as usize] as c_int,
                );
                cost00 += (*h).pixf.mbcmp[PIXEL_16x16 as c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[2],
                    FENC_STRIDE as intptr_t,
                    bi.as_mut_ptr(),
                    FENC_STRIDE as intptr_t,
                );
            } else {
                let mut pixuv: [[pixel; 256]; 2] = [[0; 256]; 2];
                let mut chromapix: c_int =
                    (*h).luma2chroma_pixel[PIXEL_16x16 as c_int as usize] as c_int;
                let mut v_shift: c_int = (*h).mb.chroma_v_shift;
                if v_shift & (*h).mb.b_interlaced & (*a).l0.bi16x16.i_ref != 0 {
                    let mut l0_mvy_offset: c_int =
                        ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int;
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        (*pixuv.as_mut_ptr().offset(0)).as_mut_ptr(),
                        (*pixuv.as_mut_ptr().offset(0)).as_mut_ptr().offset(8),
                        FENC_STRIDE as intptr_t,
                        (*h).mb.pic.p_fref[0][(*a).l0.bi16x16.i_ref as usize][4],
                        (*h).mb.pic.i_stride[1] as intptr_t,
                        0 as c_int,
                        0 as c_int + l0_mvy_offset,
                        8 as c_int,
                        8 as c_int,
                    );
                } else {
                    (*h).mc
                        .load_deinterleave_chroma_fenc
                        .expect("non-null function pointer")(
                        (*pixuv.as_mut_ptr().offset(0)).as_mut_ptr(),
                        (*h).mb.pic.p_fref[0][(*a).l0.bi16x16.i_ref as usize][4],
                        (*h).mb.pic.i_stride[1] as intptr_t,
                        16 as c_int >> v_shift,
                    );
                }
                if v_shift & (*h).mb.b_interlaced & (*a).l1.bi16x16.i_ref != 0 {
                    let mut l1_mvy_offset: c_int =
                        ((*h).mb.i_mb_y & 1 as c_int) * 4 as c_int - 2 as c_int;
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        (*pixuv.as_mut_ptr().offset(1)).as_mut_ptr(),
                        (*pixuv.as_mut_ptr().offset(1)).as_mut_ptr().offset(8),
                        FENC_STRIDE as intptr_t,
                        (*h).mb.pic.p_fref[1][(*a).l1.bi16x16.i_ref as usize][4],
                        (*h).mb.pic.i_stride[1] as intptr_t,
                        0 as c_int,
                        0 as c_int + l1_mvy_offset,
                        8 as c_int,
                        8 as c_int,
                    );
                } else {
                    (*h).mc
                        .load_deinterleave_chroma_fenc
                        .expect("non-null function pointer")(
                        (*pixuv.as_mut_ptr().offset(1)).as_mut_ptr(),
                        (*h).mb.pic.p_fref[1][(*a).l1.bi16x16.i_ref as usize][4],
                        (*h).mb.pic.i_stride[1] as intptr_t,
                        16 as c_int >> v_shift,
                    );
                }
                (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                    bi.as_mut_ptr(),
                    FENC_STRIDE as intptr_t,
                    (*pixuv.as_mut_ptr().offset(0)).as_mut_ptr(),
                    FENC_STRIDE as intptr_t,
                    (*pixuv.as_mut_ptr().offset(1)).as_mut_ptr(),
                    FENC_STRIDE as intptr_t,
                    (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
                        [(*a).l1.bi16x16.i_ref as usize] as c_int,
                );
                (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                    bi.as_mut_ptr().offset(8),
                    FENC_STRIDE as intptr_t,
                    (*pixuv.as_mut_ptr().offset(0)).as_mut_ptr().offset(8),
                    FENC_STRIDE as intptr_t,
                    (*pixuv.as_mut_ptr().offset(1)).as_mut_ptr().offset(8),
                    FENC_STRIDE as intptr_t,
                    (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
                        [(*a).l1.bi16x16.i_ref as usize] as c_int,
                );
                cost00 += (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[1],
                    FENC_STRIDE as intptr_t,
                    bi.as_mut_ptr(),
                    FENC_STRIDE as intptr_t,
                ) + (*h).pixf.mbcmp[chromapix as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[2],
                    FENC_STRIDE as intptr_t,
                    bi.as_mut_ptr().offset(8),
                    FENC_STRIDE as intptr_t,
                );
            }
        }
        if cost00 < (*a).i_cost16x16bi {
            (*((*a).l0.bi16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
            (*((*a).l1.bi16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
            (*a).l0.bi16x16.cost_mv = l0_mv_cost;
            (*a).l1.bi16x16.cost_mv = l1_mv_cost;
            (*a).i_cost16x16bi = cost00;
        }
    }
    (*a).i_cost16x16bi += (*a).i_lambda * i_mb_b_cost_table[B_BI_BI as c_int as usize] as c_int;
    (*a).l0.me16x16.cost += (*a).i_lambda * i_mb_b_cost_table[B_L0_L0 as c_int as usize] as c_int;
    (*a).l1.me16x16.cost += (*a).i_lambda * i_mb_b_cost_table[B_L1_L1 as c_int as usize] as c_int;
}
#[inline]
#[c2rust::src_loc = "2076:1"]
unsafe extern "C" fn mb_cache_mv_p8x8(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i: c_int,
) {
    let mut x: c_int = 2 as c_int * (i & 1 as c_int);
    let mut y: c_int = i & 2 as c_int;
    match (*h).mb.i_sub_partition[i as usize] as c_int {
        3 => {
            x264_macroblock_cache_mv(
                h,
                x,
                y,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*((*(*a).l0.me8x8.as_mut_ptr().offset(i as isize))
                    .mv
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
        }
        1 => {
            x264_macroblock_cache_mv(
                h,
                x,
                y + 0 as c_int,
                2 as c_int,
                1 as c_int,
                0 as c_int,
                (*((*(*(*a).l0.me8x4.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(0))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
            x264_macroblock_cache_mv(
                h,
                x,
                y + 1 as c_int,
                2 as c_int,
                1 as c_int,
                0 as c_int,
                (*((*(*(*a).l0.me8x4.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(1))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
        }
        2 => {
            x264_macroblock_cache_mv(
                h,
                x + 0 as c_int,
                y,
                1 as c_int,
                2 as c_int,
                0 as c_int,
                (*((*(*(*a).l0.me4x8.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(0))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
            x264_macroblock_cache_mv(
                h,
                x + 1 as c_int,
                y,
                1 as c_int,
                2 as c_int,
                0 as c_int,
                (*((*(*(*a).l0.me4x8.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(1))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
        }
        0 => {
            x264_macroblock_cache_mv(
                h,
                x + 0 as c_int,
                y + 0 as c_int,
                1 as c_int,
                1 as c_int,
                0 as c_int,
                (*((*(*(*a).l0.me4x4.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(0))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
            x264_macroblock_cache_mv(
                h,
                x + 1 as c_int,
                y + 0 as c_int,
                1 as c_int,
                1 as c_int,
                0 as c_int,
                (*((*(*(*a).l0.me4x4.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(1))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
            x264_macroblock_cache_mv(
                h,
                x + 0 as c_int,
                y + 1 as c_int,
                1 as c_int,
                1 as c_int,
                0 as c_int,
                (*((*(*(*a).l0.me4x4.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(2))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
            x264_macroblock_cache_mv(
                h,
                x + 1 as c_int,
                y + 1 as c_int,
                1 as c_int,
                1 as c_int,
                0 as c_int,
                (*((*(*(*a).l0.me4x4.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(3))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
        }
        _ => {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"internal error\n\0" as *const u8 as *const c_char,
            );
        }
    };
}
#[c2rust::src_loc = "2106:1"]
unsafe extern "C" fn mb_load_mv_direct8x8(mut h: *mut x264_t, mut idx: c_int) {
    let mut x: c_int = 2 as c_int * (idx & 1 as c_int);
    let mut y: c_int = idx & 2 as c_int;
    x264_macroblock_cache_ref(
        h,
        x,
        y,
        2 as c_int,
        2 as c_int,
        0 as c_int,
        (*h).mb.cache.direct_ref[0][idx as usize],
    );
    x264_macroblock_cache_ref(
        h,
        x,
        y,
        2 as c_int,
        2 as c_int,
        1 as c_int,
        (*h).mb.cache.direct_ref[1][idx as usize],
    );
    x264_macroblock_cache_mv(
        h,
        x,
        y,
        2 as c_int,
        2 as c_int,
        0 as c_int,
        (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset(idx as isize))
        .as_mut_ptr() as *mut x264_union32_t))
            .i,
    );
    x264_macroblock_cache_mv(
        h,
        x,
        y,
        2 as c_int,
        2 as c_int,
        1 as c_int,
        (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(1))
            .as_mut_ptr()
            .offset(idx as isize))
        .as_mut_ptr() as *mut x264_union32_t))
            .i,
    );
}
#[inline]
#[c2rust::src_loc = "2142:1"]
unsafe extern "C" fn mb_cache_mv_b8x8(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i: c_int,
    mut b_mvd: c_int,
) {
    let mut x: c_int = 2 as c_int * (i & 1 as c_int);
    let mut y: c_int = i & 2 as c_int;
    if (*h).mb.i_sub_partition[i as usize] as c_int == D_DIRECT_8x8 as c_int {
        mb_load_mv_direct8x8(h, i);
        if b_mvd != 0 {
            x264_macroblock_cache_mvd(h, x, y, 2 as c_int, 2 as c_int, 0 as c_int, 0 as uint16_t);
            x264_macroblock_cache_mvd(h, x, y, 2 as c_int, 2 as c_int, 1 as c_int, 0 as uint16_t);
            x264_macroblock_cache_skip(h, x, y, 2 as c_int, 2 as c_int, 1 as c_int);
        }
    } else {
        if x264_mb_partition_listX_table[0][(*h).mb.i_sub_partition[i as usize] as usize] != 0 {
            x264_macroblock_cache_ref(
                h,
                x,
                y,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[i as usize].i_ref as int8_t,
            );
            x264_macroblock_cache_mv(
                h,
                x,
                y,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*((*(*a).l0.me8x8.as_mut_ptr().offset(i as isize))
                    .mv
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
        } else {
            x264_macroblock_cache_ref(h, x, y, 2 as c_int, 2 as c_int, 0 as c_int, -1 as int8_t);
            x264_macroblock_cache_mv(h, x, y, 2 as c_int, 2 as c_int, 0 as c_int, 0 as uint32_t);
            if b_mvd != 0 {
                x264_macroblock_cache_mvd(
                    h,
                    x,
                    y,
                    2 as c_int,
                    2 as c_int,
                    0 as c_int,
                    0 as uint16_t,
                );
            }
        }
        if x264_mb_partition_listX_table[1][(*h).mb.i_sub_partition[i as usize] as usize] != 0 {
            x264_macroblock_cache_ref(
                h,
                x,
                y,
                2 as c_int,
                2 as c_int,
                1 as c_int,
                (*a).l1.me8x8[i as usize].i_ref as int8_t,
            );
            x264_macroblock_cache_mv(
                h,
                x,
                y,
                2 as c_int,
                2 as c_int,
                1 as c_int,
                (*((*(*a).l1.me8x8.as_mut_ptr().offset(i as isize))
                    .mv
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i,
            );
        } else {
            x264_macroblock_cache_ref(h, x, y, 2 as c_int, 2 as c_int, 1 as c_int, -1 as int8_t);
            x264_macroblock_cache_mv(h, x, y, 2 as c_int, 2 as c_int, 1 as c_int, 0 as uint32_t);
            if b_mvd != 0 {
                x264_macroblock_cache_mvd(
                    h,
                    x,
                    y,
                    2 as c_int,
                    2 as c_int,
                    1 as c_int,
                    0 as uint16_t,
                );
            }
        }
    };
}
#[inline]
#[c2rust::src_loc = "2161:1"]
unsafe extern "C" fn mb_cache_mv_b16x8(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i: c_int,
    mut b_mvd: c_int,
) {
    if x264_mb_partition_listX_table[0][(*a).i_mb_partition16x8[i as usize] as usize] != 0 {
        x264_macroblock_cache_ref(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            0 as c_int,
            (*a).l0.me16x8[i as usize].i_ref as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            0 as c_int,
            (*((*(*a).l0.me16x8.as_mut_ptr().offset(i as isize))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                .i,
        );
    } else {
        x264_macroblock_cache_ref(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            0 as c_int,
            -1 as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            0 as c_int,
            0 as uint32_t,
        );
        if b_mvd != 0 {
            x264_macroblock_cache_mvd(
                h,
                0 as c_int,
                2 as c_int * i,
                4 as c_int,
                2 as c_int,
                0 as c_int,
                0 as uint16_t,
            );
        }
    }
    if x264_mb_partition_listX_table[1][(*a).i_mb_partition16x8[i as usize] as usize] != 0 {
        x264_macroblock_cache_ref(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            1 as c_int,
            (*a).l1.me16x8[i as usize].i_ref as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            1 as c_int,
            (*((*(*a).l1.me16x8.as_mut_ptr().offset(i as isize))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                .i,
        );
    } else {
        x264_macroblock_cache_ref(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            1 as c_int,
            -1 as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            0 as c_int,
            2 as c_int * i,
            4 as c_int,
            2 as c_int,
            1 as c_int,
            0 as uint32_t,
        );
        if b_mvd != 0 {
            x264_macroblock_cache_mvd(
                h,
                0 as c_int,
                2 as c_int * i,
                4 as c_int,
                2 as c_int,
                1 as c_int,
                0 as uint16_t,
            );
        }
    };
}
#[inline]
#[c2rust::src_loc = "2165:1"]
unsafe extern "C" fn mb_cache_mv_b8x16(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i: c_int,
    mut b_mvd: c_int,
) {
    if x264_mb_partition_listX_table[0][(*a).i_mb_partition8x16[i as usize] as usize] != 0 {
        x264_macroblock_cache_ref(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            0 as c_int,
            (*a).l0.me8x16[i as usize].i_ref as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            0 as c_int,
            (*((*(*a).l0.me8x16.as_mut_ptr().offset(i as isize))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                .i,
        );
    } else {
        x264_macroblock_cache_ref(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            0 as c_int,
            -1 as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            0 as c_int,
            0 as uint32_t,
        );
        if b_mvd != 0 {
            x264_macroblock_cache_mvd(
                h,
                2 as c_int * i,
                0 as c_int,
                2 as c_int,
                4 as c_int,
                0 as c_int,
                0 as uint16_t,
            );
        }
    }
    if x264_mb_partition_listX_table[1][(*a).i_mb_partition8x16[i as usize] as usize] != 0 {
        x264_macroblock_cache_ref(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            1 as c_int,
            (*a).l1.me8x16[i as usize].i_ref as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            1 as c_int,
            (*((*(*a).l1.me8x16.as_mut_ptr().offset(i as isize))
                .mv
                .as_mut_ptr() as *mut x264_union32_t))
                .i,
        );
    } else {
        x264_macroblock_cache_ref(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            1 as c_int,
            -1 as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            2 as c_int * i,
            0 as c_int,
            2 as c_int,
            4 as c_int,
            1 as c_int,
            0 as uint32_t,
        );
        if b_mvd != 0 {
            x264_macroblock_cache_mvd(
                h,
                2 as c_int * i,
                0 as c_int,
                2 as c_int,
                4 as c_int,
                1 as c_int,
                0 as uint16_t,
            );
        }
    };
}
#[c2rust::src_loc = "2171:1"]
unsafe extern "C" fn mb_analyse_inter_b8x8_mixed_ref(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
) {
    let mut pix: [[pixel; 64]; 2] = [[0; 64]; 2];
    let mut i_maxref: [c_int; 2] = [
        (*h).mb.pic.i_fref[0] - 1 as c_int,
        (*h).mb.pic.i_fref[1] - 1 as c_int,
    ];
    let mut l: c_int = 0 as c_int;
    while l < 2 as c_int {
        let mut lX: *mut x264_mb_analysis_list_t = if l != 0 { &mut (*a).l1 } else { &mut (*a).l0 };
        if i_maxref[l as usize] > 0 as c_int
            && (*lX).me16x16.i_ref == 0 as c_int
            && (*h).mb.i_mb_type_top > 0 as c_int
            && (*h).mb.i_mb_type_left[0] > 0 as c_int
        {
            i_maxref[l as usize] = 0 as c_int;
            let mut ref_0: c_int = (*h).mb.cache.ref_0[l as usize]
                [(X264_SCAN8_0 + -(8 as c_int) - 1 as c_int) as usize]
                as c_int;
            if ref_0 > i_maxref[l as usize] {
                i_maxref[l as usize] = ref_0;
            }
            let mut ref_1: c_int = (*h).mb.cache.ref_0[l as usize]
                [(X264_SCAN8_0 + -(8 as c_int) + 0 as c_int) as usize]
                as c_int;
            if ref_1 > i_maxref[l as usize] {
                i_maxref[l as usize] = ref_1;
            }
            let mut ref_2: c_int = (*h).mb.cache.ref_0[l as usize]
                [(X264_SCAN8_0 + -(8 as c_int) + 2 as c_int) as usize]
                as c_int;
            if ref_2 > i_maxref[l as usize] {
                i_maxref[l as usize] = ref_2;
            }
            let mut ref_3: c_int = (*h).mb.cache.ref_0[l as usize]
                [(X264_SCAN8_0 + -(8 as c_int) + 4 as c_int) as usize]
                as c_int;
            if ref_3 > i_maxref[l as usize] {
                i_maxref[l as usize] = ref_3;
            }
            let mut ref_4: c_int = (*h).mb.cache.ref_0[l as usize]
                [(X264_SCAN8_0 + 0 as c_int - 1 as c_int) as usize]
                as c_int;
            if ref_4 > i_maxref[l as usize] {
                i_maxref[l as usize] = ref_4;
            }
            let mut ref_5: c_int = (*h).mb.cache.ref_0[l as usize]
                [(X264_SCAN8_0 + 2 as c_int * 8 as c_int - 1 as c_int) as usize]
                as c_int;
            if ref_5 > i_maxref[l as usize] {
                i_maxref[l as usize] = ref_5;
            }
        }
        l += 1;
    }
    (*h).mb.i_partition = D_8x8 as c_int;
    (*a).i_cost8x8bi = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        let mut x8: c_int = i & 1 as c_int;
        let mut y8: c_int = i >> 1 as c_int;
        let mut i_part_cost: c_int = 0;
        let mut i_part_cost_bi: c_int = 0;
        let mut stride: [intptr_t; 2] = [8 as c_int as intptr_t, 8 as c_int as intptr_t];
        let mut src: [*mut pixel; 2] = [0 as *mut pixel; 2];
        let mut m: x264_me_t = x264_me_t {
            i_pixel: 0,
            p_cost_mv: 0 as *mut uint16_t,
            i_ref_cost: 0,
            i_ref: 0,
            weight: 0 as *const x264_weight_t,
            p_fref: [0 as *mut pixel; 12],
            p_fref_w: 0 as *mut pixel,
            p_fenc: [0 as *mut pixel; 3],
            integral: 0 as *mut uint16_t,
            i_stride: [0; 3],
            mvp: [0; 2],
            cost_mv: 0,
            cost: 0,
            mv: [0; 2],
        };
        m.i_pixel = PIXEL_8x8 as c_int;
        m.p_cost_mv = (*a).p_cost_mv;
        m.i_stride[0] = (*h).mb.pic.i_stride[0];
        m.i_stride[1] = (*h).mb.pic.i_stride[1];
        m.i_stride[2] = (*h).mb.pic.i_stride[2];
        m.p_fenc[0] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(0))
            .offset((8 as c_int * x8 + 8 as c_int * y8 * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            m.p_fenc[1] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(1)).offset(
                ((8 as c_int * x8 >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * y8 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            m.p_fenc[2] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(2)).offset(
                ((8 as c_int * x8 >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * y8 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        let mut l_0: c_int = 0 as c_int;
        while l_0 < 2 as c_int {
            let mut lX_0: *mut x264_mb_analysis_list_t =
                if l_0 != 0 { &mut (*a).l1 } else { &mut (*a).l0 };
            (*lX_0).me8x8[i as usize].cost = INT_MAX;
            let mut i_ref: c_int = 0 as c_int;
            while i_ref <= i_maxref[l_0 as usize] {
                m.i_ref_cost = *(*a).p_cost_ref[l_0 as usize].offset(i_ref as isize) as c_int;
                m.p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                        as isize,
                ) as *mut pixel;
                m.p_fref_w = m.p_fref[0];
                if (*h).param.analyse.i_subpel_refine != 0 {
                    m.p_fref[1] =
                        &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                            .as_mut_ptr()
                            .offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset(1))
                        .offset(
                            (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                                as isize,
                        ) as *mut pixel;
                    m.p_fref[2] =
                        &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                            .as_mut_ptr()
                            .offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset(2))
                        .offset(
                            (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                                as isize,
                        ) as *mut pixel;
                    m.p_fref[3] =
                        &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                            .as_mut_ptr()
                            .offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset(3))
                        .offset(
                            (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                                as isize,
                        ) as *mut pixel;
                }
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                    m.p_fref[4] =
                        &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                            .as_mut_ptr()
                            .offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset(4))
                        .offset(
                            (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(1))
                                as isize,
                        ) as *mut pixel;
                    m.p_fref[8] =
                        &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                            .as_mut_ptr()
                            .offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset(8))
                        .offset(
                            (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(2))
                                as isize,
                        ) as *mut pixel;
                    if (*h).param.analyse.i_subpel_refine != 0 {
                        m.p_fref[5] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(5))
                            .offset(
                                (8 as c_int * x8
                                    + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[6] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(6))
                            .offset(
                                (8 as c_int * x8
                                    + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[7] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(7))
                            .offset(
                                (8 as c_int * x8
                                    + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[9] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(9 as c_int as isize))
                            .offset(
                                (8 as c_int * x8
                                    + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[10] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(10 as c_int as isize))
                            .offset(
                                (8 as c_int * x8
                                    + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[11] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(11 as c_int as isize))
                            .offset(
                                (8 as c_int * x8
                                    + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                    }
                } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                    m.p_fref[4] =
                        &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l_0 as isize))
                            .as_mut_ptr()
                            .offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset(4))
                        .offset(
                            (8 as c_int * x8
                                + (8 as c_int * y8 >> (*h).mb.chroma_v_shift)
                                    * *m.i_stride.as_mut_ptr().offset(1))
                                as isize,
                        ) as *mut pixel;
                }
                if (*h).param.analyse.me_method.exhaustive_search() {
                    m.integral =
                        &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(l_0 as isize))
                            .as_mut_ptr()
                            .offset(i_ref as isize))
                        .offset(
                            (8 as c_int * x8 + 8 as c_int * y8 * *m.i_stride.as_mut_ptr().offset(0))
                                as isize,
                        ) as *mut uint16_t;
                }
                m.weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
                m.i_ref = i_ref;
                x264_macroblock_cache_ref(
                    h,
                    x8 * 2 as c_int,
                    y8 * 2 as c_int,
                    2 as c_int,
                    2 as c_int,
                    l_0,
                    i_ref as int8_t,
                );
                x264_10_mb_predict_mv(h, l_0, 4 as c_int * i, 2 as c_int, m.mvp.as_mut_ptr());
                x264_10_me_search_ref(
                    h,
                    &mut m,
                    (*(*lX_0).mvc.as_mut_ptr().offset(i_ref as isize)).as_mut_ptr()
                        as *mut [int16_t; 2],
                    i + 1 as c_int,
                    0 as *mut c_int,
                );
                m.cost += m.i_ref_cost;
                if m.cost < (*lX_0).me8x8[i as usize].cost {
                    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                        &mut *(*lX_0).me8x8.as_mut_ptr().offset(i as isize) as *mut x264_me_t
                            as *mut c_void,
                        &mut m as *mut x264_me_t as *const c_void,
                        size_of::<x264_me_t>() as size_t,
                    );
                    (*a).i_satd8x8[l_0 as usize][i as usize] = m.cost - (m.cost_mv + m.i_ref_cost);
                }
                (*((*(*(*lX_0).mvc.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset((i + 1 as c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*(m.mv.as_mut_ptr() as *mut x264_union32_t)).i;
                i_ref += 1;
            }
            l_0 += 1;
        }
        src[0] = (*h).mc.get_ref.expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            &mut *stride.as_mut_ptr().offset(0),
            (*(*a).l0.me8x8.as_mut_ptr().offset(i as isize))
                .p_fref
                .as_mut_ptr(),
            (*a).l0.me8x8[i as usize].i_stride[0] as intptr_t,
            (*a).l0.me8x8[i as usize].mv[0] as c_int,
            (*a).l0.me8x8[i as usize].mv[1] as c_int,
            8 as c_int,
            8 as c_int,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src[1] = (*h).mc.get_ref.expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
            &mut *stride.as_mut_ptr().offset(1),
            (*(*a).l1.me8x8.as_mut_ptr().offset(i as isize))
                .p_fref
                .as_mut_ptr(),
            (*a).l1.me8x8[i as usize].i_stride[0] as intptr_t,
            (*a).l1.me8x8[i as usize].mv[0] as c_int,
            (*a).l1.me8x8[i as usize].mv[1] as c_int,
            8 as c_int,
            8 as c_int,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        (*h).mc.avg[PIXEL_8x8 as c_int as usize].expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            8 as intptr_t,
            src[0],
            stride[0],
            src[1],
            stride[1],
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me8x8[i as usize].i_ref as isize))
                [(*a).l1.me8x8[i as usize].i_ref as usize] as c_int,
        );
        (*a).i_satd8x8[2][i as usize] = (*h).pixf.mbcmp[PIXEL_8x8 as c_int as usize]
            .expect("non-null function pointer")(
            (*a).l0.me8x8[i as usize].p_fenc[0],
            FENC_STRIDE as intptr_t,
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            8 as intptr_t,
        );
        i_part_cost_bi = (*a).i_satd8x8[2][i as usize]
            + (*a).l0.me8x8[i as usize].cost_mv
            + (*a).l1.me8x8[i as usize].cost_mv
            + (*a).l0.me8x8[i as usize].i_ref_cost
            + (*a).l1.me8x8[i as usize].i_ref_cost
            + (*a).i_lambda * i_sub_mb_b_cost_table[D_BI_8x8 as c_int as usize] as c_int;
        if (*h).mb.b_chroma_me != 0 {
            let mut i_chroma_cost: c_int = analyse_bi_chroma(h, a, i, PIXEL_8x8 as c_int);
            i_part_cost_bi += i_chroma_cost;
            (*a).i_satd8x8[2][i as usize] += i_chroma_cost;
        }
        (*a).l0.me8x8[i as usize].cost +=
            (*a).i_lambda * i_sub_mb_b_cost_table[D_L0_8x8 as c_int as usize] as c_int;
        (*a).l1.me8x8[i as usize].cost +=
            (*a).i_lambda * i_sub_mb_b_cost_table[D_L1_8x8 as c_int as usize] as c_int;
        i_part_cost = (*a).l0.me8x8[i as usize].cost;
        (*h).mb.i_sub_partition[i as usize] = D_L0_8x8 as c_int as uint8_t;
        if (*a).l1.me8x8[i as usize].cost < i_part_cost {
            i_part_cost = (*a).l1.me8x8[i as usize].cost;
            (*h).mb.i_sub_partition[i as usize] = D_L1_8x8 as c_int as uint8_t;
        }
        if i_part_cost_bi < i_part_cost {
            i_part_cost = i_part_cost_bi;
            (*h).mb.i_sub_partition[i as usize] = D_BI_8x8 as c_int as uint8_t;
        }
        if (*a).i_cost8x8direct[i as usize] < i_part_cost {
            i_part_cost = (*a).i_cost8x8direct[i as usize];
            (*h).mb.i_sub_partition[i as usize] = D_DIRECT_8x8 as c_int as uint8_t;
        }
        (*a).i_cost8x8bi += i_part_cost;
        mb_cache_mv_b8x8(h, a, i, 0 as c_int);
        i += 1;
    }
    (*a).i_cost8x8bi += (*a).i_lambda * i_mb_b_cost_table[B_8x8 as c_int as usize] as c_int;
}
#[c2rust::src_loc = "2283:1"]
unsafe extern "C" fn mb_analyse_inter_b8x8(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    let mut p_fref: [*mut *mut pixel; 2] = [
        (*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset((*a).l0.me16x16.i_ref as isize))
        .as_mut_ptr(),
        (*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(1))
            .as_mut_ptr()
            .offset((*a).l1.me16x16.i_ref as isize))
        .as_mut_ptr(),
    ];
    let mut pix: [[pixel; 64]; 2] = [[0; 64]; 2];
    (*h).mb.i_partition = D_8x8 as c_int;
    (*a).i_cost8x8bi = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        let mut x8: c_int = i & 1 as c_int;
        let mut y8: c_int = i >> 1 as c_int;
        let mut i_part_cost: c_int = 0;
        let mut i_part_cost_bi: c_int = 0 as c_int;
        let mut stride: [intptr_t; 2] = [8 as c_int as intptr_t, 8 as c_int as intptr_t];
        let mut src: [*mut pixel; 2] = [0 as *mut pixel; 2];
        let mut l: c_int = 0 as c_int;
        while l < 2 as c_int {
            let mut lX: *mut x264_mb_analysis_list_t =
                if l != 0 { &mut (*a).l1 } else { &mut (*a).l0 };
            let mut m: *mut x264_me_t =
                &mut *(*lX).me8x8.as_mut_ptr().offset(i as isize) as *mut x264_me_t;
            (*m).i_pixel = PIXEL_8x8 as c_int;
            (*m).p_cost_mv = (*a).p_cost_mv;
            (*m).i_stride[0] = (*h).mb.pic.i_stride[0];
            (*m).i_stride[1] = (*h).mb.pic.i_stride[1];
            (*m).i_stride[2] = (*h).mb.pic.i_stride[2];
            (*m).p_fenc[0] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(0))
                .offset((8 as c_int * x8 + 8 as c_int * y8 * FENC_STRIDE) as isize)
                as *mut pixel;
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                (*m).p_fenc[1] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(1)).offset(
                    ((8 as c_int * x8 >> (*h).mb.chroma_h_shift)
                        + (8 as c_int * y8 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                        as isize,
                ) as *mut pixel;
                (*m).p_fenc[2] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(2)).offset(
                    ((8 as c_int * x8 >> (*h).mb.chroma_h_shift)
                        + (8 as c_int * y8 >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                        as isize,
                ) as *mut pixel;
            }
            (*m).i_ref_cost =
                *(*a).p_cost_ref[l as usize].offset((*lX).me16x16.i_ref as isize) as c_int;
            (*m).i_ref = (*lX).me16x16.i_ref;
            (*m).p_fref[0] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(0)).offset(
                (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                    as isize,
            ) as *mut pixel;
            (*m).p_fref_w = (*m).p_fref[0];
            if (*h).param.analyse.i_subpel_refine != 0 {
                (*m).p_fref[1] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(1))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                            as isize,
                    ) as *mut pixel;
                (*m).p_fref[2] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(2))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                            as isize,
                    ) as *mut pixel;
                (*m).p_fref[3] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(3))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                            as isize,
                    ) as *mut pixel;
            }
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                (*m).p_fref[4] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(4))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(1))
                            as isize,
                    ) as *mut pixel;
                (*m).p_fref[8] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(8))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(2))
                            as isize,
                    ) as *mut pixel;
                if (*h).param.analyse.i_subpel_refine != 0 {
                    (*m).p_fref[5] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(5))
                        .offset(
                            (8 as c_int * x8
                                + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(1))
                                as isize,
                        ) as *mut pixel;
                    (*m).p_fref[6] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(6))
                        .offset(
                            (8 as c_int * x8
                                + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(1))
                                as isize,
                        ) as *mut pixel;
                    (*m).p_fref[7] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(7))
                        .offset(
                            (8 as c_int * x8
                                + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(1))
                                as isize,
                        ) as *mut pixel;
                    (*m).p_fref[9] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize))
                        .offset(9 as c_int as isize))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(2))
                            as isize,
                    ) as *mut pixel;
                    (*m).p_fref[10] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize))
                        .offset(10 as c_int as isize))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(2))
                            as isize,
                    ) as *mut pixel;
                    (*m).p_fref[11] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize))
                        .offset(11 as c_int as isize))
                    .offset(
                        (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(2))
                            as isize,
                    ) as *mut pixel;
                }
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                (*m).p_fref[4] = &mut *(*(*p_fref.as_mut_ptr().offset(l as isize)).offset(4))
                    .offset(
                        (8 as c_int * x8
                            + (8 as c_int * y8 >> (*h).mb.chroma_v_shift)
                                * *(*m).i_stride.as_mut_ptr().offset(1))
                            as isize,
                    ) as *mut pixel;
            }
            if (*h).param.analyse.me_method.exhaustive_search() {
                (*m).integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset((*lX).me16x16.i_ref as isize))
                .offset(
                    (8 as c_int * x8 + 8 as c_int * y8 * *(*m).i_stride.as_mut_ptr().offset(0))
                        as isize,
                ) as *mut uint16_t;
            }
            (*m).weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
            (*m).i_ref = (*lX).me16x16.i_ref;
            x264_macroblock_cache_ref(
                h,
                x8 * 2 as c_int,
                y8 * 2 as c_int,
                2 as c_int,
                2 as c_int,
                l,
                (*lX).me16x16.i_ref as int8_t,
            );
            x264_10_mb_predict_mv(h, l, 4 as c_int * i, 2 as c_int, (*m).mvp.as_mut_ptr());
            x264_10_me_search_ref(h, m, &mut (*lX).me16x16.mv, 1 as c_int, 0 as *mut c_int);
            (*a).i_satd8x8[l as usize][i as usize] = (*m).cost - (*m).cost_mv;
            (*m).cost += (*m).i_ref_cost;
            x264_macroblock_cache_mv(
                h,
                2 as c_int * x8,
                2 as c_int * y8,
                2 as c_int,
                2 as c_int,
                l,
                (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i,
            );
            (*((*(*(*lX).mvc.as_mut_ptr().offset((*lX).me16x16.i_ref as isize))
                .as_mut_ptr()
                .offset((i + 1 as c_int) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i;
            src[l as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                (*pix.as_mut_ptr().offset(l as isize)).as_mut_ptr(),
                &mut *stride.as_mut_ptr().offset(l as isize),
                (*m).p_fref.as_mut_ptr(),
                (*m).i_stride[0] as intptr_t,
                (*m).mv[0] as c_int,
                (*m).mv[1] as c_int,
                8 as c_int,
                8 as c_int,
                x264_zero.as_mut_ptr() as *const x264_weight_t,
            );
            i_part_cost_bi += (*m).cost_mv + (*m).i_ref_cost;
            l += 1;
        }
        (*h).mc.avg[PIXEL_8x8 as c_int as usize].expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            8 as intptr_t,
            src[0],
            stride[0],
            src[1],
            stride[1],
            (*(*h).mb.bipred_weight.offset((*a).l0.me16x16.i_ref as isize))
                [(*a).l1.me16x16.i_ref as usize] as c_int,
        );
        (*a).i_satd8x8[2][i as usize] = (*h).pixf.mbcmp[PIXEL_8x8 as c_int as usize]
            .expect("non-null function pointer")(
            (*a).l0.me8x8[i as usize].p_fenc[0],
            FENC_STRIDE as intptr_t,
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            8 as intptr_t,
        );
        i_part_cost_bi += (*a).i_satd8x8[2][i as usize]
            + (*a).i_lambda * i_sub_mb_b_cost_table[D_BI_8x8 as c_int as usize] as c_int;
        (*a).l0.me8x8[i as usize].cost +=
            (*a).i_lambda * i_sub_mb_b_cost_table[D_L0_8x8 as c_int as usize] as c_int;
        (*a).l1.me8x8[i as usize].cost +=
            (*a).i_lambda * i_sub_mb_b_cost_table[D_L1_8x8 as c_int as usize] as c_int;
        if (*h).mb.b_chroma_me != 0 {
            let mut i_chroma_cost: c_int = analyse_bi_chroma(h, a, i, PIXEL_8x8 as c_int);
            i_part_cost_bi += i_chroma_cost;
            (*a).i_satd8x8[2][i as usize] += i_chroma_cost;
        }
        i_part_cost = (*a).l0.me8x8[i as usize].cost;
        (*h).mb.i_sub_partition[i as usize] = D_L0_8x8 as c_int as uint8_t;
        if (*a).l1.me8x8[i as usize].cost < i_part_cost {
            i_part_cost = (*a).l1.me8x8[i as usize].cost;
            (*h).mb.i_sub_partition[i as usize] = D_L1_8x8 as c_int as uint8_t;
        }
        if i_part_cost_bi < i_part_cost {
            i_part_cost = i_part_cost_bi;
            (*h).mb.i_sub_partition[i as usize] = D_BI_8x8 as c_int as uint8_t;
        }
        if (*a).i_cost8x8direct[i as usize] < i_part_cost {
            i_part_cost = (*a).i_cost8x8direct[i as usize];
            (*h).mb.i_sub_partition[i as usize] = D_DIRECT_8x8 as c_int as uint8_t;
        }
        (*a).i_cost8x8bi += i_part_cost;
        mb_cache_mv_b8x8(h, a, i, 0 as c_int);
        i += 1;
    }
    (*a).i_cost8x8bi += (*a).i_lambda * i_mb_b_cost_table[B_8x8 as c_int as usize] as c_int;
}
#[c2rust::src_loc = "2360:1"]
unsafe extern "C" fn mb_analyse_inter_b16x8(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_best_satd: c_int,
) {
    let mut pix: [[pixel; 128]; 2] = [[0; 128]; 2];
    let mut mvc: [[int16_t; 2]; 3] = [[0; 2]; 3];
    (*h).mb.i_partition = D_16x8 as c_int;
    (*a).i_cost16x8bi = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 2 as c_int {
        let mut i_part_cost: c_int = 0;
        let mut i_part_cost_bi: c_int = 0 as c_int;
        let mut stride: [intptr_t; 2] = [16 as c_int as intptr_t, 16 as c_int as intptr_t];
        let mut src: [*mut pixel; 2] = [0 as *mut pixel; 2];
        let mut m: x264_me_t = x264_me_t {
            i_pixel: 0,
            p_cost_mv: 0 as *mut uint16_t,
            i_ref_cost: 0,
            i_ref: 0,
            weight: 0 as *const x264_weight_t,
            p_fref: [0 as *mut pixel; 12],
            p_fref_w: 0 as *mut pixel,
            p_fenc: [0 as *mut pixel; 3],
            integral: 0 as *mut uint16_t,
            i_stride: [0; 3],
            mvp: [0; 2],
            cost_mv: 0,
            cost: 0,
            mv: [0; 2],
        };
        m.i_pixel = PIXEL_16x8 as c_int;
        m.p_cost_mv = (*a).p_cost_mv;
        m.i_stride[0] = (*h).mb.pic.i_stride[0];
        m.i_stride[1] = (*h).mb.pic.i_stride[1];
        m.i_stride[2] = (*h).mb.pic.i_stride[2];
        m.p_fenc[0] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(0))
            .offset((0 as c_int + 8 as c_int * i * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            m.p_fenc[1] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(1)).offset(
                ((0 as c_int >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * i >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            m.p_fenc[2] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(2)).offset(
                ((0 as c_int >> (*h).mb.chroma_h_shift)
                    + (8 as c_int * i >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        let mut l: c_int = 0 as c_int;
        while l < 2 as c_int {
            let mut lX: *mut x264_mb_analysis_list_t =
                if l != 0 { &mut (*a).l1 } else { &mut (*a).l0 };
            let mut ref8: [c_int; 2] = [
                (*lX).me8x8[(2 as c_int * i) as usize].i_ref,
                (*lX).me8x8[(2 as c_int * i + 1 as c_int) as usize].i_ref,
            ];
            let mut i_ref8s: c_int = if ref8[0] == ref8[1] {
                1 as c_int
            } else {
                2 as c_int
            };
            (*lX).me16x8[i as usize].cost = INT_MAX;
            let mut j: c_int = 0 as c_int;
            while j < i_ref8s {
                let mut i_ref: c_int = ref8[j as usize];
                m.i_ref_cost = *(*a).p_cost_ref[l as usize].offset(i_ref as isize) as c_int;
                m.p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0))
                .offset((0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
                m.p_fref_w = m.p_fref[0];
                if (*h).param.analyse.i_subpel_refine != 0 {
                    m.p_fref[1] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize,
                    ) as *mut pixel;
                    m.p_fref[2] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize,
                    ) as *mut pixel;
                    m.p_fref[3] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(3))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize,
                    ) as *mut pixel;
                }
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                    m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(4))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                    ) as *mut pixel;
                    m.p_fref[8] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(8))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(2)) as isize,
                    ) as *mut pixel;
                    if (*h).param.analyse.i_subpel_refine != 0 {
                        m.p_fref[5] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(5))
                            .offset(
                                (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[6] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(6))
                            .offset(
                                (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[7] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(7))
                            .offset(
                                (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[9] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(9 as c_int as isize))
                            .offset(
                                (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[10] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(10 as c_int as isize))
                            .offset(
                                (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[11] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(11 as c_int as isize))
                            .offset(
                                (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                    }
                } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                    m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(4))
                    .offset(
                        (0 as c_int
                            + (8 as c_int * i >> (*h).mb.chroma_v_shift)
                                * *m.i_stride.as_mut_ptr().offset(1))
                            as isize,
                    ) as *mut pixel;
                }
                if (*h).param.analyse.me_method.exhaustive_search() {
                    m.integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .offset(
                        (0 as c_int + 8 as c_int * i * *m.i_stride.as_mut_ptr().offset(0)) as isize,
                    ) as *mut uint16_t;
                }
                m.weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
                m.i_ref = i_ref;
                (*((*mvc.as_mut_ptr().offset(0)).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*((*(*(*lX).mvc.as_mut_ptr().offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset(0))
                    .as_mut_ptr() as *mut x264_union32_t))
                        .i;
                (*((*mvc.as_mut_ptr().offset(1)).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*((*(*(*lX).mvc.as_mut_ptr().offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset((2 as c_int * i + 1 as c_int) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                        .i;
                (*((*mvc.as_mut_ptr().offset(2)).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*((*(*(*lX).mvc.as_mut_ptr().offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset((2 as c_int * i + 2 as c_int) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                        .i;
                x264_macroblock_cache_ref(
                    h,
                    0 as c_int,
                    2 as c_int * i,
                    4 as c_int,
                    2 as c_int,
                    l,
                    i_ref as int8_t,
                );
                x264_10_mb_predict_mv(h, l, 8 as c_int * i, 4 as c_int, m.mvp.as_mut_ptr());
                x264_10_me_search_ref(
                    h,
                    &mut m,
                    mvc.as_mut_ptr() as *mut [int16_t; 2],
                    3 as c_int,
                    0 as *mut c_int,
                );
                m.cost += m.i_ref_cost;
                if m.cost < (*lX).me16x8[i as usize].cost {
                    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                        &mut *(*lX).me16x8.as_mut_ptr().offset(i as isize) as *mut x264_me_t
                            as *mut c_void,
                        &mut m as *mut x264_me_t as *const c_void,
                        size_of::<x264_me_t>() as size_t,
                    );
                }
                j += 1;
            }
            l += 1;
        }
        src[0] = (*h).mc.get_ref.expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            &mut *stride.as_mut_ptr().offset(0),
            (*(*a).l0.me16x8.as_mut_ptr().offset(i as isize))
                .p_fref
                .as_mut_ptr(),
            (*a).l0.me16x8[i as usize].i_stride[0] as intptr_t,
            (*a).l0.me16x8[i as usize].mv[0] as c_int,
            (*a).l0.me16x8[i as usize].mv[1] as c_int,
            16 as c_int,
            8 as c_int,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src[1] = (*h).mc.get_ref.expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
            &mut *stride.as_mut_ptr().offset(1),
            (*(*a).l1.me16x8.as_mut_ptr().offset(i as isize))
                .p_fref
                .as_mut_ptr(),
            (*a).l1.me16x8[i as usize].i_stride[0] as intptr_t,
            (*a).l1.me16x8[i as usize].mv[0] as c_int,
            (*a).l1.me16x8[i as usize].mv[1] as c_int,
            16 as c_int,
            8 as c_int,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        (*h).mc.avg[PIXEL_16x8 as c_int as usize].expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
            src[0],
            stride[0],
            src[1],
            stride[1],
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me16x8[i as usize].i_ref as isize))
                [(*a).l1.me16x8[i as usize].i_ref as usize] as c_int,
        );
        i_part_cost_bi = (*h).pixf.mbcmp[PIXEL_16x8 as c_int as usize]
            .expect("non-null function pointer")(
            (*a).l0.me16x8[i as usize].p_fenc[0],
            FENC_STRIDE as intptr_t,
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            16 as intptr_t,
        ) + (*a).l0.me16x8[i as usize].cost_mv
            + (*a).l1.me16x8[i as usize].cost_mv
            + (*a).l0.me16x8[i as usize].i_ref_cost
            + (*a).l1.me16x8[i as usize].i_ref_cost;
        if (*h).mb.b_chroma_me != 0 {
            i_part_cost_bi += analyse_bi_chroma(h, a, i, PIXEL_16x8 as c_int);
        }
        i_part_cost = (*a).l0.me16x8[i as usize].cost;
        (*a).i_mb_partition16x8[i as usize] = D_L0_8x8 as c_int;
        if (*a).l1.me16x8[i as usize].cost < i_part_cost {
            i_part_cost = (*a).l1.me16x8[i as usize].cost;
            (*a).i_mb_partition16x8[i as usize] = D_L1_8x8 as c_int;
        }
        if (i_part_cost_bi + (*a).i_lambda * 1 as c_int) < i_part_cost {
            i_part_cost = i_part_cost_bi;
            (*a).i_mb_partition16x8[i as usize] = D_BI_8x8 as c_int;
        }
        (*a).i_cost16x8bi += i_part_cost;
        if (*a).b_early_terminate != 0
            && (i == 0
                && i_part_cost + (*a).i_cost_est16x8[1]
                    > i_best_satd
                        * (16 as c_int
                            + (((*a).i_mbrd != 0) as c_int + ((*h).mb.i_psy_rd != 0) as c_int))
                        / 16 as c_int)
        {
            (*a).i_cost16x8bi = COST_MAX;
            return;
        }
        mb_cache_mv_b16x8(h, a, i, 0 as c_int);
        i += 1;
    }
    (*a).i_mb_type16x8 = B_L0_L0 as c_int
        + ((*a).i_mb_partition16x8[0] >> 2 as c_int) * 3 as c_int
        + ((*a).i_mb_partition16x8[1] >> 2 as c_int);
    (*a).i_cost16x8bi +=
        (*a).i_lambda * i_mb_b16x8_cost_table[(*a).i_mb_type16x8 as usize] as c_int;
}
#[c2rust::src_loc = "2454:1"]
unsafe extern "C" fn mb_analyse_inter_b8x16(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_best_satd: c_int,
) {
    let mut pix: [[pixel; 128]; 2] = [[0; 128]; 2];
    let mut mvc: [[int16_t; 2]; 3] = [[0; 2]; 3];
    (*h).mb.i_partition = D_8x16 as c_int;
    (*a).i_cost8x16bi = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 2 as c_int {
        let mut i_part_cost: c_int = 0;
        let mut i_part_cost_bi: c_int = 0 as c_int;
        let mut stride: [intptr_t; 2] = [8 as c_int as intptr_t, 8 as c_int as intptr_t];
        let mut src: [*mut pixel; 2] = [0 as *mut pixel; 2];
        let mut m: x264_me_t = x264_me_t {
            i_pixel: 0,
            p_cost_mv: 0 as *mut uint16_t,
            i_ref_cost: 0,
            i_ref: 0,
            weight: 0 as *const x264_weight_t,
            p_fref: [0 as *mut pixel; 12],
            p_fref_w: 0 as *mut pixel,
            p_fenc: [0 as *mut pixel; 3],
            integral: 0 as *mut uint16_t,
            i_stride: [0; 3],
            mvp: [0; 2],
            cost_mv: 0,
            cost: 0,
            mv: [0; 2],
        };
        m.i_pixel = PIXEL_8x16 as c_int;
        m.p_cost_mv = (*a).p_cost_mv;
        m.i_stride[0] = (*h).mb.pic.i_stride[0];
        m.i_stride[1] = (*h).mb.pic.i_stride[1];
        m.i_stride[2] = (*h).mb.pic.i_stride[2];
        m.p_fenc[0] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(0))
            .offset((8 as c_int * i + 0 as c_int * FENC_STRIDE) as isize)
            as *mut pixel;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            m.p_fenc[1] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(1)).offset(
                ((8 as c_int * i >> (*h).mb.chroma_h_shift)
                    + (0 as c_int >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
            m.p_fenc[2] = &mut *(*(*h).mb.pic.p_fenc.as_mut_ptr().offset(2)).offset(
                ((8 as c_int * i >> (*h).mb.chroma_h_shift)
                    + (0 as c_int >> (*h).mb.chroma_v_shift) * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        }
        let mut l: c_int = 0 as c_int;
        while l < 2 as c_int {
            let mut lX: *mut x264_mb_analysis_list_t =
                if l != 0 { &mut (*a).l1 } else { &mut (*a).l0 };
            let mut ref8: [c_int; 2] = [
                (*lX).me8x8[i as usize].i_ref,
                (*lX).me8x8[(i + 2 as c_int) as usize].i_ref,
            ];
            let mut i_ref8s: c_int = if ref8[0] == ref8[1] {
                1 as c_int
            } else {
                2 as c_int
            };
            (*lX).me8x16[i as usize].cost = INT_MAX;
            let mut j: c_int = 0 as c_int;
            while j < i_ref8s {
                let mut i_ref: c_int = ref8[j as usize];
                m.i_ref_cost = *(*a).p_cost_ref[l as usize].offset(i_ref as isize) as c_int;
                m.p_fref[0] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0))
                .offset((8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize)
                    as *mut pixel;
                m.p_fref_w = m.p_fref[0];
                if (*h).param.analyse.i_subpel_refine != 0 {
                    m.p_fref[1] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize,
                    ) as *mut pixel;
                    m.p_fref[2] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize,
                    ) as *mut pixel;
                    m.p_fref[3] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(3))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize,
                    ) as *mut pixel;
                }
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                    m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(4))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1)) as isize,
                    ) as *mut pixel;
                    m.p_fref[8] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(8))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2)) as isize,
                    ) as *mut pixel;
                    if (*h).param.analyse.i_subpel_refine != 0 {
                        m.p_fref[5] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(5))
                            .offset(
                                (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[6] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(6))
                            .offset(
                                (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[7] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(7))
                            .offset(
                                (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(1))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[9] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(9 as c_int as isize))
                            .offset(
                                (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[10] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(10 as c_int as isize))
                            .offset(
                                (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                        m.p_fref[11] =
                            &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                                .as_mut_ptr()
                                .offset(i_ref as isize))
                            .as_mut_ptr()
                            .offset(11 as c_int as isize))
                            .offset(
                                (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(2))
                                    as isize,
                            ) as *mut pixel;
                    }
                } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                    m.p_fref[4] = &mut *(*(*(*(*h).mb.pic.p_fref.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(4))
                    .offset(
                        (8 as c_int * i
                            + (0 as c_int >> (*h).mb.chroma_v_shift)
                                * *m.i_stride.as_mut_ptr().offset(1))
                            as isize,
                    ) as *mut pixel;
                }
                if (*h).param.analyse.me_method.exhaustive_search() {
                    m.integral = &mut *(*(*(*h).mb.pic.p_integral.as_mut_ptr().offset(l as isize))
                        .as_mut_ptr()
                        .offset(i_ref as isize))
                    .offset(
                        (8 as c_int * i + 0 as c_int * *m.i_stride.as_mut_ptr().offset(0)) as isize,
                    ) as *mut uint16_t;
                }
                m.weight = x264_zero.as_mut_ptr() as *const x264_weight_t;
                m.i_ref = i_ref;
                (*((*mvc.as_mut_ptr().offset(0)).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*((*(*(*lX).mvc.as_mut_ptr().offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset(0))
                    .as_mut_ptr() as *mut x264_union32_t))
                        .i;
                (*((*mvc.as_mut_ptr().offset(1)).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*((*(*(*lX).mvc.as_mut_ptr().offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset((i + 1 as c_int) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                        .i;
                (*((*mvc.as_mut_ptr().offset(2)).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*((*(*(*lX).mvc.as_mut_ptr().offset(i_ref as isize))
                        .as_mut_ptr()
                        .offset((i + 3 as c_int) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                        .i;
                x264_macroblock_cache_ref(
                    h,
                    2 as c_int * i,
                    0 as c_int,
                    2 as c_int,
                    4 as c_int,
                    l,
                    i_ref as int8_t,
                );
                x264_10_mb_predict_mv(h, l, 4 as c_int * i, 2 as c_int, m.mvp.as_mut_ptr());
                x264_10_me_search_ref(
                    h,
                    &mut m,
                    mvc.as_mut_ptr() as *mut [int16_t; 2],
                    3 as c_int,
                    0 as *mut c_int,
                );
                m.cost += m.i_ref_cost;
                if m.cost < (*lX).me8x16[i as usize].cost {
                    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                        &mut *(*lX).me8x16.as_mut_ptr().offset(i as isize) as *mut x264_me_t
                            as *mut c_void,
                        &mut m as *mut x264_me_t as *const c_void,
                        size_of::<x264_me_t>() as size_t,
                    );
                }
                j += 1;
            }
            l += 1;
        }
        src[0] = (*h).mc.get_ref.expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            &mut *stride.as_mut_ptr().offset(0),
            (*(*a).l0.me8x16.as_mut_ptr().offset(i as isize))
                .p_fref
                .as_mut_ptr(),
            (*a).l0.me8x16[i as usize].i_stride[0] as intptr_t,
            (*a).l0.me8x16[i as usize].mv[0] as c_int,
            (*a).l0.me8x16[i as usize].mv[1] as c_int,
            8 as c_int,
            16 as c_int,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src[1] = (*h).mc.get_ref.expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(1)).as_mut_ptr(),
            &mut *stride.as_mut_ptr().offset(1),
            (*(*a).l1.me8x16.as_mut_ptr().offset(i as isize))
                .p_fref
                .as_mut_ptr(),
            (*a).l1.me8x16[i as usize].i_stride[0] as intptr_t,
            (*a).l1.me8x16[i as usize].mv[0] as c_int,
            (*a).l1.me8x16[i as usize].mv[1] as c_int,
            8 as c_int,
            16 as c_int,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        (*h).mc.avg[PIXEL_8x16 as c_int as usize].expect("non-null function pointer")(
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            8 as intptr_t,
            src[0],
            stride[0],
            src[1],
            stride[1],
            (*(*h)
                .mb
                .bipred_weight
                .offset((*a).l0.me8x16[i as usize].i_ref as isize))
                [(*a).l1.me8x16[i as usize].i_ref as usize] as c_int,
        );
        i_part_cost_bi = (*h).pixf.mbcmp[PIXEL_8x16 as c_int as usize]
            .expect("non-null function pointer")(
            (*a).l0.me8x16[i as usize].p_fenc[0],
            FENC_STRIDE as intptr_t,
            (*pix.as_mut_ptr().offset(0)).as_mut_ptr(),
            8 as intptr_t,
        ) + (*a).l0.me8x16[i as usize].cost_mv
            + (*a).l1.me8x16[i as usize].cost_mv
            + (*a).l0.me8x16[i as usize].i_ref_cost
            + (*a).l1.me8x16[i as usize].i_ref_cost;
        if (*h).mb.b_chroma_me != 0 {
            i_part_cost_bi += analyse_bi_chroma(h, a, i, PIXEL_8x16 as c_int);
        }
        i_part_cost = (*a).l0.me8x16[i as usize].cost;
        (*a).i_mb_partition8x16[i as usize] = D_L0_8x8 as c_int;
        if (*a).l1.me8x16[i as usize].cost < i_part_cost {
            i_part_cost = (*a).l1.me8x16[i as usize].cost;
            (*a).i_mb_partition8x16[i as usize] = D_L1_8x8 as c_int;
        }
        if (i_part_cost_bi + (*a).i_lambda * 1 as c_int) < i_part_cost {
            i_part_cost = i_part_cost_bi;
            (*a).i_mb_partition8x16[i as usize] = D_BI_8x8 as c_int;
        }
        (*a).i_cost8x16bi += i_part_cost;
        if (*a).b_early_terminate != 0
            && (i == 0
                && i_part_cost + (*a).i_cost_est8x16[1]
                    > i_best_satd
                        * (16 as c_int
                            + (((*a).i_mbrd != 0) as c_int + ((*h).mb.i_psy_rd != 0) as c_int))
                        / 16 as c_int)
        {
            (*a).i_cost8x16bi = COST_MAX;
            return;
        }
        mb_cache_mv_b8x16(h, a, i, 0 as c_int);
        i += 1;
    }
    (*a).i_mb_type8x16 = B_L0_L0 as c_int
        + ((*a).i_mb_partition8x16[0] >> 2 as c_int) * 3 as c_int
        + ((*a).i_mb_partition8x16[1] >> 2 as c_int);
    (*a).i_cost8x16bi +=
        (*a).i_lambda * i_mb_b16x8_cost_table[(*a).i_mb_type8x16 as usize] as c_int;
}
#[c2rust::src_loc = "2547:1"]
unsafe extern "C" fn mb_analyse_p_rd(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_satd: c_int,
) {
    let mut thresh: c_int = if (*a).b_early_terminate != 0 {
        i_satd * 5 as c_int / 4 as c_int + 1 as c_int
    } else {
        COST_MAX
    };
    (*h).mb.i_type = P_L0 as c_int;
    if (*a).l0.i_rd16x16 == COST_MAX
        && ((*a).b_early_terminate == 0 || (*a).l0.me16x16.cost <= i_satd * 3 as c_int / 2 as c_int)
    {
        (*h).mb.i_partition = D_16x16 as c_int;
        analyse_update_cache(h, a);
        (*a).l0.i_rd16x16 = rd_cost_mb(h, (*a).i_lambda2);
    }
    if (*a).l0.i_cost16x8 < thresh {
        (*h).mb.i_partition = D_16x8 as c_int;
        analyse_update_cache(h, a);
        (*a).l0.i_cost16x8 = rd_cost_mb(h, (*a).i_lambda2);
    } else {
        (*a).l0.i_cost16x8 = COST_MAX;
    }
    if (*a).l0.i_cost8x16 < thresh {
        (*h).mb.i_partition = D_8x16 as c_int;
        analyse_update_cache(h, a);
        (*a).l0.i_cost8x16 = rd_cost_mb(h, (*a).i_lambda2);
    } else {
        (*a).l0.i_cost8x16 = COST_MAX;
    }
    if (*a).l0.i_cost8x8 < thresh {
        (*h).mb.i_type = P_8x8 as c_int;
        (*h).mb.i_partition = D_8x8 as c_int;
        if (*h).param.analyse.inter & X264_ANALYSE_PSUB8x8 != 0 {
            x264_macroblock_cache_ref(
                h,
                0 as c_int,
                0 as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[0].i_ref as int8_t,
            );
            x264_macroblock_cache_ref(
                h,
                2 as c_int,
                0 as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[1].i_ref as int8_t,
            );
            x264_macroblock_cache_ref(
                h,
                0 as c_int,
                2 as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[2].i_ref as int8_t,
            );
            x264_macroblock_cache_ref(
                h,
                2 as c_int,
                2 as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[3].i_ref as int8_t,
            );
            let mut i: c_int = 0 as c_int;
            while i < 4 as c_int {
                let mut costs: [c_int; 4] = [
                    (*a).l0.i_cost4x4[i as usize],
                    (*a).l0.i_cost8x4[i as usize],
                    (*a).l0.i_cost4x8[i as usize],
                    (*a).l0.me8x8[i as usize].cost,
                ];
                let mut sub8x8_thresh: c_int = if (*a).b_early_terminate != 0 {
                    (if costs[0]
                        < (if costs[1]
                            < (if costs[2] < costs[3] {
                                costs[2]
                            } else {
                                costs[3]
                            })
                        {
                            costs[1]
                        } else {
                            if costs[2] < costs[3] {
                                costs[2]
                            } else {
                                costs[3]
                            }
                        })
                    {
                        costs[0]
                    } else {
                        if costs[1]
                            < (if costs[2] < costs[3] {
                                costs[2]
                            } else {
                                costs[3]
                            })
                        {
                            costs[1]
                        } else {
                            if costs[2] < costs[3] {
                                costs[2]
                            } else {
                                costs[3]
                            }
                        }
                    }) * 5 as c_int
                        / 4 as c_int
                } else {
                    COST_MAX
                };
                let mut subtype: c_int = 0;
                let mut btype: c_int = D_L0_8x8 as c_int;
                let mut bcost: uint64_t = COST_MAX64 as uint64_t;
                subtype = D_L0_4x4 as c_int;
                while subtype <= D_L0_8x8 as c_int {
                    let mut cost: uint64_t = 0;
                    if !(costs[subtype as usize] > sub8x8_thresh) {
                        (*h).mb.i_sub_partition[i as usize] = subtype as uint8_t;
                        mb_cache_mv_p8x8(h, a, i);
                        if !(subtype == btype) {
                            cost = x264_10_rd_cost_part(
                                h,
                                (*a).i_lambda2,
                                i << 2 as c_int,
                                PIXEL_8x8 as c_int,
                            );
                            if cost < bcost {
                                bcost = cost;
                                btype = subtype;
                            }
                        }
                    }
                    subtype += 1;
                }
                if (*h).mb.i_sub_partition[i as usize] as c_int != btype {
                    (*h).mb.i_sub_partition[i as usize] = btype as uint8_t;
                    mb_cache_mv_p8x8(h, a, i);
                }
                i += 1;
            }
        } else {
            analyse_update_cache(h, a);
        }
        (*a).l0.i_cost8x8 = rd_cost_mb(h, (*a).i_lambda2);
    } else {
        (*a).l0.i_cost8x8 = COST_MAX;
    };
}
#[c2rust::src_loc = "2622:1"]
unsafe extern "C" fn mb_analyse_b_rd(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_satd_inter: c_int,
) {
    let mut thresh: c_int = if (*a).b_early_terminate != 0 {
        i_satd_inter * (17 as c_int + ((*h).mb.i_psy_rd != 0) as c_int) / 16 as c_int + 1 as c_int
    } else {
        COST_MAX
    };
    if (*a).b_direct_available != 0 && (*a).i_rd16x16direct == COST_MAX {
        (*h).mb.i_type = B_DIRECT as c_int;
        (*h).mb.b_skip_mc = 1 as c_int;
        analyse_update_cache(h, a);
        (*a).i_rd16x16direct = rd_cost_mb(h, (*a).i_lambda2);
        (*h).mb.b_skip_mc = 0 as c_int;
    }
    (*h).mb.i_partition = D_16x16 as c_int;
    if (*a).l0.me16x16.cost < thresh && (*a).l0.i_rd16x16 == COST_MAX {
        (*h).mb.i_type = B_L0_L0 as c_int;
        analyse_update_cache(h, a);
        (*a).l0.i_rd16x16 = rd_cost_mb(h, (*a).i_lambda2);
    }
    if (*a).l1.me16x16.cost < thresh && (*a).l1.i_rd16x16 == COST_MAX {
        (*h).mb.i_type = B_L1_L1 as c_int;
        analyse_update_cache(h, a);
        (*a).l1.i_rd16x16 = rd_cost_mb(h, (*a).i_lambda2);
    }
    if (*a).i_cost16x16bi < thresh && (*a).i_rd16x16bi == COST_MAX {
        (*h).mb.i_type = B_BI_BI as c_int;
        analyse_update_cache(h, a);
        (*a).i_rd16x16bi = rd_cost_mb(h, (*a).i_lambda2);
    }
    if (*a).i_cost8x8bi < thresh && (*a).i_rd8x8bi == COST_MAX {
        (*h).mb.i_type = B_8x8 as c_int;
        (*h).mb.i_partition = D_8x8 as c_int;
        analyse_update_cache(h, a);
        (*a).i_rd8x8bi = rd_cost_mb(h, (*a).i_lambda2);
        x264_macroblock_cache_skip(
            h, 0 as c_int, 0 as c_int, 4 as c_int, 4 as c_int, 0 as c_int,
        );
    }
    if (*a).i_cost16x8bi < thresh && (*a).i_rd16x8bi == COST_MAX {
        (*h).mb.i_type = (*a).i_mb_type16x8;
        (*h).mb.i_partition = D_16x8 as c_int;
        analyse_update_cache(h, a);
        (*a).i_rd16x8bi = rd_cost_mb(h, (*a).i_lambda2);
    }
    if (*a).i_cost8x16bi < thresh && (*a).i_rd8x16bi == COST_MAX {
        (*h).mb.i_type = (*a).i_mb_type8x16;
        (*h).mb.i_partition = D_8x16 as c_int;
        analyse_update_cache(h, a);
        (*a).i_rd8x16bi = rd_cost_mb(h, (*a).i_lambda2);
    }
}
#[c2rust::src_loc = "2692:1"]
unsafe extern "C" fn refine_bidir(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    let mut i_biweight: c_int = 0;
    if (*h).mb.i_type == I_4x4 as c_int
        || (*h).mb.i_type == I_8x8 as c_int
        || (*h).mb.i_type == I_16x16 as c_int
        || (*h).mb.i_type == I_PCM as c_int
    {
        return;
    }
    match (*h).mb.i_partition {
        16 => {
            if (*h).mb.i_type == B_BI_BI as c_int {
                i_biweight = (*(*h).mb.bipred_weight.offset((*a).l0.bi16x16.i_ref as isize))
                    [(*a).l1.bi16x16.i_ref as usize] as c_int;
                x264_10_me_refine_bidir_satd(
                    h,
                    &mut (*a).l0.bi16x16,
                    &mut (*a).l1.bi16x16,
                    i_biweight,
                );
            }
        }
        14 => {
            let mut i: c_int = 0 as c_int;
            while i < 2 as c_int {
                if (*a).i_mb_partition16x8[i as usize] == D_BI_8x8 as c_int {
                    i_biweight = (*(*h)
                        .mb
                        .bipred_weight
                        .offset((*a).l0.me16x8[i as usize].i_ref as isize))
                        [(*a).l1.me16x8[i as usize].i_ref as usize]
                        as c_int;
                    x264_10_me_refine_bidir_satd(
                        h,
                        &mut *(*a).l0.me16x8.as_mut_ptr().offset(i as isize),
                        &mut *(*a).l1.me16x8.as_mut_ptr().offset(i as isize),
                        i_biweight,
                    );
                }
                i += 1;
            }
        }
        15 => {
            let mut i_0: c_int = 0 as c_int;
            while i_0 < 2 as c_int {
                if (*a).i_mb_partition8x16[i_0 as usize] == D_BI_8x8 as c_int {
                    i_biweight = (*(*h)
                        .mb
                        .bipred_weight
                        .offset((*a).l0.me8x16[i_0 as usize].i_ref as isize))
                        [(*a).l1.me8x16[i_0 as usize].i_ref as usize]
                        as c_int;
                    x264_10_me_refine_bidir_satd(
                        h,
                        &mut *(*a).l0.me8x16.as_mut_ptr().offset(i_0 as isize),
                        &mut *(*a).l1.me8x16.as_mut_ptr().offset(i_0 as isize),
                        i_biweight,
                    );
                }
                i_0 += 1;
            }
        }
        13 => {
            let mut i_1: c_int = 0 as c_int;
            while i_1 < 4 as c_int {
                if (*h).mb.i_sub_partition[i_1 as usize] as c_int == D_BI_8x8 as c_int {
                    i_biweight = (*(*h)
                        .mb
                        .bipred_weight
                        .offset((*a).l0.me8x8[i_1 as usize].i_ref as isize))
                        [(*a).l1.me8x8[i_1 as usize].i_ref as usize]
                        as c_int;
                    x264_10_me_refine_bidir_satd(
                        h,
                        &mut *(*a).l0.me8x8.as_mut_ptr().offset(i_1 as isize),
                        &mut *(*a).l1.me8x8.as_mut_ptr().offset(i_1 as isize),
                        i_biweight,
                    );
                }
                i_1 += 1;
            }
        }
        _ => {}
    };
}
#[inline]
#[c2rust::src_loc = "2735:1"]
unsafe extern "C" fn mb_analyse_transform(mut h: *mut x264_t) {
    if x264_mb_transform_8x8_allowed(h) != 0
        && (*h).param.analyse.b_transform_8x8 != 0
        && (*h).mb.b_lossless == 0
    {
        x264_10_mb_mc(h);
        let mut plane_count: c_int = if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
            == CHROMA_444 as c_int
            && (*h).mb.b_chroma_me != 0
        {
            3 as c_int
        } else {
            1 as c_int
        };
        let mut i_cost8: c_int = 0 as c_int;
        let mut i_cost4: c_int = 0 as c_int;
        if (*h).pixf.sa8d_satd[PIXEL_16x16 as c_int as usize].is_some() {
            let mut cost: uint64_t = 0 as uint64_t;
            let mut p: c_int = 0 as c_int;
            while p < plane_count {
                cost = cost.wrapping_add((*h).pixf.sa8d_satd[PIXEL_16x16 as c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[p as usize],
                    FENC_STRIDE as intptr_t,
                    (*h).mb.pic.p_fdec[p as usize],
                    FDEC_STRIDE as intptr_t,
                ));
                p += 1;
            }
            i_cost8 = cost as uint32_t as c_int;
            i_cost4 = (cost >> 32 as c_int) as uint32_t as c_int;
        } else {
            let mut p_0: c_int = 0 as c_int;
            while p_0 < plane_count {
                i_cost8 += (*h).pixf.sa8d[PIXEL_16x16 as c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[p_0 as usize],
                    FENC_STRIDE as intptr_t,
                    (*h).mb.pic.p_fdec[p_0 as usize],
                    FDEC_STRIDE as intptr_t,
                );
                i_cost4 += (*h).pixf.satd[PIXEL_16x16 as c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[p_0 as usize],
                    FENC_STRIDE as intptr_t,
                    (*h).mb.pic.p_fdec[p_0 as usize],
                    FDEC_STRIDE as intptr_t,
                );
                p_0 += 1;
            }
        }
        (*h).mb.b_transform_8x8 = (i_cost8 < i_cost4) as c_int;
        (*h).mb.b_skip_mc = 1 as c_int;
    }
}
#[inline]
#[c2rust::src_loc = "2773:1"]
unsafe extern "C" fn mb_analyse_transform_rd(
    mut h: *mut x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut i_satd: *mut c_int,
    mut i_rd: *mut c_int,
) {
    if (*h).param.analyse.b_transform_8x8 != 0 && (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0
    {
        let mut subpart_bak: uint32_t =
            (*((*h).mb.i_sub_partition.as_mut_ptr() as *mut x264_union32_t)).i;
        if (*h).mb.i_type == P_8x8 as c_int {
            (*((*h).mb.i_sub_partition.as_mut_ptr() as *mut x264_union32_t)).i =
                (D_L0_8x8 as c_int * 0x1010101 as c_int) as uint32_t;
        } else if x264_transform_allowed[(*h).mb.i_type as usize] == 0 {
            return;
        }
        analyse_update_cache(h, a);
        (*h).mb.b_transform_8x8 ^= 1 as c_int;
        let mut i_rd8: c_int = rd_cost_mb(h, (*a).i_lambda2);
        if *i_rd >= i_rd8 {
            if *i_rd > 0 as c_int {
                *i_satd = (*i_satd as int64_t * i_rd8 as int64_t / *i_rd as int64_t) as c_int;
            }
            *i_rd = i_rd8;
        } else {
            (*h).mb.b_transform_8x8 ^= 1 as c_int;
            (*((*h).mb.i_sub_partition.as_mut_ptr() as *mut x264_union32_t)).i = subpart_bak;
        }
    }
}
#[inline]
#[c2rust::src_loc = "2810:1"]
unsafe extern "C" fn mb_analyse_qp_rd(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    let mut bcost: c_int = 0;
    let mut cost: c_int = 0;
    let mut failures: c_int = 0;
    let mut prevcost: c_int = 0;
    let mut origcost: c_int = 0;
    let mut orig_qp: c_int = (*h).mb.i_qp;
    let mut bqp: c_int = (*h).mb.i_qp;
    let mut last_qp_tried: c_int = 0 as c_int;
    bcost = rd_cost_mb(h, (*a).i_lambda2);
    origcost = bcost;
    let mut origcbp: c_int = *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) as c_int;
    let mut direction: c_int = if origcbp != 0 { 1 as c_int } else { -1 };
    while direction >= -1 {
        let mut threshold: c_int = ((*h).mb.i_psy_rd != 0) as c_int;
        if (*h).mb.i_last_qp < orig_qp && direction == -1
            || (*h).mb.i_last_qp > orig_qp && direction == 1 as c_int
        {
            threshold += 1;
        }
        (*h).mb.i_qp = orig_qp;
        failures = 0 as c_int;
        prevcost = origcost;
        let mut already_checked_qp: c_int = -1;
        let mut already_checked_cost: c_int = COST_MAX;
        if direction == -1 {
            if origcbp == 0 {
                (*h).mb.i_qp = if (*h).mb.i_qp - threshold - 1 as c_int
                    > (if (*h).param.rc.i_qp_min
                        < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                    {
                        (*h).param.rc.i_qp_min
                    } else {
                        51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                    }) {
                    (*h).mb.i_qp - threshold - 1 as c_int
                } else if (*h).param.rc.i_qp_min
                    < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                {
                    (*h).param.rc.i_qp_min
                } else {
                    51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                };
                (*h).mb.i_chroma_qp = *(*h).chroma_qp_table.offset((*h).mb.i_qp as isize) as c_int;
                already_checked_cost = rd_cost_mb(h, (*a).i_lambda2);
                if *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) == 0 {
                    if (*h).mb.i_last_qp > (*h).mb.i_qp {
                        last_qp_tried = 1 as c_int;
                    }
                    break;
                } else {
                    already_checked_qp = (*h).mb.i_qp;
                    (*h).mb.i_qp = orig_qp;
                }
            }
        }
        (*h).mb.i_qp += direction;
        while (*h).mb.i_qp >= (*h).param.rc.i_qp_min
            && (*h).mb.i_qp
                <= (if (*h).param.rc.i_qp_max
                    < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                {
                    (*h).param.rc.i_qp_max
                } else {
                    51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                })
        {
            if (*h).mb.i_last_qp == (*h).mb.i_qp {
                last_qp_tried = 1 as c_int;
            }
            if (*h).mb.i_qp == already_checked_qp {
                cost = already_checked_cost;
            } else {
                (*h).mb.i_chroma_qp = *(*h).chroma_qp_table.offset((*h).mb.i_qp as isize) as c_int;
                cost = rd_cost_mb(h, (*a).i_lambda2);
                if cost < bcost {
                    bcost = cost;
                    bqp = (*h).mb.i_qp;
                }
            }
            if cost < prevcost {
                failures = 0 as c_int;
            } else {
                failures += 1;
            }
            prevcost = cost;
            if failures > threshold {
                break;
            }
            if direction == 1 as c_int && *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) == 0 {
                break;
            }
            (*h).mb.i_qp += direction;
        }
        direction -= 2 as c_int;
    }
    if last_qp_tried == 0 {
        (*h).mb.i_qp = (*h).mb.i_last_qp;
        (*h).mb.i_chroma_qp = *(*h).chroma_qp_table.offset((*h).mb.i_qp as isize) as c_int;
        cost = rd_cost_mb(h, (*a).i_lambda2);
        if cost < bcost {
            bcost = cost;
            bqp = (*h).mb.i_qp;
        }
    }
    (*h).mb.i_qp = bqp;
    (*h).mb.i_chroma_qp = *(*h).chroma_qp_table.offset((*h).mb.i_qp as isize) as c_int;
    if (*h).mb.i_qp != orig_qp
        && (*h).param.analyse.b_transform_8x8 != 0
        && x264_mb_transform_8x8_allowed(h) != 0
    {
        (*h).mb.b_transform_8x8 ^= 1 as c_int;
        cost = rd_cost_mb(h, (*a).i_lambda2);
        if cost > bcost {
            (*h).mb.b_transform_8x8 ^= 1 as c_int;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "2918:1"]
pub unsafe extern "C" fn x264_10_macroblock_analyse(mut h: *mut x264_t) {
    let mut current_block: u64;
    let mut analysis: x264_mb_analysis_t = x264_mb_analysis_t {
        i_lambda: 0,
        i_lambda2: 0,
        i_qp: 0,
        p_cost_mv: 0 as *mut uint16_t,
        p_cost_ref: [0 as *mut uint16_t; 2],
        i_mbrd: 0,
        b_fast_intra: 0,
        b_force_intra: 0,
        b_avoid_topright: 0,
        b_try_skip: 0,
        i_satd_i16x16: 0,
        i_satd_i16x16_dir: [0; 7],
        i_predict16x16: 0,
        i_satd_i8x8: 0,
        i_cbp_i8x8_luma: 0,
        i_satd_i8x8_dir: [[0; 16]; 4],
        i_predict8x8: [0; 4],
        i_satd_i4x4: 0,
        i_predict4x4: [0; 16],
        i_satd_pcm: 0,
        i_satd_chroma: 0,
        i_satd_chroma_dir: [0; 7],
        i_predict8x8chroma: 0,
        l0: x264_mb_analysis_list_t {
            me16x16: x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            },
            bi16x16: x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            },
            me8x8: [x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 4],
            me4x4: [[x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 4]; 4],
            me8x4: [[x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2]; 4],
            me4x8: [[x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2]; 4],
            me16x8: [x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2],
            me8x16: [x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2],
            i_rd16x16: 0,
            i_cost8x8: 0,
            i_cost4x4: [0; 4],
            i_cost8x4: [0; 4],
            i_cost4x8: [0; 4],
            i_cost16x8: 0,
            i_cost8x16: 0,
            mvc: [[[0; 2]; 6]; 32],
        },
        l1: x264_mb_analysis_list_t {
            me16x16: x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            },
            bi16x16: x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            },
            me8x8: [x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 4],
            me4x4: [[x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 4]; 4],
            me8x4: [[x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2]; 4],
            me4x8: [[x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2]; 4],
            me16x8: [x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2],
            me8x16: [x264_me_t {
                i_pixel: 0,
                p_cost_mv: 0 as *mut uint16_t,
                i_ref_cost: 0,
                i_ref: 0,
                weight: 0 as *const x264_weight_t,
                p_fref: [0 as *mut pixel; 12],
                p_fref_w: 0 as *mut pixel,
                p_fenc: [0 as *mut pixel; 3],
                integral: 0 as *mut uint16_t,
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2],
            i_rd16x16: 0,
            i_cost8x8: 0,
            i_cost4x4: [0; 4],
            i_cost8x4: [0; 4],
            i_cost4x8: [0; 4],
            i_cost16x8: 0,
            i_cost8x16: 0,
            mvc: [[[0; 2]; 6]; 32],
        },
        i_cost16x16bi: 0,
        i_cost16x16direct: 0,
        i_cost8x8bi: 0,
        i_cost8x8direct: [0; 4],
        i_satd8x8: [[0; 4]; 3],
        i_cost_est16x8: [0; 2],
        i_cost_est8x16: [0; 2],
        i_cost16x8bi: 0,
        i_cost8x16bi: 0,
        i_rd16x16bi: 0,
        i_rd16x16direct: 0,
        i_rd16x8bi: 0,
        i_rd8x16bi: 0,
        i_rd8x8bi: 0,
        i_mb_partition16x8: [0; 2],
        i_mb_partition8x16: [0; 2],
        i_mb_type16x8: 0,
        i_mb_type8x16: 0,
        b_direct_available: 0,
        b_early_terminate: 0,
    };
    let mut i_cost: c_int = COST_MAX;
    (*h).mb.i_qp = x264_10_ratecontrol_mb_qp(h);
    if (*h).param.rc.i_aq_mode != 0 && (*h).param.analyse.i_subpel_refine < 10 as c_int {
        (*h).mb.i_qp = if abs((*h).mb.i_qp - (*h).mb.i_last_qp) == 1 as c_int {
            (*h).mb.i_last_qp
        } else {
            (*h).mb.i_qp
        };
    }
    if (*h).param.analyse.b_mb_info != 0 {
        *(*(*h).fdec).effective_qp.offset((*h).mb.i_mb_xy as isize) = (*h).mb.i_qp as uint8_t;
    }
    mb_analyse_init(h, &mut analysis, (*h).mb.i_qp);
    if (*h).sh.i_type == SLICE_TYPE_I as c_int {
        current_block = 3870634877451421603;
    } else if (*h).sh.i_type == SLICE_TYPE_P as c_int {
        let mut b_skip: c_int = 0 as c_int;
        (*h).mc.prefetch_ref.expect("non-null function pointer")(
            (*h).mb.pic.p_fref[0][0][((*h).mb.i_mb_x & 3 as c_int) as usize],
            (*h).mb.pic.i_stride[0] as intptr_t,
            0 as c_int,
        );
        analysis.b_try_skip = 0 as c_int;
        if analysis.b_force_intra != 0 {
            if (*h).param.analyse.b_psy == 0 {
                mb_analyse_init_qp(
                    h,
                    &mut analysis,
                    if (*h).mb.i_qp - (*h).mb.ip_offset > (*h).param.rc.i_qp_min {
                        (*h).mb.i_qp - (*h).mb.ip_offset
                    } else {
                        (*h).param.rc.i_qp_min
                    },
                );
                current_block = 3870634877451421603;
            } else {
                current_block = 13460095289871124136;
            }
        } else {
            if !(*(*h).fdec).mb_info.is_null()
                && *(*(*h).fdec).mb_info.offset((*h).mb.i_mb_xy as isize) as c_uint
                    & X264_MBINFO_CONSTANT
                    != 0
            {
                if (*h).sh.b_mbaff == 0
                    && (*(*h).fdec).i_frame - (*(*h).fref[0][0]).i_frame == 1 as c_int
                    && (*h).sh.b_weighted_pred == 0
                    && *(*(*h).fref[0][0])
                        .effective_qp
                        .offset((*h).mb.i_mb_xy as isize) as c_int
                        <= (*h).mb.i_qp
                {
                    (*h).mb.i_partition = D_16x16 as c_int;
                    if (*((*h).mb.cache.pskip_mv.as_mut_ptr() as *mut x264_union32_t)).i == 0 {
                        b_skip = 1 as c_int;
                        (*h).mb.i_type = P_SKIP as c_int;
                    } else {
                        (*h).mb.i_type = P_L0 as c_int;
                        analysis.l0.me16x16.i_ref = 0 as c_int;
                        (*(analysis.l0.me16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i =
                            0 as uint32_t;
                    }
                    current_block = 10061345519188545595;
                } else {
                    if (*h).param.analyse.b_mb_info_update != 0 {
                        let ref mut fresh3 = *(*(*h).fdec).mb_info.offset((*h).mb.i_mb_xy as isize);
                        *fresh3 = (*fresh3 as c_uint & !X264_MBINFO_CONSTANT) as uint8_t;
                    }
                    current_block = 14072441030219150333;
                }
            } else {
                current_block = 14072441030219150333;
            }
            match current_block {
                10061345519188545595 => {}
                _ => {
                    let mut skip_invalid: c_int = ((*h).i_thread_frames > 1 as c_int
                        && (*h).mb.cache.pskip_mv[1] as c_int > (*h).mb.mv_max_spel[1])
                        as c_int;
                    if HAVE_INTERLACED != 0
                        && (*h).mb.b_interlaced == 0
                        && (*h).mb.i_mb_y * 16 as c_int >= (*h).param.height as c_int
                        && skip_invalid == 0
                    {
                        b_skip = 1 as c_int;
                    } else if (*h).param.analyse.b_fast_pskip != 0 {
                        if !(skip_invalid != 0) {
                            if (*h).param.analyse.i_subpel_refine >= 3 as c_int {
                                analysis.b_try_skip = 1 as c_int;
                            } else if (*h).mb.i_mb_type_left[0] == P_SKIP as c_int
                                || (*h).mb.i_mb_type_top == P_SKIP as c_int
                                || (*h).mb.i_mb_type_topleft == P_SKIP as c_int
                                || (*h).mb.i_mb_type_topright == P_SKIP as c_int
                            {
                                b_skip = x264_10_macroblock_probe_skip(h, 0 as c_int);
                            }
                        }
                    }
                    current_block = 13460095289871124136;
                }
            }
        }
        match current_block {
            3870634877451421603 => {}
            _ => {
                match current_block {
                    13460095289871124136 => {
                        (*h).mc.prefetch_ref.expect("non-null function pointer")(
                            (*h).mb.pic.p_fref[0][0][((*h).mb.i_mb_x & 3 as c_int) as usize],
                            (*h).mb.pic.i_stride[0] as intptr_t,
                            1 as c_int,
                        );
                        if b_skip != 0 {
                            (*h).mb.i_type = P_SKIP as c_int;
                            (*h).mb.i_partition = D_16x16 as c_int;
                            if (*h).mb.cache.pskip_mv[1] as c_int <= (*h).mb.mv_max_spel[1]
                                || (*h).i_thread_frames == 1 as c_int
                            {
                            } else {
                                __assert_fail(
                                    b"h->mb.cache.pskip_mv[1] <= h->mb.mv_max_spel[1] || h->i_thread_frames == 1\0"
                                        as *const u8 as *const c_char,
                                    b"encoder/analyse.c\0" as *const u8
                                        as *const c_char,
                                    3023 as c_uint,
                                    ::core::mem::transmute::<
                                        [u8; 42],
                                        [c_char; 42],
                                    >(*b"void x264_10_macroblock_analyse(x264_t *)\0")
                                        .as_ptr(),
                                );
                            }
                            current_block = 10061345519188545595;
                        } else {
                            let flags: c_uint = (*h).param.analyse.inter;
                            let mut i_type: c_int = 0;
                            let mut i_partition: c_int = 0;
                            let mut i_satd_inter: c_int = 0;
                            let mut i_satd_intra: c_int = 0;
                            mb_analyse_load_costs(h, &mut analysis);
                            mb_analyse_inter_p16x16(h, &mut analysis);
                            if (*h).mb.i_type == P_SKIP as c_int {
                                let mut i_0: c_int = 1 as c_int;
                                while i_0 < (*h).mb.pic.i_fref[0] {
                                    (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(0))
                                        .as_mut_ptr()
                                        .offset(i_0 as isize))
                                    .offset((*h).mb.i_mb_xy as isize))
                                    .as_mut_ptr()
                                        as *mut x264_union32_t))
                                        .i = 0 as uint32_t;
                                    i_0 += 1;
                                }
                                return;
                            }
                            if flags & X264_ANALYSE_PSUB16x16 != 0 {
                                if (*h).param.analyse.b_mixed_references != 0 {
                                    mb_analyse_inter_p8x8_mixed_ref(h, &mut analysis);
                                } else {
                                    mb_analyse_inter_p8x8(h, &mut analysis);
                                }
                            }
                            i_type = P_L0 as c_int;
                            i_partition = D_16x16 as c_int;
                            i_cost = analysis.l0.me16x16.cost;
                            if flags & X264_ANALYSE_PSUB16x16 != 0
                                && (analysis.b_early_terminate == 0
                                    || analysis.l0.i_cost8x8 < analysis.l0.me16x16.cost)
                            {
                                i_type = P_8x8 as c_int;
                                i_partition = D_8x8 as c_int;
                                i_cost = analysis.l0.i_cost8x8;
                                if flags & X264_ANALYSE_PSUB8x8 != 0 {
                                    let mut i_1: c_int = 0 as c_int;
                                    while i_1 < 4 as c_int {
                                        mb_analyse_inter_p4x4(h, &mut analysis, i_1);
                                        let mut i_thresh8x4: c_int =
                                            analysis.l0.me4x4[i_1 as usize][1].cost_mv
                                                + analysis.l0.me4x4[i_1 as usize][2].cost_mv;
                                        if analysis.b_early_terminate == 0
                                            || analysis.l0.i_cost4x4[i_1 as usize]
                                                < analysis.l0.me8x8[i_1 as usize].cost + i_thresh8x4
                                        {
                                            let mut i_cost8x8: c_int =
                                                analysis.l0.i_cost4x4[i_1 as usize];
                                            (*h).mb.i_sub_partition[i_1 as usize] =
                                                D_L0_4x4 as c_int as uint8_t;
                                            mb_analyse_inter_p8x4(h, &mut analysis, i_1);
                                            if analysis.l0.i_cost8x4[i_1 as usize] < i_cost8x8 {
                                                i_cost8x8 = analysis.l0.i_cost8x4[i_1 as usize];
                                                (*h).mb.i_sub_partition[i_1 as usize] =
                                                    D_L0_8x4 as c_int as uint8_t;
                                            }
                                            mb_analyse_inter_p4x8(h, &mut analysis, i_1);
                                            if analysis.l0.i_cost4x8[i_1 as usize] < i_cost8x8 {
                                                i_cost8x8 = analysis.l0.i_cost4x8[i_1 as usize];
                                                (*h).mb.i_sub_partition[i_1 as usize] =
                                                    D_L0_4x8 as c_int as uint8_t;
                                            }
                                            i_cost +=
                                                i_cost8x8 - analysis.l0.me8x8[i_1 as usize].cost;
                                        }
                                        mb_cache_mv_p8x8(h, &mut analysis, i_1);
                                        i_1 += 1;
                                    }
                                    analysis.l0.i_cost8x8 = i_cost;
                                }
                            }
                            let mut i_thresh16x8: c_int =
                                analysis.l0.me8x8[1].cost_mv + analysis.l0.me8x8[2].cost_mv;
                            if flags & X264_ANALYSE_PSUB16x16 != 0
                                && (analysis.b_early_terminate == 0
                                    || analysis.l0.i_cost8x8
                                        < analysis.l0.me16x16.cost + i_thresh16x8)
                            {
                                let mut i_avg_mv_ref_cost: c_int = analysis.l0.me8x8[2].cost_mv
                                    + analysis.l0.me8x8[2].i_ref_cost
                                    + analysis.l0.me8x8[3].cost_mv
                                    + analysis.l0.me8x8[3].i_ref_cost
                                    + 1 as c_int
                                    >> 1 as c_int;
                                analysis.i_cost_est16x8[1] = analysis.i_satd8x8[0][2]
                                    + analysis.i_satd8x8[0][3]
                                    + i_avg_mv_ref_cost;
                                mb_analyse_inter_p16x8(h, &mut analysis, i_cost);
                                if analysis.l0.i_cost16x8 < i_cost {
                                    i_cost = analysis.l0.i_cost16x8;
                                    i_type = P_L0 as c_int;
                                    i_partition = D_16x8 as c_int;
                                }
                                i_avg_mv_ref_cost = analysis.l0.me8x8[1].cost_mv
                                    + analysis.l0.me8x8[1].i_ref_cost
                                    + analysis.l0.me8x8[3].cost_mv
                                    + analysis.l0.me8x8[3].i_ref_cost
                                    + 1 as c_int
                                    >> 1 as c_int;
                                analysis.i_cost_est8x16[1] = analysis.i_satd8x8[0][1]
                                    + analysis.i_satd8x8[0][3]
                                    + i_avg_mv_ref_cost;
                                mb_analyse_inter_p8x16(h, &mut analysis, i_cost);
                                if analysis.l0.i_cost8x16 < i_cost {
                                    i_cost = analysis.l0.i_cost8x16;
                                    i_type = P_L0 as c_int;
                                    i_partition = D_8x16 as c_int;
                                }
                            }
                            (*h).mb.i_partition = i_partition;
                            if !(analysis.i_mbrd != 0 || (*h).mb.i_subpel_refine == 0) {
                                if i_partition == D_16x16 as c_int {
                                    x264_10_me_refine_qpel(h, &mut analysis.l0.me16x16);
                                    i_cost = analysis.l0.me16x16.cost;
                                } else if i_partition == D_16x8 as c_int {
                                    x264_10_me_refine_qpel(
                                        h,
                                        &mut *analysis.l0.me16x8.as_mut_ptr().offset(0),
                                    );
                                    x264_10_me_refine_qpel(
                                        h,
                                        &mut *analysis.l0.me16x8.as_mut_ptr().offset(1),
                                    );
                                    i_cost =
                                        analysis.l0.me16x8[0].cost + analysis.l0.me16x8[1].cost;
                                } else if i_partition == D_8x16 as c_int {
                                    x264_10_me_refine_qpel(
                                        h,
                                        &mut *analysis.l0.me8x16.as_mut_ptr().offset(0),
                                    );
                                    x264_10_me_refine_qpel(
                                        h,
                                        &mut *analysis.l0.me8x16.as_mut_ptr().offset(1),
                                    );
                                    i_cost =
                                        analysis.l0.me8x16[0].cost + analysis.l0.me8x16[1].cost;
                                } else if i_partition == D_8x8 as c_int {
                                    i_cost = 0 as c_int;
                                    let mut i8x8: c_int = 0 as c_int;
                                    while i8x8 < 4 as c_int {
                                        match (*h).mb.i_sub_partition[i8x8 as usize] as c_int {
                                            3 => {
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *analysis
                                                        .l0
                                                        .me8x8
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize),
                                                );
                                                i_cost += analysis.l0.me8x8[i8x8 as usize].cost;
                                            }
                                            1 => {
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me8x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize))
                                                    .as_mut_ptr()
                                                    .offset(0),
                                                );
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me8x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize))
                                                    .as_mut_ptr()
                                                    .offset(1),
                                                );
                                                i_cost += analysis.l0.me8x4[i8x8 as usize][0].cost
                                                    + analysis.l0.me8x4[i8x8 as usize][1].cost;
                                            }
                                            2 => {
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x8
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize))
                                                    .as_mut_ptr()
                                                    .offset(0),
                                                );
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x8
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize))
                                                    .as_mut_ptr()
                                                    .offset(1),
                                                );
                                                i_cost += analysis.l0.me4x8[i8x8 as usize][0].cost
                                                    + analysis.l0.me4x8[i8x8 as usize][1].cost;
                                            }
                                            0 => {
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize))
                                                    .as_mut_ptr()
                                                    .offset(0),
                                                );
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize))
                                                    .as_mut_ptr()
                                                    .offset(1),
                                                );
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize))
                                                    .as_mut_ptr()
                                                    .offset(2),
                                                );
                                                x264_10_me_refine_qpel(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8 as isize))
                                                    .as_mut_ptr()
                                                    .offset(3),
                                                );
                                                i_cost += analysis.l0.me4x4[i8x8 as usize][0].cost
                                                    + analysis.l0.me4x4[i8x8 as usize][1].cost
                                                    + analysis.l0.me4x4[i8x8 as usize][2].cost
                                                    + analysis.l0.me4x4[i8x8 as usize][3].cost;
                                            }
                                            _ => {
                                                x264_10_log(
                                                    h,
                                                    X264_LOG_ERROR,
                                                    b"internal error (!8x8 && !4x4)\n\0"
                                                        as *const u8
                                                        as *const c_char,
                                                );
                                            }
                                        }
                                        i8x8 += 1;
                                    }
                                }
                            }
                            if (*h).mb.b_chroma_me != 0 {
                                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                    == CHROMA_444 as c_int
                                {
                                    mb_analyse_intra(h, &mut analysis, i_cost);
                                    mb_analyse_intra_chroma(h, &mut analysis);
                                } else {
                                    mb_analyse_intra_chroma(h, &mut analysis);
                                    mb_analyse_intra(
                                        h,
                                        &mut analysis,
                                        i_cost - analysis.i_satd_chroma,
                                    );
                                }
                                analysis.i_satd_i16x16 += analysis.i_satd_chroma;
                                analysis.i_satd_i8x8 += analysis.i_satd_chroma;
                                analysis.i_satd_i4x4 += analysis.i_satd_chroma;
                            } else {
                                mb_analyse_intra(h, &mut analysis, i_cost);
                            }
                            i_satd_inter = i_cost;
                            i_satd_intra = if analysis.i_satd_i16x16
                                < (if analysis.i_satd_i8x8 < analysis.i_satd_i4x4 {
                                    analysis.i_satd_i8x8
                                } else {
                                    analysis.i_satd_i4x4
                                }) {
                                analysis.i_satd_i16x16
                            } else if analysis.i_satd_i8x8 < analysis.i_satd_i4x4 {
                                analysis.i_satd_i8x8
                            } else {
                                analysis.i_satd_i4x4
                            };
                            if analysis.i_mbrd != 0 {
                                mb_analyse_p_rd(
                                    h,
                                    &mut analysis,
                                    if i_satd_inter < i_satd_intra {
                                        i_satd_inter
                                    } else {
                                        i_satd_intra
                                    },
                                );
                                i_type = P_L0 as c_int;
                                i_partition = D_16x16 as c_int;
                                i_cost = analysis.l0.i_rd16x16;
                                if analysis.l0.i_cost16x8 < i_cost {
                                    i_cost = analysis.l0.i_cost16x8;
                                    i_partition = D_16x8 as c_int;
                                }
                                if analysis.l0.i_cost8x16 < i_cost {
                                    i_cost = analysis.l0.i_cost8x16;
                                    i_partition = D_8x16 as c_int;
                                }
                                if analysis.l0.i_cost8x8 < i_cost {
                                    i_cost = analysis.l0.i_cost8x8;
                                    i_partition = D_8x8 as c_int;
                                    i_type = P_8x8 as c_int;
                                }
                                (*h).mb.i_type = i_type;
                                (*h).mb.i_partition = i_partition;
                                if i_cost < COST_MAX {
                                    mb_analyse_transform_rd(
                                        h,
                                        &mut analysis,
                                        &mut i_satd_inter,
                                        &mut i_cost,
                                    );
                                }
                                intra_rd(
                                    h,
                                    &mut analysis,
                                    i_satd_inter * 5 as c_int / 4 as c_int + 1 as c_int,
                                );
                            }
                            if analysis.i_satd_i16x16 < i_cost {
                                i_cost = analysis.i_satd_i16x16;
                                i_type = I_16x16 as c_int;
                            }
                            if analysis.i_satd_i8x8 < i_cost {
                                i_cost = analysis.i_satd_i8x8;
                                i_type = I_8x8 as c_int;
                            }
                            if analysis.i_satd_i4x4 < i_cost {
                                i_cost = analysis.i_satd_i4x4;
                                i_type = I_4x4 as c_int;
                            }
                            if analysis.i_satd_pcm < i_cost {
                                i_cost = analysis.i_satd_pcm;
                                i_type = I_PCM as c_int;
                            }
                            (*h).mb.i_type = i_type;
                            if analysis.b_force_intra != 0
                                && !(i_type == I_4x4 as c_int
                                    || i_type == I_8x8 as c_int
                                    || i_type == I_16x16 as c_int
                                    || i_type == I_PCM as c_int)
                            {
                                analyse_update_cache(h, &mut analysis);
                                x264_10_macroblock_encode(h);
                                let mut p: c_int = 0 as c_int;
                                while p
                                    < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                        == CHROMA_444 as c_int
                                    {
                                        3 as c_int
                                    } else {
                                        1 as c_int
                                    })
                                {
                                    (*h).mc.copy[PIXEL_16x16 as c_int as usize]
                                        .expect("non-null function pointer")(
                                        (*h).mb.pic.p_fenc[p as usize],
                                        FENC_STRIDE as intptr_t,
                                        (*h).mb.pic.p_fdec[p as usize],
                                        FDEC_STRIDE as intptr_t,
                                        16 as c_int,
                                    );
                                    p += 1;
                                }
                                if !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                    == CHROMA_444 as c_int)
                                {
                                    let mut height: c_int = 16 as c_int >> (*h).mb.chroma_v_shift;
                                    (*h).mc.copy[PIXEL_8x8 as c_int as usize]
                                        .expect("non-null function pointer")(
                                        (*h).mb.pic.p_fenc[1],
                                        FENC_STRIDE as intptr_t,
                                        (*h).mb.pic.p_fdec[1],
                                        FDEC_STRIDE as intptr_t,
                                        height,
                                    );
                                    (*h).mc.copy[PIXEL_8x8 as c_int as usize]
                                        .expect("non-null function pointer")(
                                        (*h).mb.pic.p_fenc[2],
                                        FENC_STRIDE as intptr_t,
                                        (*h).mb.pic.p_fdec[2],
                                        FDEC_STRIDE as intptr_t,
                                        height,
                                    );
                                }
                                mb_analyse_init_qp(
                                    h,
                                    &mut analysis,
                                    if (*h).mb.i_qp - (*h).mb.ip_offset > (*h).param.rc.i_qp_min {
                                        (*h).mb.i_qp - (*h).mb.ip_offset
                                    } else {
                                        (*h).param.rc.i_qp_min
                                    },
                                );
                                current_block = 3870634877451421603;
                            } else {
                                if analysis.i_mbrd >= 2 as c_int && (*h).mb.i_type != I_PCM as c_int
                                {
                                    if (*h).mb.i_type == I_4x4 as c_int
                                        || (*h).mb.i_type == I_8x8 as c_int
                                        || (*h).mb.i_type == I_16x16 as c_int
                                        || (*h).mb.i_type == I_PCM as c_int
                                    {
                                        intra_rd_refine(h, &mut analysis);
                                    } else if i_partition == D_16x16 as c_int {
                                        x264_macroblock_cache_ref(
                                            h,
                                            0 as c_int,
                                            0 as c_int,
                                            4 as c_int,
                                            4 as c_int,
                                            0 as c_int,
                                            analysis.l0.me16x16.i_ref as int8_t,
                                        );
                                        analysis.l0.me16x16.cost = i_cost;
                                        x264_10_me_refine_qpel_rd(
                                            h,
                                            &mut analysis.l0.me16x16,
                                            analysis.i_lambda2,
                                            0 as c_int,
                                            0 as c_int,
                                        );
                                    } else if i_partition == D_16x8 as c_int {
                                        (*((*h).mb.i_sub_partition.as_mut_ptr()
                                            as *mut x264_union32_t))
                                            .i =
                                            (D_L0_8x8 as c_int * 0x1010101 as c_int) as uint32_t;
                                        x264_macroblock_cache_ref(
                                            h,
                                            0 as c_int,
                                            0 as c_int,
                                            4 as c_int,
                                            2 as c_int,
                                            0 as c_int,
                                            analysis.l0.me16x8[0].i_ref as int8_t,
                                        );
                                        x264_macroblock_cache_ref(
                                            h,
                                            0 as c_int,
                                            2 as c_int,
                                            4 as c_int,
                                            2 as c_int,
                                            0 as c_int,
                                            analysis.l0.me16x8[1].i_ref as int8_t,
                                        );
                                        x264_10_me_refine_qpel_rd(
                                            h,
                                            &mut *analysis.l0.me16x8.as_mut_ptr().offset(0),
                                            analysis.i_lambda2,
                                            0 as c_int,
                                            0 as c_int,
                                        );
                                        x264_10_me_refine_qpel_rd(
                                            h,
                                            &mut *analysis.l0.me16x8.as_mut_ptr().offset(1),
                                            analysis.i_lambda2,
                                            8 as c_int,
                                            0 as c_int,
                                        );
                                    } else if i_partition == D_8x16 as c_int {
                                        (*((*h).mb.i_sub_partition.as_mut_ptr()
                                            as *mut x264_union32_t))
                                            .i =
                                            (D_L0_8x8 as c_int * 0x1010101 as c_int) as uint32_t;
                                        x264_macroblock_cache_ref(
                                            h,
                                            0 as c_int,
                                            0 as c_int,
                                            2 as c_int,
                                            4 as c_int,
                                            0 as c_int,
                                            analysis.l0.me8x16[0].i_ref as int8_t,
                                        );
                                        x264_macroblock_cache_ref(
                                            h,
                                            2 as c_int,
                                            0 as c_int,
                                            2 as c_int,
                                            4 as c_int,
                                            0 as c_int,
                                            analysis.l0.me8x16[1].i_ref as int8_t,
                                        );
                                        x264_10_me_refine_qpel_rd(
                                            h,
                                            &mut *analysis.l0.me8x16.as_mut_ptr().offset(0),
                                            analysis.i_lambda2,
                                            0 as c_int,
                                            0 as c_int,
                                        );
                                        x264_10_me_refine_qpel_rd(
                                            h,
                                            &mut *analysis.l0.me8x16.as_mut_ptr().offset(1),
                                            analysis.i_lambda2,
                                            4 as c_int,
                                            0 as c_int,
                                        );
                                    } else if i_partition == D_8x8 as c_int {
                                        analyse_update_cache(h, &mut analysis);
                                        let mut i8x8_0: c_int = 0 as c_int;
                                        while i8x8_0 < 4 as c_int {
                                            if (*h).mb.i_sub_partition[i8x8_0 as usize] as c_int
                                                == D_L0_8x8 as c_int
                                            {
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *analysis
                                                        .l0
                                                        .me8x8
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int,
                                                    0 as c_int,
                                                );
                                            } else if (*h).mb.i_sub_partition[i8x8_0 as usize]
                                                as c_int
                                                == D_L0_8x4 as c_int
                                            {
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me8x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset(0),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int + 0 as c_int,
                                                    0 as c_int,
                                                );
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me8x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset(1),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int + 2 as c_int,
                                                    0 as c_int,
                                                );
                                            } else if (*h).mb.i_sub_partition[i8x8_0 as usize]
                                                as c_int
                                                == D_L0_4x8 as c_int
                                            {
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x8
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset(0),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int + 0 as c_int,
                                                    0 as c_int,
                                                );
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x8
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset(1),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int + 1 as c_int,
                                                    0 as c_int,
                                                );
                                            } else if (*h).mb.i_sub_partition[i8x8_0 as usize]
                                                as c_int
                                                == D_L0_4x4 as c_int
                                            {
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset(0),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int + 0 as c_int,
                                                    0 as c_int,
                                                );
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset(1),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int + 1 as c_int,
                                                    0 as c_int,
                                                );
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset(2),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int + 2 as c_int,
                                                    0 as c_int,
                                                );
                                                x264_10_me_refine_qpel_rd(
                                                    h,
                                                    &mut *(*analysis
                                                        .l0
                                                        .me4x4
                                                        .as_mut_ptr()
                                                        .offset(i8x8_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset(3),
                                                    analysis.i_lambda2,
                                                    i8x8_0 * 4 as c_int + 3 as c_int,
                                                    0 as c_int,
                                                );
                                            }
                                            i8x8_0 += 1;
                                        }
                                    }
                                }
                                current_block = 17193116482801528934;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    17193116482801528934 => {}
                    3870634877451421603 => {}
                    _ => {
                        let mut i: c_int = 0 as c_int;
                        while i < (*h).mb.pic.i_fref[0] {
                            (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(0))
                                .as_mut_ptr()
                                .offset(i as isize))
                            .offset((*h).mb.i_mb_xy as isize))
                            .as_mut_ptr() as *mut x264_union32_t))
                                .i = 0 as uint32_t;
                            i += 1;
                        }
                        current_block = 17193116482801528934;
                    }
                }
            }
        }
    } else {
        if (*h).sh.i_type == SLICE_TYPE_B as c_int {
            let mut i_bskip_cost: c_int = COST_MAX;
            let mut b_skip_0: c_int = 0 as c_int;
            if analysis.i_mbrd != 0 {
                mb_init_fenc_cache(h, (analysis.i_mbrd >= 2 as c_int) as c_int);
            }
            (*h).mb.i_type = B_SKIP as c_int;
            if (*h).mb.b_direct_auto_write != 0 {
                let mut i_2: c_int = 0 as c_int;
                while i_2 < 2 as c_int {
                    let mut b_changed: c_int = 1 as c_int;
                    (*h).sh.b_direct_spatial_mv_pred ^= 1 as c_int;
                    analysis.b_direct_available = x264_10_mb_predict_mv_direct16x16(
                        h,
                        if i_2 != 0 && analysis.b_direct_available != 0 {
                            &mut b_changed
                        } else {
                            0 as *mut c_int
                        },
                    );
                    if analysis.b_direct_available != 0 {
                        if b_changed != 0 {
                            x264_10_mb_mc(h);
                            b_skip_0 = x264_10_macroblock_probe_skip(h, 1 as c_int);
                        }
                        (*h).stat.frame.i_direct_score
                            [(*h).sh.b_direct_spatial_mv_pred as usize] += b_skip_0;
                    } else {
                        b_skip_0 = 0 as c_int;
                    }
                    i_2 += 1;
                }
            } else {
                analysis.b_direct_available = x264_10_mb_predict_mv_direct16x16(h, 0 as *mut c_int);
            }
            analysis.b_try_skip = 0 as c_int;
            if analysis.b_direct_available != 0 {
                if (*h).mb.b_direct_auto_write == 0 {
                    x264_10_mb_mc(h);
                }
                if HAVE_INTERLACED != 0
                    && (*h).mb.b_interlaced == 0
                    && (*h).mb.i_mb_y * 16 as c_int >= (*h).param.height as c_int
                {
                    b_skip_0 = 1 as c_int;
                } else if analysis.i_mbrd != 0 {
                    i_bskip_cost = ssd_mb(h);
                    (*h).mb.b_skip_mc = (i_bskip_cost
                        <= 6 as c_int * analysis.i_lambda2 + 128 as c_int >> 8 as c_int)
                        as c_int;
                    b_skip_0 = (*h).mb.b_skip_mc;
                } else if (*h).mb.b_direct_auto_write == 0 {
                    analysis.b_try_skip = x264_10_macroblock_probe_skip(h, 1 as c_int);
                    if (*h).param.analyse.i_subpel_refine < 3 as c_int {
                        b_skip_0 = analysis.b_try_skip;
                    }
                }
                if b_skip_0 != 0 {
                    let mut i_3: c_int = 0 as c_int;
                    while i_3 < (*h).mb.pic.i_fref[0] {
                        (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(0))
                            .as_mut_ptr()
                            .offset(i_3 as isize))
                        .offset((*h).mb.i_mb_xy as isize))
                        .as_mut_ptr() as *mut x264_union32_t))
                            .i = 0 as uint32_t;
                        i_3 += 1;
                    }
                    let mut i_4: c_int = 0 as c_int;
                    while i_4 < (*h).mb.pic.i_fref[1] {
                        (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(1))
                            .as_mut_ptr()
                            .offset(i_4 as isize))
                        .offset((*h).mb.i_mb_xy as isize))
                        .as_mut_ptr() as *mut x264_union32_t))
                            .i = 0 as uint32_t;
                        i_4 += 1;
                    }
                }
            }
            if b_skip_0 == 0 {
                let flags_0: c_uint = (*h).param.analyse.inter;
                let mut i_type_0: c_int = 0;
                let mut i_partition_0: c_int = 0;
                let mut i_satd_inter_0: c_int = 0;
                (*h).mb.b_skip_mc = 0 as c_int;
                (*h).mb.i_type = B_DIRECT as c_int;
                mb_analyse_load_costs(h, &mut analysis);
                if analysis.b_direct_available != 0 {
                    mb_analyse_inter_direct(h, &mut analysis);
                }
                mb_analyse_inter_b16x16(h, &mut analysis);
                if (*h).mb.i_type == B_SKIP as c_int {
                    let mut i_5: c_int = 1 as c_int;
                    while i_5 < (*h).mb.pic.i_fref[0] {
                        (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(0))
                            .as_mut_ptr()
                            .offset(i_5 as isize))
                        .offset((*h).mb.i_mb_xy as isize))
                        .as_mut_ptr() as *mut x264_union32_t))
                            .i = 0 as uint32_t;
                        i_5 += 1;
                    }
                    let mut i_6: c_int = 1 as c_int;
                    while i_6 < (*h).mb.pic.i_fref[1] {
                        (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(1))
                            .as_mut_ptr()
                            .offset(i_6 as isize))
                        .offset((*h).mb.i_mb_xy as isize))
                        .as_mut_ptr() as *mut x264_union32_t))
                            .i = 0 as uint32_t;
                        i_6 += 1;
                    }
                    return;
                }
                i_type_0 = B_L0_L0 as c_int;
                i_partition_0 = D_16x16 as c_int;
                i_cost = analysis.l0.me16x16.cost;
                if analysis.l1.me16x16.cost < i_cost {
                    i_cost = analysis.l1.me16x16.cost;
                    i_type_0 = B_L1_L1 as c_int;
                }
                if analysis.i_cost16x16bi < i_cost {
                    i_cost = analysis.i_cost16x16bi;
                    i_type_0 = B_BI_BI as c_int;
                }
                if analysis.i_cost16x16direct < i_cost {
                    i_cost = analysis.i_cost16x16direct;
                    i_type_0 = B_DIRECT as c_int;
                }
                if analysis.i_mbrd != 0
                    && analysis.b_early_terminate != 0
                    && analysis.i_cost16x16direct <= i_cost * 33 as c_int / 32 as c_int
                {
                    mb_analyse_b_rd(h, &mut analysis, i_cost);
                    if i_bskip_cost < analysis.i_rd16x16direct
                        && i_bskip_cost < analysis.i_rd16x16bi
                        && i_bskip_cost < analysis.l0.i_rd16x16
                        && i_bskip_cost < analysis.l1.i_rd16x16
                    {
                        (*h).mb.i_type = B_SKIP as c_int;
                        analyse_update_cache(h, &mut analysis);
                        return;
                    }
                }
                if flags_0 & X264_ANALYSE_BSUB16x16 != 0 {
                    if (*h).param.analyse.b_mixed_references != 0 {
                        mb_analyse_inter_b8x8_mixed_ref(h, &mut analysis);
                    } else {
                        mb_analyse_inter_b8x8(h, &mut analysis);
                    }
                    if analysis.i_cost8x8bi < i_cost {
                        i_cost = analysis.i_cost8x8bi;
                        i_type_0 = B_8x8 as c_int;
                        i_partition_0 = D_8x8 as c_int;
                    }
                    let mut i_cost_est16x8bi_total: c_int = 0 as c_int;
                    let mut i_cost_est8x16bi_total: c_int = 0 as c_int;
                    let mut i_mb_type: c_int = 0;
                    let mut i_partition16x8: [c_int; 2] = [0; 2];
                    let mut i_partition8x16: [c_int; 2] = [0; 2];
                    let mut i_7: c_int = 0 as c_int;
                    while i_7 < 2 as c_int {
                        let mut avg_l0_mv_ref_cost: c_int = 0;
                        let mut avg_l1_mv_ref_cost: c_int = 0;
                        let mut i_l0_satd: c_int = 0;
                        let mut i_l1_satd: c_int = 0;
                        let mut i_bi_satd: c_int = 0;
                        let mut i_best_cost: c_int = 0;
                        i_best_cost = COST_MAX;
                        i_l0_satd = analysis.i_satd8x8[0][(i_7 * 2 as c_int) as usize]
                            + analysis.i_satd8x8[0][(i_7 * 2 as c_int + 1 as c_int) as usize];
                        i_l1_satd = analysis.i_satd8x8[1][(i_7 * 2 as c_int) as usize]
                            + analysis.i_satd8x8[1][(i_7 * 2 as c_int + 1 as c_int) as usize];
                        i_bi_satd = analysis.i_satd8x8[2][(i_7 * 2 as c_int) as usize]
                            + analysis.i_satd8x8[2][(i_7 * 2 as c_int + 1 as c_int) as usize];
                        avg_l0_mv_ref_cost = analysis.l0.me8x8[(i_7 * 2 as c_int) as usize].cost_mv
                            + analysis.l0.me8x8[(i_7 * 2 as c_int) as usize].i_ref_cost
                            + analysis.l0.me8x8[(i_7 * 2 as c_int + 1 as c_int) as usize].cost_mv
                            + analysis.l0.me8x8[(i_7 * 2 as c_int + 1 as c_int) as usize]
                                .i_ref_cost
                            + 1 as c_int
                            >> 1 as c_int;
                        avg_l1_mv_ref_cost = analysis.l1.me8x8[(i_7 * 2 as c_int) as usize].cost_mv
                            + analysis.l1.me8x8[(i_7 * 2 as c_int) as usize].i_ref_cost
                            + analysis.l1.me8x8[(i_7 * 2 as c_int + 1 as c_int) as usize].cost_mv
                            + analysis.l1.me8x8[(i_7 * 2 as c_int + 1 as c_int) as usize]
                                .i_ref_cost
                            + 1 as c_int
                            >> 1 as c_int;
                        if i_l0_satd + avg_l0_mv_ref_cost < i_best_cost {
                            i_best_cost = i_l0_satd + avg_l0_mv_ref_cost;
                            i_partition16x8[i_7 as usize] = D_L0_8x8 as c_int;
                        }
                        if i_l1_satd + avg_l1_mv_ref_cost < i_best_cost {
                            i_best_cost = i_l1_satd + avg_l1_mv_ref_cost;
                            i_partition16x8[i_7 as usize] = D_L1_8x8 as c_int;
                        }
                        if i_bi_satd + avg_l0_mv_ref_cost + avg_l1_mv_ref_cost < i_best_cost {
                            i_best_cost = i_bi_satd + avg_l0_mv_ref_cost + avg_l1_mv_ref_cost;
                            i_partition16x8[i_7 as usize] = D_BI_8x8 as c_int;
                        }
                        analysis.i_cost_est16x8[i_7 as usize] = i_best_cost;
                        i_best_cost = COST_MAX;
                        i_l0_satd = analysis.i_satd8x8[0][i_7 as usize]
                            + analysis.i_satd8x8[0][(i_7 + 2 as c_int) as usize];
                        i_l1_satd = analysis.i_satd8x8[1][i_7 as usize]
                            + analysis.i_satd8x8[1][(i_7 + 2 as c_int) as usize];
                        i_bi_satd = analysis.i_satd8x8[2][i_7 as usize]
                            + analysis.i_satd8x8[2][(i_7 + 2 as c_int) as usize];
                        avg_l0_mv_ref_cost = analysis.l0.me8x8[i_7 as usize].cost_mv
                            + analysis.l0.me8x8[i_7 as usize].i_ref_cost
                            + analysis.l0.me8x8[(i_7 + 2 as c_int) as usize].cost_mv
                            + analysis.l0.me8x8[(i_7 + 2 as c_int) as usize].i_ref_cost
                            + 1 as c_int
                            >> 1 as c_int;
                        avg_l1_mv_ref_cost = analysis.l1.me8x8[i_7 as usize].cost_mv
                            + analysis.l1.me8x8[i_7 as usize].i_ref_cost
                            + analysis.l1.me8x8[(i_7 + 2 as c_int) as usize].cost_mv
                            + analysis.l1.me8x8[(i_7 + 2 as c_int) as usize].i_ref_cost
                            + 1 as c_int
                            >> 1 as c_int;
                        if i_l0_satd + avg_l0_mv_ref_cost < i_best_cost {
                            i_best_cost = i_l0_satd + avg_l0_mv_ref_cost;
                            i_partition8x16[i_7 as usize] = D_L0_8x8 as c_int;
                        }
                        if i_l1_satd + avg_l1_mv_ref_cost < i_best_cost {
                            i_best_cost = i_l1_satd + avg_l1_mv_ref_cost;
                            i_partition8x16[i_7 as usize] = D_L1_8x8 as c_int;
                        }
                        if i_bi_satd + avg_l0_mv_ref_cost + avg_l1_mv_ref_cost < i_best_cost {
                            i_best_cost = i_bi_satd + avg_l0_mv_ref_cost + avg_l1_mv_ref_cost;
                            i_partition8x16[i_7 as usize] = D_BI_8x8 as c_int;
                        }
                        analysis.i_cost_est8x16[i_7 as usize] = i_best_cost;
                        i_7 += 1;
                    }
                    i_mb_type = B_L0_L0 as c_int
                        + (i_partition16x8[0] >> 2 as c_int) * 3 as c_int
                        + (i_partition16x8[1] >> 2 as c_int);
                    analysis.i_cost_est16x8[1] +=
                        analysis.i_lambda * i_mb_b16x8_cost_table[i_mb_type as usize] as c_int;
                    i_cost_est16x8bi_total =
                        analysis.i_cost_est16x8[0] + analysis.i_cost_est16x8[1];
                    i_mb_type = B_L0_L0 as c_int
                        + (i_partition8x16[0] >> 2 as c_int) * 3 as c_int
                        + (i_partition8x16[1] >> 2 as c_int);
                    analysis.i_cost_est8x16[1] +=
                        analysis.i_lambda * i_mb_b16x8_cost_table[i_mb_type as usize] as c_int;
                    i_cost_est8x16bi_total =
                        analysis.i_cost_est8x16[0] + analysis.i_cost_est8x16[1];
                    let mut try_16x8_first: c_int =
                        (i_cost_est16x8bi_total < i_cost_est8x16bi_total) as c_int;
                    if try_16x8_first != 0
                        && (analysis.b_early_terminate == 0 || i_cost_est16x8bi_total < i_cost)
                    {
                        mb_analyse_inter_b16x8(h, &mut analysis, i_cost);
                        if analysis.i_cost16x8bi < i_cost {
                            i_cost = analysis.i_cost16x8bi;
                            i_type_0 = analysis.i_mb_type16x8;
                            i_partition_0 = D_16x8 as c_int;
                        }
                    }
                    if analysis.b_early_terminate == 0 || i_cost_est8x16bi_total < i_cost {
                        mb_analyse_inter_b8x16(h, &mut analysis, i_cost);
                        if analysis.i_cost8x16bi < i_cost {
                            i_cost = analysis.i_cost8x16bi;
                            i_type_0 = analysis.i_mb_type8x16;
                            i_partition_0 = D_8x16 as c_int;
                        }
                    }
                    if try_16x8_first == 0
                        && (analysis.b_early_terminate == 0 || i_cost_est16x8bi_total < i_cost)
                    {
                        mb_analyse_inter_b16x8(h, &mut analysis, i_cost);
                        if analysis.i_cost16x8bi < i_cost {
                            i_cost = analysis.i_cost16x8bi;
                            i_type_0 = analysis.i_mb_type16x8;
                            i_partition_0 = D_16x8 as c_int;
                        }
                    }
                }
                if !(analysis.i_mbrd != 0 || (*h).mb.i_subpel_refine == 0) {
                    if i_partition_0 == D_16x16 as c_int {
                        analysis.l0.me16x16.cost -= analysis.i_lambda
                            * i_mb_b_cost_table[B_L0_L0 as c_int as usize] as c_int;
                        analysis.l1.me16x16.cost -= analysis.i_lambda
                            * i_mb_b_cost_table[B_L1_L1 as c_int as usize] as c_int;
                        if i_type_0 == B_L0_L0 as c_int {
                            x264_10_me_refine_qpel(h, &mut analysis.l0.me16x16);
                            i_cost = analysis.l0.me16x16.cost
                                + analysis.i_lambda
                                    * i_mb_b_cost_table[B_L0_L0 as c_int as usize] as c_int;
                        } else if i_type_0 == B_L1_L1 as c_int {
                            x264_10_me_refine_qpel(h, &mut analysis.l1.me16x16);
                            i_cost = analysis.l1.me16x16.cost
                                + analysis.i_lambda
                                    * i_mb_b_cost_table[B_L1_L1 as c_int as usize] as c_int;
                        } else if i_type_0 == B_BI_BI as c_int {
                            x264_10_me_refine_qpel(h, &mut analysis.l0.bi16x16);
                            x264_10_me_refine_qpel(h, &mut analysis.l1.bi16x16);
                        }
                    } else if i_partition_0 == D_16x8 as c_int {
                        let mut i_8: c_int = 0 as c_int;
                        while i_8 < 2 as c_int {
                            if analysis.i_mb_partition16x8[i_8 as usize] != D_L1_8x8 as c_int {
                                x264_10_me_refine_qpel(
                                    h,
                                    &mut *analysis.l0.me16x8.as_mut_ptr().offset(i_8 as isize),
                                );
                            }
                            if analysis.i_mb_partition16x8[i_8 as usize] != D_L0_8x8 as c_int {
                                x264_10_me_refine_qpel(
                                    h,
                                    &mut *analysis.l1.me16x8.as_mut_ptr().offset(i_8 as isize),
                                );
                            }
                            i_8 += 1;
                        }
                    } else if i_partition_0 == D_8x16 as c_int {
                        let mut i_9: c_int = 0 as c_int;
                        while i_9 < 2 as c_int {
                            if analysis.i_mb_partition8x16[i_9 as usize] != D_L1_8x8 as c_int {
                                x264_10_me_refine_qpel(
                                    h,
                                    &mut *analysis.l0.me8x16.as_mut_ptr().offset(i_9 as isize),
                                );
                            }
                            if analysis.i_mb_partition8x16[i_9 as usize] != D_L0_8x8 as c_int {
                                x264_10_me_refine_qpel(
                                    h,
                                    &mut *analysis.l1.me8x16.as_mut_ptr().offset(i_9 as isize),
                                );
                            }
                            i_9 += 1;
                        }
                    } else if i_partition_0 == D_8x8 as c_int {
                        let mut i_10: c_int = 0 as c_int;
                        while i_10 < 4 as c_int {
                            let mut m: *mut x264_me_t = 0 as *mut x264_me_t;
                            let mut i_part_cost_old: c_int = 0;
                            let mut i_type_cost: c_int = 0;
                            let mut i_part_type: c_int =
                                (*h).mb.i_sub_partition[i_10 as usize] as c_int;
                            let mut b_bidir: c_int = (i_part_type == D_BI_8x8 as c_int) as c_int;
                            if !(i_part_type == D_DIRECT_8x8 as c_int) {
                                if x264_mb_partition_listX_table[0][i_part_type as usize] != 0 {
                                    m = &mut *analysis.l0.me8x8.as_mut_ptr().offset(i_10 as isize)
                                        as *mut x264_me_t;
                                    i_part_cost_old = (*m).cost;
                                    i_type_cost = analysis.i_lambda
                                        * i_sub_mb_b_cost_table[D_L0_8x8 as c_int as usize]
                                            as c_int;
                                    (*m).cost -= i_type_cost;
                                    x264_10_me_refine_qpel(h, m);
                                    if b_bidir == 0 {
                                        analysis.i_cost8x8bi +=
                                            (*m).cost + i_type_cost - i_part_cost_old;
                                    }
                                }
                                if x264_mb_partition_listX_table[1][i_part_type as usize] != 0 {
                                    m = &mut *analysis.l1.me8x8.as_mut_ptr().offset(i_10 as isize)
                                        as *mut x264_me_t;
                                    i_part_cost_old = (*m).cost;
                                    i_type_cost = analysis.i_lambda
                                        * i_sub_mb_b_cost_table[D_L1_8x8 as c_int as usize]
                                            as c_int;
                                    (*m).cost -= i_type_cost;
                                    x264_10_me_refine_qpel(h, m);
                                    if b_bidir == 0 {
                                        analysis.i_cost8x8bi +=
                                            (*m).cost + i_type_cost - i_part_cost_old;
                                    }
                                }
                            }
                            i_10 += 1;
                        }
                    }
                }
                i_satd_inter_0 = i_cost;
                if analysis.i_mbrd != 0 {
                    mb_analyse_b_rd(h, &mut analysis, i_satd_inter_0);
                    i_type_0 = B_SKIP as c_int;
                    i_cost = i_bskip_cost;
                    i_partition_0 = D_16x16 as c_int;
                    if analysis.l0.i_rd16x16 < i_cost {
                        i_cost = analysis.l0.i_rd16x16;
                        i_type_0 = B_L0_L0 as c_int;
                    }
                    if analysis.l1.i_rd16x16 < i_cost {
                        i_cost = analysis.l1.i_rd16x16;
                        i_type_0 = B_L1_L1 as c_int;
                    }
                    if analysis.i_rd16x16bi < i_cost {
                        i_cost = analysis.i_rd16x16bi;
                        i_type_0 = B_BI_BI as c_int;
                    }
                    if analysis.i_rd16x16direct < i_cost {
                        i_cost = analysis.i_rd16x16direct;
                        i_type_0 = B_DIRECT as c_int;
                    }
                    if analysis.i_rd16x8bi < i_cost {
                        i_cost = analysis.i_rd16x8bi;
                        i_type_0 = analysis.i_mb_type16x8;
                        i_partition_0 = D_16x8 as c_int;
                    }
                    if analysis.i_rd8x16bi < i_cost {
                        i_cost = analysis.i_rd8x16bi;
                        i_type_0 = analysis.i_mb_type8x16;
                        i_partition_0 = D_8x16 as c_int;
                    }
                    if analysis.i_rd8x8bi < i_cost {
                        i_cost = analysis.i_rd8x8bi;
                        i_type_0 = B_8x8 as c_int;
                        i_partition_0 = D_8x8 as c_int;
                    }
                    (*h).mb.i_type = i_type_0;
                    (*h).mb.i_partition = i_partition_0;
                }
                if (*h).mb.b_chroma_me != 0 {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                        mb_analyse_intra(h, &mut analysis, i_satd_inter_0);
                        mb_analyse_intra_chroma(h, &mut analysis);
                    } else {
                        mb_analyse_intra_chroma(h, &mut analysis);
                        mb_analyse_intra(h, &mut analysis, i_satd_inter_0 - analysis.i_satd_chroma);
                    }
                    analysis.i_satd_i16x16 += analysis.i_satd_chroma;
                    analysis.i_satd_i8x8 += analysis.i_satd_chroma;
                    analysis.i_satd_i4x4 += analysis.i_satd_chroma;
                } else {
                    mb_analyse_intra(h, &mut analysis, i_satd_inter_0);
                }
                if analysis.i_mbrd != 0 {
                    mb_analyse_transform_rd(h, &mut analysis, &mut i_satd_inter_0, &mut i_cost);
                    intra_rd(
                        h,
                        &mut analysis,
                        i_satd_inter_0 * 17 as c_int / 16 as c_int + 1 as c_int,
                    );
                }
                if analysis.i_satd_i16x16 < i_cost {
                    i_cost = analysis.i_satd_i16x16;
                    i_type_0 = I_16x16 as c_int;
                }
                if analysis.i_satd_i8x8 < i_cost {
                    i_cost = analysis.i_satd_i8x8;
                    i_type_0 = I_8x8 as c_int;
                }
                if analysis.i_satd_i4x4 < i_cost {
                    i_cost = analysis.i_satd_i4x4;
                    i_type_0 = I_4x4 as c_int;
                }
                if analysis.i_satd_pcm < i_cost {
                    i_cost = analysis.i_satd_pcm;
                    i_type_0 = I_PCM as c_int;
                }
                (*h).mb.i_type = i_type_0;
                (*h).mb.i_partition = i_partition_0;
                if analysis.i_mbrd >= 2 as c_int
                    && (i_type_0 == I_4x4 as c_int
                        || i_type_0 == I_8x8 as c_int
                        || i_type_0 == I_16x16 as c_int
                        || i_type_0 == I_PCM as c_int)
                    && i_type_0 != I_PCM as c_int
                {
                    intra_rd_refine(h, &mut analysis);
                }
                if (*h).mb.i_subpel_refine >= 5 as c_int {
                    refine_bidir(h, &mut analysis);
                }
                if analysis.i_mbrd >= 2 as c_int
                    && i_type_0 > B_DIRECT as c_int
                    && i_type_0 < B_SKIP as c_int
                {
                    let mut i_biweight: c_int = 0;
                    analyse_update_cache(h, &mut analysis);
                    if i_partition_0 == D_16x16 as c_int {
                        if i_type_0 == B_L0_L0 as c_int {
                            analysis.l0.me16x16.cost = i_cost;
                            x264_10_me_refine_qpel_rd(
                                h,
                                &mut analysis.l0.me16x16,
                                analysis.i_lambda2,
                                0 as c_int,
                                0 as c_int,
                            );
                        } else if i_type_0 == B_L1_L1 as c_int {
                            analysis.l1.me16x16.cost = i_cost;
                            x264_10_me_refine_qpel_rd(
                                h,
                                &mut analysis.l1.me16x16,
                                analysis.i_lambda2,
                                0 as c_int,
                                1 as c_int,
                            );
                        } else if i_type_0 == B_BI_BI as c_int {
                            i_biweight = (*(*h)
                                .mb
                                .bipred_weight
                                .offset(analysis.l0.bi16x16.i_ref as isize))
                                [analysis.l1.bi16x16.i_ref as usize]
                                as c_int;
                            x264_10_me_refine_bidir_rd(
                                h,
                                &mut analysis.l0.bi16x16,
                                &mut analysis.l1.bi16x16,
                                i_biweight,
                                0 as c_int,
                                analysis.i_lambda2,
                            );
                        }
                    } else if i_partition_0 == D_16x8 as c_int {
                        let mut i_11: c_int = 0 as c_int;
                        while i_11 < 2 as c_int {
                            (*h).mb.i_sub_partition[(i_11 * 2 as c_int + 1 as c_int) as usize] =
                                analysis.i_mb_partition16x8[i_11 as usize] as uint8_t;
                            (*h).mb.i_sub_partition[(i_11 * 2 as c_int) as usize] =
                                (*h).mb.i_sub_partition[(i_11 * 2 as c_int + 1 as c_int) as usize];
                            if analysis.i_mb_partition16x8[i_11 as usize] == D_L0_8x8 as c_int {
                                x264_10_me_refine_qpel_rd(
                                    h,
                                    &mut *analysis.l0.me16x8.as_mut_ptr().offset(i_11 as isize),
                                    analysis.i_lambda2,
                                    i_11 * 8 as c_int,
                                    0 as c_int,
                                );
                            } else if analysis.i_mb_partition16x8[i_11 as usize]
                                == D_L1_8x8 as c_int
                            {
                                x264_10_me_refine_qpel_rd(
                                    h,
                                    &mut *analysis.l1.me16x8.as_mut_ptr().offset(i_11 as isize),
                                    analysis.i_lambda2,
                                    i_11 * 8 as c_int,
                                    1 as c_int,
                                );
                            } else if analysis.i_mb_partition16x8[i_11 as usize]
                                == D_BI_8x8 as c_int
                            {
                                i_biweight = (*(*h)
                                    .mb
                                    .bipred_weight
                                    .offset(analysis.l0.me16x8[i_11 as usize].i_ref as isize))
                                    [analysis.l1.me16x8[i_11 as usize].i_ref as usize]
                                    as c_int;
                                x264_10_me_refine_bidir_rd(
                                    h,
                                    &mut *analysis.l0.me16x8.as_mut_ptr().offset(i_11 as isize),
                                    &mut *analysis.l1.me16x8.as_mut_ptr().offset(i_11 as isize),
                                    i_biweight,
                                    i_11 * 2 as c_int,
                                    analysis.i_lambda2,
                                );
                            }
                            i_11 += 1;
                        }
                    } else if i_partition_0 == D_8x16 as c_int {
                        let mut i_12: c_int = 0 as c_int;
                        while i_12 < 2 as c_int {
                            (*h).mb.i_sub_partition[(i_12 + 2 as c_int) as usize] =
                                analysis.i_mb_partition8x16[i_12 as usize] as uint8_t;
                            (*h).mb.i_sub_partition[i_12 as usize] =
                                (*h).mb.i_sub_partition[(i_12 + 2 as c_int) as usize];
                            if analysis.i_mb_partition8x16[i_12 as usize] == D_L0_8x8 as c_int {
                                x264_10_me_refine_qpel_rd(
                                    h,
                                    &mut *analysis.l0.me8x16.as_mut_ptr().offset(i_12 as isize),
                                    analysis.i_lambda2,
                                    i_12 * 4 as c_int,
                                    0 as c_int,
                                );
                            } else if analysis.i_mb_partition8x16[i_12 as usize]
                                == D_L1_8x8 as c_int
                            {
                                x264_10_me_refine_qpel_rd(
                                    h,
                                    &mut *analysis.l1.me8x16.as_mut_ptr().offset(i_12 as isize),
                                    analysis.i_lambda2,
                                    i_12 * 4 as c_int,
                                    1 as c_int,
                                );
                            } else if analysis.i_mb_partition8x16[i_12 as usize]
                                == D_BI_8x8 as c_int
                            {
                                i_biweight = (*(*h)
                                    .mb
                                    .bipred_weight
                                    .offset(analysis.l0.me8x16[i_12 as usize].i_ref as isize))
                                    [analysis.l1.me8x16[i_12 as usize].i_ref as usize]
                                    as c_int;
                                x264_10_me_refine_bidir_rd(
                                    h,
                                    &mut *analysis.l0.me8x16.as_mut_ptr().offset(i_12 as isize),
                                    &mut *analysis.l1.me8x16.as_mut_ptr().offset(i_12 as isize),
                                    i_biweight,
                                    i_12,
                                    analysis.i_lambda2,
                                );
                            }
                            i_12 += 1;
                        }
                    } else if i_partition_0 == D_8x8 as c_int {
                        let mut i_13: c_int = 0 as c_int;
                        while i_13 < 4 as c_int {
                            if (*h).mb.i_sub_partition[i_13 as usize] as c_int == D_L0_8x8 as c_int
                            {
                                x264_10_me_refine_qpel_rd(
                                    h,
                                    &mut *analysis.l0.me8x8.as_mut_ptr().offset(i_13 as isize),
                                    analysis.i_lambda2,
                                    i_13 * 4 as c_int,
                                    0 as c_int,
                                );
                            } else if (*h).mb.i_sub_partition[i_13 as usize] as c_int
                                == D_L1_8x8 as c_int
                            {
                                x264_10_me_refine_qpel_rd(
                                    h,
                                    &mut *analysis.l1.me8x8.as_mut_ptr().offset(i_13 as isize),
                                    analysis.i_lambda2,
                                    i_13 * 4 as c_int,
                                    1 as c_int,
                                );
                            } else if (*h).mb.i_sub_partition[i_13 as usize] as c_int
                                == D_BI_8x8 as c_int
                            {
                                i_biweight = (*(*h)
                                    .mb
                                    .bipred_weight
                                    .offset(analysis.l0.me8x8[i_13 as usize].i_ref as isize))
                                    [analysis.l1.me8x8[i_13 as usize].i_ref as usize]
                                    as c_int;
                                x264_10_me_refine_bidir_rd(
                                    h,
                                    &mut *analysis.l0.me8x8.as_mut_ptr().offset(i_13 as isize),
                                    &mut *analysis.l1.me8x8.as_mut_ptr().offset(i_13 as isize),
                                    i_biweight,
                                    i_13,
                                    analysis.i_lambda2,
                                );
                            }
                            i_13 += 1;
                        }
                    }
                }
            }
        }
        current_block = 17193116482801528934;
    }
    match current_block {
        3870634877451421603 => {
            if analysis.i_mbrd != 0 {
                mb_init_fenc_cache(h, (analysis.i_mbrd >= 2 as c_int) as c_int);
            }
            mb_analyse_intra(h, &mut analysis, COST_MAX);
            if analysis.i_mbrd != 0 {
                intra_rd(h, &mut analysis, COST_MAX);
            }
            i_cost = analysis.i_satd_i16x16;
            (*h).mb.i_type = I_16x16 as c_int;
            if analysis.i_satd_i4x4 < i_cost {
                i_cost = analysis.i_satd_i4x4;
                (*h).mb.i_type = I_4x4 as c_int;
            }
            if analysis.i_satd_i8x8 < i_cost {
                i_cost = analysis.i_satd_i8x8;
                (*h).mb.i_type = I_8x8 as c_int;
            }
            if analysis.i_satd_pcm < i_cost {
                (*h).mb.i_type = I_PCM as c_int;
            } else if analysis.i_mbrd >= 2 as c_int {
                intra_rd_refine(h, &mut analysis);
            }
        }
        _ => {}
    }
    analyse_update_cache(h, &mut analysis);
    if analysis.i_mbrd >= 2 as c_int {
        static mut check_mv_lists: [uint8_t; 19] = [
            0,
            0,
            0,
            0,
            1 as c_int as uint8_t,
            0,
            0,
            0,
            1 as c_int as uint8_t,
            0,
            0,
            0,
            2 as c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut list: c_int = check_mv_lists[(*h).mb.i_type as usize] as c_int - 1 as c_int;
        if list >= 0 as c_int
            && (*h).mb.i_partition != D_16x16 as c_int
            && (*(&mut *(*(*h).mb.cache.mv.as_mut_ptr().offset(list as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0) as isize)
                as *mut [int16_t; 2] as *mut x264_union32_t))
                .i
                == (*(&mut *(*(*h).mb.cache.mv.as_mut_ptr().offset(list as isize))
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(12 as c_int as isize) as isize)
                    as *mut [int16_t; 2] as *mut x264_union32_t))
                    .i
            && (*h).mb.cache.ref_0[list as usize][x264_scan8[0] as usize] as c_int
                == (*h).mb.cache.ref_0[list as usize][x264_scan8[12] as usize] as c_int
        {
            (*h).mb.i_partition = D_16x16 as c_int;
        }
    }
    if analysis.i_mbrd == 0 {
        mb_analyse_transform(h);
    }
    if analysis.i_mbrd == 3 as c_int
        && !((*h).mb.i_type == P_SKIP as c_int || (*h).mb.i_type == B_SKIP as c_int)
    {
        mb_analyse_qp_rd(h, &mut analysis);
    }
    (*h).mb.b_trellis = (*h).param.analyse.i_trellis;
    (*h).mb.b_noise_reduction = ((*h).mb.b_noise_reduction != 0
        || (*h).param.analyse.i_noise_reduction != 0
            && !((*h).mb.i_type == I_4x4 as c_int
                || (*h).mb.i_type == I_8x8 as c_int
                || (*h).mb.i_type == I_16x16 as c_int
                || (*h).mb.i_type == I_PCM as c_int)) as c_int;
    if !((*h).mb.i_type == P_SKIP as c_int || (*h).mb.i_type == B_SKIP as c_int)
        && (*h).mb.i_psy_trellis != 0
        && (*h).param.analyse.i_trellis == 1 as c_int
    {
        psy_trellis_init(h, 0 as c_int);
    }
    if (*h).mb.b_trellis == 1 as c_int || (*h).mb.b_noise_reduction != 0 {
        (*h).mb.i_skip_intra = 0 as c_int;
    }
}
#[c2rust::src_loc = "3734:1"]
unsafe extern "C" fn analyse_update_cache(mut h: *mut x264_t, mut a: *mut x264_mb_analysis_t) {
    match (*h).mb.i_type {
        0 => {
            let mut i: c_int = 0 as c_int;
            while i < 16 as c_int {
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[i as usize] as usize] =
                    (*a).i_predict4x4[i as usize] as int8_t;
                i += 1;
            }
            mb_analyse_intra_chroma(h, a);
        }
        1 => {
            let mut i_0: c_int = 0 as c_int;
            while i_0 < 4 as c_int {
                x264_macroblock_cache_intra8x8_pred(
                    h,
                    2 as c_int * (i_0 & 1 as c_int),
                    2 as c_int * (i_0 >> 1 as c_int),
                    (*a).i_predict8x8[i_0 as usize],
                );
                i_0 += 1;
            }
            mb_analyse_intra_chroma(h, a);
        }
        2 => {
            (*h).mb.i_intra16x16_pred_mode = (*a).i_predict16x16;
            mb_analyse_intra_chroma(h, a);
        }
        3 => {}
        4 => match (*h).mb.i_partition {
            16 => {
                x264_macroblock_cache_ref(
                    h,
                    0 as c_int,
                    0 as c_int,
                    4 as c_int,
                    4 as c_int,
                    0 as c_int,
                    (*a).l0.me16x16.i_ref as int8_t,
                );
                x264_macroblock_cache_mv(
                    h,
                    0 as c_int,
                    0 as c_int,
                    4 as c_int,
                    4 as c_int,
                    0 as c_int,
                    (*((*a).l0.me16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i,
                );
            }
            14 => {
                x264_macroblock_cache_ref(
                    h,
                    0 as c_int,
                    0 as c_int,
                    4 as c_int,
                    2 as c_int,
                    0 as c_int,
                    (*a).l0.me16x8[0].i_ref as int8_t,
                );
                x264_macroblock_cache_ref(
                    h,
                    0 as c_int,
                    2 as c_int,
                    4 as c_int,
                    2 as c_int,
                    0 as c_int,
                    (*a).l0.me16x8[1].i_ref as int8_t,
                );
                x264_macroblock_cache_mv(
                    h,
                    0 as c_int,
                    0 as c_int,
                    4 as c_int,
                    2 as c_int,
                    0 as c_int,
                    (*((*(*a).l0.me16x8.as_mut_ptr().offset(0)).mv.as_mut_ptr()
                        as *mut x264_union32_t))
                        .i,
                );
                x264_macroblock_cache_mv(
                    h,
                    0 as c_int,
                    2 as c_int,
                    4 as c_int,
                    2 as c_int,
                    0 as c_int,
                    (*((*(*a).l0.me16x8.as_mut_ptr().offset(1)).mv.as_mut_ptr()
                        as *mut x264_union32_t))
                        .i,
                );
            }
            15 => {
                x264_macroblock_cache_ref(
                    h,
                    0 as c_int,
                    0 as c_int,
                    2 as c_int,
                    4 as c_int,
                    0 as c_int,
                    (*a).l0.me8x16[0].i_ref as int8_t,
                );
                x264_macroblock_cache_ref(
                    h,
                    2 as c_int,
                    0 as c_int,
                    2 as c_int,
                    4 as c_int,
                    0 as c_int,
                    (*a).l0.me8x16[1].i_ref as int8_t,
                );
                x264_macroblock_cache_mv(
                    h,
                    0 as c_int,
                    0 as c_int,
                    2 as c_int,
                    4 as c_int,
                    0 as c_int,
                    (*((*(*a).l0.me8x16.as_mut_ptr().offset(0)).mv.as_mut_ptr()
                        as *mut x264_union32_t))
                        .i,
                );
                x264_macroblock_cache_mv(
                    h,
                    2 as c_int,
                    0 as c_int,
                    2 as c_int,
                    4 as c_int,
                    0 as c_int,
                    (*((*(*a).l0.me8x16.as_mut_ptr().offset(1)).mv.as_mut_ptr()
                        as *mut x264_union32_t))
                        .i,
                );
            }
            _ => {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"internal error P_L0 and partition=%d\n\0" as *const u8 as *const c_char,
                    (*h).mb.i_partition,
                );
            }
        },
        5 => {
            x264_macroblock_cache_ref(
                h,
                0 as c_int,
                0 as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[0].i_ref as int8_t,
            );
            x264_macroblock_cache_ref(
                h,
                2 as c_int,
                0 as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[1].i_ref as int8_t,
            );
            x264_macroblock_cache_ref(
                h,
                0 as c_int,
                2 as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[2].i_ref as int8_t,
            );
            x264_macroblock_cache_ref(
                h,
                2 as c_int,
                2 as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                (*a).l0.me8x8[3].i_ref as int8_t,
            );
            let mut i_1: c_int = 0 as c_int;
            while i_1 < 4 as c_int {
                mb_cache_mv_p8x8(h, a, i_1);
                i_1 += 1;
            }
        }
        6 => {
            (*h).mb.i_partition = D_16x16 as c_int;
            x264_macroblock_cache_ref(
                h,
                0 as c_int,
                0 as c_int,
                4 as c_int,
                4 as c_int,
                0 as c_int,
                0 as int8_t,
            );
            x264_macroblock_cache_mv(
                h,
                0 as c_int,
                0 as c_int,
                4 as c_int,
                4 as c_int,
                0 as c_int,
                (*((*h).mb.cache.pskip_mv.as_mut_ptr() as *mut x264_union32_t)).i,
            );
        }
        18 | 7 => {
            (*h).mb.i_partition = (*h).mb.cache.direct_partition;
            mb_load_mv_direct8x8(h, 0 as c_int);
            mb_load_mv_direct8x8(h, 1 as c_int);
            mb_load_mv_direct8x8(h, 2 as c_int);
            mb_load_mv_direct8x8(h, 3 as c_int);
        }
        17 => {
            let mut i_2: c_int = 0 as c_int;
            while i_2 < 4 as c_int {
                mb_cache_mv_b8x8(h, a, i_2, 1 as c_int);
                i_2 += 1;
            }
        }
        _ => match (*h).mb.i_partition {
            16 => match (*h).mb.i_type {
                8 => {
                    x264_macroblock_cache_ref(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        0 as c_int,
                        (*a).l0.me16x16.i_ref as int8_t,
                    );
                    x264_macroblock_cache_mv(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        0 as c_int,
                        (*((*a).l0.me16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i,
                    );
                    x264_macroblock_cache_ref(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        1 as c_int,
                        -1 as int8_t,
                    );
                    x264_macroblock_cache_mv(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        1 as c_int,
                        0 as uint32_t,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        1 as c_int,
                        0 as uint16_t,
                    );
                }
                12 => {
                    x264_macroblock_cache_ref(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        0 as c_int,
                        -1 as int8_t,
                    );
                    x264_macroblock_cache_mv(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        0 as c_int,
                        0 as uint32_t,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        0 as c_int,
                        0 as uint16_t,
                    );
                    x264_macroblock_cache_ref(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        1 as c_int,
                        (*a).l1.me16x16.i_ref as int8_t,
                    );
                    x264_macroblock_cache_mv(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        1 as c_int,
                        (*((*a).l1.me16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i,
                    );
                }
                16 => {
                    x264_macroblock_cache_ref(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        0 as c_int,
                        (*a).l0.bi16x16.i_ref as int8_t,
                    );
                    x264_macroblock_cache_mv(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        0 as c_int,
                        (*((*a).l0.bi16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i,
                    );
                    x264_macroblock_cache_ref(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        1 as c_int,
                        (*a).l1.bi16x16.i_ref as int8_t,
                    );
                    x264_macroblock_cache_mv(
                        h,
                        0 as c_int,
                        0 as c_int,
                        4 as c_int,
                        4 as c_int,
                        1 as c_int,
                        (*((*a).l1.bi16x16.mv.as_mut_ptr() as *mut x264_union32_t)).i,
                    );
                }
                _ => {}
            },
            14 => {
                mb_cache_mv_b16x8(h, a, 0 as c_int, 1 as c_int);
                mb_cache_mv_b16x8(h, a, 1 as c_int, 1 as c_int);
            }
            15 => {
                mb_cache_mv_b8x16(h, a, 0 as c_int, 1 as c_int);
                mb_cache_mv_b8x16(h, a, 1 as c_int, 1 as c_int);
            }
            _ => {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"internal error (invalid MB type)\n\0" as *const u8 as *const c_char,
                );
            }
        },
    }
    if (*h).i_thread_frames > 1 as c_int
        && !((*h).mb.i_type == I_4x4 as c_int
            || (*h).mb.i_type == I_8x8 as c_int
            || (*h).mb.i_type == I_16x16 as c_int
            || (*h).mb.i_type == I_PCM as c_int)
    {
        let mut l: c_int = 0 as c_int;
        while l <= ((*h).sh.i_type == SLICE_TYPE_B as c_int) as c_int {
            let mut completed: c_int = 0;
            let mut ref_0: c_int = (*h).mb.cache.ref_0[l as usize][x264_scan8[0] as usize] as c_int;
            if !(ref_0 < 0 as c_int) {
                completed = x264_10_frame_cond_wait(
                    (*(*h).fref[l as usize][(ref_0 >> (*h).mb.b_interlaced) as usize]).orig
                        as *mut x264_frame_t,
                    -1,
                );
                if ((*h).mb.cache.mv[l as usize][x264_scan8[15] as usize][1] as c_int
                    >> 2 as c_int - (*h).mb.b_interlaced)
                    + (*h).mb.i_mb_y * 16 as c_int
                    > completed
                {
                    x264_10_log(
                        h,
                        X264_LOG_WARNING,
                        b"internal error (MV out of thread range)\n\0" as *const u8
                            as *const c_char,
                    );
                    x264_10_log(
                        h,
                        X264_LOG_DEBUG,
                        b"mb type: %d \n\0" as *const u8 as *const c_char,
                        (*h).mb.i_type,
                    );
                    x264_10_log(
                        h,
                        X264_LOG_DEBUG,
                        b"mv: l%dr%d (%d,%d) \n\0" as *const u8 as *const c_char,
                        l,
                        ref_0,
                        (*h).mb.cache.mv[l as usize][x264_scan8[15] as usize][0] as c_int,
                        (*h).mb.cache.mv[l as usize][x264_scan8[15] as usize][1] as c_int,
                    );
                    x264_10_log(
                        h,
                        X264_LOG_DEBUG,
                        b"limit: %d \n\0" as *const u8 as *const c_char,
                        (*h).mb.mv_max_spel[1],
                    );
                    x264_10_log(
                        h,
                        X264_LOG_DEBUG,
                        b"mb_xy: %d,%d \n\0" as *const u8 as *const c_char,
                        (*h).mb.i_mb_x,
                        (*h).mb.i_mb_y,
                    );
                    x264_10_log(
                        h,
                        X264_LOG_DEBUG,
                        b"completed: %d \n\0" as *const u8 as *const c_char,
                        completed,
                    );
                    x264_10_log(
                        h,
                        X264_LOG_WARNING,
                        b"recovering by using intra mode\n\0" as *const u8 as *const c_char,
                    );
                    mb_analyse_intra(h, a, COST_MAX);
                    (*h).mb.i_type = I_16x16 as c_int;
                    (*h).mb.i_intra16x16_pred_mode = (*a).i_predict16x16;
                    mb_analyse_intra_chroma(h, a);
                }
            }
            l += 1;
        }
    }
}
