use core::ffi::{c_int, c_uint};

use crate::base_h::{
    x264_clip3, x264_median_mv, x264_scan8, x264_union16_t, x264_union32_t, x264_union64_t,
    SLICE_TYPE_B, X264_SCAN8_0,
};
use crate::common_h::x264_t;
use crate::frame_h::x264_frame_t;
use crate::macroblock_h::{
    pack16to32_mask, D_16x16, D_16x8, D_8x16, D_8x8, I_16x16, I_4x4, I_8x8, I_PCM,
};
use crate::rectangle_h::{x264_macroblock_cache_mv, x264_macroblock_cache_ref};
use crate::stdint_h::{INT16_MAX, INT16_MIN};
use crate::stdint_intn_h::{int16_t, int8_t};
use crate::stdint_uintn_h::uint32_t;
use crate::stdlib_h::abs;
use crate::x264_h::X264_DIRECT_PRED_NONE;
#[no_mangle]
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn x264_10_mb_predict_mv(
    mut h: *mut x264_t,
    mut i_list: c_int,
    mut idx: c_int,
    mut i_width: c_int,
    mut mvp: *mut int16_t,
) {
    let i8: c_int = x264_scan8[idx as usize] as c_int;
    let i_ref: c_int = (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as c_int;
    let mut i_refa: c_int =
        (*h).mb.cache.ref_0[i_list as usize][(i8 - 1 as c_int) as usize] as c_int;
    let mut mv_a: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((i8 - 1 as c_int) as isize))
    .as_mut_ptr();
    let mut i_refb: c_int =
        (*h).mb.cache.ref_0[i_list as usize][(i8 - 8 as c_int) as usize] as c_int;
    let mut mv_b: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((i8 - 8 as c_int) as isize))
    .as_mut_ptr();
    let mut i_refc: c_int =
        (*h).mb.cache.ref_0[i_list as usize][(i8 - 8 as c_int + i_width) as usize] as c_int;
    let mut mv_c: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((i8 - 8 as c_int + i_width) as isize))
    .as_mut_ptr();
    if idx & 3 as c_int >= 2 as c_int + (i_width & 1 as c_int) || i_refc == -(2 as c_int) {
        i_refc =
            (*h).mb.cache.ref_0[i_list as usize][(i8 - 8 as c_int - 1 as c_int) as usize] as c_int;
        mv_c = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((i8 - 8 as c_int - 1 as c_int) as isize))
        .as_mut_ptr();
        if (*h).sh.b_mbaff != 0
            && (*h).mb.cache.ref_0[i_list as usize][(x264_scan8[0] as c_int - 1 as c_int) as usize]
                as c_int
                != -(2 as c_int)
            && (*h).mb.b_interlaced
                != *(*h).mb.field.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int
        {
            if idx == 2 as c_int {
                mv_c = (*(*(*h)
                    .mb
                    .cache
                    .topright_mv
                    .as_mut_ptr()
                    .offset(i_list as isize))
                .as_mut_ptr()
                .offset(0))
                .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize][0] as c_int;
            } else if idx == 8 as c_int {
                mv_c = (*(*(*h)
                    .mb
                    .cache
                    .topright_mv
                    .as_mut_ptr()
                    .offset(i_list as isize))
                .as_mut_ptr()
                .offset(1))
                .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize][1] as c_int;
            } else if idx == 10 as c_int {
                mv_c = (*(*(*h)
                    .mb
                    .cache
                    .topright_mv
                    .as_mut_ptr()
                    .offset(i_list as isize))
                .as_mut_ptr()
                .offset(2))
                .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize][2] as c_int;
            }
        }
    }
    if (*h).mb.i_partition == D_16x8 as c_int {
        if idx == 0 as c_int {
            if i_refb == i_ref {
                (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
                return;
            }
        } else if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
            return;
        }
    } else if (*h).mb.i_partition == D_8x16 as c_int {
        if idx == 0 as c_int {
            if i_refa == i_ref {
                (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
                return;
            }
        } else if i_refc == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
            return;
        }
    }
    let mut i_count: c_int =
        (i_refa == i_ref) as c_int + (i_refb == i_ref) as c_int + (i_refc == i_ref) as c_int;
    let mut current_block_51: u64;
    if i_count > 1 as c_int {
        current_block_51 = 12053149342108387725;
    } else if i_count == 1 as c_int {
        if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        } else if i_refb == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
        } else {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
        }
        current_block_51 = 10150597327160359210;
    } else if i_refb == -(2 as c_int) && i_refc == -(2 as c_int) && i_refa != -(2 as c_int) {
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
unsafe extern "C" fn x264_10_mb_predict_mv_16x16(
    mut h: *mut x264_t,
    mut i_list: c_int,
    mut i_ref: c_int,
    mut mvp: *mut int16_t,
) {
    let mut i_refa: c_int =
        (*h).mb.cache.ref_0[i_list as usize][(X264_SCAN8_0 - 1 as c_int) as usize] as c_int;
    let mut mv_a: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((X264_SCAN8_0 - 1 as c_int) as isize))
    .as_mut_ptr();
    let mut i_refb: c_int =
        (*h).mb.cache.ref_0[i_list as usize][(X264_SCAN8_0 - 8 as c_int) as usize] as c_int;
    let mut mv_b: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((X264_SCAN8_0 - 8 as c_int) as isize))
    .as_mut_ptr();
    let mut i_refc: c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(X264_SCAN8_0 - 8 as c_int + 4 as c_int) as usize] as c_int;
    let mut mv_c: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((X264_SCAN8_0 - 8 as c_int + 4 as c_int) as isize))
    .as_mut_ptr();
    if i_refc == -(2 as c_int) {
        i_refc = (*h).mb.cache.ref_0[i_list as usize]
            [(X264_SCAN8_0 - 8 as c_int - 1 as c_int) as usize] as c_int;
        mv_c = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((X264_SCAN8_0 - 8 as c_int - 1 as c_int) as isize))
        .as_mut_ptr();
    }
    let mut i_count: c_int =
        (i_refa == i_ref) as c_int + (i_refb == i_ref) as c_int + (i_refc == i_ref) as c_int;
    let mut current_block_11: u64;
    if i_count > 1 as c_int {
        current_block_11 = 2747653332422150726;
    } else if i_count == 1 as c_int {
        if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        } else if i_refb == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
        } else {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
        }
        current_block_11 = 13056961889198038528;
    } else if i_refb == -(2 as c_int) && i_refc == -(2 as c_int) && i_refa != -(2 as c_int) {
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
unsafe extern "C" fn x264_10_mb_predict_mv_pskip(mut h: *mut x264_t, mut mv: *mut int16_t) {
    let mut i_refa: c_int = (*h).mb.cache.ref_0[0][(X264_SCAN8_0 - 1 as c_int) as usize] as c_int;
    let mut i_refb: c_int = (*h).mb.cache.ref_0[0][(X264_SCAN8_0 - 8 as c_int) as usize] as c_int;
    let mut mv_a: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(0))
        .as_mut_ptr()
        .offset((X264_SCAN8_0 - 1 as c_int) as isize))
    .as_mut_ptr();
    let mut mv_b: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(0))
        .as_mut_ptr()
        .offset((X264_SCAN8_0 - 8 as c_int) as isize))
    .as_mut_ptr();
    if i_refa == -(2 as c_int)
        || i_refb == -(2 as c_int)
        || i_refa as uint32_t | (*(mv_a as *mut x264_union32_t)).i == 0
        || i_refb as uint32_t | (*(mv_b as *mut x264_union32_t)).i == 0
    {
        (*(mv as *mut x264_union32_t)).i = 0 as uint32_t;
    } else {
        x264_10_mb_predict_mv_16x16(h, 0 as c_int, 0 as c_int, mv);
    };
}
#[c2rust::src_loc = "183:1"]
unsafe extern "C" fn mb_predict_mv_direct16x16_temporal(mut h: *mut x264_t) -> c_int {
    let mut mb_x: c_int = (*h).mb.i_mb_x;
    let mut mb_y: c_int = (*h).mb.i_mb_y;
    let mut mb_xy: c_int = (*h).mb.i_mb_xy;
    let mut type_col: [c_int; 2] = [
        *(*(*h).fref[1][0]).mb_type.offset(mb_xy as isize) as c_int,
        *(*(*h).fref[1][0]).mb_type.offset(mb_xy as isize) as c_int,
    ];
    let mut partition_col: [c_int; 2] = [
        *(*(*h).fref[1][0]).mb_partition.offset(mb_xy as isize) as c_int,
        *(*(*h).fref[1][0]).mb_partition.offset(mb_xy as isize) as c_int,
    ];
    let mut preshift: c_int = (*h).mb.b_interlaced;
    let mut postshift: c_int = (*h).mb.b_interlaced;
    let mut offset: c_int = 1 as c_int;
    let mut yshift: c_int = 1 as c_int;
    (*h).mb.i_partition = partition_col[0];
    if (*h).param.b_interlaced != 0
        && *(*(*h).fref[1][0]).field.offset(mb_xy as isize) as c_int != (*h).mb.b_interlaced
    {
        if (*h).mb.b_interlaced != 0 {
            mb_y = (*h).mb.i_mb_y & !(1 as c_int);
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[0] = *(*(*h).fref[1][0]).mb_type.offset(mb_xy as isize) as c_int;
            type_col[1] = *(*(*h).fref[1][0])
                .mb_type
                .offset((mb_xy + (*h).mb.i_mb_stride) as isize) as c_int;
            partition_col[0] = *(*(*h).fref[1][0]).mb_partition.offset(mb_xy as isize) as c_int;
            partition_col[1] = *(*(*h).fref[1][0])
                .mb_partition
                .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                as c_int;
            preshift = 0 as c_int;
            yshift = 0 as c_int;
            if (type_col[0] == I_4x4 as c_int
                || type_col[0] == I_8x8 as c_int
                || type_col[0] == I_16x16 as c_int
                || type_col[0] == I_PCM as c_int
                || partition_col[0] == D_16x16 as c_int)
                && (type_col[1] == I_4x4 as c_int
                    || type_col[1] == I_8x8 as c_int
                    || type_col[1] == I_16x16 as c_int
                    || type_col[1] == I_PCM as c_int
                    || partition_col[1] == D_16x16 as c_int)
                && partition_col[0] != D_8x8 as c_int
            {
                (*h).mb.i_partition = D_16x8 as c_int;
            } else {
                (*h).mb.i_partition = D_8x8 as c_int;
            }
        } else {
            let mut cur_poc: c_int = (*(*h).fdec).i_poc
                + (*(*h).fdec).i_delta_poc
                    [((*h).mb.b_interlaced & (*h).mb.i_mb_y & 1 as c_int) as usize];
            let mut col_parity: c_int =
                (abs((*(*h).fref[1][0]).i_poc + (*(*h).fref[1][0]).i_delta_poc[0] - cur_poc)
                    >= abs((*(*h).fref[1][0]).i_poc + (*(*h).fref[1][0]).i_delta_poc[1] - cur_poc))
                    as c_int;
            mb_y = ((*h).mb.i_mb_y & !(1 as c_int)) + col_parity;
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[1] = *(*(*h).fref[1][0]).mb_type.offset(mb_xy as isize) as c_int;
            type_col[0] = type_col[1];
            partition_col[1] = *(*(*h).fref[1][0]).mb_partition.offset(mb_xy as isize) as c_int;
            partition_col[0] = partition_col[1];
            preshift = 1 as c_int;
            yshift = 2 as c_int;
            (*h).mb.i_partition = partition_col[0];
        }
        offset = 0 as c_int;
    }
    let mut i_mb_4x4: c_int = 16 as c_int * (*h).mb.i_mb_stride * mb_y + 4 as c_int * mb_x;
    let mut i_mb_8x8: c_int = 4 as c_int * (*h).mb.i_mb_stride * mb_y + 2 as c_int * mb_x;
    x264_macroblock_cache_ref(
        h,
        0 as c_int,
        0 as c_int,
        4 as c_int,
        4 as c_int,
        1 as c_int,
        0 as int8_t,
    );
    let mut max_i8: c_int = D_16x16 as c_int - (*h).mb.i_partition + 1 as c_int;
    let mut step: c_int = ((*h).mb.i_partition == D_16x8 as c_int) as c_int + 1 as c_int;
    let mut width: c_int = 4 as c_int >> (D_16x16 as c_int - (*h).mb.i_partition & 1 as c_int);
    let mut height: c_int = 4 as c_int >> (D_16x16 as c_int - (*h).mb.i_partition >> 1 as c_int);
    let mut i8: c_int = 0 as c_int;
    while i8 < max_i8 {
        let mut x8: c_int = i8 & 1 as c_int;
        let mut y8: c_int = i8 >> 1 as c_int;
        let mut ypart: c_int = if (*h).sh.b_mbaff != 0
            && *(*(*h).fref[1][0]).field.offset(mb_xy as isize) as c_int != (*h).mb.b_interlaced
        {
            if (*h).mb.b_interlaced != 0 {
                y8 * 6 as c_int
            } else {
                2 as c_int * ((*h).mb.i_mb_y & 1 as c_int) + y8
            }
        } else {
            3 as c_int * y8
        };
        if type_col[y8 as usize] == I_4x4 as c_int
            || type_col[y8 as usize] == I_8x8 as c_int
            || type_col[y8 as usize] == I_16x16 as c_int
            || type_col[y8 as usize] == I_PCM as c_int
        {
            x264_macroblock_cache_ref(
                h,
                2 as c_int * x8,
                2 as c_int * y8,
                width,
                height,
                0 as c_int,
                0 as int8_t,
            );
            x264_macroblock_cache_mv(
                h,
                2 as c_int * x8,
                2 as c_int * y8,
                width,
                height,
                0 as c_int,
                0 as uint32_t,
            );
            x264_macroblock_cache_mv(
                h,
                2 as c_int * x8,
                2 as c_int * y8,
                width,
                height,
                1 as c_int,
                0 as uint32_t,
            );
        } else {
            let mut i_part_8x8: c_int = i_mb_8x8 + x8 + (ypart >> 1 as c_int) * (*h).mb.i_b8_stride;
            let mut i_ref1_ref: c_int =
                *(*(*h).fref[1][0]).ref_0[0].offset(i_part_8x8 as isize) as c_int;
            let mut i_ref: c_int =
                (*h).mb.map_col_to_list0[((i_ref1_ref >> preshift) + 2 as c_int) as usize] as c_int
                    * ((1 as c_int) << postshift)
                    + (offset & i_ref1_ref & (*h).mb.b_interlaced);
            if i_ref >= 0 as c_int {
                let mut dist_scale_factor: c_int =
                    (*(*h).mb.dist_scale_factor.offset(i_ref as isize))[0] as c_int;
                let mut mv_col: *mut int16_t =
                    (*(*(**(*(*h).fref.as_mut_ptr().offset(1)).as_mut_ptr().offset(0))
                        .mv
                        .as_mut_ptr()
                        .offset(0))
                    .offset((i_mb_4x4 + 3 as c_int * x8 + ypart * (*h).mb.i_b4_stride) as isize))
                    .as_mut_ptr();
                let mut mv_y: int16_t =
                    (*mv_col.offset(1) as c_int * ((1 as c_int) << yshift) / 2 as c_int) as int16_t;
                let mut l0x: c_int =
                    dist_scale_factor * *mv_col.offset(0) as c_int + 128 as c_int >> 8 as c_int;
                let mut l0y: c_int = dist_scale_factor * mv_y as c_int + 128 as c_int >> 8 as c_int;
                if (*h).param.i_threads > 1 as c_int
                    && (l0y > (*h).mb.mv_max_spel[1]
                        || l0y - mv_y as c_int > (*h).mb.mv_max_spel[1])
                {
                    return 0 as c_int;
                }
                x264_macroblock_cache_ref(
                    h,
                    2 as c_int * x8,
                    2 as c_int * y8,
                    width,
                    height,
                    0 as c_int,
                    i_ref as int8_t,
                );
                x264_macroblock_cache_mv(
                    h,
                    2 as c_int * x8,
                    2 as c_int * y8,
                    width,
                    height,
                    0 as c_int,
                    pack16to32_mask(l0x, l0y),
                );
                x264_macroblock_cache_mv(
                    h,
                    2 as c_int * x8,
                    2 as c_int * y8,
                    width,
                    height,
                    1 as c_int,
                    pack16to32_mask(l0x - *mv_col.offset(0) as c_int, l0y - mv_y as c_int),
                );
            } else {
                return 0 as c_int;
            }
        }
        i8 += step;
    }
    return 1 as c_int;
}
#[inline(always)]
#[c2rust::src_loc = "289:1"]
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial(
    mut h: *mut x264_t,
    mut b_interlaced: c_int,
) -> c_int {
    let mut ref_0: [int8_t; 2] = [0; 2];
    let mut mv: [[int16_t; 2]; 2] = [[0; 2]; 2];
    let mut i_list: c_int = 0 as c_int;
    while i_list < 2 as c_int {
        let mut i_refa: c_int =
            (*h).mb.cache.ref_0[i_list as usize][(X264_SCAN8_0 - 1 as c_int) as usize] as c_int;
        let mut mv_a: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((X264_SCAN8_0 - 1 as c_int) as isize))
        .as_mut_ptr();
        let mut i_refb: c_int =
            (*h).mb.cache.ref_0[i_list as usize][(X264_SCAN8_0 - 8 as c_int) as usize] as c_int;
        let mut mv_b: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((X264_SCAN8_0 - 8 as c_int) as isize))
        .as_mut_ptr();
        let mut i_refc: c_int = (*h).mb.cache.ref_0[i_list as usize]
            [(X264_SCAN8_0 - 8 as c_int + 4 as c_int) as usize]
            as c_int;
        let mut mv_c: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((X264_SCAN8_0 - 8 as c_int + 4 as c_int) as isize))
        .as_mut_ptr();
        if i_refc == -(2 as c_int) {
            i_refc = (*h).mb.cache.ref_0[i_list as usize]
                [(X264_SCAN8_0 - 8 as c_int - 1 as c_int) as usize] as c_int;
            mv_c = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((X264_SCAN8_0 - 8 as c_int - 1 as c_int) as isize))
            .as_mut_ptr();
        }
        let mut i_ref: c_int = (if (i_refa as c_uint)
            < (if (i_refb as c_uint) < i_refc as c_uint {
                i_refb as c_uint
            } else {
                i_refc as c_uint
            }) {
            i_refa as c_uint
        } else if (i_refb as c_uint) < i_refc as c_uint {
            i_refb as c_uint
        } else {
            i_refc as c_uint
        }) as c_int;
        if i_ref < 0 as c_int {
            i_ref = -1;
            (*((*mv.as_mut_ptr().offset(i_list as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
                0 as uint32_t;
        } else {
            let mut i_count: c_int = (i_refa == i_ref) as c_int
                + (i_refb == i_ref) as c_int
                + (i_refc == i_ref) as c_int;
            if i_count > 1 as c_int {
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
            0 as c_int,
            0 as c_int,
            4 as c_int,
            4 as c_int,
            i_list,
            i_ref as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            0 as c_int,
            0 as c_int,
            4 as c_int,
            4 as c_int,
            i_list,
            (*((*mv.as_mut_ptr().offset(i_list as isize)).as_mut_ptr() as *mut x264_union32_t)).i,
        );
        ref_0[i_list as usize] = i_ref as int8_t;
        i_list += 1;
    }
    let mut mb_x: c_int = (*h).mb.i_mb_x;
    let mut mb_y: c_int = (*h).mb.i_mb_y;
    let mut mb_xy: c_int = (*h).mb.i_mb_xy;
    let mut type_col: [c_int; 2] = [
        *(*(*h).fref[1][0]).mb_type.offset(mb_xy as isize) as c_int,
        *(*(*h).fref[1][0]).mb_type.offset(mb_xy as isize) as c_int,
    ];
    let mut partition_col: [c_int; 2] = [
        *(*(*h).fref[1][0]).mb_partition.offset(mb_xy as isize) as c_int,
        *(*(*h).fref[1][0]).mb_partition.offset(mb_xy as isize) as c_int,
    ];
    (*h).mb.i_partition = partition_col[0];
    if b_interlaced != 0
        && *(*(*h).fref[1][0]).field.offset(mb_xy as isize) as c_int != (*h).mb.b_interlaced
    {
        if (*h).mb.b_interlaced != 0 {
            mb_y = (*h).mb.i_mb_y & !(1 as c_int);
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[0] = *(*(*h).fref[1][0]).mb_type.offset(mb_xy as isize) as c_int;
            type_col[1] = *(*(*h).fref[1][0])
                .mb_type
                .offset((mb_xy + (*h).mb.i_mb_stride) as isize) as c_int;
            partition_col[0] = *(*(*h).fref[1][0]).mb_partition.offset(mb_xy as isize) as c_int;
            partition_col[1] = *(*(*h).fref[1][0])
                .mb_partition
                .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                as c_int;
            if (type_col[0] == I_4x4 as c_int
                || type_col[0] == I_8x8 as c_int
                || type_col[0] == I_16x16 as c_int
                || type_col[0] == I_PCM as c_int
                || partition_col[0] == D_16x16 as c_int)
                && (type_col[1] == I_4x4 as c_int
                    || type_col[1] == I_8x8 as c_int
                    || type_col[1] == I_16x16 as c_int
                    || type_col[1] == I_PCM as c_int
                    || partition_col[1] == D_16x16 as c_int)
                && partition_col[0] != D_8x8 as c_int
            {
                (*h).mb.i_partition = D_16x8 as c_int;
            } else {
                (*h).mb.i_partition = D_8x8 as c_int;
            }
        } else {
            let mut cur_poc: c_int = (*(*h).fdec).i_poc
                + (*(*h).fdec).i_delta_poc
                    [((*h).mb.b_interlaced & (*h).mb.i_mb_y & 1 as c_int) as usize];
            let mut col_parity: c_int =
                (abs((*(*h).fref[1][0]).i_poc + (*(*h).fref[1][0]).i_delta_poc[0] - cur_poc)
                    >= abs((*(*h).fref[1][0]).i_poc + (*(*h).fref[1][0]).i_delta_poc[1] - cur_poc))
                    as c_int;
            mb_y = ((*h).mb.i_mb_y & !(1 as c_int)) + col_parity;
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[1] = *(*(*h).fref[1][0]).mb_type.offset(mb_xy as isize) as c_int;
            type_col[0] = type_col[1];
            partition_col[1] = *(*(*h).fref[1][0]).mb_partition.offset(mb_xy as isize) as c_int;
            partition_col[0] = partition_col[1];
            (*h).mb.i_partition = partition_col[0];
        }
    }
    let mut i_mb_4x4: c_int = if b_interlaced != 0 {
        4 as c_int * ((*h).mb.i_b4_stride * mb_y + mb_x)
    } else {
        (*h).mb.i_b4_xy
    };
    let mut i_mb_8x8: c_int = if b_interlaced != 0 {
        2 as c_int * ((*h).mb.i_b8_stride * mb_y + mb_x)
    } else {
        (*h).mb.i_b8_xy
    };
    let mut l1ref0: *mut int8_t =
        &mut *(*(**(*(*h).fref.as_mut_ptr().offset(1)).as_mut_ptr().offset(0))
            .ref_0
            .as_mut_ptr()
            .offset(0))
        .offset(i_mb_8x8 as isize) as *mut int8_t;
    let mut l1ref1: *mut int8_t =
        &mut *(*(**(*(*h).fref.as_mut_ptr().offset(1)).as_mut_ptr().offset(0))
            .ref_0
            .as_mut_ptr()
            .offset(1))
        .offset(i_mb_8x8 as isize) as *mut int8_t;
    let mut l1mv: [*mut [int16_t; 2]; 2] = [
        &mut *(*(**(*(*h).fref.as_mut_ptr().offset(1)).as_mut_ptr().offset(0))
            .mv
            .as_mut_ptr()
            .offset(0))
        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2],
        &mut *(*(**(*(*h).fref.as_mut_ptr().offset(1)).as_mut_ptr().offset(0))
            .mv
            .as_mut_ptr()
            .offset(1))
        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2],
    ];
    if (*(ref_0.as_mut_ptr() as *mut x264_union16_t)).i as c_int & 0x8080 as c_int
        == 0x8080 as c_int
    {
        x264_macroblock_cache_ref(
            h,
            0 as c_int,
            0 as c_int,
            4 as c_int,
            4 as c_int,
            0 as c_int,
            0 as int8_t,
        );
        x264_macroblock_cache_ref(
            h,
            0 as c_int,
            0 as c_int,
            4 as c_int,
            4 as c_int,
            1 as c_int,
            0 as int8_t,
        );
        return 1 as c_int;
    }
    if (*h).param.i_threads > 1 as c_int
        && (mv[0][1] as c_int > (*h).mb.mv_max_spel[1]
            || mv[1][1] as c_int > (*h).mb.mv_max_spel[1])
    {
        return 0 as c_int;
    }
    if (*(mv.as_mut_ptr() as *mut x264_union64_t)).i == 0
        || b_interlaced == 0
            && (type_col[0] == I_4x4 as c_int
                || type_col[0] == I_8x8 as c_int
                || type_col[0] == I_16x16 as c_int
                || type_col[0] == I_PCM as c_int)
        || ref_0[0] as c_int != 0 && ref_0[1] as c_int != 0
    {
        return 1 as c_int;
    }
    let mut max_i8: c_int = D_16x16 as c_int - (*h).mb.i_partition + 1 as c_int;
    let mut step: c_int = ((*h).mb.i_partition == D_16x8 as c_int) as c_int + 1 as c_int;
    let mut width: c_int = 4 as c_int >> (D_16x16 as c_int - (*h).mb.i_partition & 1 as c_int);
    let mut height: c_int = 4 as c_int >> (D_16x16 as c_int - (*h).mb.i_partition >> 1 as c_int);
    let mut current_block_59: u64;
    let mut i8: c_int = 0 as c_int;
    while i8 < max_i8 {
        let x8: c_int = i8 & 1 as c_int;
        let y8: c_int = i8 >> 1 as c_int;
        let mut ypart: c_int = if b_interlaced != 0
            && *(*(*h).fref[1][0]).field.offset(mb_xy as isize) as c_int != (*h).mb.b_interlaced
        {
            if (*h).mb.b_interlaced != 0 {
                y8 * 6 as c_int
            } else {
                2 as c_int * ((*h).mb.i_mb_y & 1 as c_int) + y8
            }
        } else {
            3 as c_int * y8
        };
        let mut o8: c_int = x8 + (ypart >> 1 as c_int) * (*h).mb.i_b8_stride;
        let mut o4: c_int = 3 as c_int * x8 + ypart * (*h).mb.i_b4_stride;
        if !(b_interlaced != 0
            && (type_col[y8 as usize] == I_4x4 as c_int
                || type_col[y8 as usize] == I_8x8 as c_int
                || type_col[y8 as usize] == I_16x16 as c_int
                || type_col[y8 as usize] == I_PCM as c_int))
        {
            let mut idx: c_int = 0;
            if *l1ref0.offset(o8 as isize) as c_int == 0 as c_int {
                idx = 0 as c_int;
                current_block_59 = 6528285054092551010;
            } else if (*l1ref0.offset(o8 as isize) as c_int) < 0 as c_int
                && *l1ref1.offset(o8 as isize) as c_int == 0 as c_int
            {
                idx = 1 as c_int;
                current_block_59 = 6528285054092551010;
            } else {
                current_block_59 = 1423531122933789233;
            }
            match current_block_59 {
                1423531122933789233 => {}
                _ => {
                    if abs((*l1mv[idx as usize].offset(o4 as isize))[0] as c_int) <= 1 as c_int
                        && abs((*l1mv[idx as usize].offset(o4 as isize))[1] as c_int) <= 1 as c_int
                    {
                        if ref_0[0] as c_int == 0 as c_int {
                            x264_macroblock_cache_mv(
                                h,
                                2 as c_int * x8,
                                2 as c_int * y8,
                                width,
                                height,
                                0 as c_int,
                                0 as uint32_t,
                            );
                        }
                        if ref_0[1] as c_int == 0 as c_int {
                            x264_macroblock_cache_mv(
                                h,
                                2 as c_int * x8,
                                2 as c_int * y8,
                                width,
                                height,
                                1 as c_int,
                                0 as uint32_t,
                            );
                        }
                    }
                }
            }
        }
        i8 += step;
    }
    return 1 as c_int;
}
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial_interlaced(mut h: *mut x264_t) -> c_int {
    return mb_predict_mv_direct16x16_spatial(h, 1 as c_int);
}
#[c2rust::src_loc = "449:1"]
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial_progressive(mut h: *mut x264_t) -> c_int {
    return mb_predict_mv_direct16x16_spatial(h, 0 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "454:1"]
unsafe extern "C" fn x264_10_mb_predict_mv_direct16x16(
    mut h: *mut x264_t,
    mut b_changed: *mut c_int,
) -> c_int {
    let mut b_available: c_int = 0;
    if (*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_NONE {
        return 0 as c_int;
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
        let mut changed: c_int = 0;
        changed = ((*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset(0))
        .as_mut_ptr() as *mut x264_union32_t))
            .i
            ^ (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i) as c_int;
        changed |= ((*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(1))
            .as_mut_ptr()
            .offset(0))
        .as_mut_ptr() as *mut x264_union32_t))
            .i
            ^ (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(1))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i) as c_int;
        changed |= (*h).mb.cache.direct_ref[0][0] as c_int
            ^ (*h).mb.cache.ref_0[0][x264_scan8[0] as usize] as c_int;
        changed |= (*h).mb.cache.direct_ref[1][0] as c_int
            ^ (*h).mb.cache.ref_0[1][x264_scan8[0] as usize] as c_int;
        if changed == 0 && (*h).mb.i_partition != D_16x16 as c_int {
            changed |= ((*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(3))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(12 as c_int as isize) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as c_int;
            changed |= ((*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(1))
                .as_mut_ptr()
                .offset(3))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(12 as c_int as isize) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as c_int;
            changed |= (*h).mb.cache.direct_ref[0][3] as c_int
                ^ (*h).mb.cache.ref_0[0][x264_scan8[12] as usize] as c_int;
            changed |= (*h).mb.cache.direct_ref[1][3] as c_int
                ^ (*h).mb.cache.ref_0[1][x264_scan8[12] as usize] as c_int;
        }
        if changed == 0 && (*h).mb.i_partition == D_8x8 as c_int {
            changed |= ((*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(1))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(4) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as c_int;
            changed |= ((*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(1))
                .as_mut_ptr()
                .offset(1))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(4) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as c_int;
            changed |= ((*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(0))
                .as_mut_ptr()
                .offset(2))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(8) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as c_int;
            changed |= ((*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(1))
                .as_mut_ptr()
                .offset(2))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(8) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as c_int;
            changed |= (*h).mb.cache.direct_ref[0][1] as c_int
                ^ (*h).mb.cache.ref_0[0][x264_scan8[4] as usize] as c_int;
            changed |= (*h).mb.cache.direct_ref[1][1] as c_int
                ^ (*h).mb.cache.ref_0[1][x264_scan8[4] as usize] as c_int;
            changed |= (*h).mb.cache.direct_ref[0][2] as c_int
                ^ (*h).mb.cache.ref_0[0][x264_scan8[8] as usize] as c_int;
            changed |= (*h).mb.cache.direct_ref[1][2] as c_int
                ^ (*h).mb.cache.ref_0[1][x264_scan8[8] as usize] as c_int;
        }
        *b_changed = changed;
        if changed == 0 {
            return b_available;
        }
    }
    if b_available != 0 {
        let mut l: c_int = 0 as c_int;
        while l < 2 as c_int {
            (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(0))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(1))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(4) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(2))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(3))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(12 as c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*h).mb.cache.direct_ref[l as usize][0] =
                (*h).mb.cache.ref_0[l as usize][x264_scan8[0] as usize];
            (*h).mb.cache.direct_ref[l as usize][1] =
                (*h).mb.cache.ref_0[l as usize][x264_scan8[4] as usize];
            (*h).mb.cache.direct_ref[l as usize][2] =
                (*h).mb.cache.ref_0[l as usize][x264_scan8[8] as usize];
            (*h).mb.cache.direct_ref[l as usize][3] =
                (*h).mb.cache.ref_0[l as usize][x264_scan8[12] as usize];
            (*h).mb.cache.direct_partition = (*h).mb.i_partition;
            l += 1;
        }
    }
    return b_available;
}
#[no_mangle]
#[c2rust::src_loc = "519:1"]
unsafe extern "C" fn x264_10_mb_predict_mv_ref16x16(
    mut h: *mut x264_t,
    mut i_list: c_int,
    mut i_ref: c_int,
    mut mvc: *mut [int16_t; 2],
    mut i_mvc: *mut c_int,
) {
    let mut mvr: *mut [int16_t; 2] = (*h).mb.mvr[i_list as usize][i_ref as usize];
    let mut i: c_int = 0 as c_int;
    if (*h).sh.i_type == SLICE_TYPE_B as c_int
        && (*h).mb.cache.ref_0[i_list as usize][x264_scan8[12] as usize] as c_int == i_ref
    {
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(12 as c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
        i += 1;
    }
    if i_ref == 0 as c_int && (*h).frames.b_have_lowres != 0 {
        let mut idx: c_int = if i_list != 0 {
            (*(*h).fref[1][0]).i_frame - (*(*h).fenc).i_frame - 1 as c_int
        } else {
            (*(*h).fenc).i_frame - (*(*h).fref[0][0]).i_frame - 1 as c_int
        };
        if idx <= (*h).param.i_bframe {
            let mut lowres_mv: *mut [int16_t; 2] =
                (*(*h).fenc).lowres_mvs[i_list as usize][idx as usize];
            if (*lowres_mv.offset(0))[0] as c_int != 0x7fff as c_int {
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
        if (*h).mb.i_mb_left_xy[0] >= 0 as c_int {
            let mut shift: c_int = 1 as c_int + (*h).mb.b_interlaced
                - *(*h).mb.field.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int;
            let mut mvp: *mut int16_t = (*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((i_ref << 1 as c_int >> shift) as isize))
            .offset(*(*h).mb.i_mb_left_xy.as_mut_ptr().offset(0) as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0] = *mvp.offset(0);
            (*mvc.offset(i as isize))[1] =
                (*mvp.offset(1) as c_int * 2 as c_int >> shift) as int16_t;
            i += 1;
        }
        if (*h).mb.i_mb_top_xy >= 0 as c_int {
            let mut shift_0: c_int = 1 as c_int + (*h).mb.b_interlaced
                - *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as c_int;
            let mut mvp_0: *mut int16_t = (*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((i_ref << 1 as c_int >> shift_0) as isize))
            .offset((*h).mb.i_mb_top_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0] = *mvp_0.offset(0);
            (*mvc.offset(i as isize))[1] =
                (*mvp_0.offset(1) as c_int * 2 as c_int >> shift_0) as int16_t;
            i += 1;
        }
        if (*h).mb.i_mb_topleft_xy >= 0 as c_int {
            let mut shift_1: c_int = 1 as c_int + (*h).mb.b_interlaced
                - *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize) as c_int;
            let mut mvp_1: *mut int16_t = (*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((i_ref << 1 as c_int >> shift_1) as isize))
            .offset((*h).mb.i_mb_topleft_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0] = *mvp_1.offset(0);
            (*mvc.offset(i as isize))[1] =
                (*mvp_1.offset(1) as c_int * 2 as c_int >> shift_1) as int16_t;
            i += 1;
        }
        if (*h).mb.i_mb_topright_xy >= 0 as c_int {
            let mut shift_2: c_int = 1 as c_int + (*h).mb.b_interlaced
                - *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize) as c_int;
            let mut mvp_2: *mut int16_t = (*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((i_ref << 1 as c_int >> shift_2) as isize))
            .offset((*h).mb.i_mb_topright_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0] = *mvp_2.offset(0);
            (*mvc.offset(i as isize))[1] =
                (*mvp_2.offset(1) as c_int * 2 as c_int >> shift_2) as int16_t;
            i += 1;
        }
    } else {
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*mvr.offset(*(*h).mb.i_mb_left_xy.as_mut_ptr().offset(0) as isize)).as_mut_ptr()
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
    if (*(*h).fref[0][0]).i_ref[0] > 0 as c_int {
        let mut l0: *mut x264_frame_t = (*h).fref[0][0];
        let mut field: c_int = (*h).mb.i_mb_y & 1 as c_int;
        let mut curpoc: c_int = (*(*h).fdec).i_poc + (*(*h).fdec).i_delta_poc[field as usize];
        let mut refpoc: c_int =
            (*(*h).fref[i_list as usize][(i_ref >> (*h).sh.b_mbaff) as usize]).i_poc;
        refpoc += (*l0).i_delta_poc[(field ^ i_ref & 1 as c_int) as usize];
        let mut mb_index: c_int = (*h).mb.i_mb_xy + 0 as c_int + 0 as c_int * (*h).mb.i_mb_stride;
        let mut scale: c_int =
            (curpoc - refpoc) * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as c_int;
        (*mvc.offset(i as isize))[0] = x264_clip3(
            (*(*l0).mv16x16.offset(mb_index as isize))[0] as c_int * scale + 128 as c_int
                >> 8 as c_int,
            INT16_MIN,
            INT16_MAX,
        ) as int16_t;
        (*mvc.offset(i as isize))[1] = x264_clip3(
            (*(*l0).mv16x16.offset(mb_index as isize))[1] as c_int * scale + 128 as c_int
                >> 8 as c_int,
            INT16_MIN,
            INT16_MAX,
        ) as int16_t;
        i += 1;
        if (*h).mb.i_mb_x < (*h).mb.i_mb_width - 1 as c_int {
            let mut mb_index_0: c_int =
                (*h).mb.i_mb_xy + 1 as c_int + 0 as c_int * (*h).mb.i_mb_stride;
            let mut scale_0: c_int = (curpoc - refpoc)
                * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as c_int;
            (*mvc.offset(i as isize))[0] = x264_clip3(
                (*(*l0).mv16x16.offset(mb_index_0 as isize))[0] as c_int * scale_0 + 128 as c_int
                    >> 8 as c_int,
                INT16_MIN,
                INT16_MAX,
            ) as int16_t;
            (*mvc.offset(i as isize))[1] = x264_clip3(
                (*(*l0).mv16x16.offset(mb_index_0 as isize))[1] as c_int * scale_0 + 128 as c_int
                    >> 8 as c_int,
                INT16_MIN,
                INT16_MAX,
            ) as int16_t;
            i += 1;
        }
        if (*h).mb.i_mb_y < (*h).mb.i_mb_height - 1 as c_int {
            let mut mb_index_1: c_int =
                (*h).mb.i_mb_xy + 0 as c_int + 1 as c_int * (*h).mb.i_mb_stride;
            let mut scale_1: c_int = (curpoc - refpoc)
                * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as c_int;
            (*mvc.offset(i as isize))[0] = x264_clip3(
                (*(*l0).mv16x16.offset(mb_index_1 as isize))[0] as c_int * scale_1 + 128 as c_int
                    >> 8 as c_int,
                INT16_MIN,
                INT16_MAX,
            ) as int16_t;
            (*mvc.offset(i as isize))[1] = x264_clip3(
                (*(*l0).mv16x16.offset(mb_index_1 as isize))[1] as c_int * scale_1 + 128 as c_int
                    >> 8 as c_int,
                INT16_MIN,
                INT16_MAX,
            ) as int16_t;
            i += 1;
        }
    }
    *i_mvc = i;
}
