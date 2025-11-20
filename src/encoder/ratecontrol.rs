use ::core::ffi::{c_char, c_double, c_float, c_int, c_long, c_uint, c_ulong, c_void};

use crate::__stddef_null_h::NULL;
use crate::__stddef_size_t_h::size_t;
use crate::assert_h::__assert_fail;
use crate::base_h::{
    slice_type_to_char, x264_clip3, x264_clip3f, x264_exp2fix8, x264_free, x264_log2, x264_malloc,
    x264_param2string, x264_reduce_fraction64, x264_slurp_file, CHROMA_444, PROFILE_HIGH,
    SLICE_TYPE_B, SLICE_TYPE_I, SLICE_TYPE_P,
};
use crate::common_h::{pixel, x264_10_log, x264_t, FENC_STRIDE, QP_BD_OFFSET, QP_MAX, QP_MAX_SPEC};
use crate::frame_h::x264_frame_t;
use crate::internal::BIT_DEPTH;
use crate::limits_h::INT_MAX;
use crate::macroblock_h::{
    x264_10_prefetch_fenc, B_8x8, I_16x16, I_4x4, I_8x8, P_8x8, B_DIRECT, B_SKIP, I_PCM, P_L0,
    P_SKIP,
};
use crate::mathcalls_h::{ceil, exp, fabs, log, log2, log2f, pow, powf, sqrt};
use crate::mc_h::{weight_fn_t, x264_weight_t};
use crate::osdep_h::x264_is_regular_file;
use crate::pixel_h::PIXEL_16x16;
use crate::ratecontrol_h::{x264_10_encoder_reconfig_apply, x264_10_rc_analyse_slice};
use crate::stdint_h::intptr_t;
use crate::stdint_intn_h::{int16_t, int32_t, int64_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{fclose, fopen, fprintf, fread, fwrite, rename, sprintf, sscanf};
use crate::stdlib_h::abs;
use crate::string_h::{
    memcpy, memset, strcat, strchr, strcmp, strcpy, strcspn, strlen, strncmp, strstr, strtok_r,
};
use crate::x264_h::{
    x264_level_t, x264_levels, x264_param_cleanup, x264_param_parse, x264_param_t, x264_zone_t,
    FramePacking, X264_AQ_AUTOVARIANCE, X264_AQ_AUTOVARIANCE_BIASED, X264_AQ_NONE,
    X264_B_ADAPT_NONE, X264_B_ADAPT_TRELLIS, X264_DIRECT_PRED_AUTO, X264_KEYINT_MAX_INFINITE,
    X264_LOG_DEBUG, X264_LOG_ERROR, X264_LOG_INFO, X264_LOG_WARNING, X264_NAL_HRD_CBR,
    X264_QP_AUTO, X264_RC_ABR, X264_RC_CQP, X264_RC_CRF, X264_TYPE_AUTO, X264_TYPE_B,
    X264_TYPE_BREF, X264_TYPE_I, X264_TYPE_IDR, X264_TYPE_KEYFRAME, X264_TYPE_P,
    X264_WEIGHTP_SIMPLE,
};
use crate::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "72:8"]
pub struct x264_ratecontrol_t {
    b_abr: c_int,
    b_2pass: c_int,
    b_vbv: c_int,
    b_vbv_min_rate: c_int,
    fps: c_double,
    bitrate: c_double,
    rate_tolerance: c_double,
    qcompress: c_double,
    nmb: c_int,
    qp_constant: [c_int; 3],
    rce: *mut ratecontrol_entry_t,
    qpm: c_float,
    qpa_rc: c_float,
    qpa_rc_prev: c_float,
    qpa_aq: c_int,
    qpa_aq_prev: c_int,
    qp_novbv: c_float,
    buffer_size: c_double,
    buffer_fill_final: int64_t,
    buffer_fill_final_min: int64_t,
    buffer_fill: c_double,
    buffer_rate: c_double,
    vbv_max_rate: c_double,
    pred: *mut predictor_t,
    single_frame_vbv: c_int,
    rate_factor_max_increment: c_float,
    last_satd: c_int,
    last_rceq: c_double,
    cplxr_sum: c_double,
    expected_bits_sum: c_double,
    filler_bits_sum: int64_t,
    wanted_bits_window: c_double,
    cbr_decay: c_double,
    short_term_cplxsum: c_double,
    short_term_cplxcount: c_double,
    rate_factor_constant: c_double,
    ip_offset: c_double,
    pb_offset: c_double,
    p_stat_file_out: *mut FILE,
    psz_stat_file_tmpname: *mut c_char,
    p_mbtree_stat_file_out: *mut FILE,
    psz_mbtree_stat_file_tmpname: *mut c_char,
    psz_mbtree_stat_file_name: *mut c_char,
    p_mbtree_stat_file_in: *mut FILE,
    num_entries: c_int,
    entry: *mut ratecontrol_entry_t,
    entry_out: *mut *mut ratecontrol_entry_t,
    last_qscale: c_double,
    last_qscale_for: [c_double; 3],
    last_non_b_pict_type: c_int,
    accum_p_qp: c_double,
    accum_p_norm: c_double,
    last_accum_p_norm: c_double,
    lmin: [c_double; 3],
    lmax: [c_double; 3],
    lstep: c_double,
    mbtree: C2RustUnnamed_7,
    frame_size_estimated: c_float,
    bits_so_far: c_float,
    frame_size_maximum: c_double,
    frame_size_planned: c_double,
    slice_size_planned: c_double,
    row_pred: *mut predictor_t,
    row_preds: [[predictor_t; 2]; 3],
    pred_b_from_p: *mut predictor_t,
    bframes: c_int,
    bframe_bits: c_int,
    i_zones: c_int,
    zones: *mut x264_zone_t,
    prev_zone: *mut x264_zone_t,
    initial_cpb_removal_delay: c_int,
    initial_cpb_removal_delay_offset: c_int,
    nrt_first_access_unit: c_double,
    previous_cpb_final_arrival_time: c_double,
    hrd_multiply_denom: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "63:9"]
struct predictor_t {
    coeff_min: c_float,
    coeff: c_float,
    count: c_float,
    decay: c_float,
    offset: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "140:5"]
struct C2RustUnnamed_7 {
    qp_buffer: [*mut uint16_t; 2],
    qpbuf_pos: c_int,
    src_mb_count: c_int,
    rescale_enabled: c_int,
    scale_buffer: [*mut c_float; 2],
    filtersize: [c_int; 2],
    coeffs: [*mut c_float; 2],
    pos: [*mut c_int; 2],
    srcdim: [c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "36:9"]
struct ratecontrol_entry_t {
    pict_type: c_int,
    frame_type: c_int,
    kept_as_ref: c_int,
    qscale: c_double,
    mv_bits: c_int,
    tex_bits: c_int,
    misc_bits: c_int,
    expected_bits: c_double,
    expected_vbv: c_double,
    new_qscale: c_double,
    new_qp: c_float,
    i_count: c_int,
    p_count: c_int,
    s_count: c_int,
    blurred_complexity: c_float,
    direct_mode: c_char,
    weight: [[int16_t; 2]; 3],
    i_weight_denom: [int16_t; 2],
    refcount: [c_int; 16],
    refs: c_int,
    i_duration: int64_t,
    i_cpb_duration: int64_t,
    out_num: c_int,
}
#[inline]
#[c2rust::src_loc = "203:1"]
unsafe extern "C" fn qp2qscale(mut qp: c_float) -> c_float {
    return 0.85f32 * powf(2.0f32, (qp - (12.0f32 + QP_BD_OFFSET as c_float)) / 6.0f32);
}
#[inline]
#[c2rust::src_loc = "207:1"]
unsafe extern "C" fn qscale2qp(mut qscale: c_float) -> c_float {
    return 12.0f32 + QP_BD_OFFSET as c_float + 6.0f32 * log2f(qscale / 0.85f32);
}
#[inline]
#[c2rust::src_loc = "216:1"]
unsafe extern "C" fn qscale2bits(
    mut rce: *mut ratecontrol_entry_t,
    mut qscale: c_double,
) -> c_double {
    if qscale < 0.1f64 {
        qscale = 0.1f64;
    }
    return ((*rce).tex_bits as c_double + 0.1f64) * pow((*rce).qscale / qscale, 1.1f64)
        + (*rce).mv_bits as c_double
            * pow(
                (if (*rce).qscale > 1 as c_int as c_double {
                    (*rce).qscale
                } else {
                    1 as c_int as c_double
                }) / (if qscale > 1 as c_int as c_double {
                    qscale
                } else {
                    1 as c_int as c_double
                }),
                0.5f64,
            )
        + (*rce).misc_bits as c_double;
}
#[inline(always)]
#[c2rust::src_loc = "225:1"]
unsafe extern "C" fn ac_energy_var(
    mut sum_ssd: uint64_t,
    mut shift: c_int,
    mut frame: *mut x264_frame_t,
    mut i: c_int,
    mut b_store: c_int,
) -> uint32_t {
    let mut sum: uint32_t = sum_ssd as uint32_t;
    let mut ssd: uint32_t = (sum_ssd >> 32 as c_int) as uint32_t;
    if b_store != 0 {
        (*frame).i_pixel_sum[i as usize] = (*frame).i_pixel_sum[i as usize].wrapping_add(sum);
        (*frame).i_pixel_ssd[i as usize] =
            (*frame).i_pixel_ssd[i as usize].wrapping_add(ssd as uint64_t);
    }
    return (ssd as uint64_t).wrapping_sub((sum as uint64_t).wrapping_mul(sum as uint64_t) >> shift)
        as uint32_t;
}
#[inline(always)]
#[c2rust::src_loc = "237:1"]
unsafe extern "C" fn ac_energy_plane(
    mut h: *mut x264_t,
    mut mb_x: c_int,
    mut mb_y: c_int,
    mut frame: *mut x264_frame_t,
    mut i: c_int,
    mut b_chroma: c_int,
    mut b_field: c_int,
    mut b_store: c_int,
) -> uint32_t {
    let mut height: c_int = if b_chroma != 0 {
        16 as c_int >> (*h).mb.chroma_v_shift
    } else {
        16 as c_int
    };
    let mut stride: c_int = (*frame).i_stride[i as usize];
    let mut offset: c_int = if b_field != 0 {
        16 as c_int * mb_x + height * (mb_y & !(1 as c_int)) * stride + (mb_y & 1 as c_int) * stride
    } else {
        16 as c_int * mb_x + height * mb_y * stride
    };
    stride <<= b_field;
    if b_chroma != 0 {
        let mut pix: [pixel; 256] = [0; 256];
        let mut chromapix: c_int = (*h).luma2chroma_pixel[PIXEL_16x16 as c_int as usize] as c_int;
        let mut shift: c_int = 7 as c_int - (*h).mb.chroma_v_shift;
        (*h).mc
            .load_deinterleave_chroma_fenc
            .expect("non-null function pointer")(
            pix.as_mut_ptr(),
            (*frame).plane[1 as c_int as usize].offset(offset as isize),
            stride as intptr_t,
            height,
        );
        return ac_energy_var(
            (*h).pixf.var[chromapix as usize].expect("non-null function pointer")(
                pix.as_mut_ptr(),
                FENC_STRIDE as intptr_t,
            ),
            shift,
            frame,
            1 as c_int,
            b_store,
        )
        .wrapping_add(ac_energy_var(
            (*h).pixf.var[chromapix as usize].expect("non-null function pointer")(
                pix.as_mut_ptr().offset((FENC_STRIDE / 2 as c_int) as isize),
                FENC_STRIDE as intptr_t,
            ),
            shift,
            frame,
            2 as c_int,
            b_store,
        ));
    } else {
        return ac_energy_var(
            (*h).pixf.var[PIXEL_16x16 as c_int as usize].expect("non-null function pointer")(
                (*frame).plane[i as usize].offset(offset as isize),
                stride as intptr_t,
            ),
            8 as c_int,
            frame,
            i,
            b_store,
        );
    };
}
#[inline(never)]
#[c2rust::src_loc = "260:1"]
unsafe extern "C" fn ac_energy_mb(
    mut h: *mut x264_t,
    mut mb_x: c_int,
    mut mb_y: c_int,
    mut frame: *mut x264_frame_t,
) -> uint32_t {
    let mut var: uint32_t = 0;
    x264_10_prefetch_fenc(h, frame, mb_x, mb_y);
    if (*h).mb.b_adaptive_mbaff != 0 {
        let mut var_interlaced: uint32_t = 0;
        let mut var_progressive: uint32_t = 0;
        var_interlaced = ac_energy_plane(
            h, mb_x, mb_y, frame, 0 as c_int, 0 as c_int, 1 as c_int, 1 as c_int,
        );
        var_progressive = ac_energy_plane(
            h, mb_x, mb_y, frame, 0 as c_int, 0 as c_int, 0 as c_int, 0 as c_int,
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            var_interlaced = var_interlaced.wrapping_add(ac_energy_plane(
                h, mb_x, mb_y, frame, 1 as c_int, 0 as c_int, 1 as c_int, 1 as c_int,
            ));
            var_progressive = var_progressive.wrapping_add(ac_energy_plane(
                h, mb_x, mb_y, frame, 1 as c_int, 0 as c_int, 0 as c_int, 0 as c_int,
            ));
            var_interlaced = var_interlaced.wrapping_add(ac_energy_plane(
                h, mb_x, mb_y, frame, 2 as c_int, 0 as c_int, 1 as c_int, 1 as c_int,
            ));
            var_progressive = var_progressive.wrapping_add(ac_energy_plane(
                h, mb_x, mb_y, frame, 2 as c_int, 0 as c_int, 0 as c_int, 0 as c_int,
            ));
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            var_interlaced = var_interlaced.wrapping_add(ac_energy_plane(
                h, mb_x, mb_y, frame, 1 as c_int, 1 as c_int, 1 as c_int, 1 as c_int,
            ));
            var_progressive = var_progressive.wrapping_add(ac_energy_plane(
                h, mb_x, mb_y, frame, 1 as c_int, 1 as c_int, 0 as c_int, 0 as c_int,
            ));
        }
        var = if var_interlaced < var_progressive {
            var_interlaced
        } else {
            var_progressive
        };
    } else {
        var = ac_energy_plane(
            h,
            mb_x,
            mb_y,
            frame,
            0 as c_int,
            0 as c_int,
            (*h).param.b_interlaced,
            1 as c_int,
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            var = var.wrapping_add(ac_energy_plane(
                h,
                mb_x,
                mb_y,
                frame,
                1 as c_int,
                0 as c_int,
                (*h).param.b_interlaced,
                1 as c_int,
            ));
            var = var.wrapping_add(ac_energy_plane(
                h,
                mb_x,
                mb_y,
                frame,
                2 as c_int,
                0 as c_int,
                (*h).param.b_interlaced,
                1 as c_int,
            ));
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            var = var.wrapping_add(ac_energy_plane(
                h,
                mb_x,
                mb_y,
                frame,
                1 as c_int,
                1 as c_int,
                (*h).param.b_interlaced,
                1 as c_int,
            ));
        }
    }
    return var;
}
#[no_mangle]
#[c2rust::src_loc = "304:1"]
unsafe extern "C" fn x264_10_adaptive_quant_frame(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut quant_offsets: *mut c_float,
) {
    let mut i: c_int = 0 as c_int;
    while i < 3 as c_int {
        (*frame).i_pixel_sum[i as usize] = 0 as uint32_t;
        (*frame).i_pixel_ssd[i as usize] = 0 as uint64_t;
        i += 1;
    }
    if (*h).param.rc.i_aq_mode == X264_AQ_NONE
        || (*h).param.rc.f_aq_strength == 0 as c_int as c_float
    {
        if (*h).param.rc.i_aq_mode != 0 && (*h).param.rc.f_aq_strength == 0 as c_int as c_float {
            if !quant_offsets.is_null() {
                let mut mb_xy: c_int = 0 as c_int;
                while mb_xy < (*h).mb.i_mb_count {
                    let ref mut fresh6 = *(*frame).f_qp_offset_aq.offset(mb_xy as isize);
                    *fresh6 = *quant_offsets.offset(mb_xy as isize);
                    *(*frame).f_qp_offset.offset(mb_xy as isize) = *fresh6;
                    mb_xy += 1;
                }
                if (*h).frames.b_have_lowres != 0 {
                    let mut mb_xy_0: c_int = 0 as c_int;
                    while mb_xy_0 < (*h).mb.i_mb_count {
                        *(*frame).i_inv_qscale_factor.offset(mb_xy_0 as isize) =
                            x264_exp2fix8(*(*frame).f_qp_offset.offset(mb_xy_0 as isize))
                                as uint16_t;
                        mb_xy_0 += 1;
                    }
                }
            } else {
                memset(
                    (*frame).f_qp_offset as *mut c_void,
                    0 as c_int,
                    ((*h).mb.i_mb_count as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_float>() as size_t),
                );
                memset(
                    (*frame).f_qp_offset_aq as *mut c_void,
                    0 as c_int,
                    ((*h).mb.i_mb_count as size_t)
                        .wrapping_mul(::core::mem::size_of::<c_float>() as size_t),
                );
                if (*h).frames.b_have_lowres != 0 {
                    let mut mb_xy_1: c_int = 0 as c_int;
                    while mb_xy_1 < (*h).mb.i_mb_count {
                        *(*frame).i_inv_qscale_factor.offset(mb_xy_1 as isize) = 256 as uint16_t;
                        mb_xy_1 += 1;
                    }
                }
            }
        }
        if (*h).param.analyse.i_weighted_pred != 0 {
            let mut mb_y: c_int = 0 as c_int;
            while mb_y < (*h).mb.i_mb_height {
                let mut mb_x: c_int = 0 as c_int;
                while mb_x < (*h).mb.i_mb_width {
                    ac_energy_mb(h, mb_x, mb_y, frame);
                    mb_x += 1;
                }
                mb_y += 1;
            }
        } else {
            return;
        }
    } else {
        let mut strength: c_float = 0.;
        let mut avg_adj: c_float = 0.0f32;
        let mut bias_strength: c_float = 0.0f32;
        if (*h).param.rc.i_aq_mode == X264_AQ_AUTOVARIANCE
            || (*h).param.rc.i_aq_mode == X264_AQ_AUTOVARIANCE_BIASED
        {
            let mut bit_depth_correction: c_float =
                1.0f32 / ((1 as c_int) << 2 as c_int * (BIT_DEPTH - 8 as c_int)) as c_float;
            let mut avg_adj_pow2: c_float = 0.0f32;
            let mut mb_y_0: c_int = 0 as c_int;
            while mb_y_0 < (*h).mb.i_mb_height {
                let mut mb_x_0: c_int = 0 as c_int;
                while mb_x_0 < (*h).mb.i_mb_width {
                    let mut energy: uint32_t = ac_energy_mb(h, mb_x_0, mb_y_0, frame);
                    let mut qp_adj: c_float = powf(
                        energy as c_float * bit_depth_correction + 1 as c_int as c_float,
                        0.125f32,
                    );
                    *(*frame)
                        .f_qp_offset
                        .offset((mb_x_0 + mb_y_0 * (*h).mb.i_mb_stride) as isize) = qp_adj;
                    avg_adj += qp_adj;
                    avg_adj_pow2 += qp_adj * qp_adj;
                    mb_x_0 += 1;
                }
                mb_y_0 += 1;
            }
            avg_adj /= (*h).mb.i_mb_count as c_float;
            avg_adj_pow2 /= (*h).mb.i_mb_count as c_float;
            strength = (*h).param.rc.f_aq_strength * avg_adj;
            avg_adj = avg_adj - 0.5f32 * (avg_adj_pow2 - 14.0f32) / avg_adj;
            bias_strength = (*h).param.rc.f_aq_strength;
        } else {
            strength = (*h).param.rc.f_aq_strength * 1.0397f32;
        }
        let mut mb_y_1: c_int = 0 as c_int;
        while mb_y_1 < (*h).mb.i_mb_height {
            let mut mb_x_1: c_int = 0 as c_int;
            while mb_x_1 < (*h).mb.i_mb_width {
                let mut qp_adj_0: c_float = 0.;
                let mut mb_xy_2: c_int = mb_x_1 + mb_y_1 * (*h).mb.i_mb_stride;
                if (*h).param.rc.i_aq_mode == X264_AQ_AUTOVARIANCE_BIASED {
                    qp_adj_0 = *(*frame).f_qp_offset.offset(mb_xy_2 as isize);
                    qp_adj_0 = strength * (qp_adj_0 - avg_adj)
                        + bias_strength * (1.0f32 - 14.0f32 / (qp_adj_0 * qp_adj_0));
                } else if (*h).param.rc.i_aq_mode == X264_AQ_AUTOVARIANCE {
                    qp_adj_0 = *(*frame).f_qp_offset.offset(mb_xy_2 as isize);
                    qp_adj_0 = strength * (qp_adj_0 - avg_adj);
                } else {
                    let mut energy_0: uint32_t = ac_energy_mb(h, mb_x_1, mb_y_1, frame);
                    qp_adj_0 = strength
                        * (x264_log2(if energy_0 > 1 as uint32_t {
                            energy_0
                        } else {
                            1 as uint32_t
                        }) - (14.427f32 + (2 as c_int * (BIT_DEPTH - 8 as c_int)) as c_float));
                }
                if !quant_offsets.is_null() {
                    qp_adj_0 += *quant_offsets.offset(mb_xy_2 as isize);
                }
                let ref mut fresh7 = *(*frame).f_qp_offset_aq.offset(mb_xy_2 as isize);
                *fresh7 = qp_adj_0;
                *(*frame).f_qp_offset.offset(mb_xy_2 as isize) = *fresh7;
                if (*h).frames.b_have_lowres != 0 {
                    *(*frame).i_inv_qscale_factor.offset(mb_xy_2 as isize) =
                        x264_exp2fix8(qp_adj_0) as uint16_t;
                }
                mb_x_1 += 1;
            }
            mb_y_1 += 1;
        }
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 3 as c_int {
        let mut ssd: uint64_t = (*frame).i_pixel_ssd[i_0 as usize];
        let mut sum: uint64_t = (*frame).i_pixel_sum[i_0 as usize] as uint64_t;
        let mut width: c_int =
            16 as c_int * (*h).mb.i_mb_width >> (i_0 != 0 && (*h).mb.chroma_h_shift != 0) as c_int;
        let mut height: c_int =
            16 as c_int * (*h).mb.i_mb_height >> (i_0 != 0 && (*h).mb.chroma_v_shift != 0) as c_int;
        (*frame).i_pixel_ssd[i_0 as usize] = ssd.wrapping_sub(
            sum.wrapping_mul(sum)
                .wrapping_add((width * height / 2 as c_int) as uint64_t)
                .wrapping_div((width * height) as uint64_t),
        );
        i_0 += 1;
    }
}
#[c2rust::src_loc = "417:1"]
unsafe extern "C" fn macroblock_tree_rescale_init(
    mut h: *mut x264_t,
    mut rc: *mut x264_ratecontrol_t,
) -> c_int {
    let mut current_block: u64;
    let mut srcdim: [c_float; 2] = [
        (*rc).mbtree.srcdim[0 as c_int as usize] as c_float / 16.0f32,
        (*rc).mbtree.srcdim[1 as c_int as usize] as c_float / 16.0f32,
    ];
    let mut dstdim: [c_float; 2] = [
        (*h).param.i_width as c_float / 16.0f32,
        (*h).param.i_height as c_float / 16.0f32,
    ];
    let mut srcdimi: [c_int; 2] = [
        ceil(srcdim[0 as c_int as usize] as c_double) as c_int,
        ceil(srcdim[1 as c_int as usize] as c_double) as c_int,
    ];
    let mut dstdimi: [c_int; 2] = [
        ceil(dstdim[0 as c_int as usize] as c_double) as c_int,
        ceil(dstdim[1 as c_int as usize] as c_double) as c_int,
    ];
    if (*h).param.b_interlaced != 0 || (*h).param.b_fake_interlaced != 0 {
        srcdimi[1 as c_int as usize] = srcdimi[1 as c_int as usize] + 1 as c_int & !(1 as c_int);
        dstdimi[1 as c_int as usize] = dstdimi[1 as c_int as usize] + 1 as c_int & !(1 as c_int);
    }
    (*rc).mbtree.src_mb_count = srcdimi[0 as c_int as usize] * srcdimi[1 as c_int as usize];
    (*rc).mbtree.qp_buffer[0 as c_int as usize] = x264_malloc(
        ((*rc).mbtree.src_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as int64_t,
    ) as *mut uint16_t;
    if !(*rc).mbtree.qp_buffer[0 as c_int as usize].is_null() {
        if (*h).param.i_bframe_pyramid != 0 && (*h).param.rc.b_stat_read != 0 {
            (*rc).mbtree.qp_buffer[1 as c_int as usize] = x264_malloc(
                ((*rc).mbtree.src_mb_count as usize)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                    as int64_t,
            ) as *mut uint16_t;
            if (*rc).mbtree.qp_buffer[1 as c_int as usize].is_null() {
                current_block = 9112284078615121668;
            } else {
                current_block = 5399440093318478209;
            }
        } else {
            current_block = 5399440093318478209;
        }
        match current_block {
            9112284078615121668 => {}
            _ => {
                (*rc).mbtree.qpbuf_pos = -(1 as c_int);
                if srcdimi[0 as c_int as usize] == dstdimi[0 as c_int as usize]
                    && srcdimi[1 as c_int as usize] == dstdimi[1 as c_int as usize]
                {
                    return 0 as c_int;
                }
                (*rc).mbtree.rescale_enabled = 1 as c_int;
                (*rc).mbtree.scale_buffer[0 as c_int as usize] = x264_malloc(
                    ((srcdimi[0 as c_int as usize] * srcdimi[1 as c_int as usize]) as usize)
                        .wrapping_mul(::core::mem::size_of::<c_float>() as usize)
                        as int64_t,
                ) as *mut c_float;
                if !(*rc).mbtree.scale_buffer[0 as c_int as usize].is_null() {
                    (*rc).mbtree.scale_buffer[1 as c_int as usize] = x264_malloc(
                        ((dstdimi[0 as c_int as usize] * srcdimi[1 as c_int as usize]) as usize)
                            .wrapping_mul(::core::mem::size_of::<c_float>() as usize)
                            as int64_t,
                    )
                        as *mut c_float;
                    if !(*rc).mbtree.scale_buffer[1 as c_int as usize].is_null() {
                        let mut i: c_int = 0 as c_int;
                        loop {
                            if !(i < 2 as c_int) {
                                current_block = 3160140712158701372;
                                break;
                            }
                            if srcdim[i as usize] > dstdim[i as usize] {
                                (*rc).mbtree.filtersize[i as usize] = 1 as c_int
                                    + (2 as c_int * srcdimi[i as usize] + dstdimi[i as usize]
                                        - 1 as c_int)
                                        / dstdimi[i as usize];
                            } else {
                                (*rc).mbtree.filtersize[i as usize] = 3 as c_int;
                            }
                            (*rc).mbtree.coeffs[i as usize] = x264_malloc(
                                (((*rc).mbtree.filtersize[i as usize] * dstdimi[i as usize])
                                    as usize)
                                    .wrapping_mul(::core::mem::size_of::<c_float>() as usize)
                                    as int64_t,
                            )
                                as *mut c_float;
                            if (*rc).mbtree.coeffs[i as usize].is_null() {
                                current_block = 9112284078615121668;
                                break;
                            }
                            (*rc).mbtree.pos[i as usize] =
                                x264_malloc(
                                    (dstdimi[i as usize] as usize)
                                        .wrapping_mul(::core::mem::size_of::<c_int>() as usize)
                                        as int64_t,
                                ) as *mut c_int;
                            if (*rc).mbtree.pos[i as usize].is_null() {
                                current_block = 9112284078615121668;
                                break;
                            }
                            let mut inc: c_float = srcdim[i as usize] / dstdim[i as usize];
                            let mut dmul: c_float = if inc > 1.0f32 {
                                dstdim[i as usize] / srcdim[i as usize]
                            } else {
                                1.0f32
                            };
                            let mut dstinsrc: c_float = 0.5f32 * inc - 0.5f32;
                            let mut filtersize: c_int = (*rc).mbtree.filtersize[i as usize];
                            let mut j: c_int = 0 as c_int;
                            while j < dstdimi[i as usize] {
                                let mut pos: c_int =
                                    (dstinsrc - (filtersize as c_float - 2.0f32) * 0.5f32) as c_int;
                                let mut sum: c_float = 0.0f32;
                                *(*rc).mbtree.pos[i as usize].offset(j as isize) = pos;
                                let mut k: c_int = 0 as c_int;
                                while k < filtersize {
                                    let mut d: c_float =
                                        (fabs(((pos + k) as c_float - dstinsrc) as c_double)
                                            * dmul as c_double)
                                            as c_float;
                                    let mut coeff: c_float = if 1.0f32 - d > 0 as c_int as c_float {
                                        1.0f32 - d
                                    } else {
                                        0 as c_int as c_float
                                    };
                                    *(*rc).mbtree.coeffs[i as usize]
                                        .offset((j * filtersize + k) as isize) = coeff;
                                    sum += coeff;
                                    k += 1;
                                }
                                sum = 1.0f32 / sum;
                                let mut k_0: c_int = 0 as c_int;
                                while k_0 < filtersize {
                                    *(*rc).mbtree.coeffs[i as usize]
                                        .offset((j * filtersize + k_0) as isize) *= sum;
                                    k_0 += 1;
                                }
                                dstinsrc += inc;
                                j += 1;
                            }
                            i += 1;
                        }
                        match current_block {
                            9112284078615121668 => {}
                            _ => {
                                (*rc).mbtree.srcdim[0 as c_int as usize] =
                                    srcdimi[0 as c_int as usize];
                                (*rc).mbtree.srcdim[1 as c_int as usize] =
                                    srcdimi[1 as c_int as usize];
                                return 0 as c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    return -(1 as c_int);
}
#[c2rust::src_loc = "490:1"]
unsafe extern "C" fn macroblock_tree_rescale_destroy(mut rc: *mut x264_ratecontrol_t) {
    let mut i: c_int = 0 as c_int;
    while i < 2 as c_int {
        x264_free((*rc).mbtree.qp_buffer[i as usize] as *mut c_void);
        x264_free((*rc).mbtree.scale_buffer[i as usize] as *mut c_void);
        x264_free((*rc).mbtree.coeffs[i as usize] as *mut c_void);
        x264_free((*rc).mbtree.pos[i as usize] as *mut c_void);
        i += 1;
    }
}
#[inline(always)]
#[c2rust::src_loc = "501:1"]
unsafe extern "C" fn tapfilter(
    mut src: *mut c_float,
    mut pos: c_int,
    mut max: c_int,
    mut stride: c_int,
    mut coeff: *mut c_float,
    mut filtersize: c_int,
) -> c_float {
    let mut sum: c_float = 0.0f32;
    let mut i: c_int = 0 as c_int;
    while i < filtersize {
        sum += *src.offset((x264_clip3(pos, 0 as c_int, max - 1 as c_int) * stride) as isize)
            * *coeff.offset(i as isize);
        i += 1;
        pos += 1;
    }
    return sum;
}
#[c2rust::src_loc = "509:1"]
unsafe extern "C" fn macroblock_tree_rescale(
    mut h: *mut x264_t,
    mut rc: *mut x264_ratecontrol_t,
    mut dst: *mut c_float,
) {
    let mut input: *mut c_float = 0 as *mut c_float;
    let mut output: *mut c_float = 0 as *mut c_float;
    let mut filtersize: c_int = 0;
    let mut stride: c_int = 0;
    let mut height: c_int = 0;
    input = (*rc).mbtree.scale_buffer[0 as c_int as usize];
    output = (*rc).mbtree.scale_buffer[1 as c_int as usize];
    filtersize = (*rc).mbtree.filtersize[0 as c_int as usize];
    stride = (*rc).mbtree.srcdim[0 as c_int as usize];
    height = (*rc).mbtree.srcdim[1 as c_int as usize];
    let mut y: c_int = 0 as c_int;
    while y < height {
        let mut coeff: *mut c_float = (*rc).mbtree.coeffs[0 as c_int as usize];
        let mut x: c_int = 0 as c_int;
        while x < (*h).mb.i_mb_width {
            *output.offset(x as isize) = tapfilter(
                input,
                *(*rc).mbtree.pos[0 as c_int as usize].offset(x as isize),
                stride,
                1 as c_int,
                coeff,
                filtersize,
            );
            x += 1;
            coeff = coeff.offset(filtersize as isize);
        }
        y += 1;
        input = input.offset(stride as isize);
        output = output.offset((*h).mb.i_mb_width as isize);
    }
    input = (*rc).mbtree.scale_buffer[1 as c_int as usize];
    output = dst;
    filtersize = (*rc).mbtree.filtersize[1 as c_int as usize];
    stride = (*h).mb.i_mb_width;
    height = (*rc).mbtree.srcdim[1 as c_int as usize];
    let mut x_0: c_int = 0 as c_int;
    while x_0 < (*h).mb.i_mb_width {
        let mut coeff_0: *mut c_float = (*rc).mbtree.coeffs[1 as c_int as usize];
        let mut y_0: c_int = 0 as c_int;
        while y_0 < (*h).mb.i_mb_height {
            *output.offset((y_0 * stride) as isize) = tapfilter(
                input,
                *(*rc).mbtree.pos[1 as c_int as usize].offset(y_0 as isize),
                height,
                stride,
                coeff_0,
                filtersize,
            );
            y_0 += 1;
            coeff_0 = coeff_0.offset(filtersize as isize);
        }
        x_0 += 1;
        input = input.offset(1);
        output = output.offset(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "541:1"]
unsafe extern "C" fn x264_10_macroblock_tree_read(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut quant_offsets: *mut c_float,
) -> c_int {
    let mut current_block: u64;
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut i_type_actual: uint8_t =
        (*(*rc).entry.offset((*frame).i_frame as isize)).pict_type as uint8_t;
    if (*(*rc).entry.offset((*frame).i_frame as isize)).kept_as_ref != 0 {
        let mut i_type: uint8_t = 0;
        if (*rc).mbtree.qpbuf_pos < 0 as c_int {
            current_block = 12675440807659640239;
        } else {
            current_block = 11812396948646013369;
        }
        loop {
            match current_block {
                12675440807659640239 => {
                    (*rc).mbtree.qpbuf_pos += 1;
                    if fread(
                        &mut i_type as *mut uint8_t as *mut c_void,
                        1 as size_t,
                        1 as size_t,
                        (*rc).p_mbtree_stat_file_in,
                    ) == 0
                    {
                        current_block = 5482255857212657476;
                        break;
                    }
                    if fread(
                        (*rc).mbtree.qp_buffer[(*rc).mbtree.qpbuf_pos as usize] as *mut c_void,
                        ::core::mem::size_of::<uint16_t>() as size_t,
                        (*rc).mbtree.src_mb_count as size_t,
                        (*rc).p_mbtree_stat_file_in,
                    ) != (*rc).mbtree.src_mb_count as c_uint as c_ulong
                    {
                        current_block = 5482255857212657476;
                        break;
                    }
                    if i_type as c_int != i_type_actual as c_int
                        && (*rc).mbtree.qpbuf_pos == 1 as c_int
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"MB-tree frametype %d doesn't match actual frametype %d.\n\0"
                                as *const u8 as *const c_char,
                            i_type as c_int,
                            i_type_actual as c_int,
                        );
                        return -(1 as c_int);
                    }
                    if i_type as c_int != i_type_actual as c_int {
                        current_block = 12675440807659640239;
                    } else {
                        current_block = 11812396948646013369;
                    }
                }
                _ => {
                    let mut dst: *mut c_float = if (*rc).mbtree.rescale_enabled != 0 {
                        (*rc).mbtree.scale_buffer[0 as c_int as usize]
                    } else {
                        (*frame).f_qp_offset
                    };
                    (*h).mc
                        .mbtree_fix8_unpack
                        .expect("non-null function pointer")(
                        dst,
                        (*rc).mbtree.qp_buffer[(*rc).mbtree.qpbuf_pos as usize],
                        (*rc).mbtree.src_mb_count,
                    );
                    if (*rc).mbtree.rescale_enabled != 0 {
                        macroblock_tree_rescale(h, rc, (*frame).f_qp_offset);
                    }
                    if (*h).frames.b_have_lowres != 0 {
                        let mut i: c_int = 0 as c_int;
                        while i < (*h).mb.i_mb_count {
                            *(*frame).i_inv_qscale_factor.offset(i as isize) =
                                x264_exp2fix8(*(*frame).f_qp_offset.offset(i as isize)) as uint16_t;
                            i += 1;
                        }
                    }
                    (*rc).mbtree.qpbuf_pos -= 1;
                    current_block = 5689001924483802034;
                    break;
                }
            }
        }
        match current_block {
            5689001924483802034 => {}
            _ => {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"Incomplete MB-tree stats file.\n\0" as *const u8 as *const c_char,
                );
                return -(1 as c_int);
            }
        }
    } else {
        x264_10_adaptive_quant_frame(h, frame, quant_offsets);
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "585:1"]
unsafe extern "C" fn x264_10_reference_build_list_optimal(mut h: *mut x264_t) -> c_int {
    let mut rce: *mut ratecontrol_entry_t = (*(*h).rc).rce;
    let mut frames: [*mut x264_frame_t; 16] = [0 as *mut x264_frame_t; 16];
    let mut weights: [[x264_weight_t; 3]; 16] = [[x264_weight_t {
        cachea: [0; 8],
        cacheb: [0; 8],
        i_denom: 0,
        i_scale: 0,
        i_offset: 0,
        weightfn: 0 as *mut weight_fn_t,
    }; 3]; 16];
    let mut refcount: [c_int; 16] = [0; 16];
    if (*rce).refs != (*h).i_ref[0 as c_int as usize] {
        return -(1 as c_int);
    }
    memcpy(
        frames.as_mut_ptr() as *mut c_void,
        (*(*h).fref.as_mut_ptr().offset(0 as c_int as isize)).as_mut_ptr() as *const c_void,
        ::core::mem::size_of::<[*mut x264_frame_t; 16]>() as size_t,
    );
    memcpy(
        refcount.as_mut_ptr() as *mut c_void,
        (*rce).refcount.as_mut_ptr() as *const c_void,
        ::core::mem::size_of::<[c_int; 16]>() as size_t,
    );
    memcpy(
        weights.as_mut_ptr() as *mut c_void,
        (*(*h).fenc).weight.as_mut_ptr() as *const c_void,
        ::core::mem::size_of::<[[x264_weight_t; 3]; 16]>() as size_t,
    );
    memset(
        &mut *(*(*(*h).fenc).weight.as_mut_ptr().offset(1 as c_int as isize))
            .as_mut_ptr()
            .offset(0 as c_int as isize) as *mut x264_weight_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[[x264_weight_t; 3]; 15]>() as size_t,
    );
    let mut ref_0: c_int = 1 as c_int;
    while ref_0 < (*h).i_ref[0 as c_int as usize] {
        let mut max: c_int = -(1 as c_int);
        let mut bestref: c_int = 1 as c_int;
        let mut i: c_int = 1 as c_int;
        while i < (*h).i_ref[0 as c_int as usize] {
            if refcount[i as usize] > max {
                max = refcount[i as usize];
                bestref = i;
            }
            i += 1;
        }
        refcount[bestref as usize] = -(1 as c_int);
        (*h).fref[0 as c_int as usize][ref_0 as usize] = frames[bestref as usize];
        memcpy(
            (*(*(*h).fenc).weight.as_mut_ptr().offset(ref_0 as isize)).as_mut_ptr() as *mut c_void,
            (*weights.as_mut_ptr().offset(bestref as isize)).as_mut_ptr() as *const c_void,
            ::core::mem::size_of::<[x264_weight_t; 3]>() as size_t,
        );
        ref_0 += 1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "622:1"]
unsafe extern "C" fn strcat_filename(
    mut input: *mut c_char,
    mut suffix: *mut c_char,
) -> *mut c_char {
    let mut output: *mut c_char = x264_malloc(
        strlen(input)
            .wrapping_add(strlen(suffix))
            .wrapping_add(1 as size_t) as int64_t,
    ) as *mut c_char;
    if output.is_null() {
        return 0 as *mut c_char;
    }
    strcpy(output, input);
    strcat(output, suffix);
    return output;
}
#[no_mangle]
#[c2rust::src_loc = "632:1"]
unsafe extern "C" fn x264_10_ratecontrol_init_reconfigurable(
    mut h: *mut x264_t,
    mut b_init: c_int,
) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    if b_init == 0 && (*rc).b_2pass != 0 {
        return;
    }
    if (*h).param.rc.i_rc_method == X264_RC_CRF {
        let mut base_cplx: c_double = ((*h).mb.i_mb_count
            * (if (*h).param.i_bframe != 0 {
                120 as c_int
            } else {
                80 as c_int
            })) as c_double;
        let mut mbtree_offset: c_double = if (*h).param.rc.b_mb_tree != 0 {
            (1.0f64 - (*h).param.rc.f_qcompress as c_double) * 13.5f64
        } else {
            0 as c_int as c_double
        };
        (*rc).rate_factor_constant = pow(base_cplx, 1 as c_int as c_double - (*rc).qcompress)
            / qp2qscale(
                ((*h).param.rc.f_rf_constant as c_double + mbtree_offset + QP_BD_OFFSET as c_double)
                    as c_float,
            ) as c_double;
    }
    if (*h).param.rc.i_vbv_max_bitrate > 0 as c_int && (*h).param.rc.i_vbv_buffer_size > 0 as c_int
    {
        if (*rc).b_vbv_min_rate != 0 {
            (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
        }
        if (*h).param.rc.i_vbv_buffer_size
            < ((*h).param.rc.i_vbv_max_bitrate as c_double / (*rc).fps) as c_int
        {
            (*h).param.rc.i_vbv_buffer_size =
                ((*h).param.rc.i_vbv_max_bitrate as c_double / (*rc).fps) as c_int;
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV buffer size cannot be smaller than one frame, using %d kbit\n\0" as *const u8
                    as *const c_char,
                (*h).param.rc.i_vbv_buffer_size,
            );
        }
        let mut kilobit_size: c_int = if (*h).param.i_avcintra_class != 0 {
            1024 as c_int
        } else {
            1000 as c_int
        };
        let mut vbv_buffer_size: c_int = (*h).param.rc.i_vbv_buffer_size * kilobit_size;
        let mut vbv_max_bitrate: c_int = (*h).param.rc.i_vbv_max_bitrate * kilobit_size;
        if (*h).param.i_nal_hrd != 0 && b_init != 0 {
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_cnt = 1 as c_int;
            (*(*h).sps.as_mut_ptr()).vui.hrd.b_cbr_hrd =
                ((*h).param.i_nal_hrd == X264_NAL_HRD_CBR) as c_int;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_time_offset_length = 0 as c_int;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_scale = x264_clip3(
                (vbv_max_bitrate as c_uint).trailing_zeros() as i32 - BR_SHIFT,
                0 as c_int,
                15 as c_int,
            );
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_value =
                vbv_max_bitrate >> (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_scale + BR_SHIFT;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled =
                (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_value
                    << (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_scale + BR_SHIFT;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_scale = x264_clip3(
                (vbv_buffer_size as c_uint).trailing_zeros() as i32 - CPB_SHIFT,
                0 as c_int,
                15 as c_int,
            );
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_value =
                vbv_buffer_size >> (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_scale + CPB_SHIFT;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled =
                (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_value
                    << (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_scale + CPB_SHIFT;
            let mut max_cpb_output_delay: c_int = (if ((*h).param.i_keyint_max as c_double
                * 0.5f64
                * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double
                / (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double)
                < 2147483647 as c_int as c_double
            {
                (*h).param.i_keyint_max as c_double
                    * 0.5f64
                    * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double
                    / (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
            } else {
                2147483647 as c_int as c_double
            }) as c_int;
            let mut max_dpb_output_delay: c_int =
                ((*(*h).sps.as_mut_ptr()).vui.i_max_dec_frame_buffering as c_double
                    * MAX_DURATION
                    * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double
                    / (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double)
                    as c_int;
            let mut max_delay: c_int = (90000.0f64
                * (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled as c_double
                / (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled as c_double
                + 0.5f64) as c_int;
            (*(*h).sps.as_mut_ptr())
                .vui
                .hrd
                .i_initial_cpb_removal_delay_length = 2 as c_int
                + x264_clip3(
                    32 as c_int - (max_delay as c_uint).leading_zeros() as i32,
                    4 as c_int,
                    22 as c_int,
                );
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_removal_delay_length = x264_clip3(
                32 as c_int - (max_cpb_output_delay as c_uint).leading_zeros() as i32,
                4 as c_int,
                31 as c_int,
            );
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_dpb_output_delay_length = x264_clip3(
                32 as c_int - (max_dpb_output_delay as c_uint).leading_zeros() as i32,
                4 as c_int,
                31 as c_int,
            );
            vbv_buffer_size = (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled;
            vbv_max_bitrate = (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled;
        } else if (*h).param.i_nal_hrd != 0 && b_init == 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV parameters cannot be changed when NAL HRD is in use\n\0" as *const u8
                    as *const c_char,
            );
            return;
        }
        (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled = vbv_max_bitrate;
        (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled = vbv_buffer_size;
        if (*rc).b_vbv_min_rate != 0 {
            (*rc).bitrate = (*h).param.rc.i_bitrate as c_double * kilobit_size as c_double;
        }
        (*rc).buffer_rate = vbv_max_bitrate as c_double / (*rc).fps;
        (*rc).vbv_max_rate = vbv_max_bitrate as c_double;
        (*rc).buffer_size = vbv_buffer_size as c_double;
        (*rc).single_frame_vbv = ((*rc).buffer_rate * 1.1f64 > (*rc).buffer_size) as c_int;
        if (*rc).b_abr != 0 && (*h).param.rc.i_rc_method == X264_RC_ABR {
            (*rc).cbr_decay = 1.0f64
                - (*rc).buffer_rate / (*rc).buffer_size
                    * 0.5f64
                    * (if 0 as c_int as c_double
                        > 1.5f64 - (*rc).buffer_rate * (*rc).fps / (*rc).bitrate
                    {
                        0 as c_int as c_double
                    } else {
                        1.5f64 - (*rc).buffer_rate * (*rc).fps / (*rc).bitrate
                    });
        }
        if (*h).param.rc.i_rc_method == X264_RC_CRF && (*h).param.rc.f_rf_constant_max != 0. {
            (*rc).rate_factor_max_increment =
                (*h).param.rc.f_rf_constant_max - (*h).param.rc.f_rf_constant;
            if (*rc).rate_factor_max_increment <= 0 as c_int as c_float {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"CRF max must be greater than CRF\n\0" as *const u8 as *const c_char,
                );
                (*rc).rate_factor_max_increment = 0 as c_int as c_float;
            }
        }
        if b_init != 0 {
            if (*h).param.rc.f_vbv_buffer_init as c_double > 1.0f64 {
                (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
                    ((*h).param.rc.f_vbv_buffer_init / (*h).param.rc.i_vbv_buffer_size as c_float)
                        as c_double,
                    0 as c_int as c_double,
                    1 as c_int as c_double,
                ) as c_float;
            }
            (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
                if (*h).param.rc.f_vbv_buffer_init as c_double
                    > (*rc).buffer_rate / (*rc).buffer_size
                {
                    (*h).param.rc.f_vbv_buffer_init as c_double
                } else {
                    (*rc).buffer_rate / (*rc).buffer_size
                },
                0 as c_int as c_double,
                1 as c_int as c_double,
            ) as c_float;
            (*rc).buffer_fill_final_min = ((*rc).buffer_size
                * (*h).param.rc.f_vbv_buffer_init as c_double
                * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double)
                as int64_t;
            (*rc).buffer_fill_final = (*rc).buffer_fill_final_min;
            (*rc).b_vbv = 1 as c_int;
            (*rc).b_vbv_min_rate = ((*rc).b_2pass == 0
                && (*h).param.rc.i_rc_method == X264_RC_ABR
                && (*h).param.rc.i_vbv_max_bitrate <= (*h).param.rc.i_bitrate)
                as c_int;
        }
    }
}
#[c2rust::src_loc = "673:21"]
const BR_SHIFT: c_int = 6 as c_int;
#[c2rust::src_loc = "674:21"]
const CPB_SHIFT: c_int = 4 as c_int;
#[c2rust::src_loc = "688:21"]
const MAX_DURATION: c_double = 0.5f64;
#[no_mangle]
#[c2rust::src_loc = "744:1"]
unsafe extern "C" fn x264_10_ratecontrol_new(mut h: *mut x264_t) -> c_int {
    let mut w: *mut c_char = 0 as *mut c_char;
    let mut num_preds: c_int = 0;
    static mut pred_coeff_table: [c_float; 3] =
        [1.0f64 as c_float, 1.0f64 as c_float, 1.5f64 as c_float];
    let mut current_block: u64;
    let mut rc: *mut x264_ratecontrol_t = 0 as *mut x264_ratecontrol_t;
    (*h).rc = x264_malloc(
        ((*h).param.i_threads as usize)
            .wrapping_mul(::core::mem::size_of::<x264_ratecontrol_t>() as usize) as int64_t,
    ) as *mut x264_ratecontrol_t;
    if !(*h).rc.is_null() {
        memset(
            (*h).rc as *mut c_void,
            0 as c_int,
            ((*h).param.i_threads as size_t)
                .wrapping_mul(::core::mem::size_of::<x264_ratecontrol_t>() as size_t),
        );
        rc = (*h).rc;
        (*rc).b_abr =
            ((*h).param.rc.i_rc_method != X264_RC_CQP && (*h).param.rc.b_stat_read == 0) as c_int;
        (*rc).b_2pass =
            ((*h).param.rc.i_rc_method == X264_RC_ABR && (*h).param.rc.b_stat_read != 0) as c_int;
        if (*h).param.i_fps_num > 0 as uint32_t && (*h).param.i_fps_den > 0 as uint32_t {
            (*rc).fps =
                ((*h).param.i_fps_num as c_float / (*h).param.i_fps_den as c_float) as c_double;
        } else {
            (*rc).fps = 25.0f64;
        }
        if (*h).param.rc.b_mb_tree != 0 {
            (*h).param.rc.f_pb_factor = 1 as c_int as c_float;
            (*rc).qcompress = 1 as c_int as c_double;
        } else {
            (*rc).qcompress = (*h).param.rc.f_qcompress as c_double;
        }
        (*rc).bitrate = (*h).param.rc.i_bitrate as c_double
            * (if (*h).param.i_avcintra_class != 0 {
                1024.0f64
            } else {
                1000.0f64
            });
        (*rc).rate_tolerance = (*h).param.rc.f_rate_tolerance as c_double;
        (*rc).nmb = (*h).mb.i_mb_count;
        (*rc).last_non_b_pict_type = -(1 as c_int);
        (*rc).cbr_decay = 1.0f64;
        if (*h).param.rc.i_rc_method != X264_RC_ABR && (*h).param.rc.b_stat_read != 0 {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"CRF/CQP is incompatible with 2pass.\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        x264_10_ratecontrol_init_reconfigurable(h, 1 as c_int);
        if (*h).param.i_nal_hrd != 0 {
            let mut denom: uint64_t = ((*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled
                as uint64_t)
                .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t);
            let mut num: uint64_t = 90000 as uint64_t;
            x264_reduce_fraction64(&mut num, &mut denom);
            (*rc).hrd_multiply_denom = (90000 as uint64_t).wrapping_div(num);
            let mut bits_required: c_double = log2(num as c_double)
                + log2((*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double)
                + log2((*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled as c_double);
            if bits_required >= 63 as c_int as c_double {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"HRD with very large timescale and bufsize not supported\n\0" as *const u8
                        as *const c_char,
                );
                return -(1 as c_int);
            }
        }
        if (*rc).rate_tolerance < 0.01f64 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"bitrate tolerance too small, using .01\n\0" as *const u8 as *const c_char,
            );
            (*rc).rate_tolerance = 0.01f64;
        }
        (*h).mb.b_variable_qp = ((*rc).b_vbv != 0 || (*h).param.rc.i_aq_mode != 0) as c_int;
        if (*rc).b_abr != 0 {
            (*rc).accum_p_norm = 0.01f64;
            (*rc).accum_p_qp = ((if (*h).param.rc.i_rc_method == X264_RC_CRF {
                (*h).param.rc.f_rf_constant
            } else {
                24 as c_int as c_float
            }) + QP_BD_OFFSET as c_float) as c_double
                * (*rc).accum_p_norm;
            (*rc).cplxr_sum = 0.01f64
                * pow(7.0e5f64, (*rc).qcompress)
                * pow((*h).mb.i_mb_count as c_double, 0.5f64);
            (*rc).wanted_bits_window = 1.0f64 * (*rc).bitrate / (*rc).fps;
            (*rc).last_non_b_pict_type = SLICE_TYPE_I as c_int;
        }
        (*rc).ip_offset = 6.0f64 * log2f((*h).param.rc.f_ip_factor) as c_double;
        (*rc).pb_offset = 6.0f64 * log2f((*h).param.rc.f_pb_factor) as c_double;
        (*rc).qp_constant[SLICE_TYPE_P as c_int as usize] = (*h).param.rc.i_qp_constant;
        (*rc).qp_constant[SLICE_TYPE_I as c_int as usize] = x264_clip3(
            ((*h).param.rc.i_qp_constant as c_double - (*rc).ip_offset + 0.5f64) as c_int,
            0 as c_int,
            QP_MAX,
        );
        (*rc).qp_constant[SLICE_TYPE_B as c_int as usize] = x264_clip3(
            ((*h).param.rc.i_qp_constant as c_double + (*rc).pb_offset + 0.5f64) as c_int,
            0 as c_int,
            QP_MAX,
        );
        (*h).mb.ip_offset = ((*rc).ip_offset + 0.5f64) as c_int;
        (*rc).lstep = pow(
            2 as c_int as c_double,
            (*h).param.rc.i_qp_step as c_double / 6.0f64,
        );
        (*rc).last_qscale = qp2qscale((26 as c_int + QP_BD_OFFSET) as c_float) as c_double;
        num_preds = (*h).param.b_sliced_threads * (*h).param.i_threads + 1 as c_int;
        (*rc).pred = x264_malloc(
            (5 as usize)
                .wrapping_mul(::core::mem::size_of::<predictor_t>() as usize)
                .wrapping_mul(num_preds as usize) as int64_t,
        ) as *mut predictor_t;
        if !(*rc).pred.is_null() {
            (*rc).pred_b_from_p =
                x264_malloc(::core::mem::size_of::<predictor_t>() as int64_t) as *mut predictor_t;
            if !(*rc).pred_b_from_p.is_null() {
                let mut i: c_int = 0 as c_int;
                while i < 3 as c_int {
                    (*rc).last_qscale_for[i as usize] = qp2qscale(
                        (if (*h).param.rc.i_rc_method == X264_RC_CRF {
                            (*h).param.rc.f_rf_constant
                        } else {
                            24 as c_int as c_float
                        }) + QP_BD_OFFSET as c_float,
                    ) as c_double;
                    (*rc).lmin[i as usize] =
                        qp2qscale((*h).param.rc.i_qp_min as c_float) as c_double;
                    (*rc).lmax[i as usize] =
                        qp2qscale((*h).param.rc.i_qp_max as c_float) as c_double;
                    let mut j: c_int = 0 as c_int;
                    while j < num_preds {
                        (*(*rc).pred.offset((i + j * 5 as c_int) as isize)).coeff_min =
                            pred_coeff_table[i as usize] / 2 as c_int as c_float;
                        (*(*rc).pred.offset((i + j * 5 as c_int) as isize)).coeff =
                            pred_coeff_table[i as usize];
                        (*(*rc).pred.offset((i + j * 5 as c_int) as isize)).count = 1.0f32;
                        (*(*rc).pred.offset((i + j * 5 as c_int) as isize)).decay = 0.5f32;
                        (*(*rc).pred.offset((i + j * 5 as c_int) as isize)).offset = 0.0f32;
                        j += 1;
                    }
                    let mut j_0: c_int = 0 as c_int;
                    while j_0 < 2 as c_int {
                        (*rc).row_preds[i as usize][j_0 as usize].coeff_min =
                            (0.25f64 / 4 as c_int as c_double) as c_float;
                        (*rc).row_preds[i as usize][j_0 as usize].coeff = 0.25f32;
                        (*rc).row_preds[i as usize][j_0 as usize].count = 1.0f32;
                        (*rc).row_preds[i as usize][j_0 as usize].decay = 0.5f32;
                        (*rc).row_preds[i as usize][j_0 as usize].offset = 0.0f32;
                        j_0 += 1;
                    }
                    i += 1;
                }
                (*(*rc).pred_b_from_p).coeff_min = (0.5f64 / 2 as c_int as c_double) as c_float;
                (*(*rc).pred_b_from_p).coeff = 0.5f32;
                (*(*rc).pred_b_from_p).count = 1.0f32;
                (*(*rc).pred_b_from_p).decay = 0.5f32;
                (*(*rc).pred_b_from_p).offset = 0.0f32;
                if parse_zones(h) < 0 as c_int {
                    x264_10_log(
                        h,
                        X264_LOG_ERROR,
                        b"failed to parse zones\n\0" as *const u8 as *const c_char,
                    );
                    return -(1 as c_int);
                }
                if (*h).param.rc.b_stat_read != 0 {
                    let mut p: *mut c_char = 0 as *mut c_char;
                    let mut stats_in: *mut c_char = 0 as *mut c_char;
                    let mut stats_buf: *mut c_char = 0 as *mut c_char;
                    if !(*h).param.rc.psz_stat_in.is_null() {
                    } else {
                        __assert_fail(
                            b"h->param.rc.psz_stat_in\0" as *const u8 as *const c_char,
                            b"encoder/ratecontrol.c\0" as *const u8 as *const c_char,
                            874 as c_uint,
                            ::core::mem::transmute::<[u8; 38], [c_char; 38]>(
                                *b"int x264_10_ratecontrol_new(x264_t *)\0",
                            )
                            .as_ptr(),
                        );
                    }
                    stats_in = x264_slurp_file((*h).param.rc.psz_stat_in);
                    stats_buf = stats_in;
                    if stats_buf.is_null() {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"ratecontrol_init: can't open stats file\n\0" as *const u8
                                as *const c_char,
                        );
                        return -(1 as c_int);
                    }
                    if (*h).param.rc.b_mb_tree != 0 {
                        let mut mbtree_stats_in: *mut c_char = strcat_filename(
                            (*h).param.rc.psz_stat_in,
                            b".mbtree\0" as *const u8 as *const c_char as *mut c_char,
                        );
                        if mbtree_stats_in.is_null() {
                            return -(1 as c_int);
                        }
                        (*rc).p_mbtree_stat_file_in =
                            fopen(mbtree_stats_in, b"rb\0" as *const u8 as *const c_char)
                                as *mut FILE;
                        x264_free(mbtree_stats_in as *mut c_void);
                        if (*rc).p_mbtree_stat_file_in.is_null() {
                            x264_10_log(
                                h,
                                X264_LOG_ERROR,
                                b"ratecontrol_init: can't open mbtree stats file\n\0" as *const u8
                                    as *const c_char,
                            );
                            return -(1 as c_int);
                        }
                    }
                    if strncmp(
                        stats_buf,
                        b"#options:\0" as *const u8 as *const c_char,
                        9 as size_t,
                    ) != 0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"options list in stats file not valid\n\0" as *const u8
                                as *const c_char,
                        );
                        return -(1 as c_int);
                    }
                    let mut res_factor: c_float = 0.;
                    let mut res_factor_bits: c_float = 0.;
                    let mut i_0: c_int = 0;
                    let mut j_1: c_int = 0;
                    let mut k: uint32_t = 0;
                    let mut l: uint32_t = 0;
                    let mut opts: *mut c_char = stats_buf;
                    stats_in = strchr(stats_buf, '\n' as i32);
                    if stats_in.is_null() {
                        return -(1 as c_int);
                    }
                    *stats_in = '\0' as i32 as c_char;
                    stats_in = stats_in.offset(1);
                    if sscanf(
                        opts,
                        b"#options: %dx%d\0" as *const u8 as *const c_char,
                        &mut i_0 as *mut c_int,
                        &mut j_1 as *mut c_int,
                    ) != 2 as c_int
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"resolution specified in stats file not valid\n\0" as *const u8
                                as *const c_char,
                        );
                        return -(1 as c_int);
                    } else if (*h).param.rc.b_mb_tree != 0 {
                        (*rc).mbtree.srcdim[0 as c_int as usize] = i_0;
                        (*rc).mbtree.srcdim[1 as c_int as usize] = j_1;
                    }
                    res_factor = (*h).param.i_width as c_float * (*h).param.i_height as c_float
                        / (i_0 * j_1) as c_float;
                    res_factor_bits = powf(res_factor, 0.7f32);
                    p = strstr(opts, b"timebase=\0" as *const u8 as *const c_char);
                    if p.is_null()
                        || sscanf(
                            p,
                            b"timebase=%u/%u\0" as *const u8 as *const c_char,
                            &mut k as *mut uint32_t,
                            &mut l as *mut uint32_t,
                        ) != 2 as c_int
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"timebase specified in stats file not valid\n\0" as *const u8
                                as *const c_char,
                        );
                        return -(1 as c_int);
                    }
                    if k != (*h).param.i_timebase_num || l != (*h).param.i_timebase_den {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"timebase mismatch with 1st pass (%u/%u vs %u/%u)\n\0" as *const u8
                                as *const c_char,
                            (*h).param.i_timebase_num,
                            (*h).param.i_timebase_den,
                            k,
                            l,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"bitdepth=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"bitdepth=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && 10 as c_int != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different bitdepth setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const c_char,
                            10 as c_int,
                            i_0,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"weightp=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"weightp=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && (if 0 as c_int > (*h).param.analyse.i_weighted_pred {
                            0 as c_int
                        } else {
                            (*h).param.analyse.i_weighted_pred
                        }) != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different weightp setting than first pass (%d vs %d)\n\0" as *const u8
                                as *const c_char,
                            if 0 as c_int > (*h).param.analyse.i_weighted_pred {
                                0 as c_int
                            } else {
                                (*h).param.analyse.i_weighted_pred
                            },
                            i_0,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"bframes=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"bframes=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && (*h).param.i_bframe != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different bframes setting than first pass (%d vs %d)\n\0" as *const u8
                                as *const c_char,
                            (*h).param.i_bframe,
                            i_0,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"b_pyramid=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"b_pyramid=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && (*h).param.i_bframe_pyramid != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different b_pyramid setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const c_char,
                            (*h).param.i_bframe_pyramid,
                            i_0,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"intra_refresh=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"intra_refresh=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && (*h).param.b_intra_refresh != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different intra_refresh setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const c_char,
                            (*h).param.b_intra_refresh,
                            i_0,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"open_gop=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"open_gop=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && (*h).param.b_open_gop != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different open_gop setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const c_char,
                            (*h).param.b_open_gop,
                            i_0,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"bluray_compat=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"bluray_compat=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && (*h).param.b_bluray_compat != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different bluray_compat setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const c_char,
                            (*h).param.b_bluray_compat,
                            i_0,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"mbtree=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"mbtree=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && (*h).param.rc.b_mb_tree != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different mbtree setting than first pass (%d vs %d)\n\0" as *const u8
                                as *const c_char,
                            (*h).param.rc.b_mb_tree,
                            i_0,
                        );
                        return -(1 as c_int);
                    }
                    p = strstr(opts, b"interlaced=\0" as *const u8 as *const c_char);
                    if !p.is_null() {
                        let mut current: *mut c_char = (if (*h).param.b_interlaced != 0 {
                            if (*h).param.b_tff != 0 {
                                b"tff\0" as *const u8 as *const c_char
                            } else {
                                b"bff\0" as *const u8 as *const c_char
                            }
                        } else if (*h).param.b_fake_interlaced != 0 {
                            b"fake\0" as *const u8 as *const c_char
                        } else {
                            b"0\0" as *const u8 as *const c_char
                        }) as *mut c_char;
                        let mut buf: [c_char; 5] = [0; 5];
                        sscanf(
                            p,
                            b"interlaced=%4s\0" as *const u8 as *const c_char,
                            buf.as_mut_ptr(),
                        );
                        if strcmp(current, buf.as_mut_ptr()) != 0 {
                            x264_10_log(
                                h,
                                X264_LOG_ERROR,
                                b"different interlaced setting than first pass (%s vs %s)\n\0"
                                    as *const u8 as *const c_char,
                                current,
                                buf.as_mut_ptr(),
                            );
                            return -(1 as c_int);
                        }
                    }
                    p = strstr(opts, b"keyint=\0" as *const u8 as *const c_char);
                    if !p.is_null() {
                        p = p.offset(7 as c_int as isize);
                        let mut buf_0: [c_char; 13] =
                            ::core::mem::transmute::<[u8; 13], [c_char; 13]>(*b"infinite \0\0\0\0");
                        if (*h).param.i_keyint_max != X264_KEYINT_MAX_INFINITE {
                            sprintf(
                                buf_0.as_mut_ptr(),
                                b"%d \0" as *const u8 as *const c_char,
                                (*h).param.i_keyint_max,
                            );
                        }
                        if strncmp(p, buf_0.as_mut_ptr(), strlen(buf_0.as_mut_ptr())) != 0 {
                            x264_10_log(
                                h,
                                X264_LOG_ERROR,
                                b"different keyint setting than first pass (%.*s vs %.*s)\n\0"
                                    as *const u8 as *const c_char,
                                strlen(buf_0.as_mut_ptr()).wrapping_sub(1 as size_t),
                                buf_0.as_mut_ptr(),
                                strcspn(p, b" \0" as *const u8 as *const c_char),
                                p,
                            );
                            return -(1 as c_int);
                        }
                    }
                    if !strstr(opts, b"qp=0\0" as *const u8 as *const c_char).is_null()
                        && (*h).param.rc.i_rc_method == X264_RC_ABR
                    {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"1st pass was lossless, bitrate prediction will be inaccurate\n\0"
                                as *const u8 as *const c_char,
                        );
                    }
                    if strstr(opts, b"direct=3\0" as *const u8 as *const c_char).is_null()
                        && (*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_AUTO
                    {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"direct=auto not used on the first pass\n\0" as *const u8
                                as *const c_char,
                        );
                        (*h).mb.b_direct_auto_write = 1 as c_int;
                    }
                    p = strstr(opts, b"b_adapt=\0" as *const u8 as *const c_char);
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"b_adapt=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                        && i_0 >= X264_B_ADAPT_NONE
                        && i_0 <= X264_B_ADAPT_TRELLIS
                    {
                        (*h).param.i_bframe_adaptive = i_0;
                    } else if (*h).param.i_bframe != 0 {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"b_adapt method specified in stats file not valid\n\0" as *const u8
                                as *const c_char,
                        );
                        return -(1 as c_int);
                    }
                    if ((*h).param.rc.b_mb_tree != 0 || (*h).param.rc.i_vbv_buffer_size != 0)
                        && {
                            p = strstr(opts, b"rc_lookahead=\0" as *const u8 as *const c_char);
                            !p.is_null()
                        }
                        && sscanf(
                            p,
                            b"rc_lookahead=%d\0" as *const u8 as *const c_char,
                            &mut i_0 as *mut c_int,
                        ) != 0
                    {
                        (*h).param.rc.i_lookahead = i_0;
                    }
                    p = stats_in;
                    let mut num_entries: c_int = 0;
                    num_entries = -(1 as c_int);
                    while !p.is_null() {
                        p = strchr(p.offset(1 as c_int as isize), ';' as i32);
                        num_entries += 1;
                    }
                    if num_entries == 0 {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"empty stats file\n\0" as *const u8 as *const c_char,
                        );
                        return -(1 as c_int);
                    }
                    (*rc).num_entries = num_entries;
                    if (*h).param.i_frame_total < (*rc).num_entries
                        && (*h).param.i_frame_total > 0 as c_int
                    {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"2nd pass has fewer frames than 1st pass (%d vs %d)\n\0" as *const u8
                                as *const c_char,
                            (*h).param.i_frame_total,
                            (*rc).num_entries,
                        );
                    }
                    if (*h).param.i_frame_total > (*rc).num_entries {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"2nd pass has more frames than 1st pass (%d vs %d)\n\0" as *const u8
                                as *const c_char,
                            (*h).param.i_frame_total,
                            (*rc).num_entries,
                        );
                        return -(1 as c_int);
                    }
                    (*rc).entry = x264_malloc(
                        ((*rc).num_entries as usize)
                            .wrapping_mul(::core::mem::size_of::<ratecontrol_entry_t>() as usize)
                            as int64_t,
                    ) as *mut ratecontrol_entry_t;
                    if (*rc).entry.is_null() {
                        current_block = 4515208850251936372;
                    } else {
                        memset(
                            (*rc).entry as *mut c_void,
                            0 as c_int,
                            ((*rc).num_entries as size_t).wrapping_mul(::core::mem::size_of::<
                                ratecontrol_entry_t,
                            >(
                            )
                                as size_t),
                        );
                        (*rc).entry_out = x264_malloc(((*rc).num_entries as usize).wrapping_mul(
                            ::core::mem::size_of::<*mut ratecontrol_entry_t>() as usize,
                        ) as int64_t)
                            as *mut *mut ratecontrol_entry_t;
                        if (*rc).entry_out.is_null() {
                            current_block = 4515208850251936372;
                        } else {
                            let mut i_1: c_int = 0 as c_int;
                            while i_1 < (*rc).num_entries {
                                let mut rce: *mut ratecontrol_entry_t =
                                    &mut *(*rc).entry.offset(i_1 as isize)
                                        as *mut ratecontrol_entry_t;
                                (*rce).pict_type = SLICE_TYPE_P as c_int;
                                (*rce).new_qscale =
                                    qp2qscale((20 as c_int + QP_BD_OFFSET) as c_float) as c_double;
                                (*rce).qscale = (*rce).new_qscale;
                                (*rce).misc_bits = (*rc).nmb + 10 as c_int;
                                (*rce).new_qp = 0 as c_int as c_float;
                                let ref mut fresh0 = *(*rc).entry_out.offset(i_1 as isize);
                                *fresh0 = rce;
                                i_1 += 1;
                            }
                            p = stats_in;
                            let mut total_qp_aq: c_double = 0 as c_int as c_double;
                            let mut i_2: c_int = 0 as c_int;
                            while i_2 < (*rc).num_entries {
                                let mut rce_0: *mut ratecontrol_entry_t =
                                    0 as *mut ratecontrol_entry_t;
                                let mut frame_number: c_int = 0 as c_int;
                                let mut frame_out_number: c_int = 0 as c_int;
                                let mut pict_type: c_char = 0 as c_char;
                                let mut e: c_int = 0;
                                let mut next: *mut c_char = 0 as *mut c_char;
                                let mut qp_rc: c_float = 0.;
                                let mut qp_aq: c_float = 0.;
                                let mut ref_0: c_int = 0;
                                next = strchr(p, ';' as i32);
                                if !next.is_null() {
                                    let fresh1 = next;
                                    next = next.offset(1);
                                    *fresh1 = 0 as c_char;
                                }
                                e = sscanf(
                                    p,
                                    b" in:%d out:%d \0" as *const u8 as *const c_char,
                                    &mut frame_number as *mut c_int,
                                    &mut frame_out_number as *mut c_int,
                                );
                                if frame_number < 0 as c_int || frame_number >= (*rc).num_entries {
                                    x264_10_log(
                                        h,
                                        X264_LOG_ERROR,
                                        b"bad frame number (%d) at stats line %d\n\0" as *const u8
                                            as *const c_char,
                                        frame_number,
                                        i_2,
                                    );
                                    return -(1 as c_int);
                                }
                                if frame_out_number < 0 as c_int
                                    || frame_out_number >= (*rc).num_entries
                                {
                                    x264_10_log(
                                        h,
                                        X264_LOG_ERROR,
                                        b"bad frame output number (%d) at stats line %d\n\0"
                                            as *const u8
                                            as *const c_char,
                                        frame_out_number,
                                        i_2,
                                    );
                                    return -(1 as c_int);
                                }
                                rce_0 = &mut *(*rc).entry.offset(frame_number as isize)
                                    as *mut ratecontrol_entry_t;
                                let ref mut fresh2 =
                                    *(*rc).entry_out.offset(frame_out_number as isize);
                                *fresh2 = rce_0;
                                (*rce_0).direct_mode = 0 as c_char;
                                e
                                    += sscanf(
                                        p,
                                        b" in:%*d out:%*d type:%c dur:%ld cpbdur:%ld q:%f aq:%f tex:%d mv:%d misc:%d imb:%d pmb:%d smb:%d d:%c\0"
                                            as *const u8 as *const c_char,
                                        &mut pict_type as *mut c_char,
                                        &mut (*rce_0).i_duration as *mut int64_t,
                                        &mut (*rce_0).i_cpb_duration as *mut int64_t,
                                        &mut qp_rc as *mut c_float,
                                        &mut qp_aq as *mut c_float,
                                        &mut (*rce_0).tex_bits as *mut c_int,
                                        &mut (*rce_0).mv_bits as *mut c_int,
                                        &mut (*rce_0).misc_bits as *mut c_int,
                                        &mut (*rce_0).i_count as *mut c_int,
                                        &mut (*rce_0).p_count as *mut c_int,
                                        &mut (*rce_0).s_count as *mut c_int,
                                        &mut (*rce_0).direct_mode as *mut c_char,
                                    );
                                (*rce_0).tex_bits =
                                    ((*rce_0).tex_bits as c_float * res_factor_bits) as c_int;
                                (*rce_0).mv_bits =
                                    ((*rce_0).mv_bits as c_float * res_factor_bits) as c_int;
                                (*rce_0).misc_bits =
                                    ((*rce_0).misc_bits as c_float * res_factor_bits) as c_int;
                                (*rce_0).i_count =
                                    ((*rce_0).i_count as c_float * res_factor) as c_int;
                                (*rce_0).p_count =
                                    ((*rce_0).p_count as c_float * res_factor) as c_int;
                                (*rce_0).s_count =
                                    ((*rce_0).s_count as c_float * res_factor) as c_int;
                                p = strstr(p, b"ref:\0" as *const u8 as *const c_char);
                                if !p.is_null() {
                                    p = p.offset(4 as c_int as isize);
                                    ref_0 = 0 as c_int;
                                    loop {
                                        if !(ref_0 < 16 as c_int) {
                                            current_block = 9728093949049737828;
                                            break;
                                        }
                                        if sscanf(
                                            p,
                                            b" %d\0" as *const u8 as *const c_char,
                                            &mut *(*rce_0)
                                                .refcount
                                                .as_mut_ptr()
                                                .offset(ref_0 as isize)
                                                as *mut c_int,
                                        ) != 1 as c_int
                                        {
                                            current_block = 9728093949049737828;
                                            break;
                                        }
                                        p = strchr(p.offset(1 as c_int as isize), ' ' as i32);
                                        if p.is_null() {
                                            current_block = 12194954648860098588;
                                            break;
                                        }
                                        ref_0 += 1;
                                    }
                                    match current_block {
                                        12194954648860098588 => {}
                                        _ => {
                                            (*rce_0).refs = ref_0;
                                            (*rce_0).i_weight_denom[1 as c_int as usize] =
                                                -(1 as c_int) as int16_t;
                                            (*rce_0).i_weight_denom[0 as c_int as usize] =
                                                (*rce_0).i_weight_denom[1 as c_int as usize];
                                            w = strchr(p, 'w' as i32);
                                            if !w.is_null() {
                                                let mut count: c_int = sscanf(
                                                    w,
                                                    b"w:%hd,%hd,%hd,%hd,%hd,%hd,%hd,%hd\0"
                                                        as *const u8
                                                        as *const c_char,
                                                    &mut *(*rce_0)
                                                        .i_weight_denom
                                                        .as_mut_ptr()
                                                        .offset(0 as c_int as isize)
                                                        as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(0 as c_int as isize))
                                                    .as_mut_ptr()
                                                    .offset(0 as c_int as isize)
                                                        as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(0 as c_int as isize))
                                                    .as_mut_ptr()
                                                    .offset(1 as c_int as isize)
                                                        as *mut int16_t,
                                                    &mut *(*rce_0)
                                                        .i_weight_denom
                                                        .as_mut_ptr()
                                                        .offset(1 as c_int as isize)
                                                        as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(1 as c_int as isize))
                                                    .as_mut_ptr()
                                                    .offset(0 as c_int as isize)
                                                        as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(1 as c_int as isize))
                                                    .as_mut_ptr()
                                                    .offset(1 as c_int as isize)
                                                        as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(2 as c_int as isize))
                                                    .as_mut_ptr()
                                                    .offset(0 as c_int as isize)
                                                        as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(2 as c_int as isize))
                                                    .as_mut_ptr()
                                                    .offset(1 as c_int as isize)
                                                        as *mut int16_t,
                                                );
                                                if count == 3 as c_int {
                                                    (*rce_0).i_weight_denom[1 as c_int as usize] =
                                                        -(1 as c_int) as int16_t;
                                                } else if count != 8 as c_int {
                                                    (*rce_0).i_weight_denom[1 as c_int as usize] =
                                                        -(1 as c_int) as int16_t;
                                                    (*rce_0).i_weight_denom[0 as c_int as usize] =
                                                        (*rce_0).i_weight_denom
                                                            [1 as c_int as usize];
                                                }
                                            }
                                            if pict_type as c_int != 'b' as i32 {
                                                (*rce_0).kept_as_ref = 1 as c_int;
                                            }
                                            match pict_type as c_int {
                                                73 => {
                                                    (*rce_0).frame_type = X264_TYPE_IDR;
                                                    (*rce_0).pict_type = SLICE_TYPE_I as c_int;
                                                }
                                                105 => {
                                                    (*rce_0).frame_type = X264_TYPE_I;
                                                    (*rce_0).pict_type = SLICE_TYPE_I as c_int;
                                                }
                                                80 => {
                                                    (*rce_0).frame_type = X264_TYPE_P;
                                                    (*rce_0).pict_type = SLICE_TYPE_P as c_int;
                                                }
                                                66 => {
                                                    (*rce_0).frame_type = X264_TYPE_BREF;
                                                    (*rce_0).pict_type = SLICE_TYPE_B as c_int;
                                                }
                                                98 => {
                                                    (*rce_0).frame_type = X264_TYPE_B;
                                                    (*rce_0).pict_type = SLICE_TYPE_B as c_int;
                                                }
                                                _ => {
                                                    e = -(1 as c_int);
                                                }
                                            }
                                            if !(e < 14 as c_int) {
                                                (*rce_0).qscale = qp2qscale(qp_rc) as c_double;
                                                total_qp_aq += qp_aq as c_double;
                                                p = next;
                                                i_2 += 1;
                                                continue;
                                            }
                                        }
                                    }
                                }
                                x264_10_log(
                                    h,
                                    X264_LOG_ERROR,
                                    b"statistics are damaged at line %d, parser out=%d\n\0"
                                        as *const u8
                                        as *const c_char,
                                    i_2,
                                    e,
                                );
                                return -(1 as c_int);
                            }
                            if (*h).param.b_stitchable == 0 {
                                (*(*h).pps.as_mut_ptr()).i_pic_init_qp = if ((total_qp_aq
                                    / (*rc).num_entries as c_double
                                    + 0.5f64)
                                    as c_int)
                                    < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                                {
                                    (total_qp_aq / (*rc).num_entries as c_double + 0.5f64) as c_int
                                } else {
                                    51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
                                };
                            }
                            x264_free(stats_buf as *mut c_void);
                            if (*h).param.rc.i_rc_method == X264_RC_ABR {
                                if init_pass2(h) < 0 as c_int {
                                    return -(1 as c_int);
                                }
                            }
                            current_block = 2756754640271984560;
                        }
                    }
                } else {
                    current_block = 2756754640271984560;
                }
                match current_block {
                    4515208850251936372 => {}
                    _ => {
                        if (*h).param.rc.b_stat_write != 0 {
                            let mut p_0: *mut c_char = 0 as *mut c_char;
                            (*rc).psz_stat_file_tmpname = strcat_filename(
                                (*h).param.rc.psz_stat_out,
                                b".temp\0" as *const u8 as *const c_char as *mut c_char,
                            );
                            if (*rc).psz_stat_file_tmpname.is_null() {
                                return -(1 as c_int);
                            }
                            (*rc).p_stat_file_out = fopen(
                                (*rc).psz_stat_file_tmpname,
                                b"wb\0" as *const u8 as *const c_char,
                            ) as *mut FILE;
                            if (*rc).p_stat_file_out.is_null() {
                                x264_10_log(
                                    h,
                                    X264_LOG_ERROR,
                                    b"ratecontrol_init: can't open stats file\n\0" as *const u8
                                        as *const c_char,
                                );
                                return -(1 as c_int);
                            }
                            p_0 = x264_param2string(&mut (*h).param, 1 as c_int);
                            if !p_0.is_null() {
                                fprintf(
                                    (*rc).p_stat_file_out,
                                    b"#options: %s\n\0" as *const u8 as *const c_char,
                                    p_0,
                                );
                            }
                            x264_free(p_0 as *mut c_void);
                            if (*h).param.rc.b_mb_tree != 0 && (*h).param.rc.b_stat_read == 0 {
                                (*rc).psz_mbtree_stat_file_tmpname = strcat_filename(
                                    (*h).param.rc.psz_stat_out,
                                    b".mbtree.temp\0" as *const u8 as *const c_char as *mut c_char,
                                );
                                (*rc).psz_mbtree_stat_file_name = strcat_filename(
                                    (*h).param.rc.psz_stat_out,
                                    b".mbtree\0" as *const u8 as *const c_char as *mut c_char,
                                );
                                if (*rc).psz_mbtree_stat_file_tmpname.is_null()
                                    || (*rc).psz_mbtree_stat_file_name.is_null()
                                {
                                    return -(1 as c_int);
                                }
                                (*rc).p_mbtree_stat_file_out = fopen(
                                    (*rc).psz_mbtree_stat_file_tmpname,
                                    b"wb\0" as *const u8 as *const c_char,
                                )
                                    as *mut FILE;
                                if (*rc).p_mbtree_stat_file_out.is_null() {
                                    x264_10_log(
                                        h,
                                        X264_LOG_ERROR,
                                        b"ratecontrol_init: can't open mbtree stats file\n\0"
                                            as *const u8
                                            as *const c_char,
                                    );
                                    return -(1 as c_int);
                                }
                            }
                        }
                        if (*h).param.rc.b_mb_tree != 0
                            && ((*h).param.rc.b_stat_read != 0 || (*h).param.rc.b_stat_write != 0)
                        {
                            if (*h).param.rc.b_stat_read == 0 {
                                (*rc).mbtree.srcdim[0 as c_int as usize] = (*h).param.i_width;
                                (*rc).mbtree.srcdim[1 as c_int as usize] = (*h).param.i_height;
                            }
                            if macroblock_tree_rescale_init(h, rc) < 0 as c_int {
                                return -(1 as c_int);
                            }
                        }
                        let mut i_3: c_int = 0 as c_int;
                        while i_3 < (*h).param.i_threads {
                            (*(*h).thread[i_3 as usize]).rc = rc.offset(i_3 as isize);
                            if i_3 != 0 {
                                *rc.offset(i_3 as isize) = *rc.offset(0 as c_int as isize);
                                (*(*h).thread[i_3 as usize]).param = (*h).param;
                                (*(*h).thread[i_3 as usize]).mb.b_variable_qp =
                                    (*h).mb.b_variable_qp;
                                (*(*h).thread[i_3 as usize]).mb.ip_offset = (*h).mb.ip_offset;
                            }
                            i_3 += 1;
                        }
                        return 0 as c_int;
                    }
                }
            }
        }
    }
    return -(1 as c_int);
}
#[c2rust::src_loc = "1219:1"]
unsafe extern "C" fn parse_zone(
    mut h: *mut x264_t,
    mut z: *mut x264_zone_t,
    mut p: *mut c_char,
) -> c_int {
    let mut len: c_int = 0 as c_int;
    let mut tok: *mut c_char = 0 as *mut c_char;
    let mut saveptr: *mut c_char = 0 as *mut c_char;
    (*z).param = 0 as *mut x264_param_t;
    (*z).f_bitrate_factor = 1 as c_int as c_float;
    if 3 as c_int
        <= sscanf(
            p,
            b"%d,%d,q=%d%n\0" as *const u8 as *const c_char,
            &mut (*z).i_start as *mut c_int,
            &mut (*z).i_end as *mut c_int,
            &mut (*z).i_qp as *mut c_int,
            &mut len as *mut c_int,
        )
    {
        (*z).b_force_qp = 1 as c_int;
    } else if 3 as c_int
        <= sscanf(
            p,
            b"%d,%d,b=%f%n\0" as *const u8 as *const c_char,
            &mut (*z).i_start as *mut c_int,
            &mut (*z).i_end as *mut c_int,
            &mut (*z).f_bitrate_factor as *mut c_float,
            &mut len as *mut c_int,
        )
    {
        (*z).b_force_qp = 0 as c_int;
    } else if 2 as c_int
        <= sscanf(
            p,
            b"%d,%d%n\0" as *const u8 as *const c_char,
            &mut (*z).i_start as *mut c_int,
            &mut (*z).i_end as *mut c_int,
            &mut len as *mut c_int,
        )
    {
        (*z).b_force_qp = 0 as c_int;
    } else {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"invalid zone: \"%s\"\n\0" as *const u8 as *const c_char,
            p,
        );
        return -(1 as c_int);
    }
    p = p.offset(len as isize);
    if *p == 0 {
        return 0 as c_int;
    }
    (*z).param =
        x264_malloc(::core::mem::size_of::<x264_param_t>() as int64_t) as *mut x264_param_t;
    if (*z).param.is_null() {
        return -(1 as c_int);
    } else {
        memcpy(
            (*z).param as *mut c_void,
            &mut (*h).param as *mut x264_param_t as *const c_void,
            ::core::mem::size_of::<x264_param_t>() as size_t,
        );
        (*(*z).param).opaque = NULL;
        (*(*z).param).param_free = Some(x264_free as unsafe extern "C" fn(*mut c_void) -> ())
            as Option<unsafe extern "C" fn(*mut c_void) -> ()>;
        loop {
            tok = strtok_r(p, b",\0" as *const u8 as *const c_char, &mut saveptr);
            if tok.is_null() {
                break;
            }
            let mut val: *mut c_char = strchr(tok, '=' as i32);
            if !val.is_null() {
                *val = '\0' as i32 as c_char;
                val = val.offset(1);
            }
            if x264_param_parse((*z).param as *mut x264_param_t, tok, val) != 0 {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"invalid zone param: %s = %s\n\0" as *const u8 as *const c_char,
                    tok,
                    val,
                );
                return -(1 as c_int);
            }
            p = 0 as *mut c_char;
        }
        return 0 as c_int;
    };
}
#[c2rust::src_loc = "1263:1"]
unsafe extern "C" fn parse_zones(mut h: *mut x264_t) -> c_int {
    let mut current_block: u64;
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    if !(*h).param.rc.psz_zones.is_null() && (*h).param.rc.i_zones == 0 {
        let mut psz_zones: *mut c_char = 0 as *mut c_char;
        let mut p: *mut c_char = 0 as *mut c_char;
        psz_zones = x264_malloc(strlen((*h).param.rc.psz_zones).wrapping_add(1 as size_t) as int64_t)
            as *mut c_char;
        if psz_zones.is_null() {
            current_block = 12230949131536263527;
        } else {
            strcpy(psz_zones, (*h).param.rc.psz_zones);
            (*h).param.rc.i_zones = 1 as c_int;
            p = psz_zones;
            while *p != 0 {
                (*h).param.rc.i_zones += (*p as c_int == '/' as i32) as c_int;
                p = p.offset(1);
            }
            (*h).param.rc.zones = x264_malloc(
                ((*h).param.rc.i_zones as usize)
                    .wrapping_mul(::core::mem::size_of::<x264_zone_t>() as usize)
                    as int64_t,
            ) as *mut x264_zone_t;
            if (*h).param.rc.zones.is_null() {
                current_block = 12230949131536263527;
            } else {
                p = psz_zones;
                let mut i: c_int = 0 as c_int;
                while i < (*h).param.rc.i_zones {
                    let mut i_tok: c_int =
                        strcspn(p, b"/\0" as *const u8 as *const c_char) as c_int;
                    *p.offset(i_tok as isize) = 0 as c_char;
                    if parse_zone(h, &mut *(*h).param.rc.zones.offset(i as isize), p) != 0 {
                        x264_free(psz_zones as *mut c_void);
                        return -(1 as c_int);
                    }
                    p = p.offset((i_tok + 1 as c_int) as isize);
                    i += 1;
                }
                x264_free(psz_zones as *mut c_void);
                current_block = 11584701595673473500;
            }
        }
    } else {
        current_block = 11584701595673473500;
    }
    match current_block {
        11584701595673473500 => {
            if (*h).param.rc.i_zones > 0 as c_int {
                let mut i_0: c_int = 0 as c_int;
                while i_0 < (*h).param.rc.i_zones {
                    let mut z: x264_zone_t = *(*h).param.rc.zones.offset(i_0 as isize);
                    if z.i_start < 0 as c_int || z.i_start > z.i_end {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"invalid zone: start=%d end=%d\n\0" as *const u8 as *const c_char,
                            z.i_start,
                            z.i_end,
                        );
                        return -(1 as c_int);
                    } else if z.b_force_qp == 0 && z.f_bitrate_factor <= 0 as c_int as c_float {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"invalid zone: bitrate_factor=%f\n\0" as *const u8 as *const c_char,
                            z.f_bitrate_factor as c_double,
                        );
                        return -(1 as c_int);
                    }
                    i_0 += 1;
                }
                (*rc).i_zones = (*h).param.rc.i_zones + 1 as c_int;
                (*rc).zones = x264_malloc(
                    ((*rc).i_zones as usize)
                        .wrapping_mul(::core::mem::size_of::<x264_zone_t>() as usize)
                        as int64_t,
                ) as *mut x264_zone_t;
                if (*rc).zones.is_null() {
                    current_block = 12230949131536263527;
                } else {
                    memcpy(
                        (*rc).zones.offset(1 as c_int as isize) as *mut c_void,
                        (*h).param.rc.zones as *const c_void,
                        (((*rc).i_zones - 1 as c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<x264_zone_t>() as size_t),
                    );
                    (*(*rc).zones.offset(0 as c_int as isize)).i_start = 0 as c_int;
                    (*(*rc).zones.offset(0 as c_int as isize)).i_end = INT_MAX;
                    (*(*rc).zones.offset(0 as c_int as isize)).b_force_qp = 0 as c_int;
                    (*(*rc).zones.offset(0 as c_int as isize)).f_bitrate_factor =
                        1 as c_int as c_float;
                    let ref mut fresh3 = (*(*rc).zones.offset(0 as c_int as isize)).param;
                    *fresh3 = x264_malloc(::core::mem::size_of::<x264_param_t>() as int64_t)
                        as *mut x264_param_t;
                    if (*(*rc).zones.offset(0 as c_int as isize)).param.is_null() {
                        current_block = 12230949131536263527;
                    } else {
                        memcpy(
                            (*(*rc).zones.offset(0 as c_int as isize)).param as *mut c_void,
                            &mut (*h).param as *mut x264_param_t as *const c_void,
                            ::core::mem::size_of::<x264_param_t>() as size_t,
                        );
                        let ref mut fresh4 =
                            (*(*(*rc).zones.offset(0 as c_int as isize)).param).opaque;
                        *fresh4 = NULL;
                        let mut i_1: c_int = 1 as c_int;
                        while i_1 < (*rc).i_zones {
                            if (*(*rc).zones.offset(i_1 as isize)).param.is_null() {
                                let ref mut fresh5 = (*(*rc).zones.offset(i_1 as isize)).param;
                                *fresh5 = (*(*rc).zones.offset(0 as c_int as isize)).param;
                            }
                            i_1 += 1;
                        }
                        current_block = 2122094917359643297;
                    }
                }
            } else {
                current_block = 2122094917359643297;
            }
            match current_block {
                12230949131536263527 => {}
                _ => return 0 as c_int,
            }
        }
        _ => {}
    }
    return -(1 as c_int);
}
#[c2rust::src_loc = "1333:1"]
unsafe extern "C" fn get_zone(mut h: *mut x264_t, mut frame_num: c_int) -> *mut x264_zone_t {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut i: c_int = (*rc).i_zones - 1 as c_int;
    while i >= 0 as c_int {
        let mut z: *mut x264_zone_t = &mut *(*rc).zones.offset(i as isize) as *mut x264_zone_t;
        if frame_num >= (*z).i_start && frame_num <= (*z).i_end {
            return z;
        }
        i -= 1;
    }
    return 0 as *mut x264_zone_t;
}
#[no_mangle]
#[c2rust::src_loc = "1345:1"]
unsafe extern "C" fn x264_10_ratecontrol_summary(mut h: *mut x264_t) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    if (*rc).b_abr != 0 && (*h).param.rc.i_rc_method == X264_RC_ABR && (*rc).cbr_decay > 0.9999f64 {
        let mut base_cplx: c_double = ((*h).mb.i_mb_count
            * (if (*h).param.i_bframe != 0 {
                120 as c_int
            } else {
                80 as c_int
            })) as c_double;
        let mut mbtree_offset: c_double = if (*h).param.rc.b_mb_tree != 0 {
            (1.0f64 - (*h).param.rc.f_qcompress as c_double) * 13.5f64
        } else {
            0 as c_int as c_double
        };
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"final ratefactor: %.2f\n\0" as *const u8 as *const c_char,
            qscale2qp(
                (pow(base_cplx, 1 as c_int as c_double - (*rc).qcompress) * (*rc).cplxr_sum
                    / (*rc).wanted_bits_window) as c_float,
            ) as c_double
                - mbtree_offset
                - QP_BD_OFFSET as c_double,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "1358:1"]
unsafe extern "C" fn x264_10_ratecontrol_delete(mut h: *mut x264_t) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut b_regular_file: c_int = 0;
    if !(*rc).p_stat_file_out.is_null() {
        b_regular_file = x264_is_regular_file((*rc).p_stat_file_out);
        fclose((*rc).p_stat_file_out);
        if (*h).i_frame >= (*rc).num_entries && b_regular_file != 0 {
            if rename((*rc).psz_stat_file_tmpname, (*h).param.rc.psz_stat_out) != 0 as c_int {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"failed to rename \"%s\" to \"%s\"\n\0" as *const u8 as *const c_char,
                    (*rc).psz_stat_file_tmpname,
                    (*h).param.rc.psz_stat_out,
                );
            }
        }
        x264_free((*rc).psz_stat_file_tmpname as *mut c_void);
    }
    if !(*rc).p_mbtree_stat_file_out.is_null() {
        b_regular_file = x264_is_regular_file((*rc).p_mbtree_stat_file_out);
        fclose((*rc).p_mbtree_stat_file_out);
        if (*h).i_frame >= (*rc).num_entries && b_regular_file != 0 {
            if rename(
                (*rc).psz_mbtree_stat_file_tmpname,
                (*rc).psz_mbtree_stat_file_name,
            ) != 0 as c_int
            {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"failed to rename \"%s\" to \"%s\"\n\0" as *const u8 as *const c_char,
                    (*rc).psz_mbtree_stat_file_tmpname,
                    (*rc).psz_mbtree_stat_file_name,
                );
            }
        }
        x264_free((*rc).psz_mbtree_stat_file_tmpname as *mut c_void);
        x264_free((*rc).psz_mbtree_stat_file_name as *mut c_void);
    }
    if !(*rc).p_mbtree_stat_file_in.is_null() {
        fclose((*rc).p_mbtree_stat_file_in);
    }
    x264_free((*rc).pred as *mut c_void);
    x264_free((*rc).pred_b_from_p as *mut c_void);
    x264_free((*rc).entry as *mut c_void);
    x264_free((*rc).entry_out as *mut c_void);
    macroblock_tree_rescale_destroy(rc);
    if !(*rc).zones.is_null() {
        x264_param_cleanup((*(*rc).zones.offset(0 as c_int as isize)).param as *mut x264_param_t);
        x264_free((*(*rc).zones.offset(0 as c_int as isize)).param as *mut c_void);
        let mut i: c_int = 1 as c_int;
        while i < (*rc).i_zones {
            if (*(*rc).zones.offset(i as isize)).param
                != (*(*rc).zones.offset(0 as c_int as isize)).param
                && (*(*(*rc).zones.offset(i as isize)).param)
                    .param_free
                    .is_some()
            {
                x264_param_cleanup((*(*rc).zones.offset(i as isize)).param as *mut x264_param_t);
                (*(*(*rc).zones.offset(i as isize)).param)
                    .param_free
                    .expect("non-null function pointer")(
                    (*(*rc).zones.offset(i as isize)).param as *mut c_void,
                );
            }
            i += 1;
        }
        x264_free((*rc).zones as *mut c_void);
    }
    x264_free(rc as *mut c_void);
}
#[c2rust::src_loc = "1410:1"]
unsafe extern "C" fn accum_p_qp_update(mut h: *mut x264_t, mut qp: c_float) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    (*rc).accum_p_qp *= 0.95f64;
    (*rc).accum_p_norm *= 0.95f64;
    (*rc).accum_p_norm += 1 as c_int as c_double;
    if (*h).sh.i_type == SLICE_TYPE_I as c_int {
        (*rc).accum_p_qp += qp as c_double + (*rc).ip_offset;
    } else {
        (*rc).accum_p_qp += qp as c_double;
    };
}
#[no_mangle]
#[c2rust::src_loc = "1422:1"]
unsafe extern "C" fn x264_10_ratecontrol_zone_init(mut h: *mut x264_t) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut zone: *mut x264_zone_t = get_zone(h, (*(*h).fenc).i_frame);
    if !zone.is_null() && ((*rc).prev_zone.is_null() || (*zone).param != (*(*rc).prev_zone).param) {
        x264_10_encoder_reconfig_apply(h, (*zone).param as *mut x264_param_t);
    }
    (*rc).prev_zone = zone;
}
#[no_mangle]
#[c2rust::src_loc = "1432:1"]
unsafe extern "C" fn x264_10_ratecontrol_start(
    mut h: *mut x264_t,
    mut i_force_qp: c_int,
    mut overhead: c_int,
) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut rce: *mut ratecontrol_entry_t = 0 as *mut ratecontrol_entry_t;
    let mut zone: *mut x264_zone_t = get_zone(h, (*(*h).fenc).i_frame);
    let mut q: c_float = 0.;
    if (*h).param.rc.b_stat_read != 0 {
        let mut frame: c_int = (*(*h).fenc).i_frame;
        if frame >= 0 as c_int && frame < (*rc).num_entries {
        } else {
            __assert_fail(
                b"frame >= 0 && frame < rc->num_entries\0" as *const u8 as *const c_char,
                b"encoder/ratecontrol.c\0" as *const u8 as *const c_char,
                1444 as c_uint,
                ::core::mem::transmute::<[u8; 51], [c_char; 51]>(
                    *b"void x264_10_ratecontrol_start(x264_t *, int, int)\0",
                )
                .as_ptr(),
            );
        }
        (*rc).rce = &mut *(*rc).entry.offset(frame as isize) as *mut ratecontrol_entry_t;
        rce = (*rc).rce;
        if (*h).sh.i_type == SLICE_TYPE_B as c_int
            && (*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_AUTO
        {
            (*h).sh.b_direct_spatial_mv_pred = ((*rce).direct_mode as c_int == 's' as i32) as c_int;
            (*h).mb.b_direct_auto_read = ((*rce).direct_mode as c_int == 's' as i32
                || (*rce).direct_mode as c_int == 't' as i32)
                as c_int;
        }
    }
    if (*rc).b_vbv != 0 {
        memset(
            (*(*h).fdec).i_row_bits as *mut c_void,
            0 as c_int,
            ((*h).mb.i_mb_height as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
        );
        memset(
            (*(*h).fdec).f_row_qp as *mut c_void,
            0 as c_int,
            ((*h).mb.i_mb_height as size_t)
                .wrapping_mul(::core::mem::size_of::<c_float>() as size_t),
        );
        memset(
            (*(*h).fdec).f_row_qscale as *mut c_void,
            0 as c_int,
            ((*h).mb.i_mb_height as size_t)
                .wrapping_mul(::core::mem::size_of::<c_float>() as size_t),
        );
        (*rc).row_pred =
            (*(*rc).row_preds.as_mut_ptr().offset((*h).sh.i_type as isize)).as_mut_ptr();
        (*rc).buffer_rate = (*(*h).fenc).i_cpb_duration as c_double
            * (*rc).vbv_max_rate
            * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double;
        update_vbv_plan(h, overhead);
        let mut l: *const x264_level_t = x264_levels.as_ptr();
        while (*l).level_idc as c_int != 0 as c_int
            && (*l).level_idc as c_int != (*h).param.i_level_idc
        {
            l = l.offset(1);
        }
        let mut mincr: c_int = (*l).mincr as c_int;
        if (*h).param.b_bluray_compat != 0 {
            mincr = 4 as c_int;
        }
        if (*(*h).sps.as_mut_ptr()).i_profile_idc > PROFILE_HIGH as c_int {
            (*rc).frame_size_maximum = 1e9f64;
        } else if (*h).i_frame == 0 as c_int {
            let mut fr: c_double = 1.0f64
                / (if (*h).param.i_level_idc >= 60 as c_int {
                    300 as c_int
                } else {
                    172 as c_int
                }) as c_double;
            let mut pic_size_in_mbs: c_int = (*h).mb.i_mb_width * (*h).mb.i_mb_height;
            (*rc).frame_size_maximum = (384 as c_int * BIT_DEPTH) as c_double
                * (if pic_size_in_mbs as c_double > fr * (*l).mbps as c_double {
                    pic_size_in_mbs as c_double
                } else {
                    fr * (*l).mbps as c_double
                })
                / mincr as c_double;
        } else {
            (*rc).frame_size_maximum = (384 as c_int * BIT_DEPTH) as c_double
                * ((*(*h).fenc).i_cpb_duration as c_double
                    * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
                    / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double)
                * (*l).mbps as c_double
                / mincr as c_double;
        }
    }
    if (*h).sh.i_type != SLICE_TYPE_B as c_int {
        (*rc).bframes = (*(*h).fenc).i_bframes as c_int;
    }
    if (*rc).b_abr != 0 {
        q = qscale2qp(rate_estimate_qscale(h));
    } else if (*rc).b_2pass != 0 {
        (*rce).new_qscale = rate_estimate_qscale(h) as c_double;
        q = qscale2qp((*rce).new_qscale as c_float);
    } else {
        if (*h).sh.i_type == SLICE_TYPE_B as c_int && (*(*h).fdec).b_kept_as_ref != 0 {
            q = (((*rc).qp_constant[SLICE_TYPE_B as c_int as usize]
                + (*rc).qp_constant[SLICE_TYPE_P as c_int as usize])
                / 2 as c_int) as c_float;
        } else {
            q = (*rc).qp_constant[(*h).sh.i_type as usize] as c_float;
        }
        if !zone.is_null() {
            if (*zone).b_force_qp != 0 {
                q += ((*zone).i_qp - (*rc).qp_constant[SLICE_TYPE_P as c_int as usize]) as c_float;
            } else {
                q -= 6 as c_int as c_float * log2f((*zone).f_bitrate_factor);
            }
        }
    }
    if i_force_qp != X264_QP_AUTO {
        q = (i_force_qp - 1 as c_int) as c_float;
    }
    q = x264_clip3f(
        q as c_double,
        (*h).param.rc.i_qp_min as c_double,
        (*h).param.rc.i_qp_max as c_double,
    ) as c_float;
    (*rc).qpa_aq_prev = 0 as c_int;
    (*rc).qpa_aq = (*rc).qpa_aq_prev;
    (*rc).qpa_rc_prev = (*rc).qpa_aq as c_float;
    (*rc).qpa_rc = (*rc).qpa_rc_prev;
    (*rc).qpm = q;
    (*(*h).fdec).f_qp_avg_aq = (*rc).qpm;
    (*(*h).fdec).f_qp_avg_rc = (*(*h).fdec).f_qp_avg_aq;
    if !rce.is_null() {
        (*rce).new_qp = q;
    }
    accum_p_qp_update(h, (*rc).qpm);
    if (*h).sh.i_type != SLICE_TYPE_B as c_int {
        (*rc).last_non_b_pict_type = (*h).sh.i_type;
    }
}
#[c2rust::src_loc = "1540:1"]
unsafe extern "C" fn predict_row_size(
    mut h: *mut x264_t,
    mut y: c_int,
    mut qscale: c_float,
) -> c_float {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut pred_s: c_float = predict_size(
        &mut *(*rc).row_pred.offset(0 as c_int as isize),
        qscale,
        *(*(*h).fdec).i_row_satd.offset(y as isize) as c_float,
    );
    if (*h).sh.i_type == SLICE_TYPE_I as c_int
        || qscale
            >= *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                .f_row_qscale
                .offset(y as isize)
    {
        if (*h).sh.i_type == SLICE_TYPE_P as c_int
            && (*(*h).fref[0 as c_int as usize][0 as c_int as usize]).i_type == (*(*h).fdec).i_type
            && *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                .f_row_qscale
                .offset(y as isize)
                > 0 as c_int as c_float
            && *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                .i_row_satd
                .offset(y as isize)
                > 0 as c_int
            && abs(*(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                .i_row_satd
                .offset(y as isize)
                - *(*(*h).fdec).i_row_satd.offset(y as isize))
                < *(*(*h).fdec).i_row_satd.offset(y as isize) / 2 as c_int
        {
            let mut pred_t: c_float = (*(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                .i_row_bits
                .offset(y as isize)
                * *(*(*h).fdec).i_row_satd.offset(y as isize)
                / *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                    .i_row_satd
                    .offset(y as isize)) as c_float
                * *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                    .f_row_qscale
                    .offset(y as isize)
                / qscale;
            return (pred_s + pred_t) * 0.5f32;
        }
        return pred_s;
    } else {
        let mut pred_intra: c_float = predict_size(
            &mut *(*rc).row_pred.offset(1 as c_int as isize),
            qscale,
            *(*(*h).fdec).i_row_satds[0 as c_int as usize][0 as c_int as usize].offset(y as isize)
                as c_float,
        );
        return pred_intra + pred_s;
    };
}
#[c2rust::src_loc = "1569:1"]
unsafe extern "C" fn row_bits_so_far(mut h: *mut x264_t, mut y: c_int) -> c_int {
    let mut bits: c_int = 0 as c_int;
    let mut i: c_int = (*h).i_threadslice_start;
    while i <= y {
        bits += *(*(*h).fdec).i_row_bits.offset(i as isize);
        i += 1;
    }
    return bits;
}
#[c2rust::src_loc = "1577:1"]
unsafe extern "C" fn predict_row_size_to_end(
    mut h: *mut x264_t,
    mut y: c_int,
    mut qp: c_float,
) -> c_float {
    let mut qscale: c_float = qp2qscale(qp);
    let mut bits: c_float = 0 as c_int as c_float;
    let mut i: c_int = y + 1 as c_int;
    while i < (*h).i_threadslice_end {
        bits += predict_row_size(h, i, qscale);
        i += 1;
    }
    return bits;
}
#[no_mangle]
#[c2rust::src_loc = "1590:1"]
unsafe extern "C" fn x264_10_ratecontrol_mb(mut h: *mut x264_t, mut bits: c_int) -> c_int {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let y: c_int = (*h).mb.i_mb_y;
    *(*(*h).fdec).i_row_bits.offset(y as isize) += bits;
    (*rc).qpa_aq += (*h).mb.i_qp;
    if (*h).mb.i_mb_x != (*h).mb.i_mb_width - 1 as c_int {
        return 0 as c_int;
    }
    (*rc).qpa_rc += (*rc).qpm * (*h).mb.i_mb_width as c_float;
    if (*rc).b_vbv == 0 {
        return 0 as c_int;
    }
    let mut qscale: c_float = qp2qscale((*rc).qpm);
    *(*(*h).fdec).f_row_qp.offset(y as isize) = (*rc).qpm;
    *(*(*h).fdec).f_row_qscale.offset(y as isize) = qscale;
    update_predictor(
        &mut *(*rc).row_pred.offset(0 as c_int as isize),
        qscale,
        *(*(*h).fdec).i_row_satd.offset(y as isize) as c_float,
        *(*(*h).fdec).i_row_bits.offset(y as isize) as c_float,
    );
    if (*h).sh.i_type != SLICE_TYPE_I as c_int
        && (*rc).qpm
            < *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                .f_row_qp
                .offset(y as isize)
    {
        update_predictor(
            &mut *(*rc).row_pred.offset(1 as c_int as isize),
            qscale,
            *(*(*h).fdec).i_row_satds[0 as c_int as usize][0 as c_int as usize].offset(y as isize)
                as c_float,
            *(*(*h).fdec).i_row_bits.offset(y as isize) as c_float,
        );
    }
    if (*h).sh.b_mbaff != 0 && y & 1 as c_int == 0 {
        return 0 as c_int;
    }
    let mut can_reencode_row: c_int =
        ((*h).sh.i_first_mb <= ((*h).mb.i_mb_y - (*h).sh.b_mbaff) * (*h).mb.i_mb_stride) as c_int;
    let mut prev_row_qp: c_float = *(*(*h).fdec).f_row_qp.offset(y as isize);
    let mut qp_absolute_max: c_float = (*h).param.rc.i_qp_max as c_float;
    if (*rc).rate_factor_max_increment != 0. {
        qp_absolute_max = if qp_absolute_max < (*rc).qp_novbv + (*rc).rate_factor_max_increment {
            qp_absolute_max
        } else {
            (*rc).qp_novbv + (*rc).rate_factor_max_increment
        };
    }
    let mut qp_max: c_float =
        if (prev_row_qp + (*h).param.rc.i_qp_step as c_float) < qp_absolute_max {
            prev_row_qp + (*h).param.rc.i_qp_step as c_float
        } else {
            qp_absolute_max
        };
    let mut qp_min: c_float =
        if prev_row_qp - (*h).param.rc.i_qp_step as c_float > (*h).param.rc.i_qp_min as c_float {
            prev_row_qp - (*h).param.rc.i_qp_step as c_float
        } else {
            (*h).param.rc.i_qp_min as c_float
        };
    let mut step_size: c_float = 0.5f32;
    let mut slice_size_planned: c_float = (if (*h).param.b_sliced_threads != 0 {
        (*rc).slice_size_planned
    } else {
        (*rc).frame_size_planned
    }) as c_float;
    let mut bits_so_far: c_float = row_bits_so_far(h, y) as c_float;
    ::core::ptr::write_volatile(&mut (*rc).bits_so_far as *mut c_float, bits_so_far);
    let mut max_frame_error: c_float =
        x264_clip3f(1.0f64 / (*h).mb.i_mb_height as c_double, 0.05f64, 0.25f64) as c_float;
    let mut max_frame_size: c_float = ((*rc).frame_size_maximum
        - (*rc).frame_size_maximum * max_frame_error as c_double)
        as c_float;
    max_frame_size = (if (max_frame_size as c_double)
        < (*rc).buffer_fill - (*rc).buffer_rate * max_frame_error as c_double
    {
        max_frame_size as c_double
    } else {
        (*rc).buffer_fill - (*rc).buffer_rate * max_frame_error as c_double
    }) as c_float;
    let mut size_of_other_slices: c_float = 0 as c_int as c_float;
    if (*h).param.b_sliced_threads != 0 {
        let mut bits_so_far_of_other_slices: c_float = 0 as c_int as c_float;
        let mut i: c_int = 0 as c_int;
        while i < (*h).param.i_threads {
            if h != (*h).thread[i as usize] {
                size_of_other_slices += (*(*(*h).thread[i as usize]).rc).frame_size_estimated;
                bits_so_far_of_other_slices += (*(*(*h).thread[i as usize]).rc).bits_so_far;
            }
            i += 1;
        }
        let mut weight: c_float = x264_clip3f(
            ((bits_so_far_of_other_slices + (*rc).frame_size_estimated)
                / (size_of_other_slices + (*rc).frame_size_estimated)) as c_double,
            0.0f64,
            1.0f64,
        ) as c_float;
        let mut frame_size_planned: c_float = ((*rc).frame_size_planned
            - (*rc).frame_size_planned * max_frame_error as c_double)
            as c_float;
        let mut size_of_other_slices_planned: c_float = ((if frame_size_planned < max_frame_size {
            frame_size_planned
        } else {
            max_frame_size
        }) as c_double
            - (*rc).slice_size_planned)
            as c_float;
        size_of_other_slices_planned = if size_of_other_slices_planned > bits_so_far_of_other_slices
        {
            size_of_other_slices_planned
        } else {
            bits_so_far_of_other_slices
        };
        size_of_other_slices = (size_of_other_slices - size_of_other_slices_planned) * weight
            + size_of_other_slices_planned;
    }
    if y < (*h).i_threadslice_end - 1 as c_int {
        if (*h).sh.i_type == SLICE_TYPE_B as c_int {
            qp_min = if qp_min
                > (if *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as c_int) as isize)
                    > *(*(*h).fref[1 as c_int as usize][0 as c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as c_int) as isize)
                {
                    *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as c_int) as isize)
                } else {
                    *(*(*h).fref[1 as c_int as usize][0 as c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as c_int) as isize)
                }) {
                qp_min
            } else if *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                .f_row_qp
                .offset((y + 1 as c_int) as isize)
                > *(*(*h).fref[1 as c_int as usize][0 as c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as c_int) as isize)
            {
                *(*(*h).fref[0 as c_int as usize][0 as c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as c_int) as isize)
            } else {
                *(*(*h).fref[1 as c_int as usize][0 as c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as c_int) as isize)
            };
            (*rc).qpm = if (*rc).qpm > qp_min {
                (*rc).qpm
            } else {
                qp_min
            };
        }
        let mut buffer_left_planned: c_float =
            ((*rc).buffer_fill - (*rc).frame_size_planned) as c_float;
        buffer_left_planned = if buffer_left_planned > 0.0f32 {
            buffer_left_planned
        } else {
            0.0f32
        };
        let mut rc_tol: c_float = ((buffer_left_planned / (*h).param.i_threads as c_float)
            as c_double
            * (*rc).rate_tolerance) as c_float;
        let mut b1: c_float =
            bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
        let mut trust_coeff: c_float = x264_clip3f(
            (bits_so_far / slice_size_planned) as c_double,
            0.0f64,
            1.0f64,
        ) as c_float;
        if trust_coeff < 0.05f32 {
            qp_absolute_max = prev_row_qp;
            qp_max = qp_absolute_max;
        }
        if (*h).sh.i_type != SLICE_TYPE_I as c_int {
            rc_tol *= 0.5f32;
        }
        if (*rc).b_vbv_min_rate == 0 {
            qp_min = if qp_min > (*rc).qp_novbv {
                qp_min
            } else {
                (*rc).qp_novbv
            };
        }
        while (*rc).qpm < qp_max
            && (b1 as c_double > (*rc).frame_size_planned + rc_tol as c_double
                || b1 as c_double > (*rc).frame_size_planned && (*rc).qpm < (*rc).qp_novbv
                || b1 as c_double > (*rc).buffer_fill - (buffer_left_planned * 0.5f32) as c_double)
        {
            (*rc).qpm += step_size;
            b1 = bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
        }
        let mut b_max: c_float = (b1 as c_double
            + (((*rc).buffer_fill - (*rc).buffer_size + (*rc).buffer_rate) * 0.90f64
                - b1 as c_double)
                * trust_coeff as c_double) as c_float;
        (*rc).qpm -= step_size;
        let mut b2: c_float =
            bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
        while (*rc).qpm > qp_min
            && (*rc).qpm < prev_row_qp
            && ((*rc).qpm > *(*(*h).fdec).f_row_qp.offset(0 as c_int as isize)
                || (*rc).single_frame_vbv != 0)
            && b2 < max_frame_size
            && ((b2 as c_double) < (*rc).frame_size_planned * 0.8f64 || b2 < b_max)
        {
            b1 = b2;
            (*rc).qpm -= step_size;
            b2 = bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
        }
        (*rc).qpm += step_size;
        while (*rc).qpm < qp_absolute_max && b1 > max_frame_size {
            (*rc).qpm += step_size;
            b1 = bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
        }
        ::core::ptr::write_volatile(
            &mut (*rc).frame_size_estimated as *mut c_float,
            b1 - size_of_other_slices,
        );
        if (*rc).qpm > qp_max && prev_row_qp < qp_max && can_reencode_row != 0 {
            (*rc).qpm = x264_clip3f(
                ((prev_row_qp + (*rc).qpm) * 0.5f32) as c_double,
                (prev_row_qp + 1.0f32) as c_double,
                qp_max as c_double,
            ) as c_float;
            (*rc).qpa_rc = (*rc).qpa_rc_prev;
            (*rc).qpa_aq = (*rc).qpa_aq_prev;
            *(*(*h).fdec).i_row_bits.offset(y as isize) = 0 as c_int;
            *(*(*h).fdec)
                .i_row_bits
                .offset((y - (*h).sh.b_mbaff) as isize) = 0 as c_int;
            return -(1 as c_int);
        }
    } else {
        ::core::ptr::write_volatile(&mut (*rc).frame_size_estimated as *mut c_float, bits_so_far);
        if (*rc).qpm < qp_max
            && can_reencode_row != 0
            && (bits_so_far + size_of_other_slices) as c_double
                > (if (*rc).frame_size_maximum < (*rc).buffer_fill {
                    (*rc).frame_size_maximum
                } else {
                    (*rc).buffer_fill
                })
        {
            (*rc).qpm = qp_max;
            (*rc).qpa_rc = (*rc).qpa_rc_prev;
            (*rc).qpa_aq = (*rc).qpa_aq_prev;
            *(*(*h).fdec).i_row_bits.offset(y as isize) = 0 as c_int;
            *(*(*h).fdec)
                .i_row_bits
                .offset((y - (*h).sh.b_mbaff) as isize) = 0 as c_int;
            return -(1 as c_int);
        }
    }
    (*rc).qpa_rc_prev = (*rc).qpa_rc;
    (*rc).qpa_aq_prev = (*rc).qpa_aq;
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1748:1"]
unsafe extern "C" fn x264_10_ratecontrol_qp(mut h: *mut x264_t) -> c_int {
    return x264_clip3(
        ((*(*h).rc).qpm + 0.5f32) as c_int,
        (*h).param.rc.i_qp_min,
        (*h).param.rc.i_qp_max,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1754:1"]
unsafe extern "C" fn x264_10_ratecontrol_mb_qp(mut h: *mut x264_t) -> c_int {
    let mut qp: c_float = (*(*h).rc).qpm;
    if (*h).param.rc.i_aq_mode != 0 {
        let mut qp_offset: c_float = if (*(*h).fdec).b_kept_as_ref != 0 {
            *(*(*h).fenc).f_qp_offset.offset((*h).mb.i_mb_xy as isize)
        } else {
            *(*(*h).fenc).f_qp_offset_aq.offset((*h).mb.i_mb_xy as isize)
        };
        if qp > QP_MAX_SPEC as c_float {
            qp_offset *= (QP_MAX as c_float - qp) / (QP_MAX - QP_MAX_SPEC) as c_float;
        }
        qp += qp_offset;
    }
    return x264_clip3(
        (qp + 0.5f32) as c_int,
        (*h).param.rc.i_qp_min,
        (*h).param.rc.i_qp_max,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1771:1"]
unsafe extern "C" fn x264_10_ratecontrol_slice_type(
    mut h: *mut x264_t,
    mut frame_num: c_int,
) -> c_int {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    if (*h).param.rc.b_stat_read != 0 {
        if frame_num >= (*rc).num_entries {
            (*h).param.rc.i_qp_constant =
                (if (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize] == 0 as c_int {
                    (24 as c_int + QP_BD_OFFSET) as c_double
                } else {
                    1 as c_int as c_double
                        + (*h).stat.f_frame_qp[SLICE_TYPE_P as c_int as usize]
                            / (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize] as c_double
                }) as c_int;
            (*rc).qp_constant[SLICE_TYPE_P as c_int as usize] =
                x264_clip3((*h).param.rc.i_qp_constant, 0 as c_int, QP_MAX);
            (*rc).qp_constant[SLICE_TYPE_I as c_int as usize] = x264_clip3(
                (qscale2qp(
                    qp2qscale((*h).param.rc.i_qp_constant as c_float) / (*h).param.rc.f_ip_factor,
                ) as c_double
                    + 0.5f64) as c_int,
                0 as c_int,
                QP_MAX,
            );
            (*rc).qp_constant[SLICE_TYPE_B as c_int as usize] = x264_clip3(
                (qscale2qp(
                    qp2qscale((*h).param.rc.i_qp_constant as c_float) * (*h).param.rc.f_pb_factor,
                ) as c_double
                    + 0.5f64) as c_int,
                0 as c_int,
                QP_MAX,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"2nd pass has more frames than 1st pass (%d)\n\0" as *const u8 as *const c_char,
                (*rc).num_entries,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"continuing anyway, at constant QP=%d\n\0" as *const u8 as *const c_char,
                (*h).param.rc.i_qp_constant,
            );
            if (*h).param.i_bframe_adaptive != 0 {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"disabling adaptive B-frames\n\0" as *const u8 as *const c_char,
                );
            }
            let mut i: c_int = 0 as c_int;
            while i < (*h).param.i_threads {
                (*(*(*h).thread[i as usize]).rc).b_abr = 0 as c_int;
                (*(*(*h).thread[i as usize]).rc).b_2pass = 0 as c_int;
                (*(*h).thread[i as usize]).param.rc.i_rc_method = X264_RC_CQP;
                (*(*h).thread[i as usize]).param.rc.b_stat_read = 0 as c_int;
                (*(*h).thread[i as usize]).param.i_bframe_adaptive = 0 as c_int;
                (*(*h).thread[i as usize]).param.i_scenecut_threshold = 0 as c_int;
                (*(*h).thread[i as usize]).param.rc.b_mb_tree = 0 as c_int;
                if (*(*h).thread[i as usize]).param.i_bframe > 1 as c_int {
                    (*(*h).thread[i as usize]).param.i_bframe = 1 as c_int;
                }
                i += 1;
            }
            return X264_TYPE_AUTO;
        }
        return (*(*rc).entry.offset(frame_num as isize)).frame_type;
    } else {
        return X264_TYPE_AUTO;
    };
}
#[no_mangle]
#[c2rust::src_loc = "1812:1"]
unsafe extern "C" fn x264_10_ratecontrol_set_weights(
    mut h: *mut x264_t,
    mut frm: *mut x264_frame_t,
) {
    let mut rce: *mut ratecontrol_entry_t =
        &mut *(*(*h).rc).entry.offset((*frm).i_frame as isize) as *mut ratecontrol_entry_t;
    if (*h).param.analyse.i_weighted_pred <= 0 as c_int {
        return;
    }
    if (*rce).i_weight_denom[0 as c_int as usize] as c_int >= 0 as c_int {
        (*frm).weight[0 as c_int as usize][0 as c_int as usize].i_scale =
            (*rce).weight[0 as c_int as usize][0 as c_int as usize] as int32_t;
        (*frm).weight[0 as c_int as usize][0 as c_int as usize].i_denom =
            (*rce).i_weight_denom[0 as c_int as usize] as int32_t;
        (*frm).weight[0 as c_int as usize][0 as c_int as usize].i_offset =
            (*rce).weight[0 as c_int as usize][1 as c_int as usize] as int32_t;
        (*h).mc.weight_cache.expect("non-null function pointer")(
            h,
            &mut *(*(*frm).weight.as_mut_ptr().offset(0 as c_int as isize))
                .as_mut_ptr()
                .offset(0 as c_int as isize),
        );
    }
    if (*rce).i_weight_denom[1 as c_int as usize] as c_int >= 0 as c_int {
        (*frm).weight[0 as c_int as usize][1 as c_int as usize].i_scale =
            (*rce).weight[1 as c_int as usize][0 as c_int as usize] as int32_t;
        (*frm).weight[0 as c_int as usize][1 as c_int as usize].i_denom =
            (*rce).i_weight_denom[1 as c_int as usize] as int32_t;
        (*frm).weight[0 as c_int as usize][1 as c_int as usize].i_offset =
            (*rce).weight[1 as c_int as usize][1 as c_int as usize] as int32_t;
        (*h).mc.weight_cache.expect("non-null function pointer")(
            h,
            &mut *(*(*frm).weight.as_mut_ptr().offset(0 as c_int as isize))
                .as_mut_ptr()
                .offset(1 as c_int as isize),
        );
        (*frm).weight[0 as c_int as usize][2 as c_int as usize].i_scale =
            (*rce).weight[2 as c_int as usize][0 as c_int as usize] as int32_t;
        (*frm).weight[0 as c_int as usize][2 as c_int as usize].i_denom =
            (*rce).i_weight_denom[1 as c_int as usize] as int32_t;
        (*frm).weight[0 as c_int as usize][2 as c_int as usize].i_offset =
            (*rce).weight[2 as c_int as usize][1 as c_int as usize] as int32_t;
        (*h).mc.weight_cache.expect("non-null function pointer")(
            h,
            &mut *(*(*frm).weight.as_mut_ptr().offset(0 as c_int as isize))
                .as_mut_ptr()
                .offset(2 as c_int as isize),
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "1829:1"]
unsafe extern "C" fn x264_10_ratecontrol_end(
    mut h: *mut x264_t,
    mut bits: c_int,
    mut filler: *mut c_int,
) -> c_int {
    let mut current_block: u64;
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut mbs: *const c_int = (*h).stat.frame.i_mb_count.as_mut_ptr();
    (*h).stat.frame.i_mb_count_skip =
        *mbs.offset(P_SKIP as c_int as isize) + *mbs.offset(B_SKIP as c_int as isize);
    (*h).stat.frame.i_mb_count_i = *mbs.offset(I_16x16 as c_int as isize)
        + *mbs.offset(I_8x8 as c_int as isize)
        + *mbs.offset(I_4x4 as c_int as isize)
        + *mbs.offset(I_PCM as c_int as isize);
    (*h).stat.frame.i_mb_count_p =
        *mbs.offset(P_L0 as c_int as isize) + *mbs.offset(P_8x8 as c_int as isize);
    let mut i: c_int = B_DIRECT as c_int;
    while i <= B_8x8 as c_int {
        (*h).stat.frame.i_mb_count_p += *mbs.offset(i as isize);
        i += 1;
    }
    (*rc).qpa_rc /= (*h).mb.i_mb_count as c_float;
    (*(*h).fdec).f_qp_avg_rc = (*rc).qpa_rc;
    (*(*h).fdec).f_qp_avg_aq = (*rc).qpa_aq as c_float / (*h).mb.i_mb_count as c_float;
    (*(*h).fdec).f_crf_avg =
        (*h).param.rc.f_rf_constant + (*(*h).fdec).f_qp_avg_rc - (*rc).qp_novbv;
    if (*h).param.rc.b_stat_write != 0 {
        let mut c_type: c_char = (if (*h).sh.i_type == SLICE_TYPE_I as c_int {
            if (*(*h).fenc).i_poc == 0 as c_int {
                'I' as i32
            } else {
                'i' as i32
            }
        } else if (*h).sh.i_type == SLICE_TYPE_P as c_int {
            'P' as i32
        } else if (*(*h).fenc).b_kept_as_ref != 0 {
            'B' as i32
        } else {
            'b' as i32
        }) as c_char;
        let mut dir_frame: c_int = (*h).stat.frame.i_direct_score[1 as c_int as usize]
            - (*h).stat.frame.i_direct_score[0 as c_int as usize];
        let mut dir_avg: c_int = (*h).stat.i_direct_score[1 as c_int as usize]
            - (*h).stat.i_direct_score[0 as c_int as usize];
        let mut c_direct: c_char = (if (*h).mb.b_direct_auto_write != 0 {
            if dir_frame > 0 as c_int {
                's' as i32
            } else if dir_frame < 0 as c_int {
                't' as i32
            } else if dir_avg > 0 as c_int {
                's' as i32
            } else if dir_avg < 0 as c_int {
                't' as i32
            } else {
                '-' as i32
            }
        } else {
            '-' as i32
        }) as c_char;
        if fprintf(
            (*rc).p_stat_file_out,
            b"in:%d out:%d type:%c dur:%ld cpbdur:%ld q:%.2f aq:%.2f tex:%d mv:%d misc:%d imb:%d pmb:%d smb:%d d:%c ref:\0"
                as *const u8 as *const c_char,
            (*(*h).fenc).i_frame,
            (*h).i_frame,
            c_type as c_int,
            (*(*h).fenc).i_duration,
            (*(*h).fenc).i_cpb_duration,
            (*rc).qpa_rc as c_double,
            (*(*h).fdec).f_qp_avg_aq as c_double,
            (*h).stat.frame.i_tex_bits,
            (*h).stat.frame.i_mv_bits,
            (*h).stat.frame.i_misc_bits,
            (*h).stat.frame.i_mb_count_i,
            (*h).stat.frame.i_mb_count_p,
            (*h).stat.frame.i_mb_count_skip,
            c_direct as c_int,
        ) < 0 as c_int
        {
            current_block = 13175027128378642164;
        } else {
            let mut use_old_stats: c_int = ((*h).param.rc.b_stat_read != 0
                && (*(*rc).rce).refs > 1 as c_int) as c_int;
            let mut i_0: c_int = 0 as c_int;
            loop {
                if !(i_0
                    < (if use_old_stats != 0 {
                        (*(*rc).rce).refs
                    } else {
                        (*h).i_ref[0 as c_int as usize]
                    }))
                {
                    current_block = 8236137900636309791;
                    break;
                }
                let mut refcount: c_int = if use_old_stats != 0 {
                    (*(*rc).rce).refcount[i_0 as usize]
                } else if (*h).param.b_interlaced != 0 {
                    (*h)
                        .stat
                        .frame
                        .i_mb_count_ref[0 as c_int
                        as usize][(i_0 * 2 as c_int) as usize]
                        + (*h)
                            .stat
                            .frame
                            .i_mb_count_ref[0 as c_int
                            as usize][(i_0 * 2 as c_int
                            + 1 as c_int) as usize]
                } else {
                    (*h)
                        .stat
                        .frame
                        .i_mb_count_ref[0 as c_int as usize][i_0 as usize]
                };
                if fprintf(
                    (*rc).p_stat_file_out,
                    b"%d \0" as *const u8 as *const c_char,
                    refcount,
                ) < 0 as c_int
                {
                    current_block = 13175027128378642164;
                    break;
                }
                i_0 += 1;
            }
            match current_block {
                13175027128378642164 => {}
                _ => {
                    if (*h).param.analyse.i_weighted_pred >= X264_WEIGHTP_SIMPLE
                        && !(*h)
                            .sh
                            .weight[0 as c_int
                                as usize][0 as c_int as usize]
                            .weightfn
                            .is_null()
                    {
                        if fprintf(
                            (*rc).p_stat_file_out,
                            b"w:%d,%d,%d\0" as *const u8 as *const c_char,
                            (*h)
                                .sh
                                .weight[0 as c_int
                                    as usize][0 as c_int as usize]
                                .i_denom,
                            (*h)
                                .sh
                                .weight[0 as c_int
                                    as usize][0 as c_int as usize]
                                .i_scale,
                            (*h)
                                .sh
                                .weight[0 as c_int
                                    as usize][0 as c_int as usize]
                                .i_offset,
                        ) < 0 as c_int
                        {
                            current_block = 13175027128378642164;
                        } else if !(*h)
                            .sh
                            .weight[0 as c_int
                                as usize][1 as c_int as usize]
                            .weightfn
                            .is_null()
                            || !(*h)
                                .sh
                                .weight[0 as c_int
                                    as usize][2 as c_int as usize]
                                .weightfn
                                .is_null()
                        {
                            if fprintf(
                                (*rc).p_stat_file_out,
                                b",%d,%d,%d,%d,%d \0" as *const u8
                                    as *const c_char,
                                (*h)
                                    .sh
                                    .weight[0 as c_int
                                        as usize][1 as c_int as usize]
                                    .i_denom,
                                (*h)
                                    .sh
                                    .weight[0 as c_int
                                        as usize][1 as c_int as usize]
                                    .i_scale,
                                (*h)
                                    .sh
                                    .weight[0 as c_int
                                        as usize][1 as c_int as usize]
                                    .i_offset,
                                (*h)
                                    .sh
                                    .weight[0 as c_int
                                        as usize][2 as c_int as usize]
                                    .i_scale,
                                (*h)
                                    .sh
                                    .weight[0 as c_int
                                        as usize][2 as c_int as usize]
                                    .i_offset,
                            ) < 0 as c_int
                            {
                                current_block = 13175027128378642164;
                            } else {
                                current_block = 12147880666119273379;
                            }
                        } else if fprintf(
                            (*rc).p_stat_file_out,
                            b" \0" as *const u8 as *const c_char,
                        ) < 0 as c_int
                        {
                            current_block = 13175027128378642164;
                        } else {
                            current_block = 12147880666119273379;
                        }
                    } else {
                        current_block = 12147880666119273379;
                    }
                    match current_block {
                        13175027128378642164 => {}
                        _ => {
                            if fprintf(
                                (*rc).p_stat_file_out,
                                b";\n\0" as *const u8 as *const c_char,
                            ) < 0 as c_int
                            {
                                current_block = 13175027128378642164;
                            } else if (*h).param.rc.b_mb_tree != 0
                                && (*(*h).fenc).b_kept_as_ref != 0
                                && (*h).param.rc.b_stat_read == 0
                            {
                                let mut i_type: uint8_t = (*h).sh.i_type as uint8_t;
                                (*h)
                                    .mc
                                    .mbtree_fix8_pack
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*rc).mbtree.qp_buffer[0 as c_int as usize],
                                    (*(*h).fenc).f_qp_offset,
                                    (*h).mb.i_mb_count,
                                );
                                if fwrite(
                                    &mut i_type as *mut uint8_t as *const c_void,
                                    1 as size_t,
                                    1 as size_t,
                                    (*rc).p_mbtree_stat_file_out,
                                ) < 1 as c_ulong
                                {
                                    current_block = 13175027128378642164;
                                } else if fwrite(
                                    (*rc).mbtree.qp_buffer[0 as c_int as usize]
                                        as *const c_void,
                                    ::core::mem::size_of::<uint16_t>() as size_t,
                                    (*h).mb.i_mb_count as size_t,
                                    (*rc).p_mbtree_stat_file_out,
                                )
                                    < (*h).mb.i_mb_count as c_uint
                                        as c_ulong
                                {
                                    current_block = 13175027128378642164;
                                } else {
                                    current_block = 15125582407903384992;
                                }
                            } else {
                                current_block = 15125582407903384992;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            15125582407903384992 => {}
            _ => {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"ratecontrol_end: stats file could not be written to\n\0" as *const u8
                        as *const c_char,
                );
                return -(1 as c_int);
            }
        }
    }
    if (*rc).b_abr != 0 {
        if (*h).sh.i_type != SLICE_TYPE_B as c_int {
            (*rc).cplxr_sum +=
                (bits as c_float * qp2qscale((*rc).qpa_rc)) as c_double / (*rc).last_rceq;
        } else {
            (*rc).cplxr_sum += (bits as c_float * qp2qscale((*rc).qpa_rc)) as c_double
                / ((*rc).last_rceq * (*h).param.rc.f_pb_factor as c_double);
        }
        (*rc).cplxr_sum *= (*rc).cbr_decay;
        (*rc).wanted_bits_window += (*(*h).fenc).f_duration as c_double * (*rc).bitrate;
        (*rc).wanted_bits_window *= (*rc).cbr_decay;
    }
    if (*rc).b_2pass != 0 {
        (*rc).expected_bits_sum +=
            qscale2bits((*rc).rce, qp2qscale((*(*rc).rce).new_qp) as c_double);
    }
    if (*h).mb.b_variable_qp != 0 {
        if (*h).sh.i_type == SLICE_TYPE_B as c_int {
            (*rc).bframe_bits += bits;
            if (*(*h).fenc).b_last_minigop_bframe != 0 {
                update_predictor(
                    (*rc).pred_b_from_p,
                    qp2qscale((*rc).qpa_rc),
                    (*(*h).fref[1 as c_int as usize]
                        [((*h).i_ref[1 as c_int as usize] - 1 as c_int) as usize])
                        .i_satd as c_float,
                    ((*rc).bframe_bits / (*rc).bframes) as c_float,
                );
                (*rc).bframe_bits = 0 as c_int;
            }
        }
    }
    *filler = update_vbv(h, bits);
    (*rc).filler_bits_sum += (*filler * 8 as c_int) as int64_t;
    if (*(*h).sps.as_mut_ptr()).vui.b_nal_hrd_parameters_present != 0 {
        if (*(*h).fenc).i_frame == 0 as c_int {
            (*(*h).fenc).hrd_timing.cpb_initial_arrival_time = 0 as c_int as c_double;
            (*rc).initial_cpb_removal_delay = (*h).initial_cpb_removal_delay;
            (*rc).initial_cpb_removal_delay_offset = (*h).initial_cpb_removal_delay_offset;
            (*rc).nrt_first_access_unit =
                (*rc).initial_cpb_removal_delay as c_double / 90000 as c_int as c_double;
            (*(*h).fenc).hrd_timing.cpb_removal_time = (*rc).nrt_first_access_unit;
        } else {
            (*(*h).fenc).hrd_timing.cpb_removal_time = (*rc).nrt_first_access_unit
                + ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset) as c_double
                    * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
                    / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double;
            if (*(*h).fenc).b_keyframe != 0 {
                (*rc).nrt_first_access_unit = (*(*h).fenc).hrd_timing.cpb_removal_time;
                (*rc).initial_cpb_removal_delay = (*h).initial_cpb_removal_delay;
                (*rc).initial_cpb_removal_delay_offset = (*h).initial_cpb_removal_delay_offset;
            }
            let mut cpb_earliest_arrival_time: c_double = (*(*h).fenc).hrd_timing.cpb_removal_time
                - (*rc).initial_cpb_removal_delay as c_double / 90000 as c_int as c_double;
            if (*(*h).fenc).b_keyframe == 0 {
                cpb_earliest_arrival_time -=
                    (*rc).initial_cpb_removal_delay_offset as c_double / 90000 as c_int as c_double;
            }
            if (*(*h).sps.as_mut_ptr()).vui.hrd.b_cbr_hrd != 0 {
                (*(*h).fenc).hrd_timing.cpb_initial_arrival_time =
                    (*rc).previous_cpb_final_arrival_time;
            } else {
                (*(*h).fenc).hrd_timing.cpb_initial_arrival_time =
                    if (*rc).previous_cpb_final_arrival_time > cpb_earliest_arrival_time {
                        (*rc).previous_cpb_final_arrival_time
                    } else {
                        cpb_earliest_arrival_time
                    };
            }
        }
        let mut filler_bits: c_int = if *filler != 0 {
            (if 5 as c_int + 1 as c_int - (*h).param.b_annexb > *filler {
                5 as c_int + 1 as c_int - (*h).param.b_annexb
            } else {
                *filler
            }) * 8 as c_int
        } else {
            0 as c_int
        };
        (*rc).previous_cpb_final_arrival_time = (*(*h).fenc).hrd_timing.cpb_initial_arrival_time
            + (bits + filler_bits) as c_double
                / (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled as c_double;
        (*(*h).fenc).hrd_timing.cpb_final_arrival_time = (*rc).previous_cpb_final_arrival_time;
        (*(*h).fenc).hrd_timing.dpb_output_time = (*(*h).fenc).i_dpb_output_delay as c_double
            * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double
            + (*(*h).fenc).hrd_timing.cpb_removal_time;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "2003:1"]
unsafe extern "C" fn get_qscale(
    mut h: *mut x264_t,
    mut rce: *mut ratecontrol_entry_t,
    mut rate_factor: c_double,
    mut frame_num: c_int,
) -> c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut zone: *mut x264_zone_t = get_zone(h, frame_num);
    let mut q: c_double = 0.;
    if (*h).param.rc.b_mb_tree != 0 {
        let mut timescale: c_double = (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double;
        let frame_packing = (*h).param.frame_packing;
        q = pow(
            FramePacking::base_frame_duration(frame_packing)
                / x264_clip3f(
                    (*rce).i_duration as c_double * timescale,
                    FramePacking::min_frame_duration(frame_packing),
                    FramePacking::max_frame_duration(frame_packing),
                ),
            (1 as c_int as c_float - (*h).param.rc.f_qcompress) as c_double,
        );
    } else {
        q = pow(
            (*rce).blurred_complexity as c_double,
            1 as c_int as c_double - (*rcc).qcompress,
        );
    }
    if q.is_finite() as i32 == 0 || (*rce).tex_bits + (*rce).mv_bits == 0 as c_int {
        q = (*rcc).last_qscale_for[(*rce).pict_type as usize];
    } else {
        (*rcc).last_rceq = q;
        q /= rate_factor;
        (*rcc).last_qscale = q;
    }
    if !zone.is_null() {
        if (*zone).b_force_qp != 0 {
            q = qp2qscale((*zone).i_qp as c_float) as c_double;
        } else {
            q /= (*zone).f_bitrate_factor as c_double;
        }
    }
    return q;
}
#[c2rust::src_loc = "2037:1"]
unsafe extern "C" fn get_diff_limited_q(
    mut h: *mut x264_t,
    mut rce: *mut ratecontrol_entry_t,
    mut q: c_double,
    mut frame_num: c_int,
) -> c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let pict_type: c_int = (*rce).pict_type;
    let mut zone: *mut x264_zone_t = get_zone(h, frame_num);
    if pict_type == SLICE_TYPE_I as c_int {
        let mut iq: c_double = q;
        let mut pq: c_double =
            qp2qscale(((*rcc).accum_p_qp / (*rcc).accum_p_norm) as c_float) as c_double;
        let mut ip_factor: c_double = (*h).param.rc.f_ip_factor as c_double;
        if (*rcc).accum_p_norm <= 0 as c_int as c_double {
            q = iq;
        } else if (*rcc).accum_p_norm >= 1 as c_int as c_double {
            q = pq / ip_factor;
        } else {
            q = (*rcc).accum_p_norm * pq / ip_factor
                + (1 as c_int as c_double - (*rcc).accum_p_norm) * iq;
        }
    } else if pict_type == SLICE_TYPE_B as c_int {
        q = (*rcc).last_qscale_for[(*rcc).last_non_b_pict_type as usize];
        if (*rce).kept_as_ref == 0 {
            q *= (*h).param.rc.f_pb_factor as c_double;
        }
    } else if pict_type == SLICE_TYPE_P as c_int
        && (*rcc).last_non_b_pict_type == SLICE_TYPE_P as c_int
        && (*rce).tex_bits == 0 as c_int
    {
        q = (*rcc).last_qscale_for[SLICE_TYPE_P as c_int as usize];
    }
    if (*rcc).last_non_b_pict_type == pict_type
        && (pict_type != SLICE_TYPE_I as c_int || (*rcc).last_accum_p_norm < 1 as c_int as c_double)
    {
        let mut last_q: c_double = (*rcc).last_qscale_for[pict_type as usize];
        let mut max_qscale: c_double = last_q * (*rcc).lstep;
        let mut min_qscale: c_double = last_q / (*rcc).lstep;
        if q > max_qscale {
            q = max_qscale;
        } else if q < min_qscale {
            q = min_qscale;
        }
    }
    (*rcc).last_qscale_for[pict_type as usize] = q;
    if pict_type != SLICE_TYPE_B as c_int {
        (*rcc).last_non_b_pict_type = pict_type;
    }
    if pict_type == SLICE_TYPE_I as c_int {
        (*rcc).last_accum_p_norm = (*rcc).accum_p_norm;
        (*rcc).accum_p_norm = 0 as c_int as c_double;
        (*rcc).accum_p_qp = 0 as c_int as c_double;
    }
    if pict_type == SLICE_TYPE_P as c_int {
        let mut mask: c_float = (1 as c_int as c_double
            - pow(
                ((*rce).i_count as c_float / (*rcc).nmb as c_float) as c_double,
                2 as c_int as c_double,
            )) as c_float;
        (*rcc).accum_p_qp =
            mask as c_double * (qscale2qp(q as c_float) as c_double + (*rcc).accum_p_qp);
        (*rcc).accum_p_norm = mask as c_double * (1 as c_int as c_double + (*rcc).accum_p_norm);
    }
    if !zone.is_null() {
        if (*zone).b_force_qp != 0 {
            q = qp2qscale((*zone).i_qp as c_float) as c_double;
        } else {
            q /= (*zone).f_bitrate_factor as c_double;
        }
    }
    return q;
}
#[c2rust::src_loc = "2109:1"]
unsafe extern "C" fn predict_size(
    mut p: *mut predictor_t,
    mut q: c_float,
    mut var: c_float,
) -> c_float {
    return ((*p).coeff * var + (*p).offset) / (q * (*p).count);
}
#[c2rust::src_loc = "2114:1"]
unsafe extern "C" fn update_predictor(
    mut p: *mut predictor_t,
    mut q: c_float,
    mut var: c_float,
    mut bits: c_float,
) {
    let mut range: c_float = 1.5f32;
    if var < 10 as c_int as c_float {
        return;
    }
    let mut old_coeff: c_float = (*p).coeff / (*p).count;
    let mut old_offset: c_float = (*p).offset / (*p).count;
    let mut new_coeff: c_float = if (bits * q - old_offset) / var > (*p).coeff_min {
        (bits * q - old_offset) / var
    } else {
        (*p).coeff_min
    };
    let mut new_coeff_clipped: c_float = x264_clip3f(
        new_coeff as c_double,
        (old_coeff / range) as c_double,
        (old_coeff * range) as c_double,
    ) as c_float;
    let mut new_offset: c_float = bits * q - new_coeff_clipped * var;
    if new_offset >= 0 as c_int as c_float {
        new_coeff = new_coeff_clipped;
    } else {
        new_offset = 0 as c_int as c_float;
    }
    (*p).count *= (*p).decay;
    (*p).coeff *= (*p).decay;
    (*p).offset *= (*p).decay;
    (*p).count += 1.;
    (*p).coeff += new_coeff;
    (*p).offset += new_offset;
}
#[c2rust::src_loc = "2137:1"]
unsafe extern "C" fn update_vbv(mut h: *mut x264_t, mut bits: c_int) -> c_int {
    let mut filler: c_int = 0 as c_int;
    let mut bitrate: c_int = (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled;
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut rct: *mut x264_ratecontrol_t = (*(*h).thread[0 as c_int as usize]).rc;
    let mut buffer_size: int64_t = (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled as int64_t
        * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as int64_t;
    if (*rcc).last_satd >= (*h).mb.i_mb_count {
        update_predictor(
            &mut *(*rct).pred.offset((*h).sh.i_type as isize),
            qp2qscale((*rcc).qpa_rc),
            (*rcc).last_satd as c_float,
            bits as c_float,
        );
    }
    if (*rcc).b_vbv == 0 {
        return filler;
    }
    let mut buffer_diff: uint64_t =
        (bits as uint64_t).wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t);
    (*rct).buffer_fill_final =
        ((*rct).buffer_fill_final as uint64_t).wrapping_sub(buffer_diff) as int64_t as int64_t;
    (*rct).buffer_fill_final_min =
        ((*rct).buffer_fill_final_min as uint64_t).wrapping_sub(buffer_diff) as int64_t as int64_t;
    if (*rct).buffer_fill_final_min < 0 as int64_t {
        let mut underflow: c_double = (*rct).buffer_fill_final_min as c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double;
        if (*rcc).rate_factor_max_increment != 0.
            && (*rcc).qpm >= (*rcc).qp_novbv + (*rcc).rate_factor_max_increment
        {
            x264_10_log(
                h,
                X264_LOG_DEBUG,
                b"VBV underflow due to CRF-max (frame %d, %.0f bits)\n\0" as *const u8
                    as *const c_char,
                (*h).i_frame,
                underflow,
            );
        } else {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV underflow (frame %d, %.0f bits)\n\0" as *const u8 as *const c_char,
                (*h).i_frame,
                underflow,
            );
        }
        (*rct).buffer_fill_final_min = 0 as int64_t;
        (*rct).buffer_fill_final = (*rct).buffer_fill_final_min;
    }
    if (*h).param.i_avcintra_class != 0 {
        buffer_diff = buffer_size as uint64_t;
    } else {
        buffer_diff = (bitrate as uint64_t)
            .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as uint64_t)
            .wrapping_mul((*(*h).fenc).i_cpb_duration as uint64_t);
    }
    (*rct).buffer_fill_final =
        ((*rct).buffer_fill_final as uint64_t).wrapping_add(buffer_diff) as int64_t as int64_t;
    (*rct).buffer_fill_final_min =
        ((*rct).buffer_fill_final_min as uint64_t).wrapping_add(buffer_diff) as int64_t as int64_t;
    if (*rct).buffer_fill_final > buffer_size {
        if (*h).param.rc.b_filler != 0 {
            let mut scale: int64_t =
                (*(*h).sps.as_mut_ptr()).vui.i_time_scale as int64_t * 8 as int64_t;
            filler =
                (((*rct).buffer_fill_final - buffer_size + scale - 1 as int64_t) / scale) as c_int;
            bits = if (*h).param.i_avcintra_class != 0 {
                filler * 8 as c_int
            } else {
                (if 5 as c_int + 1 as c_int - (*h).param.b_annexb > filler {
                    5 as c_int + 1 as c_int - (*h).param.b_annexb
                } else {
                    filler
                }) * 8 as c_int
            };
            buffer_diff = (bits as uint64_t)
                .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t);
            (*rct).buffer_fill_final = ((*rct).buffer_fill_final as uint64_t)
                .wrapping_sub(buffer_diff) as int64_t
                as int64_t;
            (*rct).buffer_fill_final_min = ((*rct).buffer_fill_final_min as uint64_t)
                .wrapping_sub(buffer_diff) as int64_t
                as int64_t;
        } else {
            (*rct).buffer_fill_final = if (*rct).buffer_fill_final < buffer_size {
                (*rct).buffer_fill_final
            } else {
                buffer_size
            };
            (*rct).buffer_fill_final_min = if (*rct).buffer_fill_final_min < buffer_size {
                (*rct).buffer_fill_final_min
            } else {
                buffer_size
            };
        }
    }
    return filler;
}
#[no_mangle]
#[c2rust::src_loc = "2194:1"]
unsafe extern "C" fn x264_10_hrd_fullness(mut h: *mut x264_t) {
    let mut rct: *mut x264_ratecontrol_t = (*(*h).thread[0 as c_int as usize]).rc;
    let mut denom: uint64_t = ((*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled as uint64_t)
        .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t)
        .wrapping_div((*rct).hrd_multiply_denom);
    let mut cpb_state: uint64_t = (*rct).buffer_fill_final as uint64_t;
    let mut cpb_size: uint64_t = ((*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled as uint64_t)
        .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t);
    let mut multiply_factor: uint64_t = (90000 as uint64_t).wrapping_div((*rct).hrd_multiply_denom);
    if (*rct).buffer_fill_final < 0 as int64_t || (*rct).buffer_fill_final > cpb_size as int64_t {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"CPB %s: %.0f bits in a %.0f-bit buffer\n\0" as *const u8 as *const c_char,
            if (*rct).buffer_fill_final < 0 as int64_t {
                b"underflow\0" as *const u8 as *const c_char
            } else {
                b"overflow\0" as *const u8 as *const c_char
            },
            (*rct).buffer_fill_final as c_double
                / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double,
            cpb_size as c_double / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double,
        );
    }
    (*h).initial_cpb_removal_delay =
        multiply_factor.wrapping_mul(cpb_state).wrapping_div(denom) as c_int;
    (*h).initial_cpb_removal_delay_offset = multiply_factor
        .wrapping_mul(cpb_size)
        .wrapping_div(denom)
        .wrapping_sub((*h).initial_cpb_removal_delay as uint64_t)
        as c_int;
    let mut decoder_buffer_fill: int64_t = ((*h).initial_cpb_removal_delay as uint64_t)
        .wrapping_mul(denom)
        .wrapping_div(multiply_factor) as int64_t;
    (*rct).buffer_fill_final_min = if (*rct).buffer_fill_final_min < decoder_buffer_fill {
        (*rct).buffer_fill_final_min
    } else {
        decoder_buffer_fill
    };
}
#[c2rust::src_loc = "2217:1"]
unsafe extern "C" fn update_vbv_plan(mut h: *mut x264_t, mut overhead: c_int) {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    (*rcc).buffer_fill = ((*(*(*h).thread[0 as c_int as usize]).rc).buffer_fill_final_min
        / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as int64_t)
        as c_double;
    if (*h).i_thread_frames > 1 as c_int {
        let mut j: c_int =
            rcc.offset_from((*(*h).thread[0 as c_int as usize]).rc) as c_long as c_int;
        let mut i: c_int = 1 as c_int;
        while i < (*h).i_thread_frames {
            let mut t: *mut x264_t = (*h).thread[((j + i) % (*h).i_thread_frames) as usize];
            let mut bits: c_double = (*(*t).rc).frame_size_planned;
            if !((*t).b_thread_active == 0) {
                bits = if bits > (*(*t).rc).frame_size_estimated as c_double {
                    bits
                } else {
                    (*(*t).rc).frame_size_estimated as c_double
                };
                (*rcc).buffer_fill -= bits;
                (*rcc).buffer_fill = if (*rcc).buffer_fill > 0 as c_int as c_double {
                    (*rcc).buffer_fill
                } else {
                    0 as c_int as c_double
                };
                (*rcc).buffer_fill += (*(*t).rc).buffer_rate;
                (*rcc).buffer_fill = if (*rcc).buffer_fill < (*rcc).buffer_size {
                    (*rcc).buffer_fill
                } else {
                    (*rcc).buffer_size
                };
            }
            i += 1;
        }
    }
    (*rcc).buffer_fill = if (*rcc).buffer_fill < (*rcc).buffer_size {
        (*rcc).buffer_fill
    } else {
        (*rcc).buffer_size
    };
    (*rcc).buffer_fill -= overhead as c_double;
}
#[c2rust::src_loc = "2242:1"]
unsafe extern "C" fn clip_qscale(
    mut h: *mut x264_t,
    mut pict_type: c_int,
    mut q: c_double,
) -> c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut lmin: c_double = (*rcc).lmin[pict_type as usize];
    let mut lmax: c_double = (*rcc).lmax[pict_type as usize];
    if (*rcc).rate_factor_max_increment != 0. {
        lmax = if lmax < qp2qscale((*rcc).qp_novbv + (*rcc).rate_factor_max_increment) as c_double {
            lmax
        } else {
            qp2qscale((*rcc).qp_novbv + (*rcc).rate_factor_max_increment) as c_double
        };
    }
    if lmin == lmax {
        return lmin;
    } else if (*rcc).b_2pass != 0 {
        let mut min2: c_double = log(lmin);
        let mut max2: c_double = log(lmax);
        q = (log(q) - min2) / (max2 - min2) - 0.5f64;
        q = 1.0f64 / (1.0f64 + exp(-(4 as c_int) as c_double * q));
        q = q * (max2 - min2) + min2;
        return exp(q);
    } else {
        return x264_clip3f(q, lmin, lmax);
    };
}
#[c2rust::src_loc = "2266:1"]
unsafe extern "C" fn vbv_pass1(
    mut h: *mut x264_t,
    mut pict_type: c_int,
    mut q: c_double,
) -> c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    if (*rcc).b_vbv != 0 && (*rcc).last_satd > 0 as c_int {
        let mut q0: c_double = q;
        let mut fenc_cpb_duration: c_double = (*(*h).fenc).i_cpb_duration as c_double
            * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double;
        if (*h).param.rc.i_lookahead != 0 {
            let mut terminate: c_int = 0 as c_int;
            let mut iterations: c_int = 0 as c_int;
            while iterations < 1000 as c_int && terminate != 3 as c_int {
                let mut frame_q: [c_double; 3] = [0.; 3];
                let mut cur_bits: c_double = predict_size(
                    &mut *(*rcc).pred.offset((*h).sh.i_type as isize),
                    q as c_float,
                    (*rcc).last_satd as c_float,
                ) as c_double;
                let mut buffer_fill_cur: c_double = (*rcc).buffer_fill - cur_bits;
                let mut target_fill: c_double = 0.;
                let mut total_duration: c_double = 0 as c_int as c_double;
                let mut last_duration: c_double = fenc_cpb_duration;
                frame_q[0 as c_int as usize] = if (*h).sh.i_type == SLICE_TYPE_I as c_int {
                    q * (*h).param.rc.f_ip_factor as c_double
                } else {
                    q
                };
                frame_q[1 as c_int as usize] =
                    frame_q[0 as c_int as usize] * (*h).param.rc.f_pb_factor as c_double;
                frame_q[2 as c_int as usize] =
                    frame_q[0 as c_int as usize] / (*h).param.rc.f_ip_factor as c_double;
                let mut j: c_int = 0 as c_int;
                while buffer_fill_cur >= 0 as c_int as c_double
                    && buffer_fill_cur <= (*rcc).buffer_size
                {
                    total_duration += last_duration;
                    buffer_fill_cur += (*rcc).vbv_max_rate * last_duration;
                    let mut i_type: c_int = (*(*h).fenc).i_planned_type[j as usize] as c_int;
                    let mut i_satd: c_int = (*(*h).fenc).i_planned_satd[j as usize];
                    if i_type == X264_TYPE_AUTO {
                        break;
                    }
                    i_type = if i_type == X264_TYPE_I
                        || i_type == X264_TYPE_IDR
                        || i_type == X264_TYPE_KEYFRAME
                    {
                        SLICE_TYPE_I as c_int
                    } else if i_type == X264_TYPE_B || i_type == X264_TYPE_BREF {
                        SLICE_TYPE_B as c_int
                    } else {
                        SLICE_TYPE_P as c_int
                    };
                    cur_bits = predict_size(
                        &mut *(*rcc).pred.offset(i_type as isize),
                        frame_q[i_type as usize] as c_float,
                        i_satd as c_float,
                    ) as c_double;
                    buffer_fill_cur -= cur_bits;
                    last_duration = (*(*h).fenc).f_planned_cpb_duration[j as usize];
                    j += 1;
                }
                target_fill = if (*rcc).buffer_fill + total_duration * (*rcc).vbv_max_rate * 0.5f64
                    < (*rcc).buffer_size * 0.5f64
                {
                    (*rcc).buffer_fill + total_duration * (*rcc).vbv_max_rate * 0.5f64
                } else {
                    (*rcc).buffer_size * 0.5f64
                };
                if buffer_fill_cur < target_fill {
                    q *= 1.01f64;
                    terminate |= 1 as c_int;
                } else {
                    target_fill = x264_clip3f(
                        (*rcc).buffer_fill - total_duration * (*rcc).vbv_max_rate * 0.5f64,
                        (*rcc).buffer_size * 0.8f64,
                        (*rcc).buffer_size,
                    );
                    if !((*rcc).b_vbv_min_rate != 0 && buffer_fill_cur > target_fill) {
                        break;
                    }
                    q /= 1.01f64;
                    terminate |= 2 as c_int;
                }
                iterations += 1;
            }
        } else {
            if (pict_type == SLICE_TYPE_P as c_int
                || pict_type == SLICE_TYPE_I as c_int
                    && (*rcc).last_non_b_pict_type == SLICE_TYPE_I as c_int)
                && (*rcc).buffer_fill / (*rcc).buffer_size < 0.5f64
            {
                q /= x264_clip3f(
                    2.0f64 * (*rcc).buffer_fill / (*rcc).buffer_size,
                    0.5f64,
                    1.0f64,
                );
            }
            let mut bits: c_double = predict_size(
                &mut *(*rcc).pred.offset((*h).sh.i_type as isize),
                q as c_float,
                (*rcc).last_satd as c_float,
            ) as c_double;
            let mut max_fill_factor: c_double = (if (*h).param.rc.i_vbv_buffer_size as c_double
                >= (5 as c_int * (*h).param.rc.i_vbv_max_bitrate) as c_double / (*rcc).fps
            {
                2 as c_int
            } else {
                1 as c_int
            }) as c_double;
            let mut min_fill_factor: c_double = (if (*rcc).single_frame_vbv != 0 {
                1 as c_int
            } else {
                2 as c_int
            }) as c_double;
            if bits > (*rcc).buffer_fill / max_fill_factor {
                let mut qf: c_double = x264_clip3f(
                    (*rcc).buffer_fill / (max_fill_factor * bits),
                    0.2f64,
                    1.0f64,
                );
                q /= qf;
                bits *= qf;
            }
            if bits < (*rcc).buffer_rate / min_fill_factor {
                let mut qf_0: c_double = x264_clip3f(
                    bits * min_fill_factor / (*rcc).buffer_rate,
                    0.001f64,
                    1.0f64,
                );
                q *= qf_0;
            }
            q = if q0 > q { q0 } else { q };
        }
        if (*h).sh.i_type == SLICE_TYPE_P as c_int && (*rcc).single_frame_vbv == 0 {
            let mut nb: c_int = (*rcc).bframes;
            let mut bits_0: c_double = predict_size(
                &mut *(*rcc).pred.offset((*h).sh.i_type as isize),
                q as c_float,
                (*rcc).last_satd as c_float,
            ) as c_double;
            let mut pbbits: c_double = bits_0;
            let mut bbits: c_double = predict_size(
                (*rcc).pred_b_from_p,
                (q * (*h).param.rc.f_pb_factor as c_double) as c_float,
                (*rcc).last_satd as c_float,
            ) as c_double;
            let mut space: c_double = 0.;
            let mut bframe_cpb_duration: c_double = 0 as c_int as c_double;
            let mut minigop_cpb_duration: c_double = 0.;
            let mut i: c_int = 0 as c_int;
            while i < nb {
                bframe_cpb_duration += (*(*h).fenc).f_planned_cpb_duration[i as usize];
                i += 1;
            }
            if bbits * nb as c_double > bframe_cpb_duration * (*rcc).vbv_max_rate {
                nb = 0 as c_int;
                bframe_cpb_duration = 0 as c_int as c_double;
            }
            pbbits += nb as c_double * bbits;
            minigop_cpb_duration = bframe_cpb_duration + fenc_cpb_duration;
            space = (*rcc).buffer_fill + minigop_cpb_duration * (*rcc).vbv_max_rate
                - (*rcc).buffer_size;
            if pbbits < space {
                q *= if pbbits / space > bits_0 / (0.5f64 * (*rcc).buffer_size) {
                    pbbits / space
                } else {
                    bits_0 / (0.5f64 * (*rcc).buffer_size)
                };
            }
            q = if q0 / 2 as c_int as c_double > q {
                q0 / 2 as c_int as c_double
            } else {
                q
            };
        }
        if (*rcc).b_vbv_min_rate == 0 {
            q = if q0 > q { q0 } else { q };
        }
    }
    return clip_qscale(h, pict_type, q);
}
#[c2rust::src_loc = "2400:1"]
unsafe extern "C" fn rate_estimate_qscale(mut h: *mut x264_t) -> c_float {
    let mut q: c_float = 0.;
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut rce: ratecontrol_entry_t = {
        let mut init = ratecontrol_entry_t {
            pict_type: 0 as c_int,
            frame_type: 0,
            kept_as_ref: 0,
            qscale: 0.,
            mv_bits: 0,
            tex_bits: 0,
            misc_bits: 0,
            expected_bits: 0.,
            expected_vbv: 0.,
            new_qscale: 0.,
            new_qp: 0.,
            i_count: 0,
            p_count: 0,
            s_count: 0,
            blurred_complexity: 0.,
            direct_mode: 0,
            weight: [[0; 2]; 3],
            i_weight_denom: [0; 2],
            refcount: [0; 16],
            refs: 0,
            i_duration: 0,
            i_cpb_duration: 0,
            out_num: 0,
        };
        init
    };
    let mut pict_type: c_int = (*h).sh.i_type;
    let mut total_bits: int64_t = 8 as int64_t
        * ((*h).stat.i_frame_size[SLICE_TYPE_I as c_int as usize]
            + (*h).stat.i_frame_size[SLICE_TYPE_P as c_int as usize]
            + (*h).stat.i_frame_size[SLICE_TYPE_B as c_int as usize])
        - (*rcc).filler_bits_sum;
    if (*rcc).b_2pass != 0 {
        rce = *(*rcc).rce;
        if pict_type != rce.pict_type {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"slice=%c but 2pass stats say %c\n\0" as *const u8 as *const c_char,
                slice_type_to_char[pict_type as usize] as c_int,
                slice_type_to_char[rce.pict_type as usize] as c_int,
            );
        }
    }
    if pict_type == SLICE_TYPE_B as c_int {
        let mut i0: c_int = ((*(*h).fref_nearest[0 as c_int as usize]).i_type == X264_TYPE_I
            || (*(*h).fref_nearest[0 as c_int as usize]).i_type == X264_TYPE_IDR
            || (*(*h).fref_nearest[0 as c_int as usize]).i_type == X264_TYPE_KEYFRAME)
            as c_int;
        let mut i1: c_int = ((*(*h).fref_nearest[1 as c_int as usize]).i_type == X264_TYPE_I
            || (*(*h).fref_nearest[1 as c_int as usize]).i_type == X264_TYPE_IDR
            || (*(*h).fref_nearest[1 as c_int as usize]).i_type == X264_TYPE_KEYFRAME)
            as c_int;
        let mut dt0: c_int =
            abs((*(*h).fenc).i_poc - (*(*h).fref_nearest[0 as c_int as usize]).i_poc);
        let mut dt1: c_int =
            abs((*(*h).fenc).i_poc - (*(*h).fref_nearest[1 as c_int as usize]).i_poc);
        let mut q0: c_float = (*(*h).fref_nearest[0 as c_int as usize]).f_qp_avg_rc;
        let mut q1: c_float = (*(*h).fref_nearest[1 as c_int as usize]).f_qp_avg_rc;
        if (*(*h).fref_nearest[0 as c_int as usize]).i_type == X264_TYPE_BREF {
            q0 = (q0 as c_double - (*rcc).pb_offset / 2 as c_int as c_double) as c_float;
        }
        if (*(*h).fref_nearest[1 as c_int as usize]).i_type == X264_TYPE_BREF {
            q1 = (q1 as c_double - (*rcc).pb_offset / 2 as c_int as c_double) as c_float;
        }
        if i0 != 0 && i1 != 0 {
            q = (((q0 + q1) / 2 as c_int as c_float) as c_double + (*rcc).ip_offset) as c_float;
        } else if i0 != 0 {
            q = q1;
        } else if i1 != 0 {
            q = q0;
        } else {
            q = (q0 * dt1 as c_float + q1 * dt0 as c_float) / (dt0 + dt1) as c_float;
        }
        if (*(*h).fenc).b_kept_as_ref != 0 {
            q = (q as c_double + (*rcc).pb_offset / 2 as c_int as c_double) as c_float;
        } else {
            q = (q as c_double + (*rcc).pb_offset) as c_float;
        }
        (*rcc).qp_novbv = q;
        q = qp2qscale(q);
        if (*rcc).b_2pass != 0 {
            (*rcc).frame_size_planned = qscale2bits(&mut rce, q as c_double);
        } else {
            (*rcc).frame_size_planned = predict_size(
                (*rcc).pred_b_from_p,
                q,
                (*(*h).fref[1 as c_int as usize]
                    [((*h).i_ref[1 as c_int as usize] - 1 as c_int) as usize])
                    .i_satd as c_float,
            ) as c_double;
        }
        if (*rcc).b_vbv != 0 {
            let mut frame_size_maximum: c_double = if (*rcc).frame_size_maximum
                < (if (*rcc).buffer_fill > 0.001f64 {
                    (*rcc).buffer_fill
                } else {
                    0.001f64
                }) {
                (*rcc).frame_size_maximum
            } else if (*rcc).buffer_fill > 0.001f64 {
                (*rcc).buffer_fill
            } else {
                0.001f64
            };
            if (*rcc).frame_size_planned > frame_size_maximum {
                q = (q as c_double * ((*rcc).frame_size_planned / frame_size_maximum)) as c_float;
                (*rcc).frame_size_planned = frame_size_maximum;
            }
        }
        ::core::ptr::write_volatile(
            &mut (*rcc).frame_size_estimated as *mut c_float,
            (*rcc).frame_size_planned as c_float,
        );
        if (*rcc).b_vbv != 0 {
            (*rcc).last_satd = x264_10_rc_analyse_slice(h);
        }
        return q;
    } else {
        let mut abr_buffer: c_double =
            2 as c_int as c_double * (*rcc).rate_tolerance * (*rcc).bitrate;
        let mut predicted_bits: c_double = total_bits as c_double;
        if (*h).i_thread_frames > 1 as c_int {
            let mut j: c_int =
                rcc.offset_from((*(*h).thread[0 as c_int as usize]).rc) as c_long as c_int;
            let mut i: c_int = 1 as c_int;
            while i < (*h).i_thread_frames {
                let mut t: *mut x264_t = (*h).thread[((j + i) % (*h).i_thread_frames) as usize];
                let mut bits: c_double = (*(*t).rc).frame_size_planned;
                if !((*t).b_thread_active == 0) {
                    bits = if bits > (*(*t).rc).frame_size_estimated as c_double {
                        bits
                    } else {
                        (*(*t).rc).frame_size_estimated as c_double
                    };
                    predicted_bits += bits;
                }
                i += 1;
            }
        }
        if (*rcc).b_2pass != 0 {
            let mut lmin: c_double = (*rcc).lmin[pict_type as usize];
            let mut lmax: c_double = (*rcc).lmax[pict_type as usize];
            let mut diff: c_double = 0.;
            if (*rcc).num_entries > (*h).i_frame {
                let mut final_bits: c_double = (**(*rcc)
                    .entry_out
                    .offset(((*rcc).num_entries - 1 as c_int) as isize))
                .expected_bits;
                let mut video_pos: c_double = rce.expected_bits / final_bits;
                let mut scale_factor: c_double =
                    sqrt((1 as c_int as c_double - video_pos) * (*rcc).num_entries as c_double);
                abr_buffer *= 0.5f64
                    * (if scale_factor > 0.5f64 {
                        scale_factor
                    } else {
                        0.5f64
                    });
            }
            diff = predicted_bits - rce.expected_bits;
            q = rce.new_qscale as c_float;
            q = (q as c_double
                / x264_clip3f(
                    (abr_buffer - diff) / abr_buffer,
                    0.5f64,
                    2 as c_int as c_double,
                )) as c_float;
            if (*h).i_frame as c_double >= (*rcc).fps
                && (*rcc).expected_bits_sum >= 1 as c_int as c_double
            {
                let mut cur_time: c_double =
                    (*h).i_frame as c_double / (*rcc).num_entries as c_double;
                let mut w: c_double =
                    x264_clip3f(cur_time * 100 as c_int as c_double, 0.0f64, 1.0f64);
                q = (q as c_double * pow(total_bits as c_double / (*rcc).expected_bits_sum, w))
                    as c_float;
            }
            (*rcc).qp_novbv = qscale2qp(q);
            if (*rcc).b_vbv != 0 {
                let mut expected_size: c_double = qscale2bits(&mut rce, q as c_double);
                let mut expected_vbv: c_double =
                    (*rcc).buffer_fill + (*rcc).buffer_rate - expected_size;
                let mut expected_fullness: c_double = rce.expected_vbv / (*rcc).buffer_size;
                let mut qmax: c_double =
                    q as c_double * (2 as c_int as c_double - expected_fullness);
                let mut size_constraint: c_double = 1 as c_int as c_double + expected_fullness;
                qmax = if qmax > rce.new_qscale {
                    qmax
                } else {
                    rce.new_qscale
                };
                if expected_fullness < 0.05f64 {
                    qmax = lmax;
                }
                qmax = if qmax < lmax { qmax } else { lmax };
                while expected_vbv < rce.expected_vbv / size_constraint && (q as c_double) < qmax
                    || expected_vbv < 0 as c_int as c_double && (q as c_double) < lmax
                {
                    q = (q as c_double * 1.05f64) as c_float;
                    expected_size = qscale2bits(&mut rce, q as c_double);
                    expected_vbv = (*rcc).buffer_fill + (*rcc).buffer_rate - expected_size;
                }
                (*rcc).last_satd = x264_10_rc_analyse_slice(h);
            }
            q = x264_clip3f(q as c_double, lmin, lmax) as c_float;
        } else {
            let mut wanted_bits: c_double = 0.;
            let mut overflow: c_double = 1 as c_int as c_double;
            let frame_packing = (*h).param.frame_packing;
            (*rcc).last_satd = x264_10_rc_analyse_slice(h);
            (*rcc).short_term_cplxsum *= 0.5f64;
            (*rcc).short_term_cplxcount *= 0.5f64;
            (*rcc).short_term_cplxsum += (*rcc).last_satd as c_double
                / (x264_clip3f(
                    (*(*h).fenc).f_duration as c_double,
                    FramePacking::min_frame_duration(frame_packing),
                    FramePacking::max_frame_duration(frame_packing),
                ) / FramePacking::base_frame_duration(frame_packing));
            (*rcc).short_term_cplxcount += 1.;
            rce.tex_bits = (*rcc).last_satd;
            rce.blurred_complexity =
                ((*rcc).short_term_cplxsum / (*rcc).short_term_cplxcount) as c_float;
            rce.mv_bits = 0 as c_int;
            rce.p_count = (*rcc).nmb;
            rce.i_count = 0 as c_int;
            rce.s_count = 0 as c_int;
            rce.qscale = 1 as c_int as c_double;
            rce.pict_type = pict_type;
            rce.i_duration = (*(*h).fenc).i_duration;
            if (*h).param.rc.i_rc_method == X264_RC_CRF {
                q = get_qscale(
                    h,
                    &mut rce,
                    (*rcc).rate_factor_constant,
                    (*(*h).fenc).i_frame,
                ) as c_float;
            } else {
                q = get_qscale(
                    h,
                    &mut rce,
                    (*rcc).wanted_bits_window / (*rcc).cplxr_sum,
                    (*(*h).fenc).i_frame,
                ) as c_float;
                if (*rcc).b_vbv_min_rate == 0 && (*rcc).last_satd != 0 {
                    let mut i_frame_done: c_int = (*h).i_frame;
                    let mut time_done: c_double = i_frame_done as c_double / (*rcc).fps;
                    if (*h).param.b_vfr_input != 0 && i_frame_done > 0 as c_int {
                        time_done = ((*(*h).fenc).i_reordered_pts - (*h).i_reordered_pts_delay)
                            as c_double
                            * (*h).param.i_timebase_num as c_double
                            / (*h).param.i_timebase_den as c_double;
                    }
                    wanted_bits = time_done * (*rcc).bitrate;
                    if wanted_bits > 0 as c_int as c_double {
                        abr_buffer *= if 1 as c_int as c_double > sqrt(time_done) {
                            1 as c_int as c_double
                        } else {
                            sqrt(time_done)
                        };
                        overflow = x264_clip3f(
                            1.0f64 + (predicted_bits - wanted_bits) / abr_buffer,
                            0.5f64,
                            2 as c_int as c_double,
                        );
                        q = (q as c_double * overflow) as c_float;
                    }
                }
            }
            if pict_type == SLICE_TYPE_I as c_int
                && (*h).param.i_keyint_max > 1 as c_int
                && (*rcc).last_non_b_pict_type != SLICE_TYPE_I as c_int
            {
                q = qp2qscale(((*rcc).accum_p_qp / (*rcc).accum_p_norm) as c_float);
                q /= (*h).param.rc.f_ip_factor;
            } else if (*h).i_frame > 0 as c_int {
                if (*h).param.rc.i_rc_method != X264_RC_CRF {
                    let mut lmin_0: c_double =
                        (*rcc).last_qscale_for[pict_type as usize] / (*rcc).lstep;
                    let mut lmax_0: c_double =
                        (*rcc).last_qscale_for[pict_type as usize] * (*rcc).lstep;
                    if overflow > 1.1f64 && (*h).i_frame > 3 as c_int {
                        lmax_0 *= (*rcc).lstep;
                    } else if overflow < 0.9f64 {
                        lmin_0 /= (*rcc).lstep;
                    }
                    q = x264_clip3f(q as c_double, lmin_0, lmax_0) as c_float;
                }
            } else if (*h).param.rc.i_rc_method == X264_RC_CRF
                && (*rcc).qcompress != 1 as c_int as c_double
            {
                q = qp2qscale(
                    (if (*h).param.rc.i_rc_method == X264_RC_CRF {
                        (*h).param.rc.f_rf_constant
                    } else {
                        24 as c_int as c_float
                    }) + QP_BD_OFFSET as c_float,
                ) / (*h).param.rc.f_ip_factor;
            }
            (*rcc).qp_novbv = qscale2qp(q);
            q = vbv_pass1(h, pict_type, q as c_double) as c_float;
        }
        (*rcc).last_qscale = q as c_double;
        (*rcc).last_qscale_for[pict_type as usize] = (*rcc).last_qscale;
        if !((*rcc).b_2pass != 0 && (*rcc).b_vbv == 0) && (*(*h).fenc).i_frame == 0 as c_int {
            (*rcc).last_qscale_for[SLICE_TYPE_P as c_int as usize] =
                (q * (*h).param.rc.f_ip_factor) as c_double;
        }
        if (*rcc).b_2pass != 0 {
            (*rcc).frame_size_planned = qscale2bits(&mut rce, q as c_double);
        } else {
            (*rcc).frame_size_planned = predict_size(
                &mut *(*rcc).pred.offset((*h).sh.i_type as isize),
                q,
                (*rcc).last_satd as c_float,
            ) as c_double;
        }
        if (*rcc).b_vbv != 0 {
            let mut frame_size_maximum_0: c_double = if (*rcc).frame_size_maximum
                < (if (*rcc).buffer_fill > 0.001f64 {
                    (*rcc).buffer_fill
                } else {
                    0.001f64
                }) {
                (*rcc).frame_size_maximum
            } else if (*rcc).buffer_fill > 0.001f64 {
                (*rcc).buffer_fill
            } else {
                0.001f64
            };
            if (*rcc).frame_size_planned > frame_size_maximum_0 {
                q = (q as c_double * ((*rcc).frame_size_planned / frame_size_maximum_0)) as c_float;
                (*rcc).frame_size_planned = frame_size_maximum_0;
            }
            if (*rcc).single_frame_vbv != 0 {
                (*rcc).frame_size_planned = if (*rcc).buffer_rate < frame_size_maximum_0 {
                    (*rcc).buffer_rate
                } else {
                    frame_size_maximum_0
                };
            }
        }
        ::core::ptr::write_volatile(
            &mut (*rcc).frame_size_estimated as *mut c_float,
            (*rcc).frame_size_planned as c_float,
        );
        return q;
    };
}
#[c2rust::src_loc = "2665:1"]
unsafe extern "C" fn threads_normalize_predictors(mut h: *mut x264_t) {
    let mut totalsize: c_double = 0 as c_int as c_double;
    let mut i: c_int = 0 as c_int;
    while i < (*h).param.i_threads {
        totalsize += (*(*(*h).thread[i as usize]).rc).slice_size_planned;
        i += 1;
    }
    let mut factor: c_double = (*(*h).rc).frame_size_planned / totalsize;
    let mut i_0: c_int = 0 as c_int;
    while i_0 < (*h).param.i_threads {
        (*(*(*h).thread[i_0 as usize]).rc).slice_size_planned *= factor;
        i_0 += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "2675:1"]
unsafe extern "C" fn x264_10_threads_distribute_ratecontrol(mut h: *mut x264_t) {
    let mut row: c_int = 0;
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut qscale: c_float = qp2qscale((*rc).qpm);
    if (*h).i_frame == 0 as c_int {
        let mut i: c_int = 0 as c_int;
        while i < (*h).param.i_threads {
            let mut t: *mut x264_t = (*h).thread[i as usize];
            if t != h {
                memcpy(
                    (*(*t).rc).row_preds.as_mut_ptr() as *mut c_void,
                    (*rc).row_preds.as_mut_ptr() as *const c_void,
                    ::core::mem::size_of::<[[predictor_t; 2]; 3]>() as size_t,
                );
            }
            i += 1;
        }
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < (*h).param.i_threads {
        let mut t_0: *mut x264_t = (*h).thread[i_0 as usize];
        if t_0 != h {
            memcpy((*t_0).rc as *mut c_void, rc as *const c_void, 576 as size_t);
        }
        (*(*t_0).rc).row_pred = (*(*(*t_0).rc)
            .row_preds
            .as_mut_ptr()
            .offset((*h).sh.i_type as isize))
        .as_mut_ptr();
        if (*rc).b_vbv != 0 && (*rc).frame_size_planned != 0. {
            let mut size: c_int = 0 as c_int;
            row = (*t_0).i_threadslice_start;
            while row < (*t_0).i_threadslice_end {
                size += *(*(*h).fdec).i_row_satd.offset(row as isize);
                row += 1;
            }
            (*(*t_0).rc).slice_size_planned = predict_size(
                &mut *(*rc)
                    .pred
                    .offset(((*h).sh.i_type + (i_0 + 1 as c_int) * 5 as c_int) as isize),
                qscale,
                size as c_float,
            ) as c_double;
        } else {
            (*(*t_0).rc).slice_size_planned = 0 as c_int as c_double;
        }
        i_0 += 1;
    }
    if (*rc).b_vbv != 0 && (*rc).frame_size_planned != 0. {
        threads_normalize_predictors(h);
        if (*rc).single_frame_vbv != 0 {
            let mut i_1: c_int = 0 as c_int;
            while i_1 < (*h).param.i_threads {
                let mut t_1: *mut x264_t = (*h).thread[i_1 as usize];
                let mut max_frame_error: c_float = x264_clip3f(
                    1.0f64 / ((*t_1).i_threadslice_end - (*t_1).i_threadslice_start) as c_double,
                    0.05f64,
                    0.25f64,
                ) as c_float;
                (*(*t_1).rc).slice_size_planned += (2 as c_int as c_float * max_frame_error)
                    as c_double
                    * (*rc).frame_size_planned;
                i_1 += 1;
            }
            threads_normalize_predictors(h);
        }
        let mut i_2: c_int = 0 as c_int;
        while i_2 < (*h).param.i_threads {
            ::core::ptr::write_volatile(
                &mut (*(*(*h).thread[i_2 as usize]).rc).frame_size_estimated as *mut c_float,
                (*(*(*h).thread[i_2 as usize]).rc).slice_size_planned as c_float,
            );
            i_2 += 1;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "2729:1"]
unsafe extern "C" fn x264_10_threads_merge_ratecontrol(mut h: *mut x264_t) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut i: c_int = 0 as c_int;
    while i < (*h).param.i_threads {
        let mut t: *mut x264_t = (*h).thread[i as usize];
        let mut rct: *mut x264_ratecontrol_t = (*(*h).thread[i as usize]).rc;
        if (*h).param.rc.i_vbv_buffer_size != 0 {
            let mut size: c_int = 0 as c_int;
            let mut row: c_int = (*t).i_threadslice_start;
            while row < (*t).i_threadslice_end {
                size += *(*(*h).fdec).i_row_satd.offset(row as isize);
                row += 1;
            }
            let mut bits: c_int = (*t).stat.frame.i_mv_bits
                + (*t).stat.frame.i_tex_bits
                + (*t).stat.frame.i_misc_bits;
            let mut mb_count: c_int =
                ((*t).i_threadslice_end - (*t).i_threadslice_start) * (*h).mb.i_mb_width;
            update_predictor(
                &mut *(*rc)
                    .pred
                    .offset(((*h).sh.i_type + (i + 1 as c_int) * 5 as c_int) as isize),
                qp2qscale((*rct).qpa_rc / mb_count as c_float),
                size as c_float,
                bits as c_float,
            );
        }
        if !(i == 0) {
            (*rc).qpa_rc += (*rct).qpa_rc;
            (*rc).qpa_aq += (*rct).qpa_aq;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "2754:1"]
unsafe extern "C" fn x264_10_thread_sync_ratecontrol(
    mut cur: *mut x264_t,
    mut prev: *mut x264_t,
    mut next: *mut x264_t,
) {
    if cur != prev {
        memcpy(
            &mut (*(*cur).rc).accum_p_qp as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).accum_p_qp as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).accum_p_norm as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).accum_p_norm as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).last_satd as *mut c_int as *mut c_void,
            &mut (*(*prev).rc).last_satd as *mut c_int as *const c_void,
            ::core::mem::size_of::<c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).last_rceq as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).last_rceq as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).last_qscale_for as *mut [c_double; 3] as *mut c_void,
            &mut (*(*prev).rc).last_qscale_for as *mut [c_double; 3] as *const c_void,
            ::core::mem::size_of::<[c_double; 3]>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).last_non_b_pict_type as *mut c_int as *mut c_void,
            &mut (*(*prev).rc).last_non_b_pict_type as *mut c_int as *const c_void,
            ::core::mem::size_of::<c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).short_term_cplxsum as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).short_term_cplxsum as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).short_term_cplxcount as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).short_term_cplxcount as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).bframes as *mut c_int as *mut c_void,
            &mut (*(*prev).rc).bframes as *mut c_int as *const c_void,
            ::core::mem::size_of::<c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).prev_zone as *mut *mut x264_zone_t as *mut c_void,
            &mut (*(*prev).rc).prev_zone as *mut *mut x264_zone_t as *const c_void,
            ::core::mem::size_of::<*mut x264_zone_t>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).mbtree.qpbuf_pos as *mut c_int as *mut c_void,
            &mut (*(*prev).rc).mbtree.qpbuf_pos as *mut c_int as *const c_void,
            ::core::mem::size_of::<c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).bitrate as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).bitrate as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).buffer_size as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).buffer_size as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).buffer_rate as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).buffer_rate as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).vbv_max_rate as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).vbv_max_rate as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).single_frame_vbv as *mut c_int as *mut c_void,
            &mut (*(*prev).rc).single_frame_vbv as *mut c_int as *const c_void,
            ::core::mem::size_of::<c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).cbr_decay as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).cbr_decay as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).rate_factor_constant as *mut c_double as *mut c_void,
            &mut (*(*prev).rc).rate_factor_constant as *mut c_double as *const c_void,
            ::core::mem::size_of::<c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).rate_factor_max_increment as *mut c_float as *mut c_void,
            &mut (*(*prev).rc).rate_factor_max_increment as *mut c_float as *const c_void,
            ::core::mem::size_of::<c_float>() as size_t,
        );
    }
    if cur != next {
        (*(*next).rc).cplxr_sum = (*(*cur).rc).cplxr_sum;
        (*(*next).rc).expected_bits_sum = (*(*cur).rc).expected_bits_sum;
        (*(*next).rc).filler_bits_sum = (*(*cur).rc).filler_bits_sum;
        (*(*next).rc).wanted_bits_window = (*(*cur).rc).wanted_bits_window;
        (*(*next).rc).bframe_bits = (*(*cur).rc).bframe_bits;
        (*(*next).rc).initial_cpb_removal_delay = (*(*cur).rc).initial_cpb_removal_delay;
        (*(*next).rc).initial_cpb_removal_delay_offset =
            (*(*cur).rc).initial_cpb_removal_delay_offset;
        (*(*next).rc).nrt_first_access_unit = (*(*cur).rc).nrt_first_access_unit;
        (*(*next).rc).previous_cpb_final_arrival_time =
            (*(*cur).rc).previous_cpb_final_arrival_time;
    }
}
#[c2rust::src_loc = "2805:1"]
unsafe extern "C" fn find_underflow(
    mut h: *mut x264_t,
    mut fills: *mut c_double,
    mut t0: *mut c_int,
    mut t1: *mut c_int,
    mut over: c_int,
) -> c_int {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let buffer_min: c_double = 0.1f64 * (*rcc).buffer_size;
    let buffer_max: c_double = 0.9f64 * (*rcc).buffer_size;
    let mut fill: c_double = *fills.offset((*t0 - 1 as c_int) as isize);
    let mut parity: c_double = if over != 0 { 1.0f64 } else { -1.0f64 };
    let mut start: c_int = -(1 as c_int);
    let mut end: c_int = -(1 as c_int);
    let mut i: c_int = *t0;
    while i < (*rcc).num_entries {
        fill += ((**(*rcc).entry_out.offset(i as isize)).i_cpb_duration as c_double
            * (*rcc).vbv_max_rate
            * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double
            - qscale2bits(
                *(*rcc).entry_out.offset(i as isize),
                (**(*rcc).entry_out.offset(i as isize)).new_qscale,
            ))
            * parity;
        fill = x264_clip3f(fill, 0 as c_int as c_double, (*rcc).buffer_size);
        *fills.offset(i as isize) = fill;
        if fill <= buffer_min || i == 0 as c_int {
            if end >= 0 as c_int {
                break;
            }
            start = i;
        } else if fill >= buffer_max && start >= 0 as c_int {
            end = i;
        }
        i += 1;
    }
    *t0 = start;
    *t1 = end;
    return (start >= 0 as c_int && end >= 0 as c_int) as c_int;
}
#[c2rust::src_loc = "2836:1"]
unsafe extern "C" fn fix_underflow(
    mut h: *mut x264_t,
    mut t0: c_int,
    mut t1: c_int,
    mut adjustment: c_double,
    mut qscale_min: c_double,
    mut qscale_max: c_double,
) -> c_int {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut qscale_orig: c_double = 0.;
    let mut qscale_new: c_double = 0.;
    let mut adjusted: c_int = 0 as c_int;
    if t0 > 0 as c_int {
        t0 += 1;
    }
    let mut i: c_int = t0;
    while i <= t1 {
        qscale_orig = (**(*rcc).entry_out.offset(i as isize)).new_qscale;
        qscale_orig = x264_clip3f(qscale_orig, qscale_min, qscale_max);
        qscale_new = qscale_orig * adjustment;
        qscale_new = x264_clip3f(qscale_new, qscale_min, qscale_max);
        (**(*rcc).entry_out.offset(i as isize)).new_qscale = qscale_new;
        adjusted = (adjusted != 0 || qscale_new != qscale_orig) as c_int;
        i += 1;
    }
    return adjusted;
}
#[c2rust::src_loc = "2855:1"]
unsafe extern "C" fn count_expected_bits(mut h: *mut x264_t) -> c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut expected_bits: c_double = 0 as c_int as c_double;
    let mut i: c_int = 0 as c_int;
    while i < (*rcc).num_entries {
        let mut rce: *mut ratecontrol_entry_t = *(*rcc).entry_out.offset(i as isize);
        (*rce).expected_bits = expected_bits;
        expected_bits += qscale2bits(rce, (*rce).new_qscale);
        i += 1;
    }
    return expected_bits;
}
#[c2rust::src_loc = "2868:1"]
unsafe extern "C" fn vbv_pass2(mut h: *mut x264_t, mut all_available_bits: c_double) -> c_int {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut fills: *mut c_double = 0 as *mut c_double;
    let mut expected_bits: c_double = 0 as c_int as c_double;
    let mut adjustment: c_double = 0.;
    let mut prev_bits: c_double = 0 as c_int as c_double;
    let mut t0: c_int = 0;
    let mut t1: c_int = 0;
    let mut qscale_min: c_double = qp2qscale((*h).param.rc.i_qp_min as c_float) as c_double;
    let mut qscale_max: c_double = qp2qscale((*h).param.rc.i_qp_max as c_float) as c_double;
    // TODO: scoping weirdness
    let mut _iterations: c_int = 0 as c_int;
    let mut adj_min: c_int = 0;
    let mut adj_max: c_int = 0;
    fills = x264_malloc(
        (((*rcc).num_entries + 1 as c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<c_double>() as usize) as int64_t,
    ) as *mut c_double;
    if fills.is_null() {
        return -(1 as c_int);
    } else {
        fills = fills.offset(1);
        loop {
            _iterations += 1;
            prev_bits = expected_bits;
            if expected_bits != 0. {
                adjustment = if (if expected_bits / all_available_bits < 0.999f64 {
                    expected_bits / all_available_bits
                } else {
                    0.999f64
                }) > 0.9f64
                {
                    if expected_bits / all_available_bits < 0.999f64 {
                        expected_bits / all_available_bits
                    } else {
                        0.999f64
                    }
                } else {
                    0.9f64
                };
                *fills.offset(-(1 as c_int) as isize) =
                    (*rcc).buffer_size * (*h).param.rc.f_vbv_buffer_init as c_double;
                t0 = 0 as c_int;
                adj_min = 1 as c_int;
                while adj_min != 0 && find_underflow(h, fills, &mut t0, &mut t1, 1 as c_int) != 0 {
                    adj_min = fix_underflow(h, t0, t1, adjustment, qscale_min, qscale_max);
                    t0 = t1;
                }
            }
            *fills.offset(-(1 as c_int) as isize) =
                (*rcc).buffer_size * (1.0f64 - (*h).param.rc.f_vbv_buffer_init as c_double);
            t0 = 0 as c_int;
            adj_max = 1 as c_int;
            while adj_max != 0 && find_underflow(h, fills, &mut t0, &mut t1, 0 as c_int) != 0 {
                adj_max = fix_underflow(h, t0, t1, 1.001f64, qscale_min, qscale_max);
            }
            expected_bits = count_expected_bits(h);
            if !(expected_bits < 0.995f64 * all_available_bits
                && (expected_bits + 0.5f64) as int64_t > (prev_bits + 0.5f64) as int64_t)
            {
                break;
            }
        }
        if adj_max == 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"vbv-maxrate issue, qpmax or vbv-maxrate too low\n\0" as *const u8
                    as *const c_char,
            );
        }
        let mut i: c_int = 0 as c_int;
        while i < (*rcc).num_entries {
            (**(*rcc).entry_out.offset(i as isize)).expected_vbv =
                (*rcc).buffer_size - *fills.offset(i as isize);
            i += 1;
        }
        x264_free(fills.offset(-(1 as c_int as isize)) as *mut c_void);
        return 0 as c_int;
    };
}
#[c2rust::src_loc = "2932:1"]
unsafe extern "C" fn init_pass2(mut h: *mut x264_t) -> c_int {
    let mut current_block: u64;
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut all_const_bits: uint64_t = 0 as uint64_t;
    let mut timescale: c_double = (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as c_double
        / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as c_double;
    let mut duration: c_double = 0 as c_int as c_double;
    let mut i: c_int = 0 as c_int;
    while i < (*rcc).num_entries {
        duration += (*(*rcc).entry.offset(i as isize)).i_duration as c_double;
        i += 1;
    }
    duration *= timescale;
    let mut all_available_bits: uint64_t =
        ((*h).param.rc.i_bitrate as c_double * 1000.0f64 * duration) as uint64_t;
    let mut rate_factor: c_double = 0.;
    let mut step_mult: c_double = 0.;
    let mut qblur: c_double = (*h).param.rc.f_qblur as c_double;
    let mut cplxblur: c_double = (*h).param.rc.f_complexity_blur as c_double;
    let filter_size: c_int = (qblur * 4 as c_int as c_double) as c_int | 1 as c_int;
    let mut expected_bits: c_double = 0.;
    let mut qscale: *mut c_double = 0 as *mut c_double;
    let mut blurred_qscale: *mut c_double = 0 as *mut c_double;
    let mut base_cplx: c_double = ((*h).mb.i_mb_count
        * (if (*h).param.i_bframe != 0 {
            120 as c_int
        } else {
            80 as c_int
        })) as c_double;
    let mut i_0: c_int = 0 as c_int;
    while i_0 < (*rcc).num_entries {
        let mut rce: *mut ratecontrol_entry_t =
            &mut *(*rcc).entry.offset(i_0 as isize) as *mut ratecontrol_entry_t;
        all_const_bits = all_const_bits.wrapping_add((*rce).misc_bits as uint64_t);
        i_0 += 1;
    }
    if all_available_bits < all_const_bits {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"requested bitrate is too low. estimated minimum is %d kbps\n\0" as *const u8
                as *const c_char,
            (all_const_bits as c_double * (*rcc).fps / ((*rcc).num_entries as c_double * 1000.0f64))
                as c_int,
        );
        return -(1 as c_int);
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < (*rcc).num_entries {
        let mut rce_0: *mut ratecontrol_entry_t =
            &mut *(*rcc).entry.offset(i_1 as isize) as *mut ratecontrol_entry_t;
        let mut weight_sum: c_double = 0 as c_int as c_double;
        let mut cplx_sum: c_double = 0 as c_int as c_double;
        let mut weight: c_double = 1.0f64;
        let mut gaussian_weight: c_double = 0.;
        let mut j: c_int = 1 as c_int;
        while (j as c_double) < cplxblur * 2 as c_int as c_double && j < (*rcc).num_entries - i_1 {
            let mut rcj: *mut ratecontrol_entry_t =
                &mut *(*rcc).entry.offset((i_1 + j) as isize) as *mut ratecontrol_entry_t;
            let frame_packing = (*h).param.frame_packing;
            let mut frame_duration: c_double = x264_clip3f(
                (*rcj).i_duration as c_double * timescale,
                FramePacking::min_frame_duration(frame_packing),
                FramePacking::max_frame_duration(frame_packing),
            ) / FramePacking::base_frame_duration(frame_packing);
            weight *= 1 as c_int as c_double
                - pow(
                    ((*rcj).i_count as c_float / (*rcc).nmb as c_float) as c_double,
                    2 as c_int as c_double,
                );
            if weight < 0.0001f64 {
                break;
            }
            gaussian_weight = weight * exp((-j * j) as c_double / 200.0f64);
            weight_sum += gaussian_weight;
            cplx_sum += gaussian_weight
                * (qscale2bits(rcj, 1 as c_int as c_double) - (*rcj).misc_bits as c_double)
                / frame_duration;
            j += 1;
        }
        weight = 1.0f64;
        let mut j_0: c_int = 0 as c_int;
        while j_0 as c_double <= cplxblur * 2 as c_int as c_double && j_0 <= i_1 {
            let mut rcj_0: *mut ratecontrol_entry_t =
                &mut *(*rcc).entry.offset((i_1 - j_0) as isize) as *mut ratecontrol_entry_t;
            let frame_packing = (*h).param.frame_packing;
            let mut frame_duration_0: c_double = x264_clip3f(
                (*rcj_0).i_duration as c_double * timescale,
                FramePacking::min_frame_duration(frame_packing),
                FramePacking::max_frame_duration(frame_packing),
            ) / FramePacking::base_frame_duration(frame_packing);
            gaussian_weight = weight * exp((-j_0 * j_0) as c_double / 200.0f64);
            weight_sum += gaussian_weight;
            cplx_sum += gaussian_weight
                * (qscale2bits(rcj_0, 1 as c_int as c_double) - (*rcj_0).misc_bits as c_double)
                / frame_duration_0;
            weight *= 1 as c_int as c_double
                - pow(
                    ((*rcj_0).i_count as c_float / (*rcc).nmb as c_float) as c_double,
                    2 as c_int as c_double,
                );
            if weight < 0.0001f64 {
                break;
            }
            j_0 += 1;
        }
        (*rce_0).blurred_complexity = (cplx_sum / weight_sum) as c_float;
        i_1 += 1;
    }
    qscale = x264_malloc(
        (::core::mem::size_of::<c_double>() as usize).wrapping_mul((*rcc).num_entries as usize)
            as int64_t,
    ) as *mut c_double;
    if !qscale.is_null() {
        if filter_size > 1 as c_int {
            blurred_qscale = x264_malloc(
                (::core::mem::size_of::<c_double>() as usize)
                    .wrapping_mul((*rcc).num_entries as usize) as int64_t,
            ) as *mut c_double;
            if blurred_qscale.is_null() {
                current_block = 8692108595190266206;
            } else {
                current_block = 12199444798915819164;
            }
        } else {
            blurred_qscale = qscale;
            current_block = 12199444798915819164;
        }
        match current_block {
            8692108595190266206 => {}
            _ => {
                expected_bits = 1 as c_int as c_double;
                let mut i_2: c_int = 0 as c_int;
                while i_2 < (*rcc).num_entries {
                    let mut q: c_double =
                        get_qscale(h, &mut *(*rcc).entry.offset(i_2 as isize), 1.0f64, i_2);
                    expected_bits += qscale2bits(&mut *(*rcc).entry.offset(i_2 as isize), q);
                    (*rcc).last_qscale_for
                        [(*(*rcc).entry.offset(i_2 as isize)).pict_type as usize] = q;
                    i_2 += 1;
                }
                step_mult = all_available_bits as c_double / expected_bits;
                rate_factor = 0 as c_int as c_double;
                let mut step: c_double = 1E4f64 * step_mult;
                while step > 1E-7f64 * step_mult {
                    expected_bits = 0 as c_int as c_double;
                    rate_factor += step;
                    (*rcc).last_non_b_pict_type = -(1 as c_int);
                    (*rcc).last_accum_p_norm = 1 as c_int as c_double;
                    (*rcc).accum_p_norm = 0 as c_int as c_double;
                    (*rcc).last_qscale_for[2 as c_int as usize] =
                        pow(base_cplx, 1 as c_int as c_double - (*rcc).qcompress) / rate_factor;
                    (*rcc).last_qscale_for[1 as c_int as usize] =
                        (*rcc).last_qscale_for[2 as c_int as usize];
                    (*rcc).last_qscale_for[0 as c_int as usize] =
                        (*rcc).last_qscale_for[1 as c_int as usize];
                    let mut i_3: c_int = 0 as c_int;
                    while i_3 < (*rcc).num_entries {
                        *qscale.offset(i_3 as isize) = get_qscale(
                            h,
                            &mut *(*rcc).entry.offset(i_3 as isize),
                            rate_factor,
                            -(1 as c_int),
                        );
                        (*rcc).last_qscale_for
                            [(*(*rcc).entry.offset(i_3 as isize)).pict_type as usize] =
                            *qscale.offset(i_3 as isize);
                        i_3 += 1;
                    }
                    let mut i_4: c_int = (*rcc).num_entries - 1 as c_int;
                    while i_4 >= 0 as c_int {
                        *qscale.offset(i_4 as isize) = get_diff_limited_q(
                            h,
                            &mut *(*rcc).entry.offset(i_4 as isize),
                            *qscale.offset(i_4 as isize),
                            i_4,
                        );
                        if *qscale.offset(i_4 as isize) >= 0 as c_int as c_double {
                        } else {
                            __assert_fail(
                                b"qscale[i] >= 0\0" as *const u8 as *const c_char,
                                b"encoder/ratecontrol.c\0" as *const u8 as *const c_char,
                                3050 as c_uint,
                                ::core::mem::transmute::<[u8; 25], [c_char; 25]>(
                                    *b"int init_pass2(x264_t *)\0",
                                )
                                .as_ptr(),
                            );
                        }
                        i_4 -= 1;
                    }
                    if filter_size > 1 as c_int {
                        if filter_size % 2 as c_int == 1 as c_int {
                        } else {
                            __assert_fail(
                                b"filter_size%2 == 1\0" as *const u8 as *const c_char,
                                b"encoder/ratecontrol.c\0" as *const u8 as *const c_char,
                                3056 as c_uint,
                                ::core::mem::transmute::<[u8; 25], [c_char; 25]>(
                                    *b"int init_pass2(x264_t *)\0",
                                )
                                .as_ptr(),
                            );
                        }
                        let mut i_5: c_int = 0 as c_int;
                        while i_5 < (*rcc).num_entries {
                            let mut rce_1: *mut ratecontrol_entry_t =
                                &mut *(*rcc).entry.offset(i_5 as isize) as *mut ratecontrol_entry_t;
                            let mut q_0: c_double = 0.0f64;
                            let mut sum: c_double = 0.0f64;
                            let mut j_1: c_int = 0 as c_int;
                            while j_1 < filter_size {
                                let mut idx: c_int = i_5 + j_1 - filter_size / 2 as c_int;
                                let mut d: c_double = (idx - i_5) as c_double;
                                let mut coeff: c_double = if qblur == 0 as c_int as c_double {
                                    1.0f64
                                } else {
                                    exp(-d * d / (qblur * qblur))
                                };
                                if !(idx < 0 as c_int || idx >= (*rcc).num_entries) {
                                    if !((*rce_1).pict_type
                                        != (*(*rcc).entry.offset(idx as isize)).pict_type)
                                    {
                                        q_0 += *qscale.offset(idx as isize) * coeff;
                                        sum += coeff;
                                    }
                                }
                                j_1 += 1;
                            }
                            *blurred_qscale.offset(i_5 as isize) = q_0 / sum;
                            i_5 += 1;
                        }
                    }
                    let mut i_6: c_int = 0 as c_int;
                    while i_6 < (*rcc).num_entries {
                        let mut rce_2: *mut ratecontrol_entry_t =
                            &mut *(*rcc).entry.offset(i_6 as isize) as *mut ratecontrol_entry_t;
                        (*rce_2).new_qscale = clip_qscale(
                            h,
                            (*rce_2).pict_type,
                            *blurred_qscale.offset(i_6 as isize),
                        );
                        if (*rce_2).new_qscale >= 0 as c_int as c_double {
                        } else {
                            __assert_fail(
                                b"rce->new_qscale >= 0\0" as *const u8 as *const c_char,
                                b"encoder/ratecontrol.c\0" as *const u8 as *const c_char,
                                3083 as c_uint,
                                ::core::mem::transmute::<[u8; 25], [c_char; 25]>(
                                    *b"int init_pass2(x264_t *)\0",
                                )
                                .as_ptr(),
                            );
                        }
                        expected_bits += qscale2bits(rce_2, (*rce_2).new_qscale);
                        i_6 += 1;
                    }
                    if expected_bits > all_available_bits as c_double {
                        rate_factor -= step;
                    }
                    step *= 0.5f64;
                }
                x264_free(qscale as *mut c_void);
                if filter_size > 1 as c_int {
                    x264_free(blurred_qscale as *mut c_void);
                }
                if (*rcc).b_vbv != 0 {
                    if vbv_pass2(h, all_available_bits as c_double) != 0 {
                        return -(1 as c_int);
                    }
                }
                expected_bits = count_expected_bits(h);
                if fabs(expected_bits / all_available_bits as c_double - 1.0f64) > 0.01f64 {
                    let mut avgq: c_double = 0 as c_int as c_double;
                    let mut i_7: c_int = 0 as c_int;
                    while i_7 < (*rcc).num_entries {
                        avgq += (*(*rcc).entry.offset(i_7 as isize)).new_qscale;
                        i_7 += 1;
                    }
                    avgq =
                        qscale2qp((avgq / (*rcc).num_entries as c_double) as c_float) as c_double;
                    if expected_bits > all_available_bits as c_double || (*rcc).b_vbv == 0 {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"Error: 2pass curve failed to converge\n\0" as *const u8
                                as *const c_char,
                        );
                    }
                    x264_10_log(
                        h,
                        X264_LOG_WARNING,
                        b"target: %.2f kbit/s, expected: %.2f kbit/s, avg QP: %.4f\n\0" as *const u8
                            as *const c_char,
                        (*h).param.rc.i_bitrate as c_float as c_double,
                        expected_bits * (*rcc).fps / ((*rcc).num_entries as c_double * 1000.0f64),
                        avgq,
                    );
                    if expected_bits < all_available_bits as c_double
                        && avgq < ((*h).param.rc.i_qp_min + 2 as c_int) as c_double
                    {
                        if (*h).param.rc.i_qp_min > 0 as c_int {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"try reducing target bitrate or reducing qp_min (currently %d)\n\0"
                                    as *const u8 as *const c_char,
                                (*h).param.rc.i_qp_min,
                            );
                        } else {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"try reducing target bitrate\n\0" as *const u8 as *const c_char,
                            );
                        }
                    } else if expected_bits > all_available_bits as c_double
                        && avgq > ((*h).param.rc.i_qp_max - 2 as c_int) as c_double
                    {
                        if (*h).param.rc.i_qp_max < QP_MAX {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"try increasing target bitrate or increasing qp_max (currently %d)\n\0"
                                    as *const u8 as *const c_char,
                                (*h).param.rc.i_qp_max,
                            );
                        } else {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"try increasing target bitrate\n\0" as *const u8 as *const c_char,
                            );
                        }
                    } else if !((*rcc).b_2pass != 0 && (*rcc).b_vbv != 0) {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"internal error\n\0" as *const u8 as *const c_char,
                        );
                    }
                }
                return 0 as c_int;
            }
        }
    }
    return -(1 as c_int);
}
