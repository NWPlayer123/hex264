use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

use crate::assert_h::__assert_fail;
use crate::base_h::{
    x264_cabac_mvd_sum, x264_scan8, x264_union32_t, CHROMA_422, CHROMA_444, LUMA_DC, SLICE_TYPE_B,
    SLICE_TYPE_I, SLICE_TYPE_P,
};
use crate::bitstream_h::{bs_flush, bs_init, bs_s, bs_t, bs_write};
use crate::cabac_h::{
    x264_10_cabac_encode_bypass_c, x264_10_cabac_encode_decision_c, x264_10_cabac_encode_flush,
    x264_10_cabac_encode_init_core, x264_10_cabac_encode_terminal_c,
    x264_10_cabac_encode_ue_bypass, x264_cabac_pos, x264_cabac_t,
};
use crate::common_h::{dctcoef, x264_t, FENC_STRIDE, QP_MAX_SPEC};
use crate::internal::BIT_DEPTH;
use crate::macroblock_h::{
    block_idx_x, block_idx_y, ctx_cat_plane, pack8to16, x264_10_mb_predict_mv,
    x264_mb_partition_listX_table, x264_mb_predict_intra4x4_mode, x264_mb_transform_8x8_allowed,
    x264_mb_type_list_table, B_8x8, DCT_LUMA_4x4, DCT_LUMA_8x8, D_16x16, D_16x8, D_8x16, D_BI_8x8,
    D_DIRECT_8x8, D_L0_4x8, D_L0_8x4, D_L0_8x8, D_L1_8x8, I_16x16, I_4x4, I_8x8, P_8x8, B_BI_BI,
    B_DIRECT, B_L0_L0, B_SKIP, DCT_CHROMA_AC, DCT_CHROMA_DC, DCT_LUMA_AC, DCT_LUMA_DC, I_PCM,
    MB_LEFT, MB_TOP, P_L0, P_SKIP,
};
use crate::osdep_h::x264_ctz_4bit;
use crate::predict_h::{
    x264_mb_chroma_pred_mode_fix, x264_mb_pred_mode16x16_fix, x264_mb_pred_mode4x4_fix,
};
use crate::rectangle_h::x264_macroblock_cache_mvd;
use crate::stdint_intn_h::{int16_t, int32_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::stdlib_h::abs;
use crate::tables_h::{
    x264_coeff_abs_level_m1_offset, x264_coeff_flag_offset_chroma_422_dc, x264_count_cat_m1,
    x264_last_coeff_flag_offset, x264_last_coeff_flag_offset_8x8,
    x264_significant_coeff_flag_offset, x264_significant_coeff_flag_offset_8x8,
};
#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn cabac_mb_type_intra(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: c_int,
    mut ctx0: c_int,
    mut ctx1: c_int,
    mut ctx2: c_int,
    mut ctx3: c_int,
    mut ctx4: c_int,
    mut ctx5: c_int,
) {
    if i_mb_type == I_4x4 as c_int || i_mb_type == I_8x8 as c_int {
        x264_10_cabac_encode_decision_c(cb, ctx0, 0 as c_int);
    } else if i_mb_type == I_PCM as c_int {
        x264_10_cabac_encode_decision_c(cb, ctx0, 1 as c_int);
        x264_10_cabac_encode_flush(h, cb);
    } else {
        let mut i_pred: c_int =
            x264_mb_pred_mode16x16_fix[(*h).mb.i_intra16x16_pred_mode as usize] as c_int;
        x264_10_cabac_encode_decision_c(cb, ctx0, 1 as c_int);
        x264_10_cabac_encode_terminal_c(cb);
        x264_10_cabac_encode_decision_c(cb, ctx1, ((*h).mb.i_cbp_luma != 0) as c_int);
        if (*h).mb.i_cbp_chroma == 0 as c_int {
            x264_10_cabac_encode_decision_c(cb, ctx2, 0 as c_int);
        } else {
            x264_10_cabac_encode_decision_c(cb, ctx2, 1 as c_int);
            x264_10_cabac_encode_decision_c(cb, ctx3, (*h).mb.i_cbp_chroma >> 1 as c_int);
        }
        x264_10_cabac_encode_decision_c(cb, ctx4, i_pred >> 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, ctx5, i_pred & 1 as c_int);
    };
}
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn cabac_field_decoding_flag(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut ctx: c_int = 0 as c_int;
    ctx += (*h).mb.field_decoding_flag & ((*h).mb.i_mb_x != 0) as c_int;
    ctx += ((*h).mb.i_mb_top_mbpair_xy >= 0 as c_int
        && *(*h)
            .mb
            .slice_table
            .offset((*h).mb.i_mb_top_mbpair_xy as isize)
            == (*h).sh.i_first_mb as int32_t
        && *(*h).mb.field.offset((*h).mb.i_mb_top_mbpair_xy as isize) as c_int != 0)
        as c_int;
    x264_10_cabac_encode_decision_c(cb, 70 as c_int + ctx, (*h).mb.b_interlaced);
    (*h).mb.field_decoding_flag = (*h).mb.b_interlaced;
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn cabac_intra4x4_pred_mode(
    mut cb: *mut x264_cabac_t,
    mut i_pred: c_int,
    mut i_mode: c_int,
) {
    if i_pred == i_mode {
        x264_10_cabac_encode_decision_c(cb, 68 as c_int, 1 as c_int);
    } else {
        x264_10_cabac_encode_decision_c(cb, 68 as c_int, 0 as c_int);
        if i_mode > i_pred {
            i_mode -= 1;
        }
        x264_10_cabac_encode_decision_c(cb, 69 as c_int, i_mode & 0x1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 69 as c_int, i_mode >> 1 as c_int & 0x1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 69 as c_int, i_mode >> 2 as c_int);
    };
}
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn cabac_intra_chroma_pred_mode(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut i_mode: c_int =
        x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as c_int;
    let mut ctx: c_int = 0 as c_int;
    if (*h).mb.i_neighbour & MB_LEFT as c_int as c_uint != 0
        && *(*h)
            .mb
            .chroma_pred_mode
            .offset((*h).mb.i_mb_left_xy[0] as isize) as c_int
            != 0 as c_int
    {
        ctx += 1;
    }
    if (*h).mb.i_neighbour & MB_TOP as c_int as c_uint != 0
        && *(*h)
            .mb
            .chroma_pred_mode
            .offset((*h).mb.i_mb_top_xy as isize) as c_int
            != 0 as c_int
    {
        ctx += 1;
    }
    x264_10_cabac_encode_decision_c(cb, 64 as c_int + ctx, (i_mode > 0 as c_int) as c_int);
    if i_mode > 0 as c_int {
        x264_10_cabac_encode_decision_c(
            cb,
            64 as c_int + 3 as c_int,
            (i_mode > 1 as c_int) as c_int,
        );
        if i_mode > 1 as c_int {
            x264_10_cabac_encode_decision_c(
                cb,
                64 as c_int + 3 as c_int,
                (i_mode > 2 as c_int) as c_int,
            );
        }
    }
}
#[c2rust::src_loc = "118:1"]
unsafe extern "C" fn cabac_cbp_luma(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut cbp: c_int = (*h).mb.i_cbp_luma;
    let mut cbp_l: c_int = (*h).mb.cache.i_cbp_left;
    let mut cbp_t: c_int = (*h).mb.cache.i_cbp_top;
    x264_10_cabac_encode_decision_c(
        cb,
        76 as c_int - (cbp_l >> 1 as c_int & 1 as c_int) - (cbp_t >> 1 as c_int & 2 as c_int),
        cbp >> 0 as c_int & 1 as c_int,
    );
    x264_10_cabac_encode_decision_c(
        cb,
        76 as c_int - (cbp >> 0 as c_int & 1 as c_int) - (cbp_t >> 2 as c_int & 2 as c_int),
        cbp >> 1 as c_int & 1 as c_int,
    );
    x264_10_cabac_encode_decision_c(
        cb,
        76 as c_int - (cbp_l >> 3 as c_int & 1 as c_int) - (cbp << 1 as c_int & 2 as c_int),
        cbp >> 2 as c_int & 1 as c_int,
    );
    x264_10_cabac_encode_decision_c(
        cb,
        76 as c_int - (cbp >> 2 as c_int & 1 as c_int) - (cbp >> 0 as c_int & 2 as c_int),
        cbp >> 3 as c_int & 1 as c_int,
    );
}
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn cabac_cbp_chroma(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut cbp_a: c_int = (*h).mb.cache.i_cbp_left & 0x30 as c_int;
    let mut cbp_b: c_int = (*h).mb.cache.i_cbp_top & 0x30 as c_int;
    let mut ctx: c_int = 0 as c_int;
    if cbp_a != 0 && (*h).mb.cache.i_cbp_left != -1 {
        ctx += 1;
    }
    if cbp_b != 0 && (*h).mb.cache.i_cbp_top != -1 {
        ctx += 2 as c_int;
    }
    if (*h).mb.i_cbp_chroma == 0 as c_int {
        x264_10_cabac_encode_decision_c(cb, 77 as c_int + ctx, 0 as c_int);
    } else {
        x264_10_cabac_encode_decision_c(cb, 77 as c_int + ctx, 1 as c_int);
        ctx = 4 as c_int;
        if cbp_a == 0x20 as c_int {
            ctx += 1;
        }
        if cbp_b == 0x20 as c_int {
            ctx += 2 as c_int;
        }
        x264_10_cabac_encode_decision_c(cb, 77 as c_int + ctx, (*h).mb.i_cbp_chroma >> 1 as c_int);
    };
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn cabac_qp_delta(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut i_dqp: c_int = (*h).mb.i_qp - (*h).mb.i_last_qp;
    let mut ctx: c_int = 0;
    if (*h).mb.i_type == I_16x16 as c_int
        && *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) == 0
        && (*h).mb.i_qp > (*h).mb.i_last_qp
    {
        (*h).mb.i_qp = (*h).mb.i_last_qp;
        i_dqp = 0 as c_int;
    }
    ctx = ((*h).mb.i_last_dqp != 0
        && (*(*h).mb.type_0.offset((*h).mb.i_mb_prev_xy as isize) as c_int == I_16x16 as c_int
            || *(*h).mb.cbp.offset((*h).mb.i_mb_prev_xy as isize) as c_int & 0x3f as c_int != 0))
        as c_int;
    if i_dqp != 0 as c_int {
        i_dqp *= 2 as c_int;
        let mut val: c_int = 1 as c_int - i_dqp;
        if val < 0 as c_int {
            val = i_dqp;
        }
        val -= 1;
        if val >= QP_MAX_SPEC && val != QP_MAX_SPEC + 1 as c_int {
            val = 2 as c_int * QP_MAX_SPEC + 1 as c_int - val;
        }
        loop {
            x264_10_cabac_encode_decision_c(cb, 60 as c_int + ctx, 1 as c_int);
            ctx = 2 as c_int + (ctx >> 1 as c_int);
            val -= 1;
            if !(val != 0) {
                break;
            }
        }
    }
    x264_10_cabac_encode_decision_c(cb, 60 as c_int + ctx, 0 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "189:1"]
unsafe extern "C" fn x264_10_cabac_mb_skip(mut h: *mut x264_t, mut b_skip: c_int) {
    let mut ctx: c_int = (*h).mb.cache.i_neighbour_skip + 11 as c_int;
    if (*h).sh.i_type != SLICE_TYPE_P as c_int {
        ctx += 13 as c_int;
    }
    x264_10_cabac_encode_decision_c(&mut (*h).cabac, ctx, b_skip);
}
#[inline]
#[c2rust::src_loc = "198:1"]
unsafe extern "C" fn cabac_subpartition_p(mut cb: *mut x264_cabac_t, mut i_sub: c_int) {
    if i_sub == D_L0_8x8 as c_int {
        x264_10_cabac_encode_decision_c(cb, 21 as c_int, 1 as c_int);
        return;
    }
    x264_10_cabac_encode_decision_c(cb, 21 as c_int, 0 as c_int);
    if i_sub == D_L0_8x4 as c_int {
        x264_10_cabac_encode_decision_c(cb, 22 as c_int, 0 as c_int);
    } else {
        x264_10_cabac_encode_decision_c(cb, 22 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 23 as c_int, (i_sub == D_L0_4x8 as c_int) as c_int);
    };
}
#[inline(always)]
#[c2rust::src_loc = "215:1"]
unsafe extern "C" fn cabac_subpartition_b(mut cb: *mut x264_cabac_t, mut i_sub: c_int) {
    if i_sub == D_DIRECT_8x8 as c_int {
        x264_10_cabac_encode_decision_c(cb, 36 as c_int, 0 as c_int);
        return;
    }
    x264_10_cabac_encode_decision_c(cb, 36 as c_int, 1 as c_int);
    if i_sub == D_BI_8x8 as c_int {
        x264_10_cabac_encode_decision_c(cb, 37 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 38 as c_int, 0 as c_int);
        x264_10_cabac_encode_decision_c(cb, 39 as c_int, 0 as c_int);
        x264_10_cabac_encode_decision_c(cb, 39 as c_int, 0 as c_int);
        return;
    }
    x264_10_cabac_encode_decision_c(cb, 37 as c_int, 0 as c_int);
    x264_10_cabac_encode_decision_c(cb, 39 as c_int, (i_sub == D_L1_8x8 as c_int) as c_int);
}
#[inline(always)]
#[c2rust::src_loc = "235:1"]
unsafe extern "C" fn cabac_transform_size(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut ctx: c_int = 399 as c_int + (*h).mb.cache.i_neighbour_transform_size;
    x264_10_cabac_encode_decision_c(cb, ctx, (*h).mb.b_transform_8x8);
}
#[inline(always)]
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn cabac_ref_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: c_int,
    mut idx: c_int,
    mut bframe: c_int,
) {
    let i8: c_int = x264_scan8[idx as usize] as c_int;
    let i_refa: c_int = (*h).mb.cache.ref_0[i_list as usize][(i8 - 1 as c_int) as usize] as c_int;
    let i_refb: c_int = (*h).mb.cache.ref_0[i_list as usize][(i8 - 8 as c_int) as usize] as c_int;
    let mut ctx: c_int = 0 as c_int;
    if i_refa > 0 as c_int && (bframe == 0 || (*h).mb.cache.skip[(i8 - 1 as c_int) as usize] == 0) {
        ctx += 1;
    }
    if i_refb > 0 as c_int && (bframe == 0 || (*h).mb.cache.skip[(i8 - 8 as c_int) as usize] == 0) {
        ctx += 2 as c_int;
    }
    let mut i_ref: c_int = (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as c_int;
    while i_ref > 0 as c_int {
        x264_10_cabac_encode_decision_c(cb, 54 as c_int + ctx, 1 as c_int);
        ctx = (ctx >> 2 as c_int) + 4 as c_int;
        i_ref -= 1;
    }
    x264_10_cabac_encode_decision_c(cb, 54 as c_int + ctx, 0 as c_int);
}
#[inline(never)]
#[c2rust::src_loc = "261:1"]
unsafe extern "C" fn cabac_ref_p(mut h: *mut x264_t, mut cb: *mut x264_cabac_t, mut idx: c_int) {
    cabac_ref_internal(h, cb, 0 as c_int, idx, 0 as c_int);
}
#[inline(never)]
#[c2rust::src_loc = "265:1"]
unsafe extern "C" fn cabac_ref_b(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: c_int,
    mut idx: c_int,
) {
    cabac_ref_internal(h, cb, i_list, idx, 1 as c_int);
}
#[inline(always)]
#[c2rust::src_loc = "270:1"]
unsafe extern "C" fn cabac_mvd_cpn(
    mut _h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut _i_list: c_int,
    mut _idx: c_int,
    mut l: c_int,
    mut mvd: c_int,
    mut ctx: c_int,
) -> c_int {
    let mut ctxbase: c_int = if l != 0 { 47 as c_int } else { 40 as c_int };
    if mvd == 0 as c_int {
        x264_10_cabac_encode_decision_c(cb, ctxbase + ctx, 0 as c_int);
        return 0 as c_int;
    }
    let mut i_abs: c_int = abs(mvd);
    x264_10_cabac_encode_decision_c(cb, ctxbase + ctx, 1 as c_int);
    static mut ctxes: [uint8_t; 8] = [
        3 as c_int as uint8_t,
        4 as c_int as uint8_t,
        5 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
        6 as c_int as uint8_t,
    ];
    if i_abs < 9 as c_int {
        let mut i: c_int = 1 as c_int;
        while i < i_abs {
            x264_10_cabac_encode_decision_c(
                cb,
                ctxbase + ctxes[(i - 1 as c_int) as usize] as c_int,
                1 as c_int,
            );
            i += 1;
        }
        x264_10_cabac_encode_decision_c(
            cb,
            ctxbase + ctxes[(i_abs - 1 as c_int) as usize] as c_int,
            0 as c_int,
        );
    } else {
        let mut i_0: c_int = 1 as c_int;
        while i_0 < 9 as c_int {
            x264_10_cabac_encode_decision_c(
                cb,
                ctxbase + ctxes[(i_0 - 1 as c_int) as usize] as c_int,
                1 as c_int,
            );
            i_0 += 1;
        }
        x264_10_cabac_encode_ue_bypass(cb, 3 as c_int, i_abs - 9 as c_int);
    }
    x264_10_cabac_encode_bypass_c(cb, mvd >> 31 as c_int);
    return if i_abs < 66 as c_int {
        i_abs
    } else {
        66 as c_int
    };
}
#[inline(never)]
#[c2rust::src_loc = "329:1"]
unsafe extern "C" fn cabac_mvd(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: c_int,
    mut idx: c_int,
    mut width: c_int,
) -> uint16_t {
    let mut mvp: [int16_t; 2] = [0; 2];
    let mut mdx: c_int = 0;
    let mut mdy: c_int = 0;
    x264_10_mb_predict_mv(h, i_list, idx, width, mvp.as_mut_ptr());
    mdx = (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize][0] as c_int
        - mvp[0] as c_int;
    mdy = (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize][1] as c_int
        - mvp[1] as c_int;
    let mut amvd: uint16_t = x264_cabac_mvd_sum(
        (*(*(*h).mb.cache.mvd.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((*x264_scan8.as_ptr().offset(idx as isize) as c_int - 1 as c_int) as isize))
        .as_mut_ptr(),
        (*(*(*h).mb.cache.mvd.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((*x264_scan8.as_ptr().offset(idx as isize) as c_int - 8 as c_int) as isize))
        .as_mut_ptr(),
    );
    mdx = cabac_mvd_cpn(
        h,
        cb,
        i_list,
        idx,
        0 as c_int,
        mdx,
        amvd as c_int & 0xff as c_int,
    );
    mdy = cabac_mvd_cpn(
        h,
        cb,
        i_list,
        idx,
        1 as c_int,
        mdy,
        amvd as c_int >> 8 as c_int,
    );
    return pack8to16(mdx as uint32_t, mdy as uint32_t) as uint16_t;
}
#[inline]
#[c2rust::src_loc = "355:1"]
unsafe extern "C" fn cabac_8x8_mvd(mut h: *mut x264_t, mut cb: *mut x264_cabac_t, mut i: c_int) {
    match (*h).mb.i_sub_partition[i as usize] as c_int {
        3 => {
            let mut mvd: uint16_t = cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i, 2 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i) as usize] as c_int,
                block_idx_y[(4 as c_int * i) as usize] as c_int,
                2 as c_int,
                2 as c_int,
                0 as c_int,
                mvd,
            );
        }
        1 => {
            let mut mvd_0: uint16_t =
                cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i + 0 as c_int, 2 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i + 0 as c_int) as usize] as c_int,
                block_idx_y[(4 as c_int * i + 0 as c_int) as usize] as c_int,
                2 as c_int,
                1 as c_int,
                0 as c_int,
                mvd_0,
            );
            let mut mvd_1: uint16_t =
                cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i + 2 as c_int, 2 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i + 2 as c_int) as usize] as c_int,
                block_idx_y[(4 as c_int * i + 2 as c_int) as usize] as c_int,
                2 as c_int,
                1 as c_int,
                0 as c_int,
                mvd_1,
            );
        }
        2 => {
            let mut mvd_2: uint16_t =
                cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i + 0 as c_int, 1 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i + 0 as c_int) as usize] as c_int,
                block_idx_y[(4 as c_int * i + 0 as c_int) as usize] as c_int,
                1 as c_int,
                2 as c_int,
                0 as c_int,
                mvd_2,
            );
            let mut mvd_3: uint16_t =
                cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i + 1 as c_int, 1 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i + 1 as c_int) as usize] as c_int,
                block_idx_y[(4 as c_int * i + 1 as c_int) as usize] as c_int,
                1 as c_int,
                2 as c_int,
                0 as c_int,
                mvd_3,
            );
        }
        0 => {
            let mut mvd_4: uint16_t =
                cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i + 0 as c_int, 1 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i + 0 as c_int) as usize] as c_int,
                block_idx_y[(4 as c_int * i + 0 as c_int) as usize] as c_int,
                1 as c_int,
                1 as c_int,
                0 as c_int,
                mvd_4,
            );
            let mut mvd_5: uint16_t =
                cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i + 1 as c_int, 1 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i + 1 as c_int) as usize] as c_int,
                block_idx_y[(4 as c_int * i + 1 as c_int) as usize] as c_int,
                1 as c_int,
                1 as c_int,
                0 as c_int,
                mvd_5,
            );
            let mut mvd_6: uint16_t =
                cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i + 2 as c_int, 1 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i + 2 as c_int) as usize] as c_int,
                block_idx_y[(4 as c_int * i + 2 as c_int) as usize] as c_int,
                1 as c_int,
                1 as c_int,
                0 as c_int,
                mvd_6,
            );
            let mut mvd_7: uint16_t =
                cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i + 3 as c_int, 1 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as c_int * i + 3 as c_int) as usize] as c_int,
                block_idx_y[(4 as c_int * i + 3 as c_int) as usize] as c_int,
                1 as c_int,
                1 as c_int,
                0 as c_int,
                mvd_7,
            );
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const c_char,
                b"encoder/cabac.c\0" as *const u8 as *const c_char,
                377 as c_uint,
                ::core::mem::transmute::<[u8; 50], [c_char; 50]>(
                    *b"void cabac_8x8_mvd(x264_t *, x264_cabac_t *, int)\0",
                )
                .as_ptr(),
            );
        }
    };
}
#[inline(always)]
#[c2rust::src_loc = "381:1"]
unsafe extern "C" fn cabac_mb_header_i(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: c_int,
    mut slice_type: c_int,
    mut chroma: c_int,
) {
    if slice_type == SLICE_TYPE_I as c_int {
        let mut ctx: c_int = 0 as c_int;
        if (*h).mb.i_neighbour & MB_LEFT as c_int as c_uint != 0
            && (*h).mb.i_mb_type_left[0] != I_4x4 as c_int
        {
            ctx += 1;
        }
        if (*h).mb.i_neighbour & MB_TOP as c_int as c_uint != 0
            && (*h).mb.i_mb_type_top != I_4x4 as c_int
        {
            ctx += 1;
        }
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            3 as c_int + ctx,
            3 as c_int + 3 as c_int,
            3 as c_int + 4 as c_int,
            3 as c_int + 5 as c_int,
            3 as c_int + 6 as c_int,
            3 as c_int + 7 as c_int,
        );
    } else if slice_type == SLICE_TYPE_P as c_int {
        x264_10_cabac_encode_decision_c(cb, 14 as c_int, 1 as c_int);
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            17 as c_int + 0 as c_int,
            17 as c_int + 1 as c_int,
            17 as c_int + 2 as c_int,
            17 as c_int + 2 as c_int,
            17 as c_int + 3 as c_int,
            17 as c_int + 3 as c_int,
        );
    } else if slice_type == SLICE_TYPE_B as c_int {
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 3 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 4 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, 0 as c_int);
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, 1 as c_int);
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            32 as c_int + 0 as c_int,
            32 as c_int + 1 as c_int,
            32 as c_int + 2 as c_int,
            32 as c_int + 2 as c_int,
            32 as c_int + 3 as c_int,
            32 as c_int + 3 as c_int,
        );
    }
    if i_mb_type == I_PCM as c_int {
        return;
    }
    if i_mb_type != I_16x16 as c_int {
        if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
            cabac_transform_size(h, cb);
        }
        let mut di: c_int = if (*h).mb.b_transform_8x8 != 0 {
            4 as c_int
        } else {
            1 as c_int
        };
        let mut i: c_int = 0 as c_int;
        while i < 16 as c_int {
            let i_pred: c_int = x264_mb_predict_intra4x4_mode(h, i) as c_int;
            let i_mode: c_int = x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                [x264_scan8[i as usize] as usize]
                as c_int
                + 1 as c_int) as usize] as c_int;
            cabac_intra4x4_pred_mode(cb, i_pred, i_mode);
            i += di;
        }
    }
    if chroma != 0 {
        cabac_intra_chroma_pred_mode(h, cb);
    }
}
#[inline(always)]
#[c2rust::src_loc = "435:1"]
unsafe extern "C" fn cabac_mb_header_p(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: c_int,
    mut chroma: c_int,
) {
    if i_mb_type == P_L0 as c_int {
        x264_10_cabac_encode_decision_c(cb, 14 as c_int, 0 as c_int);
        if (*h).mb.i_partition == D_16x16 as c_int {
            x264_10_cabac_encode_decision_c(cb, 15 as c_int, 0 as c_int);
            x264_10_cabac_encode_decision_c(cb, 16 as c_int, 0 as c_int);
            if (*h).mb.pic.i_fref[0] > 1 as c_int {
                cabac_ref_p(h, cb, 0 as c_int);
            }
            let mut mvd: uint16_t = cabac_mvd(h, cb, 0 as c_int, 0 as c_int, 4 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0] as c_int,
                block_idx_y[0] as c_int,
                4 as c_int,
                4 as c_int,
                0 as c_int,
                mvd,
            );
        } else if (*h).mb.i_partition == D_16x8 as c_int {
            x264_10_cabac_encode_decision_c(cb, 15 as c_int, 1 as c_int);
            x264_10_cabac_encode_decision_c(cb, 17 as c_int, 1 as c_int);
            if (*h).mb.pic.i_fref[0] > 1 as c_int {
                cabac_ref_p(h, cb, 0 as c_int);
                cabac_ref_p(h, cb, 8 as c_int);
            }
            let mut mvd_0: uint16_t = cabac_mvd(h, cb, 0 as c_int, 0 as c_int, 4 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0] as c_int,
                block_idx_y[0] as c_int,
                4 as c_int,
                2 as c_int,
                0 as c_int,
                mvd_0,
            );
            let mut mvd_1: uint16_t = cabac_mvd(h, cb, 0 as c_int, 8 as c_int, 4 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[8] as c_int,
                block_idx_y[8] as c_int,
                4 as c_int,
                2 as c_int,
                0 as c_int,
                mvd_1,
            );
        } else {
            x264_10_cabac_encode_decision_c(cb, 15 as c_int, 1 as c_int);
            x264_10_cabac_encode_decision_c(cb, 17 as c_int, 0 as c_int);
            if (*h).mb.pic.i_fref[0] > 1 as c_int {
                cabac_ref_p(h, cb, 0 as c_int);
                cabac_ref_p(h, cb, 4 as c_int);
            }
            let mut mvd_2: uint16_t = cabac_mvd(h, cb, 0 as c_int, 0 as c_int, 2 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0] as c_int,
                block_idx_y[0] as c_int,
                2 as c_int,
                4 as c_int,
                0 as c_int,
                mvd_2,
            );
            let mut mvd_3: uint16_t = cabac_mvd(h, cb, 0 as c_int, 4 as c_int, 2 as c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[4] as c_int,
                block_idx_y[4] as c_int,
                2 as c_int,
                4 as c_int,
                0 as c_int,
                mvd_3,
            );
        }
    } else if i_mb_type == P_8x8 as c_int {
        x264_10_cabac_encode_decision_c(cb, 14 as c_int, 0 as c_int);
        x264_10_cabac_encode_decision_c(cb, 15 as c_int, 0 as c_int);
        x264_10_cabac_encode_decision_c(cb, 16 as c_int, 1 as c_int);
        let mut i: c_int = 0 as c_int;
        while i < 4 as c_int {
            cabac_subpartition_p(cb, (*h).mb.i_sub_partition[i as usize] as c_int);
            i += 1;
        }
        if (*h).mb.pic.i_fref[0] > 1 as c_int {
            cabac_ref_p(h, cb, 0 as c_int);
            cabac_ref_p(h, cb, 4 as c_int);
            cabac_ref_p(h, cb, 8 as c_int);
            cabac_ref_p(h, cb, 12 as c_int);
        }
        let mut i_0: c_int = 0 as c_int;
        while i_0 < 4 as c_int {
            cabac_8x8_mvd(h, cb, i_0);
            i_0 += 1;
        }
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_P as c_int, chroma);
    };
}
#[inline(always)]
#[c2rust::src_loc = "499:1"]
unsafe extern "C" fn cabac_mb_header_b(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: c_int,
    mut chroma: c_int,
) {
    let mut ctx: c_int = 0 as c_int;
    if (*h).mb.i_neighbour & MB_LEFT as c_int as c_uint != 0
        && (*h).mb.i_mb_type_left[0] != B_SKIP as c_int
        && (*h).mb.i_mb_type_left[0] != B_DIRECT as c_int
    {
        ctx += 1;
    }
    if (*h).mb.i_neighbour & MB_TOP as c_int as c_uint != 0
        && (*h).mb.i_mb_type_top != B_SKIP as c_int
        && (*h).mb.i_mb_type_top != B_DIRECT as c_int
    {
        ctx += 1;
    }
    if i_mb_type == B_DIRECT as c_int {
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + ctx, 0 as c_int);
        return;
    }
    x264_10_cabac_encode_decision_c(cb, 27 as c_int + ctx, 1 as c_int);
    if i_mb_type == B_8x8 as c_int {
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 3 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 4 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, 1 as c_int);
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, 1 as c_int);
        let mut i: c_int = 0 as c_int;
        while i < 4 as c_int {
            cabac_subpartition_b(cb, (*h).mb.i_sub_partition[i as usize] as c_int);
            i += 1;
        }
        if (*h).mb.pic.i_fref[0] > 1 as c_int {
            let mut i_0: c_int = 0 as c_int;
            while i_0 < 4 as c_int {
                if x264_mb_partition_listX_table[0][(*h).mb.i_sub_partition[i_0 as usize] as usize]
                    != 0
                {
                    cabac_ref_b(h, cb, 0 as c_int, 4 as c_int * i_0);
                }
                i_0 += 1;
            }
        }
        if (*h).mb.pic.i_fref[1] > 1 as c_int {
            let mut i_1: c_int = 0 as c_int;
            while i_1 < 4 as c_int {
                if x264_mb_partition_listX_table[1][(*h).mb.i_sub_partition[i_1 as usize] as usize]
                    != 0
                {
                    cabac_ref_b(h, cb, 1 as c_int, 4 as c_int * i_1);
                }
                i_1 += 1;
            }
        }
        let mut i_2: c_int = 0 as c_int;
        while i_2 < 4 as c_int {
            if x264_mb_partition_listX_table[0][(*h).mb.i_sub_partition[i_2 as usize] as usize] != 0
            {
                let mut mvd: uint16_t = cabac_mvd(h, cb, 0 as c_int, 4 as c_int * i_2, 2 as c_int);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as c_int * i_2) as usize] as c_int,
                    block_idx_y[(4 as c_int * i_2) as usize] as c_int,
                    2 as c_int,
                    2 as c_int,
                    0 as c_int,
                    mvd,
                );
            }
            i_2 += 1;
        }
        let mut i_3: c_int = 0 as c_int;
        while i_3 < 4 as c_int {
            if x264_mb_partition_listX_table[1][(*h).mb.i_sub_partition[i_3 as usize] as usize] != 0
            {
                let mut mvd_0: uint16_t =
                    cabac_mvd(h, cb, 1 as c_int, 4 as c_int * i_3, 2 as c_int);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as c_int * i_3) as usize] as c_int,
                    block_idx_y[(4 as c_int * i_3) as usize] as c_int,
                    2 as c_int,
                    2 as c_int,
                    1 as c_int,
                    mvd_0,
                );
            }
            i_3 += 1;
        }
    } else if i_mb_type >= B_L0_L0 as c_int && i_mb_type <= B_BI_BI as c_int {
        static mut i_mb_bits: [uint8_t; 27] = [
            0x31 as c_int as uint8_t,
            0x29 as c_int as uint8_t,
            0x4 as c_int as uint8_t,
            0x35 as c_int as uint8_t,
            0x2d as c_int as uint8_t,
            0 as c_int as uint8_t,
            0x43 as c_int as uint8_t,
            0x63 as c_int as uint8_t,
            0 as c_int as uint8_t,
            0x3d as c_int as uint8_t,
            0x2f as c_int as uint8_t,
            0 as c_int as uint8_t,
            0x39 as c_int as uint8_t,
            0x25 as c_int as uint8_t,
            0x6 as c_int as uint8_t,
            0x53 as c_int as uint8_t,
            0x73 as c_int as uint8_t,
            0 as c_int as uint8_t,
            0x4b as c_int as uint8_t,
            0x6b as c_int as uint8_t,
            0 as c_int as uint8_t,
            0x5b as c_int as uint8_t,
            0x7b as c_int as uint8_t,
            0 as c_int as uint8_t,
            0x47 as c_int as uint8_t,
            0x67 as c_int as uint8_t,
            0x21 as c_int as uint8_t,
        ];
        let idx: c_int = (i_mb_type - B_L0_L0 as c_int as c_int) * 3 as c_int
            + ((*h).mb.i_partition - D_16x8 as c_int as c_int);
        let mut bits: c_int = i_mb_bits[idx as usize] as c_int;
        x264_10_cabac_encode_decision_c(cb, 27 as c_int + 3 as c_int, bits & 1 as c_int);
        x264_10_cabac_encode_decision_c(
            cb,
            27 as c_int + 5 as c_int - (bits & 1 as c_int),
            bits >> 1 as c_int & 1 as c_int,
        );
        bits >>= 2 as c_int;
        if bits != 1 as c_int {
            x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, bits & 1 as c_int);
            bits >>= 1 as c_int;
            x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, bits & 1 as c_int);
            bits >>= 1 as c_int;
            x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, bits & 1 as c_int);
            bits >>= 1 as c_int;
            if bits != 1 as c_int {
                x264_10_cabac_encode_decision_c(cb, 27 as c_int + 5 as c_int, bits & 1 as c_int);
            }
        }
        let mut b_list: *const [uint8_t; 2] =
            (*x264_mb_type_list_table.as_ptr().offset(i_mb_type as isize)).as_ptr()
                as *const [uint8_t; 2];
        if (*h).mb.pic.i_fref[0] > 1 as c_int {
            if (*b_list.offset(0))[0] != 0 {
                cabac_ref_b(h, cb, 0 as c_int, 0 as c_int);
            }
            if (*b_list.offset(0))[1] as c_int != 0 && (*h).mb.i_partition != D_16x16 as c_int {
                cabac_ref_b(
                    h,
                    cb,
                    0 as c_int,
                    8 as c_int >> ((*h).mb.i_partition == D_8x16 as c_int) as c_int,
                );
            }
        }
        if (*h).mb.pic.i_fref[1] > 1 as c_int {
            if (*b_list.offset(1))[0] != 0 {
                cabac_ref_b(h, cb, 1 as c_int, 0 as c_int);
            }
            if (*b_list.offset(1))[1] as c_int != 0 && (*h).mb.i_partition != D_16x16 as c_int {
                cabac_ref_b(
                    h,
                    cb,
                    1 as c_int,
                    8 as c_int >> ((*h).mb.i_partition == D_8x16 as c_int) as c_int,
                );
            }
        }
        let mut i_list: c_int = 0 as c_int;
        while i_list < 2 as c_int {
            if (*h).mb.i_partition == D_16x16 as c_int {
                if (*b_list.offset(i_list as isize))[0] != 0 {
                    let mut mvd_1: uint16_t = cabac_mvd(h, cb, i_list, 0 as c_int, 4 as c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0] as c_int,
                        block_idx_y[0] as c_int,
                        4 as c_int,
                        4 as c_int,
                        i_list,
                        mvd_1,
                    );
                }
            } else if (*h).mb.i_partition == D_16x8 as c_int {
                if (*b_list.offset(i_list as isize))[0] != 0 {
                    let mut mvd_2: uint16_t = cabac_mvd(h, cb, i_list, 0 as c_int, 4 as c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0] as c_int,
                        block_idx_y[0] as c_int,
                        4 as c_int,
                        2 as c_int,
                        i_list,
                        mvd_2,
                    );
                }
                if (*b_list.offset(i_list as isize))[1] != 0 {
                    let mut mvd_3: uint16_t = cabac_mvd(h, cb, i_list, 8 as c_int, 4 as c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[8] as c_int,
                        block_idx_y[8] as c_int,
                        4 as c_int,
                        2 as c_int,
                        i_list,
                        mvd_3,
                    );
                }
            } else {
                if (*b_list.offset(i_list as isize))[0] != 0 {
                    let mut mvd_4: uint16_t = cabac_mvd(h, cb, i_list, 0 as c_int, 2 as c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0] as c_int,
                        block_idx_y[0] as c_int,
                        2 as c_int,
                        4 as c_int,
                        i_list,
                        mvd_4,
                    );
                }
                if (*b_list.offset(i_list as isize))[1] != 0 {
                    let mut mvd_5: uint16_t = cabac_mvd(h, cb, i_list, 4 as c_int, 2 as c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[4] as c_int,
                        block_idx_y[4] as c_int,
                        2 as c_int,
                        4 as c_int,
                        i_list,
                        mvd_5,
                    );
                }
            }
            i_list += 1;
        }
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_B as c_int, chroma);
    };
}
#[inline(always)]
#[c2rust::src_loc = "612:1"]
unsafe extern "C" fn cabac_cbf_ctxidxinc(
    mut h: *mut x264_t,
    mut i_cat: c_int,
    mut i_idx: c_int,
    mut b_intra: c_int,
    mut b_dc: c_int,
) -> c_int {
    static mut base_ctx: [uint16_t; 14] = [
        85 as c_int as uint16_t,
        89 as c_int as uint16_t,
        93 as c_int as uint16_t,
        97 as c_int as uint16_t,
        101 as c_int as uint16_t,
        1012 as c_int as uint16_t,
        460 as c_int as uint16_t,
        464 as c_int as uint16_t,
        468 as c_int as uint16_t,
        1016 as c_int as uint16_t,
        472 as c_int as uint16_t,
        476 as c_int as uint16_t,
        480 as c_int as uint16_t,
        1020 as c_int as uint16_t,
    ];
    if b_dc != 0 {
        i_idx -= LUMA_DC;
        if i_cat == DCT_CHROMA_DC as c_int {
            let mut i_nza: c_int = if (*h).mb.cache.i_cbp_left != -1 {
                (*h).mb.cache.i_cbp_left >> 8 as c_int + i_idx & 1 as c_int
            } else {
                b_intra
            };
            let mut i_nzb: c_int = if (*h).mb.cache.i_cbp_top != -1 {
                (*h).mb.cache.i_cbp_top >> 8 as c_int + i_idx & 1 as c_int
            } else {
                b_intra
            };
            return base_ctx[i_cat as usize] as c_int + 2 as c_int * i_nzb + i_nza;
        } else {
            let mut i_nza_0: c_int = (*h).mb.cache.i_cbp_left >> 8 as c_int + i_idx & 1 as c_int;
            let mut i_nzb_0: c_int = (*h).mb.cache.i_cbp_top >> 8 as c_int + i_idx & 1 as c_int;
            return base_ctx[i_cat as usize] as c_int + 2 as c_int * i_nzb_0 + i_nza_0;
        }
    } else {
        let mut i_nza_1: c_int = (*h).mb.cache.non_zero_count
            [(x264_scan8[i_idx as usize] as c_int - 1 as c_int) as usize]
            as c_int;
        let mut i_nzb_1: c_int = (*h).mb.cache.non_zero_count
            [(x264_scan8[i_idx as usize] as c_int - 8 as c_int) as usize]
            as c_int;
        if 0 != 0 && b_intra == 0 {
            return base_ctx[i_cat as usize] as c_int
                + (2 as c_int * i_nzb_1 + i_nza_1 & 0x7f as c_int);
        } else {
            i_nza_1 &= 0x7f as c_int + (b_intra << 7 as c_int);
            i_nzb_1 &= 0x7f as c_int + (b_intra << 7 as c_int);
            return base_ctx[i_cat as usize] as c_int
                + 2 as c_int * (i_nzb_1 != 0) as c_int
                + (i_nza_1 != 0) as c_int;
        }
    };
}
#[c2rust::src_loc = "650:22"]
static mut coeff_abs_level1_ctx: [uint8_t; 8] = [
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    3 as c_int as uint8_t,
    4 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
];
#[c2rust::src_loc = "652:22"]
static mut coeff_abs_levelgt1_ctx: [uint8_t; 8] = [
    5 as c_int as uint8_t,
    5 as c_int as uint8_t,
    5 as c_int as uint8_t,
    5 as c_int as uint8_t,
    6 as c_int as uint8_t,
    7 as c_int as uint8_t,
    8 as c_int as uint8_t,
    9 as c_int as uint8_t,
];
#[c2rust::src_loc = "655:22"]
static mut coeff_abs_levelgt1_ctx_chroma_dc: [uint8_t; 8] = [
    5 as c_int as uint8_t,
    5 as c_int as uint8_t,
    5 as c_int as uint8_t,
    5 as c_int as uint8_t,
    6 as c_int as uint8_t,
    7 as c_int as uint8_t,
    8 as c_int as uint8_t,
    8 as c_int as uint8_t,
];
#[c2rust::src_loc = "657:22"]
static mut coeff_abs_level_transition: [[uint8_t; 8]; 2] = [
    [
        1 as c_int as uint8_t,
        2 as c_int as uint8_t,
        3 as c_int as uint8_t,
        3 as c_int as uint8_t,
        4 as c_int as uint8_t,
        5 as c_int as uint8_t,
        6 as c_int as uint8_t,
        7 as c_int as uint8_t,
    ],
    [
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        4 as c_int as uint8_t,
        5 as c_int as uint8_t,
        6 as c_int as uint8_t,
        7 as c_int as uint8_t,
        7 as c_int as uint8_t,
    ],
];
#[inline(always)]
#[c2rust::src_loc = "665:1"]
unsafe extern "C" fn cabac_block_residual_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: c_int,
    mut l: *mut dctcoef,
    mut chroma422dc: c_int,
) {
    let mut ctx_sig: c_int = x264_significant_coeff_flag_offset[(*h).mb.b_interlaced as usize]
        [ctx_block_cat as usize] as c_int;
    let mut ctx_last: c_int =
        x264_last_coeff_flag_offset[(*h).mb.b_interlaced as usize][ctx_block_cat as usize] as c_int;
    let mut ctx_level: c_int = x264_coeff_abs_level_m1_offset[ctx_block_cat as usize] as c_int;
    let mut coeff_idx: c_int = -1;
    let mut node_ctx: c_int = 0 as c_int;
    let mut last: c_int =
        (*h).quantf.coeff_last[ctx_block_cat as usize].expect("non-null function pointer")(l);
    let mut levelgt1_ctx: *const uint8_t = if chroma422dc != 0 {
        coeff_abs_levelgt1_ctx_chroma_dc.as_ptr()
    } else {
        coeff_abs_levelgt1_ctx.as_ptr()
    };
    let mut coeffs: [dctcoef; 64] = [0; 64];
    if chroma422dc != 0 {
        let mut count_m1: c_int = 7 as c_int;
        let mut i: c_int = 0 as c_int;
        loop {
            if *l.offset(i as isize) != 0 {
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i as isize);
                x264_10_cabac_encode_decision_c(
                    cb,
                    ctx_sig + x264_coeff_flag_offset_chroma_422_dc[i as usize] as c_int,
                    1 as c_int,
                );
                if i == last {
                    x264_10_cabac_encode_decision_c(
                        cb,
                        ctx_last + x264_coeff_flag_offset_chroma_422_dc[i as usize] as c_int,
                        1 as c_int,
                    );
                    break;
                } else {
                    x264_10_cabac_encode_decision_c(
                        cb,
                        ctx_last + x264_coeff_flag_offset_chroma_422_dc[i as usize] as c_int,
                        0 as c_int,
                    );
                }
            } else {
                x264_10_cabac_encode_decision_c(
                    cb,
                    ctx_sig + x264_coeff_flag_offset_chroma_422_dc[i as usize] as c_int,
                    0 as c_int,
                );
            }
            i += 1;
            if !(i == count_m1) {
                continue;
            }
            coeff_idx += 1;
            coeffs[coeff_idx as usize] = *l.offset(i as isize);
            break;
        }
    } else {
        let mut count_m1_0: c_int = x264_count_cat_m1[ctx_block_cat as usize] as c_int;
        if count_m1_0 == 63 as c_int {
            let mut sig_offset: *const uint8_t = (*x264_significant_coeff_flag_offset_8x8
                .as_ptr()
                .offset((*h).mb.b_interlaced as isize))
            .as_ptr();
            let mut i_0: c_int = 0 as c_int;
            loop {
                if *l.offset(i_0 as isize) != 0 {
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                    x264_10_cabac_encode_decision_c(
                        cb,
                        ctx_sig + *sig_offset.offset(i_0 as isize) as c_int,
                        1 as c_int,
                    );
                    if i_0 == last {
                        x264_10_cabac_encode_decision_c(
                            cb,
                            ctx_last + x264_last_coeff_flag_offset_8x8[i_0 as usize] as c_int,
                            1 as c_int,
                        );
                        break;
                    } else {
                        x264_10_cabac_encode_decision_c(
                            cb,
                            ctx_last + x264_last_coeff_flag_offset_8x8[i_0 as usize] as c_int,
                            0 as c_int,
                        );
                    }
                } else {
                    x264_10_cabac_encode_decision_c(
                        cb,
                        ctx_sig + *sig_offset.offset(i_0 as isize) as c_int,
                        0 as c_int,
                    );
                }
                i_0 += 1;
                if !(i_0 == count_m1_0) {
                    continue;
                }
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                break;
            }
        } else {
            let mut i_1: c_int = 0 as c_int;
            loop {
                if *l.offset(i_1 as isize) != 0 {
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                    x264_10_cabac_encode_decision_c(cb, ctx_sig + i_1, 1 as c_int);
                    if i_1 == last {
                        x264_10_cabac_encode_decision_c(cb, ctx_last + i_1, 1 as c_int);
                        break;
                    } else {
                        x264_10_cabac_encode_decision_c(cb, ctx_last + i_1, 0 as c_int);
                    }
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctx_sig + i_1, 0 as c_int);
                }
                i_1 += 1;
                if !(i_1 == count_m1_0) {
                    continue;
                }
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                break;
            }
        }
    }
    loop {
        let mut coeff: c_int = coeffs[coeff_idx as usize];
        let mut abs_coeff: c_int = abs(coeff);
        let mut coeff_sign: c_int = coeff >> 31 as c_int;
        let mut ctx: c_int = coeff_abs_level1_ctx[node_ctx as usize] as c_int + ctx_level;
        if abs_coeff > 1 as c_int {
            x264_10_cabac_encode_decision_c(cb, ctx, 1 as c_int);
            ctx = *levelgt1_ctx.offset(node_ctx as isize) as c_int + ctx_level;
            let mut i_2: c_int = (if abs_coeff < 15 as c_int {
                abs_coeff
            } else {
                15 as c_int
            }) - 2 as c_int;
            while i_2 > 0 as c_int {
                x264_10_cabac_encode_decision_c(cb, ctx, 1 as c_int);
                i_2 -= 1;
            }
            if abs_coeff < 15 as c_int {
                x264_10_cabac_encode_decision_c(cb, ctx, 0 as c_int);
            } else {
                x264_10_cabac_encode_ue_bypass(cb, 0 as c_int, abs_coeff - 15 as c_int);
            }
            node_ctx = coeff_abs_level_transition[1][node_ctx as usize] as c_int;
        } else {
            x264_10_cabac_encode_decision_c(cb, ctx, 0 as c_int);
            node_ctx = coeff_abs_level_transition[0][node_ctx as usize] as c_int;
        }
        x264_10_cabac_encode_bypass_c(cb, coeff_sign);
        coeff_idx -= 1;
        if !(coeff_idx >= 0 as c_int) {
            break;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "750:1"]
unsafe extern "C" fn x264_10_cabac_block_residual_c(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: c_int,
    mut l: *mut dctcoef,
) {
    cabac_block_residual_internal(h, cb, ctx_block_cat, l, 0 as c_int);
}
#[inline(always)]
#[c2rust::src_loc = "755:1"]
unsafe extern "C" fn cabac_block_residual(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: c_int,
    mut l: *mut dctcoef,
) {
    x264_10_cabac_block_residual_c(h, cb, ctx_block_cat, l);
}
#[c2rust::src_loc = "763:1"]
unsafe extern "C" fn cabac_block_residual_422_dc(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut _ctx_block_cat: c_int,
    mut l: *mut dctcoef,
) {
    cabac_block_residual_internal(h, cb, DCT_CHROMA_DC as c_int, l, 1 as c_int);
}
#[inline(always)]
#[c2rust::src_loc = "917:1"]
unsafe extern "C" fn macroblock_write_cabac_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut plane_count: c_int,
    mut chroma: c_int,
) {
    let i_mb_type: c_int = (*h).mb.i_type;
    let i_mb_pos_start: c_int = x264_cabac_pos(cb) as c_int;
    let mut i_mb_pos_tex: c_int = 0;
    if (*h).sh.b_mbaff != 0
        && ((*h).mb.i_mb_y & 1 as c_int == 0
            || (*(*h)
                .mb
                .type_0
                .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize) as c_int
                == P_SKIP as c_int
                || *(*h)
                    .mb
                    .type_0
                    .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                    as c_int
                    == B_SKIP as c_int))
    {
        cabac_field_decoding_flag(h, cb);
    }
    if (*h).sh.i_type == SLICE_TYPE_P as c_int {
        cabac_mb_header_p(h, cb, i_mb_type, chroma);
    } else if (*h).sh.i_type == SLICE_TYPE_B as c_int {
        cabac_mb_header_b(h, cb, i_mb_type, chroma);
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_I as c_int, chroma);
    }
    i_mb_pos_tex = x264_cabac_pos(cb);
    (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
    if i_mb_type == I_PCM as c_int {
        let mut s: bs_t = bs_s {
            p_start: 0 as *mut uint8_t,
            p: 0 as *mut uint8_t,
            p_end: 0 as *mut uint8_t,
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        bs_init(
            &mut s,
            (*cb).p as *mut c_void,
            (*cb).p_end.offset_from((*cb).p) as c_long as c_int,
        );
        let mut p: c_int = 0 as c_int;
        while p < plane_count {
            let mut i: c_int = 0 as c_int;
            while i < 256 as c_int {
                bs_write(
                    &mut s,
                    BIT_DEPTH,
                    *(*h).mb.pic.p_fenc[p as usize].offset(i as isize) as uint32_t,
                );
                i += 1;
            }
            p += 1;
        }
        if chroma != 0 {
            let mut ch: c_int = 1 as c_int;
            while ch < 3 as c_int {
                let mut i_0: c_int = 0 as c_int;
                while i_0 < 16 as c_int >> (*h).mb.chroma_v_shift {
                    let mut j: c_int = 0 as c_int;
                    while j < 8 as c_int {
                        bs_write(
                            &mut s,
                            BIT_DEPTH,
                            *(*h).mb.pic.p_fenc[ch as usize]
                                .offset((i_0 * FENC_STRIDE + j) as isize)
                                as uint32_t,
                        );
                        j += 1;
                    }
                    i_0 += 1;
                }
                ch += 1;
            }
        }
        bs_flush(&mut s);
        (*cb).p = s.p;
        x264_10_cabac_encode_init_core(cb);
        (*h).stat.frame.i_tex_bits += x264_cabac_pos(cb) - i_mb_pos_tex;
        return;
    }
    if i_mb_type != I_16x16 as c_int {
        cabac_cbp_luma(h, cb);
        if chroma != 0 {
            cabac_cbp_chroma(h, cb);
        }
    }
    if x264_mb_transform_8x8_allowed(h) != 0 && (*h).mb.i_cbp_luma != 0 {
        cabac_transform_size(h, cb);
    }
    if (*h).mb.i_cbp_luma != 0
        || chroma != 0 && (*h).mb.i_cbp_chroma != 0
        || i_mb_type == I_16x16 as c_int
    {
        let b_intra: c_int = (i_mb_type == I_4x4 as c_int
            || i_mb_type == I_8x8 as c_int
            || i_mb_type == I_16x16 as c_int
            || i_mb_type == I_PCM as c_int) as c_int;
        cabac_qp_delta(h, cb);
        if i_mb_type == I_16x16 as c_int {
            let mut p_0: c_int = 0 as c_int;
            while p_0 < plane_count {
                let mut ctxidxinc: c_int = cabac_cbf_ctxidxinc(
                    h,
                    ctx_cat_plane[DCT_LUMA_DC as c_int as usize][p_0 as usize] as c_int,
                    48 as c_int + p_0,
                    1 as c_int,
                    1 as c_int,
                );
                if (*h).mb.cache.non_zero_count[x264_scan8[(48 as c_int + p_0) as usize] as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc, 1 as c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        ctx_cat_plane[DCT_LUMA_DC as c_int as usize][p_0 as usize] as c_int,
                        (*(*h).dct.luma16x16_dc.as_mut_ptr().offset(p_0 as isize)).as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc, 0 as c_int);
                }
                if (*h).mb.i_cbp_luma != 0 {
                    let mut i_1: c_int = p_0 * 16 as c_int;
                    while i_1 < p_0 * 16 as c_int + 16 as c_int {
                        let mut ctxidxinc_0: c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_AC as c_int as usize][p_0 as usize] as c_int,
                            i_1,
                            1 as c_int,
                            0 as c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8[i_1 as usize] as usize] != 0 {
                            x264_10_cabac_encode_decision_c(cb, ctxidxinc_0, 1 as c_int);
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_AC as c_int as usize][p_0 as usize] as c_int,
                                (*(*h).dct.luma4x4.as_mut_ptr().offset(i_1 as isize))
                                    .as_mut_ptr()
                                    .offset(1),
                            );
                        } else {
                            x264_10_cabac_encode_decision_c(cb, ctxidxinc_0, 0 as c_int);
                        }
                        i_1 += 1;
                    }
                }
                p_0 += 1;
            }
        } else if (*h).mb.b_transform_8x8 != 0 {
            if plane_count == 3 as c_int {
                let mut nnzbak: [[uint8_t; 8]; 3] = [[0; 8]; 3];
                if (*h).mb.i_neighbour & MB_LEFT as c_int as c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_left_xy[0] as isize)
                        == 0
                    && *(*h).mb.cbp.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int
                        & 0x1000 as c_int
                        == 0
                {
                    nnzbak[0][0] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[0][1] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[1][0] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[1][1] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[2][0] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[2][1] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                }
                if (*h).mb.i_neighbour & MB_LEFT as c_int as c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_left_xy[1] as isize)
                        == 0
                    && *(*h).mb.cbp.offset((*h).mb.i_mb_left_xy[1] as isize) as c_int
                        & 0x1000 as c_int
                        == 0
                {
                    nnzbak[0][2] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[0][3] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[1][2] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[1][3] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[2][2] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                    nnzbak[2][3] = (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int)
                        as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = 0 as uint8_t;
                }
                if (*h).mb.i_neighbour & MB_TOP as c_int as c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_top_xy as isize)
                        == 0
                    && *(*h).mb.cbp.offset((*h).mb.i_mb_top_xy as isize) as c_int & 0x1000 as c_int
                        == 0
                {
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(4) as *mut uint8_t
                        as *mut x264_union32_t))
                        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 0 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 0 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as c_uint as uint32_t;
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(1))
                        .as_mut_ptr()
                        .offset(4) as *mut uint8_t
                        as *mut x264_union32_t))
                        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 1 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 1 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as c_uint as uint32_t;
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(2))
                        .as_mut_ptr()
                        .offset(4) as *mut uint8_t
                        as *mut x264_union32_t))
                        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 2 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 2 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as c_uint as uint32_t;
                }
                let mut p_1: c_int = 0 as c_int;
                while p_1 < 3 as c_int {
                    let mut i_2: c_int = 0 as c_int;
                    let mut msk: c_int = (*h).mb.i_cbp_luma;
                    let mut skip: c_int = 0;
                    while msk != 0 && {
                        skip = x264_ctz_4bit(msk as uint32_t);
                        i_2 += skip;
                        msk >>= skip + 1 as c_int;
                        1 as c_int != 0
                    } {
                        let mut ctxidxinc_1: c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_8x8 as c_int as usize][p_1 as usize] as c_int,
                            i_2 * 4 as c_int + p_1 * 16 as c_int,
                            b_intra,
                            0 as c_int,
                        );
                        if (*h).mb.cache.non_zero_count
                            [x264_scan8[(i_2 * 4 as c_int + p_1 * 16 as c_int) as usize] as usize]
                            != 0
                        {
                            x264_10_cabac_encode_decision_c(cb, ctxidxinc_1, 1 as c_int);
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_8x8 as c_int as usize][p_1 as usize]
                                    as c_int,
                                (*(*h)
                                    .dct
                                    .luma8x8
                                    .as_mut_ptr()
                                    .offset((i_2 + p_1 * 4 as c_int) as isize))
                                .as_mut_ptr(),
                            );
                        } else {
                            x264_10_cabac_encode_decision_c(cb, ctxidxinc_1, 0 as c_int);
                        }
                        i_2 += 1;
                    }
                    p_1 += 1;
                }
                if (*h).mb.i_neighbour & MB_LEFT as c_int as c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_left_xy[0] as isize)
                        == 0
                    && *(*h).mb.cbp.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int
                        & 0x1000 as c_int
                        == 0
                {
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[0][0];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[0][1];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[1][0];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[1][1];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 0 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[2][0];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 2 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[2][1];
                }
                if (*h).mb.i_neighbour & MB_LEFT as c_int as c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_left_xy[1] as isize)
                        == 0
                    && *(*h).mb.cbp.offset((*h).mb.i_mb_left_xy[1] as isize) as c_int
                        & 0x1000 as c_int
                        == 0
                {
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[0][2];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 0 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[0][3];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[1][2];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 1 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[1][3];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 8 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[2][2];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as c_int * 2 as c_int + 10 as c_int) as usize]
                        as c_int
                        - 1 as c_int) as usize] = nnzbak[2][3];
                }
                if (*h).mb.i_neighbour & MB_TOP as c_int as c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_top_xy as isize)
                        == 0
                    && *(*h).mb.cbp.offset((*h).mb.i_mb_top_xy as isize) as c_int & 0x1000 as c_int
                        == 0
                {
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 0 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*nnzbak.as_mut_ptr().offset(0)).as_mut_ptr().offset(4)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 1 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*nnzbak.as_mut_ptr().offset(1)).as_mut_ptr().offset(4)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as c_int * 2 as c_int) as isize)
                            as c_int
                            - 8 as c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*nnzbak.as_mut_ptr().offset(2)).as_mut_ptr().offset(4)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i;
                }
            } else {
                let mut i_3: c_int = 0 as c_int;
                let mut msk_0: c_int = (*h).mb.i_cbp_luma;
                let mut skip_0: c_int = 0;
                while msk_0 != 0 && {
                    skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                    i_3 += skip_0;
                    msk_0 >>= skip_0 + 1 as c_int;
                    1 as c_int != 0
                } {
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_LUMA_8x8 as c_int,
                        (*(*h).dct.luma8x8.as_mut_ptr().offset(i_3 as isize)).as_mut_ptr(),
                    );
                    i_3 += 1;
                }
            }
        } else {
            let mut p_2: c_int = 0 as c_int;
            while p_2 < plane_count {
                let mut i8x8: c_int = 0 as c_int;
                let mut msk_1: c_int = (*h).mb.i_cbp_luma;
                let mut skip_1: c_int = 0;
                while msk_1 != 0 && {
                    skip_1 = x264_ctz_4bit(msk_1 as uint32_t);
                    i8x8 += skip_1;
                    msk_1 >>= skip_1 + 1 as c_int;
                    1 as c_int != 0
                } {
                    let mut i_4: c_int = 0 as c_int;
                    while i_4 < 4 as c_int {
                        let mut ctxidxinc_2: c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_4x4 as c_int as usize][p_2 as usize] as c_int,
                            i_4 + i8x8 * 4 as c_int + p_2 * 16 as c_int,
                            b_intra,
                            0 as c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8
                            [(i_4 + i8x8 * 4 as c_int + p_2 * 16 as c_int) as usize]
                            as usize]
                            != 0
                        {
                            x264_10_cabac_encode_decision_c(cb, ctxidxinc_2, 1 as c_int);
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_4x4 as c_int as usize][p_2 as usize]
                                    as c_int,
                                (*(*h).dct.luma4x4.as_mut_ptr().offset(
                                    (i_4 + i8x8 * 4 as c_int + p_2 * 16 as c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            );
                        } else {
                            x264_10_cabac_encode_decision_c(cb, ctxidxinc_2, 0 as c_int);
                        }
                        i_4 += 1;
                    }
                    i8x8 += 1;
                }
                p_2 += 1;
            }
        }
        if chroma != 0 && (*h).mb.i_cbp_chroma != 0 {
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as c_int {
                let mut ctxidxinc_3: c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as c_int,
                    49 as c_int + 0 as c_int,
                    b_intra,
                    1 as c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(49 as c_int + 0 as c_int) as usize] as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_3, 1 as c_int);
                    cabac_block_residual_422_dc(
                        h,
                        cb,
                        DCT_CHROMA_DC as c_int,
                        (*(*h).dct.chroma_dc.as_mut_ptr().offset(0)).as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_3, 0 as c_int);
                }
                let mut ctxidxinc_4: c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as c_int,
                    49 as c_int + 1 as c_int,
                    b_intra,
                    1 as c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(49 as c_int + 1 as c_int) as usize] as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_4, 1 as c_int);
                    cabac_block_residual_422_dc(
                        h,
                        cb,
                        DCT_CHROMA_DC as c_int,
                        (*(*h).dct.chroma_dc.as_mut_ptr().offset(1)).as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_4, 0 as c_int);
                }
            } else {
                let mut ctxidxinc_5: c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as c_int,
                    49 as c_int + 0 as c_int,
                    b_intra,
                    1 as c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(49 as c_int + 0 as c_int) as usize] as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_5, 1 as c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_CHROMA_DC as c_int,
                        (*(*h).dct.chroma_dc.as_mut_ptr().offset(0)).as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_5, 0 as c_int);
                }
                let mut ctxidxinc_6: c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as c_int,
                    49 as c_int + 1 as c_int,
                    b_intra,
                    1 as c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(49 as c_int + 1 as c_int) as usize] as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_6, 1 as c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_CHROMA_DC as c_int,
                        (*(*h).dct.chroma_dc.as_mut_ptr().offset(1)).as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_6, 0 as c_int);
                }
            }
            if (*h).mb.i_cbp_chroma == 2 as c_int {
                let mut step: c_int = (8 as c_int) << (*h).mb.chroma_v_shift;
                let mut i_5: c_int = 16 as c_int;
                while i_5 < 3 as c_int * 16 as c_int {
                    let mut j_0: c_int = i_5;
                    while j_0 < i_5 + 4 as c_int {
                        let mut ctxidxinc_7: c_int = cabac_cbf_ctxidxinc(
                            h,
                            DCT_CHROMA_AC as c_int,
                            j_0,
                            b_intra,
                            0 as c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8[j_0 as usize] as usize] != 0 {
                            x264_10_cabac_encode_decision_c(cb, ctxidxinc_7, 1 as c_int);
                            cabac_block_residual(
                                h,
                                cb,
                                DCT_CHROMA_AC as c_int,
                                (*(*h).dct.luma4x4.as_mut_ptr().offset(j_0 as isize))
                                    .as_mut_ptr()
                                    .offset(1),
                            );
                        } else {
                            x264_10_cabac_encode_decision_c(cb, ctxidxinc_7, 0 as c_int);
                        }
                        j_0 += 1;
                    }
                    i_5 += step;
                }
            }
        }
    }
    (*h).stat.frame.i_tex_bits += x264_cabac_pos(cb) - i_mb_pos_tex;
}
#[no_mangle]
#[c2rust::src_loc = "1088:1"]
unsafe extern "C" fn x264_10_macroblock_write_cabac(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
        macroblock_write_cabac_internal(h, cb, 3 as c_int, 0 as c_int);
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        macroblock_write_cabac_internal(h, cb, 1 as c_int, 1 as c_int);
    } else {
        macroblock_write_cabac_internal(h, cb, 1 as c_int, 0 as c_int);
    };
}
