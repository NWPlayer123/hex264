use ::core::ffi::{c_int, c_long, c_uint, c_void};

use crate::base_h::{
    x264_scan8, CHROMA_420, CHROMA_422, CHROMA_444, CHROMA_DC, LUMA_DC, PROFILE_HIGH, SLICE_TYPE_B,
    SLICE_TYPE_P,
};
use crate::bitstream_h::{
    bs_align_0, bs_init, bs_pos, bs_t, bs_write, bs_write1, bs_write_se, bs_write_te, bs_write_ue,
    x264_10_level_token, x264_10_run_before, x264_run_level_t, LEVEL_TABLE_SIZE,
};
use crate::common_h::{dctcoef, x264_t, FENC_STRIDE, QP_MAX_SPEC};
use crate::internal::BIT_DEPTH;
use crate::macroblock_h::{
    x264_10_mb_predict_mv, x264_mb_partition_listX_table, x264_mb_predict_intra4x4_mode,
    x264_mb_predict_non_zero_code, x264_mb_transform_8x8_allowed, x264_mb_type_list_table, B_8x8,
    DCT_LUMA_4x4, D_16x16, D_16x8, D_8x16, I_16x16, I_4x4, I_8x8, P_8x8, B_BI_BI, B_DIRECT,
    B_L0_L0, B_SKIP, DCT_CHROMA_AC, DCT_CHROMA_DC, DCT_LUMA_AC, DCT_LUMA_DC, I_PCM, P_L0, P_SKIP,
};
use crate::osdep_h::x264_ctz_4bit;
use crate::predict_h::{
    x264_mb_chroma_pred_mode_fix, x264_mb_pred_mode16x16_fix, x264_mb_pred_mode4x4_fix,
};
use crate::stdint_intn_h::{int16_t, int32_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::tables_h::{
    vlc_t, x264_coeff0_token, x264_coeff_token, x264_total_zeros, x264_total_zeros_2x2_dc,
    x264_total_zeros_2x4_dc,
};
use crate::x264_h::X264_ANALYSE_PSUB8x8;
#[c2rust::src_loc = "36:22"]
static mut cbp_to_golomb: [[[uint8_t; 48]; 2]; 2] = [
    [
        [
            0 as c_int as uint8_t,
            1 as c_int as uint8_t,
            2 as c_int as uint8_t,
            5 as c_int as uint8_t,
            3 as c_int as uint8_t,
            6 as c_int as uint8_t,
            14 as c_int as uint8_t,
            10 as c_int as uint8_t,
            4 as c_int as uint8_t,
            15 as c_int as uint8_t,
            7 as c_int as uint8_t,
            11 as c_int as uint8_t,
            8 as c_int as uint8_t,
            12 as c_int as uint8_t,
            13 as c_int as uint8_t,
            9 as c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            1 as c_int as uint8_t,
            10 as c_int as uint8_t,
            11 as c_int as uint8_t,
            6 as c_int as uint8_t,
            12 as c_int as uint8_t,
            7 as c_int as uint8_t,
            14 as c_int as uint8_t,
            2 as c_int as uint8_t,
            13 as c_int as uint8_t,
            15 as c_int as uint8_t,
            8 as c_int as uint8_t,
            3 as c_int as uint8_t,
            9 as c_int as uint8_t,
            4 as c_int as uint8_t,
            5 as c_int as uint8_t,
            0 as c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            0 as c_int as uint8_t,
            2 as c_int as uint8_t,
            3 as c_int as uint8_t,
            7 as c_int as uint8_t,
            4 as c_int as uint8_t,
            8 as c_int as uint8_t,
            17 as c_int as uint8_t,
            13 as c_int as uint8_t,
            5 as c_int as uint8_t,
            18 as c_int as uint8_t,
            9 as c_int as uint8_t,
            14 as c_int as uint8_t,
            10 as c_int as uint8_t,
            15 as c_int as uint8_t,
            16 as c_int as uint8_t,
            11 as c_int as uint8_t,
            1 as c_int as uint8_t,
            32 as c_int as uint8_t,
            33 as c_int as uint8_t,
            36 as c_int as uint8_t,
            34 as c_int as uint8_t,
            37 as c_int as uint8_t,
            44 as c_int as uint8_t,
            40 as c_int as uint8_t,
            35 as c_int as uint8_t,
            45 as c_int as uint8_t,
            38 as c_int as uint8_t,
            41 as c_int as uint8_t,
            39 as c_int as uint8_t,
            42 as c_int as uint8_t,
            43 as c_int as uint8_t,
            19 as c_int as uint8_t,
            6 as c_int as uint8_t,
            24 as c_int as uint8_t,
            25 as c_int as uint8_t,
            20 as c_int as uint8_t,
            26 as c_int as uint8_t,
            21 as c_int as uint8_t,
            46 as c_int as uint8_t,
            28 as c_int as uint8_t,
            27 as c_int as uint8_t,
            47 as c_int as uint8_t,
            22 as c_int as uint8_t,
            29 as c_int as uint8_t,
            23 as c_int as uint8_t,
            30 as c_int as uint8_t,
            31 as c_int as uint8_t,
            12 as c_int as uint8_t,
        ],
        [
            3 as c_int as uint8_t,
            29 as c_int as uint8_t,
            30 as c_int as uint8_t,
            17 as c_int as uint8_t,
            31 as c_int as uint8_t,
            18 as c_int as uint8_t,
            37 as c_int as uint8_t,
            8 as c_int as uint8_t,
            32 as c_int as uint8_t,
            38 as c_int as uint8_t,
            19 as c_int as uint8_t,
            9 as c_int as uint8_t,
            20 as c_int as uint8_t,
            10 as c_int as uint8_t,
            11 as c_int as uint8_t,
            2 as c_int as uint8_t,
            16 as c_int as uint8_t,
            33 as c_int as uint8_t,
            34 as c_int as uint8_t,
            21 as c_int as uint8_t,
            35 as c_int as uint8_t,
            22 as c_int as uint8_t,
            39 as c_int as uint8_t,
            4 as c_int as uint8_t,
            36 as c_int as uint8_t,
            40 as c_int as uint8_t,
            23 as c_int as uint8_t,
            5 as c_int as uint8_t,
            24 as c_int as uint8_t,
            6 as c_int as uint8_t,
            7 as c_int as uint8_t,
            1 as c_int as uint8_t,
            41 as c_int as uint8_t,
            42 as c_int as uint8_t,
            43 as c_int as uint8_t,
            25 as c_int as uint8_t,
            44 as c_int as uint8_t,
            26 as c_int as uint8_t,
            46 as c_int as uint8_t,
            12 as c_int as uint8_t,
            45 as c_int as uint8_t,
            47 as c_int as uint8_t,
            27 as c_int as uint8_t,
            13 as c_int as uint8_t,
            28 as c_int as uint8_t,
            14 as c_int as uint8_t,
            15 as c_int as uint8_t,
            0 as c_int as uint8_t,
        ],
    ],
];
#[c2rust::src_loc = "48:22"]
static mut mb_type_b_to_golomb: [[uint8_t; 9]; 3] = [
    [
        4 as c_int as uint8_t,
        8 as c_int as uint8_t,
        12 as c_int as uint8_t,
        10 as c_int as uint8_t,
        6 as c_int as uint8_t,
        14 as c_int as uint8_t,
        16 as c_int as uint8_t,
        18 as c_int as uint8_t,
        20 as c_int as uint8_t,
    ],
    [
        5 as c_int as uint8_t,
        9 as c_int as uint8_t,
        13 as c_int as uint8_t,
        11 as c_int as uint8_t,
        7 as c_int as uint8_t,
        15 as c_int as uint8_t,
        17 as c_int as uint8_t,
        19 as c_int as uint8_t,
        21 as c_int as uint8_t,
    ],
    [
        1 as c_int as uint8_t,
        -(1 as c_int) as uint8_t,
        -(1 as c_int) as uint8_t,
        -(1 as c_int) as uint8_t,
        2 as c_int as uint8_t,
        -(1 as c_int) as uint8_t,
        -(1 as c_int) as uint8_t,
        -(1 as c_int) as uint8_t,
        3 as c_int as uint8_t,
    ],
];
#[c2rust::src_loc = "55:22"]
static mut subpartition_p_to_golomb: [uint8_t; 4] = [
    3 as c_int as uint8_t,
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    0 as c_int as uint8_t,
];
#[c2rust::src_loc = "60:22"]
static mut subpartition_b_to_golomb: [uint8_t; 13] = [
    10 as c_int as uint8_t,
    4 as c_int as uint8_t,
    5 as c_int as uint8_t,
    1 as c_int as uint8_t,
    11 as c_int as uint8_t,
    6 as c_int as uint8_t,
    7 as c_int as uint8_t,
    2 as c_int as uint8_t,
    12 as c_int as uint8_t,
    8 as c_int as uint8_t,
    9 as c_int as uint8_t,
    3 as c_int as uint8_t,
    0 as c_int as uint8_t,
];
#[inline]
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn cavlc_block_residual_escape(
    mut h: *mut x264_t,
    mut i_suffix_length: c_int,
    mut level: c_int,
) -> c_int {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    static mut next_suffix: [uint16_t; 7] = [
        0 as c_int as uint16_t,
        3 as c_int as uint16_t,
        6 as c_int as uint16_t,
        12 as c_int as uint16_t,
        24 as c_int as uint16_t,
        48 as c_int as uint16_t,
        0xffff as c_int as uint16_t,
    ];
    let mut i_level_prefix: c_int = 15 as c_int;
    let mut mask: c_int = level >> 31 as c_int;
    let mut abs_level: c_int = (level ^ mask) - mask;
    let mut i_level_code: c_int = abs_level * 2 as c_int - mask - 2 as c_int;
    if i_level_code >> i_suffix_length < 15 as c_int {
        bs_write(
            s,
            (i_level_code >> i_suffix_length) + 1 as c_int + i_suffix_length,
            (((1 as c_int) << i_suffix_length)
                + (i_level_code & ((1 as c_int) << i_suffix_length) - 1 as c_int))
                as uint32_t,
        );
    } else {
        i_level_code -= (15 as c_int) << i_suffix_length;
        if i_suffix_length == 0 as c_int {
            i_level_code -= 15 as c_int;
        }
        if i_level_code >= (1 as c_int) << 12 as c_int {
            if (*(*h).sps.as_mut_ptr()).i_profile_idc >= PROFILE_HIGH as c_int {
                while i_level_code >= (1 as c_int) << i_level_prefix - 3 as c_int {
                    i_level_code -= (1 as c_int) << i_level_prefix - 3 as c_int;
                    i_level_prefix += 1;
                }
            } else {
                (*h).mb.b_overflow = 1 as c_int;
            }
        }
        bs_write(s, i_level_prefix + 1 as c_int, 1 as uint32_t);
        bs_write(
            s,
            i_level_prefix - 3 as c_int,
            (i_level_code & ((1 as c_int) << i_level_prefix - 3 as c_int) - 1 as c_int) as uint32_t,
        );
    }
    if i_suffix_length == 0 as c_int {
        i_suffix_length += 1;
    }
    if abs_level > next_suffix[i_suffix_length as usize] as c_int {
        i_suffix_length += 1;
    }
    return i_suffix_length;
}
#[c2rust::src_loc = "121:1"]
unsafe extern "C" fn cavlc_block_residual_internal(
    mut h: *mut x264_t,
    mut ctx_block_cat: c_int,
    mut l: *mut dctcoef,
    mut nC: c_int,
) -> c_int {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    static mut ctz_index: [uint8_t; 8] = [
        3 as c_int as uint8_t,
        0 as c_int as uint8_t,
        1 as c_int as uint8_t,
        0 as c_int as uint8_t,
        2 as c_int as uint8_t,
        0 as c_int as uint8_t,
        1 as c_int as uint8_t,
        0 as c_int as uint8_t,
    ];
    static mut count_cat: [uint8_t; 14] = [
        16 as c_int as uint8_t,
        15 as c_int as uint8_t,
        16 as c_int as uint8_t,
        0 as c_int as uint8_t,
        15 as c_int as uint8_t,
        64 as c_int as uint8_t,
        16 as c_int as uint8_t,
        15 as c_int as uint8_t,
        16 as c_int as uint8_t,
        64 as c_int as uint8_t,
        16 as c_int as uint8_t,
        15 as c_int as uint8_t,
        16 as c_int as uint8_t,
        64 as c_int as uint8_t,
    ];
    let mut runlevel: x264_run_level_t = x264_run_level_t {
        last: 0,
        mask: 0,
        level: [0; 18],
    };
    let mut i_total: c_int = 0;
    let mut i_trailing: c_int = 0;
    let mut i_total_zero: c_int = 0;
    let mut i_suffix_length: c_int = 0;
    let mut i_sign: c_uint = 0;
    i_total = (*h).quantf.coeff_level_run[ctx_block_cat as usize]
        .expect("non-null function pointer")(l, &mut runlevel);
    &mut *x264_10_run_before
        .as_mut_ptr()
        .offset(runlevel.mask as isize) as *mut uint32_t;
    i_total_zero = (runlevel.last + 1 as int32_t - i_total as int32_t) as c_int;
    runlevel.level[(i_total + 0 as c_int) as usize] = 2 as c_int as dctcoef;
    runlevel.level[(i_total + 1 as c_int) as usize] = 2 as c_int as dctcoef;
    i_trailing = ((runlevel.level[0 as c_int as usize] + 1 as dctcoef
        | 1 as dctcoef - runlevel.level[0 as c_int as usize])
        >> 31 as c_int
        & 1 as dctcoef
        | (runlevel.level[1 as c_int as usize] + 1 as dctcoef
            | 1 as dctcoef - runlevel.level[1 as c_int as usize])
            >> 31 as c_int
            & 2 as dctcoef
        | (runlevel.level[2 as c_int as usize] + 1 as dctcoef
            | 1 as dctcoef - runlevel.level[2 as c_int as usize])
            >> 31 as c_int
            & 4 as dctcoef) as c_int;
    i_trailing = ctz_index[i_trailing as usize] as c_int;
    i_sign = (runlevel.level[2 as c_int as usize] >> 31 as c_int & 1 as dctcoef
        | runlevel.level[1 as c_int as usize] >> 31 as c_int & 2 as dctcoef
        | runlevel.level[0 as c_int as usize] >> 31 as c_int & 4 as dctcoef) as c_uint;
    i_sign >>= 3 as c_int - i_trailing;
    bs_write(
        s,
        x264_coeff_token[nC as usize][(i_total - 1 as c_int) as usize][i_trailing as usize].i_size
            as c_int,
        x264_coeff_token[nC as usize][(i_total - 1 as c_int) as usize][i_trailing as usize].i_bits
            as uint32_t,
    );
    i_suffix_length = (i_total > 10 as c_int && i_trailing < 3 as c_int) as c_int;
    bs_write(s, i_trailing, i_sign as uint32_t);
    if i_trailing < i_total {
        let mut val: c_int = runlevel.level[i_trailing as usize];
        let mut val_original: c_int =
            runlevel.level[i_trailing as usize] + LEVEL_TABLE_SIZE / 2 as c_int;
        val -= (val >> 31 as c_int | 1 as c_int) & -((i_trailing < 3 as c_int) as c_int);
        val += LEVEL_TABLE_SIZE / 2 as c_int;
        if (val_original as c_uint) < LEVEL_TABLE_SIZE as c_uint {
            bs_write(
                s,
                x264_10_level_token[i_suffix_length as usize][val as usize].i_size as c_int,
                x264_10_level_token[i_suffix_length as usize][val as usize].i_bits as uint32_t,
            );
            i_suffix_length = x264_10_level_token[i_suffix_length as usize][val_original as usize]
                .i_next as c_int;
        } else {
            i_suffix_length = cavlc_block_residual_escape(
                h,
                i_suffix_length,
                val - LEVEL_TABLE_SIZE / 2 as c_int,
            );
        }
        let mut i: c_int = i_trailing + 1 as c_int;
        while i < i_total {
            val =
                (runlevel.level[i as usize] + LEVEL_TABLE_SIZE as dctcoef / 2 as dctcoef) as c_int;
            if (val as c_uint) < LEVEL_TABLE_SIZE as c_uint {
                bs_write(
                    s,
                    x264_10_level_token[i_suffix_length as usize][val as usize].i_size as c_int,
                    x264_10_level_token[i_suffix_length as usize][val as usize].i_bits as uint32_t,
                );
                i_suffix_length =
                    x264_10_level_token[i_suffix_length as usize][val as usize].i_next as c_int;
            } else {
                i_suffix_length = cavlc_block_residual_escape(
                    h,
                    i_suffix_length,
                    val - LEVEL_TABLE_SIZE / 2 as c_int,
                );
            }
            i += 1;
        }
    }
    if ctx_block_cat == DCT_CHROMA_DC as c_int {
        if i_total < 8 as c_int >> (*h).mb.chroma_v_shift {
            let mut total_zeros: vlc_t =
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as c_int {
                    x264_total_zeros_2x2_dc[(i_total - 1 as c_int) as usize][i_total_zero as usize]
                } else {
                    x264_total_zeros_2x4_dc[(i_total - 1 as c_int) as usize][i_total_zero as usize]
                };
            bs_write(
                s,
                total_zeros.i_size as c_int,
                total_zeros.i_bits as uint32_t,
            );
        }
    } else if (i_total as uint8_t as c_int) < count_cat[ctx_block_cat as usize] as c_int {
        bs_write(
            s,
            x264_total_zeros[(i_total - 1 as c_int) as usize][i_total_zero as usize].i_size
                as c_int,
            x264_total_zeros[(i_total - 1 as c_int) as usize][i_total_zero as usize].i_bits
                as uint32_t,
        );
    }
    let mut zero_run_code: c_int = x264_10_run_before[runlevel.mask as usize] as c_int;
    bs_write(
        s,
        zero_run_code & 0x1f as c_int,
        (zero_run_code >> 5 as c_int) as uint32_t,
    );
    return i_total;
}
#[c2rust::src_loc = "198:22"]
static mut ct_index: [uint8_t; 17] = [
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    1 as c_int as uint8_t,
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
];
#[c2rust::src_loc = "211:1"]
unsafe extern "C" fn cavlc_qp_delta(mut h: *mut x264_t) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    let mut i_dqp: c_int = (*h).mb.i_qp - (*h).mb.i_last_qp;
    if (*h).mb.i_type == I_16x16 as c_int
        && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
        && (*h).mb.cache.non_zero_count[x264_scan8[LUMA_DC as usize] as usize] == 0
        && (*h).mb.cache.non_zero_count[x264_scan8[(CHROMA_DC + 0 as c_int) as usize] as usize] == 0
        && (*h).mb.cache.non_zero_count[x264_scan8[(CHROMA_DC + 1 as c_int) as usize] as usize] == 0
        && (*h).mb.i_qp > (*h).mb.i_last_qp
    {
        (*h).mb.i_qp = (*h).mb.i_last_qp;
        i_dqp = 0 as c_int;
    }
    if i_dqp != 0 {
        if i_dqp < -(QP_MAX_SPEC + 1 as c_int) / 2 as c_int {
            i_dqp += QP_MAX_SPEC + 1 as c_int;
        } else if i_dqp > QP_MAX_SPEC / 2 as c_int {
            i_dqp -= QP_MAX_SPEC + 1 as c_int;
        }
    }
    bs_write_se(s, i_dqp);
}
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn cavlc_mvd(
    mut h: *mut x264_t,
    mut i_list: c_int,
    mut idx: c_int,
    mut width: c_int,
) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    let mut mvp: [int16_t; 2] = [0; 2];
    x264_10_mb_predict_mv(h, i_list, idx, width, mvp.as_mut_ptr());
    bs_write_se(
        s,
        (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize][0 as c_int as usize]
            as c_int
            - mvp[0 as c_int as usize] as c_int,
    );
    bs_write_se(
        s,
        (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize][1 as c_int as usize]
            as c_int
            - mvp[1 as c_int as usize] as c_int,
    );
}
#[inline]
#[c2rust::src_loc = "250:1"]
unsafe extern "C" fn cavlc_8x8_mvd(mut h: *mut x264_t, mut i: c_int) {
    match (*h).mb.i_sub_partition[i as usize] as c_int {
        3 => {
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i, 2 as c_int);
        }
        1 => {
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i + 0 as c_int, 2 as c_int);
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i + 2 as c_int, 2 as c_int);
        }
        2 => {
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i + 0 as c_int, 1 as c_int);
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i + 1 as c_int, 1 as c_int);
        }
        0 => {
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i + 0 as c_int, 1 as c_int);
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i + 1 as c_int, 1 as c_int);
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i + 2 as c_int, 1 as c_int);
            cavlc_mvd(h, 0 as c_int, 4 as c_int * i + 3 as c_int, 1 as c_int);
        }
        _ => {}
    };
}
#[inline(always)]
#[c2rust::src_loc = "274:1"]
unsafe extern "C" fn cavlc_macroblock_luma_residual(mut h: *mut x264_t, mut plane_count: c_int) {
    if (*h).mb.b_transform_8x8 != 0 {
        let mut p: c_int = 0 as c_int;
        while p < plane_count {
            let mut i8: c_int = 0 as c_int;
            while i8 < 4 as c_int {
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(p * 16 as c_int + i8 * 4 as c_int) as usize] as usize]
                    != 0
                {
                    (*h).zigzagf
                        .interleave_8x8_cavlc
                        .expect("non-null function pointer")(
                        (*(*h)
                            .dct
                            .luma4x4
                            .as_mut_ptr()
                            .offset((p * 16 as c_int + i8 * 4 as c_int) as isize))
                        .as_mut_ptr(),
                        (*(*h)
                            .dct
                            .luma8x8
                            .as_mut_ptr()
                            .offset((p * 4 as c_int + i8) as isize))
                        .as_mut_ptr(),
                        &mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                            *x264_scan8
                                .as_ptr()
                                .offset((p * 16 as c_int + i8 * 4 as c_int) as isize)
                                as isize,
                        ),
                    );
                }
                i8 += 1;
            }
            p += 1;
        }
    }
    let mut p_0: c_int = 0 as c_int;
    while p_0 < plane_count {
        let mut i8_0: c_int = 0 as c_int;
        let mut msk: c_int = (*h).mb.i_cbp_luma;
        let mut skip: c_int = 0;
        while msk != 0 && {
            skip = x264_ctz_4bit(msk as uint32_t);
            i8_0 += skip;
            msk >>= skip + 1 as c_int;
            1 as c_int != 0
        } {
            let mut i4: c_int = 0 as c_int;
            while i4 < 4 as c_int {
                let mut nC: c_int = if DCT_LUMA_4x4 as c_int == DCT_CHROMA_DC as c_int {
                    5 as c_int - (*h).mb.chroma_v_shift
                } else {
                    ct_index[x264_mb_predict_non_zero_code(
                        h,
                        if DCT_LUMA_4x4 as c_int == DCT_LUMA_DC as c_int {
                            (i4 + i8_0 * 4 as c_int + p_0 * 16 as c_int - LUMA_DC) * 16 as c_int
                        } else {
                            i4 + i8_0 * 4 as c_int + p_0 * 16 as c_int
                        },
                    ) as usize] as c_int
                };
                let mut nnz: *mut uint8_t = &mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                    *x264_scan8
                        .as_ptr()
                        .offset((i4 + i8_0 * 4 as c_int + p_0 * 16 as c_int) as isize)
                        as isize,
                ) as *mut uint8_t;
                if *nnz == 0 {
                    bs_write(
                        &mut (*h).out.bs,
                        x264_coeff0_token[nC as usize].i_size as c_int,
                        x264_coeff0_token[nC as usize].i_bits as uint32_t,
                    );
                } else {
                    *nnz = cavlc_block_residual_internal(
                        h,
                        DCT_LUMA_4x4 as c_int,
                        (*(*h)
                            .dct
                            .luma4x4
                            .as_mut_ptr()
                            .offset((i4 + i8_0 * 4 as c_int + p_0 * 16 as c_int) as isize))
                        .as_mut_ptr(),
                        nC,
                    ) as uint8_t;
                }
                i4 += 1;
            }
            i8_0 += 1;
        }
        p_0 += 1;
    }
}
#[c2rust::src_loc = "305:1"]
unsafe extern "C" fn cavlc_mb_header_i(
    mut h: *mut x264_t,
    mut i_mb_type: c_int,
    mut i_mb_i_offset: c_int,
    mut chroma: c_int,
) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    if i_mb_type == I_16x16 as c_int {
        bs_write_ue(
            s,
            i_mb_i_offset
                + 1 as c_int
                + x264_mb_pred_mode16x16_fix[(*h).mb.i_intra16x16_pred_mode as usize] as c_int
                + (*h).mb.i_cbp_chroma * 4 as c_int
                + (if (*h).mb.i_cbp_luma == 0 as c_int {
                    0 as c_int
                } else {
                    12 as c_int
                }),
        );
    } else {
        let mut di: c_int = if i_mb_type == I_8x8 as c_int {
            4 as c_int
        } else {
            1 as c_int
        };
        bs_write_ue(s, i_mb_i_offset + 0 as c_int);
        if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
            bs_write1(s, (*h).mb.b_transform_8x8 as uint32_t);
        }
        let mut i: c_int = 0 as c_int;
        while i < 16 as c_int {
            let mut i_pred: c_int = x264_mb_predict_intra4x4_mode(h, i);
            let mut i_mode: c_int = x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                [x264_scan8[i as usize] as usize]
                as c_int
                + 1 as c_int) as usize] as c_int;
            if i_pred == i_mode {
                bs_write1(s, 1 as uint32_t);
            } else {
                bs_write(
                    s,
                    4 as c_int,
                    (i_mode - (i_mode > i_pred) as c_int) as uint32_t,
                );
            }
            i += di;
        }
    }
    if chroma != 0 {
        bs_write_ue(
            s,
            x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as c_int,
        );
    }
}
#[inline(always)]
#[c2rust::src_loc = "337:1"]
unsafe extern "C" fn cavlc_mb_header_p(
    mut h: *mut x264_t,
    mut i_mb_type: c_int,
    mut chroma: c_int,
) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    if i_mb_type == P_L0 as c_int {
        if (*h).mb.i_partition == D_16x16 as c_int {
            bs_write1(s, 1 as uint32_t);
            if (*h).mb.pic.i_fref[0 as c_int as usize] > 1 as c_int {
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                    (*h).mb.cache.ref_0[0 as c_int as usize]
                        [x264_scan8[0 as c_int as usize] as usize] as c_int,
                );
            }
            cavlc_mvd(h, 0 as c_int, 0 as c_int, 4 as c_int);
        } else if (*h).mb.i_partition == D_16x8 as c_int {
            bs_write_ue(s, 1 as c_int);
            if (*h).mb.pic.i_fref[0 as c_int as usize] > 1 as c_int {
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                    (*h).mb.cache.ref_0[0 as c_int as usize]
                        [x264_scan8[0 as c_int as usize] as usize] as c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                    (*h).mb.cache.ref_0[0 as c_int as usize]
                        [x264_scan8[8 as c_int as usize] as usize] as c_int,
                );
            }
            cavlc_mvd(h, 0 as c_int, 0 as c_int, 4 as c_int);
            cavlc_mvd(h, 0 as c_int, 8 as c_int, 4 as c_int);
        } else if (*h).mb.i_partition == D_8x16 as c_int {
            bs_write_ue(s, 2 as c_int);
            if (*h).mb.pic.i_fref[0 as c_int as usize] > 1 as c_int {
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                    (*h).mb.cache.ref_0[0 as c_int as usize]
                        [x264_scan8[0 as c_int as usize] as usize] as c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                    (*h).mb.cache.ref_0[0 as c_int as usize]
                        [x264_scan8[4 as c_int as usize] as usize] as c_int,
                );
            }
            cavlc_mvd(h, 0 as c_int, 0 as c_int, 2 as c_int);
            cavlc_mvd(h, 0 as c_int, 4 as c_int, 2 as c_int);
        }
    } else if i_mb_type == P_8x8 as c_int {
        let mut b_sub_ref: c_int = 0;
        if (*h).mb.cache.ref_0[0 as c_int as usize][x264_scan8[0 as c_int as usize] as usize]
            as c_int
            | (*h).mb.cache.ref_0[0 as c_int as usize][x264_scan8[4 as c_int as usize] as usize]
                as c_int
            | (*h).mb.cache.ref_0[0 as c_int as usize][x264_scan8[8 as c_int as usize] as usize]
                as c_int
            | (*h).mb.cache.ref_0[0 as c_int as usize][x264_scan8[12 as c_int as usize] as usize]
                as c_int
            == 0 as c_int
        {
            bs_write_ue(s, 4 as c_int);
            b_sub_ref = 0 as c_int;
        } else {
            bs_write_ue(s, 3 as c_int);
            b_sub_ref = 1 as c_int;
        }
        if (*h).param.analyse.inter & X264_ANALYSE_PSUB8x8 != 0 {
            let mut i: c_int = 0 as c_int;
            while i < 4 as c_int {
                bs_write_ue(
                    s,
                    subpartition_p_to_golomb[(*h).mb.i_sub_partition[i as usize] as usize] as c_int,
                );
                i += 1;
            }
        } else {
            bs_write(s, 4 as c_int, 0xf as uint32_t);
        }
        if b_sub_ref != 0 {
            bs_write_te(
                s,
                (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                (*h).mb.cache.ref_0[0 as c_int as usize][x264_scan8[0 as c_int as usize] as usize]
                    as c_int,
            );
            bs_write_te(
                s,
                (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                (*h).mb.cache.ref_0[0 as c_int as usize][x264_scan8[4 as c_int as usize] as usize]
                    as c_int,
            );
            bs_write_te(
                s,
                (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                (*h).mb.cache.ref_0[0 as c_int as usize][x264_scan8[8 as c_int as usize] as usize]
                    as c_int,
            );
            bs_write_te(
                s,
                (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                (*h).mb.cache.ref_0[0 as c_int as usize][x264_scan8[12 as c_int as usize] as usize]
                    as c_int,
            );
        }
        let mut i_0: c_int = 0 as c_int;
        while i_0 < 4 as c_int {
            cavlc_8x8_mvd(h, i_0);
            i_0 += 1;
        }
    } else {
        cavlc_mb_header_i(h, i_mb_type, 5 as c_int, chroma);
    };
}
#[inline(always)]
#[c2rust::src_loc = "411:1"]
unsafe extern "C" fn cavlc_mb_header_b(
    mut h: *mut x264_t,
    mut i_mb_type: c_int,
    mut chroma: c_int,
) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    if i_mb_type == B_8x8 as c_int {
        bs_write_ue(s, 22 as c_int);
        let mut i: c_int = 0 as c_int;
        while i < 4 as c_int {
            bs_write_ue(
                s,
                subpartition_b_to_golomb[(*h).mb.i_sub_partition[i as usize] as usize] as c_int,
            );
            i += 1;
        }
        if (*h).mb.pic.i_fref[0 as c_int as usize] > 1 as c_int {
            let mut i_0: c_int = 0 as c_int;
            while i_0 < 4 as c_int {
                if x264_mb_partition_listX_table[0 as c_int as usize]
                    [(*h).mb.i_sub_partition[i_0 as usize] as usize]
                    != 0
                {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int,
                        (*h).mb.cache.ref_0[0 as c_int as usize]
                            [x264_scan8[(i_0 * 4 as c_int) as usize] as usize]
                            as c_int,
                    );
                }
                i_0 += 1;
            }
        }
        if (*h).mb.pic.i_fref[1 as c_int as usize] > 1 as c_int {
            let mut i_1: c_int = 0 as c_int;
            while i_1 < 4 as c_int {
                if x264_mb_partition_listX_table[1 as c_int as usize]
                    [(*h).mb.i_sub_partition[i_1 as usize] as usize]
                    != 0
                {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[1 as c_int as usize] - 1 as c_int,
                        (*h).mb.cache.ref_0[1 as c_int as usize]
                            [x264_scan8[(i_1 * 4 as c_int) as usize] as usize]
                            as c_int,
                    );
                }
                i_1 += 1;
            }
        }
        let mut i_2: c_int = 0 as c_int;
        while i_2 < 4 as c_int {
            if x264_mb_partition_listX_table[0 as c_int as usize]
                [(*h).mb.i_sub_partition[i_2 as usize] as usize]
                != 0
            {
                cavlc_mvd(h, 0 as c_int, 4 as c_int * i_2, 2 as c_int);
            }
            i_2 += 1;
        }
        let mut i_3: c_int = 0 as c_int;
        while i_3 < 4 as c_int {
            if x264_mb_partition_listX_table[1 as c_int as usize]
                [(*h).mb.i_sub_partition[i_3 as usize] as usize]
                != 0
            {
                cavlc_mvd(h, 1 as c_int, 4 as c_int * i_3, 2 as c_int);
            }
            i_3 += 1;
        }
    } else if i_mb_type >= B_L0_L0 as c_int && i_mb_type <= B_BI_BI as c_int {
        let mut b_list: *const [uint8_t; 2] =
            (*x264_mb_type_list_table.as_ptr().offset(i_mb_type as isize)).as_ptr()
                as *const [uint8_t; 2];
        let i_ref0_max: c_int = (*h).mb.pic.i_fref[0 as c_int as usize] - 1 as c_int;
        let i_ref1_max: c_int = (*h).mb.pic.i_fref[1 as c_int as usize] - 1 as c_int;
        bs_write_ue(
            s,
            mb_type_b_to_golomb[((*h).mb.i_partition - D_16x8 as c_int) as usize]
                [(i_mb_type - B_L0_L0 as c_int) as usize] as c_int,
        );
        if (*h).mb.i_partition == D_16x16 as c_int {
            if i_ref0_max != 0
                && (*b_list.offset(0 as c_int as isize))[0 as c_int as usize] as c_int != 0
            {
                bs_write_te(
                    s,
                    i_ref0_max,
                    (*h).mb.cache.ref_0[0 as c_int as usize]
                        [x264_scan8[0 as c_int as usize] as usize] as c_int,
                );
            }
            if i_ref1_max != 0
                && (*b_list.offset(1 as c_int as isize))[0 as c_int as usize] as c_int != 0
            {
                bs_write_te(
                    s,
                    i_ref1_max,
                    (*h).mb.cache.ref_0[1 as c_int as usize]
                        [x264_scan8[0 as c_int as usize] as usize] as c_int,
                );
            }
            if (*b_list.offset(0 as c_int as isize))[0 as c_int as usize] != 0 {
                cavlc_mvd(h, 0 as c_int, 0 as c_int, 4 as c_int);
            }
            if (*b_list.offset(1 as c_int as isize))[0 as c_int as usize] != 0 {
                cavlc_mvd(h, 1 as c_int, 0 as c_int, 4 as c_int);
            }
        } else {
            if i_ref0_max != 0
                && (*b_list.offset(0 as c_int as isize))[0 as c_int as usize] as c_int != 0
            {
                bs_write_te(
                    s,
                    i_ref0_max,
                    (*h).mb.cache.ref_0[0 as c_int as usize]
                        [x264_scan8[0 as c_int as usize] as usize] as c_int,
                );
            }
            if i_ref0_max != 0
                && (*b_list.offset(0 as c_int as isize))[1 as c_int as usize] as c_int != 0
            {
                bs_write_te(
                    s,
                    i_ref0_max,
                    (*h).mb.cache.ref_0[0 as c_int as usize]
                        [x264_scan8[12 as c_int as usize] as usize] as c_int,
                );
            }
            if i_ref1_max != 0
                && (*b_list.offset(1 as c_int as isize))[0 as c_int as usize] as c_int != 0
            {
                bs_write_te(
                    s,
                    i_ref1_max,
                    (*h).mb.cache.ref_0[1 as c_int as usize]
                        [x264_scan8[0 as c_int as usize] as usize] as c_int,
                );
            }
            if i_ref1_max != 0
                && (*b_list.offset(1 as c_int as isize))[1 as c_int as usize] as c_int != 0
            {
                bs_write_te(
                    s,
                    i_ref1_max,
                    (*h).mb.cache.ref_0[1 as c_int as usize]
                        [x264_scan8[12 as c_int as usize] as usize] as c_int,
                );
            }
            if (*h).mb.i_partition == D_16x8 as c_int {
                if (*b_list.offset(0 as c_int as isize))[0 as c_int as usize] != 0 {
                    cavlc_mvd(h, 0 as c_int, 0 as c_int, 4 as c_int);
                }
                if (*b_list.offset(0 as c_int as isize))[1 as c_int as usize] != 0 {
                    cavlc_mvd(h, 0 as c_int, 8 as c_int, 4 as c_int);
                }
                if (*b_list.offset(1 as c_int as isize))[0 as c_int as usize] != 0 {
                    cavlc_mvd(h, 1 as c_int, 0 as c_int, 4 as c_int);
                }
                if (*b_list.offset(1 as c_int as isize))[1 as c_int as usize] != 0 {
                    cavlc_mvd(h, 1 as c_int, 8 as c_int, 4 as c_int);
                }
            } else {
                if (*b_list.offset(0 as c_int as isize))[0 as c_int as usize] != 0 {
                    cavlc_mvd(h, 0 as c_int, 0 as c_int, 2 as c_int);
                }
                if (*b_list.offset(0 as c_int as isize))[1 as c_int as usize] != 0 {
                    cavlc_mvd(h, 0 as c_int, 4 as c_int, 2 as c_int);
                }
                if (*b_list.offset(1 as c_int as isize))[0 as c_int as usize] != 0 {
                    cavlc_mvd(h, 1 as c_int, 0 as c_int, 2 as c_int);
                }
                if (*b_list.offset(1 as c_int as isize))[1 as c_int as usize] != 0 {
                    cavlc_mvd(h, 1 as c_int, 4 as c_int, 2 as c_int);
                }
            }
        }
    } else if i_mb_type == B_DIRECT as c_int {
        bs_write1(s, 1 as uint32_t);
    } else {
        cavlc_mb_header_i(h, i_mb_type, 23 as c_int, chroma);
    };
}
#[no_mangle]
#[c2rust::src_loc = "487:1"]
unsafe extern "C" fn x264_10_macroblock_write_cavlc(mut h: *mut x264_t) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    let i_mb_type: c_int = (*h).mb.i_type;
    let mut plane_count: c_int =
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            3 as c_int
        } else {
            1 as c_int
        };
    let mut chroma: c_int = ((*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as c_int
        || (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as c_int)
        as c_int;
    let i_mb_pos_start: c_int = bs_pos(s) as c_int;
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
        bs_write1(s, (*h).mb.b_interlaced as uint32_t);
        (*h).mb.field_decoding_flag = (*h).mb.b_interlaced;
    }
    if i_mb_type == I_PCM as c_int {
        static mut i_offsets: [uint8_t; 3] = [
            5 as c_int as uint8_t,
            23 as c_int as uint8_t,
            0 as c_int as uint8_t,
        ];
        let mut p_start: *mut uint8_t = (*s).p_start;
        bs_write_ue(s, i_offsets[(*h).sh.i_type as usize] as c_int + 25 as c_int);
        i_mb_pos_tex = bs_pos(s);
        (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
        bs_align_0(s);
        let mut p: c_int = 0 as c_int;
        while p < plane_count {
            let mut i: c_int = 0 as c_int;
            while i < 256 as c_int {
                bs_write(
                    s,
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
                            s,
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
        bs_init(
            s,
            (*s).p as *mut c_void,
            (*s).p_end.offset_from((*s).p) as c_long as c_int,
        );
        (*s).p_start = p_start;
        (*h).stat.frame.i_tex_bits += bs_pos(s) - i_mb_pos_tex;
        return;
    }
    if (*h).sh.i_type == SLICE_TYPE_P as c_int {
        cavlc_mb_header_p(h, i_mb_type, chroma);
    } else if (*h).sh.i_type == SLICE_TYPE_B as c_int {
        cavlc_mb_header_b(h, i_mb_type, chroma);
    } else {
        cavlc_mb_header_i(h, i_mb_type, 0 as c_int, chroma);
    }
    i_mb_pos_tex = bs_pos(s);
    (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
    if i_mb_type != I_16x16 as c_int {
        bs_write_ue(
            s,
            cbp_to_golomb[chroma as usize][(i_mb_type == I_4x4 as c_int
                || i_mb_type == I_8x8 as c_int
                || i_mb_type == I_16x16 as c_int
                || i_mb_type == I_PCM as c_int) as c_int
                as usize][((*h).mb.i_cbp_chroma << 4 as c_int | (*h).mb.i_cbp_luma) as usize]
                as c_int,
        );
    }
    if x264_mb_transform_8x8_allowed(h) != 0 && (*h).mb.i_cbp_luma != 0 {
        bs_write1(s, (*h).mb.b_transform_8x8 as uint32_t);
    }
    if i_mb_type == I_16x16 as c_int {
        cavlc_qp_delta(h);
        let mut p_0: c_int = 0 as c_int;
        while p_0 < plane_count {
            let mut nC: c_int = if DCT_LUMA_DC as c_int == DCT_CHROMA_DC as c_int {
                5 as c_int - (*h).mb.chroma_v_shift
            } else {
                ct_index[x264_mb_predict_non_zero_code(
                    h,
                    if DCT_LUMA_DC as c_int == DCT_LUMA_DC as c_int {
                        (48 as c_int + p_0 - LUMA_DC) * 16 as c_int
                    } else {
                        48 as c_int + p_0
                    },
                ) as usize] as c_int
            };
            let mut nnz: *mut uint8_t = &mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset((48 as c_int + p_0) as isize) as isize)
                as *mut uint8_t;
            if *nnz == 0 {
                bs_write(
                    &mut (*h).out.bs,
                    x264_coeff0_token[nC as usize].i_size as c_int,
                    x264_coeff0_token[nC as usize].i_bits as uint32_t,
                );
            } else {
                *nnz = cavlc_block_residual_internal(
                    h,
                    DCT_LUMA_DC as c_int,
                    (*(*h).dct.luma16x16_dc.as_mut_ptr().offset(p_0 as isize)).as_mut_ptr(),
                    nC,
                ) as uint8_t;
            }
            if (*h).mb.i_cbp_luma != 0 {
                let mut i_1: c_int = p_0 * 16 as c_int;
                while i_1 < p_0 * 16 as c_int + 16 as c_int {
                    let mut nC_0: c_int = if DCT_LUMA_AC as c_int == DCT_CHROMA_DC as c_int {
                        5 as c_int - (*h).mb.chroma_v_shift
                    } else {
                        ct_index[x264_mb_predict_non_zero_code(
                            h,
                            if DCT_LUMA_AC as c_int == DCT_LUMA_DC as c_int {
                                (i_1 - LUMA_DC) * 16 as c_int
                            } else {
                                i_1
                            },
                        ) as usize] as c_int
                    };
                    let mut nnz_0: *mut uint8_t = &mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(*x264_scan8.as_ptr().offset(i_1 as isize) as isize)
                        as *mut uint8_t;
                    if *nnz_0 == 0 {
                        bs_write(
                            &mut (*h).out.bs,
                            x264_coeff0_token[nC_0 as usize].i_size as c_int,
                            x264_coeff0_token[nC_0 as usize].i_bits as uint32_t,
                        );
                    } else {
                        *nnz_0 = cavlc_block_residual_internal(
                            h,
                            DCT_LUMA_AC as c_int,
                            (*(*h).dct.luma4x4.as_mut_ptr().offset(i_1 as isize))
                                .as_mut_ptr()
                                .offset(1 as c_int as isize),
                            nC_0,
                        ) as uint8_t;
                    }
                    i_1 += 1;
                }
            }
            p_0 += 1;
        }
    } else if (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma != 0 {
        cavlc_qp_delta(h);
        cavlc_macroblock_luma_residual(h, plane_count);
    }
    if (*h).mb.i_cbp_chroma != 0 {
        let mut nC_1: c_int = if DCT_CHROMA_DC as c_int == DCT_CHROMA_DC as c_int {
            5 as c_int - (*h).mb.chroma_v_shift
        } else {
            ct_index[x264_mb_predict_non_zero_code(
                h,
                if DCT_CHROMA_DC as c_int == DCT_LUMA_DC as c_int {
                    (49 as c_int + 0 as c_int - LUMA_DC) * 16 as c_int
                } else {
                    49 as c_int + 0 as c_int
                },
            ) as usize] as c_int
        };
        let mut nnz_1: *mut uint8_t = &mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((49 as c_int + 0 as c_int) as isize) as isize,
        ) as *mut uint8_t;
        if *nnz_1 == 0 {
            bs_write(
                &mut (*h).out.bs,
                x264_coeff0_token[nC_1 as usize].i_size as c_int,
                x264_coeff0_token[nC_1 as usize].i_bits as uint32_t,
            );
        } else {
            *nnz_1 = cavlc_block_residual_internal(
                h,
                DCT_CHROMA_DC as c_int,
                (*(*h).dct.chroma_dc.as_mut_ptr().offset(0 as c_int as isize)).as_mut_ptr(),
                nC_1,
            ) as uint8_t;
        }
        let mut nC_2: c_int = if DCT_CHROMA_DC as c_int == DCT_CHROMA_DC as c_int {
            5 as c_int - (*h).mb.chroma_v_shift
        } else {
            ct_index[x264_mb_predict_non_zero_code(
                h,
                if DCT_CHROMA_DC as c_int == DCT_LUMA_DC as c_int {
                    (49 as c_int + 1 as c_int - LUMA_DC) * 16 as c_int
                } else {
                    49 as c_int + 1 as c_int
                },
            ) as usize] as c_int
        };
        let mut nnz_2: *mut uint8_t = &mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((49 as c_int + 1 as c_int) as isize) as isize,
        ) as *mut uint8_t;
        if *nnz_2 == 0 {
            bs_write(
                &mut (*h).out.bs,
                x264_coeff0_token[nC_2 as usize].i_size as c_int,
                x264_coeff0_token[nC_2 as usize].i_bits as uint32_t,
            );
        } else {
            *nnz_2 = cavlc_block_residual_internal(
                h,
                DCT_CHROMA_DC as c_int,
                (*(*h).dct.chroma_dc.as_mut_ptr().offset(1 as c_int as isize)).as_mut_ptr(),
                nC_2,
            ) as uint8_t;
        }
        if (*h).mb.i_cbp_chroma == 2 as c_int {
            let mut step: c_int = (8 as c_int) << (*h).mb.chroma_v_shift;
            let mut i_2: c_int = 16 as c_int;
            while i_2 < 3 as c_int * 16 as c_int {
                let mut j_0: c_int = i_2;
                while j_0 < i_2 + 4 as c_int {
                    let mut nC_3: c_int = if DCT_CHROMA_AC as c_int == DCT_CHROMA_DC as c_int {
                        5 as c_int - (*h).mb.chroma_v_shift
                    } else {
                        ct_index[x264_mb_predict_non_zero_code(
                            h,
                            if DCT_CHROMA_AC as c_int == DCT_LUMA_DC as c_int {
                                (j_0 - LUMA_DC) * 16 as c_int
                            } else {
                                j_0
                            },
                        ) as usize] as c_int
                    };
                    let mut nnz_3: *mut uint8_t = &mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(*x264_scan8.as_ptr().offset(j_0 as isize) as isize)
                        as *mut uint8_t;
                    if *nnz_3 == 0 {
                        bs_write(
                            &mut (*h).out.bs,
                            x264_coeff0_token[nC_3 as usize].i_size as c_int,
                            x264_coeff0_token[nC_3 as usize].i_bits as uint32_t,
                        );
                    } else {
                        *nnz_3 = cavlc_block_residual_internal(
                            h,
                            DCT_CHROMA_AC as c_int,
                            (*(*h).dct.luma4x4.as_mut_ptr().offset(j_0 as isize))
                                .as_mut_ptr()
                                .offset(1 as c_int as isize),
                            nC_3,
                        ) as uint8_t;
                    }
                    j_0 += 1;
                }
                i_2 += step;
            }
        }
    }
    (*h).stat.frame.i_tex_bits += bs_pos(s) - i_mb_pos_tex;
}
