use core::ffi::{c_int, c_uint};

use crate::base_h::{
    x264_clip3, x264_union32_t, x264_union64_t, CHROMA_420, CHROMA_422, CHROMA_444, SLICE_TYPE_B,
    X264_SCAN8_0,
};
use crate::common_h::{pixel, x264_clip_pixel, x264_t, FDEC_STRIDE, QP_BD_OFFSET};
use crate::frame_h::{x264_deblock_function_t, x264_deblock_inter_t, x264_deblock_intra_t};
use crate::internal::BIT_DEPTH;
use crate::macroblock_h::{
    x264_10_prefetch_fenc, D_16x16, I_16x16, I_4x4, I_8x8, I_PCM, MB_LEFT, MB_TOP,
};
use crate::stdint_h::intptr_t;
use crate::stdint_intn_h::{int16_t, int8_t};
use crate::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
use crate::stdlib_h::abs;
use crate::x264_h::X264_MBINFO_CONSTANT;
#[c2rust::src_loc = "32:22"]
static mut i_alpha_table: [uint8_t; 88] = [
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    5 as c_int as uint8_t,
    6 as c_int as uint8_t,
    7 as c_int as uint8_t,
    8 as c_int as uint8_t,
    9 as c_int as uint8_t,
    10 as c_int as uint8_t,
    12 as c_int as uint8_t,
    13 as c_int as uint8_t,
    15 as c_int as uint8_t,
    17 as c_int as uint8_t,
    20 as c_int as uint8_t,
    22 as c_int as uint8_t,
    25 as c_int as uint8_t,
    28 as c_int as uint8_t,
    32 as c_int as uint8_t,
    36 as c_int as uint8_t,
    40 as c_int as uint8_t,
    45 as c_int as uint8_t,
    50 as c_int as uint8_t,
    56 as c_int as uint8_t,
    63 as c_int as uint8_t,
    71 as c_int as uint8_t,
    80 as c_int as uint8_t,
    90 as c_int as uint8_t,
    101 as c_int as uint8_t,
    113 as c_int as uint8_t,
    127 as c_int as uint8_t,
    144 as c_int as uint8_t,
    162 as c_int as uint8_t,
    182 as c_int as uint8_t,
    203 as c_int as uint8_t,
    226 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
    255 as c_int as uint8_t,
];
#[c2rust::src_loc = "44:22"]
static mut i_beta_table: [uint8_t; 88] = [
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    4 as c_int as uint8_t,
    6 as c_int as uint8_t,
    6 as c_int as uint8_t,
    7 as c_int as uint8_t,
    7 as c_int as uint8_t,
    8 as c_int as uint8_t,
    8 as c_int as uint8_t,
    9 as c_int as uint8_t,
    9 as c_int as uint8_t,
    10 as c_int as uint8_t,
    10 as c_int as uint8_t,
    11 as c_int as uint8_t,
    11 as c_int as uint8_t,
    12 as c_int as uint8_t,
    12 as c_int as uint8_t,
    13 as c_int as uint8_t,
    13 as c_int as uint8_t,
    14 as c_int as uint8_t,
    14 as c_int as uint8_t,
    15 as c_int as uint8_t,
    15 as c_int as uint8_t,
    16 as c_int as uint8_t,
    16 as c_int as uint8_t,
    17 as c_int as uint8_t,
    17 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
    18 as c_int as uint8_t,
];
#[c2rust::src_loc = "56:21"]
static mut i_tc0_table: [[int8_t; 4]; 88] = [
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        0 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        0 as c_int as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
        2 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
        2 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
        2 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        1 as c_int as int8_t,
        2 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        2 as c_int as int8_t,
        3 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        1 as c_int as int8_t,
        2 as c_int as int8_t,
        3 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        2 as c_int as int8_t,
        2 as c_int as int8_t,
        3 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        2 as c_int as int8_t,
        2 as c_int as int8_t,
        4 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        2 as c_int as int8_t,
        3 as c_int as int8_t,
        4 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        2 as c_int as int8_t,
        3 as c_int as int8_t,
        4 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        3 as c_int as int8_t,
        3 as c_int as int8_t,
        5 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        3 as c_int as int8_t,
        4 as c_int as int8_t,
        6 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        3 as c_int as int8_t,
        4 as c_int as int8_t,
        6 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        4 as c_int as int8_t,
        5 as c_int as int8_t,
        7 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        4 as c_int as int8_t,
        5 as c_int as int8_t,
        8 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        4 as c_int as int8_t,
        6 as c_int as int8_t,
        9 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        5 as c_int as int8_t,
        7 as c_int as int8_t,
        10 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        6 as c_int as int8_t,
        8 as c_int as int8_t,
        11 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        6 as c_int as int8_t,
        8 as c_int as int8_t,
        13 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        7 as c_int as int8_t,
        10 as c_int as int8_t,
        14 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        8 as c_int as int8_t,
        11 as c_int as int8_t,
        16 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        9 as c_int as int8_t,
        12 as c_int as int8_t,
        18 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        10 as c_int as int8_t,
        13 as c_int as int8_t,
        20 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        11 as c_int as int8_t,
        15 as c_int as int8_t,
        23 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
    [
        -1 as int8_t,
        13 as c_int as int8_t,
        17 as c_int as int8_t,
        25 as c_int as int8_t,
    ],
];
#[inline(always)]
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn deblock_edge_luma_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: int8_t,
) {
    let mut p2: c_int = *pix.offset((-(3 as c_int) as intptr_t * xstride) as isize) as c_int;
    let mut p1: c_int = *pix.offset((-(2 as c_int) as intptr_t * xstride) as isize) as c_int;
    let mut p0: c_int = *pix.offset((-1 as intptr_t * xstride) as isize) as c_int;
    let mut q0: c_int = *pix.offset((0 as intptr_t * xstride) as isize) as c_int;
    let mut q1: c_int = *pix.offset((1 as intptr_t * xstride) as isize) as c_int;
    let mut q2: c_int = *pix.offset((2 as intptr_t * xstride) as isize) as c_int;
    if abs(p0 - q0) < alpha && abs(p1 - p0) < beta && abs(q1 - q0) < beta {
        let mut tc: c_int = tc0 as c_int;
        let mut delta: c_int = 0;
        if abs(p2 - p0) < beta {
            if tc0 != 0 {
                *pix.offset((-(2 as c_int) as intptr_t * xstride) as isize) =
                    (p1 + x264_clip3(
                        (p2 + (p0 + q0 + 1 as c_int >> 1 as c_int) >> 1 as c_int) - p1,
                        -(tc0 as c_int),
                        tc0 as c_int,
                    )) as pixel;
            }
            tc += 1;
        }
        if abs(q2 - q0) < beta {
            if tc0 != 0 {
                *pix.offset((1 as intptr_t * xstride) as isize) = (q1
                    + x264_clip3(
                        (q2 + (p0 + q0 + 1 as c_int >> 1 as c_int) >> 1 as c_int) - q1,
                        -(tc0 as c_int),
                        tc0 as c_int,
                    )) as pixel;
            }
            tc += 1;
        }
        delta = x264_clip3(
            (q0 - p0) * 4 as c_int + (p1 - q1) + 4 as c_int >> 3 as c_int,
            -tc,
            tc,
        );
        *pix.offset((-1 as intptr_t * xstride) as isize) = x264_clip_pixel(p0 + delta);
        *pix.offset((0 as intptr_t * xstride) as isize) = x264_clip_pixel(q0 - delta);
    }
}
#[inline]
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn deblock_luma_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut ystride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        if (*tc0.offset(i as isize) as c_int) < 0 as c_int {
            pix = pix.offset((4 as intptr_t * ystride) as isize);
        } else {
            let mut d: c_int = 0 as c_int;
            while d < 4 as c_int {
                deblock_edge_luma_c(pix, xstride, alpha, beta, *tc0.offset(i as isize));
                d += 1;
                pix = pix.offset(ystride as isize);
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "123:1"]
unsafe extern "C" fn deblock_h_luma_mbaff_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    let mut d: c_int = 0 as c_int;
    while d < 8 as c_int {
        deblock_edge_luma_c(
            pix,
            1 as intptr_t,
            alpha,
            beta,
            *tc0.offset((d >> 1 as c_int) as isize),
        );
        d += 1;
        pix = pix.offset(stride as isize);
    }
}
#[c2rust::src_loc = "128:1"]
unsafe extern "C" fn deblock_v_luma_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    deblock_luma_c(pix, stride, 1 as intptr_t, alpha, beta, tc0);
}
#[c2rust::src_loc = "132:1"]
unsafe extern "C" fn deblock_h_luma_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    deblock_luma_c(pix, 1 as intptr_t, stride, alpha, beta, tc0);
}
#[inline(always)]
#[c2rust::src_loc = "137:1"]
unsafe extern "C" fn deblock_edge_chroma_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc: int8_t,
) {
    let mut p1: c_int = *pix.offset((-(2 as c_int) as intptr_t * xstride) as isize) as c_int;
    let mut p0: c_int = *pix.offset((-1 as intptr_t * xstride) as isize) as c_int;
    let mut q0: c_int = *pix.offset((0 as intptr_t * xstride) as isize) as c_int;
    let mut q1: c_int = *pix.offset((1 as intptr_t * xstride) as isize) as c_int;
    if abs(p0 - q0) < alpha && abs(p1 - p0) < beta && abs(q1 - q0) < beta {
        let mut delta: c_int = x264_clip3(
            (q0 - p0) * 4 as c_int + (p1 - q1) + 4 as c_int >> 3 as c_int,
            -(tc as c_int),
            tc as c_int,
        );
        *pix.offset((-1 as intptr_t * xstride) as isize) = x264_clip_pixel(p0 + delta);
        *pix.offset((0 as intptr_t * xstride) as isize) = x264_clip_pixel(q0 - delta);
    }
}
#[inline(always)]
#[c2rust::src_loc = "151:1"]
unsafe extern "C" fn deblock_chroma_c(
    mut pix: *mut pixel,
    mut height: c_int,
    mut xstride: intptr_t,
    mut ystride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        let mut tc: c_int = *tc0.offset(i as isize) as c_int;
        if tc <= 0 as c_int {
            pix = pix.offset((height as intptr_t * ystride) as isize);
        } else {
            let mut d: c_int = 0 as c_int;
            while d < height {
                let mut e: c_int = 0 as c_int;
                while e < 2 as c_int {
                    deblock_edge_chroma_c(pix, xstride, alpha, beta, *tc0.offset(i as isize));
                    e += 1;
                    pix = pix.offset(1);
                }
                d += 1;
                pix = pix.offset((ystride - 2 as intptr_t) as isize);
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "166:1"]
unsafe extern "C" fn deblock_h_chroma_mbaff_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    deblock_chroma_c(pix, 1 as c_int, 2 as intptr_t, stride, alpha, beta, tc0);
}
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn deblock_v_chroma_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    deblock_chroma_c(pix, 2 as c_int, stride, 2 as intptr_t, alpha, beta, tc0);
}
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn deblock_h_chroma_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    deblock_chroma_c(pix, 2 as c_int, 2 as intptr_t, stride, alpha, beta, tc0);
}
#[c2rust::src_loc = "178:1"]
unsafe extern "C" fn deblock_h_chroma_422_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
    mut tc0: *mut int8_t,
) {
    deblock_chroma_c(pix, 4 as c_int, 2 as intptr_t, stride, alpha, beta, tc0);
}
#[inline(always)]
#[c2rust::src_loc = "183:1"]
unsafe extern "C" fn deblock_edge_luma_intra_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    let mut p2: c_int = *pix.offset((-(3 as c_int) as intptr_t * xstride) as isize) as c_int;
    let mut p1: c_int = *pix.offset((-(2 as c_int) as intptr_t * xstride) as isize) as c_int;
    let mut p0: c_int = *pix.offset((-1 as intptr_t * xstride) as isize) as c_int;
    let mut q0: c_int = *pix.offset((0 as intptr_t * xstride) as isize) as c_int;
    let mut q1: c_int = *pix.offset((1 as intptr_t * xstride) as isize) as c_int;
    let mut q2: c_int = *pix.offset((2 as intptr_t * xstride) as isize) as c_int;
    if abs(p0 - q0) < alpha && abs(p1 - p0) < beta && abs(q1 - q0) < beta {
        if abs(p0 - q0) < (alpha >> 2 as c_int) + 2 as c_int {
            if abs(p2 - p0) < beta {
                let p3: c_int =
                    *pix.offset((-(4 as c_int) as intptr_t * xstride) as isize) as c_int;
                *pix.offset((-1 as intptr_t * xstride) as isize) =
                    (p2 + 2 as c_int * p1 + 2 as c_int * p0 + 2 as c_int * q0 + q1 + 4 as c_int
                        >> 3 as c_int) as pixel;
                *pix.offset((-(2 as c_int) as intptr_t * xstride) as isize) =
                    (p2 + p1 + p0 + q0 + 2 as c_int >> 2 as c_int) as pixel;
                *pix.offset((-(3 as c_int) as intptr_t * xstride) as isize) =
                    (2 as c_int * p3 + 3 as c_int * p2 + p1 + p0 + q0 + 4 as c_int >> 3 as c_int)
                        as pixel;
            } else {
                *pix.offset((-1 as intptr_t * xstride) as isize) =
                    (2 as c_int * p1 + p0 + q1 + 2 as c_int >> 2 as c_int) as pixel;
            }
            if abs(q2 - q0) < beta {
                let q3: c_int = *pix.offset((3 as intptr_t * xstride) as isize) as c_int;
                *pix.offset((0 as intptr_t * xstride) as isize) =
                    (p1 + 2 as c_int * p0 + 2 as c_int * q0 + 2 as c_int * q1 + q2 + 4 as c_int
                        >> 3 as c_int) as pixel;
                *pix.offset((1 as intptr_t * xstride) as isize) =
                    (p0 + q0 + q1 + q2 + 2 as c_int >> 2 as c_int) as pixel;
                *pix.offset((2 as intptr_t * xstride) as isize) =
                    (2 as c_int * q3 + 3 as c_int * q2 + q1 + q0 + p0 + 4 as c_int >> 3 as c_int)
                        as pixel;
            } else {
                *pix.offset((0 as intptr_t * xstride) as isize) =
                    (2 as c_int * q1 + q0 + p1 + 2 as c_int >> 2 as c_int) as pixel;
            }
        } else {
            *pix.offset((-1 as intptr_t * xstride) as isize) =
                (2 as c_int * p1 + p0 + q1 + 2 as c_int >> 2 as c_int) as pixel;
            *pix.offset((0 as intptr_t * xstride) as isize) =
                (2 as c_int * q1 + q0 + p1 + 2 as c_int >> 2 as c_int) as pixel;
        }
    }
}
#[inline]
#[c2rust::src_loc = "222:1"]
unsafe extern "C" fn deblock_luma_intra_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut ystride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    let mut d: c_int = 0 as c_int;
    while d < 16 as c_int {
        deblock_edge_luma_intra_c(pix, xstride, alpha, beta);
        d += 1;
        pix = pix.offset(ystride as isize);
    }
}
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn deblock_h_luma_intra_mbaff_c(
    mut pix: *mut pixel,
    mut ystride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    let mut d: c_int = 0 as c_int;
    while d < 8 as c_int {
        deblock_edge_luma_intra_c(pix, 1 as intptr_t, alpha, beta);
        d += 1;
        pix = pix.offset(ystride as isize);
    }
}
#[c2rust::src_loc = "232:1"]
unsafe extern "C" fn deblock_v_luma_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    deblock_luma_intra_c(pix, stride, 1 as intptr_t, alpha, beta);
}
#[c2rust::src_loc = "236:1"]
unsafe extern "C" fn deblock_h_luma_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    deblock_luma_intra_c(pix, 1 as intptr_t, stride, alpha, beta);
}
#[inline(always)]
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn deblock_edge_chroma_intra_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    let mut p1: c_int = *pix.offset((-(2 as c_int) as intptr_t * xstride) as isize) as c_int;
    let mut p0: c_int = *pix.offset((-1 as intptr_t * xstride) as isize) as c_int;
    let mut q0: c_int = *pix.offset((0 as intptr_t * xstride) as isize) as c_int;
    let mut q1: c_int = *pix.offset((1 as intptr_t * xstride) as isize) as c_int;
    if abs(p0 - q0) < alpha && abs(p1 - p0) < beta && abs(q1 - q0) < beta {
        *pix.offset((-1 as intptr_t * xstride) as isize) =
            (2 as c_int * p1 + p0 + q1 + 2 as c_int >> 2 as c_int) as pixel;
        *pix.offset((0 as intptr_t * xstride) as isize) =
            (2 as c_int * q1 + q0 + p1 + 2 as c_int >> 2 as c_int) as pixel;
    }
}
#[inline(always)]
#[c2rust::src_loc = "254:1"]
unsafe extern "C" fn deblock_chroma_intra_c(
    mut pix: *mut pixel,
    mut width: c_int,
    mut height: c_int,
    mut xstride: intptr_t,
    mut ystride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    let mut d: c_int = 0 as c_int;
    while d < height {
        let mut e: c_int = 0 as c_int;
        while e < width {
            deblock_edge_chroma_intra_c(pix, xstride, alpha, beta);
            e += 1;
            pix = pix.offset(1);
        }
        d += 1;
        pix = pix.offset((ystride - 2 as intptr_t) as isize);
    }
}
#[c2rust::src_loc = "260:1"]
unsafe extern "C" fn deblock_h_chroma_intra_mbaff_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    deblock_chroma_intra_c(
        pix,
        2 as c_int,
        4 as c_int,
        2 as intptr_t,
        stride,
        alpha,
        beta,
    );
}
#[c2rust::src_loc = "264:1"]
unsafe extern "C" fn deblock_v_chroma_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    deblock_chroma_intra_c(
        pix,
        1 as c_int,
        16 as c_int,
        stride,
        2 as intptr_t,
        alpha,
        beta,
    );
}
#[c2rust::src_loc = "268:1"]
unsafe extern "C" fn deblock_h_chroma_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    deblock_chroma_intra_c(
        pix,
        2 as c_int,
        8 as c_int,
        2 as intptr_t,
        stride,
        alpha,
        beta,
    );
}
#[c2rust::src_loc = "272:1"]
unsafe extern "C" fn deblock_h_chroma_422_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: c_int,
    mut beta: c_int,
) {
    deblock_chroma_intra_c(
        pix,
        2 as c_int,
        16 as c_int,
        2 as intptr_t,
        stride,
        alpha,
        beta,
    );
}
#[c2rust::src_loc = "277:1"]
unsafe extern "C" fn deblock_strength_c(
    mut nnz: *mut uint8_t,
    mut ref_0: *mut [int8_t; 40],
    mut mv: *mut [[int16_t; 2]; 40],
    mut bs: *mut [[uint8_t; 4]; 8],
    mut mvy_limit: c_int,
    mut bframe: c_int,
) {
    let mut dir: c_int = 0 as c_int;
    while dir < 2 as c_int {
        let mut s1: c_int = if dir != 0 { 1 as c_int } else { 8 as c_int };
        let mut s2: c_int = if dir != 0 { 8 as c_int } else { 1 as c_int };
        let mut edge: c_int = 0 as c_int;
        while edge < 4 as c_int {
            let mut i: c_int = 0 as c_int;
            let mut loc: c_int = X264_SCAN8_0 + edge * s2;
            while i < 4 as c_int {
                let mut locn: c_int = loc - s2;
                if *nnz.offset(loc as isize) as c_int != 0
                    || *nnz.offset(locn as isize) as c_int != 0
                {
                    (*bs.offset(dir as isize))[edge as usize][i as usize] = 2 as uint8_t;
                } else if (*ref_0.offset(0))[loc as usize] as c_int
                    != (*ref_0.offset(0))[locn as usize] as c_int
                    || abs((*mv.offset(0))[loc as usize][0] as c_int
                        - (*mv.offset(0))[locn as usize][0] as c_int)
                        >= 4 as c_int
                    || abs((*mv.offset(0))[loc as usize][1] as c_int
                        - (*mv.offset(0))[locn as usize][1] as c_int)
                        >= mvy_limit
                    || bframe != 0
                        && ((*ref_0.offset(1))[loc as usize] as c_int
                            != (*ref_0.offset(1))[locn as usize] as c_int
                            || abs((*mv.offset(1))[loc as usize][0] as c_int
                                - (*mv.offset(1))[locn as usize][0] as c_int)
                                >= 4 as c_int
                            || abs((*mv.offset(1))[loc as usize][1] as c_int
                                - (*mv.offset(1))[locn as usize][1] as c_int)
                                >= mvy_limit)
                {
                    (*bs.offset(dir as isize))[edge as usize][i as usize] = 1 as uint8_t;
                } else {
                    (*bs.offset(dir as isize))[edge as usize][i as usize] = 0 as uint8_t;
                }
                i += 1;
                loc += s1;
            }
            edge += 1;
        }
        dir += 1;
    }
}
#[inline(always)]
#[c2rust::src_loc = "306:1"]
unsafe extern "C" fn deblock_edge(
    mut _h: *mut x264_t,
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
    mut bS: *mut uint8_t,
    mut i_qp: c_int,
    mut a: c_int,
    mut b: c_int,
    mut b_chroma: c_int,
    mut pf_inter: x264_deblock_inter_t,
) {
    let mut index_a: c_int = i_qp + a;
    let mut index_b: c_int = i_qp + b;
    let mut alpha: c_int =
        (i_alpha_table[(index_a + 24 as c_int) as usize] as c_int) << BIT_DEPTH - 8 as c_int;
    let mut beta: c_int =
        (i_beta_table[(index_b + 24 as c_int) as usize] as c_int) << BIT_DEPTH - 8 as c_int;
    let mut tc: [int8_t; 4] = [0; 4];
    if (*(bS as *mut x264_union32_t)).i == 0 || alpha == 0 || beta == 0 {
        return;
    }
    tc[0] = (i_tc0_table[(index_a + 24 as c_int) as usize][*bS.offset(0) as usize] as c_int
        * ((1 as c_int) << BIT_DEPTH - 8 as c_int)
        + b_chroma) as int8_t;
    tc[1] = (i_tc0_table[(index_a + 24 as c_int) as usize][*bS.offset(1) as usize] as c_int
        * ((1 as c_int) << BIT_DEPTH - 8 as c_int)
        + b_chroma) as int8_t;
    tc[2] = (i_tc0_table[(index_a + 24 as c_int) as usize][*bS.offset(2) as usize] as c_int
        * ((1 as c_int) << BIT_DEPTH - 8 as c_int)
        + b_chroma) as int8_t;
    tc[3] = (i_tc0_table[(index_a + 24 as c_int) as usize][*bS.offset(3) as usize] as c_int
        * ((1 as c_int) << BIT_DEPTH - 8 as c_int)
        + b_chroma) as int8_t;
    pf_inter.expect("non-null function pointer")(pix, i_stride, alpha, beta, tc.as_mut_ptr());
}
#[inline(always)]
#[c2rust::src_loc = "326:1"]
unsafe extern "C" fn deblock_edge_intra(
    mut _h: *mut x264_t,
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
    mut _bS: *mut uint8_t,
    mut i_qp: c_int,
    mut a: c_int,
    mut b: c_int,
    mut _b_chroma: c_int,
    mut pf_intra: x264_deblock_intra_t,
) {
    let mut index_a: c_int = i_qp + a;
    let mut index_b: c_int = i_qp + b;
    let mut alpha: c_int =
        (i_alpha_table[(index_a + 24 as c_int) as usize] as c_int) << BIT_DEPTH - 8 as c_int;
    let mut beta: c_int =
        (i_beta_table[(index_b + 24 as c_int) as usize] as c_int) << BIT_DEPTH - 8 as c_int;
    if alpha == 0 || beta == 0 {
        return;
    }
    pf_intra.expect("non-null function pointer")(pix, i_stride, alpha, beta);
}
#[inline(always)]
#[c2rust::src_loc = "340:1"]
unsafe extern "C" fn macroblock_cache_load_neighbours_deblock(
    mut h: *mut x264_t,
    mut mb_x: c_int,
    mut mb_y: c_int,
) {
    let mut deblock_on_slice_edges: c_int =
        ((*h).sh.i_disable_deblocking_filter_idc != 2 as c_int) as c_int;
    (*h).mb.i_neighbour = 0 as c_uint;
    (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
    (*h).mb.interlaced =
        (*h).param.interlaced && *(*h).mb.field.offset((*h).mb.i_mb_xy as isize) != 0;
    (*h).mb.i_mb_top_y = mb_y - (1 << (*h).mb.interlaced as i32);
    (*h).mb.i_mb_top_xy = mb_x + (*h).mb.i_mb_stride * (*h).mb.i_mb_top_y;
    (*h).mb.i_mb_left_xy[0] = (*h).mb.i_mb_xy - 1 as c_int;
    (*h).mb.i_mb_left_xy[1] = (*h).mb.i_mb_left_xy[0];
    if (*h).sh.mbaff {
        if mb_y & 1 as c_int != 0 {
            if mb_x != 0
                && *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1) as isize) != (*h).mb.interlaced as u8
            {
                (*h).mb.i_mb_left_xy[0] -= (*h).mb.i_mb_stride;
            }
        } else {
            if (*h).mb.i_mb_top_xy >= 0 as c_int
                && (*h).mb.interlaced
                && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) == 0
            {
                (*h).mb.i_mb_top_xy += (*h).mb.i_mb_stride;
                (*h).mb.i_mb_top_y += 1;
            }
            if mb_x != 0
                && *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1) as isize) != (*h).mb.interlaced as u8
            {
                (*h).mb.i_mb_left_xy[1] += (*h).mb.i_mb_stride;
            }
        }
    }
    if mb_x > 0 as c_int
        && (deblock_on_slice_edges != 0
            || *(*h).mb.slice_table.offset((*h).mb.i_mb_left_xy[0] as isize)
                == *(*h).mb.slice_table.offset((*h).mb.i_mb_xy as isize))
    {
        (*h).mb.i_neighbour |= MB_LEFT as c_int as c_uint;
    }
    if mb_y > (*h).mb.interlaced as i32
        && (deblock_on_slice_edges != 0
            || *(*h).mb.slice_table.offset((*h).mb.i_mb_top_xy as isize)
                == *(*h).mb.slice_table.offset((*h).mb.i_mb_xy as isize))
    {
        (*h).mb.i_neighbour |= MB_TOP as c_int as c_uint;
    }
}
#[no_mangle]
#[c2rust::src_loc = "378:1"]
unsafe extern "C" fn x264_10_frame_deblock_row(mut h: *mut x264_t, mut mb_y: c_int) {
    let mut interlaced = (*h).sh.mbaff;
    let mut a: c_int = (*h).sh.i_alpha_c0_offset - QP_BD_OFFSET;
    let mut b: c_int = (*h).sh.i_beta_offset - QP_BD_OFFSET;
    let mut qp_thresh: c_int = 15 as c_int
        - (if a < b { a } else { b })
        - (if 0 as c_int > (*(*h).pps.as_mut_ptr()).i_chroma_qp_index_offset {
            0 as c_int
        } else {
            (*(*h).pps.as_mut_ptr()).i_chroma_qp_index_offset
        });
    let mut stridey: c_int = (*(*h).fdec).i_stride[0];
    let mut strideuv: c_int = (*(*h).fdec).i_stride[1];
    let mut chroma_format: c_int = (*(*h).sps.as_mut_ptr()).i_chroma_format_idc;
    let mut chroma444: c_int =
        ((*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int) as c_int;
    let mut chroma_height: c_int = 16 as c_int >> (*h).mb.chroma_v_shift;
    let mut uvdiff: intptr_t = if chroma444 != 0 {
        (*(*h).fdec).plane[2].offset_from((*(*h).fdec).plane[1]) as intptr_t
    } else {
        1 as intptr_t
    };
    let mut mb_x: c_int = 0 as c_int;
    while mb_x < (*h).mb.i_mb_width {
        x264_10_prefetch_fenc(h, (*h).fdec, mb_x, mb_y);
        macroblock_cache_load_neighbours_deblock(h, mb_x, mb_y);
        let mut mb_xy: c_int = (*h).mb.i_mb_xy;
        let mut transform_8x8: c_int = *(*h).mb.mb_transform_size.offset(mb_xy as isize) as c_int;
        let mut intra_cur: c_int = (*(*h).mb.type_0.offset(mb_xy as isize) as c_int
            == I_4x4 as c_int
            || *(*h).mb.type_0.offset(mb_xy as isize) as c_int == I_8x8 as c_int
            || *(*h).mb.type_0.offset(mb_xy as isize) as c_int == I_16x16 as c_int
            || *(*h).mb.type_0.offset(mb_xy as isize) as c_int == I_PCM as c_int)
            as c_int;
        let mut bs: *mut [[uint8_t; 4]; 8] = (*(*(*h)
            .deblock_strength
            .as_mut_ptr()
            .offset((mb_y & 1 as c_int) as isize))
        .offset(
            (if (*h).param.sliced_threads {
                mb_xy
            } else {
                mb_x
            }) as isize,
        ))
        .as_mut_ptr() as *mut [[uint8_t; 4]; 8];
        let mut pixy: *mut pixel = (*(*h).fdec).plane[0]
            .offset((16 as c_int * mb_y * stridey) as isize)
            .offset((16 as c_int * mb_x) as isize);
        let mut pixuv: *mut pixel = if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*(*h).fdec).plane[1]
                .offset((chroma_height * mb_y * strideuv) as isize)
                .offset((16 as c_int * mb_x) as isize)
        } else {
            0 as *mut pixel
        };
        if mb_y & (*h).mb.interlaced as i32 != 0 {
            pixy = pixy.offset(-((15 as c_int * stridey) as isize));
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                pixuv = pixuv.offset(-(((chroma_height - 1 as c_int) * strideuv) as isize));
            }
        }
        let mut stride2y: c_int = stridey << (*h).mb.interlaced as i32;
        let mut stride2uv: c_int = strideuv << (*h).mb.interlaced as i32;
        let mut qp: c_int = *(*h).mb.qp.offset(mb_xy as isize) as c_int;
        let mut qpc: c_int = *(*h).chroma_qp_table.offset(qp as isize) as c_int;
        let mut first_edge_only: c_int = (*(*h).mb.partition.offset(mb_xy as isize) as c_int
            == D_16x16 as c_int
            && *(*h).mb.cbp.offset(mb_xy as isize) == 0
            && intra_cur == 0
            || qp <= qp_thresh) as c_int;
        if (*h).mb.i_neighbour & MB_LEFT != 0 {
            if interlaced
                && *(*h).mb.field.offset((*h).mb.i_mb_left_xy[0] as isize)
                    != (*h).mb.interlaced as u8
            {
                let mut luma_qp: [c_int; 2] = [0; 2];
                let mut chroma_qp: [c_int; 2] = [0; 2];
                let mut left_qp: [c_int; 2] = [0; 2];
                let mut luma_deblock: x264_deblock_inter_t = (*h).loopf.deblock_luma_mbaff;
                let mut chroma_deblock: x264_deblock_inter_t = (*h).loopf.deblock_chroma_mbaff;
                let mut luma_intra_deblock: x264_deblock_intra_t =
                    (*h).loopf.deblock_luma_intra_mbaff;
                let mut chroma_intra_deblock: x264_deblock_intra_t =
                    (*h).loopf.deblock_chroma_intra_mbaff;
                let mut c: c_int = if chroma444 != 0 {
                    0 as c_int
                } else {
                    1 as c_int
                };
                left_qp[0] = *(*h).mb.qp.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int;
                luma_qp[0] = qp + left_qp[0] + 1 as c_int >> 1 as c_int;
                chroma_qp[0] =
                    qpc + *(*h).chroma_qp_table.offset(left_qp[0] as isize) as c_int + 1 as c_int
                        >> 1 as c_int;
                if intra_cur != 0
                    || (*(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int
                        == I_4x4 as c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int
                            == I_8x8 as c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int
                            == I_16x16 as c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0] as isize) as c_int
                            == I_PCM as c_int)
                {
                    deblock_edge_intra(
                        h,
                        pixy,
                        (2 as c_int * stridey) as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                        luma_qp[0],
                        a,
                        b,
                        0 as c_int,
                        luma_intra_deblock,
                    );
                    if chroma_format != 0 {
                        deblock_edge_intra(
                            h,
                            pixuv,
                            (2 as c_int * strideuv) as intptr_t,
                            (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            chroma_qp[0],
                            a,
                            b,
                            c,
                            chroma_intra_deblock,
                        );
                        if chroma444 != 0 {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(uvdiff as isize),
                                (2 as c_int * strideuv) as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                chroma_qp[0],
                                a,
                                b,
                                c,
                                chroma_intra_deblock,
                            );
                        }
                    }
                } else {
                    deblock_edge(
                        h,
                        pixy,
                        (2 as c_int * stridey) as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                        luma_qp[0],
                        a,
                        b,
                        0 as c_int,
                        luma_deblock,
                    );
                    if chroma_format != 0 {
                        deblock_edge(
                            h,
                            pixuv,
                            (2 as c_int * strideuv) as intptr_t,
                            (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            chroma_qp[0],
                            a,
                            b,
                            c,
                            chroma_deblock,
                        );
                        if chroma444 != 0 {
                            deblock_edge(
                                h,
                                pixuv.offset(uvdiff as isize),
                                (2 as c_int * strideuv) as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                chroma_qp[0],
                                a,
                                b,
                                c,
                                chroma_deblock,
                            );
                        }
                    }
                }
                let mut offy = match (*h).mb.interlaced {
                    true => 4,
                    false => 0,
                };
                let mut offuv = match (*h).mb.interlaced {
                    true => 4 - (*h).mb.chroma_v_shift,
                    false => 0,
                };
                left_qp[1] = *(*h).mb.qp.offset((*h).mb.i_mb_left_xy[1] as isize) as c_int;
                luma_qp[1] = qp + left_qp[1] + 1 as c_int >> 1 as c_int;
                chroma_qp[1] =
                    qpc + *(*h).chroma_qp_table.offset(left_qp[1] as isize) as c_int + 1 as c_int
                        >> 1 as c_int;
                if intra_cur != 0
                    || (*(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1] as isize) as c_int
                        == I_4x4 as c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1] as isize) as c_int
                            == I_8x8 as c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1] as isize) as c_int
                            == I_16x16 as c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1] as isize) as c_int
                            == I_PCM as c_int)
                {
                    deblock_edge_intra(
                        h,
                        pixy.offset((stridey << offy) as isize),
                        (2 as c_int * stridey) as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(4)).as_mut_ptr(),
                        luma_qp[1],
                        a,
                        b,
                        0 as c_int,
                        luma_intra_deblock,
                    );
                    if chroma_format != 0 {
                        deblock_edge_intra(
                            h,
                            pixuv.offset((strideuv << offuv) as isize),
                            (2 as c_int * strideuv) as intptr_t,
                            (*(*bs.offset(0)).as_mut_ptr().offset(4)).as_mut_ptr(),
                            chroma_qp[1],
                            a,
                            b,
                            c,
                            chroma_intra_deblock,
                        );
                        if chroma444 != 0 {
                            deblock_edge_intra(
                                h,
                                pixuv
                                    .offset(uvdiff as isize)
                                    .offset((strideuv << offuv) as isize),
                                (2 as c_int * strideuv) as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(4)).as_mut_ptr(),
                                chroma_qp[1],
                                a,
                                b,
                                c,
                                chroma_intra_deblock,
                            );
                        }
                    }
                } else {
                    deblock_edge(
                        h,
                        pixy.offset((stridey << offy) as isize),
                        (2 as c_int * stridey) as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(4)).as_mut_ptr(),
                        luma_qp[1],
                        a,
                        b,
                        0 as c_int,
                        luma_deblock,
                    );
                    if chroma_format != 0 {
                        deblock_edge(
                            h,
                            pixuv.offset((strideuv << offuv) as isize),
                            (2 as c_int * strideuv) as intptr_t,
                            (*(*bs.offset(0)).as_mut_ptr().offset(4)).as_mut_ptr(),
                            chroma_qp[1],
                            a,
                            b,
                            c,
                            chroma_deblock,
                        );
                        if chroma444 != 0 {
                            deblock_edge(
                                h,
                                pixuv
                                    .offset(uvdiff as isize)
                                    .offset((strideuv << offuv) as isize),
                                (2 as c_int * strideuv) as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(4)).as_mut_ptr(),
                                chroma_qp[1],
                                a,
                                b,
                                c,
                                chroma_deblock,
                            );
                        }
                    }
                }
            } else {
                let mut qpl: c_int =
                    *(*h).mb.qp.offset(((*h).mb.i_mb_xy - 1 as c_int) as isize) as c_int;
                let mut qp_left: c_int = qp + qpl + 1 as c_int >> 1 as c_int;
                let mut qpc_left: c_int =
                    qpc + *(*h).chroma_qp_table.offset(qpl as isize) as c_int + 1 as c_int
                        >> 1 as c_int;
                let mut intra_left: c_int = (*(*h)
                    .mb
                    .type_0
                    .offset(((*h).mb.i_mb_xy - 1 as c_int) as isize)
                    as c_int
                    == I_4x4 as c_int
                    || *(*h)
                        .mb
                        .type_0
                        .offset(((*h).mb.i_mb_xy - 1 as c_int) as isize)
                        as c_int
                        == I_8x8 as c_int
                    || *(*h)
                        .mb
                        .type_0
                        .offset(((*h).mb.i_mb_xy - 1 as c_int) as isize)
                        as c_int
                        == I_16x16 as c_int
                    || *(*h)
                        .mb
                        .type_0
                        .offset(((*h).mb.i_mb_xy - 1 as c_int) as isize)
                        as c_int
                        == I_PCM as c_int) as c_int;
                let mut intra_deblock: c_int = (intra_cur != 0 || intra_left != 0) as c_int;
                if !(*(*h).fdec).mb_info.is_null()
                    && (*((*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr()
                        as *mut x264_union32_t))
                        .i
                        != 0
                {
                    let ref mut fresh0 = *(*(*h).fdec).effective_qp.offset(mb_xy as isize);
                    *fresh0 = (*fresh0 as c_int
                        | 0xff as c_int
                            * (*(*(*h).fdec).mb_info.offset(mb_xy as isize) as c_uint
                                & X264_MBINFO_CONSTANT
                                != 0) as c_int) as uint8_t;
                    let ref mut fresh1 = *(*(*h).fdec)
                        .effective_qp
                        .offset((*h).mb.i_mb_left_xy[0] as isize);
                    *fresh1 = (*fresh1 as c_int
                        | 0xff as c_int
                            * (*(*(*h).fdec)
                                .mb_info
                                .offset((*h).mb.i_mb_left_xy[0] as isize)
                                as c_uint
                                & X264_MBINFO_CONSTANT
                                != 0) as c_int) as uint8_t;
                }
                if intra_deblock != 0 {
                    if 0 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                        deblock_edge_intra(
                            h,
                            pixy.offset(
                                (4 as c_int
                                    * 0 as c_int
                                    * (if 0 as c_int != 0 {
                                        stride2y
                                    } else {
                                        1 as c_int
                                    })) as isize,
                            ),
                            stride2y as intptr_t,
                            (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            qp_left,
                            a,
                            b,
                            0 as c_int,
                            (*h).loopf.deblock_luma_intra[0],
                        );
                        if chroma_format == CHROMA_444 as c_int {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (4 as c_int
                                        * 0 as c_int
                                        * (if 0 as c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                0 as c_int,
                                (*h).loopf.deblock_luma_intra[0],
                            );
                            deblock_edge_intra(
                                h,
                                pixuv.offset(uvdiff as isize).offset(
                                    (4 as c_int
                                        * 0 as c_int
                                        * (if 0 as c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                0 as c_int,
                                (*h).loopf.deblock_luma_intra[0],
                            );
                        } else if chroma_format == CHROMA_420 as c_int
                            && 0 as c_int & 1 as c_int == 0
                        {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (0 as c_int
                                        * (if 0 as c_int != 0 {
                                            2 as c_int * stride2uv
                                        } else {
                                            4 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                1 as c_int,
                                (*h).loopf.deblock_chroma_intra[0],
                            );
                        }
                    }
                    if chroma_format == CHROMA_422 as c_int
                        && (0 as c_int != 0 || 0 as c_int & 1 as c_int == 0)
                    {
                        deblock_edge_intra(
                            h,
                            pixuv.offset(
                                (0 as c_int
                                    * (if 0 as c_int != 0 {
                                        4 as c_int * stride2uv
                                    } else {
                                        4 as c_int
                                    })) as isize,
                            ),
                            stride2uv as intptr_t,
                            (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            qpc_left,
                            a,
                            b,
                            1 as c_int,
                            (*h).loopf.deblock_chroma_intra[0],
                        );
                    }
                } else {
                    if 0 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                        deblock_edge(
                            h,
                            pixy.offset(
                                (4 as c_int
                                    * 0 as c_int
                                    * (if 0 as c_int != 0 {
                                        stride2y
                                    } else {
                                        1 as c_int
                                    })) as isize,
                            ),
                            stride2y as intptr_t,
                            (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            qp_left,
                            a,
                            b,
                            0 as c_int,
                            (*h).loopf.deblock_luma[0],
                        );
                        if chroma_format == CHROMA_444 as c_int {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (4 as c_int
                                        * 0 as c_int
                                        * (if 0 as c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                0 as c_int,
                                (*h).loopf.deblock_luma[0],
                            );
                            deblock_edge(
                                h,
                                pixuv.offset(uvdiff as isize).offset(
                                    (4 as c_int
                                        * 0 as c_int
                                        * (if 0 as c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                0 as c_int,
                                (*h).loopf.deblock_luma[0],
                            );
                        } else if chroma_format == CHROMA_420 as c_int
                            && 0 as c_int & 1 as c_int == 0
                        {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (0 as c_int
                                        * (if 0 as c_int != 0 {
                                            2 as c_int * stride2uv
                                        } else {
                                            4 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                1 as c_int,
                                (*h).loopf.deblock_chroma[0],
                            );
                        }
                    }
                    if chroma_format == CHROMA_422 as c_int
                        && (0 as c_int != 0 || 0 as c_int & 1 as c_int == 0)
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (0 as c_int
                                    * (if 0 as c_int != 0 {
                                        4 as c_int * stride2uv
                                    } else {
                                        4 as c_int
                                    })) as isize,
                            ),
                            stride2uv as intptr_t,
                            (*(*bs.offset(0)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            qpc_left,
                            a,
                            b,
                            1 as c_int,
                            (*h).loopf.deblock_chroma[0],
                        );
                    }
                }
            }
        }
        if first_edge_only == 0 {
            if 1 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as c_int
                            * 1 as c_int
                            * (if 0 as c_int != 0 {
                                stride2y
                            } else {
                                1 as c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    (*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as c_int,
                    (*h).loopf.deblock_luma[0],
                );
                if chroma_format == CHROMA_444 as c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as c_int
                                * 1 as c_int
                                * (if 0 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[0],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as c_int
                                * 1 as c_int
                                * (if 0 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[0],
                    );
                } else if chroma_format == CHROMA_420 as c_int && 1 as c_int & 1 as c_int == 0 {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (1 as c_int
                                * (if 0 as c_int != 0 {
                                    2 as c_int * stride2uv
                                } else {
                                    4 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as c_int,
                        (*h).loopf.deblock_chroma[0],
                    );
                }
            }
            if chroma_format == CHROMA_422 as c_int
                && (0 as c_int != 0 || 1 as c_int & 1 as c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (1 as c_int
                            * (if 0 as c_int != 0 {
                                4 as c_int * stride2uv
                            } else {
                                4 as c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    (*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as c_int,
                    (*h).loopf.deblock_chroma[0],
                );
            }
            if 2 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as c_int
                            * 2 as c_int
                            * (if 0 as c_int != 0 {
                                stride2y
                            } else {
                                1 as c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    (*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as c_int,
                    (*h).loopf.deblock_luma[0],
                );
                if chroma_format == CHROMA_444 as c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as c_int
                                * 2 as c_int
                                * (if 0 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[0],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as c_int
                                * 2 as c_int
                                * (if 0 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[0],
                    );
                } else if chroma_format == CHROMA_420 as c_int && 2 as c_int & 1 as c_int == 0 {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (2 as c_int
                                * (if 0 as c_int != 0 {
                                    2 as c_int * stride2uv
                                } else {
                                    4 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as c_int,
                        (*h).loopf.deblock_chroma[0],
                    );
                }
            }
            if chroma_format == CHROMA_422 as c_int
                && (0 as c_int != 0 || 2 as c_int & 1 as c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (2 as c_int
                            * (if 0 as c_int != 0 {
                                4 as c_int * stride2uv
                            } else {
                                4 as c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    (*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as c_int,
                    (*h).loopf.deblock_chroma[0],
                );
            }
            if 3 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as c_int
                            * 3 as c_int
                            * (if 0 as c_int != 0 {
                                stride2y
                            } else {
                                1 as c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    (*(*bs.offset(0)).as_mut_ptr().offset(3)).as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as c_int,
                    (*h).loopf.deblock_luma[0],
                );
                if chroma_format == CHROMA_444 as c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as c_int
                                * 3 as c_int
                                * (if 0 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(3)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[0],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as c_int
                                * 3 as c_int
                                * (if 0 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(3)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[0],
                    );
                } else if chroma_format == CHROMA_420 as c_int && 3 as c_int & 1 as c_int == 0 {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (3 as c_int
                                * (if 0 as c_int != 0 {
                                    2 as c_int * stride2uv
                                } else {
                                    4 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(0)).as_mut_ptr().offset(3)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as c_int,
                        (*h).loopf.deblock_chroma[0],
                    );
                }
            }
            if chroma_format == CHROMA_422 as c_int
                && (0 as c_int != 0 || 3 as c_int & 1 as c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (3 as c_int
                            * (if 0 as c_int != 0 {
                                4 as c_int * stride2uv
                            } else {
                                4 as c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    (*(*bs.offset(0)).as_mut_ptr().offset(3)).as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as c_int,
                    (*h).loopf.deblock_chroma[0],
                );
            }
        }
        if (*h).mb.i_neighbour & MB_TOP != 0 {
            if interlaced
                && mb_y & 1 == 0
                && !(*h).mb.interlaced
                && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) != 0
            {
                let mut mbn_xy: c_int = mb_xy - 2 as c_int * (*h).mb.i_mb_stride;
                let mut j: c_int = 0 as c_int;
                while j < 2 as c_int {
                    let mut qpt: c_int = *(*h).mb.qp.offset(mbn_xy as isize) as c_int;
                    let mut qp_top: c_int = qp + qpt + 1 as c_int >> 1 as c_int;
                    let mut qpc_top: c_int =
                        qpc + *(*h).chroma_qp_table.offset(qpt as isize) as c_int + 1 as c_int
                            >> 1 as c_int;
                    let mut intra_top: c_int = (*(*h).mb.type_0.offset(mbn_xy as isize) as c_int
                        == I_4x4 as c_int
                        || *(*h).mb.type_0.offset(mbn_xy as isize) as c_int == I_8x8 as c_int
                        || *(*h).mb.type_0.offset(mbn_xy as isize) as c_int == I_16x16 as c_int
                        || *(*h).mb.type_0.offset(mbn_xy as isize) as c_int == I_PCM as c_int)
                        as c_int;
                    if intra_cur != 0 || intra_top != 0 {
                        (*((*(*bs.offset(1))
                            .as_mut_ptr()
                            .offset((4 as c_int * j) as isize))
                        .as_mut_ptr() as *mut x264_union32_t))
                            .i = 0x3030303 as uint32_t;
                    }
                    deblock_edge(
                        h,
                        pixy.offset((j * stridey) as isize),
                        (2 as c_int * stridey) as intptr_t,
                        (*(*bs.offset(1))
                            .as_mut_ptr()
                            .offset((4 as c_int * j) as isize))
                        .as_mut_ptr(),
                        qp_top,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[1],
                    );
                    if chroma444 != 0 {
                        deblock_edge(
                            h,
                            pixuv.offset((j * strideuv) as isize),
                            (2 as c_int * strideuv) as intptr_t,
                            (*(*bs.offset(1))
                                .as_mut_ptr()
                                .offset((4 as c_int * j) as isize))
                            .as_mut_ptr(),
                            qpc_top,
                            a,
                            b,
                            0 as c_int,
                            (*h).loopf.deblock_luma[1],
                        );
                        deblock_edge(
                            h,
                            pixuv
                                .offset(uvdiff as isize)
                                .offset((j * strideuv) as isize),
                            (2 as c_int * strideuv) as intptr_t,
                            (*(*bs.offset(1))
                                .as_mut_ptr()
                                .offset((4 as c_int * j) as isize))
                            .as_mut_ptr(),
                            qpc_top,
                            a,
                            b,
                            0 as c_int,
                            (*h).loopf.deblock_luma[1],
                        );
                    } else if chroma_format != 0 {
                        deblock_edge(
                            h,
                            pixuv.offset((j * strideuv) as isize),
                            (2 as c_int * strideuv) as intptr_t,
                            (*(*bs.offset(1))
                                .as_mut_ptr()
                                .offset((4 as c_int * j) as isize))
                            .as_mut_ptr(),
                            qpc_top,
                            a,
                            b,
                            1 as c_int,
                            (*h).loopf.deblock_chroma[1],
                        );
                    }
                    j += 1;
                    mbn_xy += (*h).mb.i_mb_stride;
                }
            } else {
                let mut qpt_0: c_int = *(*h).mb.qp.offset((*h).mb.i_mb_top_xy as isize) as c_int;
                let mut qp_top_0: c_int = qp + qpt_0 + 1 as c_int >> 1 as c_int;
                let mut qpc_top_0: c_int =
                    qpc + *(*h).chroma_qp_table.offset(qpt_0 as isize) as c_int + 1 as c_int
                        >> 1 as c_int;
                let mut intra_top_0: c_int = (*(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize)
                    as c_int
                    == I_4x4 as c_int
                    || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize) as c_int
                        == I_8x8 as c_int
                    || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize) as c_int
                        == I_16x16 as c_int
                    || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize) as c_int
                        == I_PCM as c_int) as c_int;
                let mut intra_deblock_0: c_int = (intra_cur != 0 || intra_top_0 != 0) as c_int;
                if !(*(*h).fdec).mb_info.is_null()
                    && (*((*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr()
                        as *mut x264_union32_t))
                        .i
                        != 0
                {
                    let ref mut fresh2 = *(*(*h).fdec).effective_qp.offset(mb_xy as isize);
                    *fresh2 = (*fresh2 as c_int
                        | 0xff as c_int
                            * (*(*(*h).fdec).mb_info.offset(mb_xy as isize) as c_uint
                                & X264_MBINFO_CONSTANT
                                != 0) as c_int) as uint8_t;
                    let ref mut fresh3 = *(*(*h).fdec)
                        .effective_qp
                        .offset((*h).mb.i_mb_top_xy as isize);
                    *fresh3 = (*fresh3 as c_int
                        | 0xff as c_int
                            * (*(*(*h).fdec).mb_info.offset((*h).mb.i_mb_top_xy as isize) as c_uint
                                & X264_MBINFO_CONSTANT
                                != 0) as c_int) as uint8_t;
                }
                if (!interlaced
                    || !(*h).mb.interlaced
                        && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) == 0)
                    && intra_deblock_0 != 0
                {
                    if 0 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                        deblock_edge_intra(
                            h,
                            pixy.offset(
                                (4 as c_int
                                    * 0 as c_int
                                    * (if 1 as c_int != 0 {
                                        stride2y
                                    } else {
                                        1 as c_int
                                    })) as isize,
                            ),
                            stride2y as intptr_t,
                            (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            qp_top_0,
                            a,
                            b,
                            0 as c_int,
                            (*h).loopf.deblock_luma_intra[1],
                        );
                        if chroma_format == CHROMA_444 as c_int {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (4 as c_int
                                        * 0 as c_int
                                        * (if 1 as c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                0 as c_int,
                                (*h).loopf.deblock_luma_intra[1],
                            );
                            deblock_edge_intra(
                                h,
                                pixuv.offset(uvdiff as isize).offset(
                                    (4 as c_int
                                        * 0 as c_int
                                        * (if 1 as c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                0 as c_int,
                                (*h).loopf.deblock_luma_intra[1],
                            );
                        } else if chroma_format == CHROMA_420 as c_int
                            && 0 as c_int & 1 as c_int == 0
                        {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (0 as c_int
                                        * (if 1 as c_int != 0 {
                                            2 as c_int * stride2uv
                                        } else {
                                            4 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                1 as c_int,
                                (*h).loopf.deblock_chroma_intra[1],
                            );
                        }
                    }
                    if chroma_format == CHROMA_422 as c_int
                        && (1 as c_int != 0 || 0 as c_int & 1 as c_int == 0)
                    {
                        deblock_edge_intra(
                            h,
                            pixuv.offset(
                                (0 as c_int
                                    * (if 1 as c_int != 0 {
                                        4 as c_int * stride2uv
                                    } else {
                                        4 as c_int
                                    })) as isize,
                            ),
                            stride2uv as intptr_t,
                            (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            qpc_top_0,
                            a,
                            b,
                            1 as c_int,
                            (*h).loopf.deblock_chroma_intra[1],
                        );
                    }
                } else {
                    if intra_deblock_0 != 0 {
                        (*((*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr()
                            as *mut x264_union32_t))
                            .i = 0x3030303 as uint32_t;
                    }
                    if 0 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                        deblock_edge(
                            h,
                            pixy.offset(
                                (4 as c_int
                                    * 0 as c_int
                                    * (if 1 as c_int != 0 {
                                        stride2y
                                    } else {
                                        1 as c_int
                                    })) as isize,
                            ),
                            stride2y as intptr_t,
                            (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            qp_top_0,
                            a,
                            b,
                            0 as c_int,
                            (*h).loopf.deblock_luma[1],
                        );
                        if chroma_format == CHROMA_444 as c_int {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (4 as c_int
                                        * 0 as c_int
                                        * (if 1 as c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                0 as c_int,
                                (*h).loopf.deblock_luma[1],
                            );
                            deblock_edge(
                                h,
                                pixuv.offset(uvdiff as isize).offset(
                                    (4 as c_int
                                        * 0 as c_int
                                        * (if 1 as c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                0 as c_int,
                                (*h).loopf.deblock_luma[1],
                            );
                        } else if chroma_format == CHROMA_420 as c_int
                            && 0 as c_int & 1 as c_int == 0
                        {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (0 as c_int
                                        * (if 1 as c_int != 0 {
                                            2 as c_int * stride2uv
                                        } else {
                                            4 as c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                1 as c_int,
                                (*h).loopf.deblock_chroma[1],
                            );
                        }
                    }
                    if chroma_format == CHROMA_422 as c_int
                        && (1 as c_int != 0 || 0 as c_int & 1 as c_int == 0)
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (0 as c_int
                                    * (if 1 as c_int != 0 {
                                        4 as c_int * stride2uv
                                    } else {
                                        4 as c_int
                                    })) as isize,
                            ),
                            stride2uv as intptr_t,
                            (*(*bs.offset(1)).as_mut_ptr().offset(0)).as_mut_ptr(),
                            qpc_top_0,
                            a,
                            b,
                            1 as c_int,
                            (*h).loopf.deblock_chroma[1],
                        );
                    }
                }
            }
        }
        if first_edge_only == 0 {
            if 1 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as c_int
                            * 1 as c_int
                            * (if 1 as c_int != 0 {
                                stride2y
                            } else {
                                1 as c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    (*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as c_int,
                    (*h).loopf.deblock_luma[1],
                );
                if chroma_format == CHROMA_444 as c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as c_int
                                * 1 as c_int
                                * (if 1 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[1],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as c_int
                                * 1 as c_int
                                * (if 1 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[1],
                    );
                } else if chroma_format == CHROMA_420 as c_int && 1 as c_int & 1 as c_int == 0 {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (1 as c_int
                                * (if 1 as c_int != 0 {
                                    2 as c_int * stride2uv
                                } else {
                                    4 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as c_int,
                        (*h).loopf.deblock_chroma[1],
                    );
                }
            }
            if chroma_format == CHROMA_422 as c_int
                && (1 as c_int != 0 || 1 as c_int & 1 as c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (1 as c_int
                            * (if 1 as c_int != 0 {
                                4 as c_int * stride2uv
                            } else {
                                4 as c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    (*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as c_int,
                    (*h).loopf.deblock_chroma[1],
                );
            }
            if 2 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as c_int
                            * 2 as c_int
                            * (if 1 as c_int != 0 {
                                stride2y
                            } else {
                                1 as c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    (*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as c_int,
                    (*h).loopf.deblock_luma[1],
                );
                if chroma_format == CHROMA_444 as c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as c_int
                                * 2 as c_int
                                * (if 1 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[1],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as c_int
                                * 2 as c_int
                                * (if 1 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[1],
                    );
                } else if chroma_format == CHROMA_420 as c_int && 2 as c_int & 1 as c_int == 0 {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (2 as c_int
                                * (if 1 as c_int != 0 {
                                    2 as c_int * stride2uv
                                } else {
                                    4 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as c_int,
                        (*h).loopf.deblock_chroma[1],
                    );
                }
            }
            if chroma_format == CHROMA_422 as c_int
                && (1 as c_int != 0 || 2 as c_int & 1 as c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (2 as c_int
                            * (if 1 as c_int != 0 {
                                4 as c_int * stride2uv
                            } else {
                                4 as c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    (*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as c_int,
                    (*h).loopf.deblock_chroma[1],
                );
            }
            if 3 as c_int & 1 as c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as c_int
                            * 3 as c_int
                            * (if 1 as c_int != 0 {
                                stride2y
                            } else {
                                1 as c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    (*(*bs.offset(1)).as_mut_ptr().offset(3)).as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as c_int,
                    (*h).loopf.deblock_luma[1],
                );
                if chroma_format == CHROMA_444 as c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as c_int
                                * 3 as c_int
                                * (if 1 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(3)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[1],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as c_int
                                * 3 as c_int
                                * (if 1 as c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(3)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as c_int,
                        (*h).loopf.deblock_luma[1],
                    );
                } else if chroma_format == CHROMA_420 as c_int && 3 as c_int & 1 as c_int == 0 {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (3 as c_int
                                * (if 1 as c_int != 0 {
                                    2 as c_int * stride2uv
                                } else {
                                    4 as c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        (*(*bs.offset(1)).as_mut_ptr().offset(3)).as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as c_int,
                        (*h).loopf.deblock_chroma[1],
                    );
                }
            }
            if chroma_format == CHROMA_422 as c_int
                && (1 as c_int != 0 || 3 as c_int & 1 as c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (3 as c_int
                            * (if 1 as c_int != 0 {
                                4 as c_int * stride2uv
                            } else {
                                4 as c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    (*(*bs.offset(1)).as_mut_ptr().offset(3)).as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as c_int,
                    (*h).loopf.deblock_chroma[1],
                );
            }
        }
        mb_x += (!interlaced as i32 | mb_y) & 1;
        mb_y ^= interlaced as i32;
    }
}
#[no_mangle]
#[c2rust::src_loc = "615:1"]
unsafe extern "C" fn x264_10_macroblock_deblock(mut h: *mut x264_t) {
    let mut a: c_int = (*h).sh.i_alpha_c0_offset - QP_BD_OFFSET;
    let mut b: c_int = (*h).sh.i_beta_offset - QP_BD_OFFSET;
    let mut qp_thresh: c_int = 15 as c_int
        - (if a < b { a } else { b })
        - (if 0 as c_int > (*(*h).pps.as_mut_ptr()).i_chroma_qp_index_offset {
            0 as c_int
        } else {
            (*(*h).pps.as_mut_ptr()).i_chroma_qp_index_offset
        });
    let mut intra_cur: c_int = ((*h).mb.i_type == I_4x4 as c_int
        || (*h).mb.i_type == I_8x8 as c_int
        || (*h).mb.i_type == I_16x16 as c_int
        || (*h).mb.i_type == I_PCM as c_int) as c_int;
    let mut qp: c_int = (*h).mb.i_qp;
    let mut qpc: c_int = (*h).mb.i_chroma_qp;
    if (*h).mb.i_partition == D_16x16 as c_int && (*h).mb.i_cbp_luma == 0 && intra_cur == 0
        || qp <= qp_thresh
    {
        return;
    }
    let mut bs: *mut [[uint8_t; 4]; 8] = (*h).mb.cache.deblock_strength;
    if intra_cur != 0 {
        (*((*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr() as *mut x264_union32_t)).i =
            0x3030303 as uint32_t;
        (*((*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr() as *mut x264_union64_t)).i =
            0x303030303030303 as uint64_t;
        (*((*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr() as *mut x264_union32_t)).i =
            0x3030303 as uint32_t;
        (*((*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr() as *mut x264_union64_t)).i =
            0x303030303030303 as uint64_t;
    } else {
        (*h).loopf
            .deblock_strength
            .expect("non-null function pointer")(
            (*h).mb.cache.non_zero_count.as_mut_ptr(),
            (*h).mb.cache.ref_0.as_mut_ptr(),
            (*h).mb.cache.mv.as_mut_ptr(),
            bs as *mut [[uint8_t; 4]; 8],
            4 >> (*h).mb.interlaced as i32,
            ((*h).sh.i_type == SLICE_TYPE_B as c_int) as c_int,
        );
    }
    let mut transform_8x8: c_int = (*h).mb.b_transform_8x8;
    if transform_8x8 == 0 {
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[0].offset(
                (4 as c_int
                    * 1 as c_int
                    * (if 0 as c_int != 0 {
                        FDEC_STRIDE
                    } else {
                        1 as c_int
                    })) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            (*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr(),
            qp,
            a,
            b,
            0 as c_int,
            (*h).loopf.deblock_luma[0],
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[1].offset(
                    (4 as c_int
                        * 1 as c_int
                        * (if 0 as c_int != 0 {
                            FDEC_STRIDE
                        } else {
                            1 as c_int
                        })) as isize,
                ),
                FDEC_STRIDE as intptr_t,
                (*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as c_int,
                (*h).loopf.deblock_luma[0],
            );
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[2].offset(
                    (4 as c_int
                        * 1 as c_int
                        * (if 0 as c_int != 0 {
                            FDEC_STRIDE
                        } else {
                            1 as c_int
                        })) as isize,
                ),
                FDEC_STRIDE as intptr_t,
                (*(*bs.offset(0)).as_mut_ptr().offset(1)).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as c_int,
                (*h).loopf.deblock_luma[0],
            );
        }
    }
    deblock_edge(
        h,
        (*h).mb.pic.p_fdec[0].offset(
            (4 as c_int
                * 2 as c_int
                * (if 0 as c_int != 0 {
                    FDEC_STRIDE
                } else {
                    1 as c_int
                })) as isize,
        ),
        FDEC_STRIDE as intptr_t,
        (*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr(),
        qp,
        a,
        b,
        0 as c_int,
        (*h).loopf.deblock_luma[0],
    );
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[1].offset(
                (4 as c_int
                    * 2 as c_int
                    * (if 0 as c_int != 0 {
                        FDEC_STRIDE
                    } else {
                        1 as c_int
                    })) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            (*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr(),
            qpc,
            a,
            b,
            0 as c_int,
            (*h).loopf.deblock_luma[0],
        );
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[2].offset(
                (4 as c_int
                    * 2 as c_int
                    * (if 0 as c_int != 0 {
                        FDEC_STRIDE
                    } else {
                        1 as c_int
                    })) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            (*(*bs.offset(0)).as_mut_ptr().offset(2)).as_mut_ptr(),
            qpc,
            a,
            b,
            0 as c_int,
            (*h).loopf.deblock_luma[0],
        );
    }
    if transform_8x8 == 0 {
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[0].offset(
                (4 as c_int
                    * 3 as c_int
                    * (if 0 as c_int != 0 {
                        FDEC_STRIDE
                    } else {
                        1 as c_int
                    })) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            (*(*bs.offset(0)).as_mut_ptr().offset(3)).as_mut_ptr(),
            qp,
            a,
            b,
            0 as c_int,
            (*h).loopf.deblock_luma[0],
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[1].offset(
                    (4 as c_int
                        * 3 as c_int
                        * (if 0 as c_int != 0 {
                            FDEC_STRIDE
                        } else {
                            1 as c_int
                        })) as isize,
                ),
                FDEC_STRIDE as intptr_t,
                (*(*bs.offset(0)).as_mut_ptr().offset(3)).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as c_int,
                (*h).loopf.deblock_luma[0],
            );
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[2].offset(
                    (4 as c_int
                        * 3 as c_int
                        * (if 0 as c_int != 0 {
                            FDEC_STRIDE
                        } else {
                            1 as c_int
                        })) as isize,
                ),
                FDEC_STRIDE as intptr_t,
                (*(*bs.offset(0)).as_mut_ptr().offset(3)).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as c_int,
                (*h).loopf.deblock_luma[0],
            );
        }
    }
    if transform_8x8 == 0 {
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[0].offset(
                (4 as c_int
                    * 1 as c_int
                    * (if 1 as c_int != 0 {
                        FDEC_STRIDE
                    } else {
                        1 as c_int
                    })) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            (*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr(),
            qp,
            a,
            b,
            0 as c_int,
            (*h).loopf.deblock_luma[1],
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[1].offset(
                    (4 as c_int
                        * 1 as c_int
                        * (if 1 as c_int != 0 {
                            FDEC_STRIDE
                        } else {
                            1 as c_int
                        })) as isize,
                ),
                FDEC_STRIDE as intptr_t,
                (*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as c_int,
                (*h).loopf.deblock_luma[1],
            );
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[2].offset(
                    (4 as c_int
                        * 1 as c_int
                        * (if 1 as c_int != 0 {
                            FDEC_STRIDE
                        } else {
                            1 as c_int
                        })) as isize,
                ),
                FDEC_STRIDE as intptr_t,
                (*(*bs.offset(1)).as_mut_ptr().offset(1)).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as c_int,
                (*h).loopf.deblock_luma[1],
            );
        }
    }
    deblock_edge(
        h,
        (*h).mb.pic.p_fdec[0].offset(
            (4 as c_int
                * 2 as c_int
                * (if 1 as c_int != 0 {
                    FDEC_STRIDE
                } else {
                    1 as c_int
                })) as isize,
        ),
        FDEC_STRIDE as intptr_t,
        (*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr(),
        qp,
        a,
        b,
        0 as c_int,
        (*h).loopf.deblock_luma[1],
    );
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[1].offset(
                (4 as c_int
                    * 2 as c_int
                    * (if 1 as c_int != 0 {
                        FDEC_STRIDE
                    } else {
                        1 as c_int
                    })) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            (*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr(),
            qpc,
            a,
            b,
            0 as c_int,
            (*h).loopf.deblock_luma[1],
        );
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[2].offset(
                (4 as c_int
                    * 2 as c_int
                    * (if 1 as c_int != 0 {
                        FDEC_STRIDE
                    } else {
                        1 as c_int
                    })) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            (*(*bs.offset(1)).as_mut_ptr().offset(2)).as_mut_ptr(),
            qpc,
            a,
            b,
            0 as c_int,
            (*h).loopf.deblock_luma[1],
        );
    }
    if transform_8x8 == 0 {
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[0].offset(
                (4 as c_int
                    * 3 as c_int
                    * (if 1 as c_int != 0 {
                        FDEC_STRIDE
                    } else {
                        1 as c_int
                    })) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            (*(*bs.offset(1)).as_mut_ptr().offset(3)).as_mut_ptr(),
            qp,
            a,
            b,
            0 as c_int,
            (*h).loopf.deblock_luma[1],
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[1].offset(
                    (4 as c_int
                        * 3 as c_int
                        * (if 1 as c_int != 0 {
                            FDEC_STRIDE
                        } else {
                            1 as c_int
                        })) as isize,
                ),
                FDEC_STRIDE as intptr_t,
                (*(*bs.offset(1)).as_mut_ptr().offset(3)).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as c_int,
                (*h).loopf.deblock_luma[1],
            );
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[2].offset(
                    (4 as c_int
                        * 3 as c_int
                        * (if 1 as c_int != 0 {
                            FDEC_STRIDE
                        } else {
                            1 as c_int
                        })) as isize,
                ),
                FDEC_STRIDE as intptr_t,
                (*(*bs.offset(1)).as_mut_ptr().offset(3)).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as c_int,
                (*h).loopf.deblock_luma[1],
            );
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "687:1"]
unsafe extern "C" fn x264_10_deblock_init(
    mut _cpu: uint32_t,
    mut pf: *mut x264_deblock_function_t,
    mut _b_mbaff: c_int,
) {
    (*pf).deblock_luma[1] = Some(
        deblock_v_luma_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int, *mut int8_t) -> (),
    ) as x264_deblock_inter_t;
    (*pf).deblock_luma[0] = Some(
        deblock_h_luma_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int, *mut int8_t) -> (),
    ) as x264_deblock_inter_t;
    (*pf).deblock_chroma[1] = Some(
        deblock_v_chroma_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int, *mut int8_t) -> (),
    ) as x264_deblock_inter_t;
    (*pf).deblock_h_chroma_420 = Some(
        deblock_h_chroma_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int, *mut int8_t) -> (),
    ) as x264_deblock_inter_t;
    (*pf).deblock_h_chroma_422 = Some(
        deblock_h_chroma_422_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int, *mut int8_t) -> (),
    ) as x264_deblock_inter_t;
    (*pf).deblock_luma_intra[1] = Some(
        deblock_v_luma_intra_c as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int) -> (),
    ) as x264_deblock_intra_t;
    (*pf).deblock_luma_intra[0] = Some(
        deblock_h_luma_intra_c as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int) -> (),
    ) as x264_deblock_intra_t;
    (*pf).deblock_chroma_intra[1] = Some(
        deblock_v_chroma_intra_c as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int) -> (),
    ) as x264_deblock_intra_t;
    (*pf).deblock_h_chroma_420_intra = Some(
        deblock_h_chroma_intra_c as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int) -> (),
    ) as x264_deblock_intra_t;
    (*pf).deblock_h_chroma_422_intra = Some(
        deblock_h_chroma_422_intra_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int) -> (),
    ) as x264_deblock_intra_t;
    (*pf).deblock_luma_mbaff = Some(
        deblock_h_luma_mbaff_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int, *mut int8_t) -> (),
    ) as x264_deblock_inter_t;
    (*pf).deblock_chroma_420_mbaff = Some(
        deblock_h_chroma_mbaff_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int, *mut int8_t) -> (),
    ) as x264_deblock_inter_t;
    (*pf).deblock_luma_intra_mbaff = Some(
        deblock_h_luma_intra_mbaff_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int) -> (),
    ) as x264_deblock_intra_t;
    (*pf).deblock_chroma_420_intra_mbaff = Some(
        deblock_h_chroma_intra_mbaff_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, c_int, c_int) -> (),
    ) as x264_deblock_intra_t;
    (*pf).deblock_strength = Some(
        deblock_strength_c
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut [int8_t; 40],
                *mut [[int16_t; 2]; 40],
                *mut [[uint8_t; 4]; 8],
                c_int,
                c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut uint8_t,
                *mut [int8_t; 40],
                *mut [[int16_t; 2]; 40],
                *mut [[uint8_t; 4]; 8],
                c_int,
                c_int,
            ) -> (),
        >;
    (*pf).deblock_chroma_422_mbaff = (*pf).deblock_h_chroma_420;
    (*pf).deblock_chroma_422_intra_mbaff = (*pf).deblock_h_chroma_420_intra;
}
