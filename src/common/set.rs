use core::ffi::{c_char, c_double, c_int, c_ulonglong, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::base_h::{x264_free, x264_malloc, x264_slurp_file, CHROMA_444, PROFILE_HIGH};
use crate::common_h::{udctcoef, x264_10_log, x264_t, QP_MAX, QP_MAX_SPEC};
use crate::internal::BIT_DEPTH;
use crate::mathcalls_h::pow;
use crate::set_h::{CQM_4IC, CQM_4IY, CQM_4PC, CQM_4PY, CQM_8IC, CQM_8IY, CQM_8PC, CQM_8PY};
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint16_t, uint8_t};
use crate::stdio_h::sscanf;
use crate::string_h::{memcmp, memcpy, memset, strchr, strcspn, strlen, strpbrk, strstr};
use crate::tables_h::{x264_cqm_jvt4i, x264_cqm_jvt4p, x264_cqm_jvt8i, x264_cqm_jvt8p};
use crate::x264_h::{X264_CQM_CUSTOM, X264_LOG_ERROR};
#[c2rust::src_loc = "31:22"]
static mut dequant4_scale: [[uint8_t; 3]; 6] = [
    [
        10 as c_int as uint8_t,
        13 as c_int as uint8_t,
        16 as c_int as uint8_t,
    ],
    [
        11 as c_int as uint8_t,
        14 as c_int as uint8_t,
        18 as c_int as uint8_t,
    ],
    [
        13 as c_int as uint8_t,
        16 as c_int as uint8_t,
        20 as c_int as uint8_t,
    ],
    [
        14 as c_int as uint8_t,
        18 as c_int as uint8_t,
        23 as c_int as uint8_t,
    ],
    [
        16 as c_int as uint8_t,
        20 as c_int as uint8_t,
        25 as c_int as uint8_t,
    ],
    [
        18 as c_int as uint8_t,
        23 as c_int as uint8_t,
        29 as c_int as uint8_t,
    ],
];
#[c2rust::src_loc = "40:23"]
static mut quant4_scale: [[uint16_t; 3]; 6] = [
    [
        13107 as c_int as uint16_t,
        8066 as c_int as uint16_t,
        5243 as c_int as uint16_t,
    ],
    [
        11916 as c_int as uint16_t,
        7490 as c_int as uint16_t,
        4660 as c_int as uint16_t,
    ],
    [
        10082 as c_int as uint16_t,
        6554 as c_int as uint16_t,
        4194 as c_int as uint16_t,
    ],
    [
        9362 as c_int as uint16_t,
        5825 as c_int as uint16_t,
        3647 as c_int as uint16_t,
    ],
    [
        8192 as c_int as uint16_t,
        5243 as c_int as uint16_t,
        3355 as c_int as uint16_t,
    ],
    [
        7282 as c_int as uint16_t,
        4559 as c_int as uint16_t,
        2893 as c_int as uint16_t,
    ],
];
#[c2rust::src_loc = "50:22"]
static mut quant8_scan: [uint8_t; 16] = [
    0 as c_int as uint8_t,
    3 as c_int as uint8_t,
    4 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    1 as c_int as uint8_t,
    5 as c_int as uint8_t,
    1 as c_int as uint8_t,
    4 as c_int as uint8_t,
    5 as c_int as uint8_t,
    2 as c_int as uint8_t,
    5 as c_int as uint8_t,
    3 as c_int as uint8_t,
    1 as c_int as uint8_t,
    5 as c_int as uint8_t,
    1 as c_int as uint8_t,
];
#[c2rust::src_loc = "54:22"]
static mut dequant8_scale: [[uint8_t; 6]; 6] = [
    [
        20 as c_int as uint8_t,
        18 as c_int as uint8_t,
        32 as c_int as uint8_t,
        19 as c_int as uint8_t,
        25 as c_int as uint8_t,
        24 as c_int as uint8_t,
    ],
    [
        22 as c_int as uint8_t,
        19 as c_int as uint8_t,
        35 as c_int as uint8_t,
        21 as c_int as uint8_t,
        28 as c_int as uint8_t,
        26 as c_int as uint8_t,
    ],
    [
        26 as c_int as uint8_t,
        23 as c_int as uint8_t,
        42 as c_int as uint8_t,
        24 as c_int as uint8_t,
        33 as c_int as uint8_t,
        31 as c_int as uint8_t,
    ],
    [
        28 as c_int as uint8_t,
        25 as c_int as uint8_t,
        45 as c_int as uint8_t,
        26 as c_int as uint8_t,
        35 as c_int as uint8_t,
        33 as c_int as uint8_t,
    ],
    [
        32 as c_int as uint8_t,
        28 as c_int as uint8_t,
        51 as c_int as uint8_t,
        30 as c_int as uint8_t,
        40 as c_int as uint8_t,
        38 as c_int as uint8_t,
    ],
    [
        36 as c_int as uint8_t,
        32 as c_int as uint8_t,
        58 as c_int as uint8_t,
        34 as c_int as uint8_t,
        46 as c_int as uint8_t,
        43 as c_int as uint8_t,
    ],
];
#[c2rust::src_loc = "63:23"]
static mut quant8_scale: [[uint16_t; 6]; 6] = [
    [
        13107 as c_int as uint16_t,
        11428 as c_int as uint16_t,
        20972 as c_int as uint16_t,
        12222 as c_int as uint16_t,
        16777 as c_int as uint16_t,
        15481 as c_int as uint16_t,
    ],
    [
        11916 as c_int as uint16_t,
        10826 as c_int as uint16_t,
        19174 as c_int as uint16_t,
        11058 as c_int as uint16_t,
        14980 as c_int as uint16_t,
        14290 as c_int as uint16_t,
    ],
    [
        10082 as c_int as uint16_t,
        8943 as c_int as uint16_t,
        15978 as c_int as uint16_t,
        9675 as c_int as uint16_t,
        12710 as c_int as uint16_t,
        11985 as c_int as uint16_t,
    ],
    [
        9362 as c_int as uint16_t,
        8228 as c_int as uint16_t,
        14913 as c_int as uint16_t,
        8931 as c_int as uint16_t,
        11984 as c_int as uint16_t,
        11259 as c_int as uint16_t,
    ],
    [
        8192 as c_int as uint16_t,
        7346 as c_int as uint16_t,
        13159 as c_int as uint16_t,
        7740 as c_int as uint16_t,
        10486 as c_int as uint16_t,
        9777 as c_int as uint16_t,
    ],
    [
        7282 as c_int as uint16_t,
        6428 as c_int as uint16_t,
        11570 as c_int as uint16_t,
        6830 as c_int as uint16_t,
        9118 as c_int as uint16_t,
        8640 as c_int as uint16_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn x264_10_cqm_init(mut h: *mut x264_t) -> c_int {
    let mut current_block: u64;
    let mut def_quant4: [[c_int; 16]; 6] = [[0; 16]; 6];
    let mut def_quant8: [[c_int; 64]; 6] = [[0; 64]; 6];
    let mut def_dequant4: [[c_int; 16]; 6] = [[0; 16]; 6];
    let mut def_dequant8: [[c_int; 64]; 6] = [[0; 64]; 6];
    let mut quant4_mf: [[[c_int; 16]; 6]; 4] = [[[0; 16]; 6]; 4];
    let mut quant8_mf: [[[c_int; 64]; 6]; 4] = [[[0; 64]; 6]; 4];
    let mut deadzone: [c_int; 4] = [
        32 as c_int - (*h).param.analyse.i_luma_deadzone[1 as c_int as usize],
        32 as c_int - (*h).param.analyse.i_luma_deadzone[0 as c_int as usize],
        32 as c_int - 11 as c_int,
        32 as c_int - 21 as c_int,
    ];
    let mut max_qp_err: c_int = -(1 as c_int);
    let mut max_chroma_qp_err: c_int = -(1 as c_int);
    let mut min_qp_err: c_int = QP_MAX + 1 as c_int;
    let mut num_8x8_lists: c_int =
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            4 as c_int
        } else if (*h).param.analyse.b_transform_8x8 != 0 {
            2 as c_int
        } else {
            0 as c_int
        };
    let mut i: c_int = 0 as c_int;
    loop {
        if !(i < 4 as c_int) {
            current_block = 5529461102203738653;
            break;
        }
        let mut size: c_int = 4 as c_int * 4 as c_int;
        let mut start: c_int = if 4 as c_int == 8 as c_int {
            4 as c_int
        } else {
            0 as c_int
        };
        let mut j: c_int = 0;
        j = 0 as c_int;
        while j < i {
            if memcmp(
                (*(*h).sps.as_mut_ptr()).scaling_list[(i + start) as usize] as *const c_void,
                (*(*h).sps.as_mut_ptr()).scaling_list[(j + start) as usize] as *const c_void,
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
                (((51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int) * size)
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                    as int64_t,
            ) as *mut [udctcoef; 16];
            if (*h).quant4_mf[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
            (*h).dequant4_mf[i as usize] = x264_malloc(
                ((6 as c_int * size) as usize)
                    .wrapping_mul(::core::mem::size_of::<c_int>() as usize)
                    as int64_t,
            ) as *mut [c_int; 16];
            if (*h).dequant4_mf[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
            (*h).unquant4_mf[i as usize] = x264_malloc(
                (((51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int) * size)
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<c_int>() as usize)
                    as int64_t,
            ) as *mut [c_int; 16];
            if (*h).unquant4_mf[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
        }
        j = 0 as c_int;
        while j < i {
            if deadzone[j as usize] == deadzone[i as usize]
                && memcmp(
                    (*(*h).sps.as_mut_ptr()).scaling_list[(i + start) as usize] as *const c_void,
                    (*(*h).sps.as_mut_ptr()).scaling_list[(j + start) as usize] as *const c_void,
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
                (((51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int) * size)
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                    as int64_t,
            ) as *mut [udctcoef; 16];
            if (*h).quant4_bias[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
            (*h).quant4_bias0[i as usize] = x264_malloc(
                (((51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int) * size)
                    as usize)
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
            let mut i_0: c_int = 0 as c_int;
            loop {
                if !(i_0 < num_8x8_lists) {
                    current_block = 7419121793134201633;
                    break;
                }
                let mut size_0: c_int = 8 as c_int * 8 as c_int;
                let mut start_0: c_int = if 8 as c_int == 8 as c_int {
                    4 as c_int
                } else {
                    0 as c_int
                };
                let mut j_0: c_int = 0;
                j_0 = 0 as c_int;
                while j_0 < i_0 {
                    if memcmp(
                        (*(*h).sps.as_mut_ptr()).scaling_list[(i_0 + start_0) as usize]
                            as *const c_void,
                        (*(*h).sps.as_mut_ptr()).scaling_list[(j_0 + start_0) as usize]
                            as *const c_void,
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
                        (((51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int)
                            * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                            as int64_t,
                    ) as *mut [udctcoef; 64];
                    if (*h).quant8_mf[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                    (*h).dequant8_mf[i_0 as usize] = x264_malloc(
                        ((6 as c_int * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<c_int>() as usize)
                            as int64_t,
                    ) as *mut [c_int; 64];
                    if (*h).dequant8_mf[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                    (*h).unquant8_mf[i_0 as usize] = x264_malloc(
                        (((51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int)
                            * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<c_int>() as usize)
                            as int64_t,
                    ) as *mut [c_int; 64];
                    if (*h).unquant8_mf[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                }
                j_0 = 0 as c_int;
                while j_0 < i_0 {
                    if deadzone[j_0 as usize] == deadzone[i_0 as usize]
                        && memcmp(
                            (*(*h).sps.as_mut_ptr()).scaling_list[(i_0 + start_0) as usize]
                                as *const c_void,
                            (*(*h).sps.as_mut_ptr()).scaling_list[(j_0 + start_0) as usize]
                                as *const c_void,
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
                        (((51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int)
                            * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                            as int64_t,
                    ) as *mut [udctcoef; 64];
                    if (*h).quant8_bias[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                    (*h).quant8_bias0[i_0 as usize] = x264_malloc(
                        (((51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int)
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
                    let mut q: c_int = 0 as c_int;
                    while q < 6 as c_int {
                        let mut i_1: c_int = 0 as c_int;
                        while i_1 < 16 as c_int {
                            let mut j_1: c_int =
                                (i_1 & 1 as c_int) + (i_1 >> 2 as c_int & 1 as c_int);
                            def_dequant4[q as usize][i_1 as usize] =
                                dequant4_scale[q as usize][j_1 as usize] as c_int;
                            def_quant4[q as usize][i_1 as usize] =
                                quant4_scale[q as usize][j_1 as usize] as c_int;
                            i_1 += 1;
                        }
                        let mut i_2: c_int = 0 as c_int;
                        while i_2 < 64 as c_int {
                            let mut j_2: c_int = quant8_scan
                                [(i_2 >> 1 as c_int & 12 as c_int | i_2 & 3 as c_int) as usize]
                                as c_int;
                            def_dequant8[q as usize][i_2 as usize] =
                                dequant8_scale[q as usize][j_2 as usize] as c_int;
                            def_quant8[q as usize][i_2 as usize] =
                                quant8_scale[q as usize][j_2 as usize] as c_int;
                            i_2 += 1;
                        }
                        q += 1;
                    }
                    let mut q_0: c_int = 0 as c_int;
                    while q_0 < 6 as c_int {
                        let mut i_list: c_int = 0 as c_int;
                        while i_list < 4 as c_int {
                            let mut i_3: c_int = 0 as c_int;
                            while i_3 < 16 as c_int {
                                (*(*h).dequant4_mf[i_list as usize].offset(q_0 as isize))
                                    [i_3 as usize] = def_dequant4[q_0 as usize][i_3 as usize]
                                    * *(*(*h).sps.as_mut_ptr()).scaling_list[i_list as usize]
                                        .offset(i_3 as isize)
                                        as c_int;
                                quant4_mf[i_list as usize][q_0 as usize][i_3 as usize] =
                                    (def_quant4[q_0 as usize][i_3 as usize] * 16 as c_int
                                        + (*(*(*h).sps.as_mut_ptr()).scaling_list[i_list as usize]
                                            .offset(i_3 as isize)
                                            as c_int
                                            >> 1 as c_int))
                                        / *(*(*h).sps.as_mut_ptr()).scaling_list[i_list as usize]
                                            .offset(i_3 as isize)
                                            as c_int;
                                i_3 += 1;
                            }
                            i_list += 1;
                        }
                        let mut i_list_0: c_int = 0 as c_int;
                        while i_list_0 < num_8x8_lists {
                            let mut i_4: c_int = 0 as c_int;
                            while i_4 < 64 as c_int {
                                (*(*h).dequant8_mf[i_list_0 as usize].offset(q_0 as isize))
                                    [i_4 as usize] = def_dequant8[q_0 as usize][i_4 as usize]
                                    * *(*(*h).sps.as_mut_ptr()).scaling_list
                                        [(4 as c_int + i_list_0) as usize]
                                        .offset(i_4 as isize)
                                        as c_int;
                                quant8_mf[i_list_0 as usize][q_0 as usize][i_4 as usize] =
                                    (def_quant8[q_0 as usize][i_4 as usize] * 16 as c_int
                                        + (*(*(*h).sps.as_mut_ptr()).scaling_list
                                            [(4 as c_int + i_list_0) as usize]
                                            .offset(i_4 as isize)
                                            as c_int
                                            >> 1 as c_int))
                                        / *(*(*h).sps.as_mut_ptr()).scaling_list
                                            [(4 as c_int + i_list_0) as usize]
                                            .offset(i_4 as isize)
                                            as c_int;
                                i_4 += 1;
                            }
                            i_list_0 += 1;
                        }
                        q_0 += 1;
                    }
                    let mut q_1: c_int = 0 as c_int;
                    while q_1 <= QP_MAX_SPEC {
                        let mut j_3: c_int = 0;
                        let mut i_list_1: c_int = 0 as c_int;
                        while i_list_1 < 4 as c_int {
                            let mut i_5: c_int = 0 as c_int;
                            while i_5 < 16 as c_int {
                                (*(*h).unquant4_mf[i_list_1 as usize].offset(q_1 as isize))
                                    [i_5 as usize] = ((1 as c_ulonglong)
                                    << q_1 / 6 as c_int + 15 as c_int + 8 as c_int)
                                    .wrapping_div(
                                        quant4_mf[i_list_1 as usize][(q_1 % 6 as c_int) as usize]
                                            [i_5 as usize]
                                            as c_ulonglong,
                                    ) as c_int;
                                j_3 = if q_1 / 6 as c_int - 1 as c_int <= 0 as c_int {
                                    quant4_mf[i_list_1 as usize][(q_1 % 6 as c_int) as usize]
                                        [i_5 as usize]
                                        << -(q_1 / 6 as c_int - 1 as c_int)
                                } else {
                                    quant4_mf[i_list_1 as usize][(q_1 % 6 as c_int) as usize]
                                        [i_5 as usize]
                                        + ((1 as c_int)
                                            << q_1 / 6 as c_int - 1 as c_int - 1 as c_int)
                                        >> q_1 / 6 as c_int - 1 as c_int
                                };
                                (*(*h).quant4_mf[i_list_1 as usize].offset(q_1 as isize))
                                    [i_5 as usize] = j_3 as uint16_t as udctcoef;
                                if j_3 == 0 {
                                    min_qp_err = if min_qp_err < q_1 { min_qp_err } else { q_1 };
                                } else {
                                    (*(*h).quant4_bias[i_list_1 as usize].offset(q_1 as isize))
                                        [i_5 as usize] =
                                        (if ((deadzone[i_list_1 as usize] << 10 as c_int)
                                            + (j_3 >> 1 as c_int))
                                            / j_3
                                            < ((1 as c_int) << 15 as c_int) / j_3
                                        {
                                            ((deadzone[i_list_1 as usize] << 10 as c_int)
                                                + (j_3 >> 1 as c_int))
                                                / j_3
                                        } else {
                                            ((1 as c_int) << 15 as c_int) / j_3
                                        }) as udctcoef;
                                    (*(*h).quant4_bias0[i_list_1 as usize].offset(q_1 as isize))
                                        [i_5 as usize] =
                                        (((1 as c_int) << 15 as c_int) / j_3) as udctcoef;
                                    if j_3
                                        > (if (0xffff as c_int)
                                            < ((1 as c_int) << 25 as c_int - 10 as c_int)
                                                - 1 as c_int
                                        {
                                            0xffff as c_int
                                        } else {
                                            ((1 as c_int) << 25 as c_int - 10 as c_int) - 1 as c_int
                                        })
                                        && q_1 > max_qp_err
                                        && (i_list_1 == CQM_4IY as c_int
                                            || i_list_1 == CQM_4PY as c_int)
                                    {
                                        max_qp_err = q_1;
                                    }
                                    if j_3
                                        > (if (0xffff as c_int)
                                            < ((1 as c_int) << 25 as c_int - 10 as c_int)
                                                - 1 as c_int
                                        {
                                            0xffff as c_int
                                        } else {
                                            ((1 as c_int) << 25 as c_int - 10 as c_int) - 1 as c_int
                                        })
                                        && q_1 > max_chroma_qp_err
                                        && (i_list_1 == CQM_4IC as c_int
                                            || i_list_1 == CQM_4PC as c_int)
                                    {
                                        max_chroma_qp_err = q_1;
                                    }
                                }
                                i_5 += 1;
                            }
                            i_list_1 += 1;
                        }
                        if (*h).param.analyse.b_transform_8x8 != 0 {
                            let mut i_list_2: c_int = 0 as c_int;
                            while i_list_2 < num_8x8_lists {
                                let mut i_6: c_int = 0 as c_int;
                                while i_6 < 64 as c_int {
                                    (*(*h).unquant8_mf[i_list_2 as usize].offset(q_1 as isize))
                                        [i_6 as usize] = ((1 as c_ulonglong)
                                        << q_1 / 6 as c_int + 16 as c_int + 8 as c_int)
                                        .wrapping_div(
                                            quant8_mf[i_list_2 as usize]
                                                [(q_1 % 6 as c_int) as usize]
                                                [i_6 as usize]
                                                as c_ulonglong,
                                        )
                                        as c_int;
                                    j_3 = if q_1 / 6 as c_int <= 0 as c_int {
                                        quant8_mf[i_list_2 as usize][(q_1 % 6 as c_int) as usize]
                                            [i_6 as usize]
                                            << -(q_1 / 6 as c_int)
                                    } else {
                                        quant8_mf[i_list_2 as usize][(q_1 % 6 as c_int) as usize]
                                            [i_6 as usize]
                                            + ((1 as c_int) << q_1 / 6 as c_int - 1 as c_int)
                                            >> q_1 / 6 as c_int
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
                                            << 10 as c_int)
                                            + (j_3 >> 1 as c_int))
                                            / j_3
                                            < ((1 as c_int) << 15 as c_int) / j_3
                                        {
                                            ((deadzone[i_list_2 as usize] << 10 as c_int)
                                                + (j_3 >> 1 as c_int))
                                                / j_3
                                        } else {
                                            ((1 as c_int) << 15 as c_int) / j_3
                                        })
                                            as udctcoef;
                                        (*(*h).quant8_bias0[i_list_2 as usize]
                                            .offset(q_1 as isize))
                                            [i_6 as usize] =
                                            (((1 as c_int) << 15 as c_int) / j_3) as udctcoef;
                                        if j_3
                                            > (if (0xffff as c_int)
                                                < ((1 as c_int) << 25 as c_int - 10 as c_int)
                                                    - 1 as c_int
                                            {
                                                0xffff as c_int
                                            } else {
                                                ((1 as c_int) << 25 as c_int - 10 as c_int)
                                                    - 1 as c_int
                                            })
                                            && q_1 > max_qp_err
                                            && (i_list_2 == CQM_8IY as c_int
                                                || i_list_2 == CQM_8PY as c_int)
                                        {
                                            max_qp_err = q_1;
                                        }
                                        if j_3
                                            > (if (0xffff as c_int)
                                                < ((1 as c_int) << 25 as c_int - 10 as c_int)
                                                    - 1 as c_int
                                            {
                                                0xffff as c_int
                                            } else {
                                                ((1 as c_int) << 25 as c_int - 10 as c_int)
                                                    - 1 as c_int
                                            })
                                            && q_1 > max_chroma_qp_err
                                            && (i_list_2 == CQM_8IC as c_int
                                                || i_list_2 == CQM_8PC as c_int)
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
                            (51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) + 18 as c_int
                                - (51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)))
                                as usize,
                        ) as int64_t,
                    ) as *mut [[udctcoef; 64]; 4];
                    if !(*h).nr_offset_emergency.is_null() {
                        let mut q_2: c_int = 0 as c_int;
                        while q_2 < QP_MAX - QP_MAX_SPEC {
                            let mut cat: c_int = 0 as c_int;
                            while cat
                                < 3 as c_int
                                    + ((*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                        == CHROMA_444 as c_int)
                                        as c_int
                            {
                                let mut dct8x8: c_int = cat & 1 as c_int;
                                if !((*h).param.analyse.b_transform_8x8 == 0 && dct8x8 != 0) {
                                    let mut size_1: c_int = if dct8x8 != 0 {
                                        64 as c_int
                                    } else {
                                        16 as c_int
                                    };
                                    let mut nr_offset: *mut udctcoef =
                                        (*(*(*h).nr_offset_emergency.offset(q_2 as isize))
                                            .as_mut_ptr()
                                            .offset(cat as isize))
                                        .as_mut_ptr();
                                    let mut dc_threshold: c_int =
                                        (QP_MAX - QP_MAX_SPEC) * 2 as c_int / 3 as c_int;
                                    let mut luma_threshold: c_int =
                                        (QP_MAX - QP_MAX_SPEC) * 2 as c_int / 3 as c_int;
                                    let mut chroma_threshold: c_int = 0 as c_int;
                                    let mut i_7: c_int = 0 as c_int;
                                    while i_7 < size_1 {
                                        let mut max: c_int =
                                            ((1 as c_int) << 7 as c_int + BIT_DEPTH) - 1 as c_int;
                                        if q_2 == QP_MAX - QP_MAX_SPEC - 1 as c_int {
                                            *nr_offset.offset(i_7 as isize) = max as udctcoef;
                                        } else {
                                            let mut thresh: c_int = if i_7 == 0 as c_int {
                                                dc_threshold
                                            } else if cat >= 2 as c_int {
                                                chroma_threshold
                                            } else {
                                                luma_threshold
                                            };
                                            if q_2 < thresh {
                                                *nr_offset.offset(i_7 as isize) = 0 as udctcoef;
                                            } else {
                                                let mut pos: c_double = (q_2 - thresh + 1 as c_int)
                                                    as c_double
                                                    / (QP_MAX - QP_MAX_SPEC - thresh) as c_double;
                                                let mut start_1: c_double = (if dct8x8 != 0 {
                                                    (*(*h).unquant8_mf[CQM_8PY as c_int as usize]
                                                        .offset(QP_MAX_SPEC as isize))
                                                        [i_7 as usize]
                                                } else {
                                                    (*(*h).unquant4_mf[CQM_4PY as c_int as usize]
                                                        .offset(QP_MAX_SPEC as isize))
                                                        [i_7 as usize]
                                                })
                                                    as c_double;
                                                let mut bias: c_double = (pow(
                                                    2 as c_int as c_double,
                                                    pos * (QP_MAX - QP_MAX_SPEC) as c_double
                                                        / 10.0f64,
                                                ) * 0.003f64
                                                    - 0.003f64)
                                                    * start_1;
                                                *nr_offset.offset(i_7 as isize) =
                                                    (if bias + 0.5f64 < max as c_double {
                                                        bias + 0.5f64
                                                    } else {
                                                        max as c_double
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
                                    < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                                {
                                    (*h).param.rc.i_qp_min
                                } else {
                                    51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                                }) as isize,
                            ) as c_int
                                <= max_chroma_qp_err
                            {
                                (*h).param.rc.i_qp_min += 1;
                            }
                            if min_qp_err <= (*h).param.rc.i_qp_max {
                                (*h).param.rc.i_qp_max = min_qp_err - 1 as c_int;
                            }
                            if max_qp_err >= (*h).param.rc.i_qp_min {
                                (*h).param.rc.i_qp_min = max_qp_err + 1 as c_int;
                            }
                            if (*h).param.b_cabac == 0
                                && (*(*h).sps.as_mut_ptr()).i_profile_idc < PROFILE_HIGH as c_int
                            {
                                while *(*h).chroma_qp_table.offset(
                                    (if (*h).param.rc.i_qp_max
                                        < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                                    {
                                        (*h).param.rc.i_qp_max
                                    } else {
                                        51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                                    }) as isize,
                                ) as c_int
                                    <= 12 as c_int
                                    || (*h).param.rc.i_qp_max <= 12 as c_int
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
                                        as *const c_char,
                                    (*h).param.rc.i_qp_min,
                                    (*h).param.rc.i_qp_max,
                                );
                                return -(1 as c_int);
                            }
                        }
                        return 0 as c_int;
                    }
                }
            }
        }
        _ => {}
    }
    x264_10_cqm_delete(h);
    return -(1 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "300:1"]
unsafe extern "C" fn x264_10_cqm_delete(mut h: *mut x264_t) {
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        let mut j: c_int = 0;
        j = 0 as c_int;
        while j < i {
            if (*h).quant4_mf[i as usize] == (*h).quant4_mf[j as usize] {
                break;
            }
            j += 1;
        }
        if j == i {
            x264_free((*h).quant4_mf[i as usize] as *mut c_void);
            x264_free((*h).dequant4_mf[i as usize] as *mut c_void);
            x264_free((*h).unquant4_mf[i as usize] as *mut c_void);
        }
        j = 0 as c_int;
        while j < i {
            if (*h).quant4_bias[i as usize] == (*h).quant4_bias[j as usize] {
                break;
            }
            j += 1;
        }
        if j == i {
            x264_free((*h).quant4_bias[i as usize] as *mut c_void);
            x264_free((*h).quant4_bias0[i as usize] as *mut c_void);
        }
        i += 1;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0
        < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            4 as c_int
        } else {
            2 as c_int
        })
    {
        let mut j_0: c_int = 0;
        j_0 = 0 as c_int;
        while j_0 < i_0 {
            if (*h).quant8_mf[i_0 as usize] == (*h).quant8_mf[j_0 as usize] {
                break;
            }
            j_0 += 1;
        }
        if j_0 == i_0 {
            x264_free((*h).quant8_mf[i_0 as usize] as *mut c_void);
            x264_free((*h).dequant8_mf[i_0 as usize] as *mut c_void);
            x264_free((*h).unquant8_mf[i_0 as usize] as *mut c_void);
        }
        j_0 = 0 as c_int;
        while j_0 < i_0 {
            if (*h).quant8_bias[i_0 as usize] == (*h).quant8_bias[j_0 as usize] {
                break;
            }
            j_0 += 1;
        }
        if j_0 == i_0 {
            x264_free((*h).quant8_bias[i_0 as usize] as *mut c_void);
            x264_free((*h).quant8_bias0[i_0 as usize] as *mut c_void);
        }
        i_0 += 1;
    }
    x264_free((*h).nr_offset_emergency as *mut c_void);
}
#[c2rust::src_loc = "307:1"]
unsafe extern "C" fn cqm_parse_jmlist(
    mut h: *mut x264_t,
    mut buf: *const c_char,
    mut name: *const c_char,
    mut cqm: *mut uint8_t,
    mut jvt: *const uint8_t,
    mut length: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut p: *mut c_char = strstr(buf, name);
    if p.is_null() {
        memset(cqm as *mut c_void, 16 as c_int, length as size_t);
        return 0 as c_int;
    }
    p = p.offset(strlen(name) as isize);
    if *p as c_int == 'U' as i32 || *p as c_int == 'V' as i32 {
        p = p.offset(1);
    }
    let mut nextvar: *mut c_char = strstr(p, b"INT\0" as *const u8 as *const c_char);
    i = 0 as c_int;
    while i < length
        && {
            p = strpbrk(p, b" \t\n,\0" as *const u8 as *const c_char);
            !p.is_null()
        }
        && {
            p = strpbrk(p, b"0123456789\0" as *const u8 as *const c_char);
            !p.is_null()
        }
    {
        let mut coef: c_int = -(1 as c_int);
        sscanf(
            p,
            b"%d\0" as *const u8 as *const c_char,
            &mut coef as *mut c_int,
        );
        if i == 0 as c_int && coef == 0 as c_int {
            memcpy(cqm as *mut c_void, jvt as *const c_void, length as size_t);
            return 0 as c_int;
        }
        if coef < 1 as c_int || coef > 255 as c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"bad coefficient in list '%s'\n\0" as *const u8 as *const c_char,
                name,
            );
            return -(1 as c_int);
        }
        *cqm.offset(i as isize) = coef as uint8_t;
        i += 1;
    }
    if !nextvar.is_null() && p > nextvar || i != length {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"not enough coefficients in list '%s'\n\0" as *const u8 as *const c_char,
            name,
        );
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "351:1"]
unsafe extern "C" fn x264_10_cqm_parse_file(
    mut h: *mut x264_t,
    mut filename: *const c_char,
) -> c_int {
    let mut p: *mut c_char = 0 as *mut c_char;
    let mut b_error: c_int = 0 as c_int;
    (*h).param.i_cqm_preset = X264_CQM_CUSTOM;
    let mut buf: *mut c_char = x264_slurp_file(filename);
    if buf.is_null() {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"can't open file '%s'\n\0" as *const u8 as *const c_char,
            filename,
        );
        return -(1 as c_int);
    }
    loop {
        p = strchr(buf, '#' as i32);
        if p.is_null() {
            break;
        }
        memset(
            p as *mut c_void,
            ' ' as i32,
            strcspn(p, b"\n\0" as *const u8 as *const c_char) as size_t,
        );
    }
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA4X4_LUMA\0" as *const u8 as *const c_char,
        (*h).param.cqm_4iy.as_mut_ptr(),
        x264_cqm_jvt4i.as_ptr(),
        16 as c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER4X4_LUMA\0" as *const u8 as *const c_char,
        (*h).param.cqm_4py.as_mut_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        16 as c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA4X4_CHROMA\0" as *const u8 as *const c_char,
        (*h).param.cqm_4ic.as_mut_ptr(),
        x264_cqm_jvt4i.as_ptr(),
        16 as c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER4X4_CHROMA\0" as *const u8 as *const c_char,
        (*h).param.cqm_4pc.as_mut_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        16 as c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA8X8_LUMA\0" as *const u8 as *const c_char,
        (*h).param.cqm_8iy.as_mut_ptr(),
        x264_cqm_jvt8i.as_ptr(),
        64 as c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER8X8_LUMA\0" as *const u8 as *const c_char,
        (*h).param.cqm_8py.as_mut_ptr(),
        x264_cqm_jvt8p.as_ptr(),
        64 as c_int,
    );
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTRA8X8_CHROMA\0" as *const u8 as *const c_char,
            (*h).param.cqm_8ic.as_mut_ptr(),
            x264_cqm_jvt8i.as_ptr(),
            64 as c_int,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER8X8_CHROMA\0" as *const u8 as *const c_char,
            (*h).param.cqm_8pc.as_mut_ptr(),
            x264_cqm_jvt8p.as_ptr(),
            64 as c_int,
        );
    }
    x264_free(buf as *mut c_void);
    return b_error;
}
