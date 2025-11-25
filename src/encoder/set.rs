use ::core::mem::size_of;
use core::ffi::{c_char, c_float, c_int, c_uint, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::base_h::{
    x264_free, x264_malloc, x264_param2string, x264_union32_t, CHROMA_400, CHROMA_420, CHROMA_422,
    CHROMA_444, PROFILE_BASELINE, PROFILE_HIGH, PROFILE_HIGH10, PROFILE_HIGH422,
    PROFILE_HIGH444_PREDICTIVE, PROFILE_MAIN, SEI_ALTERNATIVE_TRANSFER, SEI_BUFFERING_PERIOD,
    SEI_CONTENT_LIGHT_LEVEL, SEI_DEC_REF_PIC_MARKING, SEI_FRAME_PACKING, SEI_MASTERING_DISPLAY,
    SEI_PIC_TIMING, SEI_RECOVERY_POINT, SEI_USER_DATA_UNREGISTERED,
};
use crate::bitstream_h::{
    bs_align_10, bs_flush, bs_init, bs_pos, bs_rbsp_trailing, bs_realign, bs_s, bs_size_se, bs_t,
    bs_write, bs_write1, bs_write32, bs_write_se, bs_write_ue_big,
};
use crate::common_h::{x264_10_log, x264_slice_header_t, x264_t, QP_BD_OFFSET};
use crate::config_h::HAVE_GPL;
use crate::internal::BIT_DEPTH;
use crate::macroblock_h::{x264_zigzag_scan4, x264_zigzag_scan8};
use crate::mathcalls_h::log2f;
use crate::set_h::{
    x264_pps_t, x264_sps_t, CQM_4IC, CQM_4IY, CQM_4PC, CQM_4PY, CQM_8IC, CQM_8IY, CQM_8PC, CQM_8PY,
};
use crate::stdint_intn_h::{int32_t, int64_t, int8_t};
use crate::stdint_uintn_h::{uint32_t, uint8_t};
use crate::stdio_h::sprintf;
use crate::string_h::{memcmp, memcpy, memset, strlen};
use crate::tables_h::{x264_cqm_flat16, x264_cqm_jvt};
use crate::x264_config_h::X264_VERSION;
use crate::x264_h::{
    x264_level_t, x264_levels, x264_param_t, BPyramid, ContentLightLevel, FramePacking,
    MasteringDisplay, RateControlMode, X264_BUILD, X264_CQM_CUSTOM, X264_CQM_FLAT, X264_CQM_JVT,
    X264_CSP_BGR, X264_CSP_I420, X264_CSP_I422, X264_CSP_I444, X264_CSP_MASK, X264_LOG_ERROR,
    X264_LOG_WARNING,
};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "381:26"]
struct C2RustUnnamed_19 {
    w: uint8_t,
    h: uint8_t,
    sar: uint8_t,
}
#[c2rust::src_loc = "33:22"]
static mut num_clock_ts: [uint8_t; 10] = [
    0 as c_int as uint8_t,
    1 as c_int as uint8_t,
    1 as c_int as uint8_t,
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    2 as c_int as uint8_t,
    3 as c_int as uint8_t,
    3 as c_int as uint8_t,
    2 as c_int as uint8_t,
    3 as c_int as uint8_t,
];
#[c2rust::src_loc = "34:22"]
static mut avcintra_uuid: [uint8_t; 16] = [
    0xf7 as c_int as uint8_t,
    0x49 as c_int as uint8_t,
    0x3e as c_int as uint8_t,
    0xb3 as c_int as uint8_t,
    0xd4 as c_int as uint8_t,
    0 as c_int as uint8_t,
    0x47 as c_int as uint8_t,
    0x96 as c_int as uint8_t,
    0x86 as c_int as uint8_t,
    0x86 as c_int as uint8_t,
    0xc9 as c_int as uint8_t,
    0x70 as c_int as uint8_t,
    0x7b as c_int as uint8_t,
    0x64 as c_int as uint8_t,
    0x37 as c_int as uint8_t,
    0x2a as c_int as uint8_t,
];
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn transpose(mut buf: *mut uint8_t, mut w: c_int) {
    let mut i: c_int = 0 as c_int;
    while i < w {
        let mut j: c_int = 0 as c_int;
        while j < i {
            let mut t: uint8_t = *buf.offset((w * i + j) as isize);
            *buf.offset((w * i + j) as isize) = *buf.offset((w * j + i) as isize);
            *buf.offset((w * j + i) as isize) = t;
            j += 1;
        }
        i += 1;
    }
}
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn scaling_list_write(
    mut s: *mut bs_t,
    mut sps: *mut x264_sps_t,
    mut idx: c_int,
) {
    let len: c_int = if idx < 4 as c_int {
        16 as c_int
    } else {
        64 as c_int
    };
    let mut zigzag: *const uint8_t = if idx < 4 as c_int {
        (*x264_zigzag_scan4.as_ptr().offset(0)).as_ptr()
    } else {
        (*x264_zigzag_scan8.as_ptr().offset(0)).as_ptr()
    };
    let mut list: *const uint8_t = (*sps).scaling_list[idx as usize];
    let mut def_list: *const uint8_t = if idx == CQM_4IC as c_int {
        (*sps).scaling_list[CQM_4IY as c_int as usize]
    } else if idx == CQM_4PC as c_int {
        (*sps).scaling_list[CQM_4PY as c_int as usize]
    } else if idx == CQM_8IC as c_int + 4 as c_int {
        (*sps).scaling_list[(CQM_8IY as c_int + 4 as c_int) as usize]
    } else if idx == CQM_8PC as c_int + 4 as c_int {
        (*sps).scaling_list[(CQM_8PY as c_int + 4 as c_int) as usize]
    } else {
        x264_cqm_jvt[idx as usize]
    };
    if memcmp(
        list as *const c_void,
        def_list as *const c_void,
        len as size_t,
    ) == 0
    {
        bs_write1(s, 0 as uint32_t);
    } else if memcmp(
        list as *const c_void,
        x264_cqm_jvt[idx as usize] as *const c_void,
        len as size_t,
    ) == 0
    {
        bs_write1(s, 1 as uint32_t);
        bs_write_se(s, -(8 as c_int));
    } else {
        let mut run: c_int = 0;
        bs_write1(s, 1 as uint32_t);
        run = len;
        while run > 1 as c_int {
            if *list.offset(*zigzag.offset((run - 1 as c_int) as isize) as isize) as c_int
                != *list.offset(*zigzag.offset((run - 2 as c_int) as isize) as isize) as c_int
            {
                break;
            }
            run -= 1;
        }
        if run < len
            && len - run
                < bs_size_se(
                    -(*list.offset(*zigzag.offset(run as isize) as isize) as c_int) as int8_t
                        as c_int,
                )
        {
            run = len;
        }
        let mut j: c_int = 0 as c_int;
        while j < run {
            bs_write_se(
                s,
                (*list.offset(*zigzag.offset(j as isize) as isize) as c_int
                    - (if j > 0 as c_int {
                        *list.offset(*zigzag.offset((j - 1 as c_int) as isize) as isize) as c_int
                    } else {
                        8 as c_int
                    })) as int8_t as c_int,
            );
            j += 1;
        }
        if run < len {
            bs_write_se(
                s,
                -(*list.offset(*zigzag.offset(run as isize) as isize) as c_int) as int8_t as c_int,
            );
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn x264_10_sei_write(
    mut s: *mut bs_t,
    mut payload: *mut uint8_t,
    mut payload_size: c_int,
    mut payload_type: c_int,
) {
    let mut i: c_int = 0;
    bs_realign(s);
    i = 0 as c_int;
    while i <= payload_type - 255 as c_int {
        bs_write(s, 8 as c_int, 255 as uint32_t);
        i += 255 as c_int;
    }
    bs_write(s, 8 as c_int, (payload_type - i) as uint32_t);
    i = 0 as c_int;
    while i <= payload_size - 255 as c_int {
        bs_write(s, 8 as c_int, 255 as uint32_t);
        i += 255 as c_int;
    }
    bs_write(s, 8 as c_int, (payload_size - i) as uint32_t);
    i = 0 as c_int;
    while i < payload_size {
        bs_write(s, 8 as c_int, *payload.offset(i as isize) as uint32_t);
        i += 1;
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn x264_10_sps_init(
    mut sps: *mut x264_sps_t,
    mut i_id: c_int,
    mut param: *mut x264_param_t,
) {
    let mut csp: c_int = (*param).i_csp & X264_CSP_MASK;
    (*sps).i_id = i_id;
    (*sps).i_mb_width = ((*param).width as c_int + 15 as c_int) / 16 as c_int;
    (*sps).i_mb_height = ((*param).height as c_int + 15 as c_int) / 16 as c_int;
    (*sps).b_frame_mbs_only = !((*param).interlaced || (*param).fake_interlaced) as c_int;
    if (*sps).b_frame_mbs_only == 0 {
        (*sps).i_mb_height = (*sps).i_mb_height + 1 as c_int & !(1 as c_int);
    }
    (*sps).i_chroma_format_idc = if csp >= X264_CSP_I444 {
        CHROMA_444 as c_int
    } else if csp >= X264_CSP_I422 {
        CHROMA_422 as c_int
    } else if csp >= X264_CSP_I420 {
        CHROMA_420 as c_int
    } else {
        CHROMA_400 as c_int
    };
    (*sps).b_qpprime_y_zero_transform_bypass = ((*param).rc.i_rc_method == RateControlMode::CQP
        && (*param).rc.i_qp_constant == 0 as c_int)
        as c_int;
    if (*sps).b_qpprime_y_zero_transform_bypass != 0
        || (*sps).i_chroma_format_idc == CHROMA_444 as c_int
    {
        (*sps).i_profile_idc = PROFILE_HIGH444_PREDICTIVE as c_int;
    } else if (*sps).i_chroma_format_idc == CHROMA_422 as c_int {
        (*sps).i_profile_idc = PROFILE_HIGH422 as c_int;
    } else if BIT_DEPTH > 8 as c_int {
        (*sps).i_profile_idc = PROFILE_HIGH10 as c_int;
    } else if (*param).analyse.transform_8x8
        || (*param).i_cqm_preset != X264_CQM_FLAT
        || (*sps).i_chroma_format_idc == CHROMA_400 as c_int
    {
        (*sps).i_profile_idc = PROFILE_HIGH as c_int;
    } else if (*param).cabac
        || (*param).i_bframe > 0 as c_int
        || (*param).interlaced
        || (*param).fake_interlaced
        || (*param).analyse.i_weighted_pred > 0 as c_int
    {
        (*sps).i_profile_idc = PROFILE_MAIN as c_int;
    } else {
        (*sps).i_profile_idc = PROFILE_BASELINE as c_int;
    }
    (*sps).b_constraint_set0 = ((*sps).i_profile_idc == PROFILE_BASELINE as c_int) as c_int;
    (*sps).b_constraint_set1 = ((*sps).i_profile_idc <= PROFILE_MAIN as c_int) as c_int;
    (*sps).b_constraint_set2 = 0 as c_int;
    (*sps).b_constraint_set3 = 0 as c_int;
    (*sps).i_level_idc = (*param).i_level_idc;
    if (*param).i_level_idc == 9 as c_int
        && ((*sps).i_profile_idc == PROFILE_BASELINE as c_int
            || (*sps).i_profile_idc == PROFILE_MAIN as c_int)
    {
        (*sps).b_constraint_set3 = 1 as c_int;
        (*sps).i_level_idc = 11 as c_int;
    }
    if (*param).i_keyint_max == 1 as c_int && (*sps).i_profile_idc >= PROFILE_HIGH as c_int {
        (*sps).b_constraint_set3 = 1 as c_int;
    }
    (*sps).vui.i_num_reorder_frames = if (*param).bframe_pyramid != BPyramid::None {
        2 as c_int
    } else if (*param).i_bframe != 0 {
        1 as c_int
    } else {
        0 as c_int
    };
    (*sps).i_num_ref_frames = if (16 as c_int)
        < (if (*param).i_frame_reference
            > (if 1 as c_int + (*sps).vui.i_num_reorder_frames
                > (if (if (*param).bframe_pyramid != BPyramid::None {
                    4 as c_int
                } else {
                    1 as c_int
                }) > (*param).i_dpb_size
                {
                    if (*param).bframe_pyramid != BPyramid::None {
                        4 as c_int
                    } else {
                        1 as c_int
                    }
                } else {
                    (*param).i_dpb_size
                })
            {
                1 as c_int + (*sps).vui.i_num_reorder_frames
            } else {
                if (if (*param).bframe_pyramid != BPyramid::None {
                    4 as c_int
                } else {
                    1 as c_int
                }) > (*param).i_dpb_size
                {
                    if (*param).bframe_pyramid != BPyramid::None {
                        4 as c_int
                    } else {
                        1 as c_int
                    }
                } else {
                    (*param).i_dpb_size
                }
            })
        {
            (*param).i_frame_reference
        } else {
            if 1 as c_int + (*sps).vui.i_num_reorder_frames
                > (if (if (*param).bframe_pyramid != BPyramid::None {
                    4 as c_int
                } else {
                    1 as c_int
                }) > (*param).i_dpb_size
                {
                    if (*param).bframe_pyramid != BPyramid::None {
                        4 as c_int
                    } else {
                        1 as c_int
                    }
                } else {
                    (*param).i_dpb_size
                })
            {
                1 as c_int + (*sps).vui.i_num_reorder_frames
            } else {
                if (if (*param).bframe_pyramid != BPyramid::None {
                    4 as c_int
                } else {
                    1 as c_int
                }) > (*param).i_dpb_size
                {
                    if (*param).bframe_pyramid != BPyramid::None {
                        4 as c_int
                    } else {
                        1 as c_int
                    }
                } else {
                    (*param).i_dpb_size
                }
            }
        }) {
        16 as c_int
    } else if (*param).i_frame_reference
        > (if 1 as c_int + (*sps).vui.i_num_reorder_frames
            > (if (if (*param).bframe_pyramid != BPyramid::None {
                4 as c_int
            } else {
                1 as c_int
            }) > (*param).i_dpb_size
            {
                if (*param).bframe_pyramid != BPyramid::None {
                    4 as c_int
                } else {
                    1 as c_int
                }
            } else {
                (*param).i_dpb_size
            })
        {
            1 as c_int + (*sps).vui.i_num_reorder_frames
        } else {
            if (if (*param).bframe_pyramid != BPyramid::None {
                4 as c_int
            } else {
                1 as c_int
            }) > (*param).i_dpb_size
            {
                if (*param).bframe_pyramid != BPyramid::None {
                    4 as c_int
                } else {
                    1 as c_int
                }
            } else {
                (*param).i_dpb_size
            }
        })
    {
        (*param).i_frame_reference
    } else if 1 as c_int + (*sps).vui.i_num_reorder_frames
        > (if (if (*param).bframe_pyramid != BPyramid::None {
            4 as c_int
        } else {
            1 as c_int
        }) > (*param).i_dpb_size
        {
            if (*param).bframe_pyramid != BPyramid::None {
                4 as c_int
            } else {
                1 as c_int
            }
        } else {
            (*param).i_dpb_size
        })
    {
        1 as c_int + (*sps).vui.i_num_reorder_frames
    } else if (if (*param).bframe_pyramid != BPyramid::None {
        4 as c_int
    } else {
        1 as c_int
    }) > (*param).i_dpb_size
    {
        if (*param).bframe_pyramid != BPyramid::None {
            4 as c_int
        } else {
            1 as c_int
        }
    } else {
        (*param).i_dpb_size
    };
    (*sps).vui.i_max_dec_frame_buffering = (*sps).i_num_ref_frames;
    (*sps).i_num_ref_frames -= ((*param).bframe_pyramid == BPyramid::Strict) as c_int;
    if (*param).i_keyint_max == 1 as c_int {
        (*sps).i_num_ref_frames = 0 as c_int;
        (*sps).vui.i_max_dec_frame_buffering = 0 as c_int;
    }
    let mut max_frame_num: c_int = (*sps).vui.i_max_dec_frame_buffering
        * (((*param).bframe_pyramid != BPyramid::None) as c_int + 1 as c_int)
        + 1 as c_int;
    if (*param).intra_refresh {
        let mut time_to_recovery: c_int =
            (if ((*sps).i_mb_width - 1 as c_int) < (*param).i_keyint_max {
                (*sps).i_mb_width - 1 as c_int
            } else {
                (*param).i_keyint_max
            }) + (*param).i_bframe
                - 1 as c_int;
        max_frame_num = if max_frame_num > time_to_recovery + 1 as c_int {
            max_frame_num
        } else {
            time_to_recovery + 1 as c_int
        };
    }
    (*sps).i_log2_max_frame_num = 4 as c_int;
    while (1 as c_int) << (*sps).i_log2_max_frame_num <= max_frame_num {
        (*sps).i_log2_max_frame_num += 1;
    }
    (*sps).i_poc_type =
        if (*param).i_bframe != 0 || (*param).interlaced || (*param).i_avcintra_class != 0 {
            0 as c_int
        } else {
            2 as c_int
        };
    if (*sps).i_poc_type == 0 as c_int {
        let mut max_delta_poc: c_int = ((*param).i_bframe + 2 as c_int)
            * (((*param).bframe_pyramid != BPyramid::None) as c_int + 1 as c_int)
            * 2 as c_int;
        (*sps).i_log2_max_poc_lsb = 4 as c_int;
        while (1 as c_int) << (*sps).i_log2_max_poc_lsb <= max_delta_poc * 2 as c_int {
            (*sps).i_log2_max_poc_lsb += 1;
        }
    }
    (*sps).b_vui = 1 as c_int;
    (*sps).b_gaps_in_frame_num_value_allowed = 0 as c_int;
    (*sps).mb_adaptive_frame_field = (*param).interlaced;
    (*sps).b_direct8x8_inference = 1 as c_int;
    x264_10_sps_init_reconfigurable(sps, param);
    (*sps).vui.b_overscan_info_present =
        ((*param).vui.i_overscan > 0 as c_int && (*param).vui.i_overscan <= 2 as c_int) as c_int;
    if (*sps).vui.b_overscan_info_present != 0 {
        (*sps).vui.b_overscan_info = if (*param).vui.i_overscan == 2 as c_int {
            1 as c_int
        } else {
            0 as c_int
        };
    }
    (*sps).vui.b_signal_type_present = 0 as c_int;
    (*sps).vui.i_vidformat =
        if (*param).vui.i_vidformat >= 0 as c_int && (*param).vui.i_vidformat <= 5 as c_int {
            (*param).vui.i_vidformat
        } else {
            5 as c_int
        };
    (*sps).vui.b_fullrange =
        if (*param).vui.b_fullrange >= 0 as c_int && (*param).vui.b_fullrange <= 1 as c_int {
            (*param).vui.b_fullrange
        } else if csp >= X264_CSP_BGR {
            1 as c_int
        } else {
            0 as c_int
        };
    (*sps).vui.b_color_description_present = 0 as c_int;
    (*sps).vui.i_colorprim =
        if (*param).vui.i_colorprim >= 0 as c_int && (*param).vui.i_colorprim <= 12 as c_int {
            (*param).vui.i_colorprim
        } else {
            2 as c_int
        };
    (*sps).vui.i_transfer =
        if (*param).vui.i_transfer >= 0 as c_int && (*param).vui.i_transfer <= 18 as c_int {
            (*param).vui.i_transfer
        } else {
            2 as c_int
        };
    (*sps).vui.i_colmatrix =
        if (*param).vui.i_colmatrix >= 0 as c_int && (*param).vui.i_colmatrix <= 14 as c_int {
            (*param).vui.i_colmatrix
        } else if csp >= X264_CSP_BGR {
            0 as c_int
        } else {
            2 as c_int
        };
    if (*sps).vui.i_colorprim != 2 as c_int
        || (*sps).vui.i_transfer != 2 as c_int
        || (*sps).vui.i_colmatrix != 2 as c_int
    {
        (*sps).vui.b_color_description_present = 1 as c_int;
    }
    if (*sps).vui.i_vidformat != 5 as c_int
        || (*sps).vui.b_fullrange != 0
        || (*sps).vui.b_color_description_present != 0
    {
        (*sps).vui.b_signal_type_present = 1 as c_int;
    }
    (*sps).vui.b_chroma_loc_info_present = ((*param).vui.i_chroma_loc > 0 as c_int
        && (*param).vui.i_chroma_loc <= 5 as c_int
        && (*sps).i_chroma_format_idc == CHROMA_420 as c_int)
        as c_int;
    if (*sps).vui.b_chroma_loc_info_present != 0 {
        (*sps).vui.i_chroma_loc_top = (*param).vui.i_chroma_loc;
        (*sps).vui.i_chroma_loc_bottom = (*param).vui.i_chroma_loc;
    }
    (*sps).vui.b_timing_info_present = ((*param).i_timebase_num > 0 as uint32_t
        && (*param).i_timebase_den > 0 as uint32_t) as c_int;
    if (*sps).vui.b_timing_info_present != 0 {
        (*sps).vui.i_num_units_in_tick = (*param).i_timebase_num;
        (*sps).vui.i_time_scale = (*param).i_timebase_den.wrapping_mul(2 as uint32_t);
        (*sps).vui.b_fixed_frame_rate = (!(*param).vfr_input) as c_int;
    }
    (*sps).vui.b_vcl_hrd_parameters_present = 0 as c_int;
    (*sps).vui.b_nal_hrd_parameters_present = ((*param).i_nal_hrd != 0) as c_int;
    (*sps).vui.pic_struct_present = (*param).pic_struct;
    (*sps).vui.b_bitstream_restriction =
        !((*sps).b_constraint_set3 != 0 && (*sps).i_profile_idc >= PROFILE_HIGH as c_int) as c_int;
    if (*sps).vui.b_bitstream_restriction != 0 {
        (*sps).vui.b_motion_vectors_over_pic_boundaries = 1 as c_int;
        (*sps).vui.i_max_bytes_per_pic_denom = 0 as c_int;
        (*sps).vui.i_max_bits_per_mb_denom = 0 as c_int;
        (*sps).vui.i_log2_max_mv_length_vertical = log2f(
            (if 1 as c_int > (*param).analyse.i_mv_range * 4 as c_int - 1 as c_int {
                1 as c_int
            } else {
                (*param).analyse.i_mv_range * 4 as c_int - 1 as c_int
            }) as c_float,
        ) as c_int
            + 1 as c_int;
        (*sps).vui.i_log2_max_mv_length_horizontal = (*sps).vui.i_log2_max_mv_length_vertical;
    }
    (*sps).b_avcintra_hd =
        ((*param).i_avcintra_class != 0 && (*param).i_avcintra_class <= 200 as c_int) as c_int;
    (*sps).b_avcintra_4k = ((*param).i_avcintra_class > 200 as c_int) as c_int;
    (*sps).i_cqm_preset = (*param).i_cqm_preset;
}
#[no_mangle]
#[c2rust::src_loc = "249:1"]
unsafe extern "C" fn x264_10_sps_init_reconfigurable(
    mut sps: *mut x264_sps_t,
    mut param: *mut x264_param_t,
) {
    (*sps).crop.i_left = (*param).crop_rect.left as c_int;
    (*sps).crop.i_top = (*param).crop_rect.top as c_int;
    (*sps).crop.i_right =
        (*param).crop_rect.right as c_int + (*sps).i_mb_width * 16 - (*param).width as c_int;
    (*sps).crop.i_bottom =
        (*param).crop_rect.bottom as c_int + (*sps).i_mb_height * 16 - (*param).height as c_int;
    (*sps).b_crop = ((*sps).crop.i_left != 0
        || (*sps).crop.i_top != 0
        || (*sps).crop.i_right != 0
        || (*sps).crop.i_bottom != 0) as c_int;
    (*sps).vui.b_aspect_ratio_info_present = 0 as c_int;
    if (*param).vui.i_sar_width > 0 as c_int && (*param).vui.i_sar_height > 0 as c_int {
        (*sps).vui.b_aspect_ratio_info_present = 1 as c_int;
        (*sps).vui.i_sar_width = (*param).vui.i_sar_width;
        (*sps).vui.i_sar_height = (*param).vui.i_sar_height;
    }
}
#[no_mangle]
#[c2rust::src_loc = "267:1"]
unsafe extern "C" fn x264_10_sps_init_scaling_list(
    mut sps: *mut x264_sps_t,
    mut param: *mut x264_param_t,
) {
    match (*sps).i_cqm_preset {
        X264_CQM_FLAT => {
            let mut i: c_int = 0 as c_int;
            while i < 8 as c_int {
                (*sps).scaling_list[i as usize] = x264_cqm_flat16.as_ptr();
                i += 1;
            }
        }
        X264_CQM_JVT => {
            let mut i_0: c_int = 0 as c_int;
            while i_0 < 8 as c_int {
                (*sps).scaling_list[i_0 as usize] = x264_cqm_jvt[i_0 as usize];
                i_0 += 1;
            }
        }
        X264_CQM_CUSTOM => {
            transpose((*param).cqm_4iy.as_mut_ptr(), 4 as c_int);
            transpose((*param).cqm_4py.as_mut_ptr(), 4 as c_int);
            transpose((*param).cqm_4ic.as_mut_ptr(), 4 as c_int);
            transpose((*param).cqm_4pc.as_mut_ptr(), 4 as c_int);
            transpose((*param).cqm_8iy.as_mut_ptr(), 8 as c_int);
            transpose((*param).cqm_8py.as_mut_ptr(), 8 as c_int);
            transpose((*param).cqm_8ic.as_mut_ptr(), 8 as c_int);
            transpose((*param).cqm_8pc.as_mut_ptr(), 8 as c_int);
            (*sps).scaling_list[CQM_4IY as c_int as usize] = (*param).cqm_4iy.as_mut_ptr();
            (*sps).scaling_list[CQM_4PY as c_int as usize] = (*param).cqm_4py.as_mut_ptr();
            (*sps).scaling_list[CQM_4IC as c_int as usize] = (*param).cqm_4ic.as_mut_ptr();
            (*sps).scaling_list[CQM_4PC as c_int as usize] = (*param).cqm_4pc.as_mut_ptr();
            (*sps).scaling_list[(CQM_8IY as c_int + 4 as c_int) as usize] =
                (*param).cqm_8iy.as_mut_ptr();
            (*sps).scaling_list[(CQM_8PY as c_int + 4 as c_int) as usize] =
                (*param).cqm_8py.as_mut_ptr();
            (*sps).scaling_list[(CQM_8IC as c_int + 4 as c_int) as usize] =
                (*param).cqm_8ic.as_mut_ptr();
            (*sps).scaling_list[(CQM_8PC as c_int + 4 as c_int) as usize] =
                (*param).cqm_8pc.as_mut_ptr();
            let mut i_1: c_int = 0 as c_int;
            while i_1 < 8 as c_int {
                let mut j: c_int = 0 as c_int;
                while j
                    < (if i_1 < 4 as c_int {
                        16 as c_int
                    } else {
                        64 as c_int
                    })
                {
                    if *(*sps).scaling_list[i_1 as usize].offset(j as isize) as c_int == 0 as c_int
                    {
                        (*sps).scaling_list[i_1 as usize] = x264_cqm_jvt[i_1 as usize];
                    }
                    j += 1;
                }
                i_1 += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
#[c2rust::src_loc = "305:1"]
unsafe extern "C" fn x264_10_sps_write(mut s: *mut bs_t, mut sps: *mut x264_sps_t) {
    bs_realign(s);
    bs_write(s, 8 as c_int, (*sps).i_profile_idc as uint32_t);
    bs_write1(s, (*sps).b_constraint_set0 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set1 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set2 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set3 as uint32_t);
    bs_write(s, 4 as c_int, 0 as uint32_t);
    bs_write(s, 8 as c_int, (*sps).i_level_idc as uint32_t);
    bs_write_ue_big(s, (*sps).i_id as c_uint);
    if (*sps).i_profile_idc >= PROFILE_HIGH as c_int {
        bs_write_ue_big(s, (*sps).i_chroma_format_idc as c_uint);
        if (*sps).i_chroma_format_idc == CHROMA_444 as c_int {
            bs_write1(s, 0 as uint32_t);
        }
        bs_write_ue_big(s, (BIT_DEPTH - 8 as c_int) as c_uint);
        bs_write_ue_big(s, (BIT_DEPTH - 8 as c_int) as c_uint);
        bs_write1(s, (*sps).b_qpprime_y_zero_transform_bypass as uint32_t);
        bs_write1(s, (*sps).b_avcintra_hd as uint32_t);
        if (*sps).b_avcintra_hd != 0 {
            scaling_list_write(s, sps, CQM_4IY as c_int);
            scaling_list_write(s, sps, CQM_4IC as c_int);
            scaling_list_write(s, sps, CQM_4IC as c_int);
            bs_write1(s, 0 as uint32_t);
            bs_write1(s, 0 as uint32_t);
            bs_write1(s, 0 as uint32_t);
            scaling_list_write(s, sps, CQM_8IY as c_int + 4 as c_int);
            bs_write1(s, 0 as uint32_t);
            if (*sps).i_chroma_format_idc == CHROMA_444 as c_int {
                scaling_list_write(s, sps, CQM_8IC as c_int + 4 as c_int);
                bs_write1(s, 0 as uint32_t);
                scaling_list_write(s, sps, CQM_8IC as c_int + 4 as c_int);
                bs_write1(s, 0 as uint32_t);
            }
        }
    }
    bs_write_ue_big(s, ((*sps).i_log2_max_frame_num - 4 as c_int) as c_uint);
    bs_write_ue_big(s, (*sps).i_poc_type as c_uint);
    if (*sps).i_poc_type == 0 as c_int {
        bs_write_ue_big(s, ((*sps).i_log2_max_poc_lsb - 4 as c_int) as c_uint);
    }
    bs_write_ue_big(s, (*sps).i_num_ref_frames as c_uint);
    bs_write1(s, (*sps).b_gaps_in_frame_num_value_allowed as uint32_t);
    bs_write_ue_big(s, ((*sps).i_mb_width - 1 as c_int) as c_uint);
    bs_write_ue_big(
        s,
        (((*sps).i_mb_height >> ((*sps).b_frame_mbs_only == 0) as c_int) - 1 as c_int) as c_uint,
    );
    bs_write1(s, (*sps).b_frame_mbs_only as uint32_t);
    if (*sps).b_frame_mbs_only == 0 {
        bs_write1(s, (*sps).mb_adaptive_frame_field as uint32_t);
    }
    bs_write1(s, (*sps).b_direct8x8_inference as uint32_t);
    bs_write1(s, (*sps).b_crop as uint32_t);
    if (*sps).b_crop != 0 {
        let mut h_shift: c_int = ((*sps).i_chroma_format_idc == CHROMA_420 as c_int
            || (*sps).i_chroma_format_idc == CHROMA_422 as c_int)
            as c_int;
        let mut v_shift: c_int = ((*sps).i_chroma_format_idc == CHROMA_420 as c_int) as c_int
            + ((*sps).b_frame_mbs_only == 0) as c_int;
        bs_write_ue_big(s, ((*sps).crop.i_left >> h_shift) as c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_right >> h_shift) as c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_top >> v_shift) as c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_bottom >> v_shift) as c_uint);
    }
    bs_write1(s, (*sps).b_vui as uint32_t);
    if (*sps).b_vui != 0 {
        bs_write1(s, (*sps).vui.b_aspect_ratio_info_present as uint32_t);
        if (*sps).vui.b_aspect_ratio_info_present != 0 {
            let mut i: c_int = 0;
            static mut sar: [C2RustUnnamed_19; 17] = [
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 1 as uint8_t,
                        h: 1 as uint8_t,
                        sar: 1 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 12 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 2 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 10 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 3 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 16 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 4 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 40 as uint8_t,
                        h: 33 as uint8_t,
                        sar: 5 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 24 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 6 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 20 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 7 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 32 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 8 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 80 as uint8_t,
                        h: 33 as uint8_t,
                        sar: 9 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 18 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 10 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 15 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 11 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 64 as uint8_t,
                        h: 33 as uint8_t,
                        sar: 12 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 160 as uint8_t,
                        h: 99 as uint8_t,
                        sar: 13 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 4 as uint8_t,
                        h: 3 as uint8_t,
                        sar: 14 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 3 as uint8_t,
                        h: 2 as uint8_t,
                        sar: 15 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 2 as uint8_t,
                        h: 1 as uint8_t,
                        sar: 16 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 0 as uint8_t,
                        h: 0 as uint8_t,
                        sar: 255 as uint8_t,
                    };
                    init
                },
            ];
            i = 0 as c_int;
            while sar[i as usize].sar as c_int != 255 as c_int {
                if sar[i as usize].w as c_int == (*sps).vui.i_sar_width
                    && sar[i as usize].h as c_int == (*sps).vui.i_sar_height
                {
                    break;
                }
                i += 1;
            }
            bs_write(s, 8 as c_int, sar[i as usize].sar as uint32_t);
            if sar[i as usize].sar as c_int == 255 as c_int {
                bs_write(s, 16 as c_int, (*sps).vui.i_sar_width as uint32_t);
                bs_write(s, 16 as c_int, (*sps).vui.i_sar_height as uint32_t);
            }
        }
        bs_write1(s, (*sps).vui.b_overscan_info_present as uint32_t);
        if (*sps).vui.b_overscan_info_present != 0 {
            bs_write1(s, (*sps).vui.b_overscan_info as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_signal_type_present as uint32_t);
        if (*sps).vui.b_signal_type_present != 0 {
            bs_write(s, 3 as c_int, (*sps).vui.i_vidformat as uint32_t);
            bs_write1(s, (*sps).vui.b_fullrange as uint32_t);
            bs_write1(s, (*sps).vui.b_color_description_present as uint32_t);
            if (*sps).vui.b_color_description_present != 0 {
                bs_write(s, 8 as c_int, (*sps).vui.i_colorprim as uint32_t);
                bs_write(s, 8 as c_int, (*sps).vui.i_transfer as uint32_t);
                bs_write(s, 8 as c_int, (*sps).vui.i_colmatrix as uint32_t);
            }
        }
        bs_write1(s, (*sps).vui.b_chroma_loc_info_present as uint32_t);
        if (*sps).vui.b_chroma_loc_info_present != 0 {
            bs_write_ue_big(s, (*sps).vui.i_chroma_loc_top as c_uint);
            bs_write_ue_big(s, (*sps).vui.i_chroma_loc_bottom as c_uint);
        }
        bs_write1(s, (*sps).vui.b_timing_info_present as uint32_t);
        if (*sps).vui.b_timing_info_present != 0 {
            bs_write32(s, (*sps).vui.i_num_units_in_tick);
            bs_write32(s, (*sps).vui.i_time_scale);
            bs_write1(s, (*sps).vui.b_fixed_frame_rate as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_nal_hrd_parameters_present as uint32_t);
        if (*sps).vui.b_nal_hrd_parameters_present != 0 {
            bs_write_ue_big(s, ((*sps).vui.hrd.i_cpb_cnt - 1 as c_int) as c_uint);
            bs_write(s, 4 as c_int, (*sps).vui.hrd.i_bit_rate_scale as uint32_t);
            bs_write(s, 4 as c_int, (*sps).vui.hrd.i_cpb_size_scale as uint32_t);
            bs_write_ue_big(s, ((*sps).vui.hrd.i_bit_rate_value - 1 as c_int) as c_uint);
            bs_write_ue_big(s, ((*sps).vui.hrd.i_cpb_size_value - 1 as c_int) as c_uint);
            bs_write1(s, (*sps).vui.hrd.b_cbr_hrd as uint32_t);
            bs_write(
                s,
                5 as c_int,
                ((*sps).vui.hrd.i_initial_cpb_removal_delay_length - 1 as c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as c_int,
                ((*sps).vui.hrd.i_cpb_removal_delay_length - 1 as c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as c_int,
                ((*sps).vui.hrd.i_dpb_output_delay_length - 1 as c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as c_int,
                (*sps).vui.hrd.i_time_offset_length as uint32_t,
            );
        }
        bs_write1(s, (*sps).vui.b_vcl_hrd_parameters_present as uint32_t);
        if (*sps).vui.b_nal_hrd_parameters_present != 0
            || (*sps).vui.b_vcl_hrd_parameters_present != 0
        {
            bs_write1(s, 0 as uint32_t);
        }
        bs_write1(s, (*sps).vui.pic_struct_present as uint32_t);
        bs_write1(s, (*sps).vui.b_bitstream_restriction as uint32_t);
        if (*sps).vui.b_bitstream_restriction != 0 {
            bs_write1(
                s,
                (*sps).vui.b_motion_vectors_over_pic_boundaries as uint32_t,
            );
            bs_write_ue_big(s, (*sps).vui.i_max_bytes_per_pic_denom as c_uint);
            bs_write_ue_big(s, (*sps).vui.i_max_bits_per_mb_denom as c_uint);
            bs_write_ue_big(s, (*sps).vui.i_log2_max_mv_length_horizontal as c_uint);
            bs_write_ue_big(s, (*sps).vui.i_log2_max_mv_length_vertical as c_uint);
            bs_write_ue_big(s, (*sps).vui.i_num_reorder_frames as c_uint);
            bs_write_ue_big(s, (*sps).vui.i_max_dec_frame_buffering as c_uint);
        }
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
#[c2rust::src_loc = "479:1"]
unsafe extern "C" fn x264_10_pps_init(
    mut pps: *mut x264_pps_t,
    mut i_id: c_int,
    mut param: *mut x264_param_t,
    mut sps: *mut x264_sps_t,
) {
    (*pps).i_id = i_id;
    (*pps).i_sps_id = (*sps).i_id;
    (*pps).cabac = (*param).cabac;
    (*pps).b_pic_order = ((*param).i_avcintra_class == 0 && (*param).interlaced) as c_int;
    (*pps).i_num_slice_groups = 1 as c_int;
    (*pps).i_num_ref_idx_l0_default_active = (*param).i_frame_reference;
    (*pps).i_num_ref_idx_l1_default_active = 1 as c_int;
    (*pps).b_weighted_pred = ((*param).analyse.i_weighted_pred > 0 as c_int) as c_int;
    (*pps).b_weighted_bipred = if (*param).analyse.weighted_bipred {
        2
    } else {
        0
    };
    (*pps).i_pic_init_qp = if (*param).rc.i_rc_method == RateControlMode::ABR || (*param).stitchable
    {
        26 + QP_BD_OFFSET
    } else if (*param).rc.i_qp_constant < 51 + 6 * (10 - 8) {
        (*param).rc.i_qp_constant
    } else {
        51 + 6 * (10 - 8)
    };
    (*pps).i_pic_init_qs = 26 as c_int + QP_BD_OFFSET;
    (*pps).i_chroma_qp_index_offset = (*param).analyse.i_chroma_qp_offset;
    (*pps).b_deblocking_filter_control = 1 as c_int;
    (*pps).constrained_intra_pred = (*param).constrained_intra;
    (*pps).b_redundant_pic_cnt = 0 as c_int;
    (*pps).b_transform_8x8_mode = if (*param).analyse.transform_8x8 { 1 } else { 0 };
}
#[no_mangle]
#[c2rust::src_loc = "505:1"]
unsafe extern "C" fn x264_10_pps_write(
    mut s: *mut bs_t,
    mut sps: *mut x264_sps_t,
    mut pps: *mut x264_pps_t,
) {
    bs_realign(s);
    bs_write_ue_big(s, (*pps).i_id as c_uint);
    bs_write_ue_big(s, (*pps).i_sps_id as c_uint);
    bs_write1(s, (*pps).cabac as uint32_t);
    bs_write1(s, (*pps).b_pic_order as uint32_t);
    bs_write_ue_big(s, ((*pps).i_num_slice_groups - 1 as c_int) as c_uint);
    bs_write_ue_big(
        s,
        ((*pps).i_num_ref_idx_l0_default_active - 1 as c_int) as c_uint,
    );
    bs_write_ue_big(
        s,
        ((*pps).i_num_ref_idx_l1_default_active - 1 as c_int) as c_uint,
    );
    bs_write1(s, (*pps).b_weighted_pred as uint32_t);
    bs_write(s, 2 as c_int, (*pps).b_weighted_bipred as uint32_t);
    bs_write_se(s, (*pps).i_pic_init_qp - 26 as c_int - QP_BD_OFFSET);
    bs_write_se(s, (*pps).i_pic_init_qs - 26 as c_int - QP_BD_OFFSET);
    bs_write_se(s, (*pps).i_chroma_qp_index_offset);
    bs_write1(s, (*pps).b_deblocking_filter_control as uint32_t);
    bs_write1(s, (*pps).constrained_intra_pred as uint32_t);
    bs_write1(s, (*pps).b_redundant_pic_cnt as uint32_t);
    let mut b_scaling_list: c_int =
        ((*sps).b_avcintra_hd == 0 && (*sps).i_cqm_preset != X264_CQM_FLAT) as c_int;
    if (*pps).b_transform_8x8_mode != 0 || b_scaling_list != 0 {
        bs_write1(s, (*pps).b_transform_8x8_mode as uint32_t);
        bs_write1(s, b_scaling_list as uint32_t);
        if b_scaling_list != 0 {
            scaling_list_write(s, sps, CQM_4IY as c_int);
            scaling_list_write(s, sps, CQM_4IC as c_int);
            if (*sps).b_avcintra_4k != 0 {
                scaling_list_write(s, sps, CQM_4IC as c_int);
                bs_write1(s, 0 as uint32_t);
                bs_write1(s, 0 as uint32_t);
                bs_write1(s, 0 as uint32_t);
            } else {
                bs_write1(s, 0 as uint32_t);
                scaling_list_write(s, sps, CQM_4PY as c_int);
                scaling_list_write(s, sps, CQM_4PC as c_int);
                bs_write1(s, 0 as uint32_t);
            }
            if (*pps).b_transform_8x8_mode != 0 {
                scaling_list_write(s, sps, CQM_8IY as c_int + 4 as c_int);
                if (*sps).b_avcintra_4k != 0 {
                    bs_write1(s, 0 as uint32_t);
                } else {
                    scaling_list_write(s, sps, CQM_8PY as c_int + 4 as c_int);
                }
                if (*sps).i_chroma_format_idc == CHROMA_444 as c_int {
                    scaling_list_write(s, sps, CQM_8IC as c_int + 4 as c_int);
                    scaling_list_write(s, sps, CQM_8PC as c_int + 4 as c_int);
                    bs_write1(s, 0 as uint32_t);
                    bs_write1(s, 0 as uint32_t);
                }
            }
        }
        bs_write_se(s, (*pps).i_chroma_qp_index_offset);
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
#[c2rust::src_loc = "574:1"]
unsafe extern "C" fn x264_10_sei_recovery_point_write(
    mut _h: *mut x264_t,
    mut s: *mut bs_t,
    mut recovery_frame_cnt: c_int,
) {
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(&mut q, tmp_buf.as_mut_ptr() as *mut c_void, 100 as c_int);
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, recovery_frame_cnt as c_uint);
    bs_write1(&mut q, 1 as uint32_t);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write(&mut q, 2 as c_int, 0 as uint32_t);
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as c_int,
        SEI_RECOVERY_POINT as c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "593:1"]
unsafe extern "C" fn x264_10_sei_version_write(mut h: *mut x264_t, mut s: *mut bs_t) -> c_int {
    static mut uuid: [uint8_t; 16] = [
        0xdc as c_int as uint8_t,
        0x45 as c_int as uint8_t,
        0xe9 as c_int as uint8_t,
        0xbd as c_int as uint8_t,
        0xe6 as c_int as uint8_t,
        0xd9 as c_int as uint8_t,
        0x48 as c_int as uint8_t,
        0xb7 as c_int as uint8_t,
        0x96 as c_int as uint8_t,
        0x2c as c_int as uint8_t,
        0xd8 as c_int as uint8_t,
        0x20 as c_int as uint8_t,
        0xd9 as c_int as uint8_t,
        0x23 as c_int as uint8_t,
        0xee as c_int as uint8_t,
        0xef as c_int as uint8_t,
    ];
    let mut opts: *mut c_char = x264_param2string(&mut (*h).param, 0 as c_int);
    let mut payload: *mut c_char = 0 as *mut c_char;
    let mut length: c_int = 0;
    if opts.is_null() {
        return -1;
    }
    payload = x264_malloc((200 as size_t).wrapping_add(strlen(opts)) as int64_t) as *mut c_char;
    if payload.is_null() {
        x264_free(opts as *mut c_void);
        return -1;
    } else {
        memcpy(
            payload as *mut c_void,
            uuid.as_ptr() as *const c_void,
            16 as size_t,
        );
        sprintf(
            payload.offset(16 as c_int as isize),
            b"x264 - core %d%s - H.264/MPEG-4 AVC codec - Copy%s 2003-2025 - http://www.videolan.org/x264.html - options: %s\0"
                as *const u8 as *const c_char,
            X264_BUILD,
            X264_VERSION.as_ptr(),
            if HAVE_GPL != 0 {
                b"left\0" as *const u8 as *const c_char
            } else {
                b"right\0" as *const u8 as *const c_char
            },
            opts,
        );
        length = strlen(payload).wrapping_add(1 as size_t) as c_int;
        x264_10_sei_write(
            s,
            payload as *mut uint8_t,
            length,
            SEI_USER_DATA_UNREGISTERED as c_int,
        );
        x264_free(opts as *mut c_void);
        x264_free(payload as *mut c_void);
        return 0 as c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "625:1"]
unsafe extern "C" fn x264_10_sei_buffering_period_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut sps: *mut x264_sps_t = (*h).sps.as_mut_ptr();
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(&mut q, tmp_buf.as_mut_ptr() as *mut c_void, 100 as c_int);
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, (*sps).i_id as c_uint);
    if (*sps).vui.b_nal_hrd_parameters_present != 0 {
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
            (*h).initial_cpb_removal_delay as uint32_t,
        );
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
            (*h).initial_cpb_removal_delay_offset as uint32_t,
        );
    }
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as c_int,
        SEI_BUFFERING_PERIOD as c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "647:1"]
unsafe extern "C" fn x264_10_sei_pic_timing_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut sps: *mut x264_sps_t = (*h).sps.as_mut_ptr();
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(&mut q, tmp_buf.as_mut_ptr() as *mut c_void, 100 as c_int);
    bs_realign(&mut q);
    if (*sps).vui.b_nal_hrd_parameters_present != 0 || (*sps).vui.b_vcl_hrd_parameters_present != 0
    {
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_cpb_removal_delay_length,
            ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset) as uint32_t,
        );
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_dpb_output_delay_length,
            (*(*h).fenc).i_dpb_output_delay as uint32_t,
        );
    }
    if (*sps).vui.pic_struct_present {
        bs_write(
            &mut q,
            4 as c_int,
            ((*(*h).fenc).i_pic_struct - 1 as c_int) as uint32_t,
        );
        let mut i: c_int = 0 as c_int;
        while i < num_clock_ts[(*(*h).fenc).i_pic_struct as usize] as c_int {
            bs_write1(&mut q, 0 as uint32_t);
            i += 1;
        }
    }
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as c_int,
        SEI_PIC_TIMING as c_int,
    );
}

pub unsafe fn x264_sei_frame_packing_write(
    frame_packing: FramePacking,
    i_frame: c_int,
    mut s: *mut bs_t,
) {
    let mut quincunx_sampling_flag: c_int = (frame_packing == FramePacking::Checkerboard) as c_int;
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(&mut q, tmp_buf.as_mut_ptr() as *mut c_void, 100 as c_int);
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, 0 as c_uint);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write(&mut q, 7 as c_int, frame_packing as u32);
    bs_write1(&mut q, quincunx_sampling_flag as uint32_t);
    bs_write(
        &mut q,
        6 as c_int,
        (frame_packing != FramePacking::Packing2D) as c_int as uint32_t,
    );
    bs_write1(&mut q, 0 as uint32_t);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write1(
        &mut q,
        (frame_packing == FramePacking::TemporalInterleaved && i_frame & 1 as c_int == 0) as c_int
            as uint32_t,
    );
    bs_write1(&mut q, 0 as uint32_t);
    bs_write1(&mut q, 0 as uint32_t);
    if quincunx_sampling_flag == 0 as c_int && frame_packing != FramePacking::TemporalInterleaved {
        bs_write(&mut q, 4 as c_int, 0 as uint32_t);
        bs_write(&mut q, 4 as c_int, 0 as uint32_t);
        bs_write(&mut q, 4 as c_int, 0 as uint32_t);
        bs_write(&mut q, 4 as c_int, 0 as uint32_t);
    }
    bs_write(&mut q, 8 as c_int, 0 as uint32_t);
    bs_write_ue_big(
        &mut q,
        (frame_packing != FramePacking::TemporalInterleaved) as c_int as c_uint,
    );
    bs_write1(&mut q, 0 as uint32_t);
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as c_int,
        SEI_FRAME_PACKING as c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "720:1"]
pub unsafe fn x264_sei_mastering_display_write(
    mastering_display: &MasteringDisplay,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(&mut q, tmp_buf.as_mut_ptr() as *mut c_void, 100 as c_int);
    bs_realign(&mut q);
    bs_write(&mut q, 16 as c_int, mastering_display.green.0.into());
    bs_write(&mut q, 16 as c_int, mastering_display.green.1.into());
    bs_write(&mut q, 16 as c_int, mastering_display.blue.0.into());
    bs_write(&mut q, 16 as c_int, mastering_display.blue.1.into());
    bs_write(&mut q, 16 as c_int, mastering_display.red.0.into());
    bs_write(&mut q, 16 as c_int, mastering_display.red.1.into());
    bs_write(&mut q, 16 as c_int, mastering_display.white.0.into());
    bs_write(&mut q, 16 as c_int, mastering_display.white.1.into());
    bs_write32(&mut q, mastering_display.display_max);
    bs_write32(&mut q, mastering_display.display_min);
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as c_int,
        SEI_MASTERING_DISPLAY as c_int,
    );
}

#[no_mangle]
#[c2rust::src_loc = "745:1"]
pub unsafe extern "C" fn x264_sei_content_light_level_write(
    light_level: &ContentLightLevel,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(&mut q, tmp_buf.as_mut_ptr() as *mut c_void, 100 as c_int);
    bs_realign(&mut q);
    bs_write(&mut q, 16 as c_int, light_level.max_cll as uint32_t);
    bs_write(&mut q, 16 as c_int, light_level.max_fall as uint32_t);
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as c_int,
        SEI_CONTENT_LIGHT_LEVEL as c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "762:1"]
unsafe extern "C" fn x264_10_sei_alternative_transfer_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(&mut q, tmp_buf.as_mut_ptr() as *mut c_void, 100 as c_int);
    bs_realign(&mut q);
    bs_write(
        &mut q,
        8 as c_int,
        (*h).param.i_alternative_transfer as uint32_t,
    );
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as c_int,
        SEI_ALTERNATIVE_TRANSFER as c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "778:1"]
unsafe extern "C" fn x264_10_filler_write(
    mut _h: *mut x264_t,
    mut s: *mut bs_t,
    mut filler: c_int,
) {
    bs_realign(s);
    let mut i: c_int = 0 as c_int;
    while i < filler {
        bs_write(s, 8 as c_int, 0xff as uint32_t);
        i += 1;
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
#[c2rust::src_loc = "789:1"]
unsafe extern "C" fn x264_10_sei_dec_ref_pic_marking_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut sh: *mut x264_slice_header_t = &mut (*h).sh_backup;
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(&mut q, tmp_buf.as_mut_ptr() as *mut c_void, 100 as c_int);
    bs_realign(&mut q);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write_ue_big(&mut q, (*sh).i_frame_num as c_uint);
    if (*(*h).sps.as_mut_ptr()).b_frame_mbs_only == 0 {
        bs_write1(&mut q, 0 as uint32_t);
    }
    bs_write1(
        &mut q,
        ((*sh).i_mmco_command_count > 0 as c_int) as c_int as uint32_t,
    );
    if (*sh).i_mmco_command_count > 0 as c_int {
        let mut i: c_int = 0 as c_int;
        while i < (*sh).i_mmco_command_count {
            bs_write_ue_big(&mut q, 1 as c_uint);
            bs_write_ue_big(
                &mut q,
                ((*sh).mmco[i as usize].i_difference_of_pic_nums - 1 as c_int) as c_uint,
            );
            i += 1;
        }
        bs_write_ue_big(&mut q, 0 as c_uint);
    }
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as c_int,
        SEI_DEC_REF_PIC_MARKING as c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "821:1"]
unsafe extern "C" fn x264_10_sei_avcintra_umid_write(
    mut h: *mut x264_t,
    mut _s: *mut bs_t,
) -> c_int {
    let mut data: [uint8_t; 512] = [0; 512];
    let mut msg: *const c_char = b"UMID\0" as *const u8 as *const c_char;
    let len: c_int = 497 as c_int;
    memset(
        data.as_mut_ptr() as *mut c_void,
        0xff as c_int,
        len as size_t,
    );
    memcpy(
        data.as_mut_ptr() as *mut c_void,
        avcintra_uuid.as_ptr() as *const c_void,
        size_of::<[uint8_t; 16]>() as size_t,
    );
    memcpy(
        data.as_mut_ptr().offset(16 as c_int as isize) as *mut c_void,
        msg as *const c_void,
        strlen(msg),
    );
    data[20 as c_int as usize] = 0x13 as uint8_t;
    data[26 as c_int as usize] = 0 as uint8_t;
    data[25 as c_int as usize] = data[26 as c_int as usize];
    data[23 as c_int as usize] = data[25 as c_int as usize];
    data[22 as c_int as usize] = data[23 as c_int as usize];
    data[28 as c_int as usize] = 0x14 as uint8_t;
    data[34 as c_int as usize] = 0 as uint8_t;
    data[33 as c_int as usize] = data[34 as c_int as usize];
    data[31] = data[33 as c_int as usize];
    data[30 as c_int as usize] = data[31];
    data[36 as c_int as usize] = 0x60 as uint8_t;
    data[41 as c_int as usize] = 0x22 as uint8_t;
    data[60 as c_int as usize] = 0x62 as uint8_t;
    data[66 as c_int as usize] = 0 as uint8_t;
    data[65] = data[66 as c_int as usize];
    data[63] = data[65];
    data[62 as c_int as usize] = data[63];
    data[68 as c_int as usize] = 0x63 as uint8_t;
    data[74 as c_int as usize] = 0 as uint8_t;
    data[73 as c_int as usize] = data[74 as c_int as usize];
    data[71 as c_int as usize] = data[73 as c_int as usize];
    data[70 as c_int as usize] = data[71 as c_int as usize];
    x264_10_sei_write(
        &mut (*h).out.bs,
        data.as_mut_ptr(),
        len,
        SEI_USER_DATA_UNREGISTERED as c_int,
    );
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "849:1"]
unsafe extern "C" fn x264_10_sei_avcintra_vanc_write(
    mut h: *mut x264_t,
    mut _s: *mut bs_t,
    mut len: c_int,
) -> c_int {
    let mut data: [uint8_t; 6000] = [0; 6000];
    let mut msg: *const c_char = b"VANC\0" as *const u8 as *const c_char;
    if len < 0 as c_int || len as c_uint as usize > size_of::<[uint8_t; 6000]>() as usize {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"AVC-Intra SEI is too large (%d)\n\0" as *const u8 as *const c_char,
            len,
        );
        return -1;
    }
    memset(data.as_mut_ptr() as _, 0xff, len as usize);
    memcpy(
        data.as_mut_ptr() as _,
        avcintra_uuid.as_ptr() as _,
        size_of::<[u8; 16]>(),
    );
    memcpy(
        data.as_mut_ptr().offset(16 as c_int as isize) as *mut c_void,
        msg as *const c_void,
        strlen(msg),
    );
    x264_10_sei_write(
        &mut (*h).out.bs,
        data.as_mut_ptr(),
        len,
        SEI_USER_DATA_UNREGISTERED as c_int,
    );
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "876:1"]
unsafe extern "C" fn x264_10_validate_levels(mut h: *mut x264_t, mut verbose: c_int) -> c_int {
    let mut ret: c_int = 0 as c_int;
    let mut mbs: c_int = (*(*h).sps.as_mut_ptr()).i_mb_width * (*(*h).sps.as_mut_ptr()).i_mb_height;
    let mut dpb: c_int = mbs * (*(*h).sps.as_mut_ptr()).vui.i_max_dec_frame_buffering;
    let mut cbp_factor: c_int =
        if (*(*h).sps.as_mut_ptr()).i_profile_idc >= PROFILE_HIGH422 as c_int {
            16 as c_int
        } else if (*(*h).sps.as_mut_ptr()).i_profile_idc == PROFILE_HIGH10 as c_int {
            12 as c_int
        } else if (*(*h).sps.as_mut_ptr()).i_profile_idc == PROFILE_HIGH as c_int {
            5 as c_int
        } else {
            4 as c_int
        };
    let mut l: *const x264_level_t = x264_levels.as_ptr();
    while (*l).level_idc as c_int != 0 as c_int && (*l).level_idc as c_int != (*h).param.i_level_idc
    {
        l = l.offset(1);
    }
    if (*l).frame_size < mbs as int32_t
        || ((*l).frame_size * 8 as int32_t)
            < (*(*h).sps.as_mut_ptr()).i_mb_width as int32_t
                * (*(*h).sps.as_mut_ptr()).i_mb_width as int32_t
        || ((*l).frame_size * 8 as int32_t)
            < (*(*h).sps.as_mut_ptr()).i_mb_height as int32_t
                * (*(*h).sps.as_mut_ptr()).i_mb_height as int32_t
    {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"frame MB size (%dx%d) > level limit (%d)\n\0" as *const u8 as *const c_char,
                (*(*h).sps.as_mut_ptr()).i_mb_width,
                (*(*h).sps.as_mut_ptr()).i_mb_height,
                (*l).frame_size,
            );
        }
        ret = 1 as c_int;
    }
    if dpb as int32_t > (*l).dpb {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"DPB size (%d frames, %d mbs) > level limit (%d frames, %d mbs)\n\0" as *const u8
                    as *const c_char,
                (*(*h).sps.as_mut_ptr()).vui.i_max_dec_frame_buffering,
                dpb,
                (*l).dpb / mbs as int32_t,
                (*l).dpb,
            );
        }
        ret = 1 as c_int;
    }
    if (*h).param.rc.i_vbv_max_bitrate as int32_t
        > (*l).bitrate * cbp_factor as int32_t / 4 as int32_t
    {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV bitrate (%ld) > level limit (%d)\n\0" as *const u8 as *const c_char,
                (*h).param.rc.i_vbv_max_bitrate as int64_t,
                (*l).bitrate * cbp_factor as int32_t / 4 as int32_t,
            );
        }
        ret = 1 as c_int;
    }
    if (*h).param.rc.i_vbv_buffer_size as int32_t > (*l).cpb * cbp_factor as int32_t / 4 as int32_t
    {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV buffer (%ld) > level limit (%d)\n\0" as *const u8 as *const c_char,
                (*h).param.rc.i_vbv_buffer_size as int64_t,
                (*l).cpb * cbp_factor as int32_t / 4 as int32_t,
            );
        }
        ret = 1 as c_int;
    }
    if (*h).param.analyse.i_mv_range > (*l).mv_range as c_int {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"MV range (%ld) > level limit (%d)\n\0" as *const u8 as *const c_char,
                (*h).param.analyse.i_mv_range as int64_t,
                (*l).mv_range as c_int,
            );
        }
        ret = 1 as c_int;
    }
    if (*h).param.interlaced as i32 > ((*l).frame_only == 0) as c_int {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"interlaced (%ld) > level limit (%d)\n\0" as *const u8 as *const c_char,
                (*h).param.interlaced as int64_t,
                ((*l).frame_only == 0) as c_int,
            );
        }
        ret = 1 as c_int;
    }
    if (*h).param.fake_interlaced as i32 > ((*l).frame_only == 0) as c_int {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"fake interlaced (%ld) > level limit (%d)\n\0" as *const u8 as *const c_char,
                (*h).param.fake_interlaced as int64_t,
                ((*l).frame_only == 0) as c_int,
            );
        }
        ret = 1 as c_int;
    }
    if (*h).param.i_fps_den > 0 as uint32_t {
        if mbs as int64_t * (*h).param.i_fps_num as int64_t / (*h).param.i_fps_den as int64_t
            > (*l).mbps as int64_t
        {
            if verbose != 0 {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"MB rate (%ld) > level limit (%d)\n\0" as *const u8 as *const c_char,
                    mbs as int64_t * (*h).param.i_fps_num as int64_t
                        / (*h).param.i_fps_den as int64_t,
                    (*l).mbps,
                );
            }
            ret = 1 as c_int;
        }
    }
    return ret;
}
