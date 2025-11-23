use ::core::ffi::{c_char, c_double, c_float, c_int, c_long, c_uint, c_void};
use ::core::mem::size_of;

use log::warn;

use crate::__stddef_null_h::NULL;
use crate::__stddef_size_t_h::size_t;
use crate::analyse_h::{
    x264_10_analyse_free_costs, x264_10_analyse_init_costs, x264_10_analyse_weight_frame,
    x264_10_lookahead_delete, x264_10_lookahead_get_frames, x264_10_lookahead_init,
    x264_10_lookahead_is_empty, x264_10_lookahead_put_frame, x264_10_macroblock_analyse,
};
use crate::assert_h::__assert_fail;
use crate::base_h::{
    slice_type_to_char, x264_clip3, x264_clip3f, x264_free, x264_log_internal, x264_malloc,
    x264_param_strdup, x264_reduce_fraction, x264_scan8, x264_union16_t, CHROMA_420, CHROMA_422,
    CHROMA_444, PROFILE_BASELINE, PROFILE_HIGH, PROFILE_HIGH10, PROFILE_HIGH422, PROFILE_MAIN,
    SLICE_TYPE_B, SLICE_TYPE_I, SLICE_TYPE_P, X264_LOOKAHEAD_MAX, X264_REF_MAX, X264_THREAD_HEIGHT,
    X264_THREAD_MAX, X264_WEIGHTP_FAKE,
};
use crate::bitstream_h::{
    bs_align_1, bs_flush, bs_init, bs_pos, bs_rbsp_trailing, bs_realign, bs_s, bs_size_ue_big,
    bs_t, bs_write, bs_write1, bs_write_se, bs_write_ue_big, x264_10_bitstream_init,
};
use crate::cabac_h::{
    x264_10_cabac_context_init, x264_10_cabac_encode_flush, x264_10_cabac_encode_init,
    x264_10_cabac_encode_terminal_c, x264_cabac_pos, x264_cabac_t,
};
use crate::common_h::{
    pixel, x264_10_cabac_init, x264_10_cavlc_init, x264_10_log, x264_frame_stat_t,
    x264_slice_header_t, x264_t, C2RustUnnamed_17, C2RustUnnamed_6, FILLER_OVERHEAD, NALU_OVERHEAD,
    PIXEL_MAX, QP_BD_OFFSET, QP_MAX, SIZEOF_PIXEL,
};
use crate::cpu_h::{x264_cpu_names, x264_cpu_num_processors};
use crate::dct_h::{x264_10_dct_init, x264_10_zigzag_init, x264_zigzag_function_t};
use crate::encoder_macroblock_h::{
    x264_10_cabac_mb_skip, x264_10_macroblock_encode, x264_10_macroblock_write_cabac,
    x264_10_macroblock_write_cavlc, x264_10_noise_reduction_update, x264_10_rdo_init,
};
use crate::encoder_set_h::{
    x264_10_filler_write, x264_10_pps_init, x264_10_pps_write,
    x264_10_sei_alternative_transfer_write, x264_10_sei_avcintra_umid_write,
    x264_10_sei_avcintra_vanc_write, x264_10_sei_buffering_period_write,
    x264_10_sei_dec_ref_pic_marking_write, x264_10_sei_pic_timing_write,
    x264_10_sei_recovery_point_write, x264_10_sei_version_write, x264_10_sei_write,
    x264_10_sps_init, x264_10_sps_init_reconfigurable, x264_10_sps_init_scaling_list,
    x264_10_sps_write, x264_10_validate_levels,
};
use crate::frame_h::{
    x264_10_deblock_init, x264_10_expand_border_mbpair, x264_10_frame_cond_broadcast,
    x264_10_frame_copy_picture, x264_10_frame_deblock_row, x264_10_frame_delete,
    x264_10_frame_delete_list, x264_10_frame_expand_border, x264_10_frame_expand_border_filtered,
    x264_10_frame_expand_border_mod16, x264_10_frame_filter, x264_10_frame_init_lowres,
    x264_10_frame_new_slice, x264_10_frame_pop, x264_10_frame_pop_blank_unused,
    x264_10_frame_pop_unused, x264_10_frame_push, x264_10_frame_push_blank_unused,
    x264_10_frame_push_unused, x264_10_frame_shift, x264_10_frame_unshift,
    x264_10_threadslice_cond_broadcast, x264_10_threadslice_cond_wait, x264_10_weight_scale_plane,
    x264_frame, x264_frame_t, PADH, PADV,
};
use crate::internal::BIT_DEPTH;
use crate::limits_h::INT_MAX;
use crate::macroblock_h::{
    i_chroma_qp_table, x264_10_macroblock_bipred_init, x264_10_macroblock_cache_allocate,
    x264_10_macroblock_cache_free, x264_10_macroblock_cache_load_interlaced,
    x264_10_macroblock_cache_save, x264_10_macroblock_deblock_strength,
    x264_10_macroblock_slice_init, x264_10_macroblock_thread_allocate,
    x264_10_macroblock_thread_free, x264_10_macroblock_thread_init, x264_mb_partition_pixel_table,
    x264_mb_type_list_table, D_8x8, D_BI_8x8, D_DIRECT_8x8, D_L0_8x8, D_L1_8x8, I_16x16, I_4x4,
    I_8x8, B_DIRECT, B_SKIP, DCT_CHROMA_DC, I_PCM, P_SKIP, X264_MBTYPE_MAX, X264_PARTTYPE_MAX,
};
use crate::mathcalls_h::{fabs, log10, log2f, pow};
use crate::mc_h::{weight_fn_t, x264_10_mc_init, x264_weight_t};
use crate::osdep_h::x264_is_regular_file;
use crate::pixel_h::{
    x264_10_field_vsad, x264_10_pixel_init, x264_10_pixel_ssd_nv12, x264_10_pixel_ssd_wxh,
    x264_10_pixel_ssim_wxh, x264_luma2chroma_pixel, x264_pixel_cmp_t, x264_pixel_cmp_x3_t,
    x264_pixel_cmp_x4_t, PIXEL_16x16, PIXEL_16x8, PIXEL_4x4, PIXEL_4x8, PIXEL_8x16, PIXEL_8x4,
    PIXEL_8x8,
};
use crate::predict_h::{
    x264_10_predict_16x16_init, x264_10_predict_4x4_init, x264_10_predict_8x16c_init,
    x264_10_predict_8x8c_init, x264_mb_chroma_pred_mode_fix, x264_mb_pred_mode16x16_fix,
    x264_mb_pred_mode4x4_fix, x264_predict_t, I_PRED_16x16_DC_128, I_PRED_8x8_DC_128,
    I_PRED_CHROMA_DC_128,
};
use crate::pthread_h::{
    pthread_cond_broadcast, pthread_cond_destroy, pthread_cond_init, pthread_mutex_destroy,
    pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock,
};
use crate::pthreadtypes_h::{pthread_condattr_t, pthread_mutexattr_t};
use crate::ratecontrol_h::{
    x264_10_adaptive_quant_frame, x264_10_hrd_fullness, x264_10_macroblock_tree_read,
    x264_10_ratecontrol_delete, x264_10_ratecontrol_end, x264_10_ratecontrol_init_reconfigurable,
    x264_10_ratecontrol_mb, x264_10_ratecontrol_mb_qp, x264_10_ratecontrol_new,
    x264_10_ratecontrol_qp, x264_10_ratecontrol_set_weights, x264_10_ratecontrol_start,
    x264_10_ratecontrol_summary, x264_10_ratecontrol_zone_init,
    x264_10_reference_build_list_optimal, x264_10_thread_sync_ratecontrol,
    x264_10_threads_distribute_ratecontrol, x264_10_threads_merge_ratecontrol,
};
use crate::set_h::{
    x264_10_cqm_delete, x264_10_cqm_init, x264_10_cqm_parse_file, x264_pps_t, x264_sps_t,
};
use crate::src::common::predict::x264_predict_8x8_init;
use crate::src::common::quant::x264_quant_init;
use crate::src::encoder::set::{
    x264_sei_content_light_level_write, x264_sei_frame_packing_write,
    x264_sei_mastering_display_write,
};
use crate::stdint_h::{intptr_t, UINT32_MAX};
use crate::stdint_intn_h::{int32_t, int64_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{fclose, fopen, fseeko, fwrite, snprintf, sprintf, SEEK_SET};
use crate::stdlib_h::abs;
use crate::string_h::{memcpy, memmove, memset, strcmp, strcpy, strlen};
use crate::tables_h::{
    x264_cqm_avci100_1080_4ic, x264_cqm_avci100_1080i_8iy, x264_cqm_avci100_1080p_8iy,
    x264_cqm_avci100_720p_4ic, x264_cqm_avci100_720p_8iy, x264_cqm_avci300_2160p_4ic,
    x264_cqm_avci300_2160p_4iy, x264_cqm_avci300_2160p_8iy, x264_cqm_avci50_1080i_8iy,
    x264_cqm_avci50_4ic, x264_cqm_avci50_p_8iy, x264_cqm_jvt4i, x264_zero,
};
use crate::threadpool_h::{
    x264_10_threadpool_delete, x264_10_threadpool_init, x264_10_threadpool_run,
    x264_10_threadpool_wait,
};
use crate::types_h::__off64_t;
use crate::x264_h::{
    x264_level_t, x264_levels, x264_nal_t, x264_param_cleanup, x264_param_t, x264_picture_t,
    x264_sei_payload_t, X264_ANALYSE_BSUB16x16, X264_ANALYSE_I4x4, X264_ANALYSE_I8x8,
    X264_ANALYSE_PSUB16x16, X264_ANALYSE_PSUB8x8, NAL_AUD, NAL_FILLER, NAL_PPS,
    NAL_PRIORITY_DISPOSABLE, NAL_PRIORITY_HIGH, NAL_PRIORITY_HIGHEST, NAL_PRIORITY_LOW, NAL_SEI,
    NAL_SLICE, NAL_SLICE_IDR, NAL_SPS, PIC_STRUCT_AUTO, PIC_STRUCT_BOTTOM_TOP,
    PIC_STRUCT_PROGRESSIVE, PIC_STRUCT_TOP_BOTTOM, PIC_STRUCT_TRIPLE, X264_AVCINTRA_FLAVOR_SONY,
    X264_B_ADAPT_NONE, X264_B_ADAPT_TRELLIS, X264_CPU_AVX512, X264_CPU_BMI1, X264_CPU_BMI2,
    X264_CPU_CACHELINE_64, X264_CPU_FMA3, X264_CPU_SSE2_IS_FAST, X264_CPU_SSE2_IS_SLOW,
    X264_CPU_SSE42, X264_CPU_SSSE3, X264_CQM_CUSTOM, X264_CQM_FLAT, X264_CSP_BGR,
    X264_CSP_HIGH_DEPTH, X264_CSP_I400, X264_CSP_I420, X264_CSP_I422, X264_CSP_I444, X264_CSP_MASK,
    X264_CSP_MAX, X264_CSP_NONE, X264_DIRECT_PRED_AUTO, X264_DIRECT_PRED_NONE,
    X264_DIRECT_PRED_SPATIAL, X264_KEYINT_MAX_INFINITE, X264_KEYINT_MIN_AUTO, X264_LOG_DEBUG,
    X264_LOG_ERROR, X264_LOG_INFO, X264_LOG_WARNING, X264_ME_DIA, X264_ME_ESA, X264_ME_HEX,
    X264_ME_TESA, X264_ME_UMH, X264_NAL_HRD_CBR, X264_NAL_HRD_NONE, X264_NAL_HRD_VBR, X264_RC_ABR,
    X264_RC_CQP, X264_RC_CRF, X264_THREADS_AUTO, X264_TYPE_AUTO, X264_TYPE_B, X264_TYPE_BREF,
    X264_TYPE_I, X264_TYPE_IDR, X264_TYPE_KEYFRAME, X264_TYPE_P, X264_WEIGHTP_NONE,
    X264_WEIGHTP_SIMPLE, X264_WEIGHTP_SMART,
};
use crate::x264_h::{BPyramid, FramePacking};
use crate::FILE_h::FILE;
extern "C" {
    #[allow(
        clashing_extern_declarations,
        reason = "probably need to merge x264_t types"
    )]
    #[c2rust::src_loc = "44:1"]
    fn x264_10_nal_encode(h: *mut x264_t, dst: *mut uint8_t, nal: *mut x264_nal_t);
    #[c2rust::src_loc = "45:1"]
    fn x264_10_macroblock_cache_load_progressive(h: *mut x264_t, i_mb_x: c_int, i_mb_y: c_int);
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "729:22"]
struct C2RustUnnamed_20 {
    fps_num: uint16_t,
    fps_den: uint16_t,
    interlaced: uint8_t,
    frame_size: uint16_t,
    cqm_4iy: *const uint8_t,
    cqm_4ic: *const uint8_t,
    cqm_8iy: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "2677:9"]
struct x264_bs_bak_t {
    skip: c_int,
    cabac_prevbyte: uint8_t,
    bs: bs_t,
    cabac: x264_cabac_t,
    stat: x264_frame_stat_t,
    last_qp: c_int,
    last_dqp: c_int,
    field_decoding_flag: c_int,
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn calc_psnr(mut sqe: c_double, mut size: c_double) -> c_double {
    let mut mse: c_double = sqe / ((PIXEL_MAX * PIXEL_MAX) as c_double * size);
    if mse <= 0.0000000001f64 {
        return 100 as c_int as c_double;
    }
    return -10.0f64 * log10(mse);
}
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn calc_ssim_db(mut ssim: c_double) -> c_double {
    let mut inv_ssim: c_double = 1 as c_int as c_double - ssim;
    if inv_ssim <= 0.0000000001f64 {
        return 100 as c_int as c_double;
    }
    return -10.0f64 * log10(inv_ssim);
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn threadpool_wait_all(mut h: *mut x264_t) -> c_int {
    let mut i: c_int = 0 as c_int;
    while i < (*h).param.i_threads {
        if (*(*h).thread[i as usize]).b_thread_active != 0 {
            (*(*h).thread[i as usize]).b_thread_active = 0 as c_int;
            if (x264_10_threadpool_wait((*h).threadpool, (*h).thread[i as usize] as *mut c_void)
                as intptr_t)
                < 0 as intptr_t
            {
                return -1;
            }
        }
        i += 1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "86:1"]
unsafe extern "C" fn frame_dump(mut h: *mut x264_t) {
    let mut f: *mut FILE = fopen(
        (*h).param.psz_dump_yuv,
        b"r+b\0" as *const u8 as *const c_char,
    ) as *mut FILE;
    if f.is_null() {
        return;
    }
    if (*h).param.b_sliced_threads != 0 {
        threadpool_wait_all(h);
    }
    let mut frame_size: c_int = ((*h).param.height * (*h).param.width * size_of::<pixel>() as u32
        + 2 * (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            (*h).param.height * (*h).param.width * size_of::<pixel>() as u32
                >> (*h).mb.chroma_h_shift + (*h).mb.chroma_v_shift
        } else {
            0
        }) as u32) as c_int;
    if fseeko(
        f,
        (*(*h).fdec).i_frame as __off64_t * frame_size as __off64_t,
        SEEK_SET,
    ) == 0
    {
        let mut p: c_int = 0 as c_int;
        while p
            < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                3 as c_int
            } else {
                1 as c_int
            })
        {
            let mut y: c_int = 0 as c_int;
            while y < (*h).param.height as c_int {
                fwrite(
                    &mut *(*(*(*h).fdec).plane.as_mut_ptr().offset(p as isize)).offset(
                        (y * *(*(*h).fdec).i_stride.as_mut_ptr().offset(p as isize)) as isize,
                    ) as *mut pixel as *const c_void,
                    SIZEOF_PIXEL as size_t,
                    (*h).param.width as size_t,
                    f,
                );
                y += 1;
            }
            p += 1;
        }
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as c_int
            || (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as c_int
        {
            let mut cw: c_int = (*h).param.width as c_int >> 1;
            let mut ch: c_int = (*h).param.height as c_int >> (*h).mb.chroma_v_shift;
            let mut planeu: *mut pixel =
                x264_malloc((2 as c_int * (cw * ch * SIZEOF_PIXEL + 32 as c_int)) as int64_t)
                    as *mut pixel;
            if !planeu.is_null() {
                let mut planev: *mut pixel = planeu
                    .offset((cw * ch) as isize)
                    .offset((32 as c_int / SIZEOF_PIXEL) as isize);
                (*h).mc
                    .plane_copy_deinterleave
                    .expect("non-null function pointer")(
                    planeu,
                    cw as intptr_t,
                    planev,
                    cw as intptr_t,
                    (*(*h).fdec).plane[1],
                    (*(*h).fdec).i_stride[1] as intptr_t,
                    cw,
                    ch,
                );
                fwrite(
                    planeu as *const c_void,
                    1 as size_t,
                    (cw * ch * SIZEOF_PIXEL) as size_t,
                    f,
                );
                fwrite(
                    planev as *const c_void,
                    1 as size_t,
                    (cw * ch * SIZEOF_PIXEL) as size_t,
                    f,
                );
                x264_free(planeu as *mut c_void);
            }
        }
    }
    fclose(f);
}
#[c2rust::src_loc = "122:1"]
unsafe extern "C" fn slice_header_init(
    mut h: *mut x264_t,
    mut sh: *mut x264_slice_header_t,
    mut sps: *mut x264_sps_t,
    mut pps: *mut x264_pps_t,
    mut i_idr_pic_id: c_int,
    mut i_frame: c_int,
    mut i_qp: c_int,
) {
    let mut param: *mut x264_param_t = &mut (*h).param;
    (*sh).sps = sps;
    (*sh).pps = pps;
    (*sh).i_first_mb = 0 as c_int;
    (*sh).i_last_mb = (*h).mb.i_mb_count - 1 as c_int;
    (*sh).i_pps_id = (*pps).i_id;
    (*sh).i_frame_num = i_frame;
    (*sh).b_mbaff = (*h).param.b_interlaced;
    (*sh).b_field_pic = 0 as c_int;
    (*sh).b_bottom_field = 0 as c_int;
    (*sh).i_idr_pic_id = i_idr_pic_id;
    (*sh).i_poc = 0 as c_int;
    (*sh).i_delta_poc_bottom = 0 as c_int;
    (*sh).i_delta_poc[0] = 0 as c_int;
    (*sh).i_delta_poc[1] = 0 as c_int;
    (*sh).i_redundant_pic_cnt = 0 as c_int;
    (*h).mb.b_direct_auto_write = ((*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_AUTO
        && (*h).param.i_bframe != 0
        && ((*h).param.rc.b_stat_write != 0 || (*h).param.rc.b_stat_read == 0))
        as c_int;
    if (*h).mb.b_direct_auto_read == 0 && (*sh).i_type == SLICE_TYPE_B as c_int {
        if (*(*h).fref[1][0]).i_poc_l0ref0 == (*(*h).fref[0][0]).i_poc {
            if (*h).mb.b_direct_auto_write != 0 {
                (*sh).b_direct_spatial_mv_pred =
                    ((*h).stat.i_direct_score[1] > (*h).stat.i_direct_score[0]) as c_int;
            } else {
                (*sh).b_direct_spatial_mv_pred =
                    ((*param).analyse.i_direct_mv_pred == X264_DIRECT_PRED_SPATIAL) as c_int;
            }
        } else {
            (*h).mb.b_direct_auto_write = 0 as c_int;
            (*sh).b_direct_spatial_mv_pred = 1 as c_int;
        }
    }
    (*sh).b_num_ref_idx_override = 0 as c_int;
    (*sh).i_num_ref_idx_l0_active = 1 as c_int;
    (*sh).i_num_ref_idx_l1_active = 1 as c_int;
    (*sh).b_ref_pic_list_reordering[0] = (*h).b_ref_reorder[0];
    (*sh).b_ref_pic_list_reordering[1] = (*h).b_ref_reorder[1];
    let mut list: c_int = 0 as c_int;
    while list < 2 as c_int {
        if (*sh).b_ref_pic_list_reordering[list as usize] != 0 {
            let mut pred_frame_num: c_int = i_frame;
            let mut i: c_int = 0 as c_int;
            while i < (*h).i_ref[list as usize] {
                let mut diff: c_int =
                    (*(*h).fref[list as usize][i as usize]).i_frame_num - pred_frame_num;
                (*sh).ref_pic_list_order[list as usize][i as usize].idc =
                    (diff > 0 as c_int) as c_int;
                (*sh).ref_pic_list_order[list as usize][i as usize].arg = abs(diff) - 1 as c_int
                    & ((1 as c_int) << (*sps).i_log2_max_frame_num) - 1 as c_int;
                pred_frame_num = (*(*h).fref[list as usize][i as usize]).i_frame_num;
                i += 1;
            }
        }
        list += 1;
    }
    (*sh).i_cabac_init_idc = (*param).i_cabac_init_idc;
    (*sh).i_qp = if i_qp < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) {
        i_qp
    } else {
        51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
    };
    (*sh).i_qp_delta = (*sh).i_qp - (*pps).i_pic_init_qp;
    (*sh).b_sp_for_swidth = 0 as c_int;
    (*sh).i_qs_delta = 0 as c_int;
    let mut deblock_thresh: c_int = i_qp
        + 2 as c_int
            * (if (*param).i_deblocking_filter_alphac0 < (*param).i_deblocking_filter_beta {
                (*param).i_deblocking_filter_alphac0
            } else {
                (*param).i_deblocking_filter_beta
            });
    if (*param).b_deblocking_filter != 0
        && ((*h).mb.b_variable_qp != 0 || (15 as c_int) < deblock_thresh)
    {
        (*sh).i_disable_deblocking_filter_idc = if (*param).b_sliced_threads != 0 {
            2 as c_int
        } else {
            0 as c_int
        };
    } else {
        (*sh).i_disable_deblocking_filter_idc = 1 as c_int;
    }
    (*sh).i_alpha_c0_offset = (*param).i_deblocking_filter_alphac0 * 2 as c_int;
    (*sh).i_beta_offset = (*param).i_deblocking_filter_beta * 2 as c_int;
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn slice_header_write(
    mut s: *mut bs_t,
    mut sh: *mut x264_slice_header_t,
    mut i_nal_ref_idc: c_int,
) {
    if (*sh).b_mbaff != 0 {
        let mut first_x: c_int = (*sh).i_first_mb % (*(*sh).sps).i_mb_width;
        let mut first_y: c_int = (*sh).i_first_mb / (*(*sh).sps).i_mb_width;
        if first_y & 1 as c_int == 0 as c_int {
        } else {
            __assert_fail(
                b"(first_y&1) == 0\0" as *const u8 as *const c_char,
                b"encoder/encoder.c\0" as *const u8 as *const c_char,
                219 as c_uint,
                ::core::mem::transmute::<[u8; 60], [c_char; 60]>(
                    *b"void slice_header_write(bs_t *, x264_slice_header_t *, int)\0",
                )
                .as_ptr(),
            );
        }
        bs_write_ue_big(
            s,
            (2 as c_int * first_x
                + (*(*sh).sps).i_mb_width * (first_y & !(1 as c_int))
                + (first_y & 1 as c_int)
                >> 1 as c_int) as c_uint,
        );
    } else {
        bs_write_ue_big(s, (*sh).i_first_mb as c_uint);
    }
    bs_write_ue_big(s, ((*sh).i_type + 5 as c_int) as c_uint);
    bs_write_ue_big(s, (*sh).i_pps_id as c_uint);
    bs_write(
        s,
        (*(*sh).sps).i_log2_max_frame_num,
        ((*sh).i_frame_num & ((1 as c_int) << (*(*sh).sps).i_log2_max_frame_num) - 1 as c_int)
            as uint32_t,
    );
    if (*(*sh).sps).b_frame_mbs_only == 0 {
        bs_write1(s, (*sh).b_field_pic as uint32_t);
        if (*sh).b_field_pic != 0 {
            bs_write1(s, (*sh).b_bottom_field as uint32_t);
        }
    }
    if (*sh).i_idr_pic_id >= 0 as c_int {
        bs_write_ue_big(s, (*sh).i_idr_pic_id as c_uint);
    }
    if (*(*sh).sps).i_poc_type == 0 as c_int {
        bs_write(
            s,
            (*(*sh).sps).i_log2_max_poc_lsb,
            ((*sh).i_poc & ((1 as c_int) << (*(*sh).sps).i_log2_max_poc_lsb) - 1 as c_int)
                as uint32_t,
        );
        if (*(*sh).pps).b_pic_order != 0 && (*sh).b_field_pic == 0 {
            bs_write_se(s, (*sh).i_delta_poc_bottom);
        }
    }
    if (*(*sh).pps).b_redundant_pic_cnt != 0 {
        bs_write_ue_big(s, (*sh).i_redundant_pic_cnt as c_uint);
    }
    if (*sh).i_type == SLICE_TYPE_B as c_int {
        bs_write1(s, (*sh).b_direct_spatial_mv_pred as uint32_t);
    }
    if (*sh).i_type == SLICE_TYPE_P as c_int || (*sh).i_type == SLICE_TYPE_B as c_int {
        bs_write1(s, (*sh).b_num_ref_idx_override as uint32_t);
        if (*sh).b_num_ref_idx_override != 0 {
            bs_write_ue_big(s, ((*sh).i_num_ref_idx_l0_active - 1 as c_int) as c_uint);
            if (*sh).i_type == SLICE_TYPE_B as c_int {
                bs_write_ue_big(s, ((*sh).i_num_ref_idx_l1_active - 1 as c_int) as c_uint);
            }
        }
    }
    if (*sh).i_type != SLICE_TYPE_I as c_int {
        bs_write1(s, (*sh).b_ref_pic_list_reordering[0] as uint32_t);
        if (*sh).b_ref_pic_list_reordering[0] != 0 {
            let mut i: c_int = 0 as c_int;
            while i < (*sh).i_num_ref_idx_l0_active {
                bs_write_ue_big(s, (*sh).ref_pic_list_order[0][i as usize].idc as c_uint);
                bs_write_ue_big(s, (*sh).ref_pic_list_order[0][i as usize].arg as c_uint);
                i += 1;
            }
            bs_write_ue_big(s, 3 as c_uint);
        }
    }
    if (*sh).i_type == SLICE_TYPE_B as c_int {
        bs_write1(s, (*sh).b_ref_pic_list_reordering[1] as uint32_t);
        if (*sh).b_ref_pic_list_reordering[1] != 0 {
            let mut i_0: c_int = 0 as c_int;
            while i_0 < (*sh).i_num_ref_idx_l1_active {
                bs_write_ue_big(s, (*sh).ref_pic_list_order[1][i_0 as usize].idc as c_uint);
                bs_write_ue_big(s, (*sh).ref_pic_list_order[1][i_0 as usize].arg as c_uint);
                i_0 += 1;
            }
            bs_write_ue_big(s, 3 as c_uint);
        }
    }
    (*sh).b_weighted_pred = 0 as c_int;
    if (*(*sh).pps).b_weighted_pred != 0 && (*sh).i_type == SLICE_TYPE_P as c_int {
        (*sh).b_weighted_pred = (!(*sh).weight[0][0].weightfn.is_null()
            || !(*sh).weight[0][1].weightfn.is_null()
            || !(*sh).weight[0][2].weightfn.is_null()) as c_int;
        bs_write_ue_big(s, (*sh).weight[0][0].i_denom as c_uint);
        if (*(*sh).sps).i_chroma_format_idc != 0 {
            bs_write_ue_big(s, (*sh).weight[0][1].i_denom as c_uint);
        }
        let mut i_1: c_int = 0 as c_int;
        while i_1 < (*sh).i_num_ref_idx_l0_active {
            let mut luma_weight_l0_flag: c_int =
                !(*sh).weight[i_1 as usize][0].weightfn.is_null() as c_int;
            bs_write1(s, luma_weight_l0_flag as uint32_t);
            if luma_weight_l0_flag != 0 {
                bs_write_se(s, (*sh).weight[i_1 as usize][0].i_scale as c_int);
                bs_write_se(s, (*sh).weight[i_1 as usize][0].i_offset as c_int);
            }
            if (*(*sh).sps).i_chroma_format_idc != 0 {
                let mut chroma_weight_l0_flag: c_int =
                    (!(*sh).weight[i_1 as usize][1].weightfn.is_null()
                        || !(*sh).weight[i_1 as usize][2].weightfn.is_null())
                        as c_int;
                bs_write1(s, chroma_weight_l0_flag as uint32_t);
                if chroma_weight_l0_flag != 0 {
                    let mut j: c_int = 1 as c_int;
                    while j < 3 as c_int {
                        bs_write_se(s, (*sh).weight[i_1 as usize][j as usize].i_scale as c_int);
                        bs_write_se(s, (*sh).weight[i_1 as usize][j as usize].i_offset as c_int);
                        j += 1;
                    }
                }
            }
            i_1 += 1;
        }
    } else if (*(*sh).pps).b_weighted_bipred == 1 as c_int && (*sh).i_type == SLICE_TYPE_B as c_int
    {
        // TODO: from original x264
    }
    if i_nal_ref_idc != 0 as c_int {
        if (*sh).i_idr_pic_id >= 0 as c_int {
            bs_write1(s, 0 as uint32_t);
            bs_write1(s, 0 as uint32_t);
        } else {
            bs_write1(
                s,
                ((*sh).i_mmco_command_count > 0 as c_int) as c_int as uint32_t,
            );
            if (*sh).i_mmco_command_count > 0 as c_int {
                let mut i_2: c_int = 0 as c_int;
                while i_2 < (*sh).i_mmco_command_count {
                    bs_write_ue_big(s, 1 as c_uint);
                    bs_write_ue_big(
                        s,
                        ((*sh).mmco[i_2 as usize].i_difference_of_pic_nums - 1 as c_int) as c_uint,
                    );
                    i_2 += 1;
                }
                bs_write_ue_big(s, 0 as c_uint);
            }
        }
    }
    if (*(*sh).pps).b_cabac != 0 && (*sh).i_type != SLICE_TYPE_I as c_int {
        bs_write_ue_big(s, (*sh).i_cabac_init_idc as c_uint);
    }
    bs_write_se(s, (*sh).i_qp_delta);
    if (*(*sh).pps).b_deblocking_filter_control != 0 {
        bs_write_ue_big(s, (*sh).i_disable_deblocking_filter_idc as c_uint);
        if (*sh).i_disable_deblocking_filter_idc != 1 as c_int {
            bs_write_se(s, (*sh).i_alpha_c0_offset >> 1 as c_int);
            bs_write_se(s, (*sh).i_beta_offset >> 1 as c_int);
        }
    }
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn bitstream_check_buffer_internal(
    mut h: *mut x264_t,
    mut size: c_int,
    mut b_cabac: c_int,
    mut i_nal: c_int,
) -> c_int {
    if b_cabac != 0 && ((*h).cabac.p_end.offset_from((*h).cabac.p) as c_long) < size as c_long
        || ((*h).out.bs.p_end.offset_from((*h).out.bs.p) as c_long) < size as c_long
    {
        if size > INT_MAX - (*h).out.i_bitstream {
            return -1;
        }
        let mut buf_size: c_int = (*h).out.i_bitstream + size;
        let mut buf: *mut uint8_t = x264_malloc(buf_size as int64_t) as *mut uint8_t;
        if buf.is_null() {
            return -1;
        }
        let mut aligned_size: c_int = (*h).out.i_bitstream & !(15 as c_int);
        (*h).mc.memcpy_aligned.expect("non-null function pointer")(
            buf as *mut c_void,
            (*h).out.p_bitstream as *const c_void,
            aligned_size as size_t,
        );
        memcpy(
            buf.offset(aligned_size as isize) as *mut c_void,
            (*h).out.p_bitstream.offset(aligned_size as isize) as *const c_void,
            ((*h).out.i_bitstream - aligned_size) as size_t,
        );
        let mut delta: intptr_t = buf.offset_from((*h).out.p_bitstream) as intptr_t;
        (*h).out.bs.p_start = (*h).out.bs.p_start.offset(delta as isize);
        (*h).out.bs.p = (*h).out.bs.p.offset(delta as isize);
        (*h).out.bs.p_end = buf.offset(buf_size as isize);
        (*h).cabac.p_start = (*h).cabac.p_start.offset(delta as isize);
        (*h).cabac.p = (*h).cabac.p.offset(delta as isize);
        (*h).cabac.p_end = buf.offset(buf_size as isize);
        let mut i: c_int = 0 as c_int;
        while i <= i_nal {
            let ref mut fresh2 = (*(*h).out.nal.offset(i as isize)).p_payload;
            *fresh2 = (*fresh2).offset(delta as isize);
            i += 1;
        }
        x264_free((*h).out.p_bitstream as *mut c_void);
        (*h).out.p_bitstream = buf;
        (*h).out.i_bitstream = buf_size;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "403:1"]
unsafe extern "C" fn bitstream_check_buffer(mut h: *mut x264_t) -> c_int {
    let mut max_row_size: c_int = ((2500 as c_int) << (*h).sh.b_mbaff) * (*h).mb.i_mb_width;
    return bitstream_check_buffer_internal(h, max_row_size, (*h).param.b_cabac, (*h).out.i_nal);
}
#[c2rust::src_loc = "409:1"]
unsafe extern "C" fn bitstream_check_buffer_filler(mut h: *mut x264_t, mut filler: c_int) -> c_int {
    filler += 32 as c_int;
    return bitstream_check_buffer_internal(h, filler, 0 as c_int, -1);
}
#[c2rust::src_loc = "423:1"]
unsafe extern "C" fn validate_parameters(mut h: *mut x264_t, mut b_open: c_int) -> c_int {
    if (*h).param.pf_log.is_none() {
        x264_log_internal(
            X264_LOG_ERROR,
            b"pf_log not set! did you forget to call x264_param_default?\n\0" as *const u8
                as *const c_char,
        );
        return -1;
    }
    (*h).param.b_interlaced = ((*h).param.b_interlaced != 0) as c_int;
    if (*h).param.width == 0
        || (*h).param.height == 0
        || (*h).param.width > MAX_RESOLUTION
        || (*h).param.height > MAX_RESOLUTION
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            c"invalid width x height (%dx%d)\n".as_ptr(),
            (*h).param.width,
            (*h).param.height,
        );
        return -1;
    }
    let mut i_csp: c_int = (*h).param.i_csp & X264_CSP_MASK;
    if i_csp <= X264_CSP_NONE || i_csp >= X264_CSP_MAX {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"invalid CSP (only I400/I420/YV12/NV12/NV21/I422/YV16/NV16/YUYV/UYVY/I444/YV24/BGR/BGRA/RGB supported)\n\0"
                as *const u8 as *const c_char,
        );
        return -1;
    }
    let mut w_mod: c_int = 1 as c_int;
    let mut h_mod: c_int = (1 as c_int)
        << ((*h).param.b_interlaced != 0 || (*h).param.b_fake_interlaced != 0) as c_int;
    if i_csp == X264_CSP_I400 {
        (*h).param.analyse.i_chroma_qp_offset = 0 as c_int;
        (*h).param.analyse.b_chroma_me = 0 as c_int;
        (*h).param.vui.i_colmatrix = 2 as c_int;
    } else if i_csp < X264_CSP_I444 {
        w_mod = 2 as c_int;
        if i_csp < X264_CSP_I422 {
            h_mod *= 2 as c_int;
        }
    }
    if (*h).param.width as c_int % w_mod != 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            c"width not divisible by %d (%dx%d)\n".as_ptr(),
            w_mod,
            (*h).param.width,
            (*h).param.height,
        );
        return -1;
    }
    if (*h).param.height as c_int % h_mod != 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            c"height not divisible by %d (%dx%d)\n".as_ptr(),
            h_mod,
            (*h).param.width,
            (*h).param.height,
        );
        return -1;
    }
    if (*h).param.crop_rect.left >= (*h).param.width
        || (*h).param.crop_rect.right >= (*h).param.width
        || (*h).param.crop_rect.top >= (*h).param.height
        || (*h).param.crop_rect.bottom >= (*h).param.height
        || (*h).param.crop_rect.left + (*h).param.crop_rect.right >= (*h).param.width
        || (*h).param.crop_rect.top + (*h).param.crop_rect.bottom >= (*h).param.height
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            c"invalid crop-rect %d,%d,%d,%d\n".as_ptr(),
            (*h).param.crop_rect.left,
            (*h).param.crop_rect.top,
            (*h).param.crop_rect.right,
            (*h).param.crop_rect.bottom,
        );
        return -1;
    }
    if (*h).param.crop_rect.left as c_int % w_mod != 0
        || (*h).param.crop_rect.right as c_int % w_mod != 0
        || (*h).param.crop_rect.top as c_int % h_mod != 0
        || (*h).param.crop_rect.bottom as c_int % h_mod != 0
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"crop-rect %d,%d,%d,%d not divisible by %dx%d\n\0" as *const u8 as *const c_char,
            (*h).param.crop_rect.left,
            (*h).param.crop_rect.top,
            (*h).param.crop_rect.right,
            (*h).param.crop_rect.bottom,
            w_mod,
            h_mod,
        );
        return -1;
    }
    if (*h).param.vui.i_sar_width <= 0 as c_int || (*h).param.vui.i_sar_height <= 0 as c_int {
        (*h).param.vui.i_sar_width = 0 as c_int;
        (*h).param.vui.i_sar_height = 0 as c_int;
    }
    if (*h).param.i_threads == X264_THREADS_AUTO {
        (*h).param.i_threads = x264_cpu_num_processors()
            * (if (*h).param.b_sliced_threads != 0 {
                2 as c_int
            } else {
                3 as c_int
            })
            / 2 as c_int;
        let mut max_threads: c_int =
            if 1 as c_int > ((*h).param.height as c_int + 15 as c_int) / 16 as c_int / 2 as c_int {
                1 as c_int
            } else {
                ((*h).param.height as c_int + 15 as c_int) / 16 as c_int / 2 as c_int
            };
        (*h).param.i_threads = if (*h).param.i_threads < max_threads {
            (*h).param.i_threads
        } else {
            max_threads
        };
    }
    let mut max_sliced_threads: c_int =
        if 1 as c_int > ((*h).param.height as c_int + 15 as c_int) / 16 as c_int / 4 as c_int {
            1 as c_int
        } else {
            ((*h).param.height as c_int + 15 as c_int) / 16 as c_int / 4 as c_int
        };
    if (*h).param.i_threads > 1 as c_int {
        if (*h).param.b_sliced_threads != 0 {
            (*h).param.i_threads = if (*h).param.i_threads < max_sliced_threads {
                (*h).param.i_threads
            } else {
                max_sliced_threads
            };
        }
    }
    (*h).param.i_threads = x264_clip3((*h).param.i_threads, 1 as c_int, X264_THREAD_MAX);
    if (*h).param.i_threads == 1 as c_int {
        (*h).param.b_sliced_threads = 0 as c_int;
        (*h).param.i_lookahead_threads = 1 as c_int;
    }
    (*h).i_thread_frames = if (*h).param.b_sliced_threads != 0 {
        1 as c_int
    } else {
        (*h).param.i_threads
    };
    if (*h).i_thread_frames > 1 as c_int {
        (*h).param.nalu_process = None;
    }
    if (*h).param.b_opencl != 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"OpenCL: not compiled with OpenCL support, disabling\n\0" as *const u8
                as *const c_char,
        );
        (*h).param.b_opencl = 0 as c_int;
        if !(*h).param.opencl_device_id.is_null() && (*h).param.i_opencl_device != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"OpenCL: device id and device skip count configured; dropping skip\n\0"
                    as *const u8 as *const c_char,
            );
            (*h).param.i_opencl_device = 0 as c_int;
        }
    }
    (*h).param.i_keyint_max = x264_clip3(
        (*h).param.i_keyint_max,
        1 as c_int,
        X264_KEYINT_MAX_INFINITE,
    );
    if (*h).param.i_keyint_max == 1 as c_int {
        (*h).param.b_intra_refresh = 0 as c_int;
        (*h).param.analyse.i_weighted_pred = 0 as c_int;
        (*h).param.i_frame_reference = 1 as c_int;
        (*h).param.i_dpb_size = 1 as c_int;
    }
    if (*h).param.frame_packing == Some(FramePacking::TileFormat)
        && (((*h).param.width - (*h).param.crop_rect.left - (*h).param.crop_rect.right) % 3 != 0
            || ((*h).param.height - (*h).param.crop_rect.top - (*h).param.crop_rect.bottom) % 3
                != 0)
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            c"cropped resolution %dx%d not compatible with tile format frame packing\n".as_ptr(),
            (*h).param.width - (*h).param.crop_rect.left - (*h).param.crop_rect.right,
            (*h).param.height - (*h).param.crop_rect.top - (*h).param.crop_rect.bottom,
        );
        return -1;
    }
    if let Some(mastering_display) = (*h).param.mastering_display {
        if mastering_display.display_min == 50000 && mastering_display.display_max == 50000 {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"mastering display min and max brightness cannot both be 50000\n\0" as *const u8
                    as *const c_char,
            );
            return -1;
        }
    }
    if b_open != 0 {
        let mut score: c_int = 0 as c_int;
        score += ((*h).param.analyse.i_me_range == 0 as c_int) as c_int;
        score += ((*h).param.rc.i_qp_step == 3 as c_int) as c_int;
        score += ((*h).param.i_keyint_max == 12 as c_int) as c_int;
        score += ((*h).param.rc.i_qp_min == 2 as c_int) as c_int;
        score += ((*h).param.rc.i_qp_max == 31 as c_int) as c_int;
        score += ((*h).param.rc.f_qcompress as c_double == 0.5f64) as c_int;
        score += (fabs((*h).param.rc.f_ip_factor as c_double - 1.25f64) < 0.01f64) as c_int;
        score += (fabs((*h).param.rc.f_pb_factor as c_double - 1.25f64) < 0.01f64) as c_int;
        score += ((*h).param.analyse.inter == 0 as c_uint
            && (*h).param.analyse.i_subpel_refine == 8 as c_int) as c_int;
        if score >= 5 as c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"broken ffmpeg default settings detected\n\0" as *const u8 as *const c_char,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"use an encoding preset (e.g. -vpre medium)\n\0" as *const u8 as *const c_char,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"preset usage: -vpre <speed> -vpre <profile>\n\0" as *const u8 as *const c_char,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"speed presets are listed in x264 --help\n\0" as *const u8 as *const c_char,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"profile is optional; x264 defaults to high\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
    }
    if (*h).param.rc.i_rc_method < 0 as c_int || (*h).param.rc.i_rc_method > 2 as c_int {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"no ratecontrol method specified\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    if (*h).param.b_interlaced != 0 {
        (*h).param.b_pic_struct = 1 as c_int;
    }
    if (*h).param.i_avcintra_class != 0 {
        if BIT_DEPTH != 10 as c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"%2d-bit AVC-Intra is not widely compatible\n\0" as *const u8 as *const c_char,
                BIT_DEPTH,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"10-bit x264 is required to encode AVC-Intra\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        let mut type_0: c_int = if (*h).param.i_avcintra_class == 480 as c_int {
            4 as c_int
        } else if (*h).param.i_avcintra_class == 300 as c_int {
            3 as c_int
        } else if (*h).param.i_avcintra_class == 200 as c_int {
            2 as c_int
        } else if (*h).param.i_avcintra_class == 100 as c_int {
            1 as c_int
        } else if (*h).param.i_avcintra_class == 50 as c_int {
            0 as c_int
        } else {
            -1
        };
        if type_0 < 0 as c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"Invalid AVC-Intra class\n\0" as *const u8 as *const c_char,
            );
            return -1;
        } else if type_0 > 2 as c_int && (*h).param.i_avcintra_flavor != X264_AVCINTRA_FLAVOR_SONY {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"AVC-Intra %d only supported by Sony XAVC flavor\n\0" as *const u8
                    as *const c_char,
                (*h).param.i_avcintra_class,
            );
            return -1;
        }
        static mut avcintra_lut: [[[C2RustUnnamed_20; 7]; 2]; 5] = unsafe {
            [
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 912 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1100 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 912 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1100 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 912 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 1820 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 2196 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1820 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1820 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 2196 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 2196 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1820 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                    ],
                ],
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1848 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 2224 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1848 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 2224 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1848 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 3692 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 4444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 3692 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 3692 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 4444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 4444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 3692 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                    ],
                ],
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 3724 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 4472 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 7444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 8940 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 7444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 7444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 8940 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 8940 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 7444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                    ],
                ],
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [C2RustUnnamed_20 {
                        fps_num: 0,
                        fps_den: 0,
                        interlaced: 0,
                        frame_size: 0,
                        cqm_4iy: 0 as *const uint8_t,
                        cqm_4ic: 0 as *const uint8_t,
                        cqm_8iy: 0 as *const uint8_t,
                    }; 7],
                ],
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [C2RustUnnamed_20 {
                        fps_num: 0,
                        fps_den: 0,
                        interlaced: 0,
                        frame_size: 0,
                        cqm_4iy: 0 as *const uint8_t,
                        cqm_4ic: 0 as *const uint8_t,
                        cqm_8iy: 0 as *const uint8_t,
                    }; 7],
                ],
            ]
        };
        let mut res: c_int = -1;
        if i_csp >= X264_CSP_I420 && i_csp < X264_CSP_I422 && type_0 == 0 {
            if (*h).param.width == 1440 && (*h).param.height == 1080 {
                res = 1 as c_int;
            } else if (*h).param.width == 960 && (*h).param.height == 720 {
                res = 0 as c_int;
            }
        } else if i_csp >= X264_CSP_I422 && i_csp < X264_CSP_I444 && type_0 != 0 {
            if type_0 < 3 as c_int {
                if (*h).param.width == 1920 && (*h).param.height == 1080 {
                    res = 1 as c_int;
                } else if (*h).param.width == 2048 && (*h).param.height == 1080 {
                    res = 1 as c_int;
                } else if (*h).param.width == 1280 && (*h).param.height == 720 {
                    res = 0 as c_int;
                }
            } else if (*h).param.width == 3840 && (*h).param.height == 2160 {
                res = 0 as c_int;
            } else if (*h).param.width == 4096 && (*h).param.height == 2160 {
                res = 0 as c_int;
            }
        } else {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"Invalid colorspace for AVC-Intra %d\n\0" as *const u8 as *const c_char,
                (*h).param.i_avcintra_class,
            );
            return -1;
        }
        if res < 0 as c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"Resolution %dx%d invalid for AVC-Intra %d\n\0" as *const u8 as *const c_char,
                (*h).param.width,
                (*h).param.height,
                (*h).param.i_avcintra_class,
            );
            return -1;
        }
        if (*h).param.nalu_process.is_some() {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"nalu_process is not supported in AVC-Intra mode\n\0" as *const u8
                    as *const c_char,
            );
            return -1;
        }
        if (*h).param.b_repeat_headers == 0 {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"Separate headers not supported in AVC-Intra mode\n\0" as *const u8
                    as *const c_char,
            );
            return -1;
        }
        let mut i: c_int = 0;
        let mut fps_num: uint32_t = (*h).param.i_fps_num;
        let mut fps_den: uint32_t = (*h).param.i_fps_den;
        x264_reduce_fraction(&mut fps_num, &mut fps_den);
        i = 0 as c_int;
        while i < 7 as c_int {
            if avcintra_lut[type_0 as usize][res as usize][i as usize].fps_num as uint32_t
                == fps_num
                && avcintra_lut[type_0 as usize][res as usize][i as usize].fps_den as uint32_t
                    == fps_den
                && avcintra_lut[type_0 as usize][res as usize][i as usize].interlaced as c_int
                    == (*h).param.b_interlaced
            {
                break;
            }
            i += 1;
        }
        if i == 7 as c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"FPS %d/%d%c not compatible with AVC-Intra %d\n\0" as *const u8 as *const c_char,
                (*h).param.i_fps_num,
                (*h).param.i_fps_den,
                if (*h).param.b_interlaced != 0 {
                    'i' as i32
                } else {
                    'p' as i32
                },
                (*h).param.i_avcintra_class,
            );
            return -1;
        }
        (*h).param.i_keyint_max = 1 as c_int;
        (*h).param.b_intra_refresh = 0 as c_int;
        (*h).param.analyse.i_weighted_pred = 0 as c_int;
        (*h).param.i_frame_reference = 1 as c_int;
        (*h).param.i_dpb_size = 1 as c_int;
        (*h).param.b_bluray_compat = 0 as c_int;
        (*h).param.b_vfr_input = 0 as c_int;
        (*h).param.b_aud = 1 as c_int;
        (*h).param.vui.i_chroma_loc = 0 as c_int;
        (*h).param.i_nal_hrd = X264_NAL_HRD_NONE;
        (*h).param.b_deblocking_filter = 0 as c_int;
        (*h).param.b_stitchable = 1 as c_int;
        (*h).param.b_pic_struct = 0 as c_int;
        (*h).param.analyse.b_transform_8x8 = 1 as c_int;
        (*h).param.analyse.intra = X264_ANALYSE_I8x8;
        (*h).param.analyse.i_chroma_qp_offset = if type_0 > 2 as c_int {
            -(4 as c_int)
        } else if res != 0 && type_0 != 0 {
            3 as c_int
        } else {
            4 as c_int
        };
        (*h).param.b_cabac = (type_0 == 0) as c_int;
        (*h).param.rc.i_vbv_buffer_size =
            avcintra_lut[type_0 as usize][res as usize][i as usize].frame_size as c_int;
        (*h).param.rc.i_bitrate = ((*h).param.rc.i_vbv_buffer_size as uint32_t)
            .wrapping_mul(fps_num)
            .wrapping_div(fps_den) as c_int;
        (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
        (*h).param.rc.i_rc_method = X264_RC_ABR;
        (*h).param.rc.f_vbv_buffer_init = 1.0f32;
        (*h).param.rc.b_filler = 1 as c_int;
        (*h).param.i_cqm_preset = X264_CQM_CUSTOM;
        memcpy(
            (*h).param.cqm_4iy.as_mut_ptr() as *mut c_void,
            avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_4iy as *const c_void,
            size_of::<[uint8_t; 16]>() as size_t,
        );
        memcpy(
            (*h).param.cqm_4ic.as_mut_ptr() as *mut c_void,
            avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_4ic as *const c_void,
            size_of::<[uint8_t; 16]>() as size_t,
        );
        memcpy(
            (*h).param.cqm_8iy.as_mut_ptr() as *mut c_void,
            avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_8iy as *const c_void,
            size_of::<[uint8_t; 64]>() as size_t,
        );
        if (*h).param.i_avcintra_flavor == X264_AVCINTRA_FLAVOR_SONY {
            (*h).param.i_slice_count = 8 as c_int;
            if (*h).param.b_sliced_threads != 0 {
                (*h).param.i_threads = (*h).param.i_slice_count;
            }
        } else {
            (*h).param.i_slice_max_mbs = ((*h).param.width as c_int + 15 as c_int) / 16 as c_int
                * (((*h).param.height as c_int + 15 as c_int) / 16 as c_int)
                / 10 as c_int;
            (*h).param.i_slice_max_size = 0 as c_int;
            if (*h).param.b_sliced_threads != 0 {
                if res != 0 {
                    (*h).param.i_threads = if (2 as c_int) < (*h).param.i_threads {
                        2 as c_int
                    } else {
                        (*h).param.i_threads
                    };
                } else {
                    (*h).param.i_threads = if (5 as c_int) < (*h).param.i_threads {
                        5 as c_int
                    } else {
                        (*h).param.i_threads
                    };
                    if (*h).param.i_threads < 5 as c_int {
                        (*h).param.i_threads = 1 as c_int;
                    }
                }
            }
            (*h).param.rc.i_qp_min =
                if (*h).param.rc.i_qp_min > 6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int {
                    (*h).param.rc.i_qp_min
                } else {
                    6 as c_int * (10 as c_int - 8 as c_int) + 1 as c_int
                };
        }
        if type_0 != 0 {
            (*h).param.vui.i_sar_height = 1 as c_int;
            (*h).param.vui.i_sar_width = (*h).param.vui.i_sar_height;
        } else {
            (*h).param.vui.i_sar_width = 4 as c_int;
            (*h).param.vui.i_sar_height = 3 as c_int;
        }
    }
    (*h).param.rc.f_rf_constant = x264_clip3f(
        (*h).param.rc.f_rf_constant as c_double,
        -QP_BD_OFFSET as c_double,
        51 as c_int as c_double,
    ) as c_float;
    (*h).param.rc.f_rf_constant_max = x264_clip3f(
        (*h).param.rc.f_rf_constant_max as c_double,
        -QP_BD_OFFSET as c_double,
        51 as c_int as c_double,
    ) as c_float;
    (*h).param.rc.i_qp_constant = x264_clip3((*h).param.rc.i_qp_constant, -1, QP_MAX);
    (*h).param.analyse.i_subpel_refine =
        x264_clip3((*h).param.analyse.i_subpel_refine, 0 as c_int, 11 as c_int);
    (*h).param.rc.f_ip_factor =
        x264_clip3f((*h).param.rc.f_ip_factor as c_double, 0.01f64, 10.0f64) as c_float;
    (*h).param.rc.f_pb_factor =
        x264_clip3f((*h).param.rc.f_pb_factor as c_double, 0.01f64, 10.0f64) as c_float;
    if (*h).param.rc.i_rc_method == X264_RC_CRF {
        (*h).param.rc.i_qp_constant =
            ((*h).param.rc.f_rf_constant + QP_BD_OFFSET as c_float) as c_int;
        (*h).param.rc.i_bitrate = 0 as c_int;
    }
    if b_open != 0
        && ((*h).param.rc.i_rc_method == X264_RC_CQP || (*h).param.rc.i_rc_method == X264_RC_CRF)
        && (*h).param.rc.i_qp_constant == 0 as c_int
    {
        (*h).mb.b_lossless = 1 as c_int;
        (*h).param.i_cqm_preset = X264_CQM_FLAT;
        (*h).param.psz_cqm_file = 0 as *mut c_char;
        (*h).param.rc.i_rc_method = X264_RC_CQP;
        (*h).param.rc.f_ip_factor = 1 as c_int as c_float;
        (*h).param.rc.f_pb_factor = 1 as c_int as c_float;
        (*h).param.analyse.b_psnr = 0 as c_int;
        (*h).param.analyse.b_ssim = 0 as c_int;
        (*h).param.analyse.i_chroma_qp_offset = 0 as c_int;
        (*h).param.analyse.i_trellis = 0 as c_int;
        (*h).param.analyse.b_fast_pskip = 0 as c_int;
        (*h).param.analyse.i_noise_reduction = 0 as c_int;
        (*h).param.analyse.b_psy = 0 as c_int;
        (*h).param.i_bframe = 0 as c_int;
        if (*h).param.b_cabac == 0 && (*h).param.analyse.i_subpel_refine < 6 as c_int {
            (*h).param.analyse.b_transform_8x8 = 0 as c_int;
        }
    }
    if (*h).param.rc.i_rc_method == X264_RC_CQP {
        let mut qp_p: c_float = (*h).param.rc.i_qp_constant as c_float;
        let mut qp_i: c_float = qp_p - 6 as c_int as c_float * log2f((*h).param.rc.f_ip_factor);
        let mut qp_b: c_float = qp_p + 6 as c_int as c_float * log2f((*h).param.rc.f_pb_factor);
        if qp_p < 0 as c_int as c_float {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"qp not specified\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        (*h).param.rc.i_qp_min = x264_clip3(
            (if qp_p < (if qp_i < qp_b { qp_i } else { qp_b }) {
                qp_p
            } else if qp_i < qp_b {
                qp_i
            } else {
                qp_b
            }) as c_int,
            0 as c_int,
            QP_MAX,
        );
        (*h).param.rc.i_qp_max = x264_clip3(
            ((if qp_p > (if qp_i > qp_b { qp_i } else { qp_b }) {
                qp_p
            } else {
                if qp_i > qp_b {
                    qp_i
                } else {
                    qp_b
                }
            }) as c_double
                + 0.999f64) as c_int,
            0 as c_int,
            QP_MAX,
        );
        (*h).param.rc.i_aq_mode = 0 as c_int;
        (*h).param.rc.b_mb_tree = 0 as c_int;
        (*h).param.rc.i_bitrate = 0 as c_int;
    }
    (*h).param.rc.i_qp_max = x264_clip3((*h).param.rc.i_qp_max, 0 as c_int, QP_MAX);
    (*h).param.rc.i_qp_min = x264_clip3((*h).param.rc.i_qp_min, 0 as c_int, (*h).param.rc.i_qp_max);
    (*h).param.rc.i_qp_step = x264_clip3((*h).param.rc.i_qp_step, 2 as c_int, QP_MAX);
    (*h).param.rc.i_bitrate = x264_clip3((*h).param.rc.i_bitrate, 0 as c_int, 2000000 as c_int);
    if (*h).param.rc.i_rc_method == X264_RC_ABR && (*h).param.rc.i_bitrate == 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"bitrate not specified\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*h).param.rc.i_vbv_buffer_size = x264_clip3(
        (*h).param.rc.i_vbv_buffer_size,
        0 as c_int,
        2000000 as c_int,
    );
    (*h).param.rc.i_vbv_max_bitrate = x264_clip3(
        (*h).param.rc.i_vbv_max_bitrate,
        0 as c_int,
        2000000 as c_int,
    );
    (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
        (*h).param.rc.f_vbv_buffer_init as c_double,
        0 as c_int as c_double,
        2000000 as c_int as c_double,
    ) as c_float;
    if (*h).param.rc.i_vbv_buffer_size != 0 {
        if (*h).param.rc.i_rc_method == X264_RC_CQP {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV is incompatible with constant QP, ignored.\n\0" as *const u8 as *const c_char,
            );
            (*h).param.rc.i_vbv_max_bitrate = 0 as c_int;
            (*h).param.rc.i_vbv_buffer_size = 0 as c_int;
        } else if (*h).param.rc.i_vbv_max_bitrate == 0 as c_int {
            if (*h).param.rc.i_rc_method == X264_RC_ABR {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"VBV maxrate unspecified, assuming CBR\n\0" as *const u8 as *const c_char,
                );
                (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
            } else {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"VBV bufsize set but maxrate unspecified, ignored\n\0" as *const u8
                        as *const c_char,
                );
                (*h).param.rc.i_vbv_buffer_size = 0 as c_int;
            }
        } else if (*h).param.rc.i_vbv_max_bitrate < (*h).param.rc.i_bitrate
            && (*h).param.rc.i_rc_method == X264_RC_ABR
        {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"max bitrate less than average bitrate, assuming CBR\n\0" as *const u8
                    as *const c_char,
            );
            (*h).param.rc.i_bitrate = (*h).param.rc.i_vbv_max_bitrate;
        }
    } else if (*h).param.rc.i_vbv_max_bitrate != 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"VBV maxrate specified, but no bufsize, ignored\n\0" as *const u8 as *const c_char,
        );
        (*h).param.rc.i_vbv_max_bitrate = 0 as c_int;
    }
    (*h).param.i_slice_max_size = if (*h).param.i_slice_max_size > 0 as c_int {
        (*h).param.i_slice_max_size
    } else {
        0 as c_int
    };
    (*h).param.i_slice_max_mbs = if (*h).param.i_slice_max_mbs > 0 as c_int {
        (*h).param.i_slice_max_mbs
    } else {
        0 as c_int
    };
    (*h).param.i_slice_min_mbs = if (*h).param.i_slice_min_mbs > 0 as c_int {
        (*h).param.i_slice_min_mbs
    } else {
        0 as c_int
    };
    if (*h).param.i_slice_max_mbs != 0 {
        (*h).param.i_slice_min_mbs =
            if (*h).param.i_slice_min_mbs < (*h).param.i_slice_max_mbs / 2 as c_int {
                (*h).param.i_slice_min_mbs
            } else {
                (*h).param.i_slice_max_mbs / 2 as c_int
            };
    } else if (*h).param.i_slice_max_size == 0 {
        (*h).param.i_slice_min_mbs = 0 as c_int;
    }
    if (*h).param.b_interlaced != 0 && (*h).param.i_slice_min_mbs != 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"interlace + slice-min-mbs is not implemented\n\0" as *const u8 as *const c_char,
        );
        (*h).param.i_slice_min_mbs = 0 as c_int;
    }
    let mut mb_width: c_int = ((*h).param.width as c_int + 15 as c_int) / 16 as c_int;
    if (*h).param.i_slice_min_mbs > mb_width {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"slice-min-mbs > row mb size (%d) not implemented\n\0" as *const u8 as *const c_char,
            mb_width,
        );
        (*h).param.i_slice_min_mbs = mb_width;
    }
    let mut max_slices: c_int = ((*h).param.height as c_int
        + (((16 as c_int) << (*h).param.b_interlaced) - 1 as c_int))
        / ((16 as c_int) << (*h).param.b_interlaced);
    if (*h).param.b_sliced_threads != 0 {
        (*h).param.i_slice_count = x264_clip3((*h).param.i_threads, 0 as c_int, max_slices);
    } else {
        (*h).param.i_slice_count = x264_clip3((*h).param.i_slice_count, 0 as c_int, max_slices);
        if (*h).param.i_slice_max_mbs != 0 || (*h).param.i_slice_max_size != 0 {
            (*h).param.i_slice_count = 0 as c_int;
        }
    }
    if (*h).param.i_slice_count_max > 0 as c_int {
        (*h).param.i_slice_count_max = if (*h).param.i_slice_count > (*h).param.i_slice_count_max {
            (*h).param.i_slice_count
        } else {
            (*h).param.i_slice_count_max
        };
    }
    if (*h).param.b_bluray_compat != 0 {
        (*h).param.bframe_pyramid = match (*h).param.bframe_pyramid {
            BPyramid::Normal => BPyramid::Strict,
            other => other,
        };
        (*h).param.i_bframe = if (*h).param.i_bframe < 3 as c_int {
            (*h).param.i_bframe
        } else {
            3 as c_int
        };
        (*h).param.b_aud = 1 as c_int;
        (*h).param.i_nal_hrd = if (*h).param.i_nal_hrd > 1 as c_int {
            (*h).param.i_nal_hrd
        } else {
            1 as c_int
        };
        (*h).param.i_slice_max_size = 0 as c_int;
        (*h).param.i_slice_max_mbs = 0 as c_int;
        (*h).param.b_intra_refresh = 0 as c_int;
        (*h).param.i_frame_reference = if (*h).param.i_frame_reference < 6 as c_int {
            (*h).param.i_frame_reference
        } else {
            6 as c_int
        };
        (*h).param.i_dpb_size = if (*h).param.i_dpb_size < 6 as c_int {
            (*h).param.i_dpb_size
        } else {
            6 as c_int
        };
        (*h).param.i_keyint_min = 1 as c_int;
        (*h).param.analyse.i_weighted_pred = if (*h).param.analyse.i_weighted_pred < 1 as c_int {
            (*h).param.analyse.i_weighted_pred
        } else {
            1 as c_int
        };
        if (*h).param.b_fake_interlaced != 0 {
            (*h).param.b_pic_struct = 1 as c_int;
        }
    }
    (*h).param.i_frame_reference =
        x264_clip3((*h).param.i_frame_reference, 1 as c_int, X264_REF_MAX);
    (*h).param.i_dpb_size = x264_clip3((*h).param.i_dpb_size, 1 as c_int, X264_REF_MAX);
    if (*h).param.i_scenecut_threshold < 0 as c_int {
        (*h).param.i_scenecut_threshold = 0 as c_int;
    }
    (*h).param.analyse.i_direct_mv_pred = x264_clip3(
        (*h).param.analyse.i_direct_mv_pred,
        X264_DIRECT_PRED_NONE,
        X264_DIRECT_PRED_AUTO,
    );
    if (*h).param.analyse.i_subpel_refine == 0
        && (*h).param.analyse.i_direct_mv_pred > X264_DIRECT_PRED_SPATIAL
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"subme=0 + direct=temporal is not supported\n\0" as *const u8 as *const c_char,
        );
        (*h).param.analyse.i_direct_mv_pred = X264_DIRECT_PRED_SPATIAL;
    }
    (*h).param.i_bframe = x264_clip3(
        (*h).param.i_bframe,
        0 as c_int,
        if (16 as c_int) < (*h).param.i_keyint_max - 1 as c_int {
            16 as c_int
        } else {
            (*h).param.i_keyint_max - 1 as c_int
        },
    );
    (*h).param.i_bframe_bias = x264_clip3((*h).param.i_bframe_bias, -(90 as c_int), 100 as c_int);
    if (*h).param.i_bframe <= 1 as c_int {
        (*h).param.bframe_pyramid = BPyramid::None;
    }
    (*h).param.i_bframe_adaptive = x264_clip3(
        (*h).param.i_bframe_adaptive,
        X264_B_ADAPT_NONE,
        X264_B_ADAPT_TRELLIS,
    );
    if (*h).param.i_bframe == 0 {
        (*h).param.i_bframe_adaptive = X264_B_ADAPT_NONE;
        (*h).param.analyse.i_direct_mv_pred = 0 as c_int;
        (*h).param.analyse.b_weighted_bipred = 0 as c_int;
        (*h).param.b_open_gop = 0 as c_int;
    }
    if (*h).param.b_intra_refresh != 0 && (*h).param.bframe_pyramid == BPyramid::Normal {
        warn!("b-pyramid normal + intra-refresh is not supported");
        (*h).param.bframe_pyramid = BPyramid::Strict;
    }
    if (*h).param.b_intra_refresh != 0
        && ((*h).param.i_frame_reference > 1 as c_int || (*h).param.i_dpb_size > 1 as c_int)
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"ref > 1 + intra-refresh is not supported\n\0" as *const u8 as *const c_char,
        );
        (*h).param.i_frame_reference = 1 as c_int;
        (*h).param.i_dpb_size = 1 as c_int;
    }
    if (*h).param.b_intra_refresh != 0 && (*h).param.b_open_gop != 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"intra-refresh is not compatible with open-gop\n\0" as *const u8 as *const c_char,
        );
        (*h).param.b_open_gop = 0 as c_int;
    }
    if (*h).param.i_fps_num == 0 || (*h).param.i_fps_den == 0 {
        (*h).param.i_fps_num = 25 as uint32_t;
        (*h).param.i_fps_den = 1 as uint32_t;
    }
    let mut fps: c_float = (*h).param.i_fps_num as c_float / (*h).param.i_fps_den as c_float;
    if (*h).param.i_keyint_min == X264_KEYINT_MIN_AUTO {
        (*h).param.i_keyint_min = if ((*h).param.i_keyint_max / 10 as c_int) < fps as c_int {
            (*h).param.i_keyint_max / 10 as c_int
        } else {
            fps as c_int
        };
    }
    (*h).param.i_keyint_min = x264_clip3(
        (*h).param.i_keyint_min,
        1 as c_int,
        (*h).param.i_keyint_max / 2 as c_int + 1 as c_int,
    );
    (*h).param.rc.i_lookahead =
        x264_clip3((*h).param.rc.i_lookahead, 0 as c_int, X264_LOOKAHEAD_MAX);
    let mut maxrate: c_int = if (*h).param.rc.i_vbv_max_bitrate > (*h).param.rc.i_bitrate {
        (*h).param.rc.i_vbv_max_bitrate
    } else {
        (*h).param.rc.i_bitrate
    };
    let mut bufsize: c_float = if maxrate != 0 {
        (*h).param.rc.i_vbv_buffer_size as c_float / maxrate as c_float
    } else {
        0 as c_int as c_float
    };
    (*h).param.rc.i_lookahead = (if ((*h).param.rc.i_lookahead as c_float)
        < (if (*h).param.i_keyint_max as c_float > bufsize * fps {
            (*h).param.i_keyint_max as c_float
        } else {
            bufsize * fps
        }) {
        (*h).param.rc.i_lookahead as c_float
    } else if (*h).param.i_keyint_max as c_float > bufsize * fps {
        (*h).param.i_keyint_max as c_float
    } else {
        bufsize * fps
    }) as c_int;
    if (*h).param.i_timebase_num == 0
        || (*h).param.i_timebase_den == 0
        || !((*h).param.b_vfr_input != 0 || (*h).param.b_pulldown != 0)
    {
        (*h).param.i_timebase_num = (*h).param.i_fps_den;
        (*h).param.i_timebase_den = (*h).param.i_fps_num;
    }
    (*h).param.rc.f_qcompress =
        x264_clip3f((*h).param.rc.f_qcompress as c_double, 0.0f64, 1.0f64) as c_float;
    if (*h).param.i_keyint_max == 1 as c_int || (*h).param.rc.f_qcompress == 1 as c_int as c_float {
        (*h).param.rc.b_mb_tree = 0 as c_int;
    }
    if (*h).param.b_intra_refresh == 0
        && (*h).param.i_keyint_max != X264_KEYINT_MAX_INFINITE
        && (*h).param.rc.i_lookahead == 0
        && (*h).param.rc.b_mb_tree != 0
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"lookaheadless mb-tree requires intra refresh or infinite keyint\n\0" as *const u8
                as *const c_char,
        );
        (*h).param.rc.b_mb_tree = 0 as c_int;
    }
    if b_open != 0 && (*h).param.rc.b_stat_read != 0 {
        (*h).param.rc.i_lookahead = 0 as c_int;
    }
    if (*h).param.i_sync_lookahead < 0 as c_int {
        (*h).param.i_sync_lookahead = (*h).param.i_bframe + 1 as c_int;
    }
    (*h).param.i_sync_lookahead = if (*h).param.i_sync_lookahead < 250 as c_int {
        (*h).param.i_sync_lookahead
    } else {
        250 as c_int
    };
    if (*h).param.rc.b_stat_read != 0 || (*h).i_thread_frames == 1 as c_int {
        (*h).param.i_sync_lookahead = 0 as c_int;
    }
    (*h).param.i_deblocking_filter_alphac0 = x264_clip3(
        (*h).param.i_deblocking_filter_alphac0,
        -(6 as c_int),
        6 as c_int,
    );
    (*h).param.i_deblocking_filter_beta = x264_clip3(
        (*h).param.i_deblocking_filter_beta,
        -(6 as c_int),
        6 as c_int,
    );
    (*h).param.analyse.i_luma_deadzone[0] = x264_clip3(
        (*h).param.analyse.i_luma_deadzone[0],
        0 as c_int,
        32 as c_int,
    );
    (*h).param.analyse.i_luma_deadzone[1] = x264_clip3(
        (*h).param.analyse.i_luma_deadzone[1],
        0 as c_int,
        32 as c_int,
    );
    (*h).param.i_cabac_init_idc = x264_clip3((*h).param.i_cabac_init_idc, 0 as c_int, 2 as c_int);
    if (*h).param.i_cqm_preset < X264_CQM_FLAT || (*h).param.i_cqm_preset > X264_CQM_CUSTOM {
        (*h).param.i_cqm_preset = X264_CQM_FLAT;
    }
    if (*h).param.analyse.i_me_method < X264_ME_DIA || (*h).param.analyse.i_me_method > X264_ME_TESA
    {
        (*h).param.analyse.i_me_method = X264_ME_HEX;
    }
    (*h).param.analyse.i_me_range =
        x264_clip3((*h).param.analyse.i_me_range, 4 as c_int, 1024 as c_int);
    if (*h).param.analyse.i_me_range > 16 as c_int && (*h).param.analyse.i_me_method <= X264_ME_HEX
    {
        (*h).param.analyse.i_me_range = 16 as c_int;
    }
    if (*h).param.analyse.i_me_method == X264_ME_TESA
        && ((*h).mb.b_lossless != 0 || (*h).param.analyse.i_subpel_refine <= 1 as c_int)
    {
        (*h).param.analyse.i_me_method = X264_ME_ESA;
    }
    (*h).param.analyse.b_mixed_references = ((*h).param.analyse.b_mixed_references != 0
        && (*h).param.i_frame_reference > 1 as c_int)
        as c_int;
    (*h).param.analyse.inter &= X264_ANALYSE_PSUB16x16
        | X264_ANALYSE_PSUB8x8
        | X264_ANALYSE_BSUB16x16
        | X264_ANALYSE_I4x4
        | X264_ANALYSE_I8x8;
    (*h).param.analyse.intra &= X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8;
    if (*h).param.analyse.inter & X264_ANALYSE_PSUB16x16 == 0 {
        (*h).param.analyse.inter &= !X264_ANALYSE_PSUB8x8;
    }
    if (*h).param.analyse.b_transform_8x8 == 0 {
        (*h).param.analyse.inter &= !X264_ANALYSE_I8x8;
        (*h).param.analyse.intra &= !X264_ANALYSE_I8x8;
    }
    (*h).param.analyse.i_trellis = x264_clip3((*h).param.analyse.i_trellis, 0 as c_int, 2 as c_int);
    (*h).param.rc.i_aq_mode = x264_clip3((*h).param.rc.i_aq_mode, 0 as c_int, 3 as c_int);
    (*h).param.rc.f_aq_strength = x264_clip3f(
        (*h).param.rc.f_aq_strength as c_double,
        0 as c_int as c_double,
        3 as c_int as c_double,
    ) as c_float;
    if (*h).param.rc.f_aq_strength == 0 as c_int as c_float {
        (*h).param.rc.i_aq_mode = 0 as c_int;
    }
    if (*h).param.i_log_level < X264_LOG_INFO {
        (*h).param.analyse.b_psnr = 0 as c_int;
        (*h).param.analyse.b_ssim = 0 as c_int;
    }
    if b_open != 0 && ((*h).param.analyse.b_psnr != 0 || (*h).param.analyse.b_ssim != 0) {
        let mut s: *mut c_char = 0 as *mut c_char;
        if (*h).param.analyse.b_psy != 0 {
            s = (if (*h).param.analyse.b_psnr != 0 {
                b"psnr\0" as *const u8 as *const c_char
            } else {
                b"ssim\0" as *const u8 as *const c_char
            }) as *mut c_char;
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"--%s used with psy on: results will be invalid!\n\0" as *const u8
                    as *const c_char,
                s,
            );
        } else if (*h).param.rc.i_aq_mode == 0 && (*h).param.analyse.b_ssim != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"--ssim used with AQ off: results will be invalid!\n\0" as *const u8
                    as *const c_char,
            );
            s = b"ssim\0" as *const u8 as *const c_char as *mut c_char;
        } else if (*h).param.rc.i_aq_mode != 0 && (*h).param.analyse.b_psnr != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"--psnr used with AQ on: results will be invalid!\n\0" as *const u8
                    as *const c_char,
            );
            s = b"psnr\0" as *const u8 as *const c_char as *mut c_char;
        }
        if !s.is_null() {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"--tune %s should be used if attempting to benchmark %s!\n\0" as *const u8
                    as *const c_char,
                s,
                s,
            );
        }
    }
    if (*h).param.analyse.b_psy == 0 {
        (*h).param.analyse.f_psy_rd = 0 as c_int as c_float;
        (*h).param.analyse.f_psy_trellis = 0 as c_int as c_float;
    }
    (*h).param.analyse.f_psy_rd = x264_clip3f(
        (*h).param.analyse.f_psy_rd as c_double,
        0 as c_int as c_double,
        10 as c_int as c_double,
    ) as c_float;
    (*h).param.analyse.f_psy_trellis = x264_clip3f(
        (*h).param.analyse.f_psy_trellis as c_double,
        0 as c_int as c_double,
        10 as c_int as c_double,
    ) as c_float;
    (*h).mb.i_psy_rd = if (*h).param.analyse.i_subpel_refine >= 6 as c_int {
        (((*h).param.analyse.f_psy_rd * ((1 as c_int) << 8 as c_int) as c_float) as c_double
            + 0.5f64) as c_int
    } else {
        0 as c_int
    };
    (*h).mb.i_psy_trellis = if (*h).param.analyse.i_trellis != 0 {
        (((*h).param.analyse.f_psy_trellis / 4 as c_int as c_float
            * ((1 as c_int) << 8 as c_int) as c_float) as c_double
            + 0.5f64) as c_int
    } else {
        0 as c_int
    };
    (*h).param.analyse.i_chroma_qp_offset = x264_clip3(
        (*h).param.analyse.i_chroma_qp_offset,
        -(32 as c_int),
        32 as c_int,
    );
    if b_open != 0
        && i_csp >= X264_CSP_I444
        && i_csp < X264_CSP_BGR
        && (*h).param.analyse.b_psy != 0
    {
        (*h).param.analyse.i_chroma_qp_offset += 6 as c_int;
    }
    if b_open != 0 && (*h).mb.i_psy_rd != 0 && (*h).param.i_avcintra_class == 0 {
        (*h).param.analyse.i_chroma_qp_offset -=
            if ((*h).param.analyse.f_psy_rd as c_double) < 0.25f64 {
                1 as c_int
            } else {
                2 as c_int
            };
    }
    if b_open != 0 && (*h).mb.i_psy_trellis != 0 && (*h).param.i_avcintra_class == 0 {
        (*h).param.analyse.i_chroma_qp_offset -=
            if ((*h).param.analyse.f_psy_trellis as c_double) < 0.25f64 {
                1 as c_int
            } else {
                2 as c_int
            };
    }
    (*h).param.analyse.i_chroma_qp_offset = x264_clip3(
        (*h).param.analyse.i_chroma_qp_offset,
        -(12 as c_int),
        12 as c_int,
    );
    if (*h).param.rc.i_aq_mode == 0 && (*h).param.rc.b_mb_tree != 0 {
        (*h).param.rc.i_aq_mode = 1 as c_int;
        (*h).param.rc.f_aq_strength = 0 as c_int as c_float;
    }
    (*h).param.analyse.i_noise_reduction = x264_clip3(
        (*h).param.analyse.i_noise_reduction,
        0 as c_int,
        (1 as c_int) << 16 as c_int,
    );
    if (*h).param.analyse.i_subpel_refine >= 10 as c_int
        && ((*h).param.analyse.i_trellis != 2 as c_int || (*h).param.rc.i_aq_mode == 0)
    {
        (*h).param.analyse.i_subpel_refine = 9 as c_int;
    }
    if b_open != 0 {
        let mut l: *const x264_level_t = x264_levels.as_ptr();
        if (*h).param.i_level_idc < 0 as c_int {
            let mut maxrate_bak: c_int = (*h).param.rc.i_vbv_max_bitrate;
            if (*h).param.rc.i_rc_method == X264_RC_ABR
                && (*h).param.rc.i_vbv_buffer_size <= 0 as c_int
            {
                (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate * 2 as c_int;
            }
            x264_10_sps_init((*h).sps.as_mut_ptr(), (*h).param.i_sps_id, &mut (*h).param);
            loop {
                (*h).param.i_level_idc = (*l).level_idc as c_int;
                if !((*l.offset(1)).level_idc as c_int != 0
                    && x264_10_validate_levels(h, 0 as c_int) != 0
                    && {
                        let fresh0 = l;
                        l = l.offset(1);
                        !fresh0.is_null()
                    })
                {
                    break;
                }
            }
            (*h).param.rc.i_vbv_max_bitrate = maxrate_bak;
        } else {
            while (*l).level_idc as c_int != 0 && (*l).level_idc as c_int != (*h).param.i_level_idc
            {
                l = l.offset(1);
            }
            if (*l).level_idc as c_int == 0 as c_int {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"invalid level_idc: %d\n\0" as *const u8 as *const c_char,
                    (*h).param.i_level_idc,
                );
                return -1;
            }
        }
        if (*h).param.analyse.i_mv_range <= 0 as c_int {
            (*h).param.analyse.i_mv_range = (*l).mv_range as c_int >> (*h).param.b_interlaced;
        } else {
            (*h).param.analyse.i_mv_range = x264_clip3(
                (*h).param.analyse.i_mv_range,
                32 as c_int,
                8192 as c_int >> (*h).param.b_interlaced,
            );
        }
    }
    (*h).param.analyse.i_weighted_pred = x264_clip3(
        (*h).param.analyse.i_weighted_pred,
        X264_WEIGHTP_NONE,
        X264_WEIGHTP_SMART,
    );
    if (*h).param.i_lookahead_threads == X264_THREADS_AUTO {
        if (*h).param.b_sliced_threads != 0 {
            (*h).param.i_lookahead_threads = (*h).param.i_threads;
        } else {
            let mut badapt: c_int = ((*h).param.i_bframe_adaptive == X264_B_ADAPT_TRELLIS) as c_int;
            let mut subme: c_int =
                (if ((*h).param.analyse.i_subpel_refine / 3 as c_int) < 3 as c_int {
                    (*h).param.analyse.i_subpel_refine / 3 as c_int
                } else {
                    3 as c_int
                }) + ((*h).param.analyse.i_subpel_refine > 1 as c_int) as c_int;
            let mut bframes: c_int =
                if (((*h).param.i_bframe - 1 as c_int) / 3 as c_int) < 3 as c_int {
                    ((*h).param.i_bframe - 1 as c_int) / 3 as c_int
                } else {
                    3 as c_int
                };
            static mut lookahead_thread_div: [[[uint8_t; 4]; 5]; 2] = [
                [
                    [
                        6 as c_int as uint8_t,
                        6 as c_int as uint8_t,
                        6 as c_int as uint8_t,
                        6 as c_int as uint8_t,
                    ],
                    [
                        3 as c_int as uint8_t,
                        3 as c_int as uint8_t,
                        3 as c_int as uint8_t,
                        3 as c_int as uint8_t,
                    ],
                    [
                        4 as c_int as uint8_t,
                        4 as c_int as uint8_t,
                        4 as c_int as uint8_t,
                        4 as c_int as uint8_t,
                    ],
                    [
                        6 as c_int as uint8_t,
                        6 as c_int as uint8_t,
                        6 as c_int as uint8_t,
                        6 as c_int as uint8_t,
                    ],
                    [
                        12 as c_int as uint8_t,
                        12 as c_int as uint8_t,
                        12 as c_int as uint8_t,
                        12 as c_int as uint8_t,
                    ],
                ],
                [
                    [
                        3 as c_int as uint8_t,
                        2 as c_int as uint8_t,
                        1 as c_int as uint8_t,
                        1 as c_int as uint8_t,
                    ],
                    [
                        2 as c_int as uint8_t,
                        1 as c_int as uint8_t,
                        1 as c_int as uint8_t,
                        1 as c_int as uint8_t,
                    ],
                    [
                        4 as c_int as uint8_t,
                        3 as c_int as uint8_t,
                        2 as c_int as uint8_t,
                        1 as c_int as uint8_t,
                    ],
                    [
                        6 as c_int as uint8_t,
                        4 as c_int as uint8_t,
                        3 as c_int as uint8_t,
                        2 as c_int as uint8_t,
                    ],
                    [
                        12 as c_int as uint8_t,
                        9 as c_int as uint8_t,
                        6 as c_int as uint8_t,
                        4 as c_int as uint8_t,
                    ],
                ],
            ];
            (*h).param.i_lookahead_threads = (*h).param.i_threads
                / lookahead_thread_div[badapt as usize][subme as usize][bframes as usize] as c_int;
            (*h).param.i_lookahead_threads =
                if (*h).param.i_lookahead_threads < (*h).param.height as c_int / 128 as c_int {
                    (*h).param.i_lookahead_threads
                } else {
                    (*h).param.height as c_int / 128 as c_int
                };
        }
    }
    (*h).param.i_lookahead_threads = x264_clip3(
        (*h).param.i_lookahead_threads,
        1 as c_int,
        if max_sliced_threads < 16 as c_int {
            max_sliced_threads
        } else {
            16 as c_int
        },
    );
    if (*h).param.b_interlaced != 0 {
        if (*h).param.analyse.i_me_method >= X264_ME_ESA {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"interlace + me=esa is not implemented\n\0" as *const u8 as *const c_char,
            );
            (*h).param.analyse.i_me_method = X264_ME_UMH;
        }
        if (*h).param.analyse.i_weighted_pred > 0 as c_int {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"interlace + weightp is not implemented\n\0" as *const u8 as *const c_char,
            );
            (*h).param.analyse.i_weighted_pred = X264_WEIGHTP_NONE;
        }
    }
    if (*h).param.analyse.i_weighted_pred == 0
        && (*h).param.rc.b_mb_tree != 0
        && (*h).param.analyse.b_psy != 0
    {
        (*h).param.analyse.i_weighted_pred = X264_WEIGHTP_FAKE;
    }
    if (*h).i_thread_frames > 1 as c_int {
        let mut r: c_int = (*h).param.analyse.i_mv_range_thread;
        let mut r2: c_int = 0;
        if r <= 0 as c_int {
            let mut max_range: c_int = ((*h).param.height as c_int + X264_THREAD_HEIGHT)
                / (*h).i_thread_frames
                - X264_THREAD_HEIGHT;
            r = max_range / 2 as c_int;
        }
        r = if r > (*h).param.analyse.i_me_range {
            r
        } else {
            (*h).param.analyse.i_me_range
        };
        r = if r < (*h).param.analyse.i_mv_range {
            r
        } else {
            (*h).param.analyse.i_mv_range
        };
        r2 = (r & !(15 as c_int)) + (-X264_THREAD_HEIGHT & 15 as c_int);
        if r2 < r {
            r2 += 16 as c_int;
        }
        x264_10_log(
            h,
            X264_LOG_DEBUG,
            b"using mv_range_thread = %d\n\0" as *const u8 as *const c_char,
            r2,
        );
        (*h).param.analyse.i_mv_range_thread = r2;
    }
    if (*h).param.rc.f_rate_tolerance < 0 as c_int as c_float {
        (*h).param.rc.f_rate_tolerance = 0 as c_int as c_float;
    }
    if (*h).param.rc.f_qblur < 0 as c_int as c_float {
        (*h).param.rc.f_qblur = 0 as c_int as c_float;
    }
    if (*h).param.rc.f_complexity_blur < 0 as c_int as c_float {
        (*h).param.rc.f_complexity_blur = 0 as c_int as c_float;
    }
    (*h).param.i_sps_id &= 31 as c_int;
    (*h).param.i_nal_hrd = x264_clip3((*h).param.i_nal_hrd, X264_NAL_HRD_NONE, X264_NAL_HRD_CBR);
    if (*h).param.i_nal_hrd != 0 && (*h).param.rc.i_vbv_buffer_size == 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"NAL HRD parameters require VBV parameters\n\0" as *const u8 as *const c_char,
        );
        (*h).param.i_nal_hrd = X264_NAL_HRD_NONE;
    }
    if (*h).param.i_nal_hrd == X264_NAL_HRD_CBR
        && ((*h).param.rc.i_bitrate != (*h).param.rc.i_vbv_max_bitrate
            || (*h).param.rc.i_vbv_max_bitrate == 0)
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"CBR HRD requires constant bitrate\n\0" as *const u8 as *const c_char,
        );
        (*h).param.i_nal_hrd = X264_NAL_HRD_VBR;
    }
    if (*h).param.i_nal_hrd == X264_NAL_HRD_CBR {
        (*h).param.rc.b_filler = 1 as c_int;
    }
    (*h).param.b_cabac = ((*h).param.b_cabac != 0) as c_int;
    (*h).param.b_constrained_intra = ((*h).param.b_constrained_intra != 0) as c_int;
    (*h).param.b_deblocking_filter = ((*h).param.b_deblocking_filter != 0) as c_int;
    (*h).param.b_deterministic = ((*h).param.b_deterministic != 0) as c_int;
    (*h).param.b_sliced_threads = ((*h).param.b_sliced_threads != 0) as c_int;
    (*h).param.b_interlaced = ((*h).param.b_interlaced != 0) as c_int;
    (*h).param.b_intra_refresh = ((*h).param.b_intra_refresh != 0) as c_int;
    (*h).param.b_aud = ((*h).param.b_aud != 0) as c_int;
    (*h).param.b_repeat_headers = ((*h).param.b_repeat_headers != 0) as c_int;
    (*h).param.b_annexb = ((*h).param.b_annexb != 0) as c_int;
    (*h).param.b_vfr_input = ((*h).param.b_vfr_input != 0) as c_int;
    (*h).param.b_pulldown = ((*h).param.b_pulldown != 0) as c_int;
    (*h).param.b_tff = ((*h).param.b_tff != 0) as c_int;
    (*h).param.b_pic_struct = ((*h).param.b_pic_struct != 0) as c_int;
    (*h).param.b_fake_interlaced = ((*h).param.b_fake_interlaced != 0) as c_int;
    (*h).param.b_open_gop = ((*h).param.b_open_gop != 0) as c_int;
    (*h).param.b_bluray_compat = ((*h).param.b_bluray_compat != 0) as c_int;
    (*h).param.b_stitchable = ((*h).param.b_stitchable != 0) as c_int;
    (*h).param.b_full_recon = ((*h).param.b_full_recon != 0) as c_int;
    (*h).param.b_opencl = ((*h).param.b_opencl != 0) as c_int;
    (*h).param.analyse.b_transform_8x8 = ((*h).param.analyse.b_transform_8x8 != 0) as c_int;
    (*h).param.analyse.b_weighted_bipred = ((*h).param.analyse.b_weighted_bipred != 0) as c_int;
    (*h).param.analyse.b_chroma_me = ((*h).param.analyse.b_chroma_me != 0) as c_int;
    (*h).param.analyse.b_mixed_references = ((*h).param.analyse.b_mixed_references != 0) as c_int;
    (*h).param.analyse.b_fast_pskip = ((*h).param.analyse.b_fast_pskip != 0) as c_int;
    (*h).param.analyse.b_dct_decimate = ((*h).param.analyse.b_dct_decimate != 0) as c_int;
    (*h).param.analyse.b_psy = ((*h).param.analyse.b_psy != 0) as c_int;
    (*h).param.analyse.b_psnr = ((*h).param.analyse.b_psnr != 0) as c_int;
    (*h).param.analyse.b_ssim = ((*h).param.analyse.b_ssim != 0) as c_int;
    (*h).param.rc.b_stat_write = ((*h).param.rc.b_stat_write != 0) as c_int;
    (*h).param.rc.b_stat_read = ((*h).param.rc.b_stat_read != 0) as c_int;
    (*h).param.rc.b_mb_tree = ((*h).param.rc.b_mb_tree != 0) as c_int;
    (*h).param.rc.b_filler = ((*h).param.rc.b_filler != 0) as c_int;
    return 0 as c_int;
}
#[c2rust::src_loc = "467:9"]
const MAX_RESOLUTION: u32 = 16384;
#[c2rust::src_loc = "1409:1"]
unsafe extern "C" fn mbcmp_init(mut h: *mut x264_t) {
    let mut satd: c_int =
        ((*h).mb.b_lossless == 0 && (*h).param.analyse.i_subpel_refine > 1 as c_int) as c_int;
    memcpy(
        (*h).pixf.mbcmp.as_mut_ptr() as *mut c_void,
        (if satd != 0 {
            (*h).pixf.satd.as_mut_ptr()
        } else {
            (*h).pixf.sad_aligned.as_mut_ptr()
        }) as *const c_void,
        size_of::<[x264_pixel_cmp_t; 8]>() as size_t,
    );
    memcpy(
        (*h).pixf.mbcmp_unaligned.as_mut_ptr() as *mut c_void,
        (if satd != 0 {
            (*h).pixf.satd.as_mut_ptr()
        } else {
            (*h).pixf.sad.as_mut_ptr()
        }) as *const c_void,
        size_of::<[x264_pixel_cmp_t; 8]>() as size_t,
    );
    (*h).pixf.intra_mbcmp_x3_16x16 = if satd != 0 {
        (*h).pixf.intra_satd_x3_16x16
    } else {
        (*h).pixf.intra_sad_x3_16x16
    };
    (*h).pixf.intra_mbcmp_x3_8x16c = if satd != 0 {
        (*h).pixf.intra_satd_x3_8x16c
    } else {
        (*h).pixf.intra_sad_x3_8x16c
    };
    (*h).pixf.intra_mbcmp_x3_8x8c = if satd != 0 {
        (*h).pixf.intra_satd_x3_8x8c
    } else {
        (*h).pixf.intra_sad_x3_8x8c
    };
    (*h).pixf.intra_mbcmp_x3_8x8 = if satd != 0 {
        (*h).pixf.intra_sa8d_x3_8x8
    } else {
        (*h).pixf.intra_sad_x3_8x8
    };
    (*h).pixf.intra_mbcmp_x3_4x4 = if satd != 0 {
        (*h).pixf.intra_satd_x3_4x4
    } else {
        (*h).pixf.intra_sad_x3_4x4
    };
    (*h).pixf.intra_mbcmp_x9_4x4 = if (*h).param.b_cpu_independent != 0 || (*h).mb.b_lossless != 0 {
        None
    } else if satd != 0 {
        (*h).pixf.intra_satd_x9_4x4
    } else {
        (*h).pixf.intra_sad_x9_4x4
    };
    (*h).pixf.intra_mbcmp_x9_8x8 = if (*h).param.b_cpu_independent != 0 || (*h).mb.b_lossless != 0 {
        None
    } else if satd != 0 {
        (*h).pixf.intra_sa8d_x9_8x8
    } else {
        (*h).pixf.intra_sad_x9_8x8
    };
    satd &= ((*h).param.analyse.i_me_method == X264_ME_TESA) as c_int;
    memcpy(
        (*h).pixf.fpelcmp.as_mut_ptr() as *mut c_void,
        (if satd != 0 {
            (*h).pixf.satd.as_mut_ptr()
        } else {
            (*h).pixf.sad.as_mut_ptr()
        }) as *const c_void,
        size_of::<[x264_pixel_cmp_t; 8]>() as size_t,
    );
    memcpy(
        (*h).pixf.fpelcmp_x3.as_mut_ptr() as *mut c_void,
        (if satd != 0 {
            (*h).pixf.satd_x3.as_mut_ptr()
        } else {
            (*h).pixf.sad_x3.as_mut_ptr()
        }) as *const c_void,
        size_of::<[x264_pixel_cmp_x3_t; 7]>() as size_t,
    );
    memcpy(
        (*h).pixf.fpelcmp_x4.as_mut_ptr() as *mut c_void,
        (if satd != 0 {
            (*h).pixf.satd_x4.as_mut_ptr()
        } else {
            (*h).pixf.sad_x4.as_mut_ptr()
        }) as *const c_void,
        size_of::<[x264_pixel_cmp_x4_t; 7]>() as size_t,
    );
}
#[c2rust::src_loc = "1429:1"]
unsafe extern "C" fn chroma_dsp_init(mut h: *mut x264_t) {
    memcpy(
        (*h).luma2chroma_pixel.as_mut_ptr() as *mut c_void,
        (*x264_luma2chroma_pixel
            .as_ptr()
            .offset((*(*h).sps.as_mut_ptr()).i_chroma_format_idc as isize))
        .as_ptr() as *const c_void,
        size_of::<[uint8_t; 7]>() as size_t,
    );
    match (*(*h).sps.as_mut_ptr()).i_chroma_format_idc {
        0 => {
            (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_400;
        }
        1 => {
            memcpy(
                (*h).predict_chroma.as_mut_ptr() as *mut c_void,
                (*h).predict_8x8c.as_mut_ptr() as *const c_void,
                size_of::<[x264_predict_t; 7]>() as size_t,
            );
            (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_420;
            (*h).loopf.deblock_chroma[0] = (*h).loopf.deblock_h_chroma_420;
            (*h).loopf.deblock_chroma_intra[0] = (*h).loopf.deblock_h_chroma_420_intra;
            (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_chroma_420_mbaff;
            (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_chroma_420_intra_mbaff;
            (*h).pixf.intra_mbcmp_x3_chroma = (*h).pixf.intra_mbcmp_x3_8x8c;
            (*h).quantf.coeff_last[DCT_CHROMA_DC as c_int as usize] = (*h).quantf.coeff_last4;
            (*h).quantf.coeff_level_run[DCT_CHROMA_DC as c_int as usize] =
                (*h).quantf.coeff_level_run4;
        }
        2 => {
            memcpy(
                (*h).predict_chroma.as_mut_ptr() as *mut c_void,
                (*h).predict_8x16c.as_mut_ptr() as *const c_void,
                size_of::<[x264_predict_t; 7]>() as size_t,
            );
            (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_422;
            (*h).loopf.deblock_chroma[0] = (*h).loopf.deblock_h_chroma_422;
            (*h).loopf.deblock_chroma_intra[0] = (*h).loopf.deblock_h_chroma_422_intra;
            (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_chroma_422_mbaff;
            (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_chroma_422_intra_mbaff;
            (*h).pixf.intra_mbcmp_x3_chroma = (*h).pixf.intra_mbcmp_x3_8x16c;
            (*h).quantf.coeff_last[DCT_CHROMA_DC as c_int as usize] = (*h).quantf.coeff_last8;
            (*h).quantf.coeff_level_run[DCT_CHROMA_DC as c_int as usize] =
                (*h).quantf.coeff_level_run8;
        }
        3 => {
            (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_422;
            (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_luma_mbaff;
            (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_luma_intra_mbaff;
        }
        _ => {}
    };
}
#[c2rust::src_loc = "1468:1"]
unsafe extern "C" fn set_aspect_ratio(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
    mut initial: c_int,
) {
    if (*param).vui.i_sar_width > 0 as c_int && (*param).vui.i_sar_height > 0 as c_int {
        let mut i_w: uint32_t = (*param).vui.i_sar_width as uint32_t;
        let mut i_h: uint32_t = (*param).vui.i_sar_height as uint32_t;
        let mut old_w: uint32_t = (*h).param.vui.i_sar_width as uint32_t;
        let mut old_h: uint32_t = (*h).param.vui.i_sar_height as uint32_t;
        x264_reduce_fraction(&mut i_w, &mut i_h);
        while i_w > 65535 as uint32_t || i_h > 65535 as uint32_t {
            i_w = i_w.wrapping_div(2 as uint32_t);
            i_h = i_h.wrapping_div(2 as uint32_t);
        }
        x264_reduce_fraction(&mut i_w, &mut i_h);
        if i_w != old_w || i_h != old_h || initial != 0 {
            (*h).param.vui.i_sar_width = 0 as c_int;
            (*h).param.vui.i_sar_height = 0 as c_int;
            if i_w == 0 as uint32_t || i_h == 0 as uint32_t {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"cannot create valid sample aspect ratio\n\0" as *const u8 as *const c_char,
                );
            } else {
                x264_10_log(
                    h,
                    if initial != 0 {
                        X264_LOG_INFO
                    } else {
                        X264_LOG_DEBUG
                    },
                    b"using SAR=%d/%d\n\0" as *const u8 as *const c_char,
                    i_w,
                    i_h,
                );
                (*h).param.vui.i_sar_width = i_w as c_int;
                (*h).param.vui.i_sar_height = i_h as c_int;
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1507:1"]
unsafe extern "C" fn x264_10_encoder_open(
    mut param: *mut x264_param_t,
    mut api: *mut c_void,
) -> *mut x264_t {
    let mut temp: c_int = 0;
    let mut profile: *const c_char = 0 as *const c_char;
    let mut level: [c_char; 16] = [0; 16];
    static mut subsampling: [*const c_char; 4] = [
        b"4:0:0\0" as *const u8 as *const c_char,
        b"4:2:0\0" as *const u8 as *const c_char,
        b"4:2:2\0" as *const u8 as *const c_char,
        b"4:4:4\0" as *const u8 as *const c_char,
    ];
    let mut current_block: u64;
    let mut h: *mut x264_t = 0 as *mut x264_t;
    let mut buf: [c_char; 1000] = [0; 1000];
    let mut p: *mut c_char = 0 as *mut c_char;
    let mut i_slicetype_length: c_int = 0;
    h = x264_malloc(size_of::<x264_t>() as int64_t) as *mut x264_t;
    if !h.is_null() {
        memset(h as *mut c_void, 0 as c_int, size_of::<x264_t>() as size_t);
        memcpy(
            &mut (*h).param as *mut x264_param_t as *mut c_void,
            param as *const c_void,
            size_of::<x264_param_t>() as size_t,
        );
        (*h).param.opaque = NULL;
        (*h).param.param_free = None;
        if !(*h).param.psz_cqm_file.is_null() {
            (*h).param.psz_cqm_file = x264_param_strdup(&mut (*h).param, (*h).param.psz_cqm_file);
            if (*h).param.psz_cqm_file.is_null() {
                current_block = 17708740041508716982;
            } else {
                current_block = 5399440093318478209;
            }
        } else {
            current_block = 5399440093318478209;
        }
        match current_block {
            17708740041508716982 => {}
            _ => {
                if !(*h).param.psz_dump_yuv.is_null() {
                    (*h).param.psz_dump_yuv =
                        x264_param_strdup(&mut (*h).param, (*h).param.psz_dump_yuv);
                    if (*h).param.psz_dump_yuv.is_null() {
                        current_block = 17708740041508716982;
                    } else {
                        current_block = 8831408221741692167;
                    }
                } else {
                    current_block = 8831408221741692167;
                }
                match current_block {
                    17708740041508716982 => {}
                    _ => {
                        if !(*h).param.rc.psz_stat_out.is_null() {
                            (*h).param.rc.psz_stat_out =
                                x264_param_strdup(&mut (*h).param, (*h).param.rc.psz_stat_out);
                            if (*h).param.rc.psz_stat_out.is_null() {
                                current_block = 17708740041508716982;
                            } else {
                                current_block = 4808432441040389987;
                            }
                        } else {
                            current_block = 4808432441040389987;
                        }
                        match current_block {
                            17708740041508716982 => {}
                            _ => {
                                if !(*h).param.rc.psz_stat_in.is_null() {
                                    (*h).param.rc.psz_stat_in = x264_param_strdup(
                                        &mut (*h).param,
                                        (*h).param.rc.psz_stat_in,
                                    );
                                    if (*h).param.rc.psz_stat_in.is_null() {
                                        current_block = 17708740041508716982;
                                    } else {
                                        current_block = 10043043949733653460;
                                    }
                                } else {
                                    current_block = 10043043949733653460;
                                }
                                match current_block {
                                    17708740041508716982 => {}
                                    _ => {
                                        if !(*h).param.rc.psz_zones.is_null() {
                                            (*h).param.rc.psz_zones = x264_param_strdup(
                                                &mut (*h).param,
                                                (*h).param.rc.psz_zones,
                                            );
                                            if (*h).param.rc.psz_zones.is_null() {
                                                current_block = 17708740041508716982;
                                            } else {
                                                current_block = 14648156034262866959;
                                            }
                                        } else {
                                            current_block = 14648156034262866959;
                                        }
                                        match current_block {
                                            17708740041508716982 => {}
                                            _ => {
                                                if !(*h).param.psz_clbin_file.is_null() {
                                                    (*h).param.psz_clbin_file = x264_param_strdup(
                                                        &mut (*h).param,
                                                        (*h).param.psz_clbin_file,
                                                    );
                                                    if (*h).param.psz_clbin_file.is_null() {
                                                        current_block = 17708740041508716982;
                                                    } else {
                                                        current_block = 652864300344834934;
                                                    }
                                                } else {
                                                    current_block = 652864300344834934;
                                                }
                                                match current_block {
                                                    17708740041508716982 => {}
                                                    _ => {
                                                        if (*param).param_free.is_some() {
                                                            x264_param_cleanup(param);
                                                            (*param).param_free.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                param as *mut c_void
                                                            );
                                                        }
                                                        (*h).api = api;
                                                        if !(validate_parameters(h, 1 as c_int)
                                                            < 0 as c_int)
                                                        {
                                                            if !(*h).param.psz_cqm_file.is_null() {
                                                                if x264_10_cqm_parse_file(
                                                                    h,
                                                                    (*h).param.psz_cqm_file,
                                                                ) < 0 as c_int
                                                                {
                                                                    current_block =
                                                                        17708740041508716982;
                                                                } else {
                                                                    current_block =
                                                                        3934796541983872331;
                                                                }
                                                            } else {
                                                                current_block = 3934796541983872331;
                                                            }
                                                            match current_block {
                                                                17708740041508716982 => {}
                                                                _ => {
                                                                    x264_reduce_fraction(
                                                                        &mut (*h).param.i_fps_num,
                                                                        &mut (*h).param.i_fps_den,
                                                                    );
                                                                    x264_reduce_fraction(
                                                                        &mut (*h)
                                                                            .param
                                                                            .i_timebase_num,
                                                                        &mut (*h)
                                                                            .param
                                                                            .i_timebase_den,
                                                                    );
                                                                    (*h).i_frame = -1;
                                                                    (*h).i_frame_num = 0 as c_int;
                                                                    if (*h).param.i_avcintra_class
                                                                        != 0
                                                                    {
                                                                        (*h).i_idr_pic_id = if (*h)
                                                                            .param
                                                                            .i_avcintra_class
                                                                            > 200 as c_int
                                                                        {
                                                                            4 as c_int
                                                                        } else {
                                                                            5 as c_int
                                                                        };
                                                                    } else {
                                                                        (*h).i_idr_pic_id =
                                                                            0 as c_int;
                                                                    }
                                                                    if ((*h).param.i_timebase_den
                                                                        as uint64_t)
                                                                        .wrapping_mul(2 as uint64_t)
                                                                        > UINT32_MAX as uint64_t
                                                                    {
                                                                        x264_10_log(
                                                                            h,
                                                                            X264_LOG_ERROR,
                                                                            b"Effective timebase denominator %u exceeds H.264 maximum\n\0"
                                                                                as *const u8 as *const c_char,
                                                                            (*h).param.i_timebase_den,
                                                                        );
                                                                    } else {
                                                                        set_aspect_ratio(
                                                                            h,
                                                                            &mut (*h).param,
                                                                            1 as c_int,
                                                                        );
                                                                        x264_10_sps_init(
                                                                            (*h).sps.as_mut_ptr(),
                                                                            (*h).param.i_sps_id,
                                                                            &mut (*h).param,
                                                                        );
                                                                        x264_10_sps_init_scaling_list(
                                                                            (*h).sps.as_mut_ptr(),
                                                                            &mut (*h).param,
                                                                        );
                                                                        x264_10_pps_init(
                                                                            (*h).pps.as_mut_ptr(),
                                                                            (*h).param.i_sps_id,
                                                                            &mut (*h).param,
                                                                            (*h).sps.as_mut_ptr(),
                                                                        );
                                                                        x264_10_validate_levels(
                                                                            h, 1 as c_int,
                                                                        );
                                                                        (*h).chroma_qp_table = i_chroma_qp_table
                                                                            .as_ptr()
                                                                            .offset(12 as c_int as isize)
                                                                            .offset(
                                                                                (*(*h).pps.as_mut_ptr()).i_chroma_qp_index_offset as isize,
                                                                            );
                                                                        if !(x264_10_cqm_init(h)
                                                                            < 0 as c_int)
                                                                        {
                                                                            (*h).mb.i_mb_width =
                                                                                (*(*h)
                                                                                    .sps
                                                                                    .as_mut_ptr())
                                                                                .i_mb_width;
                                                                            (*h).mb.i_mb_height =
                                                                                (*(*h)
                                                                                    .sps
                                                                                    .as_mut_ptr())
                                                                                .i_mb_height;
                                                                            (*h).mb.i_mb_count = (*h).mb.i_mb_width
                                                                                * (*h).mb.i_mb_height;
                                                                            (*h).mb.chroma_h_shift = ((*(*h).sps.as_mut_ptr())
                                                                                .i_chroma_format_idc == CHROMA_420 as c_int
                                                                                || (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                                                                    == CHROMA_422 as c_int) as c_int;
                                                                            (*h).mb
                                                                                .chroma_v_shift =
                                                                                ((*(*h)
                                                                                    .sps
                                                                                    .as_mut_ptr())
                                                                                .i_chroma_format_idc
                                                                                    == CHROMA_420
                                                                                        as c_int)
                                                                                    as c_int;
                                                                            (*h).mb.b_adaptive_mbaff = ((*h).param.b_interlaced != 0
                                                                                && (*h).param.analyse.i_subpel_refine != 0)
                                                                                as c_int;
                                                                            if (*h).param.i_bframe_adaptive == X264_B_ADAPT_TRELLIS
                                                                                && (*h).param.rc.b_stat_read == 0
                                                                            {
                                                                                (*h).frames.i_delay = (if (*h).param.i_bframe
                                                                                    > 3 as c_int
                                                                                {
                                                                                    (*h).param.i_bframe
                                                                                } else {
                                                                                    3 as c_int
                                                                                }) * 4 as c_int;
                                                                            } else {
                                                                                (*h).frames.i_delay = (*h).param.i_bframe;
                                                                            }
                                                                            if (*h).param.rc.b_mb_tree != 0
                                                                                || (*h).param.rc.i_vbv_buffer_size != 0
                                                                            {
                                                                                (*h).frames.i_delay = if (*h).frames.i_delay
                                                                                    > (*h).param.rc.i_lookahead
                                                                                {
                                                                                    (*h).frames.i_delay
                                                                                } else {
                                                                                    (*h).param.rc.i_lookahead
                                                                                };
                                                                            }
                                                                            i_slicetype_length =
                                                                                (*h).frames.i_delay;
                                                                            (*h).frames.i_delay +=
                                                                                (*h).i_thread_frames
                                                                                    - 1 as c_int;
                                                                            (*h).frames.i_delay += (*h).param.i_sync_lookahead;
                                                                            (*h).frames.i_delay +=
                                                                                (*h).param
                                                                                    .b_vfr_input;
                                                                            (*h).frames
                                                                                .i_bframe_delay =
                                                                                if (*h)
                                                                                    .param
                                                                                    .i_bframe
                                                                                    != 0
                                                                                {
                                                                                    if (*h).param.bframe_pyramid != BPyramid::None {
                                                                                    2
                                                                                } else {
                                                                                    1
                                                                                }
                                                                                } else {
                                                                                    0
                                                                                };
                                                                            (*h).frames
                                                                                .i_max_ref0 = (*h)
                                                                                .param
                                                                                .i_frame_reference;
                                                                            (*h).frames.i_max_ref1 = if (*(*h).sps.as_mut_ptr())
                                                                                .vui
                                                                                .i_num_reorder_frames < (*h).param.i_frame_reference
                                                                            {
                                                                                (*(*h).sps.as_mut_ptr()).vui.i_num_reorder_frames
                                                                            } else {
                                                                                (*h).param.i_frame_reference
                                                                            };
                                                                            (*h).frames.i_max_dpb = (*(*h).sps.as_mut_ptr())
                                                                                .vui
                                                                                .i_max_dec_frame_buffering;
                                                                            (*h).frames.b_have_lowres = ((*h).param.rc.b_stat_read == 0
                                                                                && ((*h).param.rc.i_rc_method == X264_RC_ABR
                                                                                    || (*h).param.rc.i_rc_method == X264_RC_CRF
                                                                                    || (*h).param.i_bframe_adaptive != 0
                                                                                    || (*h).param.i_scenecut_threshold != 0
                                                                                    || (*h).param.rc.b_mb_tree != 0
                                                                                    || (*h).param.analyse.i_weighted_pred != 0))
                                                                                as c_int;
                                                                            (*h).frames.b_have_lowres
                                                                                |= ((*h).param.rc.b_stat_read != 0
                                                                                    && (*h).param.rc.i_vbv_buffer_size
                                                                                        > 0 as c_int) as c_int;
                                                                            (*h).frames.b_have_sub8x8_esa = ((*h).param.analyse.inter
                                                                                & X264_ANALYSE_PSUB8x8 != 0) as c_int;
                                                                            (*h).frames
                                                                                .i_last_keyframe =
                                                                                -(*h)
                                                                                    .param
                                                                                    .i_keyint_max;
                                                                            (*h).frames
                                                                                .i_last_idr = (*h)
                                                                                .frames
                                                                                .i_last_keyframe;
                                                                            (*h).frames.i_input =
                                                                                0 as c_int;
                                                                            (*h).frames.i_second_largest_pts = -(1
                                                                                as c_int) as int64_t;
                                                                            (*h).frames.i_largest_pts = (*h)
                                                                                .frames
                                                                                .i_second_largest_pts;
                                                                            (*h).frames.i_poc_last_open_gop = -(1
                                                                                as c_int);
                                                                            (*h).cost_table = x264_malloc(
                                                                                size_of::<C2RustUnnamed_17>() as int64_t,
                                                                            ) as *mut C2RustUnnamed_17;
                                                                            if !(*h)
                                                                                .cost_table
                                                                                .is_null()
                                                                            {
                                                                                memset(
                                                                                    (*h).cost_table as *mut c_void,
                                                                                    0 as c_int,
                                                                                    size_of::<C2RustUnnamed_17>() as size_t,
                                                                                );
                                                                                (*h).frames.unused[0] = x264_malloc(
                                                                                    (((*h).frames.i_delay + 3 as c_int) as usize)
                                                                                        .wrapping_mul(
                                                                                            size_of::<*mut x264_frame_t>() as usize,
                                                                                        ) as int64_t,
                                                                                ) as *mut *mut x264_frame_t;
                                                                                if !(*h)
                                                                                    .frames
                                                                                    .unused
                                                                                    [0 as c_int
                                                                                        as usize]
                                                                                    .is_null()
                                                                                {
                                                                                    memset(
                                                                                        (*h).frames.unused[0]
                                                                                            as *mut c_void,
                                                                                        0 as c_int,
                                                                                        (((*h).frames.i_delay + 3 as c_int) as size_t)
                                                                                            .wrapping_mul(
                                                                                                size_of::<*mut x264_frame_t>() as size_t,
                                                                                            ),
                                                                                    );
                                                                                    (*h).frames.unused[1] = x264_malloc(
                                                                                        (((*h).i_thread_frames + 16 as c_int
                                                                                            + 4 as c_int) as usize)
                                                                                            .wrapping_mul(
                                                                                                size_of::<*mut x264_frame_t>() as usize,
                                                                                            ) as int64_t,
                                                                                    ) as *mut *mut x264_frame_t;
                                                                                    if !(*h)
                                                                                        .frames
                                                                                        .unused[1]
                                                                                        .is_null()
                                                                                    {
                                                                                        memset(
                                                                                            (*h).frames.unused[1]
                                                                                                as *mut c_void,
                                                                                            0 as c_int,
                                                                                            (((*h).i_thread_frames + 16 as c_int
                                                                                                + 4 as c_int) as size_t)
                                                                                                .wrapping_mul(
                                                                                                    size_of::<*mut x264_frame_t>() as size_t,
                                                                                                ),
                                                                                        );
                                                                                        (*h).frames.current = x264_malloc(
                                                                                            (((*h).param.i_sync_lookahead + (*h).param.i_bframe
                                                                                                + (*h).i_thread_frames + 3 as c_int) as usize)
                                                                                                .wrapping_mul(
                                                                                                    size_of::<*mut x264_frame_t>() as usize,
                                                                                                ) as int64_t,
                                                                                        ) as *mut *mut x264_frame_t;
                                                                                        if !(*h).frames.current.is_null() {
                                                                                            memset(
                                                                                                (*h).frames.current as *mut c_void,
                                                                                                0 as c_int,
                                                                                                (((*h).param.i_sync_lookahead + (*h).param.i_bframe
                                                                                                    + (*h).i_thread_frames + 3 as c_int) as size_t)
                                                                                                    .wrapping_mul(
                                                                                                        size_of::<*mut x264_frame_t>() as size_t,
                                                                                                    ),
                                                                                            );
                                                                                            if (*h).param.analyse.i_weighted_pred
                                                                                                > 0 as c_int
                                                                                            {
                                                                                                (*h).frames.blank_unused = x264_malloc(
                                                                                                    (((*h).i_thread_frames * 4 as c_int) as usize)
                                                                                                        .wrapping_mul(
                                                                                                            size_of::<*mut x264_frame_t>() as usize,
                                                                                                        ) as int64_t,
                                                                                                ) as *mut *mut x264_frame_t;
                                                                                                if (*h).frames.blank_unused.is_null() {
                                                                                                    current_block = 17708740041508716982;
                                                                                                } else {
                                                                                                    memset(
                                                                                                        (*h).frames.blank_unused as *mut c_void,
                                                                                                        0 as c_int,
                                                                                                        (((*h).i_thread_frames * 4 as c_int) as size_t)
                                                                                                            .wrapping_mul(
                                                                                                                size_of::<*mut x264_frame_t>() as size_t,
                                                                                                            ),
                                                                                                    );
                                                                                                    current_block = 6406431739208918833;
                                                                                                }
                                                                                            } else {
                                                                                                current_block = 6406431739208918833;
                                                                                            }
                                                                                            match current_block {
                                                                                                17708740041508716982 => {}
                                                                                                _ => {
                                                                                                    (*h).i_ref[1] = 0
                                                                                                        as c_int;
                                                                                                    (*h).i_ref[0] = (*h)
                                                                                                        .i_ref[1];
                                                                                                    (*h).i_disp_fields = 0 as int64_t;
                                                                                                    (*h).i_coded_fields = (*h).i_disp_fields;
                                                                                                    (*h).i_cpb_delay = (*h).i_coded_fields;
                                                                                                    (*h).i_prev_duration = ((*h).param.i_fps_den as uint64_t)
                                                                                                        .wrapping_mul(
                                                                                                            (*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t,
                                                                                                        )
                                                                                                        .wrapping_div(
                                                                                                            ((*h).param.i_fps_num as uint64_t)
                                                                                                                .wrapping_mul(
                                                                                                                    (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as uint64_t,
                                                                                                                ),
                                                                                                        ) as int64_t;
                                                                                                    (*h).i_disp_fields_last_frame = -1;
                                                                                                    x264_10_rdo_init();
                                                                                                    (*h).param.cpu = ((*h).param.cpu as c_uint
                                                                                                        & !X264_CPU_AVX512) as uint32_t;
                                                                                                    x264_10_predict_16x16_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_16x16.as_mut_ptr(),
                                                                                                    );
                                                                                                    x264_10_predict_8x8c_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_8x8c.as_mut_ptr(),
                                                                                                    );
                                                                                                    x264_10_predict_8x16c_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_8x16c.as_mut_ptr(),
                                                                                                    );
                                                                                                    x264_predict_8x8_init(
                                                                                                        (*h).param.cpu,
                                                                                                        &mut (*h).predict_8x8,
                                                                                                        &mut (*h).predict_8x8_filter,
                                                                                                    );
                                                                                                    x264_10_predict_4x4_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_4x4.as_mut_ptr(),
                                                                                                    );
                                                                                                    x264_10_pixel_init((*h).param.cpu, &mut (*h).pixf);
                                                                                                    x264_10_dct_init((*h).param.cpu, &mut (*h).dctf);
                                                                                                    x264_10_zigzag_init(
                                                                                                        (*h).param.cpu,
                                                                                                        &mut (*h).zigzagf_progressive,
                                                                                                        &mut (*h).zigzagf_interlaced,
                                                                                                    );
                                                                                                    memcpy(
                                                                                                        &mut (*h).zigzagf as *mut x264_zigzag_function_t
                                                                                                            as *mut c_void,
                                                                                                        (if (*h).param.b_interlaced != 0 {
                                                                                                            &mut (*h).zigzagf_interlaced as *mut x264_zigzag_function_t
                                                                                                        } else {
                                                                                                            &mut (*h).zigzagf_progressive as *mut x264_zigzag_function_t
                                                                                                        }) as *const c_void,
                                                                                                        size_of::<x264_zigzag_function_t>() as size_t,
                                                                                                    );
                                                                                                    x264_10_mc_init(
                                                                                                        (*h).param.cpu,
                                                                                                        &mut (*h).mc,
                                                                                                        (*h).param.b_cpu_independent,
                                                                                                    );
                                                                                                    x264_quant_init(h, (*h).param.cpu, &mut (*h).quantf);
                                                                                                    x264_10_deblock_init(
                                                                                                        (*h).param.cpu,
                                                                                                        &mut (*h).loopf,
                                                                                                        (*h).param.b_interlaced,
                                                                                                    );
                                                                                                    x264_10_bitstream_init((*h).param.cpu, &mut (*h).bsf);
                                                                                                    if (*h).param.b_cabac != 0 {
                                                                                                        x264_10_cabac_init(h);
                                                                                                    } else {
                                                                                                        x264_10_cavlc_init(h);
                                                                                                    }
                                                                                                    mbcmp_init(h);
                                                                                                    chroma_dsp_init(h);
                                                                                                    p = buf
                                                                                                        .as_mut_ptr()
                                                                                                        .offset(
                                                                                                            sprintf(
                                                                                                                buf.as_mut_ptr(),
                                                                                                                b"using cpu capabilities:\0" as *const u8
                                                                                                                    as *const c_char,
                                                                                                            ) as isize,
                                                                                                        );
                                                                                                    let mut i: c_int = 0 as c_int;
                                                                                                    while (*x264_cpu_names.as_ptr().offset(i as isize)).flags
                                                                                                        != 0
                                                                                                    {
                                                                                                        if !(strcmp(
                                                                                                            (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                            b"SSE\0" as *const u8 as *const c_char,
                                                                                                        ) == 0
                                                                                                            && (*h).param.cpu
                                                                                                                & (1 as uint32_t) << 3 as c_int != 0)
                                                                                                        {
                                                                                                            if !(strcmp(
                                                                                                                (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                b"SSE2\0" as *const u8 as *const c_char,
                                                                                                            ) == 0
                                                                                                                && (*h).param.cpu
                                                                                                                    & (X264_CPU_SSE2_IS_FAST as uint32_t
                                                                                                                        | X264_CPU_SSE2_IS_SLOW as uint32_t) != 0)
                                                                                                            {
                                                                                                                if !(strcmp(
                                                                                                                    (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                    b"SSE3\0" as *const u8 as *const c_char,
                                                                                                                ) == 0
                                                                                                                    && ((*h).param.cpu & X264_CPU_SSSE3 as uint32_t != 0
                                                                                                                        || (*h).param.cpu & X264_CPU_CACHELINE_64 as uint32_t == 0))
                                                                                                                {
                                                                                                                    if !(strcmp(
                                                                                                                        (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                        b"SSE4.1\0" as *const u8 as *const c_char,
                                                                                                                    ) == 0 && (*h).param.cpu & X264_CPU_SSE42 as uint32_t != 0)
                                                                                                                    {
                                                                                                                        if !(strcmp(
                                                                                                                            (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                            b"LZCNT\0" as *const u8 as *const c_char,
                                                                                                                        ) == 0 && (*h).param.cpu & X264_CPU_BMI1 as uint32_t != 0)
                                                                                                                        {
                                                                                                                            if !(strcmp(
                                                                                                                                (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                                b"BMI1\0" as *const u8 as *const c_char,
                                                                                                                            ) == 0 && (*h).param.cpu & X264_CPU_BMI2 as uint32_t != 0)
                                                                                                                            {
                                                                                                                                if !(strcmp(
                                                                                                                                    (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                                    b"FMA4\0" as *const u8 as *const c_char,
                                                                                                                                ) == 0 && (*h).param.cpu & X264_CPU_FMA3 as uint32_t != 0)
                                                                                                                                {
                                                                                                                                    if (*h).param.cpu
                                                                                                                                        & (*x264_cpu_names.as_ptr().offset(i as isize)).flags
                                                                                                                                        == (*x264_cpu_names.as_ptr().offset(i as isize)).flags
                                                                                                                                        && (i == 0
                                                                                                                                            || (*x264_cpu_names.as_ptr().offset(i as isize)).flags
                                                                                                                                                != (*x264_cpu_names
                                                                                                                                                    .as_ptr()
                                                                                                                                                    .offset((i - 1 as c_int) as isize))
                                                                                                                                                    .flags)
                                                                                                                                    {
                                                                                                                                        p = p
                                                                                                                                            .offset(
                                                                                                                                                sprintf(
                                                                                                                                                    p,
                                                                                                                                                    b" %s\0" as *const u8 as *const c_char,
                                                                                                                                                    (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                                                ) as isize,
                                                                                                                                            );
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        i += 1;
                                                                                                    }
                                                                                                    if (*h).param.cpu == 0 {
                                                                                                        p = p
                                                                                                            .offset(
                                                                                                                sprintf(
                                                                                                                    p,
                                                                                                                    b" none!\0" as *const u8 as *const c_char,
                                                                                                                ) as isize,
                                                                                                            );
                                                                                                    }
                                                                                                    x264_10_log(
                                                                                                        h,
                                                                                                        X264_LOG_INFO,
                                                                                                        b"%s\n\0" as *const u8 as *const c_char,
                                                                                                        buf.as_mut_ptr(),
                                                                                                    );
                                                                                                    if !(x264_10_analyse_init_costs(h) != 0) {
                                                                                                        temp = 392 as c_int;
                                                                                                        if (temp as c_uint).leading_zeros() as i32
                                                                                                            != 23 as c_int
                                                                                                        {
                                                                                                            x264_10_log(
                                                                                                                h,
                                                                                                                X264_LOG_ERROR,
                                                                                                                b"CLZ test failed: x264 has been miscompiled!\n\0"
                                                                                                                    as *const u8 as *const c_char,
                                                                                                            );
                                                                                                            x264_10_log(
                                                                                                                h,
                                                                                                                X264_LOG_ERROR,
                                                                                                                b"Are you attempting to run an SSE4a/LZCNT-targeted build on a CPU that\n\0"
                                                                                                                    as *const u8 as *const c_char,
                                                                                                            );
                                                                                                            x264_10_log(
                                                                                                                h,
                                                                                                                X264_LOG_ERROR,
                                                                                                                b"doesn't support it?\n\0" as *const u8
                                                                                                                    as *const c_char,
                                                                                                            );
                                                                                                        } else {
                                                                                                            (*h).out.i_nal = 0 as c_int;
                                                                                                            (*h).out.i_bitstream = x264_clip3f(
                                                                                                                ((*h).param.width * (*h).param.height
                                                                                                                    * 4) as c_double
                                                                                                                    * (if (*h).param.rc.i_rc_method == X264_RC_ABR {
                                                                                                                        pow(
                                                                                                                            0.95f64,
                                                                                                                            (*h).param.rc.i_qp_min as c_double,
                                                                                                                        )
                                                                                                                    } else {
                                                                                                                        pow(
                                                                                                                            0.95f64,
                                                                                                                            (*h).param.rc.i_qp_constant as c_double,
                                                                                                                        )
                                                                                                                            * (if 1 as c_int as c_float
                                                                                                                                > (*h).param.rc.f_ip_factor
                                                                                                                            {
                                                                                                                                1 as c_int as c_float
                                                                                                                            } else {
                                                                                                                                (*h).param.rc.f_ip_factor
                                                                                                                            }) as c_double
                                                                                                                    }),
                                                                                                                1000000 as c_int as c_double,
                                                                                                                (INT_MAX / 3 as c_int) as c_double,
                                                                                                            ) as c_int;
                                                                                                            (*h).nal_buffer_size = (*h).out.i_bitstream
                                                                                                                * 3 as c_int / 2 as c_int
                                                                                                                + 4 as c_int + 64 as c_int;
                                                                                                            (*h).nal_buffer = x264_malloc(
                                                                                                                (*h).nal_buffer_size as int64_t,
                                                                                                            ) as *mut uint8_t;
                                                                                                            if !(*h).nal_buffer.is_null() {
                                                                                                                (*h).reconfig_h = x264_malloc(
                                                                                                                    size_of::<x264_t>() as int64_t,
                                                                                                                ) as *mut x264_t;
                                                                                                                if !(*h).reconfig_h.is_null() {
                                                                                                                    if !((*h).param.i_threads > 1 as c_int
                                                                                                                        && x264_10_threadpool_init(
                                                                                                                            &mut (*h).threadpool,
                                                                                                                            (*h).param.i_threads,
                                                                                                                        ) != 0)
                                                                                                                    {
                                                                                                                        if !((*h).param.i_lookahead_threads
                                                                                                                            > 1 as c_int
                                                                                                                            && x264_10_threadpool_init(
                                                                                                                                &mut (*h).lookaheadpool,
                                                                                                                                (*h).param.i_lookahead_threads,
                                                                                                                            ) != 0)
                                                                                                                        {
                                                                                                                            (*h).thread[0] = h;
                                                                                                                            let mut i_0: c_int = 1 as c_int;
                                                                                                                            loop {
                                                                                                                                if !(i_0
                                                                                                                                    < (*h).param.i_threads
                                                                                                                                        + ((*h).param.i_sync_lookahead != 0) as c_int)
                                                                                                                                {
                                                                                                                                    current_block = 4338462691184853296;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                (*h).thread[i_0 as usize] = x264_malloc(
                                                                                                                                    size_of::<x264_t>() as int64_t,
                                                                                                                                ) as *mut x264_t;
                                                                                                                                if (*h).thread[i_0 as usize].is_null() {
                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                i_0 += 1;
                                                                                                                            }
                                                                                                                            match current_block {
                                                                                                                                17708740041508716982 => {}
                                                                                                                                _ => {
                                                                                                                                    if (*h).param.i_lookahead_threads > 1 as c_int
                                                                                                                                    {
                                                                                                                                        let mut i_1: c_int = 0 as c_int;
                                                                                                                                        loop {
                                                                                                                                            if !(i_1 < (*h).param.i_lookahead_threads) {
                                                                                                                                                current_block = 11735322225073324345;
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            (*h).lookahead_thread[i_1 as usize] = x264_malloc(
                                                                                                                                                size_of::<x264_t>() as int64_t,
                                                                                                                                            ) as *mut x264_t;
                                                                                                                                            if (*h).lookahead_thread[i_1 as usize].is_null() {
                                                                                                                                                current_block = 17708740041508716982;
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            *(*h).lookahead_thread[i_1 as usize] = *h;
                                                                                                                                            i_1 += 1;
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        current_block = 11735322225073324345;
                                                                                                                                    }
                                                                                                                                    match current_block {
                                                                                                                                        17708740041508716982 => {}
                                                                                                                                        _ => {
                                                                                                                                            *(*h).reconfig_h = *h;
                                                                                                                                            let mut i_2: c_int = 0 as c_int;
                                                                                                                                            loop {
                                                                                                                                                if !(i_2 < (*h).param.i_threads) {
                                                                                                                                                    current_block = 9702083122263515018;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                let mut init_nal_count: c_int = (*h)
                                                                                                                                                    .param
                                                                                                                                                    .i_slice_count + 3 as c_int;
                                                                                                                                                let mut allocate_threadlocal_data: c_int = ((*h)
                                                                                                                                                    .param
                                                                                                                                                    .b_sliced_threads == 0 || i_2 == 0) as c_int;
                                                                                                                                                if i_2 > 0 as c_int {
                                                                                                                                                    *(*h).thread[i_2 as usize] = *h;
                                                                                                                                                }
                                                                                                                                                if pthread_mutex_init(
                                                                                                                                                    &mut (**(*h).thread.as_mut_ptr().offset(i_2 as isize))
                                                                                                                                                        .mutex,
                                                                                                                                                    0 as *const pthread_mutexattr_t,
                                                                                                                                                ) != 0
                                                                                                                                                {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                if pthread_cond_init(
                                                                                                                                                    &mut (**(*h).thread.as_mut_ptr().offset(i_2 as isize)).cv,
                                                                                                                                                    0 as *const pthread_condattr_t,
                                                                                                                                                ) != 0
                                                                                                                                                {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                if allocate_threadlocal_data != 0 {
                                                                                                                                                    (*(*h).thread[i_2 as usize]).fdec = x264_10_frame_pop_unused(
                                                                                                                                                        h,
                                                                                                                                                        1 as c_int,
                                                                                                                                                    );
                                                                                                                                                    if (*(*h).thread[i_2 as usize]).fdec.is_null() {
                                                                                                                                                        current_block = 17708740041508716982;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    (*(*h).thread[i_2 as usize]).fdec = (*(*h)
                                                                                                                                                        .thread[0])
                                                                                                                                                        .fdec;
                                                                                                                                                }
                                                                                                                                                (*(*h).thread[i_2 as usize]).out.p_bitstream = x264_malloc(
                                                                                                                                                    (*h).out.i_bitstream as int64_t,
                                                                                                                                                ) as *mut uint8_t;
                                                                                                                                                if (*(*h).thread[i_2 as usize]).out.p_bitstream.is_null() {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                (*(*h).thread[i_2 as usize]).out.nal = x264_malloc(
                                                                                                                                                    (init_nal_count as usize)
                                                                                                                                                        .wrapping_mul(size_of::<x264_nal_t>() as usize)
                                                                                                                                                        as int64_t,
                                                                                                                                                ) as *mut x264_nal_t;
                                                                                                                                                if (*(*h).thread[i_2 as usize]).out.nal.is_null() {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                (*(*h).thread[i_2 as usize]).out.i_nals_allocated = init_nal_count;
                                                                                                                                                if allocate_threadlocal_data != 0
                                                                                                                                                    && x264_10_macroblock_cache_allocate(
                                                                                                                                                        (*h).thread[i_2 as usize],
                                                                                                                                                    ) < 0 as c_int
                                                                                                                                                {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                i_2 += 1;
                                                                                                                                            }
                                                                                                                                            match current_block {
                                                                                                                                                17708740041508716982 => {}
                                                                                                                                                _ => {
                                                                                                                                                    if !(x264_10_lookahead_init(h, i_slicetype_length) != 0) {
                                                                                                                                                        let mut i_3: c_int = 0 as c_int;
                                                                                                                                                        loop {
                                                                                                                                                            if !(i_3 < (*h).param.i_threads) {
                                                                                                                                                                current_block = 9812798724717783973;
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            if x264_10_macroblock_thread_allocate(
                                                                                                                                                                (*h).thread[i_3 as usize],
                                                                                                                                                                0 as c_int,
                                                                                                                                                            ) < 0 as c_int
                                                                                                                                                            {
                                                                                                                                                                current_block = 17708740041508716982;
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            i_3 += 1;
                                                                                                                                                        }
                                                                                                                                                        match current_block {
                                                                                                                                                            17708740041508716982 => {}
                                                                                                                                                            _ => {
                                                                                                                                                                if !(x264_10_ratecontrol_new(h) < 0 as c_int) {
                                                                                                                                                                    if (*h).param.i_nal_hrd != 0 {
                                                                                                                                                                        x264_10_log(
                                                                                                                                                                            h,
                                                                                                                                                                            X264_LOG_DEBUG,
                                                                                                                                                                            b"HRD bitrate: %i bits/sec\n\0" as *const u8
                                                                                                                                                                                as *const c_char,
                                                                                                                                                                            (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled,
                                                                                                                                                                        );
                                                                                                                                                                        x264_10_log(
                                                                                                                                                                            h,
                                                                                                                                                                            X264_LOG_DEBUG,
                                                                                                                                                                            b"CPB size: %i bits\n\0" as *const u8
                                                                                                                                                                                as *const c_char,
                                                                                                                                                                            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled,
                                                                                                                                                                        );
                                                                                                                                                                    }
                                                                                                                                                                    if !(*h).param.psz_dump_yuv.is_null() {
                                                                                                                                                                        let mut f: *mut FILE = fopen(
                                                                                                                                                                            (*h).param.psz_dump_yuv,
                                                                                                                                                                            b"w\0" as *const u8 as *const c_char,
                                                                                                                                                                        ) as *mut FILE;
                                                                                                                                                                        if f.is_null() {
                                                                                                                                                                            x264_10_log(
                                                                                                                                                                                h,
                                                                                                                                                                                X264_LOG_ERROR,
                                                                                                                                                                                b"dump_yuv: can't write to %s\n\0" as *const u8
                                                                                                                                                                                    as *const c_char,
                                                                                                                                                                                (*h).param.psz_dump_yuv,
                                                                                                                                                                            );
                                                                                                                                                                            current_block = 17708740041508716982;
                                                                                                                                                                        } else if x264_is_regular_file(f) == 0 {
                                                                                                                                                                            x264_10_log(
                                                                                                                                                                                h,
                                                                                                                                                                                X264_LOG_ERROR,
                                                                                                                                                                                b"dump_yuv: incompatible with non-regular file %s\n\0"
                                                                                                                                                                                    as *const u8 as *const c_char,
                                                                                                                                                                                (*h).param.psz_dump_yuv,
                                                                                                                                                                            );
                                                                                                                                                                            fclose(f);
                                                                                                                                                                            current_block = 17708740041508716982;
                                                                                                                                                                        } else {
                                                                                                                                                                            fclose(f);
                                                                                                                                                                            current_block = 11844752514624976770;
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        current_block = 11844752514624976770;
                                                                                                                                                                    }
                                                                                                                                                                    match current_block {
                                                                                                                                                                        17708740041508716982 => {}
                                                                                                                                                                        _ => {
                                                                                                                                                                            profile = if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_BASELINE as c_int
                                                                                                                                                                            {
                                                                                                                                                                                b"Constrained Baseline\0" as *const u8
                                                                                                                                                                                    as *const c_char
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_MAIN as c_int
                                                                                                                                                                            {
                                                                                                                                                                                b"Main\0" as *const u8 as *const c_char
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_HIGH as c_int
                                                                                                                                                                            {
                                                                                                                                                                                b"High\0" as *const u8 as *const c_char
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_HIGH10 as c_int
                                                                                                                                                                            {
                                                                                                                                                                                if (*(*h).sps.as_mut_ptr()).b_constraint_set3 != 0 {
                                                                                                                                                                                    b"High 10 Intra\0" as *const u8
                                                                                                                                                                                        as *const c_char
                                                                                                                                                                                } else {
                                                                                                                                                                                    b"High 10\0" as *const u8 as *const c_char
                                                                                                                                                                                }
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_HIGH422 as c_int
                                                                                                                                                                            {
                                                                                                                                                                                if (*(*h).sps.as_mut_ptr()).b_constraint_set3 != 0 {
                                                                                                                                                                                    b"High 4:2:2 Intra\0" as *const u8
                                                                                                                                                                                        as *const c_char
                                                                                                                                                                                } else {
                                                                                                                                                                                    b"High 4:2:2\0" as *const u8 as *const c_char
                                                                                                                                                                                }
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).b_constraint_set3 != 0 {
                                                                                                                                                                                b"High 4:4:4 Intra\0" as *const u8
                                                                                                                                                                                    as *const c_char
                                                                                                                                                                            } else {
                                                                                                                                                                                b"High 4:4:4 Predictive\0" as *const u8
                                                                                                                                                                                    as *const c_char
                                                                                                                                                                            };
                                                                                                                                                                            level = [0; 16];
                                                                                                                                                                            if (*(*h).sps.as_mut_ptr()).i_level_idc
                                                                                                                                                                                == 9 as c_int
                                                                                                                                                                                || (*(*h).sps.as_mut_ptr()).i_level_idc
                                                                                                                                                                                    == 11 as c_int
                                                                                                                                                                                    && (*(*h).sps.as_mut_ptr()).b_constraint_set3 != 0
                                                                                                                                                                                    && ((*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                        == PROFILE_BASELINE as c_int
                                                                                                                                                                                        || (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                            == PROFILE_MAIN as c_int)
                                                                                                                                                                            {
                                                                                                                                                                                strcpy(
                                                                                                                                                                                    level.as_mut_ptr(),
                                                                                                                                                                                    b"1b\0" as *const u8 as *const c_char,
                                                                                                                                                                                );
                                                                                                                                                                            } else {
                                                                                                                                                                                snprintf(
                                                                                                                                                                                    level.as_mut_ptr(),
                                                                                                                                                                                    size_of::<[c_char; 16]>()
                                                                                                                                                                                        as size_t,
                                                                                                                                                                                    b"%d.%d\0" as *const u8 as *const c_char,
                                                                                                                                                                                    (*(*h).sps.as_mut_ptr()).i_level_idc
                                                                                                                                                                                        / 10 as c_int,
                                                                                                                                                                                    (*(*h).sps.as_mut_ptr()).i_level_idc
                                                                                                                                                                                        % 10 as c_int,
                                                                                                                                                                                );
                                                                                                                                                                            }
                                                                                                                                                                            x264_10_log(
                                                                                                                                                                                h,
                                                                                                                                                                                X264_LOG_INFO,
                                                                                                                                                                                b"profile %s, level %s, %s, %d-bit\n\0" as *const u8
                                                                                                                                                                                    as *const c_char,
                                                                                                                                                                                profile,
                                                                                                                                                                                level.as_mut_ptr(),
                                                                                                                                                                                subsampling[(*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                                                                                                                                                                    as usize],
                                                                                                                                                                                BIT_DEPTH,
                                                                                                                                                                            );
                                                                                                                                                                            return h;
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    x264_free(h as *mut c_void);
    return 0 as *mut x264_t;
}
#[c2rust::src_loc = "1862:1"]
unsafe extern "C" fn encoder_try_reconfig(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
    mut rc_reconfig: *mut c_int,
) -> c_int {
    *rc_reconfig = 0 as c_int;
    set_aspect_ratio(h, param, 0 as c_int);
    (*h).param.i_frame_reference = (*param).i_frame_reference;
    (*h).param.i_bframe_bias = (*param).i_bframe_bias;
    if (*h).param.i_scenecut_threshold != 0 {
        (*h).param.i_scenecut_threshold = (*param).i_scenecut_threshold;
    }
    (*h).param.b_deblocking_filter = (*param).b_deblocking_filter;
    (*h).param.i_deblocking_filter_alphac0 = (*param).i_deblocking_filter_alphac0;
    (*h).param.i_deblocking_filter_beta = (*param).i_deblocking_filter_beta;
    (*h).param.frame_packing = (*param).frame_packing;
    (*h).param.mastering_display = (*param).mastering_display;
    (*h).param.content_light_level = (*param).content_light_level;
    (*h).param.i_alternative_transfer = (*param).i_alternative_transfer;
    (*h).param.analyse.inter = (*param).analyse.inter;
    (*h).param.analyse.intra = (*param).analyse.intra;
    (*h).param.analyse.i_direct_mv_pred = (*param).analyse.i_direct_mv_pred;
    if (*h).param.analyse.i_me_method < X264_ME_ESA
        || (*param).analyse.i_me_range < (*h).param.analyse.i_me_range
    {
        (*h).param.analyse.i_me_range = (*param).analyse.i_me_range;
    }
    (*h).param.analyse.i_noise_reduction = (*param).analyse.i_noise_reduction;
    if (*h).param.analyse.i_subpel_refine != 0 {
        (*h).param.analyse.i_subpel_refine = (*param).analyse.i_subpel_refine;
    }
    (*h).param.analyse.i_trellis = (*param).analyse.i_trellis;
    (*h).param.analyse.b_chroma_me = (*param).analyse.b_chroma_me;
    (*h).param.analyse.b_dct_decimate = (*param).analyse.b_dct_decimate;
    (*h).param.analyse.b_fast_pskip = (*param).analyse.b_fast_pskip;
    (*h).param.analyse.b_mixed_references = (*param).analyse.b_mixed_references;
    (*h).param.analyse.f_psy_rd = (*param).analyse.f_psy_rd;
    (*h).param.analyse.f_psy_trellis = (*param).analyse.f_psy_trellis;
    (*h).param.crop_rect = (*param).crop_rect;
    if (*h).param.analyse.i_me_method >= X264_ME_ESA || (*param).analyse.i_me_method < X264_ME_ESA {
        (*h).param.analyse.i_me_method = (*param).analyse.i_me_method;
    }
    if (*h).param.analyse.i_me_method >= X264_ME_ESA && (*h).frames.b_have_sub8x8_esa == 0 {
        (*h).param.analyse.inter &= !X264_ANALYSE_PSUB8x8;
    }
    if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
        (*h).param.analyse.b_transform_8x8 = (*param).analyse.b_transform_8x8;
    }
    if (*h).frames.i_max_ref1 > 1 as c_int {
        (*h).param.bframe_pyramid = (*param).bframe_pyramid;
    }
    (*h).param.i_slice_max_size = (*param).i_slice_max_size;
    (*h).param.i_slice_max_mbs = (*param).i_slice_max_mbs;
    (*h).param.i_slice_min_mbs = (*param).i_slice_min_mbs;
    (*h).param.i_slice_count = (*param).i_slice_count;
    (*h).param.i_slice_count_max = (*param).i_slice_count_max;
    (*h).param.b_tff = (*param).b_tff;
    if (*h).param.rc.i_vbv_max_bitrate > 0 as c_int
        && (*h).param.rc.i_vbv_buffer_size > 0 as c_int
        && (*param).rc.i_vbv_max_bitrate > 0 as c_int
        && (*param).rc.i_vbv_buffer_size > 0 as c_int
    {
        *rc_reconfig |= ((*h).param.rc.i_vbv_max_bitrate != (*param).rc.i_vbv_max_bitrate) as c_int;
        *rc_reconfig |= ((*h).param.rc.i_vbv_buffer_size != (*param).rc.i_vbv_buffer_size) as c_int;
        *rc_reconfig |= ((*h).param.rc.i_bitrate != (*param).rc.i_bitrate) as c_int;
        (*h).param.rc.i_vbv_max_bitrate = (*param).rc.i_vbv_max_bitrate;
        (*h).param.rc.i_vbv_buffer_size = (*param).rc.i_vbv_buffer_size;
        (*h).param.rc.i_bitrate = (*param).rc.i_bitrate;
    }
    *rc_reconfig |= ((*h).param.rc.f_rf_constant != (*param).rc.f_rf_constant) as c_int;
    *rc_reconfig |= ((*h).param.rc.f_rf_constant_max != (*param).rc.f_rf_constant_max) as c_int;
    (*h).param.rc.f_rf_constant = (*param).rc.f_rf_constant;
    (*h).param.rc.f_rf_constant_max = (*param).rc.f_rf_constant_max;
    return validate_parameters(h, 0 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "1932:1"]
unsafe extern "C" fn x264_10_encoder_reconfig_apply(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) -> c_int {
    let mut rc_reconfig: c_int = 0;
    let mut ret: c_int = encoder_try_reconfig(h, param, &mut rc_reconfig);
    mbcmp_init(h);
    if ret == 0 {
        x264_10_sps_init_reconfigurable((*h).sps.as_mut_ptr(), &mut (*h).param);
    }
    if ret == 0 && rc_reconfig != 0 {
        x264_10_ratecontrol_init_reconfigurable(h, 0 as c_int);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1955:1"]
unsafe extern "C" fn x264_10_encoder_reconfig(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) -> c_int {
    h = (*h).thread[(*(*h).thread[0]).i_thread_phase as usize];
    let mut param_save: x264_param_t = (*(*h).reconfig_h).param;
    (*(*h).reconfig_h).param = (*h).param;
    let mut rc_reconfig: c_int = 0;
    let mut ret: c_int = encoder_try_reconfig((*h).reconfig_h, param, &mut rc_reconfig);
    if ret == 0 {
        (*h).reconfig = 1 as c_int;
    } else {
        (*(*h).reconfig_h).param = param_save;
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1974:1"]
unsafe extern "C" fn x264_10_encoder_parameters(mut h: *mut x264_t, mut param: *mut x264_param_t) {
    memcpy(
        param as *mut c_void,
        &mut (**(*h)
            .thread
            .as_mut_ptr()
            .offset((*h).i_thread_phase as isize))
        .param as *mut x264_param_t as *const c_void,
        size_of::<x264_param_t>() as size_t,
    );
    (*param).opaque = NULL;
}
#[c2rust::src_loc = "1981:1"]
unsafe extern "C" fn nal_start(mut h: *mut x264_t, mut i_type: c_int, mut i_ref_idc: c_int) {
    let mut nal: *mut x264_nal_t =
        &mut *(*h).out.nal.offset((*h).out.i_nal as isize) as *mut x264_nal_t;
    (*nal).i_ref_idc = i_ref_idc;
    (*nal).i_type = i_type;
    (*nal).b_long_startcode = 1 as c_int;
    (*nal).i_payload = 0 as c_int;
    (*nal).p_payload = &mut *(*h).out.p_bitstream.offset(
        ((bs_pos as unsafe extern "C" fn(*mut bs_t) -> c_int)(&mut (*h).out.bs) / 8 as c_int)
            as isize,
    ) as *mut uint8_t;
    (*nal).i_padding = 0 as c_int;
}
#[c2rust::src_loc = "1995:1"]
unsafe extern "C" fn nal_check_buffer(mut h: *mut x264_t) -> c_int {
    if (*h).out.i_nal >= (*h).out.i_nals_allocated {
        let mut new_out: *mut x264_nal_t = x264_malloc(
            (size_of::<x264_nal_t>() as usize)
                .wrapping_mul(((*h).out.i_nals_allocated * 2 as c_int) as usize)
                as int64_t,
        ) as *mut x264_nal_t;
        if new_out.is_null() {
            return -1;
        }
        memcpy(
            new_out as *mut c_void,
            (*h).out.nal as *const c_void,
            (size_of::<x264_nal_t>() as size_t).wrapping_mul((*h).out.i_nals_allocated as size_t),
        );
        x264_free((*h).out.nal as *mut c_void);
        (*h).out.nal = new_out;
        (*h).out.i_nals_allocated *= 2 as c_int;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "2010:1"]
unsafe extern "C" fn nal_end(mut h: *mut x264_t) -> c_int {
    let mut nal: *mut x264_nal_t =
        &mut *(*h).out.nal.offset((*h).out.i_nal as isize) as *mut x264_nal_t;
    let mut end: *mut uint8_t = &mut *(*h).out.p_bitstream.offset(
        ((bs_pos as unsafe extern "C" fn(*mut bs_t) -> c_int)(&mut (*h).out.bs) / 8 as c_int)
            as isize,
    ) as *mut uint8_t;
    (*nal).i_payload = end.offset_from((*nal).p_payload) as c_long as c_int;
    memset(end as *mut c_void, 0xff as c_int, 64 as size_t);
    if (*h).param.nalu_process.is_some() {
        (*h).param.nalu_process.expect("non-null function pointer")(
            // TODO: may be invalid
            (*h).api as *mut crate::x264_h::x264_t,
            nal,
            (*(*h).fenc).opaque,
        );
    }
    (*h).out.i_nal += 1;
    return nal_check_buffer(h);
}
#[c2rust::src_loc = "2025:1"]
unsafe extern "C" fn check_encapsulated_buffer(
    mut h: *mut x264_t,
    mut h0: *mut x264_t,
    mut start: c_int,
    mut previous_nal_size: int64_t,
    mut necessary_size: int64_t,
) -> c_int {
    if ((*h0).nal_buffer_size as int64_t) < necessary_size {
        necessary_size *= 2 as int64_t;
        if necessary_size > INT_MAX as int64_t {
            return -1;
        }
        let mut buf: *mut uint8_t = x264_malloc(necessary_size) as *mut uint8_t;
        if buf.is_null() {
            return -1;
        }
        if previous_nal_size != 0 {
            memcpy(
                buf as *mut c_void,
                (*h0).nal_buffer as *const c_void,
                previous_nal_size as size_t,
            );
        }
        let mut delta: intptr_t = buf.offset_from((*h0).nal_buffer) as intptr_t;
        let mut i: c_int = 0 as c_int;
        while i < start {
            let ref mut fresh1 = (*(*h).out.nal.offset(i as isize)).p_payload;
            *fresh1 = (*fresh1).offset(delta as isize);
            i += 1;
        }
        x264_free((*h0).nal_buffer as *mut c_void);
        (*h0).nal_buffer = buf;
        (*h0).nal_buffer_size = necessary_size as c_int;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "2051:1"]
unsafe extern "C" fn encoder_encapsulate_nals(mut h: *mut x264_t, mut start: c_int) -> c_int {
    let mut h0: *mut x264_t = (*h).thread[0];
    let mut nal_size: int64_t = 0 as int64_t;
    let mut previous_nal_size: int64_t = 0 as int64_t;
    if (*h).param.nalu_process.is_some() {
        let mut i: c_int = start;
        while i < (*h).out.i_nal {
            nal_size += (*(*h).out.nal.offset(i as isize)).i_payload as int64_t;
            i += 1;
        }
        if nal_size > INT_MAX as int64_t {
            return -1;
        }
        return nal_size as c_int;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < start {
        previous_nal_size += (*(*h).out.nal.offset(i_0 as isize)).i_payload as int64_t;
        i_0 += 1;
    }
    let mut i_1: c_int = start;
    while i_1 < (*h).out.i_nal {
        nal_size += (*(*h).out.nal.offset(i_1 as isize)).i_payload as int64_t;
        i_1 += 1;
    }
    let mut necessary_size: int64_t = previous_nal_size
        + nal_size * 3 as int64_t / 2 as int64_t
        + ((*h).out.i_nal * 4 as c_int) as int64_t
        + 4 as int64_t
        + 64 as int64_t;
    let mut i_2: c_int = start;
    while i_2 < (*h).out.i_nal {
        necessary_size += (*(*h).out.nal.offset(i_2 as isize)).i_padding as int64_t;
        i_2 += 1;
    }
    if check_encapsulated_buffer(h, h0, start, previous_nal_size, necessary_size) != 0 {
        return -1;
    }
    let mut nal_buffer: *mut uint8_t = (*h0).nal_buffer.offset(previous_nal_size as isize);
    let mut i_3: c_int = start;
    while i_3 < (*h).out.i_nal {
        (*(*h).out.nal.offset(i_3 as isize)).b_long_startcode = (i_3 == 0
            || (*(*h).out.nal.offset(i_3 as isize)).i_type == NAL_SPS as c_int
            || (*(*h).out.nal.offset(i_3 as isize)).i_type == NAL_PPS as c_int
            || (*h).param.i_avcintra_class != 0)
            as c_int;
        x264_10_nal_encode(h, nal_buffer, &mut *(*h).out.nal.offset(i_3 as isize));
        nal_buffer = nal_buffer.offset((*(*h).out.nal.offset(i_3 as isize)).i_payload as isize);
        i_3 += 1;
    }
    return nal_buffer.offset_from((*h0).nal_buffer.offset(previous_nal_size as isize)) as c_long
        as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2096:1"]
unsafe extern "C" fn x264_10_encoder_headers(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut c_int,
) -> c_int {
    let mut frame_size: c_int = 0 as c_int;
    (*h).out.i_nal = 0 as c_int;
    bs_init(
        &mut (*h).out.bs,
        (*h).out.p_bitstream as *mut c_void,
        (*h).out.i_bitstream,
    );
    nal_start(h, NAL_SPS as c_int, NAL_PRIORITY_HIGHEST as c_int);
    x264_10_sps_write(&mut (*h).out.bs, (*h).sps.as_mut_ptr());
    if nal_end(h) != 0 {
        return -1;
    }
    nal_start(h, NAL_PPS as c_int, NAL_PRIORITY_HIGHEST as c_int);
    x264_10_pps_write(
        &mut (*h).out.bs,
        (*h).sps.as_mut_ptr(),
        (*h).pps.as_mut_ptr(),
    );
    if nal_end(h) != 0 {
        return -1;
    }
    nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
    if x264_10_sei_version_write(h, &mut (*h).out.bs) != 0 {
        return -1;
    }
    if nal_end(h) != 0 {
        return -1;
    }
    frame_size = encoder_encapsulate_nals(h, 0 as c_int);
    if frame_size < 0 as c_int {
        return -1;
    }
    *pi_nal = (*h).out.i_nal;
    *pp_nal = &mut *(*h).out.nal.offset(0) as *mut x264_nal_t;
    (*h).out.i_nal = 0 as c_int;
    return frame_size;
}
#[inline]
#[c2rust::src_loc = "2138:1"]
unsafe extern "C" fn reference_check_reorder(mut h: *mut x264_t) {
    let mut i: c_int = 0 as c_int;
    while !(*h).frames.reference[i as usize].is_null() {
        if (*(*h).frames.reference[i as usize]).b_corrupt != 0 {
            (*h).b_ref_reorder[0] = 1 as c_int;
            return;
        }
        i += 1;
    }
    let mut list: c_int = 0 as c_int;
    while list <= ((*h).sh.i_type == SLICE_TYPE_B as c_int) as c_int {
        let mut i_0: c_int = 0 as c_int;
        while i_0 < (*h).i_ref[list as usize] - 1 as c_int {
            let mut framenum_diff: c_int = (*(*h).fref[list as usize][(i_0 + 1 as c_int) as usize])
                .i_frame_num
                - (*(*h).fref[list as usize][i_0 as usize]).i_frame_num;
            let mut poc_diff: c_int = (*(*h).fref[list as usize][(i_0 + 1 as c_int) as usize])
                .i_poc
                - (*(*h).fref[list as usize][i_0 as usize]).i_poc;
            if if (*h).sh.i_type == SLICE_TYPE_P as c_int {
                (framenum_diff > 0 as c_int) as c_int
            } else if list == 1 as c_int {
                (poc_diff < 0 as c_int) as c_int
            } else {
                (poc_diff > 0 as c_int) as c_int
            } != 0
            {
                (*h).b_ref_reorder[list as usize] = 1 as c_int;
                return;
            }
            i_0 += 1;
        }
        list += 1;
    }
}
#[c2rust::src_loc = "2163:1"]
unsafe extern "C" fn weighted_reference_duplicate(
    mut h: *mut x264_t,
    mut i_ref: c_int,
    mut w: *const x264_weight_t,
) -> c_int {
    let mut i: c_int = (*h).i_ref[0];
    let mut j: c_int = 1 as c_int;
    let mut newframe: *mut x264_frame_t = 0 as *mut x264_frame_t;
    if i <= 1 as c_int {
        return -1;
    }
    if (*h).param.analyse.i_weighted_pred != X264_WEIGHTP_SMART {
        return -1;
    }
    if BIT_DEPTH > 8 as c_int && w != x264_zero.as_mut_ptr() as *const x264_weight_t {
        return -1;
    }
    newframe = x264_10_frame_pop_blank_unused(h);
    if newframe.is_null() {
        return -1;
    }
    *newframe = *(*h).fref[0][i_ref as usize];
    (*newframe).i_reference_count = 1 as c_int;
    (*newframe).orig = (*h).fref[0][i_ref as usize] as *mut x264_frame;
    (*newframe).b_duplicate = 1 as c_int;
    memcpy(
        (*(*(*h).fenc).weight.as_mut_ptr().offset(j as isize)).as_mut_ptr() as *mut c_void,
        w as *const c_void,
        size_of::<[x264_weight_t; 3]>() as size_t,
    );
    (*h).b_ref_reorder[0] = 1 as c_int;
    if (*h).i_ref[0] < X264_REF_MAX {
        (*h).i_ref[0] += 1;
    }
    (*h).fref[0][(X264_REF_MAX - 1 as c_int) as usize] = 0 as *mut x264_frame_t;
    x264_10_frame_unshift(
        &mut *(*(*h).fref.as_mut_ptr().offset(0))
            .as_mut_ptr()
            .offset(j as isize),
        newframe,
    );
    return j;
}
#[c2rust::src_loc = "2202:1"]
unsafe extern "C" fn weighted_pred_init(mut h: *mut x264_t) {
    let mut i_ref: c_int = 0 as c_int;
    while i_ref < (*h).i_ref[0] {
        (*(*h).fenc).weighted[i_ref as usize] = (*(*h).fref[0][i_ref as usize]).filtered[0][0];
        i_ref += 1;
    }
    (*(*h).fenc).i_lines_weighted = 0 as c_int;
    let mut i_ref_0: c_int = 0 as c_int;
    while i_ref_0 < (*h).i_ref[0] << (*h).sh.b_mbaff {
        let mut i: c_int = 0 as c_int;
        while i < 3 as c_int {
            (*h).sh.weight[i_ref_0 as usize][i as usize].weightfn = 0 as *mut weight_fn_t;
            i += 1;
        }
        i_ref_0 += 1;
    }
    if (*h).sh.i_type != SLICE_TYPE_P as c_int || (*h).param.analyse.i_weighted_pred <= 0 as c_int {
        return;
    }
    let mut i_padv: c_int = PADV << (*h).param.b_interlaced;
    let mut denom: c_int = -1;
    let mut weightplane: [c_int; 2] = [0 as c_int, 0 as c_int];
    let mut buffer_next: c_int = 0 as c_int;
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 3 as c_int {
        let mut j: c_int = 0 as c_int;
        while j < (*h).i_ref[0] {
            if !(*(*h).fenc).weight[j as usize][i_0 as usize]
                .weightfn
                .is_null()
            {
                (*h).sh.weight[j as usize][i_0 as usize] =
                    (*(*h).fenc).weight[j as usize][i_0 as usize];
                if (*h).sh.weight[j as usize][i_0 as usize].i_scale
                    == (1 as int32_t) << (*h).sh.weight[j as usize][i_0 as usize].i_denom
                    && (*h).sh.weight[j as usize][i_0 as usize].i_offset == 0 as int32_t
                {
                    (*h).sh.weight[j as usize][i_0 as usize].weightfn = 0 as *mut weight_fn_t;
                } else {
                    if weightplane[(i_0 != 0) as c_int as usize] == 0 {
                        weightplane[(i_0 != 0) as c_int as usize] = 1 as c_int;
                        denom = (*h).sh.weight[j as usize][i_0 as usize].i_denom as c_int;
                        (*h).sh.weight[0][(i_0 != 0) as c_int as usize].i_denom = denom as int32_t;
                        if x264_clip3(denom, 0 as c_int, 7 as c_int) == denom {
                        } else {
                            __assert_fail(
                                b"x264_clip3( denom, 0, 7 ) == denom\0" as *const u8
                                    as *const c_char,
                                b"encoder/encoder.c\0" as *const u8 as *const c_char,
                                2240 as c_uint,
                                ::core::mem::transmute::<[u8; 34], [c_char; 34]>(
                                    *b"void weighted_pred_init(x264_t *)\0",
                                )
                                .as_ptr(),
                            );
                        }
                    }
                    if (*h).sh.weight[j as usize][i_0 as usize].i_denom == denom as int32_t {
                    } else {
                        __assert_fail(
                            b"h->sh.weight[j][i].i_denom == denom\0" as *const u8 as *const c_char,
                            b"encoder/encoder.c\0" as *const u8 as *const c_char,
                            2243 as c_uint,
                            ::core::mem::transmute::<[u8; 34], [c_char; 34]>(
                                *b"void weighted_pred_init(x264_t *)\0",
                            )
                            .as_ptr(),
                        );
                    }
                    if i_0 == 0 {
                        let fresh3 = buffer_next;
                        buffer_next = buffer_next + 1;
                        (*(*h).fenc).weighted[j as usize] = (*h).mb.p_weight_buf[fresh3 as usize]
                            .offset(((*(*h).fenc).i_stride[0] * i_padv) as isize)
                            .offset(
                                (if 32 > 64 / size_of::<pixel>() {
                                    32
                                } else {
                                    64 / size_of::<pixel>()
                                }) as isize,
                            );
                        if (*h).param.i_threads == 1 as c_int {
                            let mut src: *mut pixel = (*(*h).fref[0][j as usize]).filtered[0][0]
                                .offset(
                                    -(((*(*h).fref[0][j as usize]).i_stride[0] * i_padv) as isize),
                                )
                                .offset(
                                    -((if 32 as c_int > 64 as c_int / size_of::<pixel>() as c_int {
                                        32 as c_int
                                    } else {
                                        64 as c_int / size_of::<pixel>() as c_int
                                    }) as isize),
                                );
                            let mut dst: *mut pixel = (*(*h).fenc).weighted[j as usize]
                                .offset(-(((*(*h).fenc).i_stride[0] * i_padv) as isize))
                                .offset(
                                    -((if 32 as c_int > 64 as c_int / size_of::<pixel>() as c_int {
                                        32 as c_int
                                    } else {
                                        64 as c_int / size_of::<pixel>() as c_int
                                    }) as isize),
                                );
                            let mut stride: c_int = (*(*h).fenc).i_stride[0];
                            let mut width: c_int = (*(*h).fenc).i_width[0]
                                + ((if 32 as c_int > 64 as c_int / size_of::<pixel>() as c_int {
                                    32 as c_int
                                } else {
                                    64 as c_int / size_of::<pixel>() as c_int
                                }) + PADH);
                            let mut height: c_int = (*(*h).fenc).i_lines[0] + i_padv * 2 as c_int;
                            x264_10_weight_scale_plane(
                                h,
                                dst,
                                stride as intptr_t,
                                src,
                                stride as intptr_t,
                                width,
                                height,
                                &mut *(*(*h).sh.weight.as_mut_ptr().offset(j as isize))
                                    .as_mut_ptr()
                                    .offset(0),
                            );
                            (*(*h).fenc).i_lines_weighted = height;
                        }
                    }
                }
            }
            j += 1;
        }
        i_0 += 1;
    }
    if weightplane[1] != 0 {
        let mut i_1: c_int = 0 as c_int;
        while i_1 < (*h).i_ref[0] {
            if !(*h).sh.weight[i_1 as usize][1].weightfn.is_null()
                && (*h).sh.weight[i_1 as usize][2].weightfn.is_null()
            {
                (*h).sh.weight[i_1 as usize][2].i_scale =
                    ((1 as c_int) << (*h).sh.weight[0][1].i_denom) as int32_t;
                (*h).sh.weight[i_1 as usize][2].i_offset = 0 as c_int as int32_t;
            } else if !(*h).sh.weight[i_1 as usize][2].weightfn.is_null()
                && (*h).sh.weight[i_1 as usize][1].weightfn.is_null()
            {
                (*h).sh.weight[i_1 as usize][1].i_scale =
                    ((1 as c_int) << (*h).sh.weight[0][1].i_denom) as int32_t;
                (*h).sh.weight[i_1 as usize][1].i_offset = 0 as c_int as int32_t;
            }
            i_1 += 1;
        }
    }
    if weightplane[0] == 0 {
        (*h).sh.weight[0][0].i_denom = 0 as c_int as int32_t;
    }
    if weightplane[1] == 0 {
        (*h).sh.weight[0][1].i_denom = 0 as c_int as int32_t;
    }
    (*h).sh.weight[0][2].i_denom = (*h).sh.weight[0][1].i_denom;
}
#[inline]
#[c2rust::src_loc = "2286:1"]
unsafe extern "C" fn reference_distance(mut h: *mut x264_t, mut frame: *mut x264_frame_t) -> c_int {
    if (*h).param.frame_packing == Some(FramePacking::TemporalInterleaved) {
        return abs(((*(*h).fenc).i_frame & !(1 as c_int)) - ((*frame).i_frame & !(1 as c_int)))
            + ((*(*h).fenc).i_frame & 1 as c_int != (*frame).i_frame & 1 as c_int) as c_int;
    } else {
        return abs((*(*h).fenc).i_frame - (*frame).i_frame);
    };
}
#[inline]
#[c2rust::src_loc = "2295:1"]
unsafe extern "C" fn reference_build_list(mut h: *mut x264_t, mut i_poc: c_int) {
    let mut b_ok: c_int = 0;
    (*h).i_ref[0] = 0 as c_int;
    (*h).mb.pic.i_fref[0] = (*h).i_ref[0];
    (*h).i_ref[1] = 0 as c_int;
    (*h).mb.pic.i_fref[1] = (*h).i_ref[1];
    if (*h).sh.i_type == SLICE_TYPE_I as c_int {
        return;
    }
    let mut i: c_int = 0 as c_int;
    while !(*h).frames.reference[i as usize].is_null() {
        if !((*(*h).frames.reference[i as usize]).b_corrupt != 0) {
            if (*(*h).frames.reference[i as usize]).i_poc < i_poc {
                let fresh4 = (*h).i_ref[0];
                (*h).i_ref[0] = (*h).i_ref[0] + 1;
                (*h).fref[0][fresh4 as usize] = (*h).frames.reference[i as usize];
            } else if (*(*h).frames.reference[i as usize]).i_poc > i_poc {
                let fresh5 = (*h).i_ref[1];
                (*h).i_ref[1] = (*h).i_ref[1] + 1;
                (*h).fref[1][fresh5 as usize] = (*h).frames.reference[i as usize];
            }
        }
        i += 1;
    }
    if (*h).sh.i_mmco_remove_from_end != 0 {
        loop {
            b_ok = 1 as c_int;
            let mut i_0: c_int = 0 as c_int;
            while i_0 < (*h).i_ref[0] - 1 as c_int {
                if (*(*h).fref[0][i_0 as usize]).i_frame
                    < (*(*h).fref[0][(i_0 + 1 as c_int) as usize]).i_frame
                {
                    let mut t: *mut x264_frame_t = (*h).fref[0][i_0 as usize];
                    (*h).fref[0][i_0 as usize] = (*h).fref[0][(i_0 + 1 as c_int) as usize];
                    (*h).fref[0][(i_0 + 1 as c_int) as usize] = t;
                    b_ok = 0 as c_int;
                    break;
                } else {
                    i_0 += 1;
                }
            }
            if !(b_ok == 0) {
                break;
            }
        }
        let mut i_1: c_int = (*h).i_ref[0] - 1 as c_int;
        while i_1 >= (*h).i_ref[0] - (*h).sh.i_mmco_remove_from_end {
            let mut diff: c_int = (*h).i_frame_num - (*(*h).fref[0][i_1 as usize]).i_frame_num;
            (*h).sh.mmco[(*h).sh.i_mmco_command_count as usize].i_poc =
                (*(*h).fref[0][i_1 as usize]).i_poc;
            let fresh6 = (*h).sh.i_mmco_command_count;
            (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_command_count + 1;
            (*h).sh.mmco[fresh6 as usize].i_difference_of_pic_nums = diff;
            i_1 -= 1;
        }
    }
    let mut list: c_int = 0 as c_int;
    while list < 2 as c_int {
        (*h).fref_nearest[list as usize] = (*h).fref[list as usize][0];
        loop {
            b_ok = 1 as c_int;
            let mut i_2: c_int = 0 as c_int;
            while i_2 < (*h).i_ref[list as usize] - 1 as c_int {
                if if list != 0 {
                    ((*(*h).fref[list as usize][(i_2 + 1 as c_int) as usize]).i_poc
                        < (*(*h).fref_nearest[list as usize]).i_poc) as c_int
                } else {
                    ((*(*h).fref[list as usize][(i_2 + 1 as c_int) as usize]).i_poc
                        > (*(*h).fref_nearest[list as usize]).i_poc) as c_int
                } != 0
                {
                    (*h).fref_nearest[list as usize] =
                        (*h).fref[list as usize][(i_2 + 1 as c_int) as usize];
                }
                if reference_distance(h, (*h).fref[list as usize][i_2 as usize])
                    > reference_distance(h, (*h).fref[list as usize][(i_2 + 1 as c_int) as usize])
                {
                    let mut t_0: *mut x264_frame_t = (*h).fref[list as usize][i_2 as usize];
                    (*h).fref[list as usize][i_2 as usize] =
                        (*h).fref[list as usize][(i_2 + 1 as c_int) as usize];
                    (*h).fref[list as usize][(i_2 + 1 as c_int) as usize] = t_0;
                    b_ok = 0 as c_int;
                    break;
                } else {
                    i_2 += 1;
                }
            }
            if !(b_ok == 0) {
                break;
            }
        }
        list += 1;
    }
    reference_check_reorder(h);
    (*h).i_ref[1] = if (*h).i_ref[1] < (*h).frames.i_max_ref1 {
        (*h).i_ref[1]
    } else {
        (*h).frames.i_max_ref1
    };
    (*h).i_ref[0] = if (*h).i_ref[0] < (*h).frames.i_max_ref0 {
        (*h).i_ref[0]
    } else {
        (*h).frames.i_max_ref0
    };
    (*h).i_ref[0] = if (*h).i_ref[0] < (*h).param.i_frame_reference {
        (*h).i_ref[0]
    } else {
        (*h).param.i_frame_reference
    };
    if ((*(*h).fenc).i_type == X264_TYPE_B || (*(*h).fenc).i_type == X264_TYPE_BREF)
        && (*h).param.b_bluray_compat != 0
    {
        (*h).i_ref[0] = if (*h).i_ref[0]
            < ((*(*h).fref[0][0]).i_type == 0x5 as c_int
                || (*(*h).fref[0][0]).i_type == 0x4 as c_int) as c_int
                + 1 as c_int
        {
            (*h).i_ref[0]
        } else {
            ((*(*h).fref[0][0]).i_type == 0x5 as c_int || (*(*h).fref[0][0]).i_type == 0x4 as c_int)
                as c_int
                + 1 as c_int
        };
    }
    if (*(*h).fenc).i_type == X264_TYPE_P {
        let mut idx: c_int = -1;
        if (*h).param.analyse.i_weighted_pred >= X264_WEIGHTP_SIMPLE {
            let mut w: [x264_weight_t; 3] = [x264_weight_t {
                cachea: [0; 8],
                cacheb: [0; 8],
                i_denom: 0,
                i_scale: 0,
                i_offset: 0,
                weightfn: 0 as *mut weight_fn_t,
            }; 3];
            w[2].weightfn = 0 as *mut weight_fn_t;
            w[1].weightfn = w[2].weightfn;
            if (*h).param.rc.b_stat_read != 0 {
                x264_10_ratecontrol_set_weights(h, (*h).fenc);
            }
            if (*(*h).fenc).weight[0][0].weightfn.is_null() {
                (*(*h).fenc).weight[0][0].i_denom = 0 as c_int as int32_t;
                w[0].i_scale = 1 as c_int as int32_t;
                w[0].i_denom = 0 as c_int as int32_t;
                w[0].i_offset = -1 as int32_t;
                (*h).mc.weight_cache.expect("non-null function pointer")(
                    h,
                    &mut *w.as_mut_ptr().offset(0),
                );
                idx = weighted_reference_duplicate(h, 0 as c_int, w.as_mut_ptr());
            } else {
                if (*(*h).fenc).weight[0][0].i_scale
                    == (1 as int32_t) << (*(*h).fenc).weight[0][0].i_denom
                {
                    (*(*h).fenc).weight[0][0].i_scale = 1 as c_int as int32_t;
                    (*(*h).fenc).weight[0][0].i_denom = 0 as c_int as int32_t;
                    (*(*h).fenc).weight[0][0].i_offset = (*(*h).fenc).weight[0][0].i_offset;
                    (*h).mc.weight_cache.expect("non-null function pointer")(
                        h,
                        &mut *(*(*(*h).fenc).weight.as_mut_ptr().offset(0))
                            .as_mut_ptr()
                            .offset(0),
                    );
                }
                weighted_reference_duplicate(
                    h,
                    0 as c_int,
                    x264_zero.as_mut_ptr() as *const x264_weight_t,
                );
                if (*(*h).fenc).weight[0][0].i_offset > -(128 as int32_t) {
                    w[0] = (*(*h).fenc).weight[0][0];
                    w[0].i_offset -= 1;
                    (*h).mc.weight_cache.expect("non-null function pointer")(
                        h,
                        &mut *w.as_mut_ptr().offset(0),
                    );
                    idx = weighted_reference_duplicate(h, 0 as c_int, w.as_mut_ptr());
                }
            }
        }
        (*h).mb.ref_blind_dupe = idx;
    }
    if (*h).i_ref[0] + (*h).i_ref[1] <= 16 as c_int {
    } else {
        __assert_fail(
            b"h->i_ref[0] + h->i_ref[1] <= X264_REF_MAX\0" as *const u8 as *const c_char,
            b"encoder/encoder.c\0" as *const u8 as *const c_char,
            2408 as c_uint,
            ::core::mem::transmute::<[u8; 41], [c_char; 41]>(
                *b"void reference_build_list(x264_t *, int)\0",
            )
            .as_ptr(),
        );
    }
    (*h).mb.pic.i_fref[0] = (*h).i_ref[0];
    (*h).mb.pic.i_fref[1] = (*h).i_ref[1];
}
#[c2rust::src_loc = "2413:1"]
unsafe extern "C" fn fdec_filter_row(mut h: *mut x264_t, mut mb_y: c_int, mut pass: c_int) {
    let mut b_hpel: c_int = (*(*h).fdec).b_kept_as_ref;
    let mut b_deblock: c_int = ((*h).sh.i_disable_deblocking_filter_idc != 1 as c_int) as c_int;
    let mut b_end: c_int = (mb_y == (*h).i_threadslice_end) as c_int;
    let mut b_measure_quality: c_int = 1 as c_int;
    let mut min_y: c_int = mb_y - ((1 as c_int) << (*h).sh.b_mbaff);
    let mut b_start: c_int = (min_y == (*h).i_threadslice_start) as c_int;
    let mut minpix_y: c_int = min_y * 16 as c_int - 4 as c_int * (b_start == 0) as c_int;
    let mut maxpix_y: c_int = mb_y * 16 as c_int - 4 as c_int * (b_end == 0) as c_int;
    b_deblock &= (b_hpel != 0 || (*h).param.b_full_recon != 0 || !(*h).param.psz_dump_yuv.is_null())
        as c_int;
    if (*h).param.b_sliced_threads != 0 {
        match pass {
            1 => {
                b_deblock &= ((*h).param.b_full_recon == 0) as c_int;
                b_hpel &= !(b_start != 0 && min_y > 0 as c_int) as c_int;
                b_measure_quality = 0 as c_int;
            }
            2 => {
                b_deblock = 0 as c_int;
                b_measure_quality = 0 as c_int;
            }
            0 | _ => {
                b_deblock &= (*h).param.b_full_recon;
                b_hpel = 0 as c_int;
            }
        }
    }
    if mb_y & (*h).sh.b_mbaff != 0 {
        return;
    }
    if min_y < (*h).i_threadslice_start {
        return;
    }
    if b_deblock != 0 {
        let mut y: c_int = min_y;
        while y < mb_y {
            x264_10_frame_deblock_row(h, y);
            y += (1 as c_int) << (*h).sh.b_mbaff;
        }
    }
    if (*h).param.b_interlaced != 0 && ((*h).param.b_sliced_threads == 0 || pass == 1 as c_int) {
        let mut p: c_int = 0 as c_int;
        while p < (*(*h).fdec).i_plane {
            let mut i: c_int = minpix_y >> ((*h).mb.chroma_v_shift != 0 && p != 0) as c_int;
            while i < maxpix_y >> ((*h).mb.chroma_v_shift != 0 && p != 0) as c_int {
                memcpy(
                    (*(*h).fdec).plane_fld[p as usize]
                        .offset((i * (*(*h).fdec).i_stride[p as usize]) as isize)
                        as *mut c_void,
                    (*(*h).fdec).plane[p as usize]
                        .offset((i * (*(*h).fdec).i_stride[p as usize]) as isize)
                        as *const c_void,
                    ((*h).mb.i_mb_width * 16 as c_int * SIZEOF_PIXEL) as size_t,
                );
                i += 1;
            }
            p += 1;
        }
    }
    if (*(*h).fdec).b_kept_as_ref != 0 && ((*h).param.b_sliced_threads == 0 || pass == 1 as c_int) {
        x264_10_frame_expand_border(h, (*h).fdec, min_y);
    }
    if b_hpel != 0 {
        let mut end: c_int = (mb_y == (*h).mb.i_mb_height) as c_int;
        if (*h).param.analyse.i_subpel_refine != 0 {
            x264_10_frame_filter(h, (*h).fdec, min_y, end);
            x264_10_frame_expand_border_filtered(h, (*h).fdec, min_y, end);
        }
    }
    if (*h).sh.b_mbaff != 0 && pass == 0 as c_int {
        let mut i_0: c_int = 0 as c_int;
        while i_0 < 3 as c_int {
            let mut t: *mut pixel = (*h).intra_border_backup[0][i_0 as usize];
            (*h).intra_border_backup[0][i_0 as usize] = (*h).intra_border_backup[3][i_0 as usize];
            (*h).intra_border_backup[3][i_0 as usize] = t;
            let mut t_0: *mut pixel = (*h).intra_border_backup[1][i_0 as usize];
            (*h).intra_border_backup[1][i_0 as usize] = (*h).intra_border_backup[4][i_0 as usize];
            (*h).intra_border_backup[4][i_0 as usize] = t_0;
            i_0 += 1;
        }
    }
    if (*h).i_thread_frames > 1 as c_int && (*(*h).fdec).b_kept_as_ref != 0 {
        x264_10_frame_cond_broadcast(
            (*h).fdec,
            mb_y * 16 as c_int
                + (if b_end != 0 {
                    10000 as c_int
                } else {
                    -(X264_THREAD_HEIGHT << (*h).sh.b_mbaff)
                }),
        );
    }
    if b_measure_quality != 0 {
        maxpix_y = if maxpix_y < (*h).param.height as c_int {
            maxpix_y
        } else {
            (*h).param.height as c_int
        };
        if (*h).param.analyse.b_psnr != 0 {
            let mut p_0: c_int = 0 as c_int;
            while p_0
                < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                    3 as c_int
                } else {
                    1 as c_int
                })
            {
                (*h).stat.frame.i_ssd[p_0 as usize] = ((*h).stat.frame.i_ssd[p_0 as usize]
                    as uint64_t)
                    .wrapping_add(x264_10_pixel_ssd_wxh(
                        &mut (*h).pixf,
                        (*(*h).fdec).plane[p_0 as usize]
                            .offset((minpix_y * (*(*h).fdec).i_stride[p_0 as usize]) as isize),
                        (*(*h).fdec).i_stride[p_0 as usize] as intptr_t,
                        (*(*h).fenc).plane[p_0 as usize]
                            .offset((minpix_y * (*(*h).fenc).i_stride[p_0 as usize]) as isize),
                        (*(*h).fenc).i_stride[p_0 as usize] as intptr_t,
                        (*h).param.width as c_int,
                        maxpix_y - minpix_y,
                    )) as int64_t as int64_t;
                p_0 += 1;
            }
            if !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int) {
                let mut ssd_u: uint64_t = 0;
                let mut ssd_v: uint64_t = 0;
                let mut v_shift: c_int = (*h).mb.chroma_v_shift;
                x264_10_pixel_ssd_nv12(
                    &mut (*h).pixf,
                    (*(*h).fdec).plane[1]
                        .offset(((minpix_y >> v_shift) * (*(*h).fdec).i_stride[1]) as isize),
                    (*(*h).fdec).i_stride[1] as intptr_t,
                    (*(*h).fenc).plane[1]
                        .offset(((minpix_y >> v_shift) * (*(*h).fenc).i_stride[1]) as isize),
                    (*(*h).fenc).i_stride[1] as intptr_t,
                    (*h).param.width as c_int >> 1,
                    maxpix_y - minpix_y >> v_shift,
                    &mut ssd_u,
                    &mut ssd_v,
                );
                (*h).stat.frame.i_ssd[1] = ((*h).stat.frame.i_ssd[1] as uint64_t)
                    .wrapping_add(ssd_u) as int64_t
                    as int64_t;
                (*h).stat.frame.i_ssd[2] = ((*h).stat.frame.i_ssd[2] as uint64_t)
                    .wrapping_add(ssd_v) as int64_t
                    as int64_t;
            }
        }
        if (*h).param.analyse.b_ssim != 0 {
            let mut ssim_cnt: c_int = 0;
            minpix_y += if b_start != 0 {
                2 as c_int
            } else {
                -(6 as c_int)
            };
            (*h).stat.frame.f_ssim += x264_10_pixel_ssim_wxh(
                &mut (*h).pixf,
                (*(*h).fdec).plane[0]
                    .offset(2)
                    .offset((minpix_y * (*(*h).fdec).i_stride[0]) as isize),
                (*(*h).fdec).i_stride[0] as intptr_t,
                (*(*h).fenc).plane[0]
                    .offset(2)
                    .offset((minpix_y * (*(*h).fenc).i_stride[0]) as isize),
                (*(*h).fenc).i_stride[0] as intptr_t,
                (*h).param.width as c_int - 2,
                maxpix_y - minpix_y,
                (*h).scratch_buffer,
                &mut ssim_cnt,
            ) as c_double;
            (*h).stat.frame.i_ssim_cnt += ssim_cnt;
        }
    }
}
#[inline]
#[c2rust::src_loc = "2533:1"]
unsafe extern "C" fn reference_update(mut h: *mut x264_t) -> c_int {
    if (*(*h).fdec).b_kept_as_ref == 0 {
        if (*h).i_thread_frames > 1 as c_int {
            x264_10_frame_push_unused(h, (*h).fdec);
            (*h).fdec = x264_10_frame_pop_unused(h, 1 as c_int);
            if (*h).fdec.is_null() {
                return -1;
            }
        }
        return 0 as c_int;
    }
    let mut i: c_int = 0 as c_int;
    while i < (*h).sh.i_mmco_command_count {
        let mut j: c_int = 0 as c_int;
        while !(*h).frames.reference[j as usize].is_null() {
            if (*(*h).frames.reference[j as usize]).i_poc == (*h).sh.mmco[i as usize].i_poc {
                x264_10_frame_push_unused(
                    h,
                    x264_10_frame_shift(
                        &mut *(*h).frames.reference.as_mut_ptr().offset(j as isize),
                    ),
                );
            }
            j += 1;
        }
        i += 1;
    }
    x264_10_frame_push((*h).frames.reference.as_mut_ptr(), (*h).fdec);
    if !(*h).frames.reference[(*(*h).sps.as_mut_ptr()).i_num_ref_frames as usize].is_null() {
        x264_10_frame_push_unused(h, x264_10_frame_shift((*h).frames.reference.as_mut_ptr()));
    }
    (*h).fdec = x264_10_frame_pop_unused(h, 1 as c_int);
    if (*h).fdec.is_null() {
        return -1;
    }
    return 0 as c_int;
}
#[inline]
#[c2rust::src_loc = "2563:1"]
unsafe extern "C" fn reference_reset(mut h: *mut x264_t) {
    while !(*h).frames.reference[0].is_null() {
        x264_10_frame_push_unused(h, x264_10_frame_pop((*h).frames.reference.as_mut_ptr()));
    }
    (*(*h).fenc).i_poc = 0 as c_int;
    (*(*h).fdec).i_poc = (*(*h).fenc).i_poc;
}
#[inline]
#[c2rust::src_loc = "2571:1"]
unsafe extern "C" fn reference_hierarchy_reset(mut h: *mut x264_t) {
    let mut ref_0: c_int = 0;
    let mut b_hasdelayframe: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while !(*(*h).frames.current.offset(i as isize)).is_null()
        && (**(*h).frames.current.offset(i as isize)).i_type == X264_TYPE_B
    {
        b_hasdelayframe |= ((**(*h).frames.current.offset(i as isize)).i_coded
            != (**(*h).frames.current.offset(i as isize)).i_frame
                + (*(*h).sps.as_mut_ptr()).vui.i_num_reorder_frames)
            as c_int;
        i += 1;
    }
    if (*h).param.bframe_pyramid != BPyramid::Strict
        && b_hasdelayframe == 0
        && (*h).frames.i_poc_last_open_gop == -1
    {
        return;
    }
    ref_0 = 0 as c_int;
    while !(*h).frames.reference[ref_0 as usize].is_null() {
        if (*h).param.bframe_pyramid == BPyramid::Strict
            && (*(*h).frames.reference[ref_0 as usize]).i_type == X264_TYPE_BREF
            || (*(*h).frames.reference[ref_0 as usize]).i_poc < (*h).frames.i_poc_last_open_gop
                && (*h).sh.i_type != SLICE_TYPE_B as c_int
        {
            let mut diff: c_int =
                (*h).i_frame_num - (*(*h).frames.reference[ref_0 as usize]).i_frame_num;
            (*h).sh.mmco[(*h).sh.i_mmco_command_count as usize].i_difference_of_pic_nums = diff;
            let fresh7 = (*h).sh.i_mmco_command_count;
            (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_command_count + 1;
            (*h).sh.mmco[fresh7 as usize].i_poc = (*(*h).frames.reference[ref_0 as usize]).i_poc;
            x264_10_frame_push_unused(
                h,
                x264_10_frame_shift(
                    &mut *(*h).frames.reference.as_mut_ptr().offset(ref_0 as isize),
                ),
            );
            (*h).b_ref_reorder[0] = 1 as c_int;
            ref_0 -= 1;
        }
        ref_0 += 1;
    }
    if (*h).param.bframe_pyramid != BPyramid::None {
        (*h).sh.i_mmco_remove_from_end = if ref_0 + 2 as c_int - (*h).frames.i_max_dpb > 0 as c_int
        {
            ref_0 + 2 as c_int - (*h).frames.i_max_dpb
        } else {
            0 as c_int
        };
    }
}
#[inline]
#[c2rust::src_loc = "2608:1"]
unsafe extern "C" fn slice_init(mut h: *mut x264_t, mut i_nal_type: c_int, mut i_global_qp: c_int) {
    if i_nal_type == NAL_SLICE_IDR as c_int {
        slice_header_init(
            h,
            &mut (*h).sh,
            (*h).sps.as_mut_ptr(),
            (*h).pps.as_mut_ptr(),
            (*h).i_idr_pic_id,
            (*h).i_frame_num,
            i_global_qp,
        );
        if (*h).param.i_avcintra_class != 0 {
            match (*h).i_idr_pic_id {
                5 => {
                    (*h).i_idr_pic_id = 3 as c_int;
                }
                3 => {
                    (*h).i_idr_pic_id = 4 as c_int;
                }
                4 | _ => {
                    (*h).i_idr_pic_id = 5 as c_int;
                }
            }
        } else {
            (*h).i_idr_pic_id ^= 1 as c_int;
        }
    } else {
        slice_header_init(
            h,
            &mut (*h).sh,
            (*h).sps.as_mut_ptr(),
            (*h).pps.as_mut_ptr(),
            -1,
            (*h).i_frame_num,
            i_global_qp,
        );
        (*h).sh.i_num_ref_idx_l0_active = if (*h).i_ref[0] <= 0 as c_int {
            1 as c_int
        } else {
            (*h).i_ref[0]
        };
        (*h).sh.i_num_ref_idx_l1_active = if (*h).i_ref[1] <= 0 as c_int {
            1 as c_int
        } else {
            (*h).i_ref[1]
        };
        if (*h).sh.i_num_ref_idx_l0_active
            != (*(*h).pps.as_mut_ptr()).i_num_ref_idx_l0_default_active
            || (*h).sh.i_type == SLICE_TYPE_B as c_int
                && (*h).sh.i_num_ref_idx_l1_active
                    != (*(*h).pps.as_mut_ptr()).i_num_ref_idx_l1_default_active
        {
            (*h).sh.b_num_ref_idx_override = 1 as c_int;
        }
    }
    if (*(*h).fenc).i_type == X264_TYPE_BREF
        && (*h).param.b_bluray_compat != 0
        && (*h).sh.i_mmco_command_count != 0
    {
        (*h).b_sh_backup = 1 as c_int;
        (*h).sh_backup = (*h).sh;
    }
    (*(*h).fdec).i_frame_num = (*h).sh.i_frame_num;
    if (*(*h).sps.as_mut_ptr()).i_poc_type == 0 as c_int {
        (*h).sh.i_poc = (*(*h).fdec).i_poc;
        if (*h).param.b_interlaced != 0 {
            (*h).sh.i_delta_poc_bottom = if (*h).param.b_tff != 0 {
                1 as c_int
            } else {
                -1
            };
            (*h).sh.i_poc += ((*h).sh.i_delta_poc_bottom == -1) as c_int;
        } else {
            (*h).sh.i_delta_poc_bottom = 0 as c_int;
        }
        (*(*h).fdec).i_delta_poc[0] = ((*h).sh.i_delta_poc_bottom == -1) as c_int;
        (*(*h).fdec).i_delta_poc[1] = ((*h).sh.i_delta_poc_bottom == 1 as c_int) as c_int;
    }
    x264_10_macroblock_slice_init(h);
}
#[inline(always)]
#[c2rust::src_loc = "2689:1"]
unsafe extern "C" fn bitstream_backup(
    mut h: *mut x264_t,
    mut bak: *mut x264_bs_bak_t,
    mut i_skip: c_int,
    mut full: c_int,
) {
    if full != 0 {
        (*bak).stat = (*h).stat.frame;
        (*bak).last_qp = (*h).mb.i_last_qp;
        (*bak).last_dqp = (*h).mb.i_last_dqp;
        (*bak).field_decoding_flag = (*h).mb.field_decoding_flag;
    } else {
        (*bak).stat.i_mv_bits = (*h).stat.frame.i_mv_bits;
        (*bak).stat.i_tex_bits = (*h).stat.frame.i_tex_bits;
    }
    if (*h).param.b_cabac != 0 {
        if full != 0 {
            memcpy(
                &mut (*bak).cabac as *mut x264_cabac_t as *mut c_void,
                &mut (*h).cabac as *mut x264_cabac_t as *const c_void,
                size_of::<x264_cabac_t>() as size_t,
            );
        } else {
            memcpy(
                &mut (*bak).cabac as *mut x264_cabac_t as *mut c_void,
                &mut (*h).cabac as *mut x264_cabac_t as *const c_void,
                64 as size_t,
            );
        }
        (*bak).cabac_prevbyte = *(*h).cabac.p.offset(-1 as isize);
    } else {
        (*bak).bs = (*h).out.bs;
        (*bak).skip = i_skip;
    };
}
#[inline(always)]
#[c2rust::src_loc = "2723:1"]
unsafe extern "C" fn bitstream_restore(
    mut h: *mut x264_t,
    mut bak: *mut x264_bs_bak_t,
    mut skip: *mut c_int,
    mut full: c_int,
) {
    if full != 0 {
        (*h).stat.frame = (*bak).stat;
        (*h).mb.i_last_qp = (*bak).last_qp;
        (*h).mb.i_last_dqp = (*bak).last_dqp;
        (*h).mb.field_decoding_flag = (*bak).field_decoding_flag;
    } else {
        (*h).stat.frame.i_mv_bits = (*bak).stat.i_mv_bits;
        (*h).stat.frame.i_tex_bits = (*bak).stat.i_tex_bits;
    }
    if (*h).param.b_cabac != 0 {
        if full != 0 {
            memcpy(
                &mut (*h).cabac as *mut x264_cabac_t as *mut c_void,
                &mut (*bak).cabac as *mut x264_cabac_t as *const c_void,
                size_of::<x264_cabac_t>() as size_t,
            );
        } else {
            memcpy(
                &mut (*h).cabac as *mut x264_cabac_t as *mut c_void,
                &mut (*bak).cabac as *mut x264_cabac_t as *const c_void,
                64 as size_t,
            );
        }
        *(*h).cabac.p.offset(-1 as isize) = (*bak).cabac_prevbyte;
    } else {
        (*h).out.bs = (*bak).bs;
        *skip = (*bak).skip;
    };
}
#[c2rust::src_loc = "2752:1"]
unsafe extern "C" fn slice_write(mut h: *mut x264_t) -> intptr_t {
    let mut i_skip: c_int = 0;
    let mut mb_xy: c_int = 0;
    let mut i_mb_x: c_int = 0;
    let mut i_mb_y: c_int = 0;
    let mut overhead_guess: c_int = NALU_OVERHEAD
        - ((*h).param.b_annexb != 0 && (*h).out.i_nal != 0) as c_int
        + 1 as c_int
        + (*h).param.b_cabac
        + 5 as c_int;
    let mut slice_max_size: c_int = if (*h).param.i_slice_max_size > 0 as c_int {
        ((*h).param.i_slice_max_size - overhead_guess) * 8 as c_int
    } else {
        0 as c_int
    };
    let mut back_up_bitstream_cavlc: c_int = ((*h).param.b_cabac == 0
        && (*(*h).sps.as_mut_ptr()).i_profile_idc < PROFILE_HIGH as c_int)
        as c_int;
    let mut back_up_bitstream: c_int =
        (slice_max_size != 0 || back_up_bitstream_cavlc != 0) as c_int;
    let mut starting_bits: c_int = bs_pos(&mut (*h).out.bs);
    let mut b_deblock: c_int = ((*h).sh.i_disable_deblocking_filter_idc != 1 as c_int) as c_int;
    let mut b_hpel: c_int = (*(*h).fdec).b_kept_as_ref;
    let mut orig_last_mb: c_int = (*h).sh.i_last_mb;
    let mut thread_last_mb: c_int = (*h).i_threadslice_end * (*h).mb.i_mb_width - 1 as c_int;
    let mut last_emu_check: *mut uint8_t = 0 as *mut uint8_t;
    let mut bs_bak: [x264_bs_bak_t; 4] = [x264_bs_bak_t {
        skip: 0,
        cabac_prevbyte: 0,
        bs: bs_s {
            p_start: 0 as *mut uint8_t,
            p: 0 as *mut uint8_t,
            p_end: 0 as *mut uint8_t,
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        },
        cabac: x264_cabac_t {
            i_low: 0,
            i_range: 0,
            i_queue: 0,
            i_bytes_outstanding: 0,
            p_start: 0 as *mut uint8_t,
            p: 0 as *mut uint8_t,
            p_end: 0 as *mut uint8_t,
            f8_bits_encoded: 0,
            state: [0; 1024],
            padding: [0; 12],
        },
        stat: x264_frame_stat_t {
            i_mv_bits: 0,
            i_tex_bits: 0,
            i_misc_bits: 0,
            i_mb_count: [0; 19],
            i_mb_count_i: 0,
            i_mb_count_p: 0,
            i_mb_count_skip: 0,
            i_mb_count_8x8dct: [0; 2],
            i_mb_count_ref: [[0; 32]; 2],
            i_mb_partition: [0; 17],
            i_mb_cbp: [0; 6],
            i_mb_pred_mode: [[0; 13]; 4],
            i_mb_field: [0; 3],
            i_direct_score: [0; 2],
            i_ssd: [0; 3],
            f_ssim: 0.,
            i_ssim_cnt: 0,
        },
        last_qp: 0,
        last_dqp: 0,
        field_decoding_flag: 0,
    }; 4];
    b_deblock &= (b_hpel != 0 || (*h).param.b_full_recon != 0 || !(*h).param.psz_dump_yuv.is_null())
        as c_int;
    bs_realign(&mut (*h).out.bs);
    nal_start(h, (*h).i_nal_type, (*h).i_nal_ref_idc);
    (*(*h).out.nal.offset((*h).out.i_nal as isize)).i_first_mb = (*h).sh.i_first_mb;
    x264_10_macroblock_thread_init(h);
    (*h).mb.i_mb_xy = (*h).sh.i_first_mb;
    (*h).sh.i_qp = x264_10_ratecontrol_mb_qp(h);
    (*h).sh.i_qp = if (*h).sh.i_qp < 51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int) {
        (*h).sh.i_qp
    } else {
        51 as c_int + 6 as c_int * (10 as c_int - 8 as c_int)
    };
    (*h).sh.i_qp_delta = (*h).sh.i_qp - (*(*h).pps.as_mut_ptr()).i_pic_init_qp;
    slice_header_write(&mut (*h).out.bs, &mut (*h).sh, (*h).i_nal_ref_idc);
    if (*h).param.b_cabac != 0 {
        bs_align_1(&mut (*h).out.bs);
        x264_10_cabac_context_init(
            h,
            &mut (*h).cabac,
            (*h).sh.i_type,
            x264_clip3((*h).sh.i_qp - QP_BD_OFFSET, 0 as c_int, 51 as c_int),
            (*h).sh.i_cabac_init_idc,
        );
        x264_10_cabac_encode_init(&mut (*h).cabac, (*h).out.bs.p, (*h).out.bs.p_end);
        last_emu_check = (*h).cabac.p;
    } else {
        last_emu_check = (*h).out.bs.p;
    }
    (*h).mb.i_last_qp = (*h).sh.i_qp;
    (*h).mb.i_last_dqp = 0 as c_int;
    (*h).mb.field_decoding_flag = 0 as c_int;
    i_mb_y = (*h).sh.i_first_mb / (*h).mb.i_mb_width;
    i_mb_x = (*h).sh.i_first_mb % (*h).mb.i_mb_width;
    i_skip = 0 as c_int;
    loop {
        mb_xy = i_mb_x + i_mb_y * (*h).mb.i_mb_width;
        let mut mb_spos: c_int = bs_pos(&mut (*h).out.bs) + x264_cabac_pos(&mut (*h).cabac);
        if i_mb_x == 0 as c_int {
            if bitstream_check_buffer(h) != 0 {
                return -1 as intptr_t;
            }
            if i_mb_y & (*h).sh.b_mbaff == 0 && (*h).param.rc.i_vbv_buffer_size != 0 {
                bitstream_backup(
                    h,
                    &mut *bs_bak.as_mut_ptr().offset(BS_BAK_ROW_VBV as isize),
                    i_skip,
                    1 as c_int,
                );
            }
            if (*h).mb.b_reencode_mb == 0 {
                fdec_filter_row(h, i_mb_y, 0 as c_int);
            }
        }
        if back_up_bitstream != 0 {
            if back_up_bitstream_cavlc != 0 {
                bitstream_backup(
                    h,
                    &mut *bs_bak.as_mut_ptr().offset(BS_BAK_CAVLC_OVERFLOW as isize),
                    i_skip,
                    0 as c_int,
                );
            }
            if slice_max_size != 0 && i_mb_y & (*h).sh.b_mbaff == 0 {
                bitstream_backup(
                    h,
                    &mut *bs_bak.as_mut_ptr().offset(BS_BAK_SLICE_MAX_SIZE as isize),
                    i_skip,
                    0 as c_int,
                );
                if thread_last_mb + 1 as c_int - mb_xy == (*h).param.i_slice_min_mbs {
                    bitstream_backup(
                        h,
                        &mut *bs_bak.as_mut_ptr().offset(BS_BAK_SLICE_MIN_MBS as isize),
                        i_skip,
                        0 as c_int,
                    );
                }
            }
        }
        if (*h).param.b_interlaced != 0 {
            if (*h).mb.b_adaptive_mbaff != 0 {
                if i_mb_y & 1 as c_int == 0 {
                    (*h).mb.b_interlaced = x264_10_field_vsad(h, i_mb_x, i_mb_y);
                    memcpy(
                        &mut (*h).zigzagf as *mut x264_zigzag_function_t as *mut c_void,
                        (if (*h).mb.b_interlaced != 0 {
                            &mut (*h).zigzagf_interlaced as *mut x264_zigzag_function_t
                        } else {
                            &mut (*h).zigzagf_progressive as *mut x264_zigzag_function_t
                        }) as *const c_void,
                        size_of::<x264_zigzag_function_t>() as size_t,
                    );
                    if (*h).mb.b_interlaced == 0 && i_mb_y + 2 as c_int == (*h).mb.i_mb_height {
                        x264_10_expand_border_mbpair(h, i_mb_x, i_mb_y);
                    }
                }
            }
            *(*h).mb.field.offset(mb_xy as isize) = (*h).mb.b_interlaced as uint8_t;
        }
        if (*h).sh.b_mbaff != 0 {
            x264_10_macroblock_cache_load_interlaced(h, i_mb_x, i_mb_y);
        } else {
            x264_10_macroblock_cache_load_progressive(h, i_mb_x, i_mb_y);
        }
        x264_10_macroblock_analyse(h);
        loop {
            x264_10_macroblock_encode(h);
            if (*h).param.b_cabac != 0 {
                if mb_xy > (*h).sh.i_first_mb && !((*h).sh.b_mbaff != 0 && i_mb_y & 1 as c_int != 0)
                {
                    x264_10_cabac_encode_terminal_c(&mut (*h).cabac);
                }
                if (*h).mb.i_type == P_SKIP as c_int || (*h).mb.i_type == B_SKIP as c_int {
                    x264_10_cabac_mb_skip(h, 1 as c_int);
                } else {
                    if (*h).sh.i_type != SLICE_TYPE_I as c_int {
                        x264_10_cabac_mb_skip(h, 0 as c_int);
                    }
                    x264_10_macroblock_write_cabac(h, &mut (*h).cabac);
                }
                break;
            } else if (*h).mb.i_type == P_SKIP as c_int || (*h).mb.i_type == B_SKIP as c_int {
                i_skip += 1;
                break;
            } else {
                if (*h).sh.i_type != SLICE_TYPE_I as c_int {
                    bs_write_ue_big(&mut (*h).out.bs, i_skip as c_uint);
                    i_skip = 0 as c_int;
                }
                x264_10_macroblock_write_cavlc(h);
                if !((*h).mb.b_overflow != 0) {
                    break;
                }
                (*h).mb.i_qp += 1;
                (*h).mb.i_chroma_qp = *(*h).chroma_qp_table.offset((*h).mb.i_qp as isize) as c_int;
                (*h).mb.i_skip_intra = 0 as c_int;
                (*h).mb.b_skip_mc = 0 as c_int;
                (*h).mb.b_overflow = 0 as c_int;
                bitstream_restore(
                    h,
                    &mut *bs_bak.as_mut_ptr().offset(BS_BAK_CAVLC_OVERFLOW as isize),
                    &mut i_skip,
                    0 as c_int,
                );
            }
        }
        let mut total_bits: c_int = bs_pos(&mut (*h).out.bs) + x264_cabac_pos(&mut (*h).cabac);
        let mut mb_size: c_int = total_bits - mb_spos;
        if slice_max_size != 0 && ((*h).sh.b_mbaff == 0 || i_mb_y & 1 as c_int != 0) {
            if (*h).param.b_cabac == 0 {
                total_bits += bs_size_ue_big(i_skip as c_uint);
            }
            let mut end: *mut uint8_t = if (*h).param.b_cabac != 0 {
                (*h).cabac.p
            } else {
                (*h).out.bs.p
            };
            while last_emu_check < end.offset(-(2)) {
                if *last_emu_check.offset(0) as c_int == 0 as c_int
                    && *last_emu_check.offset(1) as c_int == 0 as c_int
                    && *last_emu_check.offset(2) as c_int <= 3 as c_int
                {
                    slice_max_size -= 8 as c_int;
                    last_emu_check = last_emu_check.offset(1);
                }
                last_emu_check = last_emu_check.offset(1);
            }
            if total_bits - starting_bits > slice_max_size && (*h).mb.b_reencode_mb == 0 {
                if x264_10_frame_new_slice(h, (*h).fdec) == 0 {
                    if mb_xy <= thread_last_mb
                        && thread_last_mb + 1 as c_int - mb_xy < (*h).param.i_slice_min_mbs
                    {
                        if thread_last_mb - (*h).param.i_slice_min_mbs
                            < (*h).sh.i_first_mb + (*h).param.i_slice_min_mbs
                        {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"slice-max-size violated (frame %d, cause: slice-min-mbs)\n\0"
                                    as *const u8 as *const c_char,
                                (*h).i_frame,
                            );
                            slice_max_size = 0 as c_int;
                        } else {
                            bitstream_restore(
                                h,
                                &mut *bs_bak.as_mut_ptr().offset(BS_BAK_SLICE_MIN_MBS as isize),
                                &mut i_skip,
                                0 as c_int,
                            );
                            (*h).mb.b_reencode_mb = 1 as c_int;
                            (*h).sh.i_last_mb = thread_last_mb - (*h).param.i_slice_min_mbs;
                            break;
                        }
                    } else if mb_xy - (*h).sh.b_mbaff * (*h).mb.i_mb_stride != (*h).sh.i_first_mb {
                        bitstream_restore(
                            h,
                            &mut *bs_bak.as_mut_ptr().offset(BS_BAK_SLICE_MAX_SIZE as isize),
                            &mut i_skip,
                            0 as c_int,
                        );
                        (*h).mb.b_reencode_mb = 1 as c_int;
                        if (*h).sh.b_mbaff != 0 {
                            if i_mb_x != 0 {
                                (*h).sh.i_last_mb = mb_xy - 1 as c_int
                                    + (*h).mb.i_mb_stride * (i_mb_y & 1 as c_int == 0) as c_int;
                            } else {
                                (*h).sh.i_last_mb = (i_mb_y - 2 as c_int
                                    + (i_mb_y & 1 as c_int == 0) as c_int)
                                    * (*h).mb.i_mb_stride
                                    + (*h).mb.i_mb_width
                                    - 1 as c_int;
                            }
                        } else {
                            (*h).sh.i_last_mb = mb_xy - 1 as c_int;
                        }
                        break;
                    } else {
                        (*h).sh.i_last_mb = mb_xy;
                    }
                } else {
                    slice_max_size = 0 as c_int;
                }
            }
        }
        (*h).mb.b_reencode_mb = 0 as c_int;
        x264_10_macroblock_cache_save(h);
        if x264_10_ratecontrol_mb(h, mb_size) < 0 as c_int {
            bitstream_restore(
                h,
                &mut *bs_bak.as_mut_ptr().offset(BS_BAK_ROW_VBV as isize),
                &mut i_skip,
                1 as c_int,
            );
            (*h).mb.b_reencode_mb = 1 as c_int;
            i_mb_x = 0 as c_int;
            i_mb_y = i_mb_y - (*h).sh.b_mbaff;
            (*h).mb.i_mb_prev_xy = i_mb_y * (*h).mb.i_mb_stride - 1 as c_int;
            (*h).sh.i_last_mb = orig_last_mb;
        } else {
            (*h).stat.frame.i_mb_count[(*h).mb.i_type as usize] += 1;
            let mut b_intra: c_int = ((*h).mb.i_type == I_4x4 as c_int
                || (*h).mb.i_type == I_8x8 as c_int
                || (*h).mb.i_type == I_16x16 as c_int
                || (*h).mb.i_type == I_PCM as c_int) as c_int;
            let mut b_skip: c_int =
                ((*h).mb.i_type == P_SKIP as c_int || (*h).mb.i_type == B_SKIP as c_int) as c_int;
            if (*h).param.i_log_level >= X264_LOG_INFO || (*h).param.rc.b_stat_write != 0 {
                if b_intra == 0 && b_skip == 0 && !((*h).mb.i_type == B_DIRECT as c_int) {
                    if (*h).mb.i_partition != D_8x8 as c_int {
                        (*h).stat.frame.i_mb_partition[(*h).mb.i_partition as usize] += 4 as c_int;
                    } else {
                        let mut i: c_int = 0 as c_int;
                        while i < 4 as c_int {
                            (*h).stat.frame.i_mb_partition
                                [(*h).mb.i_sub_partition[i as usize] as usize] += 1;
                            i += 1;
                        }
                    }
                    if (*h).param.i_frame_reference > 1 as c_int {
                        let mut i_list: c_int = 0 as c_int;
                        while i_list <= ((*h).sh.i_type == SLICE_TYPE_B as c_int) as c_int {
                            let mut i_0: c_int = 0 as c_int;
                            while i_0 < 4 as c_int {
                                let mut i_ref: c_int = (*h).mb.cache.ref_0[i_list as usize]
                                    [x264_scan8[(4 as c_int * i_0) as usize] as usize]
                                    as c_int;
                                if i_ref >= 0 as c_int {
                                    (*h).stat.frame.i_mb_count_ref[i_list as usize]
                                        [i_ref as usize] += 1;
                                }
                                i_0 += 1;
                            }
                            i_list += 1;
                        }
                    }
                }
            }
            if (*h).param.i_log_level >= X264_LOG_INFO {
                if (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma != 0 {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                        let mut i_1: c_int = 0 as c_int;
                        while i_1 < 4 as c_int {
                            if (*h).mb.i_cbp_luma & (1 as c_int) << i_1 != 0 {
                                let mut p: c_int = 0 as c_int;
                                while p < 3 as c_int {
                                    let mut s8: c_int = i_1 * 4 as c_int + p * 16 as c_int;
                                    let mut nnz8x8: c_int = (*(&mut *(*h)
                                        .mb
                                        .cache
                                        .non_zero_count
                                        .as_mut_ptr()
                                        .offset(
                                            (*x264_scan8.as_ptr().offset(s8 as isize) as c_int
                                                + 0 as c_int)
                                                as isize,
                                        )
                                        as *mut uint8_t
                                        as *mut x264_union16_t))
                                        .i
                                        as c_int
                                        | (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                                            (*x264_scan8.as_ptr().offset(s8 as isize) as c_int
                                                + 8 as c_int)
                                                as isize,
                                        )
                                            as *mut uint8_t
                                            as *mut x264_union16_t))
                                            .i as c_int;
                                    (*h).stat.frame.i_mb_cbp
                                        [((b_intra == 0) as c_int + p * 2 as c_int) as usize] +=
                                        (nnz8x8 != 0) as c_int;
                                    p += 1;
                                }
                            }
                            i_1 += 1;
                        }
                    } else {
                        let mut cbpsum: c_int = ((*h).mb.i_cbp_luma & 1 as c_int)
                            + ((*h).mb.i_cbp_luma >> 1 as c_int & 1 as c_int)
                            + ((*h).mb.i_cbp_luma >> 2 as c_int & 1 as c_int)
                            + ((*h).mb.i_cbp_luma >> 3 as c_int);
                        (*h).stat.frame.i_mb_cbp
                            [((b_intra == 0) as c_int + 0 as c_int) as usize] += cbpsum;
                        (*h).stat.frame.i_mb_cbp
                            [((b_intra == 0) as c_int + 2 as c_int) as usize] +=
                            ((*h).mb.i_cbp_chroma != 0) as c_int;
                        (*h).stat.frame.i_mb_cbp
                            [((b_intra == 0) as c_int + 4 as c_int) as usize] +=
                            (*h).mb.i_cbp_chroma >> 1 as c_int;
                    }
                }
                if (*h).mb.i_cbp_luma != 0 && b_intra == 0 {
                    (*h).stat.frame.i_mb_count_8x8dct[0] += 1;
                    (*h).stat.frame.i_mb_count_8x8dct[1] += (*h).mb.b_transform_8x8;
                }
                if b_intra != 0 && (*h).mb.i_type != I_PCM as c_int {
                    if (*h).mb.i_type == I_16x16 as c_int {
                        (*h).stat.frame.i_mb_pred_mode[0]
                            [(*h).mb.i_intra16x16_pred_mode as usize] += 1;
                    } else if (*h).mb.i_type == I_8x8 as c_int {
                        let mut i_2: c_int = 0 as c_int;
                        while i_2 < 16 as c_int {
                            (*h).stat.frame.i_mb_pred_mode[1][(*h).mb.cache.intra4x4_pred_mode
                                [x264_scan8[i_2 as usize] as usize]
                                as usize] += 1;
                            i_2 += 4 as c_int;
                        }
                    } else {
                        let mut i_3: c_int = 0 as c_int;
                        while i_3 < 16 as c_int {
                            (*h).stat.frame.i_mb_pred_mode[2][(*h).mb.cache.intra4x4_pred_mode
                                [x264_scan8[i_3 as usize] as usize]
                                as usize] += 1;
                            i_3 += 1;
                        }
                    }
                    (*h).stat.frame.i_mb_pred_mode[3][x264_mb_chroma_pred_mode_fix
                        [(*h).mb.i_chroma_pred_mode as usize]
                        as usize] += 1;
                }
                (*h).stat.frame.i_mb_field[(if b_intra != 0 {
                    0 as c_int
                } else if b_skip != 0 {
                    2 as c_int
                } else {
                    1 as c_int
                }) as usize] += (*h).mb.b_interlaced;
            }
            if b_deblock != 0 {
                x264_10_macroblock_deblock_strength(h);
            }
            if mb_xy == (*h).sh.i_last_mb {
                break;
            }
            if (*h).sh.b_mbaff != 0 {
                i_mb_x += i_mb_y & 1 as c_int;
                i_mb_y ^= (i_mb_x < (*h).mb.i_mb_width) as c_int;
            } else {
                i_mb_x += 1;
            }
            if i_mb_x == (*h).mb.i_mb_width {
                i_mb_y += 1;
                i_mb_x = 0 as c_int;
            }
        }
    }
    if (*h).sh.i_last_mb < (*h).sh.i_first_mb {
        return 0 as intptr_t;
    }
    (*(*h).out.nal.offset((*h).out.i_nal as isize)).i_last_mb = (*h).sh.i_last_mb;
    if (*h).param.b_cabac != 0 {
        x264_10_cabac_encode_flush(h, &mut (*h).cabac);
        (*h).out.bs.p = (*h).cabac.p;
    } else {
        if i_skip > 0 as c_int {
            bs_write_ue_big(&mut (*h).out.bs, i_skip as c_uint);
        }
        bs_rbsp_trailing(&mut (*h).out.bs);
        bs_flush(&mut (*h).out.bs);
    }
    if nal_end(h) != 0 {
        return -1 as intptr_t;
    }
    if (*h).sh.i_last_mb == (*h).i_threadslice_end * (*h).mb.i_mb_width - 1 as c_int {
        (*h).stat.frame.i_misc_bits = bs_pos(&mut (*h).out.bs)
            + (*h).out.i_nal * NALU_OVERHEAD * 8 as c_int
            - (*h).stat.frame.i_tex_bits
            - (*h).stat.frame.i_mv_bits;
        fdec_filter_row(h, (*h).i_threadslice_end, 0 as c_int);
        if (*h).param.b_sliced_threads != 0 {
            x264_10_threadslice_cond_broadcast(h, 1 as c_int);
            let mut mb_y: c_int = (*h).i_threadslice_start;
            while mb_y <= (*h).i_threadslice_end {
                fdec_filter_row(h, mb_y, 1 as c_int);
                mb_y += 1;
            }
            x264_10_threadslice_cond_broadcast(h, 2 as c_int);
            if (*h).i_thread_idx > 0 as c_int {
                x264_10_threadslice_cond_wait(
                    (*h).thread[((*h).i_thread_idx - 1 as c_int) as usize],
                    2 as c_int,
                );
                fdec_filter_row(
                    h,
                    (*h).i_threadslice_start + ((1 as c_int) << (*h).sh.b_mbaff),
                    2 as c_int,
                );
            }
        }
        if (*(*h).fdec).mb_info_free.is_some()
            && ((*h).param.b_sliced_threads == 0
                || (*h).i_thread_idx == (*h).param.i_threads - 1 as c_int)
        {
            (*(*h).fdec)
                .mb_info_free
                .expect("non-null function pointer")(
                (*(*h).fdec).mb_info as *mut c_void
            );
            (*(*h).fdec).mb_info = 0 as *mut uint8_t;
            (*(*h).fdec).mb_info_free = None;
        }
    }
    return 0 as intptr_t;
}
#[c2rust::src_loc = "2770:9"]
const BS_BAK_SLICE_MAX_SIZE: c_int = 0 as c_int;
#[c2rust::src_loc = "2771:9"]
const BS_BAK_CAVLC_OVERFLOW: c_int = 1 as c_int;
#[c2rust::src_loc = "2772:9"]
const BS_BAK_SLICE_MIN_MBS: c_int = 2 as c_int;
#[c2rust::src_loc = "2773:9"]
const BS_BAK_ROW_VBV: c_int = 3 as c_int;
#[c2rust::src_loc = "3132:1"]
unsafe extern "C" fn thread_sync_context(mut dst: *mut x264_t, mut src: *mut x264_t) {
    if dst == src {
        return;
    }
    let mut f: *mut *mut x264_frame_t = (*src).frames.reference.as_mut_ptr();
    while !(*f).is_null() {
        (**f).i_reference_count += 1;
        f = f.offset(1);
    }
    let mut f_0: *mut *mut x264_frame_t = (*dst).frames.reference.as_mut_ptr();
    while !(*f_0).is_null() {
        x264_10_frame_push_unused(src, *f_0);
        f_0 = f_0.offset(1);
    }
    (*(*src).fdec).i_reference_count += 1;
    x264_10_frame_push_unused(src, (*dst).fdec);
    memcpy(
        &mut (*dst).i_frame as *mut c_int as *mut c_void,
        &mut (*src).i_frame as *mut c_int as *const c_void,
        (28560 as size_t).wrapping_sub(2420 as size_t),
    );
    (*dst).param = (*src).param;
    (*dst).stat = (*src).stat;
    (*dst).pixf = (*src).pixf;
    (*dst).reconfig = (*src).reconfig;
}
#[c2rust::src_loc = "3153:1"]
unsafe extern "C" fn thread_sync_stat(mut dst: *mut x264_t, mut src: *mut x264_t) {
    if dst != src {
        memcpy(
            &mut (*dst).stat as *mut C2RustUnnamed_6 as *mut c_void,
            &mut (*src).stat as *mut C2RustUnnamed_6 as *const c_void,
            (51424 as size_t).wrapping_sub(48840 as size_t),
        );
    }
}
#[c2rust::src_loc = "3159:1"]
unsafe extern "C" fn slices_write(mut h: *mut x264_t) -> *mut c_void {
    let mut current_block: u64;
    let mut i_slice_num: c_int = 0 as c_int;
    let mut last_thread_mb: c_int = (*h).sh.i_last_mb;
    let mut round_bias: c_int = if (*h).param.i_avcintra_class != 0 {
        0 as c_int
    } else {
        (*h).param.i_slice_count / 2 as c_int
    };
    memset(
        &mut (*h).stat.frame as *mut x264_frame_stat_t as *mut c_void,
        0 as c_int,
        size_of::<x264_frame_stat_t>() as size_t,
    );
    (*h).mb.b_reencode_mb = 0 as c_int;
    loop {
        if !((*h).sh.i_first_mb + (*h).sh.b_mbaff * (*h).mb.i_mb_stride <= last_thread_mb) {
            current_block = 5634871135123216486;
            break;
        }
        (*h).sh.i_last_mb = last_thread_mb;
        if i_slice_num == 0 || x264_10_frame_new_slice(h, (*h).fdec) == 0 {
            if (*h).param.i_slice_max_mbs != 0 {
                if (*h).sh.b_mbaff != 0 {
                    let mut last_mbaff: c_int = 2 as c_int
                        * ((*h).sh.i_first_mb % (*h).mb.i_mb_width)
                        + (*h).mb.i_mb_width * ((*h).sh.i_first_mb / (*h).mb.i_mb_width)
                        + (*h).param.i_slice_max_mbs
                        - 1 as c_int;
                    let mut last_x: c_int =
                        last_mbaff % (2 as c_int * (*h).mb.i_mb_width) / 2 as c_int;
                    let mut last_y: c_int =
                        last_mbaff / (2 as c_int * (*h).mb.i_mb_width) * 2 as c_int + 1 as c_int;
                    (*h).sh.i_last_mb = last_x + (*h).mb.i_mb_stride * last_y;
                } else {
                    (*h).sh.i_last_mb =
                        (*h).sh.i_first_mb + (*h).param.i_slice_max_mbs - 1 as c_int;
                    if (*h).sh.i_last_mb < last_thread_mb
                        && last_thread_mb - (*h).sh.i_last_mb < (*h).param.i_slice_min_mbs
                    {
                        (*h).sh.i_last_mb = last_thread_mb - (*h).param.i_slice_min_mbs;
                    }
                }
                i_slice_num += 1;
            } else if (*h).param.i_slice_count != 0 && (*h).param.b_sliced_threads == 0 {
                let mut height: c_int = (*h).mb.i_mb_height >> (*h).param.b_interlaced;
                let mut width: c_int = (*h).mb.i_mb_width << (*h).param.b_interlaced;
                i_slice_num += 1;
                (*h).sh.i_last_mb = (height * i_slice_num + round_bias) / (*h).param.i_slice_count
                    * width
                    - 1 as c_int;
            }
        }
        (*h).sh.i_last_mb = if (*h).sh.i_last_mb < last_thread_mb {
            (*h).sh.i_last_mb
        } else {
            last_thread_mb
        };
        if slice_write(h) != 0 {
            current_block = 15078433425792239898;
            break;
        }
        (*h).sh.i_first_mb = (*h).sh.i_last_mb + 1 as c_int;
        if (*h).sh.b_mbaff != 0 && (*h).sh.i_first_mb % (*h).mb.i_mb_width != 0 {
            (*h).sh.i_first_mb -= (*h).mb.i_mb_stride;
        }
    }
    match current_block {
        5634871135123216486 => return 0 as *mut c_void,
        _ => {
            if (*h).param.b_sliced_threads != 0 {
                x264_10_threadslice_cond_broadcast(h, 2 as c_int);
            }
            // TODO: verify safety
            return core::ptr::null_mut();
        }
    };
}
#[c2rust::src_loc = "3219:1"]
unsafe extern "C" fn threaded_slices_write(mut h: *mut x264_t) -> c_int {
    let mut round_bias: c_int = if (*h).param.i_avcintra_class != 0 {
        0 as c_int
    } else {
        (*h).param.i_slice_count / 2 as c_int
    };
    let mut i: c_int = 0 as c_int;
    while i < (*h).param.i_threads {
        let mut t: *mut x264_t = (*h).thread[i as usize];
        if i != 0 {
            (*t).param = (*h).param;
            memcpy(
                &mut (*t).i_frame as *mut c_int as *mut c_void,
                &mut (*h).i_frame as *mut c_int as *const c_void,
                (48832 as size_t).wrapping_sub(2420 as size_t),
            );
        }
        let mut height: c_int = (*h).mb.i_mb_height >> (*h).param.b_interlaced;
        (*t).i_threadslice_start =
            (height * i + round_bias) / (*h).param.i_threads << (*h).param.b_interlaced;
        (*t).i_threadslice_end = (height * (i + 1 as c_int) + round_bias) / (*h).param.i_threads
            << (*h).param.b_interlaced;
        (*t).sh.i_first_mb = (*t).i_threadslice_start * (*h).mb.i_mb_width;
        (*t).sh.i_last_mb = (*t).i_threadslice_end * (*h).mb.i_mb_width - 1 as c_int;
        i += 1;
    }
    x264_10_analyse_weight_frame(h, (*h).mb.i_mb_height * 16 as c_int + 16 as c_int);
    x264_10_threads_distribute_ratecontrol(h);
    let mut i_0: c_int = 0 as c_int;
    while i_0 < (*h).param.i_threads {
        (*(*h).thread[i_0 as usize]).i_thread_idx = i_0;
        (*(*h).thread[i_0 as usize]).b_thread_active = 1 as c_int;
        x264_10_threadslice_cond_broadcast((*h).thread[i_0 as usize], 0 as c_int);
        i_0 += 1;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < (*h).param.i_threads {
        x264_10_threadpool_run(
            (*h).threadpool,
            ::core::mem::transmute::<
                *mut c_void,
                Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
            >(::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut x264_t) -> *mut c_void>,
                *mut c_void,
            >(Some(
                slices_write as unsafe extern "C" fn(*mut x264_t) -> *mut c_void,
            ))),
            (*h).thread[i_1 as usize] as *mut c_void,
        );
        i_1 += 1;
    }
    let mut i_2: c_int = 0 as c_int;
    while i_2 < (*h).param.i_threads {
        x264_10_threadslice_cond_wait((*h).thread[i_2 as usize], 1 as c_int);
        i_2 += 1;
    }
    x264_10_threads_merge_ratecontrol(h);
    let mut i_3: c_int = 1 as c_int;
    while i_3 < (*h).param.i_threads {
        let mut t_0: *mut x264_t = (*h).thread[i_3 as usize];
        let mut j: c_int = 0 as c_int;
        while j < (*t_0).out.i_nal {
            *(*h).out.nal.offset((*h).out.i_nal as isize) = *(*t_0).out.nal.offset(j as isize);
            (*h).out.i_nal += 1;
            nal_check_buffer(h);
            j += 1;
        }
        let mut j_0: size_t = 0 as size_t;
        while j_0
            < (52112 as usize)
                .wrapping_sub(51424 as usize)
                .wrapping_div(size_of::<c_int>() as usize)
        {
            *(&mut (*h).stat.frame as *mut x264_frame_stat_t as *mut c_int).offset(j_0 as isize) +=
                *(&mut (*t_0).stat.frame as *mut x264_frame_stat_t as *mut c_int)
                    .offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: c_int = 0 as c_int;
        while j_1 < 3 as c_int {
            (*h).stat.frame.i_ssd[j_1 as usize] += (*t_0).stat.frame.i_ssd[j_1 as usize];
            j_1 += 1;
        }
        (*h).stat.frame.f_ssim += (*t_0).stat.frame.f_ssim;
        (*h).stat.frame.i_ssim_cnt += (*t_0).stat.frame.i_ssim_cnt;
        i_3 += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "3280:1"]
unsafe extern "C" fn x264_10_encoder_intra_refresh(mut h: *mut x264_t) {
    h = (*h).thread[(*h).i_thread_phase as usize];
    (*h).b_queued_intra_refresh = 1 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "3286:1"]
unsafe extern "C" fn x264_10_encoder_invalidate_reference(
    mut h: *mut x264_t,
    mut pts: int64_t,
) -> c_int {
    if (*h).param.i_bframe != 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"x264_encoder_invalidate_reference is not supported with B-frames enabled\n\0"
                as *const u8 as *const c_char,
        );
        return -1;
    }
    if (*h).param.b_intra_refresh != 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"x264_encoder_invalidate_reference is not supported with intra refresh enabled\n\0"
                as *const u8 as *const c_char,
        );
        return -1;
    }
    h = (*h).thread[(*h).i_thread_phase as usize];
    if pts >= (*h).i_last_idr_pts {
        let mut i: c_int = 0 as c_int;
        while !(*h).frames.reference[i as usize].is_null() {
            if pts <= (*(*h).frames.reference[i as usize]).i_pts {
                (*(*h).frames.reference[i as usize]).b_corrupt = 1 as c_int;
            }
            i += 1;
        }
        if pts <= (*(*h).fdec).i_pts {
            (*(*h).fdec).b_corrupt = 1 as c_int;
        }
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "3323:1"]
unsafe extern "C" fn x264_10_encoder_encode(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut c_int,
    mut pic_in: *mut x264_picture_t,
    mut pic_out: *mut x264_picture_t,
) -> c_int {
    let mut thread_current: *mut x264_t = 0 as *mut x264_t;
    let mut thread_prev: *mut x264_t = 0 as *mut x264_t;
    let mut thread_oldest: *mut x264_t = 0 as *mut x264_t;
    let mut i_nal_type: c_int = 0;
    let mut i_nal_ref_idc: c_int = 0;
    let mut i_global_qp: c_int = 0;
    let mut overhead: c_int = NALU_OVERHEAD;
    if (*h).i_thread_frames > 1 as c_int {
        thread_prev = (*h).thread[(*h).i_thread_phase as usize];
        (*h).i_thread_phase = ((*h).i_thread_phase + 1 as c_int) % (*h).i_thread_frames;
        thread_current = (*h).thread[(*h).i_thread_phase as usize];
        thread_oldest =
            (*h).thread[(((*h).i_thread_phase + 1 as c_int) % (*h).i_thread_frames) as usize];
        thread_sync_context(thread_current, thread_prev);
        x264_10_thread_sync_ratecontrol(thread_current, thread_prev, thread_oldest);
        h = thread_current;
    } else {
        thread_oldest = h;
        thread_current = thread_oldest;
    }
    (*h).i_cpb_delay_pir_offset = (*h).i_cpb_delay_pir_offset_next;
    *pi_nal = 0 as c_int;
    *pp_nal = 0 as *mut x264_nal_t;
    if !pic_in.is_null() {
        if (*(*h).lookahead).b_exit_thread != 0 {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"lookahead thread is already stopped\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        let mut fenc: *mut x264_frame_t = x264_10_frame_pop_unused(h, 0 as c_int);
        if fenc.is_null() {
            return -1;
        }
        if x264_10_frame_copy_picture(h, fenc, pic_in) < 0 as c_int {
            return -1;
        }
        if (*h).param.width as c_int != 16 * (*h).mb.i_mb_width
            || (*h).param.height as c_int != 16 * (*h).mb.i_mb_height
        {
            x264_10_frame_expand_border_mod16(h, fenc);
        }
        let fresh8 = (*h).frames.i_input;
        (*h).frames.i_input = (*h).frames.i_input + 1;
        (*fenc).i_frame = fresh8;
        if (*fenc).i_frame == 0 as c_int {
            (*h).frames.i_first_pts = (*fenc).i_pts;
        }
        if (*h).frames.i_bframe_delay != 0 && (*fenc).i_frame == (*h).frames.i_bframe_delay {
            (*h).frames.i_bframe_delay_time = (*fenc).i_pts - (*h).frames.i_first_pts;
        }
        if (*h).param.b_vfr_input != 0 && (*fenc).i_pts <= (*h).frames.i_largest_pts {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"non-strictly-monotonic PTS\n\0" as *const u8 as *const c_char,
            );
        }
        (*h).frames.i_second_largest_pts = (*h).frames.i_largest_pts;
        (*h).frames.i_largest_pts = (*fenc).i_pts;
        if (*fenc).i_pic_struct < PIC_STRUCT_AUTO as c_int
            || (*fenc).i_pic_struct > PIC_STRUCT_TRIPLE as c_int
        {
            (*fenc).i_pic_struct = PIC_STRUCT_AUTO as c_int;
        }
        if (*fenc).i_pic_struct == PIC_STRUCT_AUTO as c_int {
            let mut b_interlaced: c_int = if !(*fenc).param.is_null() {
                (*(*fenc).param).b_interlaced
            } else {
                (*h).param.b_interlaced
            };
            if b_interlaced != 0 {
                let mut b_tff: c_int = if !(*fenc).param.is_null() {
                    (*(*fenc).param).b_tff
                } else {
                    (*h).param.b_tff
                };
                (*fenc).i_pic_struct = if b_tff != 0 {
                    PIC_STRUCT_TOP_BOTTOM as c_int
                } else {
                    PIC_STRUCT_BOTTOM_TOP as c_int
                };
            } else {
                (*fenc).i_pic_struct = PIC_STRUCT_PROGRESSIVE as c_int;
            }
        }
        if (*h).param.rc.b_mb_tree != 0 && (*h).param.rc.b_stat_read != 0 {
            if x264_10_macroblock_tree_read(h, fenc, (*pic_in).prop.quant_offsets) != 0 {
                return -1;
            }
        } else {
            x264_10_adaptive_quant_frame(h, fenc, (*pic_in).prop.quant_offsets);
        }
        if (*pic_in).prop.quant_offsets_free.is_some() {
            (*pic_in)
                .prop
                .quant_offsets_free
                .expect("non-null function pointer")(
                (*pic_in).prop.quant_offsets as *mut c_void
            );
        }
        if (*h).frames.b_have_lowres != 0 {
            x264_10_frame_init_lowres(h, fenc);
        }
        x264_10_lookahead_put_frame(h, fenc);
        if (*h).frames.i_input <= (*h).frames.i_delay + 1 as c_int - (*h).i_thread_frames {
            (*pic_out).i_type = X264_TYPE_AUTO;
            return 0 as c_int;
        }
    } else {
        pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
        ::core::ptr::write_volatile(
            &mut (*(*h).lookahead).b_exit_thread as *mut uint8_t,
            1 as uint8_t,
        );
        pthread_cond_broadcast(&mut (*(*h).lookahead).ifbuf.cv_fill);
        pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
    }
    (*h).i_frame += 1;
    if (*(*h).frames.current.offset(0)).is_null() {
        x264_10_lookahead_get_frames(h);
    }
    if (*(*h).frames.current.offset(0)).is_null() && x264_10_lookahead_is_empty(h) != 0 {
        return encoder_frame_end(thread_oldest, thread_current, pp_nal, pi_nal, pic_out);
    }
    (*h).fenc = x264_10_frame_shift((*h).frames.current);
    if (*h).param.b_sliced_threads != 0 {
        if threadpool_wait_all(h) < 0 as c_int {
            return -1;
        }
    }
    if (*h).i_frame == 0 as c_int {
        (*h).i_reordered_pts_delay = (*(*h).fenc).i_reordered_pts;
    }
    if (*h).reconfig != 0 {
        x264_10_encoder_reconfig_apply(h, &mut (*(*h).reconfig_h).param);
        (*h).reconfig = 0 as c_int;
    }
    if !(*(*h).fenc).param.is_null() {
        x264_10_encoder_reconfig_apply(h, (*(*h).fenc).param);
        if (*(*(*h).fenc).param).param_free.is_some() {
            x264_param_cleanup((*(*h).fenc).param);
            (*(*(*h).fenc).param)
                .param_free
                .expect("non-null function pointer")((*(*h).fenc).param as *mut c_void);
            (*(*h).fenc).param = 0 as *mut x264_param_t;
        }
    }
    x264_10_ratecontrol_zone_init(h);
    if reference_update(h) != 0 {
        return -1;
    }
    (*(*h).fdec).i_lines_completed = -1;
    if !((*(*h).fenc).i_type == X264_TYPE_I
        || (*(*h).fenc).i_type == X264_TYPE_IDR
        || (*(*h).fenc).i_type == X264_TYPE_KEYFRAME)
    {
        let mut valid_refs_left: c_int = 0 as c_int;
        let mut i: c_int = 0 as c_int;
        while !(*h).frames.reference[i as usize].is_null() {
            if (*(*h).frames.reference[i as usize]).b_corrupt == 0 {
                valid_refs_left += 1;
            }
            i += 1;
        }
        if valid_refs_left == 0 {
            (*(*h).fenc).b_keyframe = 1 as c_int;
            (*(*h).fenc).i_type = X264_TYPE_IDR;
        }
    }
    if (*(*h).fenc).b_keyframe != 0 {
        (*h).frames.i_last_keyframe = (*(*h).fenc).i_frame;
        if (*(*h).fenc).i_type == X264_TYPE_IDR {
            (*h).i_frame_num = 0 as c_int;
            (*h).frames.i_last_idr = (*(*h).fenc).i_frame;
        }
    }
    (*h).sh.i_mmco_remove_from_end = 0 as c_int;
    (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_remove_from_end;
    (*h).b_ref_reorder[1] = 0 as c_int;
    (*h).b_ref_reorder[0] = (*h).b_ref_reorder[1];
    (*(*h).fenc).i_poc = 2 as c_int
        * ((*(*h).fenc).i_frame
            - (if (*h).frames.i_last_idr > 0 as c_int {
                (*h).frames.i_last_idr
            } else {
                0 as c_int
            }));
    (*(*h).fdec).i_poc = (*(*h).fenc).i_poc;
    if (*(*h).fenc).i_type == X264_TYPE_IDR {
        i_nal_type = NAL_SLICE_IDR as c_int;
        i_nal_ref_idc = NAL_PRIORITY_HIGHEST as c_int;
        (*h).sh.i_type = SLICE_TYPE_I as c_int;
        reference_reset(h);
        (*h).frames.i_poc_last_open_gop = -1;
    } else if (*(*h).fenc).i_type == X264_TYPE_I {
        i_nal_type = NAL_SLICE as c_int;
        i_nal_ref_idc = NAL_PRIORITY_HIGH as c_int;
        (*h).sh.i_type = SLICE_TYPE_I as c_int;
        reference_hierarchy_reset(h);
        if (*h).param.b_open_gop != 0 {
            (*h).frames.i_poc_last_open_gop = if (*(*h).fenc).b_keyframe != 0 {
                (*(*h).fenc).i_poc
            } else {
                -1
            };
        }
    } else if (*(*h).fenc).i_type == X264_TYPE_P {
        i_nal_type = NAL_SLICE as c_int;
        i_nal_ref_idc = NAL_PRIORITY_HIGH as c_int;
        (*h).sh.i_type = SLICE_TYPE_P as c_int;
        reference_hierarchy_reset(h);
        (*h).frames.i_poc_last_open_gop = -1;
    } else if (*(*h).fenc).i_type == X264_TYPE_BREF {
        i_nal_type = NAL_SLICE as c_int;
        i_nal_ref_idc = if (*h).param.bframe_pyramid == BPyramid::Strict {
            NAL_PRIORITY_LOW as c_int
        } else {
            NAL_PRIORITY_HIGH as c_int
        };
        (*h).sh.i_type = SLICE_TYPE_B as c_int;
        reference_hierarchy_reset(h);
    } else {
        i_nal_type = NAL_SLICE as c_int;
        i_nal_ref_idc = NAL_PRIORITY_DISPOSABLE as c_int;
        (*h).sh.i_type = SLICE_TYPE_B as c_int;
    }
    (*(*h).fdec).i_type = (*(*h).fenc).i_type;
    (*(*h).fdec).i_frame = (*(*h).fenc).i_frame;
    (*(*h).fdec).b_kept_as_ref = (i_nal_ref_idc != NAL_PRIORITY_DISPOSABLE as c_int
        && (*h).param.i_keyint_max > 1 as c_int) as c_int;
    (*(*h).fenc).b_kept_as_ref = (*(*h).fdec).b_kept_as_ref;
    (*(*h).fdec).mb_info = (*(*h).fenc).mb_info;
    (*(*h).fdec).mb_info_free = (*(*h).fenc).mb_info_free;
    (*(*h).fenc).mb_info = 0 as *mut uint8_t;
    (*(*h).fenc).mb_info_free = None;
    (*(*h).fdec).i_pts = (*(*h).fenc).i_pts;
    if (*h).frames.i_bframe_delay != 0 {
        let mut prev_reordered_pts: *mut int64_t =
            (*thread_current).frames.i_prev_reordered_pts.as_mut_ptr();
        (*(*h).fdec).i_dts = if (*h).i_frame > (*h).frames.i_bframe_delay {
            *prev_reordered_pts.offset(
                (((*h).i_frame - (*h).frames.i_bframe_delay) % (*h).frames.i_bframe_delay) as isize,
            )
        } else {
            (*(*h).fenc).i_reordered_pts - (*h).frames.i_bframe_delay_time
        };
        *prev_reordered_pts.offset(((*h).i_frame % (*h).frames.i_bframe_delay) as isize) =
            (*(*h).fenc).i_reordered_pts;
    } else {
        (*(*h).fdec).i_dts = (*(*h).fenc).i_reordered_pts;
    }
    if (*(*h).fenc).i_type == X264_TYPE_IDR {
        (*h).i_last_idr_pts = (*(*h).fdec).i_pts;
    }
    reference_build_list(h, (*(*h).fdec).i_poc);
    if (*h).param.b_sliced_threads != 0 {
        let mut i_0: c_int = 0 as c_int;
        while i_0 < (*h).param.i_threads {
            bs_init(
                &mut (**(*h).thread.as_mut_ptr().offset(i_0 as isize)).out.bs,
                (*(*h).thread[i_0 as usize]).out.p_bitstream as *mut c_void,
                (*(*h).thread[i_0 as usize]).out.i_bitstream,
            );
            (*(*h).thread[i_0 as usize]).out.i_nal = 0 as c_int;
            i_0 += 1;
        }
    } else {
        bs_init(
            &mut (*h).out.bs,
            (*h).out.p_bitstream as *mut c_void,
            (*h).out.i_bitstream,
        );
        (*h).out.i_nal = 0 as c_int;
    }
    if (*h).param.b_aud != 0 {
        let mut pic_type: c_int = 0;
        if (*h).sh.i_type == SLICE_TYPE_I as c_int {
            pic_type = 0 as c_int;
        } else if (*h).sh.i_type == SLICE_TYPE_P as c_int {
            pic_type = 1 as c_int;
        } else if (*h).sh.i_type == SLICE_TYPE_B as c_int {
            pic_type = 2 as c_int;
        } else {
            pic_type = 7 as c_int;
        }
        nal_start(h, NAL_AUD as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
        bs_write(&mut (*h).out.bs, 3 as c_int, pic_type as uint32_t);
        bs_rbsp_trailing(&mut (*h).out.bs);
        bs_flush(&mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -1;
        }
        overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
            + NALU_OVERHEAD;
    }
    (*h).i_nal_type = i_nal_type;
    (*h).i_nal_ref_idc = i_nal_ref_idc;
    if (*h).param.b_intra_refresh != 0 {
        if (*(*h).fenc).i_type == X264_TYPE_I
            || (*(*h).fenc).i_type == X264_TYPE_IDR
            || (*(*h).fenc).i_type == X264_TYPE_KEYFRAME
        {
            (*(*h).fdec).i_frames_since_pir = 0 as c_int;
            (*h).b_queued_intra_refresh = 0 as c_int;
            (*(*h).fdec).f_pir_position = (*h).mb.i_mb_width as c_float;
        } else if (*(*h).fenc).i_type == X264_TYPE_P {
            let mut pocdiff: c_int = ((*(*h).fdec).i_poc - (*(*h).fref[0][0]).i_poc) / 2 as c_int;
            let mut increment: c_float = if ((*h).mb.i_mb_width as c_float - 1 as c_int as c_float)
                / (*h).param.i_keyint_max as c_float
                > 1 as c_int as c_float
            {
                ((*h).mb.i_mb_width as c_float - 1 as c_int as c_float)
                    / (*h).param.i_keyint_max as c_float
            } else {
                1 as c_int as c_float
            };
            (*(*h).fdec).f_pir_position = (*(*h).fref[0][0]).f_pir_position;
            (*(*h).fdec).i_frames_since_pir = (*(*h).fref[0][0]).i_frames_since_pir + pocdiff;
            if (*(*h).fdec).i_frames_since_pir >= (*h).param.i_keyint_max
                || (*h).b_queued_intra_refresh != 0
                    && (*(*h).fdec).f_pir_position as c_double + 0.5f64
                        >= (*h).mb.i_mb_width as c_double
            {
                (*(*h).fdec).f_pir_position = 0 as c_int as c_float;
                (*(*h).fdec).i_frames_since_pir = 0 as c_int;
                (*h).b_queued_intra_refresh = 0 as c_int;
                (*(*h).fenc).b_keyframe = 1 as c_int;
            }
            (*(*h).fdec).i_pir_start_col =
                ((*(*h).fdec).f_pir_position as c_double + 0.5f64) as c_int;
            (*(*h).fdec).f_pir_position += increment * pocdiff as c_float;
            (*(*h).fdec).i_pir_end_col =
                ((*(*h).fdec).f_pir_position as c_double + 0.5f64) as c_int;
            if (*(*h).fdec).i_pir_end_col >= (*h).mb.i_mb_width - 1 as c_int {
                (*(*h).fdec).f_pir_position = (*h).mb.i_mb_width as c_float;
                (*(*h).fdec).i_pir_end_col = (*h).mb.i_mb_width - 1 as c_int;
            }
        }
    }
    if (*(*h).fenc).b_keyframe != 0 {
        if (*h).param.b_repeat_headers != 0 {
            nal_start(h, NAL_SPS as c_int, NAL_PRIORITY_HIGHEST as c_int);
            x264_10_sps_write(&mut (*h).out.bs, (*h).sps.as_mut_ptr());
            if nal_end(h) != 0 {
                return -1;
            }
            if (*h).param.i_avcintra_class != 0 {
                (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_padding = 256
                    as c_int
                    - bs_pos(&mut (*h).out.bs) / 8 as c_int
                    - 2 as c_int * NALU_OVERHEAD;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_padding
                + NALU_OVERHEAD;
            nal_start(h, NAL_PPS as c_int, NAL_PRIORITY_HIGHEST as c_int);
            x264_10_pps_write(
                &mut (*h).out.bs,
                (*h).sps.as_mut_ptr(),
                (*h).pps.as_mut_ptr(),
            );
            if nal_end(h) != 0 {
                return -1;
            }
            if (*h).param.i_avcintra_class != 0 {
                let mut total_len: c_int = 256 as c_int;
                if (*h).param.i_avcintra_flavor == X264_AVCINTRA_FLAVOR_SONY {
                    total_len += if (*h).param.height >= 1080 {
                        18 * 512
                    } else {
                        10 * 512
                    };
                }
                (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_padding = total_len
                    - (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                    - NALU_OVERHEAD;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_padding
                + NALU_OVERHEAD;
        }
        if (*h).i_thread_frames == 1 as c_int
            && (*(*h).sps.as_mut_ptr()).vui.b_nal_hrd_parameters_present != 0
        {
            x264_10_hrd_fullness(h);
            nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
            x264_10_sei_buffering_period_write(h, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -1;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        }
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < (*(*h).fenc).extra_sei.num_payloads {
        nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
        x264_10_sei_write(
            &mut (*h).out.bs,
            (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload,
            (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload_size,
            (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload_type,
        );
        if nal_end(h) != 0 {
            return -1;
        }
        overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
            + (NALU_OVERHEAD
                - ((*h).param.b_annexb != 0
                    && (*h).param.i_avcintra_class == 0
                    && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        if (*(*h).fenc).extra_sei.sei_free.is_some() {
            (*(*h).fenc)
                .extra_sei
                .sei_free
                .expect("non-null function pointer")(
                (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload as *mut c_void,
            );
            let ref mut fresh9 = (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload;
            *fresh9 = 0 as *mut uint8_t;
        }
        i_1 += 1;
    }
    if (*(*h).fenc).extra_sei.sei_free.is_some() {
        (*(*h).fenc)
            .extra_sei
            .sei_free
            .expect("non-null function pointer")(
            (*(*h).fenc).extra_sei.payloads as *mut c_void
        );
        (*(*h).fenc).extra_sei.payloads = 0 as *mut x264_sei_payload_t;
        (*(*h).fenc).extra_sei.sei_free = None;
    }
    if (*(*h).fenc).b_keyframe != 0 {
        if (*h).param.b_repeat_headers != 0
            && (*(*h).fenc).i_frame == 0 as c_int
            && (*h).param.i_avcintra_class == 0
        {
            nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
            if x264_10_sei_version_write(h, &mut (*h).out.bs) != 0 {
                return -1;
            }
            if nal_end(h) != 0 {
                return -1;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        }
        if (*(*h).fenc).i_type != X264_TYPE_IDR {
            let mut time_to_recovery: c_int = if (*h).param.b_open_gop != 0 {
                0 as c_int
            } else {
                (if ((*h).mb.i_mb_width - 1 as c_int) < (*h).param.i_keyint_max {
                    (*h).mb.i_mb_width - 1 as c_int
                } else {
                    (*h).param.i_keyint_max
                }) + (*h).param.i_bframe
                    - 1 as c_int
            };
            nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
            x264_10_sei_recovery_point_write(h, &mut (*h).out.bs, time_to_recovery);
            if nal_end(h) != 0 {
                return -1;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        }
        if let Some(mastering_display) = (*h).param.mastering_display {
            nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
            x264_sei_mastering_display_write(&mastering_display, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -1;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        }
        if let Some(light_level) = (*h).param.content_light_level {
            nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
            x264_sei_content_light_level_write(&light_level, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -1;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        }
        if (*h).param.i_alternative_transfer != 2 as c_int {
            nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
            x264_10_sei_alternative_transfer_write(h, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -1;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        }
    }
    if let Some(frame_packing) = (*h).param.frame_packing {
        if (*(*h).fenc).b_keyframe != 0 || frame_packing == FramePacking::TemporalInterleaved {
            nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
            x264_sei_frame_packing_write(frame_packing, (*(*h).fenc).i_frame, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -1;
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        }
    }
    if (*(*h).sps.as_mut_ptr()).vui.b_pic_struct_present != 0
        || (*(*h).sps.as_mut_ptr()).vui.b_nal_hrd_parameters_present != 0
    {
        nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
        x264_10_sei_pic_timing_write(h, &mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -1;
        }
        overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
            + (NALU_OVERHEAD
                - ((*h).param.b_annexb != 0
                    && (*h).param.i_avcintra_class == 0
                    && (*h).out.i_nal - 1 as c_int != 0) as c_int);
    }
    if !((*(*h).fenc).i_type == X264_TYPE_B || (*(*h).fenc).i_type == X264_TYPE_BREF)
        && (*h).b_sh_backup != 0
    {
        (*h).b_sh_backup = 0 as c_int;
        nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
        x264_10_sei_dec_ref_pic_marking_write(h, &mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -1;
        }
        overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
            + (NALU_OVERHEAD
                - ((*h).param.b_annexb != 0
                    && (*h).param.i_avcintra_class == 0
                    && (*h).out.i_nal - 1 as c_int != 0) as c_int);
    }
    if (*(*h).fenc).b_keyframe != 0 && (*h).param.b_intra_refresh != 0 {
        (*h).i_cpb_delay_pir_offset_next = (*(*h).fenc).i_cpb_delay;
    }
    if (*h).param.i_avcintra_class != 0 && (*h).param.i_avcintra_flavor != X264_AVCINTRA_FLAVOR_SONY
    {
        nal_start(h, NAL_FILLER as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
        x264_10_filler_write(h, &mut (*h).out.bs, 0 as c_int);
        if nal_end(h) != 0 {
            return -1;
        }
        overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
            + NALU_OVERHEAD;
        nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
        if x264_10_sei_avcintra_umid_write(h, &mut (*h).out.bs) < 0 as c_int {
            return -1;
        }
        if nal_end(h) != 0 {
            return -1;
        }
        overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
            + (NALU_OVERHEAD
                - ((*h).param.b_annexb != 0
                    && (*h).param.i_avcintra_class == 0
                    && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        let mut unpadded_len: c_int = 0;
        let mut total_len_0: c_int = 0;
        if (*h).param.height == 1080 {
            unpadded_len = 5780 as c_int;
            total_len_0 = 17 as c_int * 512 as c_int;
        } else {
            unpadded_len = 2900 as c_int;
            total_len_0 = 9 as c_int * 512 as c_int;
        }
        nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
        if x264_10_sei_avcintra_vanc_write(h, &mut (*h).out.bs, unpadded_len) < 0 as c_int {
            return -1;
        }
        if nal_end(h) != 0 {
            return -1;
        }
        (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_padding = total_len_0
            - (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
            - (NALU_OVERHEAD
                - ((*h).param.b_annexb != 0
                    && (*h).param.i_avcintra_class == 0
                    && (*h).out.i_nal - 1 as c_int != 0) as c_int);
        overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_payload
            + (*(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize)).i_padding
            + (NALU_OVERHEAD
                - ((*h).param.b_annexb != 0
                    && (*h).param.i_avcintra_class == 0
                    && (*h).out.i_nal - 1 as c_int != 0) as c_int);
    }
    x264_10_ratecontrol_start(h, (*(*h).fenc).i_qpplus1, overhead * 8 as c_int);
    i_global_qp = x264_10_ratecontrol_qp(h);
    (*(*h).fdec).i_qpplus1 = i_global_qp + 1 as c_int;
    (*pic_out).i_qpplus1 = (*(*h).fdec).i_qpplus1;
    if (*h).param.rc.b_stat_read != 0 && (*h).sh.i_type != SLICE_TYPE_I as c_int {
        x264_10_reference_build_list_optimal(h);
        reference_check_reorder(h);
    }
    if (*h).i_ref[0] != 0 {
        (*(*h).fdec).i_poc_l0ref0 = (*(*h).fref[0][0]).i_poc;
    }
    slice_init(h, i_nal_type, i_global_qp);
    if (*h).sh.i_type == SLICE_TYPE_B as c_int {
        x264_10_macroblock_bipred_init(h);
    }
    weighted_pred_init(h);
    if i_nal_ref_idc != NAL_PRIORITY_DISPOSABLE as c_int {
        (*h).i_frame_num += 1;
    }
    (*h).i_threadslice_start = 0 as c_int;
    (*h).i_threadslice_end = (*h).mb.i_mb_height;
    if (*h).i_thread_frames > 1 as c_int {
        x264_10_threadpool_run(
            (*h).threadpool,
            ::core::mem::transmute::<
                *mut c_void,
                Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
            >(::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut x264_t) -> *mut c_void>,
                *mut c_void,
            >(Some(
                slices_write as unsafe extern "C" fn(*mut x264_t) -> *mut c_void,
            ))),
            h as *mut c_void,
        );
        (*h).b_thread_active = 1 as c_int;
    } else if (*h).param.b_sliced_threads != 0 {
        if threaded_slices_write(h) != 0 {
            return -1;
        }
    } else if slices_write(h) as intptr_t != 0 {
        return -1;
    }
    return encoder_frame_end(thread_oldest, thread_current, pp_nal, pi_nal, pic_out);
}
#[c2rust::src_loc = "3904:1"]
unsafe extern "C" fn encoder_frame_end(
    mut h: *mut x264_t,
    mut thread_current: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut c_int,
    mut pic_out: *mut x264_picture_t,
) -> c_int {
    let mut psz_message: [c_char; 80] = [0; 80];
    if (*h).param.b_sliced_threads == 0 && (*h).b_thread_active != 0 {
        (*h).b_thread_active = 0 as c_int;
        if x264_10_threadpool_wait((*h).threadpool, h as *mut c_void) as intptr_t != 0 {
            return -1;
        }
    }
    if (*h).out.i_nal == 0 {
        (*pic_out).i_type = X264_TYPE_AUTO;
        return 0 as c_int;
    }
    if (*h).i_thread_frames > 1 as c_int
        && (*(*h).fenc).b_keyframe != 0
        && (*(*h).sps.as_mut_ptr()).vui.b_nal_hrd_parameters_present != 0
    {
        x264_10_hrd_fullness(h);
        nal_start(h, NAL_SEI as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
        x264_10_sei_buffering_period_write(h, &mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -1;
        }
        let mut idx: c_int = 0 as c_int;
        while (*(*h).out.nal.offset(idx as isize)).i_type == NAL_AUD as c_int
            || (*(*h).out.nal.offset(idx as isize)).i_type == NAL_SPS as c_int
            || (*(*h).out.nal.offset(idx as isize)).i_type == NAL_PPS as c_int
        {
            idx += 1;
        }
        let mut nal_tmp: x264_nal_t = *(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize);
        memmove(
            &mut *(*h).out.nal.offset((idx + 1 as c_int) as isize) as *mut x264_nal_t
                as *mut c_void,
            &mut *(*h).out.nal.offset(idx as isize) as *mut x264_nal_t as *const c_void,
            (((*h).out.i_nal - idx - 1 as c_int) as size_t)
                .wrapping_mul(size_of::<x264_nal_t>() as size_t),
        );
        *(*h).out.nal.offset(idx as isize) = nal_tmp;
    }
    let mut frame_size: c_int = encoder_encapsulate_nals(h, 0 as c_int);
    if frame_size < 0 as c_int {
        return -1;
    }
    (*pic_out).i_type = (*(*h).fenc).i_type;
    (*pic_out).b_keyframe = (*(*h).fenc).b_keyframe;
    (*pic_out).i_pic_struct = (*(*h).fenc).i_pic_struct;
    (*pic_out).i_pts = (*(*h).fdec).i_pts;
    (*pic_out).i_dts = (*(*h).fdec).i_dts;
    if (*pic_out).i_pts < (*pic_out).i_dts {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"invalid DTS: PTS is less than DTS\n\0" as *const u8 as *const c_char,
        );
    }
    (*pic_out).opaque = (*(*h).fenc).opaque;
    (*pic_out).img.i_csp = (*(*h).fdec).i_csp;
    (*pic_out).img.i_csp |= X264_CSP_HIGH_DEPTH;
    (*pic_out).img.i_plane = (*(*h).fdec).i_plane;
    let mut i: c_int = 0 as c_int;
    while i < (*pic_out).img.i_plane {
        (*pic_out).img.i_stride[i as usize] = (*(*h).fdec).i_stride[i as usize] * SIZEOF_PIXEL;
        (*pic_out).img.plane[i as usize] = (*(*h).fdec).plane[i as usize] as *mut uint8_t;
        i += 1;
    }
    x264_10_frame_push_unused(thread_current, (*h).fenc);
    let mut filler: c_int = 0 as c_int;
    if x264_10_ratecontrol_end(h, frame_size * 8 as c_int, &mut filler) < 0 as c_int {
        return -1;
    }
    (*pic_out).hrd_timing = (*(*h).fenc).hrd_timing;
    (*pic_out).prop.f_crf_avg = (*(*h).fdec).f_crf_avg as c_double;
    if (*h).param.i_avcintra_class != 0 {
        if check_encapsulated_buffer(
            h,
            (*h).thread[0],
            (*h).out.i_nal,
            frame_size as int64_t,
            frame_size as int64_t + filler as int64_t,
        ) < 0 as c_int
        {
            return -1;
        }
        let mut nal: *mut x264_nal_t =
            &mut *(*h).out.nal.offset(((*h).out.i_nal - 1 as c_int) as isize) as *mut x264_nal_t;
        memset(
            (*nal).p_payload.offset((*nal).i_payload as isize) as *mut c_void,
            0 as c_int,
            filler as size_t,
        );
        (*nal).i_payload += filler;
        (*nal).i_padding = filler;
        frame_size += filler;
        if (*h).param.b_annexb == 0 {
            let mut nal_data: *mut uint8_t = (*nal).p_payload;
            let mut chunk_size: c_int = (*nal).i_payload - 4 as c_int;
            *nal_data.offset(0) = (chunk_size >> 24 as c_int) as uint8_t;
            *nal_data.offset(1) = (chunk_size >> 16 as c_int) as uint8_t;
            *nal_data.offset(2) = (chunk_size >> 8 as c_int) as uint8_t;
            *nal_data.offset(3) = (chunk_size >> 0 as c_int) as uint8_t;
        }
    } else {
        while filler > 0 as c_int {
            let mut f: c_int = 0;
            let mut overhead: c_int = FILLER_OVERHEAD - (*h).param.b_annexb;
            if (*h).param.i_slice_max_size != 0 && filler > (*h).param.i_slice_max_size {
                let mut next_size: c_int = filler - (*h).param.i_slice_max_size;
                let mut overflow: c_int = if overhead - next_size > 0 as c_int {
                    overhead - next_size
                } else {
                    0 as c_int
                };
                f = (*h).param.i_slice_max_size - overhead - overflow;
            } else {
                f = if 0 as c_int > filler - overhead {
                    0 as c_int
                } else {
                    filler - overhead
                };
            }
            if bitstream_check_buffer_filler(h, f) != 0 {
                return -1;
            }
            nal_start(h, NAL_FILLER as c_int, NAL_PRIORITY_DISPOSABLE as c_int);
            x264_10_filler_write(h, &mut (*h).out.bs, f);
            if nal_end(h) != 0 {
                return -1;
            }
            let mut total_size: c_int = encoder_encapsulate_nals(h, (*h).out.i_nal - 1 as c_int);
            if total_size < 0 as c_int {
                return -1;
            }
            frame_size += total_size;
            filler -= total_size;
        }
    }
    *pi_nal = (*h).out.i_nal;
    *pp_nal = (*h).out.nal;
    (*h).out.i_nal = 0 as c_int;
    x264_10_noise_reduction_update(h);
    thread_sync_stat(h, (*h).thread[0]);
    (*h).stat.i_frame_count[(*h).sh.i_type as usize] += 1;
    (*h).stat.i_frame_size[(*h).sh.i_type as usize] += frame_size as int64_t;
    (*h).stat.f_frame_qp[(*h).sh.i_type as usize] += (*(*h).fdec).f_qp_avg_aq as c_double;
    let mut i_0: c_int = 0 as c_int;
    while i_0 < X264_MBTYPE_MAX as c_int {
        (*h).stat.i_mb_count[(*h).sh.i_type as usize][i_0 as usize] +=
            (*h).stat.frame.i_mb_count[i_0 as usize] as int64_t;
        i_0 += 1;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < 2 as c_int {
        (*h).stat.i_mb_count_8x8dct[i_1 as usize] +=
            (*h).stat.frame.i_mb_count_8x8dct[i_1 as usize] as int64_t;
        i_1 += 1;
    }
    let mut i_2: c_int = 0 as c_int;
    while i_2 < 6 as c_int {
        (*h).stat.i_mb_cbp[i_2 as usize] += (*h).stat.frame.i_mb_cbp[i_2 as usize] as int64_t;
        i_2 += 1;
    }
    let mut i_3: c_int = 0 as c_int;
    while i_3 < 4 as c_int {
        let mut j: c_int = 0 as c_int;
        while j < 13 as c_int {
            (*h).stat.i_mb_pred_mode[i_3 as usize][j as usize] +=
                (*h).stat.frame.i_mb_pred_mode[i_3 as usize][j as usize] as int64_t;
            j += 1;
        }
        i_3 += 1;
    }
    if (*h).sh.i_type != SLICE_TYPE_I as c_int {
        let mut i_4: c_int = 0 as c_int;
        while i_4 < X264_PARTTYPE_MAX as c_int {
            (*h).stat.i_mb_partition[(*h).sh.i_type as usize][i_4 as usize] +=
                (*h).stat.frame.i_mb_partition[i_4 as usize] as int64_t;
            i_4 += 1;
        }
        let mut i_list: c_int = 0 as c_int;
        while i_list < 2 as c_int {
            let mut i_5: c_int = 0 as c_int;
            while i_5 < X264_REF_MAX * 2 as c_int {
                (*h).stat.i_mb_count_ref[(*h).sh.i_type as usize][i_list as usize][i_5 as usize] +=
                    (*h).stat.frame.i_mb_count_ref[i_list as usize][i_5 as usize] as int64_t;
                i_5 += 1;
            }
            i_list += 1;
        }
    }
    let mut i_6: c_int = 0 as c_int;
    while i_6 < 3 as c_int {
        (*h).stat.i_mb_field[i_6 as usize] += (*h).stat.frame.i_mb_field[i_6 as usize] as int64_t;
        i_6 += 1;
    }
    if (*h).sh.i_type == SLICE_TYPE_P as c_int
        && (*h).param.analyse.i_weighted_pred >= X264_WEIGHTP_SIMPLE
    {
        (*h).stat.i_wpred[0] += !(*h).sh.weight[0][0].weightfn.is_null() as c_int;
        (*h).stat.i_wpred[1] += (!(*h).sh.weight[0][1].weightfn.is_null()
            || !(*h).sh.weight[0][2].weightfn.is_null()) as c_int;
    }
    if (*h).sh.i_type == SLICE_TYPE_B as c_int {
        (*h).stat.i_direct_frames[(*h).sh.b_direct_spatial_mv_pred as usize] += 1;
        if (*h).mb.b_direct_auto_write != 0 {
            if (*h).stat.i_direct_score[0] + (*h).stat.i_direct_score[1] > (*h).mb.i_mb_count {
                let mut i_7: c_int = 0 as c_int;
                while i_7 < 2 as c_int {
                    (*h).stat.i_direct_score[i_7 as usize] =
                        (*h).stat.i_direct_score[i_7 as usize] * 9 as c_int / 10 as c_int;
                    i_7 += 1;
                }
            }
            let mut i_8: c_int = 0 as c_int;
            while i_8 < 2 as c_int {
                (*h).stat.i_direct_score[i_8 as usize] +=
                    (*h).stat.frame.i_direct_score[i_8 as usize];
                i_8 += 1;
            }
        }
    } else {
        (*h).stat.i_consecutive_bframes[(*(*h).fenc).i_bframes as usize] += 1;
    }
    psz_message[0] = '\0' as i32 as c_char;
    let mut dur: c_double = (*(*h).fenc).f_duration as c_double;
    (*h).stat.f_frame_duration[(*h).sh.i_type as usize] += dur;
    if (*h).param.analyse.b_psnr != 0 {
        let mut ssd: [int64_t; 3] = [
            (*h).stat.frame.i_ssd[0],
            (*h).stat.frame.i_ssd[1],
            (*h).stat.frame.i_ssd[2],
        ];
        let mut luma_size: c_int = (*h).param.width as c_int * (*h).param.height as c_int;
        let mut chroma_size: c_int = if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            luma_size >> (*h).mb.chroma_h_shift + (*h).mb.chroma_v_shift
        } else {
            0 as c_int
        };
        (*pic_out).prop.f_psnr[0] = calc_psnr(ssd[0] as c_double, luma_size as c_double);
        (*pic_out).prop.f_psnr[1] = calc_psnr(ssd[1] as c_double, chroma_size as c_double);
        (*pic_out).prop.f_psnr[2] = calc_psnr(ssd[2] as c_double, chroma_size as c_double);
        (*pic_out).prop.f_psnr_avg = calc_psnr(
            (ssd[0] + ssd[1] + ssd[2]) as c_double,
            (luma_size + chroma_size * 2 as c_int) as c_double,
        );
        (*h).stat.f_ssd_global[(*h).sh.i_type as usize] +=
            dur * (ssd[0] + ssd[1] + ssd[2]) as c_double;
        (*h).stat.f_psnr_average[(*h).sh.i_type as usize] += dur * (*pic_out).prop.f_psnr_avg;
        (*h).stat.f_psnr_mean_y[(*h).sh.i_type as usize] += dur * (*pic_out).prop.f_psnr[0];
        (*h).stat.f_psnr_mean_u[(*h).sh.i_type as usize] += dur * (*pic_out).prop.f_psnr[1];
        (*h).stat.f_psnr_mean_v[(*h).sh.i_type as usize] += dur * (*pic_out).prop.f_psnr[2];
        snprintf(
            psz_message.as_mut_ptr(),
            80 as size_t,
            b" PSNR Y:%5.2f U:%5.2f V:%5.2f\0" as *const u8 as *const c_char,
            (*pic_out).prop.f_psnr[0],
            (*pic_out).prop.f_psnr[1],
            (*pic_out).prop.f_psnr[2],
        );
    }
    if (*h).param.analyse.b_ssim != 0 {
        (*pic_out).prop.f_ssim = (*h).stat.frame.f_ssim / (*h).stat.frame.i_ssim_cnt as c_double;
        (*h).stat.f_ssim_mean_y[(*h).sh.i_type as usize] += (*pic_out).prop.f_ssim * dur;
        let mut msg_len: c_int = strlen(psz_message.as_mut_ptr()) as c_int;
        snprintf(
            psz_message.as_mut_ptr().offset(msg_len as isize),
            (80 as c_int - msg_len) as size_t,
            b" SSIM Y:%.5f\0" as *const u8 as *const c_char,
            (*pic_out).prop.f_ssim,
        );
    }
    psz_message[79 as c_int as usize] = '\0' as i32 as c_char;
    x264_10_log(
        h,
        X264_LOG_DEBUG,
        b"frame=%4d QP=%.2f NAL=%d Slice:%c Poc:%-3d I:%-4d P:%-4d SKIP:%-4d size=%d bytes%s\n\0"
            as *const u8 as *const c_char,
        (*h).i_frame,
        (*(*h).fdec).f_qp_avg_aq as c_double,
        (*h).i_nal_ref_idc,
        if (*h).sh.i_type == SLICE_TYPE_I as c_int {
            'I' as i32
        } else if (*h).sh.i_type == SLICE_TYPE_P as c_int {
            'P' as i32
        } else {
            'B' as i32
        },
        (*(*h).fdec).i_poc,
        (*h).stat.frame.i_mb_count_i,
        (*h).stat.frame.i_mb_count_p,
        (*h).stat.frame.i_mb_count_skip,
        frame_size,
        psz_message.as_mut_ptr(),
    );
    thread_sync_stat((*h).thread[0], h);
    thread_sync_stat(thread_current, h);
    let mut i_9: c_int = 0 as c_int;
    while i_9 < (*h).i_ref[0] {
        if !(*h).fref[0][i_9 as usize].is_null() && (*(*h).fref[0][i_9 as usize]).b_duplicate != 0 {
            x264_10_frame_push_blank_unused(h, (*h).fref[0][i_9 as usize]);
            (*h).fref[0][i_9 as usize] = 0 as *mut x264_frame_t;
        }
        i_9 += 1;
    }
    if !(*h).param.psz_dump_yuv.is_null() {
        frame_dump(h);
    }
    return frame_size;
}
#[c2rust::src_loc = "4182:1"]
unsafe extern "C" fn print_intra(
    mut i_mb_count: *mut int64_t,
    mut i_count: c_double,
    mut b_print_pcm: c_int,
    mut intra: *mut c_char,
) {
    intra = intra.offset(sprintf(
        intra,
        b"I16..4%s: %4.1f%% %4.1f%% %4.1f%%\0" as *const u8 as *const c_char,
        if b_print_pcm != 0 {
            b"..PCM\0" as *const u8 as *const c_char
        } else {
            b"\0" as *const u8 as *const c_char
        },
        *i_mb_count.offset(I_16x16 as c_int as isize) as c_double / i_count,
        *i_mb_count.offset(I_8x8 as c_int as isize) as c_double / i_count,
        *i_mb_count.offset(I_4x4 as c_int as isize) as c_double / i_count,
    ) as isize);
    if b_print_pcm != 0 {
        sprintf(
            intra,
            b" %4.1f%%\0" as *const u8 as *const c_char,
            *i_mb_count.offset(I_PCM as c_int as isize) as c_double / i_count,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "4196:1"]
unsafe extern "C" fn x264_10_encoder_close(mut h: *mut x264_t) {
    let mut i_yuv_size: int64_t = ((*h).param.width as c_int * (*h).param.height as c_int
        + 2 as c_int
            * (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                (*h).param.width as c_int * (*h).param.height as c_int
                    >> (*h).mb.chroma_h_shift + (*h).mb.chroma_v_shift
            } else {
                0 as c_int
            })) as int64_t;
    let mut i_mb_count_size: [[int64_t; 7]; 2] =
        [[0 as c_int as int64_t, 0, 0, 0, 0, 0, 0], [0; 7]];
    let mut buf: [c_char; 200] = [0; 200];
    let mut b_print_pcm: c_int =
        ((*h).stat.i_mb_count[SLICE_TYPE_I as c_int as usize][I_PCM as c_int as usize] != 0
            || (*h).stat.i_mb_count[SLICE_TYPE_P as c_int as usize][I_PCM as c_int as usize] != 0
            || (*h).stat.i_mb_count[SLICE_TYPE_B as c_int as usize][I_PCM as c_int as usize] != 0)
            as c_int;
    x264_10_lookahead_delete(h);
    if (*h).param.b_sliced_threads != 0 {
        threadpool_wait_all(h);
    }
    if (*h).param.i_threads > 1 as c_int {
        x264_10_threadpool_delete((*h).threadpool);
    }
    if (*h).param.i_lookahead_threads > 1 as c_int {
        x264_10_threadpool_delete((*h).lookaheadpool);
    }
    if (*h).i_thread_frames > 1 as c_int {
        let mut i: c_int = 0 as c_int;
        while i < (*h).i_thread_frames {
            if (*(*h).thread[i as usize]).b_thread_active != 0 {
                if (*(*(*h).thread[i as usize]).fenc).i_reference_count == 1 as c_int {
                } else {
                    __assert_fail(
                        b"h->thread[i]->fenc->i_reference_count == 1\0" as *const u8
                            as *const c_char,
                        b"encoder/encoder.c\0" as *const u8 as *const c_char,
                        4223 as c_uint,
                        ::core::mem::transmute::<[u8; 37], [c_char; 37]>(
                            *b"void x264_10_encoder_close(x264_t *)\0",
                        )
                        .as_ptr(),
                    );
                }
                x264_10_frame_delete((*(*h).thread[i as usize]).fenc);
            }
            i += 1;
        }
        let mut thread_prev: *mut x264_t = (*h).thread[(*h).i_thread_phase as usize];
        x264_10_thread_sync_ratecontrol(h, thread_prev, h);
        x264_10_thread_sync_ratecontrol(thread_prev, thread_prev, h);
        (*h).i_frame = (*thread_prev).i_frame + 1 as c_int - (*h).i_thread_frames;
    }
    (*h).i_frame += 1;
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 3 as c_int {
        static mut slice_order: [uint8_t; 3] = [
            SLICE_TYPE_I as c_int as uint8_t,
            SLICE_TYPE_P as c_int as uint8_t,
            SLICE_TYPE_B as c_int as uint8_t,
        ];
        let mut i_slice: c_int = slice_order[i_0 as usize] as c_int;
        if (*h).stat.i_frame_count[i_slice as usize] > 0 as c_int {
            let mut i_count: c_int = (*h).stat.i_frame_count[i_slice as usize];
            let mut dur: c_double = (*h).stat.f_frame_duration[i_slice as usize];
            if (*h).param.analyse.b_psnr != 0 {
                x264_10_log(
                    h,
                    X264_LOG_INFO,
                    b"frame %c:%-5d Avg QP:%5.2f  size:%6.0f  PSNR Mean Y:%5.2f U:%5.2f V:%5.2f Avg:%5.2f Global:%5.2f\n\0"
                        as *const u8 as *const c_char,
                    slice_type_to_char[i_slice as usize] as c_int,
                    i_count,
                    (*h).stat.f_frame_qp[i_slice as usize]
                        / i_count as c_double,
                    (*h).stat.i_frame_size[i_slice as usize] as c_double
                        / i_count as c_double,
                    (*h).stat.f_psnr_mean_y[i_slice as usize] / dur,
                    (*h).stat.f_psnr_mean_u[i_slice as usize] / dur,
                    (*h).stat.f_psnr_mean_v[i_slice as usize] / dur,
                    (*h).stat.f_psnr_average[i_slice as usize] / dur,
                    calc_psnr(
                        (*h).stat.f_ssd_global[i_slice as usize],
                        dur * i_yuv_size as c_double,
                    ),
                );
            } else {
                x264_10_log(
                    h,
                    X264_LOG_INFO,
                    b"frame %c:%-5d Avg QP:%5.2f  size:%6.0f\n\0" as *const u8 as *const c_char,
                    slice_type_to_char[i_slice as usize] as c_int,
                    i_count,
                    (*h).stat.f_frame_qp[i_slice as usize] / i_count as c_double,
                    (*h).stat.i_frame_size[i_slice as usize] as c_double / i_count as c_double,
                );
            }
        }
        i_0 += 1;
    }
    if (*h).param.i_bframe != 0 && (*h).stat.i_frame_count[SLICE_TYPE_B as c_int as usize] != 0 {
        let mut p: *mut c_char = buf.as_mut_ptr();
        let mut den: c_int = 0 as c_int;
        let mut i_1: c_int = 0 as c_int;
        while i_1 <= (*h).param.i_bframe {
            den += (i_1 + 1 as c_int) * (*h).stat.i_consecutive_bframes[i_1 as usize];
            i_1 += 1;
        }
        let mut i_2: c_int = 0 as c_int;
        while i_2 <= (*h).param.i_bframe {
            p = p.offset(sprintf(
                p,
                b" %4.1f%%\0" as *const u8 as *const c_char,
                100.0f64
                    * (i_2 + 1 as c_int) as c_double
                    * (*h).stat.i_consecutive_bframes[i_2 as usize] as c_double
                    / den as c_double,
            ) as isize);
            i_2 += 1;
        }
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"consecutive B-frames:%s\n\0" as *const u8 as *const c_char,
            buf.as_mut_ptr(),
        );
    }
    let mut i_type: c_int = 0 as c_int;
    while i_type < 2 as c_int {
        let mut i_3: c_int = 0 as c_int;
        while i_3 < X264_PARTTYPE_MAX as c_int {
            if !(i_3 == D_DIRECT_8x8 as c_int) {
                i_mb_count_size[i_type as usize]
                    [x264_mb_partition_pixel_table[i_3 as usize] as usize] +=
                    (*h).stat.i_mb_partition[i_type as usize][i_3 as usize];
            }
            i_3 += 1;
        }
        i_type += 1;
    }
    if (*h).stat.i_frame_count[SLICE_TYPE_I as c_int as usize] > 0 as c_int {
        let mut i_mb_count: *mut int64_t = (*(*h)
            .stat
            .i_mb_count
            .as_mut_ptr()
            .offset(SLICE_TYPE_I as c_int as isize))
        .as_mut_ptr();
        let mut i_count_0: c_double = (*h).stat.i_frame_count[SLICE_TYPE_I as c_int as usize]
            as c_double
            * (*h).mb.i_mb_count as c_double
            / 100.0f64;
        print_intra(i_mb_count, i_count_0, b_print_pcm, buf.as_mut_ptr());
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"mb I  %s\n\0" as *const u8 as *const c_char,
            buf.as_mut_ptr(),
        );
    }
    if (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize] > 0 as c_int {
        let mut i_mb_count_0: *mut int64_t = (*(*h)
            .stat
            .i_mb_count
            .as_mut_ptr()
            .offset(SLICE_TYPE_P as c_int as isize))
        .as_mut_ptr();
        let mut i_count_1: c_double = (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize]
            as c_double
            * (*h).mb.i_mb_count as c_double
            / 100.0f64;
        let mut i_mb_size: *mut int64_t = (*i_mb_count_size
            .as_mut_ptr()
            .offset(SLICE_TYPE_P as c_int as isize))
        .as_mut_ptr();
        print_intra(i_mb_count_0, i_count_1, b_print_pcm, buf.as_mut_ptr());
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"mb P  %s  P16..4: %4.1f%% %4.1f%% %4.1f%% %4.1f%% %4.1f%%    skip:%4.1f%%\n\0"
                as *const u8 as *const c_char,
            buf.as_mut_ptr(),
            *i_mb_size.offset(PIXEL_16x16 as c_int as isize) as c_double
                / (i_count_1 * 4 as c_int as c_double),
            (*i_mb_size.offset(PIXEL_16x8 as c_int as isize)
                + *i_mb_size.offset(PIXEL_8x16 as c_int as isize)) as c_double
                / (i_count_1 * 4 as c_int as c_double),
            *i_mb_size.offset(PIXEL_8x8 as c_int as isize) as c_double
                / (i_count_1 * 4 as c_int as c_double),
            (*i_mb_size.offset(PIXEL_8x4 as c_int as isize)
                + *i_mb_size.offset(PIXEL_4x8 as c_int as isize)) as c_double
                / (i_count_1 * 4 as c_int as c_double),
            *i_mb_size.offset(PIXEL_4x4 as c_int as isize) as c_double
                / (i_count_1 * 4 as c_int as c_double),
            *i_mb_count_0.offset(P_SKIP as c_int as isize) as c_double / i_count_1,
        );
    }
    if (*h).stat.i_frame_count[SLICE_TYPE_B as c_int as usize] > 0 as c_int {
        let mut i_mb_count_1: *mut int64_t = (*(*h)
            .stat
            .i_mb_count
            .as_mut_ptr()
            .offset(SLICE_TYPE_B as c_int as isize))
        .as_mut_ptr();
        let mut i_count_2: c_double = (*h).stat.i_frame_count[SLICE_TYPE_B as c_int as usize]
            as c_double
            * (*h).mb.i_mb_count as c_double
            / 100.0f64;
        let mut i_mb_list_count: c_double = 0.;
        let mut i_mb_size_0: *mut int64_t = (*i_mb_count_size
            .as_mut_ptr()
            .offset(SLICE_TYPE_B as c_int as isize))
        .as_mut_ptr();
        let mut list_count: [int64_t; 3] = [0 as c_int as int64_t, 0, 0];
        print_intra(i_mb_count_1, i_count_2, b_print_pcm, buf.as_mut_ptr());
        let mut i_4: c_int = 0 as c_int;
        while i_4 < X264_PARTTYPE_MAX as c_int {
            let mut j: c_int = 0 as c_int;
            while j < 2 as c_int {
                let mut l0: c_int = x264_mb_type_list_table[i_4 as usize][0][j as usize] as c_int;
                let mut l1: c_int = x264_mb_type_list_table[i_4 as usize][1][j as usize] as c_int;
                if l0 != 0 || l1 != 0 {
                    list_count[(l1 + l0 * l1) as usize] += (*h).stat.i_mb_count
                        [SLICE_TYPE_B as c_int as usize][i_4 as usize]
                        * 2 as int64_t;
                }
                j += 1;
            }
            i_4 += 1;
        }
        list_count[0] +=
            (*h).stat.i_mb_partition[SLICE_TYPE_B as c_int as usize][D_L0_8x8 as c_int as usize];
        list_count[1] +=
            (*h).stat.i_mb_partition[SLICE_TYPE_B as c_int as usize][D_L1_8x8 as c_int as usize];
        list_count[2] +=
            (*h).stat.i_mb_partition[SLICE_TYPE_B as c_int as usize][D_BI_8x8 as c_int as usize];
        *i_mb_count_1.offset(B_DIRECT as c_int as isize) += ((*h).stat.i_mb_partition
            [SLICE_TYPE_B as c_int as usize][D_DIRECT_8x8 as c_int as usize]
            + 2 as int64_t)
            / 4 as int64_t;
        i_mb_list_count = (list_count[0] + list_count[1] + list_count[2]) as c_double / 100.0f64;
        sprintf(
            buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize),
            b"  B16..8: %4.1f%% %4.1f%% %4.1f%%  direct:%4.1f%%  skip:%4.1f%%\0" as *const u8
                as *const c_char,
            *i_mb_size_0.offset(PIXEL_16x16 as c_int as isize) as c_double
                / (i_count_2 * 4 as c_int as c_double),
            (*i_mb_size_0.offset(PIXEL_16x8 as c_int as isize)
                + *i_mb_size_0.offset(PIXEL_8x16 as c_int as isize)) as c_double
                / (i_count_2 * 4 as c_int as c_double),
            *i_mb_size_0.offset(PIXEL_8x8 as c_int as isize) as c_double
                / (i_count_2 * 4 as c_int as c_double),
            *i_mb_count_1.offset(B_DIRECT as c_int as isize) as c_double / i_count_2,
            *i_mb_count_1.offset(B_SKIP as c_int as isize) as c_double / i_count_2,
        );
        if i_mb_list_count != 0 as c_int as c_double {
            sprintf(
                buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize),
                b"  L0:%4.1f%% L1:%4.1f%% BI:%4.1f%%\0" as *const u8 as *const c_char,
                list_count[0] as c_double / i_mb_list_count,
                list_count[1] as c_double / i_mb_list_count,
                list_count[2] as c_double / i_mb_list_count,
            );
        }
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"mb B  %s\n\0" as *const u8 as *const c_char,
            buf.as_mut_ptr(),
        );
    }
    x264_10_ratecontrol_summary(h);
    if (*h).stat.i_frame_count[SLICE_TYPE_I as c_int as usize]
        + (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize]
        + (*h).stat.i_frame_count[SLICE_TYPE_B as c_int as usize]
        > 0 as c_int
    {
        let mut i_i8x8: int64_t = (*h).stat.i_mb_count[SLICE_TYPE_I as c_int as usize]
            [I_8x8 as c_int as usize]
            + (*h).stat.i_mb_count[SLICE_TYPE_P as c_int as usize][I_8x8 as c_int as usize]
            + (*h).stat.i_mb_count[SLICE_TYPE_B as c_int as usize][I_8x8 as c_int as usize];
        let mut i_intra: int64_t = i_i8x8
            + ((*h).stat.i_mb_count[SLICE_TYPE_I as c_int as usize][I_4x4 as c_int as usize]
                + (*h).stat.i_mb_count[SLICE_TYPE_P as c_int as usize][I_4x4 as c_int as usize]
                + (*h).stat.i_mb_count[SLICE_TYPE_B as c_int as usize][I_4x4 as c_int as usize])
            + ((*h).stat.i_mb_count[SLICE_TYPE_I as c_int as usize][I_16x16 as c_int as usize]
                + (*h).stat.i_mb_count[SLICE_TYPE_P as c_int as usize][I_16x16 as c_int as usize]
                + (*h).stat.i_mb_count[SLICE_TYPE_B as c_int as usize][I_16x16 as c_int as usize]);
        let mut i_all_intra: int64_t = i_intra
            + ((*h).stat.i_mb_count[SLICE_TYPE_I as c_int as usize][I_PCM as c_int as usize]
                + (*h).stat.i_mb_count[SLICE_TYPE_P as c_int as usize][I_PCM as c_int as usize]
                + (*h).stat.i_mb_count[SLICE_TYPE_B as c_int as usize][I_PCM as c_int as usize]);
        let mut i_skip: int64_t = (*h).stat.i_mb_count[SLICE_TYPE_I as c_int as usize]
            [P_SKIP as c_int as usize]
            + (*h).stat.i_mb_count[SLICE_TYPE_P as c_int as usize][P_SKIP as c_int as usize]
            + (*h).stat.i_mb_count[SLICE_TYPE_B as c_int as usize][P_SKIP as c_int as usize]
            + ((*h).stat.i_mb_count[SLICE_TYPE_I as c_int as usize][B_SKIP as c_int as usize]
                + (*h).stat.i_mb_count[SLICE_TYPE_P as c_int as usize][B_SKIP as c_int as usize]
                + (*h).stat.i_mb_count[SLICE_TYPE_B as c_int as usize][B_SKIP as c_int as usize]);
        let i_count_3: c_int = (*h).stat.i_frame_count[SLICE_TYPE_I as c_int as usize]
            + (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize]
            + (*h).stat.i_frame_count[SLICE_TYPE_B as c_int as usize];
        let mut i_mb_count_2: int64_t = i_count_3 as int64_t * (*h).mb.i_mb_count as int64_t;
        let mut i_inter: int64_t = i_mb_count_2 - i_skip - i_all_intra;
        let duration: c_double = (*h).stat.f_frame_duration[SLICE_TYPE_I as c_int as usize]
            + (*h).stat.f_frame_duration[SLICE_TYPE_P as c_int as usize]
            + (*h).stat.f_frame_duration[SLICE_TYPE_B as c_int as usize];
        let mut f_bitrate: c_float = (((*h).stat.i_frame_size[SLICE_TYPE_I as c_int as usize]
            + (*h).stat.i_frame_size[SLICE_TYPE_P as c_int as usize]
            + (*h).stat.i_frame_size[SLICE_TYPE_B as c_int as usize])
            as c_double
            / duration
            / 125 as c_int as c_double) as c_float;
        if (*h).param.b_interlaced != 0 {
            let mut fieldstats: *mut c_char = buf.as_mut_ptr();
            *fieldstats.offset(0) = 0 as c_char;
            if i_inter != 0 {
                fieldstats = fieldstats.offset(sprintf(
                    fieldstats,
                    b" inter:%.1f%%\0" as *const u8 as *const c_char,
                    (*h).stat.i_mb_field[1] as c_double * 100.0f64 / i_inter as c_double,
                ) as isize);
            }
            if i_skip != 0 {
                fieldstats = fieldstats.offset(sprintf(
                    fieldstats,
                    b" skip:%.1f%%\0" as *const u8 as *const c_char,
                    (*h).stat.i_mb_field[2] as c_double * 100.0f64 / i_skip as c_double,
                ) as isize);
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"field mbs: intra: %.1f%%%s\n\0" as *const u8 as *const c_char,
                (*h).stat.i_mb_field[0] as c_double * 100.0f64 / i_all_intra as c_double,
                buf.as_mut_ptr(),
            );
        }
        if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
            buf[0] = 0 as c_char;
            if (*h).stat.i_mb_count_8x8dct[0] != 0 {
                sprintf(
                    buf.as_mut_ptr(),
                    b" inter:%.1f%%\0" as *const u8 as *const c_char,
                    100.0f64 * (*h).stat.i_mb_count_8x8dct[1] as c_double
                        / (*h).stat.i_mb_count_8x8dct[0] as c_double,
                );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"8x8 transform intra:%.1f%%%s\n\0" as *const u8 as *const c_char,
                100.0f64 * i_i8x8 as c_double
                    / (if i_intra > 1 as int64_t {
                        i_intra
                    } else {
                        1 as int64_t
                    }) as c_double,
                buf.as_mut_ptr(),
            );
        }
        if ((*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_AUTO
            || (*h).stat.i_direct_frames[0] != 0 && (*h).stat.i_direct_frames[1] != 0)
            && (*h).stat.i_frame_count[SLICE_TYPE_B as c_int as usize] != 0
        {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"direct mvs  spatial:%.1f%% temporal:%.1f%%\n\0" as *const u8 as *const c_char,
                (*h).stat.i_direct_frames[1] as c_double * 100.0f64
                    / (*h).stat.i_frame_count[SLICE_TYPE_B as c_int as usize] as c_double,
                (*h).stat.i_direct_frames[0] as c_double * 100.0f64
                    / (*h).stat.i_frame_count[SLICE_TYPE_B as c_int as usize] as c_double,
            );
        }
        buf[0] = 0 as c_char;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            let mut csize: c_int =
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                    4 as c_int
                } else {
                    1 as c_int
                };
            if i_mb_count_2 != i_all_intra {
                sprintf(
                    buf.as_mut_ptr(),
                    b" inter: %.1f%% %.1f%% %.1f%%\0" as *const u8 as *const c_char,
                    (*h).stat.i_mb_cbp[1] as c_double * 100.0f64
                        / ((i_mb_count_2 - i_all_intra) * 4 as int64_t) as c_double,
                    (*h).stat.i_mb_cbp[3] as c_double * 100.0f64
                        / ((i_mb_count_2 - i_all_intra) * csize as int64_t) as c_double,
                    (*h).stat.i_mb_cbp[5] as c_double * 100.0f64
                        / ((i_mb_count_2 - i_all_intra) * csize as int64_t) as c_double,
                );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"coded y,%s,%s intra: %.1f%% %.1f%% %.1f%%%s\n\0" as *const u8 as *const c_char,
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                    b"u\0" as *const u8 as *const c_char
                } else {
                    b"uvDC\0" as *const u8 as *const c_char
                },
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
                    b"v\0" as *const u8 as *const c_char
                } else {
                    b"uvAC\0" as *const u8 as *const c_char
                },
                (*h).stat.i_mb_cbp[0] as c_double * 100.0f64
                    / (i_all_intra * 4 as int64_t) as c_double,
                (*h).stat.i_mb_cbp[2] as c_double * 100.0f64
                    / (i_all_intra * csize as int64_t) as c_double,
                (*h).stat.i_mb_cbp[4] as c_double * 100.0f64
                    / (i_all_intra * csize as int64_t) as c_double,
                buf.as_mut_ptr(),
            );
        } else {
            if i_mb_count_2 != i_all_intra {
                sprintf(
                    buf.as_mut_ptr(),
                    b" inter: %.1f%%\0" as *const u8 as *const c_char,
                    (*h).stat.i_mb_cbp[1] as c_double * 100.0f64
                        / ((i_mb_count_2 - i_all_intra) * 4 as int64_t) as c_double,
                );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"coded y intra: %.1f%%%s\n\0" as *const u8 as *const c_char,
                (*h).stat.i_mb_cbp[0] as c_double * 100.0f64
                    / (i_all_intra * 4 as int64_t) as c_double,
                buf.as_mut_ptr(),
            );
        }
        let mut fixed_pred_modes: [[int64_t; 9]; 4] = [
            [0 as c_int as int64_t, 0, 0, 0, 0, 0, 0, 0, 0],
            [0; 9],
            [0; 9],
            [0; 9],
        ];
        let mut sum_pred_modes: [int64_t; 4] = [0 as c_int as int64_t, 0, 0, 0];
        let mut i_5: c_int = 0 as c_int;
        while i_5 <= I_PRED_16x16_DC_128 as c_int {
            fixed_pred_modes[0][x264_mb_pred_mode16x16_fix[i_5 as usize] as usize] +=
                (*h).stat.i_mb_pred_mode[0][i_5 as usize];
            sum_pred_modes[0] += (*h).stat.i_mb_pred_mode[0][i_5 as usize];
            i_5 += 1;
        }
        if sum_pred_modes[0] != 0 {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"i16 v,h,dc,p: %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0" as *const u8 as *const c_char,
                fixed_pred_modes[0][0] as c_double * 100.0f64 / sum_pred_modes[0] as c_double,
                fixed_pred_modes[0][1] as c_double * 100.0f64 / sum_pred_modes[0] as c_double,
                fixed_pred_modes[0][2] as c_double * 100.0f64 / sum_pred_modes[0] as c_double,
                fixed_pred_modes[0][3] as c_double * 100.0f64 / sum_pred_modes[0] as c_double,
            );
        }
        let mut i_6: c_int = 1 as c_int;
        while i_6 <= 2 as c_int {
            let mut j_0: c_int = 0 as c_int;
            while j_0 <= I_PRED_8x8_DC_128 as c_int {
                fixed_pred_modes[i_6 as usize]
                    [x264_mb_pred_mode4x4_fix[(j_0 + 1 as c_int) as usize] as usize] +=
                    (*h).stat.i_mb_pred_mode[i_6 as usize][j_0 as usize];
                sum_pred_modes[i_6 as usize] +=
                    (*h).stat.i_mb_pred_mode[i_6 as usize][j_0 as usize];
                j_0 += 1;
            }
            if sum_pred_modes[i_6 as usize] != 0 {
                x264_10_log(
                    h,
                    X264_LOG_INFO,
                    b"i%d v,h,dc,ddl,ddr,vr,hd,vl,hu: %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0"
                        as *const u8 as *const c_char,
                    (3 as c_int - i_6) * 4 as c_int,
                    fixed_pred_modes[i_6 as usize][0]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                    fixed_pred_modes[i_6 as usize][1]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                    fixed_pred_modes[i_6 as usize][2]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                    fixed_pred_modes[i_6 as usize][3]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                    fixed_pred_modes[i_6 as usize][4]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                    fixed_pred_modes[i_6 as usize][5]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                    fixed_pred_modes[i_6 as usize][6]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                    fixed_pred_modes[i_6 as usize][7]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                    fixed_pred_modes[i_6 as usize][8]
                        as c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as c_double,
                );
            }
            i_6 += 1;
        }
        let mut i_7: c_int = 0 as c_int;
        while i_7 <= I_PRED_CHROMA_DC_128 as c_int {
            fixed_pred_modes[3][x264_mb_chroma_pred_mode_fix[i_7 as usize] as usize] +=
                (*h).stat.i_mb_pred_mode[3][i_7 as usize];
            sum_pred_modes[3] += (*h).stat.i_mb_pred_mode[3][i_7 as usize];
            i_7 += 1;
        }
        if sum_pred_modes[3] != 0
            && !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int)
        {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"i8c dc,h,v,p: %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0" as *const u8 as *const c_char,
                fixed_pred_modes[3][0] as c_double * 100.0f64 / sum_pred_modes[3] as c_double,
                fixed_pred_modes[3][1] as c_double * 100.0f64 / sum_pred_modes[3] as c_double,
                fixed_pred_modes[3][2] as c_double * 100.0f64 / sum_pred_modes[3] as c_double,
                fixed_pred_modes[3][3] as c_double * 100.0f64 / sum_pred_modes[3] as c_double,
            );
        }
        if (*h).param.analyse.i_weighted_pred >= X264_WEIGHTP_SIMPLE
            && (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize] > 0 as c_int
        {
            buf[0] = 0 as c_char;
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                sprintf(
                    buf.as_mut_ptr(),
                    b" UV:%.1f%%\0" as *const u8 as *const c_char,
                    (*h).stat.i_wpred[1] as c_double * 100.0f64
                        / (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize] as c_double,
                );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"Weighted P-Frames: Y:%.1f%%%s\n\0" as *const u8 as *const c_char,
                (*h).stat.i_wpred[0] as c_double * 100.0f64
                    / (*h).stat.i_frame_count[SLICE_TYPE_P as c_int as usize] as c_double,
                buf.as_mut_ptr(),
            );
        }
        let mut i_list: c_int = 0 as c_int;
        while i_list < 2 as c_int {
            let mut i_slice_0: c_int = 0 as c_int;
            while i_slice_0 < 2 as c_int {
                let mut p_0: *mut c_char = buf.as_mut_ptr();
                let mut i_den: int64_t = 0 as int64_t;
                let mut i_max: c_int = 0 as c_int;
                let mut i_8: c_int = 0 as c_int;
                while i_8 < X264_REF_MAX * 2 as c_int {
                    if (*h).stat.i_mb_count_ref[i_slice_0 as usize][i_list as usize][i_8 as usize]
                        != 0
                    {
                        i_den += (*h).stat.i_mb_count_ref[i_slice_0 as usize][i_list as usize]
                            [i_8 as usize];
                        i_max = i_8;
                    }
                    i_8 += 1;
                }
                if !(i_max == 0 as c_int) {
                    let mut i_9: c_int = 0 as c_int;
                    while i_9 <= i_max {
                        p_0 = p_0.offset(sprintf(
                            p_0,
                            b" %4.1f%%\0" as *const u8 as *const c_char,
                            100.0f64
                                * (*h).stat.i_mb_count_ref[i_slice_0 as usize][i_list as usize]
                                    [i_9 as usize] as c_double
                                / i_den as c_double,
                        ) as isize);
                        i_9 += 1;
                    }
                    x264_10_log(
                        h,
                        X264_LOG_INFO,
                        b"ref %c L%d:%s\n\0" as *const u8 as *const c_char,
                        ::core::mem::transmute::<[u8; 3], [c_char; 3]>(*b"PB\0")[i_slice_0 as usize]
                            as c_int,
                        i_list,
                        buf.as_mut_ptr(),
                    );
                }
                i_slice_0 += 1;
            }
            i_list += 1;
        }
        if (*h).param.analyse.b_ssim != 0 {
            let mut ssim: c_float = (((*h).stat.f_ssim_mean_y[SLICE_TYPE_I as c_int as usize]
                + (*h).stat.f_ssim_mean_y[SLICE_TYPE_P as c_int as usize]
                + (*h).stat.f_ssim_mean_y[SLICE_TYPE_B as c_int as usize])
                / duration) as c_float;
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"SSIM Mean Y:%.7f (%6.3fdb)\n\0" as *const u8 as *const c_char,
                ssim as c_double,
                calc_ssim_db(ssim as c_double),
            );
        }
        if (*h).param.analyse.b_psnr != 0 {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"PSNR Mean Y:%6.3f U:%6.3f V:%6.3f Avg:%6.3f Global:%6.3f kb/s:%.2f\n\0"
                    as *const u8 as *const c_char,
                ((*h).stat.f_psnr_mean_y[SLICE_TYPE_I as c_int as usize]
                    + (*h).stat.f_psnr_mean_y[SLICE_TYPE_P as c_int as usize]
                    + (*h).stat.f_psnr_mean_y[SLICE_TYPE_B as c_int as usize])
                    / duration,
                ((*h).stat.f_psnr_mean_u[SLICE_TYPE_I as c_int as usize]
                    + (*h).stat.f_psnr_mean_u[SLICE_TYPE_P as c_int as usize]
                    + (*h).stat.f_psnr_mean_u[SLICE_TYPE_B as c_int as usize])
                    / duration,
                ((*h).stat.f_psnr_mean_v[SLICE_TYPE_I as c_int as usize]
                    + (*h).stat.f_psnr_mean_v[SLICE_TYPE_P as c_int as usize]
                    + (*h).stat.f_psnr_mean_v[SLICE_TYPE_B as c_int as usize])
                    / duration,
                ((*h).stat.f_psnr_average[SLICE_TYPE_I as c_int as usize]
                    + (*h).stat.f_psnr_average[SLICE_TYPE_P as c_int as usize]
                    + (*h).stat.f_psnr_average[SLICE_TYPE_B as c_int as usize])
                    / duration,
                calc_psnr(
                    (*h).stat.f_ssd_global[SLICE_TYPE_I as c_int as usize]
                        + (*h).stat.f_ssd_global[SLICE_TYPE_P as c_int as usize]
                        + (*h).stat.f_ssd_global[SLICE_TYPE_B as c_int as usize],
                    duration * i_yuv_size as c_double,
                ),
                f_bitrate as c_double,
            );
        } else {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"kb/s:%.2f\n\0" as *const u8 as *const c_char,
                f_bitrate as c_double,
            );
        }
    }
    x264_10_ratecontrol_delete(h);
    x264_param_cleanup(&mut (*h).param);
    x264_10_cqm_delete(h);
    x264_free((*h).nal_buffer as *mut c_void);
    x264_free((*h).reconfig_h as *mut c_void);
    x264_10_analyse_free_costs(h);
    x264_free((*h).cost_table as *mut c_void);
    if (*h).i_thread_frames > 1 as c_int {
        h = (*h).thread[(*h).i_thread_phase as usize];
    }
    x264_10_frame_delete_list((*h).frames.unused[0]);
    x264_10_frame_delete_list((*h).frames.unused[1]);
    x264_10_frame_delete_list((*h).frames.current);
    x264_10_frame_delete_list((*h).frames.blank_unused);
    h = (*h).thread[0];
    let mut i_10: c_int = 0 as c_int;
    while i_10 < (*h).i_thread_frames {
        if (*(*h).thread[i_10 as usize]).b_thread_active != 0 {
            let mut j_1: c_int = 0 as c_int;
            while j_1 < (*(*h).thread[i_10 as usize]).i_ref[0] {
                if !(*(*h).thread[i_10 as usize]).fref[0][j_1 as usize].is_null()
                    && (*(*(*h).thread[i_10 as usize]).fref[0][j_1 as usize]).b_duplicate != 0
                {
                    x264_10_frame_delete((*(*h).thread[i_10 as usize]).fref[0][j_1 as usize]);
                }
                j_1 += 1;
            }
        }
        i_10 += 1;
    }
    if (*h).param.i_lookahead_threads > 1 as c_int {
        let mut i_11: c_int = 0 as c_int;
        while i_11 < (*h).param.i_lookahead_threads {
            x264_free((*h).lookahead_thread[i_11 as usize] as *mut c_void);
            i_11 += 1;
        }
    }
    let mut i_12: c_int = (*h).param.i_threads - 1 as c_int;
    while i_12 >= 0 as c_int {
        let mut frame: *mut *mut x264_frame_t = 0 as *mut *mut x264_frame_t;
        if (*h).param.b_sliced_threads == 0 || i_12 == 0 as c_int {
            frame = (**(*h).thread.as_mut_ptr().offset(i_12 as isize))
                .frames
                .reference
                .as_mut_ptr();
            while !(*frame).is_null() {
                if (**frame).i_reference_count > 0 as c_int {
                } else {
                    __assert_fail(
                        b"(*frame)->i_reference_count > 0\0" as *const u8 as *const c_char,
                        b"encoder/encoder.c\0" as *const u8 as *const c_char,
                        4552 as c_uint,
                        ::core::mem::transmute::<[u8; 37], [c_char; 37]>(
                            *b"void x264_10_encoder_close(x264_t *)\0",
                        )
                        .as_ptr(),
                    );
                }
                (**frame).i_reference_count -= 1;
                if (**frame).i_reference_count == 0 as c_int {
                    x264_10_frame_delete(*frame);
                }
                frame = frame.offset(1);
            }
            frame = &mut (**(*h).thread.as_mut_ptr().offset(i_12 as isize)).fdec;
            if !(*frame).is_null() {
                if (**frame).i_reference_count > 0 as c_int {
                } else {
                    __assert_fail(
                        b"(*frame)->i_reference_count > 0\0" as *const u8 as *const c_char,
                        b"encoder/encoder.c\0" as *const u8 as *const c_char,
                        4560 as c_uint,
                        ::core::mem::transmute::<[u8; 37], [c_char; 37]>(
                            *b"void x264_10_encoder_close(x264_t *)\0",
                        )
                        .as_ptr(),
                    );
                }
                (**frame).i_reference_count -= 1;
                if (**frame).i_reference_count == 0 as c_int {
                    x264_10_frame_delete(*frame);
                }
            }
            x264_10_macroblock_cache_free((*h).thread[i_12 as usize]);
        }
        x264_10_macroblock_thread_free((*h).thread[i_12 as usize], 0 as c_int);
        x264_free((*(*h).thread[i_12 as usize]).out.p_bitstream as *mut c_void);
        x264_free((*(*h).thread[i_12 as usize]).out.nal as *mut c_void);
        pthread_mutex_destroy(&mut (**(*h).thread.as_mut_ptr().offset(i_12 as isize)).mutex);
        pthread_cond_destroy(&mut (**(*h).thread.as_mut_ptr().offset(i_12 as isize)).cv);
        x264_free((*h).thread[i_12 as usize] as *mut c_void);
        i_12 -= 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "4579:1"]
unsafe extern "C" fn x264_10_encoder_delayed_frames(mut h: *mut x264_t) -> c_int {
    let mut delayed_frames: c_int = 0 as c_int;
    if (*h).i_thread_frames > 1 as c_int {
        let mut i: c_int = 0 as c_int;
        while i < (*h).i_thread_frames {
            delayed_frames += (*(*h).thread[i as usize]).b_thread_active;
            i += 1;
        }
        h = (*h).thread[(*h).i_thread_phase as usize];
    }
    let mut i_0: c_int = 0 as c_int;
    while !(*(*h).frames.current.offset(i_0 as isize)).is_null() {
        delayed_frames += 1;
        i_0 += 1;
    }
    pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
    pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
    pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
    delayed_frames += (*(*h).lookahead).ifbuf.i_size
        + (*(*h).lookahead).next.i_size
        + (*(*h).lookahead).ofbuf.i_size;
    pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
    return delayed_frames;
}
#[no_mangle]
#[c2rust::src_loc = "4600:1"]
unsafe extern "C" fn x264_10_encoder_maximum_delayed_frames(mut h: *mut x264_t) -> c_int {
    return (*h).frames.i_delay;
}
