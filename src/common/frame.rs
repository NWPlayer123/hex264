use core::ffi::{c_char, c_float, c_int, c_uint, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::assert_h::__assert_fail;
use crate::base_h::{
    x264_free, x264_malloc, x264_union16_t, x264_union32_t, x264_union64_t, CHROMA_444,
};
use crate::common_h::{pixel, x264_10_log, x264_t, SIZEOF_PIXEL};
use crate::frame_h::{x264_frame, x264_frame_t, x264_sync_frame_list_t, PADH, PADV};
use crate::internal::BIT_DEPTH;
use crate::mc_h::x264_weight_t;
use crate::osdep_h::{x264_pthread_fetch_and_add, NATIVE_ALIGN, WORD_SIZE};
use crate::pthread_h::{
    pthread_cond_broadcast, pthread_cond_destroy, pthread_cond_init, pthread_cond_wait,
    pthread_mutex_destroy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock,
};
use crate::pthreadtypes_h::{pthread_condattr_t, pthread_mutexattr_t};
use crate::stdint_h::intptr_t;
use crate::stdint_intn_h::{int16_t, int64_t, int8_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::stdlib_h::abs;
use crate::string_h::{memcpy, memset};
use crate::x264_h::{
    x264_param_cleanup, x264_picture_t, PIC_STRUCT_AUTO, X264_CPU_AVX, X264_CPU_AVX512,
    X264_CPU_CACHELINE_32, X264_CPU_CACHELINE_64, X264_CSP_BGR, X264_CSP_BGRA, X264_CSP_HIGH_DEPTH,
    X264_CSP_I400, X264_CSP_I420, X264_CSP_I422, X264_CSP_I444, X264_CSP_MASK, X264_CSP_NONE,
    X264_CSP_NV12, X264_CSP_NV16, X264_CSP_NV21, X264_CSP_RGB, X264_CSP_UYVY, X264_CSP_V210,
    X264_CSP_VFLIP, X264_CSP_YUYV, X264_CSP_YV12, X264_CSP_YV16, X264_CSP_YV24, X264_LOG_ERROR,
    X264_LOG_WARNING, X264_ME_ESA, X264_QP_AUTO, X264_TYPE_AUTO, X264_TYPE_KEYFRAME,
};
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn align_stride(mut x: c_int, mut align: c_int, mut disalign: c_int) -> c_int {
    x = x + (align - 1 as c_int) & !(align - 1 as c_int);
    if x & disalign - 1 as c_int == 0 {
        x += align;
    }
    return x;
}
#[c2rust::src_loc = "38:1"]
unsafe extern "C" fn align_plane_size(mut x: c_int, mut disalign: c_int) -> c_int {
    if x & disalign - 1 as c_int == 0 {
        x += (if 128 as c_int > 64 as c_int {
            128 as c_int
        } else {
            64 as c_int
        }) / SIZEOF_PIXEL;
    }
    return x;
}
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn frame_internal_csp(mut external_csp: c_int) -> c_int {
    let mut csp: c_int = external_csp & X264_CSP_MASK;
    if csp == X264_CSP_I400 {
        return X264_CSP_I400;
    }
    if csp >= X264_CSP_I420 && csp < X264_CSP_I422 {
        return X264_CSP_NV12;
    }
    if csp >= X264_CSP_I422 && csp < X264_CSP_I444 {
        return X264_CSP_NV16;
    }
    if csp >= X264_CSP_I444 && csp <= X264_CSP_RGB {
        return X264_CSP_I444;
    }
    return X264_CSP_NONE;
}
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn frame_new(mut h: *mut x264_t, mut b_fdec: c_int) -> *mut x264_frame_t {
    let mut prealloc_idx: c_int = 0;
    let mut prealloc_size: int64_t = 0;
    let mut preallocs: [*mut *mut uint8_t; 1024] = [0 as *mut *mut uint8_t; 1024];
    let mut current_block: u64;
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    let mut i_csp: c_int = frame_internal_csp((*h).param.i_csp);
    let mut i_mb_count: c_int = (*h).mb.i_mb_count;
    let mut i_stride: c_int = 0;
    let mut i_width: c_int = 0;
    let mut i_lines: c_int = 0;
    let mut luma_plane_count: c_int = 0;
    let mut i_padv: c_int = PADV << (*h).param.b_interlaced;
    let mut align: c_int = NATIVE_ALIGN / SIZEOF_PIXEL;
    if (*h).param.cpu & X264_CPU_CACHELINE_64 as uint32_t != 0
        || (*h).param.cpu & X264_CPU_AVX512 as uint32_t != 0
    {
        align = 64 as c_int / SIZEOF_PIXEL;
    } else if (*h).param.cpu & X264_CPU_CACHELINE_32 as uint32_t != 0
        || (*h).param.cpu & X264_CPU_AVX as uint32_t != 0
    {
        align = 32 as c_int / SIZEOF_PIXEL;
    } else {
        align = 16 as c_int / SIZEOF_PIXEL;
    }
    let mut disalign: c_int = ((1 as c_int) << 10 as c_int) / SIZEOF_PIXEL;
    frame = x264_malloc(::core::mem::size_of::<x264_frame_t>() as int64_t) as *mut x264_frame_t;
    if !frame.is_null() {
        memset(
            frame as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<x264_frame_t>() as size_t,
        );
        prealloc_idx = 0 as c_int;
        prealloc_size = 0 as int64_t;
        preallocs = [0 as *mut *mut uint8_t; 1024];
        i_width = (*h).mb.i_mb_width * 16 as c_int;
        i_lines = (*h).mb.i_mb_height * 16 as c_int;
        i_stride = align_stride(
            i_width
                + ((if 32 as c_int > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int {
                    32 as c_int
                } else {
                    64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                }) + PADH),
            align,
            disalign,
        );
        if i_csp == X264_CSP_NV12 || i_csp == X264_CSP_NV16 {
            luma_plane_count = 1 as c_int;
            (*frame).i_plane = 2 as c_int;
            let mut i: c_int = 0 as c_int;
            while i < 2 as c_int {
                (*frame).i_width[i as usize] = i_width >> i;
                (*frame).i_lines[i as usize] =
                    i_lines >> (i != 0 && i_csp == X264_CSP_NV12) as c_int;
                (*frame).i_stride[i as usize] = i_stride;
                i += 1;
            }
            current_block = 7245201122033322888;
        } else if i_csp == X264_CSP_I444 {
            luma_plane_count = 3 as c_int;
            (*frame).i_plane = 3 as c_int;
            let mut i_0: c_int = 0 as c_int;
            while i_0 < 3 as c_int {
                (*frame).i_width[i_0 as usize] = i_width;
                (*frame).i_lines[i_0 as usize] = i_lines;
                (*frame).i_stride[i_0 as usize] = i_stride;
                i_0 += 1;
            }
            current_block = 7245201122033322888;
        } else if i_csp == X264_CSP_I400 {
            luma_plane_count = 1 as c_int;
            (*frame).i_plane = 1 as c_int;
            (*frame).i_width[0 as c_int as usize] = i_width;
            (*frame).i_lines[0 as c_int as usize] = i_lines;
            (*frame).i_stride[0 as c_int as usize] = i_stride;
            current_block = 7245201122033322888;
        } else {
            current_block = 18021720757857092697;
        }
        match current_block {
            18021720757857092697 => {}
            _ => {
                (*frame).i_csp = i_csp;
                (*frame).i_width_lowres = (*frame).i_width[0 as c_int as usize] / 2 as c_int;
                (*frame).i_lines_lowres = (*frame).i_lines[0 as c_int as usize] / 2 as c_int;
                (*frame).i_stride_lowres = align_stride(
                    (*frame).i_width_lowres
                        + ((if 32 as c_int > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                        {
                            32 as c_int
                        } else {
                            64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                        }) + PADH),
                    align,
                    disalign << 1 as c_int,
                );
                let mut i_1: c_int = 0 as c_int;
                while i_1 < (*h).param.i_bframe + 2 as c_int {
                    let mut j: c_int = 0 as c_int;
                    while j < (*h).param.i_bframe + 2 as c_int {
                        (*frame).i_row_satds[i_1 as usize][j as usize] =
                            prealloc_size as intptr_t as *mut c_void as *mut c_int;
                        let fresh8 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh8 as usize] =
                            &mut *(*(*frame).i_row_satds.as_mut_ptr().offset(i_1 as isize))
                                .as_mut_ptr()
                                .offset(j as isize) as *mut *mut c_int
                                as *mut *mut uint8_t;
                        prealloc_size += ((i_lines / 16 as c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<c_int>() as usize)
                            as int64_t
                            + (64 as c_int - 1 as c_int) as int64_t
                            & !(64 as c_int - 1 as c_int) as int64_t;
                        j += 1;
                    }
                    i_1 += 1;
                }
                (*frame).i_poc = -(1 as c_int);
                (*frame).i_type = X264_TYPE_AUTO;
                (*frame).i_qpplus1 = X264_QP_AUTO;
                (*frame).i_pts = -(1 as c_int) as int64_t;
                (*frame).i_frame = -(1 as c_int);
                (*frame).i_frame_num = -(1 as c_int);
                (*frame).i_lines_completed = -(1 as c_int);
                (*frame).b_fdec = b_fdec as uint8_t;
                (*frame).i_pic_struct = PIC_STRUCT_AUTO as c_int;
                (*frame).i_field_cnt = -(1 as c_int) as int64_t;
                (*frame).i_cpb_delay = 0 as int64_t;
                (*frame).i_dpb_output_delay = (*frame).i_cpb_delay;
                (*frame).i_cpb_duration = (*frame).i_dpb_output_delay;
                (*frame).i_duration = (*frame).i_cpb_duration;
                (*frame).i_cpb_delay_lookahead = -(1 as c_int) as int64_t;
                (*frame).i_coded_fields_lookahead = (*frame).i_cpb_delay_lookahead;
                (*frame).orig = frame as *mut x264_frame;
                if i_csp == X264_CSP_NV12 || i_csp == X264_CSP_NV16 {
                    let mut chroma_padv: c_int = i_padv >> (i_csp == X264_CSP_NV12) as c_int;
                    let mut chroma_plane_size: c_int = (*frame).i_stride[1 as c_int as usize]
                        * ((*frame).i_lines[1 as c_int as usize] + 2 as c_int * chroma_padv);
                    (*frame).buffer[1 as c_int as usize] =
                        prealloc_size as intptr_t as *mut c_void as *mut pixel;
                    let fresh9 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh9 as usize] =
                        &mut *(*frame).buffer.as_mut_ptr().offset(1 as c_int as isize)
                            as *mut *mut pixel as *mut *mut uint8_t;
                    prealloc_size += (chroma_plane_size * ::core::mem::size_of::<pixel>() as c_int)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    if (*h).param.b_interlaced != 0 {
                        (*frame).buffer_fld[1 as c_int as usize] =
                            prealloc_size as intptr_t as *mut c_void as *mut pixel;
                        let fresh10 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh10 as usize] =
                            &mut *(*frame).buffer_fld.as_mut_ptr().offset(1 as c_int as isize)
                                as *mut *mut pixel as *mut *mut uint8_t;
                        prealloc_size += (chroma_plane_size
                            * ::core::mem::size_of::<pixel>() as c_int)
                            as int64_t
                            + (64 as c_int - 1 as c_int) as int64_t
                            & !(64 as c_int - 1 as c_int) as int64_t;
                    }
                }
                let mut p: c_int = 0 as c_int;
                while p < luma_plane_count {
                    let mut luma_plane_size: int64_t = align_plane_size(
                        (*frame).i_stride[p as usize]
                            * ((*frame).i_lines[p as usize] + 2 as c_int * i_padv),
                        disalign,
                    ) as int64_t;
                    if (*h).param.analyse.i_subpel_refine != 0 && b_fdec != 0 {
                        luma_plane_size *= 4 as int64_t;
                    }
                    (*frame).buffer[p as usize] =
                        prealloc_size as intptr_t as *mut c_void as *mut pixel;
                    let fresh11 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh11 as usize] =
                        &mut *(*frame).buffer.as_mut_ptr().offset(p as isize) as *mut *mut pixel
                            as *mut *mut uint8_t;
                    prealloc_size += luma_plane_size
                        * ::core::mem::size_of::<pixel>() as c_int as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    if (*h).param.b_interlaced != 0 {
                        (*frame).buffer_fld[p as usize] =
                            prealloc_size as intptr_t as *mut c_void as *mut pixel;
                        let fresh12 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh12 as usize] =
                            &mut *(*frame).buffer_fld.as_mut_ptr().offset(p as isize)
                                as *mut *mut pixel as *mut *mut uint8_t;
                        prealloc_size += luma_plane_size
                            * ::core::mem::size_of::<pixel>() as c_int as int64_t
                            + (64 as c_int - 1 as c_int) as int64_t
                            & !(64 as c_int - 1 as c_int) as int64_t;
                    }
                    p += 1;
                }
                (*frame).b_duplicate = 0 as c_int;
                if b_fdec != 0 {
                    (*frame).mb_type = prealloc_size as intptr_t as *mut c_void as *mut int8_t;
                    let fresh13 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh13 as usize] =
                        &mut (*frame).mb_type as *mut *mut int8_t as *mut *mut uint8_t;
                    prealloc_size += (i_mb_count as usize)
                        .wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    (*frame).mb_partition =
                        prealloc_size as intptr_t as *mut c_void as *mut uint8_t;
                    let fresh14 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh14 as usize] = &mut (*frame).mb_partition as *mut *mut uint8_t;
                    prealloc_size += (i_mb_count as usize)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    (*frame).mv[0 as c_int as usize] =
                        prealloc_size as intptr_t as *mut c_void as *mut [int16_t; 2];
                    let fresh15 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh15 as usize] =
                        &mut *(*frame).mv.as_mut_ptr().offset(0 as c_int as isize)
                            as *mut *mut [int16_t; 2] as *mut *mut uint8_t;
                    prealloc_size += ((2 as c_int * 16 as c_int * i_mb_count) as usize)
                        .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    (*frame).mv16x16 =
                        prealloc_size as intptr_t as *mut c_void as *mut [int16_t; 2];
                    let fresh16 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh16 as usize] =
                        &mut (*frame).mv16x16 as *mut *mut [int16_t; 2] as *mut *mut uint8_t;
                    prealloc_size += ((2 as c_int * (i_mb_count + 1 as c_int)) as usize)
                        .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    (*frame).ref_0[0 as c_int as usize] =
                        prealloc_size as intptr_t as *mut c_void as *mut int8_t;
                    let fresh17 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh17 as usize] =
                        &mut *(*frame).ref_0.as_mut_ptr().offset(0 as c_int as isize)
                            as *mut *mut int8_t as *mut *mut uint8_t;
                    prealloc_size += ((4 as c_int * i_mb_count) as usize)
                        .wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    if (*h).param.i_bframe != 0 {
                        (*frame).mv[1 as c_int as usize] =
                            prealloc_size as intptr_t as *mut c_void as *mut [int16_t; 2];
                        let fresh18 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh18 as usize] =
                            &mut *(*frame).mv.as_mut_ptr().offset(1 as c_int as isize)
                                as *mut *mut [int16_t; 2]
                                as *mut *mut uint8_t;
                        prealloc_size += ((2 as c_int * 16 as c_int * i_mb_count) as usize)
                            .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                            as int64_t
                            + (64 as c_int - 1 as c_int) as int64_t
                            & !(64 as c_int - 1 as c_int) as int64_t;
                        (*frame).ref_0[1 as c_int as usize] =
                            prealloc_size as intptr_t as *mut c_void as *mut int8_t;
                        let fresh19 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh19 as usize] =
                            &mut *(*frame).ref_0.as_mut_ptr().offset(1 as c_int as isize)
                                as *mut *mut int8_t
                                as *mut *mut uint8_t;
                        prealloc_size += ((4 as c_int * i_mb_count) as usize)
                            .wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
                            as int64_t
                            + (64 as c_int - 1 as c_int) as int64_t
                            & !(64 as c_int - 1 as c_int) as int64_t;
                    } else {
                        (*frame).mv[1 as c_int as usize] = 0 as *mut [int16_t; 2];
                        (*frame).ref_0[1 as c_int as usize] = 0 as *mut int8_t;
                    }
                    (*frame).i_row_bits = prealloc_size as intptr_t as *mut c_void as *mut c_int;
                    let fresh20 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh20 as usize] =
                        &mut (*frame).i_row_bits as *mut *mut c_int as *mut *mut uint8_t;
                    prealloc_size += ((i_lines / 16 as c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<c_int>() as usize)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    (*frame).f_row_qp = prealloc_size as intptr_t as *mut c_void as *mut c_float;
                    let fresh21 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh21 as usize] =
                        &mut (*frame).f_row_qp as *mut *mut c_float as *mut *mut uint8_t;
                    prealloc_size += ((i_lines / 16 as c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<c_float>() as usize)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    (*frame).f_row_qscale =
                        prealloc_size as intptr_t as *mut c_void as *mut c_float;
                    let fresh22 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh22 as usize] =
                        &mut (*frame).f_row_qscale as *mut *mut c_float as *mut *mut uint8_t;
                    prealloc_size += ((i_lines / 16 as c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<c_float>() as usize)
                        as int64_t
                        + (64 as c_int - 1 as c_int) as int64_t
                        & !(64 as c_int - 1 as c_int) as int64_t;
                    if (*h).param.analyse.i_me_method >= X264_ME_ESA {
                        (*frame).buffer[3 as c_int as usize] =
                            prealloc_size as intptr_t as *mut c_void as *mut pixel;
                        let fresh23 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh23 as usize] =
                            &mut *(*frame).buffer.as_mut_ptr().offset(3 as c_int as isize)
                                as *mut *mut pixel as *mut *mut uint8_t;
                        prealloc_size += ((((*frame).i_stride[0 as c_int as usize]
                            * ((*frame).i_lines[0 as c_int as usize] + 2 as c_int * i_padv))
                            as usize)
                            .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                            << (*h).frames.b_have_sub8x8_esa)
                            as int64_t
                            + (64 as c_int - 1 as c_int) as int64_t
                            & !(64 as c_int - 1 as c_int) as int64_t;
                    }
                    if (*h).param.b_interlaced != 0 {
                        (*frame).field = prealloc_size as intptr_t as *mut c_void as *mut uint8_t;
                        let fresh24 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh24 as usize] = &mut (*frame).field as *mut *mut uint8_t;
                        prealloc_size +=
                            (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as int64_t
                                + (64 as c_int - 1 as c_int) as int64_t
                                & !(64 as c_int - 1 as c_int) as int64_t;
                    }
                    if (*h).param.analyse.b_mb_info != 0 {
                        (*frame).effective_qp =
                            prealloc_size as intptr_t as *mut c_void as *mut uint8_t;
                        let fresh25 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh25 as usize] =
                            &mut (*frame).effective_qp as *mut *mut uint8_t;
                        prealloc_size +=
                            (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as int64_t
                                + (64 as c_int - 1 as c_int) as int64_t
                                & !(64 as c_int - 1 as c_int) as int64_t;
                    }
                } else {
                    if (*h).frames.b_have_lowres != 0 {
                        let mut luma_plane_size_0: int64_t = align_plane_size(
                            (*frame).i_stride_lowres
                                * ((*frame).i_lines[0 as c_int as usize] / 2 as c_int
                                    + 2 as c_int * PADV),
                            disalign,
                        ) as int64_t;
                        (*frame).buffer_lowres =
                            prealloc_size as intptr_t as *mut c_void as *mut pixel;
                        let fresh26 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh26 as usize] =
                            &mut (*frame).buffer_lowres as *mut *mut pixel as *mut *mut uint8_t;
                        prealloc_size += 4 as int64_t
                            * luma_plane_size_0
                            * ::core::mem::size_of::<pixel>() as c_int as int64_t
                            + (64 as c_int - 1 as c_int) as int64_t
                            & !(64 as c_int - 1 as c_int) as int64_t;
                        let mut j_0: c_int = 0 as c_int;
                        while j_0 <= ((*h).param.i_bframe != 0) as c_int {
                            let mut i_2: c_int = 0 as c_int;
                            while i_2 <= (*h).param.i_bframe {
                                (*frame).lowres_mvs[j_0 as usize][i_2 as usize] =
                                    prealloc_size as intptr_t as *mut c_void as *mut [int16_t; 2];
                                let fresh27 = prealloc_idx;
                                prealloc_idx = prealloc_idx + 1;
                                preallocs[fresh27 as usize] =
                                    &mut *(*(*frame).lowres_mvs.as_mut_ptr().offset(j_0 as isize))
                                        .as_mut_ptr()
                                        .offset(i_2 as isize)
                                        as *mut *mut [int16_t; 2]
                                        as *mut *mut uint8_t;
                                prealloc_size += ((2 as c_int * i_mb_count) as usize)
                                    .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                                    as int64_t
                                    + (64 as c_int - 1 as c_int) as int64_t
                                    & !(64 as c_int - 1 as c_int) as int64_t;
                                (*frame).lowres_mv_costs[j_0 as usize][i_2 as usize] =
                                    prealloc_size as intptr_t as *mut c_void as *mut c_int;
                                let fresh28 = prealloc_idx;
                                prealloc_idx = prealloc_idx + 1;
                                preallocs[fresh28 as usize] = &mut *(*(*frame)
                                    .lowres_mv_costs
                                    .as_mut_ptr()
                                    .offset(j_0 as isize))
                                .as_mut_ptr()
                                .offset(i_2 as isize)
                                    as *mut *mut c_int
                                    as *mut *mut uint8_t;
                                prealloc_size += (i_mb_count as usize)
                                    .wrapping_mul(::core::mem::size_of::<c_int>() as usize)
                                    as int64_t
                                    + (64 as c_int - 1 as c_int) as int64_t
                                    & !(64 as c_int - 1 as c_int) as int64_t;
                                i_2 += 1;
                            }
                            j_0 += 1;
                        }
                        (*frame).i_propagate_cost =
                            prealloc_size as intptr_t as *mut c_void as *mut uint16_t;
                        let fresh29 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh29 as usize] = &mut (*frame).i_propagate_cost
                            as *mut *mut uint16_t
                            as *mut *mut uint8_t;
                        prealloc_size +=
                            (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                                as int64_t
                                + (64 as c_int - 1 as c_int) as int64_t
                                & !(64 as c_int - 1 as c_int) as int64_t;
                        let mut j_1: c_int = 0 as c_int;
                        while j_1 <= (*h).param.i_bframe + 1 as c_int {
                            let mut i_3: c_int = 0 as c_int;
                            while i_3 <= (*h).param.i_bframe + 1 as c_int {
                                (*frame).lowres_costs[j_1 as usize][i_3 as usize] =
                                    prealloc_size as intptr_t as *mut c_void as *mut uint16_t;
                                let fresh30 = prealloc_idx;
                                prealloc_idx = prealloc_idx + 1;
                                preallocs[fresh30 as usize] = &mut *(*(*frame)
                                    .lowres_costs
                                    .as_mut_ptr()
                                    .offset(j_1 as isize))
                                .as_mut_ptr()
                                .offset(i_3 as isize)
                                    as *mut *mut uint16_t
                                    as *mut *mut uint8_t;
                                prealloc_size += (i_mb_count as usize)
                                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                                    as int64_t
                                    + (64 as c_int - 1 as c_int) as int64_t
                                    & !(64 as c_int - 1 as c_int) as int64_t;
                                i_3 += 1;
                            }
                            j_1 += 1;
                        }
                    }
                    if (*h).param.rc.i_aq_mode != 0 {
                        (*frame).f_qp_offset =
                            prealloc_size as intptr_t as *mut c_void as *mut c_float;
                        let fresh31 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh31 as usize] =
                            &mut (*frame).f_qp_offset as *mut *mut c_float as *mut *mut uint8_t;
                        prealloc_size +=
                            (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<c_float>() as usize)
                                as int64_t
                                + (64 as c_int - 1 as c_int) as int64_t
                                & !(64 as c_int - 1 as c_int) as int64_t;
                        (*frame).f_qp_offset_aq =
                            prealloc_size as intptr_t as *mut c_void as *mut c_float;
                        let fresh32 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh32 as usize] =
                            &mut (*frame).f_qp_offset_aq as *mut *mut c_float as *mut *mut uint8_t;
                        prealloc_size +=
                            (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<c_float>() as usize)
                                as int64_t
                                + (64 as c_int - 1 as c_int) as int64_t
                                & !(64 as c_int - 1 as c_int) as int64_t;
                        if (*h).frames.b_have_lowres != 0 {
                            (*frame).i_inv_qscale_factor =
                                prealloc_size as intptr_t as *mut c_void as *mut uint16_t;
                            let fresh33 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[fresh33 as usize] = &mut (*frame).i_inv_qscale_factor
                                as *mut *mut uint16_t
                                as *mut *mut uint8_t;
                            prealloc_size += (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                                as int64_t
                                + (64 as c_int - 1 as c_int) as int64_t
                                & !(64 as c_int - 1 as c_int) as int64_t;
                        }
                    }
                    if (*h).frames.b_have_lowres != 0 {
                        prealloc_size += NATIVE_ALIGN as int64_t;
                    }
                }
                (*frame).base = x264_malloc(prealloc_size) as *mut uint8_t;
                if !(*frame).base.is_null() {
                    loop {
                        let fresh34 = prealloc_idx;
                        prealloc_idx = prealloc_idx - 1;
                        if !(fresh34 != 0) {
                            break;
                        }
                        *preallocs[prealloc_idx as usize] = (*preallocs[prealloc_idx as usize]
                            as intptr_t
                            + (*frame).base as intptr_t)
                            as *mut uint8_t;
                    }
                    if i_csp == X264_CSP_NV12 || i_csp == X264_CSP_NV16 {
                        let mut chroma_padv_0: c_int = i_padv >> (i_csp == X264_CSP_NV12) as c_int;
                        (*frame).plane[1 as c_int as usize] = (*frame).buffer[1 as c_int as usize]
                            .offset(
                                ((*frame).i_stride[1 as c_int as usize] * chroma_padv_0) as isize,
                            )
                            .offset(
                                (if 32 as c_int
                                    > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                {
                                    32 as c_int
                                } else {
                                    64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                }) as isize,
                            );
                        if (*h).param.b_interlaced != 0 {
                            (*frame).plane_fld[1 as c_int as usize] = (*frame).buffer_fld
                                [1 as c_int as usize]
                                .offset(
                                    ((*frame).i_stride[1 as c_int as usize] * chroma_padv_0)
                                        as isize,
                                )
                                .offset(
                                    (if 32 as c_int
                                        > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                    {
                                        32 as c_int
                                    } else {
                                        64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                    }) as isize,
                                );
                        }
                    }
                    let mut p_0: c_int = 0 as c_int;
                    while p_0 < luma_plane_count {
                        let mut luma_plane_size_1: int64_t = align_plane_size(
                            (*frame).i_stride[p_0 as usize]
                                * ((*frame).i_lines[p_0 as usize] + 2 as c_int * i_padv),
                            disalign,
                        ) as int64_t;
                        if (*h).param.analyse.i_subpel_refine != 0 && b_fdec != 0 {
                            let mut i_4: c_int = 0 as c_int;
                            while i_4 < 4 as c_int {
                                (*frame).filtered[p_0 as usize][i_4 as usize] = (*frame).buffer
                                    [p_0 as usize]
                                    .offset((i_4 as int64_t * luma_plane_size_1) as isize)
                                    .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                    .offset(
                                        (if 32 as c_int
                                            > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                        {
                                            32 as c_int
                                        } else {
                                            64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                        }) as isize,
                                    );
                                if (*h).param.b_interlaced != 0 {
                                    (*frame).filtered_fld[p_0 as usize][i_4 as usize] = (*frame)
                                        .buffer_fld[p_0 as usize]
                                        .offset((i_4 as int64_t * luma_plane_size_1) as isize)
                                        .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                        .offset(
                                            (if 32 as c_int
                                                > 64 as c_int
                                                    / ::core::mem::size_of::<pixel>() as c_int
                                            {
                                                32 as c_int
                                            } else {
                                                64 as c_int
                                                    / ::core::mem::size_of::<pixel>() as c_int
                                            }) as isize,
                                        );
                                }
                                i_4 += 1;
                            }
                            (*frame).plane[p_0 as usize] =
                                (*frame).filtered[p_0 as usize][0 as c_int as usize];
                            (*frame).plane_fld[p_0 as usize] =
                                (*frame).filtered_fld[p_0 as usize][0 as c_int as usize];
                        } else {
                            (*frame).plane[p_0 as usize] = (*frame).buffer[p_0 as usize]
                                .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                .offset(
                                    (if 32 as c_int
                                        > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                    {
                                        32 as c_int
                                    } else {
                                        64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                    }) as isize,
                                );
                            (*frame).filtered[p_0 as usize][0 as c_int as usize] =
                                (*frame).plane[p_0 as usize];
                            if (*h).param.b_interlaced != 0 {
                                (*frame).plane_fld[p_0 as usize] = (*frame).buffer_fld
                                    [p_0 as usize]
                                    .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                    .offset(
                                        (if 32 as c_int
                                            > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                        {
                                            32 as c_int
                                        } else {
                                            64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                        }) as isize,
                                    );
                                (*frame).filtered_fld[p_0 as usize][0 as c_int as usize] =
                                    (*frame).plane_fld[p_0 as usize];
                            }
                        }
                        p_0 += 1;
                    }
                    if b_fdec != 0 {
                        (*((*(*frame).mv16x16.offset(0 as c_int as isize)).as_mut_ptr()
                            as *mut x264_union32_t))
                            .i = 0 as uint32_t;
                        (*frame).mv16x16 = (*frame).mv16x16.offset(1);
                        if (*h).param.analyse.i_me_method >= X264_ME_ESA {
                            (*frame).integral = ((*frame).buffer[3 as c_int as usize]
                                as *mut uint16_t)
                                .offset(((*frame).i_stride[0 as c_int as usize] * i_padv) as isize)
                                .offset(
                                    (if 32 as c_int
                                        > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                    {
                                        32 as c_int
                                    } else {
                                        64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                    }) as isize,
                                );
                        }
                    } else if (*h).frames.b_have_lowres != 0 {
                        let mut luma_plane_size_2: int64_t = align_plane_size(
                            (*frame).i_stride_lowres
                                * ((*frame).i_lines[0 as c_int as usize] / 2 as c_int
                                    + 2 as c_int * PADV),
                            disalign,
                        ) as int64_t;
                        let mut i_5: c_int = 0 as c_int;
                        while i_5 < 4 as c_int {
                            (*frame).lowres[i_5 as usize] = (*frame)
                                .buffer_lowres
                                .offset(((*frame).i_stride_lowres * PADV) as isize)
                                .offset(
                                    (if 32 as c_int
                                        > 64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                    {
                                        32 as c_int
                                    } else {
                                        64 as c_int / ::core::mem::size_of::<pixel>() as c_int
                                    }) as isize,
                                )
                                .offset((i_5 as int64_t * luma_plane_size_2) as isize);
                            i_5 += 1;
                        }
                        let mut j_2: c_int = 0 as c_int;
                        while j_2 <= ((*h).param.i_bframe != 0) as c_int {
                            let mut i_6: c_int = 0 as c_int;
                            while i_6 <= (*h).param.i_bframe {
                                memset(
                                    (*frame).lowres_mvs[j_2 as usize][i_6 as usize] as *mut c_void,
                                    0 as c_int,
                                    ((2 as c_int * i_mb_count) as size_t)
                                        .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
                                );
                                i_6 += 1;
                            }
                            j_2 += 1;
                        }
                        (*frame).i_intra_cost = (*frame).lowres_costs[0 as c_int as usize]
                            [0 as c_int as usize]
                            as *mut uint16_t;
                        memset(
                            (*frame).i_intra_cost as *mut c_void,
                            -(1 as c_int),
                            (i_mb_count as size_t)
                                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
                        );
                        if (*h).param.rc.i_aq_mode != 0 {
                            memset(
                                (*frame).i_inv_qscale_factor as *mut c_void,
                                0 as c_int,
                                (i_mb_count as size_t)
                                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
                            );
                        }
                    }
                    if !(pthread_mutex_init(&mut (*frame).mutex, 0 as *const pthread_mutexattr_t)
                        != 0)
                    {
                        if !(pthread_cond_init(&mut (*frame).cv, 0 as *const pthread_condattr_t)
                            != 0)
                        {
                            return frame;
                        }
                    }
                }
            }
        }
    }
    x264_free(frame as *mut c_void);
    return 0 as *mut x264_frame_t;
}
#[no_mangle]
#[c2rust::src_loc = "312:1"]
unsafe extern "C" fn x264_10_frame_delete(mut frame: *mut x264_frame_t) {
    if (*frame).b_duplicate == 0 {
        x264_free((*frame).base as *mut c_void);
        if !(*frame).param.is_null() && (*(*frame).param).param_free.is_some() {
            x264_param_cleanup((*frame).param);
            (*(*frame).param)
                .param_free
                .expect("non-null function pointer")((*frame).param as *mut c_void);
        }
        if (*frame).mb_info_free.is_some() {
            (*frame).mb_info_free.expect("non-null function pointer")(
                (*frame).mb_info as *mut c_void,
            );
        }
        if (*frame).extra_sei.sei_free.is_some() {
            let mut i: c_int = 0 as c_int;
            while i < (*frame).extra_sei.num_payloads {
                (*frame)
                    .extra_sei
                    .sei_free
                    .expect("non-null function pointer")(
                    (*(*frame).extra_sei.payloads.offset(i as isize)).payload as *mut c_void,
                );
                i += 1;
            }
            (*frame)
                .extra_sei
                .sei_free
                .expect("non-null function pointer")(
                (*frame).extra_sei.payloads as *mut c_void
            );
        }
        pthread_mutex_destroy(&mut (*frame).mutex);
        pthread_cond_destroy(&mut (*frame).cv);
    }
    x264_free(frame as *mut c_void);
}
#[c2rust::src_loc = "342:1"]
unsafe extern "C" fn get_plane_ptr(
    mut h: *mut x264_t,
    mut src: *mut x264_picture_t,
    mut pix: *mut *mut uint8_t,
    mut stride: *mut c_int,
    mut plane: c_int,
    mut xshift: c_int,
    mut yshift: c_int,
) -> c_int {
    let mut width: c_int = (*h).param.i_width >> xshift;
    let mut height: c_int = (*h).param.i_height >> yshift;
    *pix = (*src).img.plane[plane as usize];
    *stride = (*src).img.i_stride[plane as usize];
    if (*src).img.i_csp & X264_CSP_VFLIP != 0 {
        *pix = (*pix).offset(((height - 1 as c_int) * *stride) as isize);
        *stride = -*stride;
    }
    if width > abs(*stride) {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"Input picture width (%d) is greater than stride (%d)\n\0" as *const u8
                as *const c_char,
            width,
            *stride,
        );
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "363:1"]
unsafe extern "C" fn x264_10_frame_copy_picture(
    mut h: *mut x264_t,
    mut dst: *mut x264_frame_t,
    mut src: *mut x264_picture_t,
) -> c_int {
    let mut i_csp: c_int = (*src).img.i_csp & X264_CSP_MASK;
    if (*dst).i_csp != frame_internal_csp(i_csp) {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"Invalid input colorspace\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if (*src).img.i_csp & X264_CSP_HIGH_DEPTH == 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"This build of x264 requires high depth input. Rebuild to support 8-bit input.\n\0"
                as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if BIT_DEPTH != 10 as c_int && i_csp == X264_CSP_V210 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"v210 input is only compatible with bit-depth of 10 bits\n\0" as *const u8
                as *const c_char,
        );
        return -(1 as c_int);
    }
    if (*src).i_type < X264_TYPE_AUTO || (*src).i_type > X264_TYPE_KEYFRAME {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"forced frame type (%d) at %d is unknown\n\0" as *const u8 as *const c_char,
            (*src).i_type,
            (*h).frames.i_input,
        );
        (*dst).i_forced_type = X264_TYPE_AUTO;
    } else {
        (*dst).i_forced_type = (*src).i_type;
    }
    (*dst).i_type = (*dst).i_forced_type;
    (*dst).i_qpplus1 = (*src).i_qpplus1;
    (*dst).i_reordered_pts = (*src).i_pts;
    (*dst).i_pts = (*dst).i_reordered_pts;
    (*dst).param = (*src).param;
    (*dst).i_pic_struct = (*src).i_pic_struct;
    (*dst).extra_sei = (*src).extra_sei;
    (*dst).opaque = (*src).opaque;
    (*dst).mb_info = if (*h).param.analyse.b_mb_info != 0 {
        (*src).prop.mb_info
    } else {
        0 as *mut uint8_t
    };
    (*dst).mb_info_free = if (*h).param.analyse.b_mb_info != 0 {
        (*src).prop.mb_info_free
    } else {
        None
    };
    let mut pix: [*mut uint8_t; 3] = [0 as *mut uint8_t; 3];
    let mut stride: [c_int; 3] = [0; 3];
    if i_csp == X264_CSP_YUYV || i_csp == X264_CSP_UYVY {
        let mut p: c_int = (i_csp == X264_CSP_UYVY) as c_int;
        (*h).mc
            .plane_copy_deinterleave_yuyv
            .expect("non-null function pointer")(
            (*dst).plane[p as usize],
            (*dst).i_stride[p as usize] as intptr_t,
            (*dst).plane[(p ^ 1 as c_int) as usize],
            (*dst).i_stride[(p ^ 1 as c_int) as usize] as intptr_t,
            (*src).img.plane[0 as c_int as usize] as *mut pixel,
            ((*src).img.i_stride[0 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else if i_csp == X264_CSP_V210 {
        stride[0 as c_int as usize] = (*src).img.i_stride[0 as c_int as usize];
        pix[0 as c_int as usize] = (*src).img.plane[0 as c_int as usize];
        (*h).mc
            .plane_copy_deinterleave_v210
            .expect("non-null function pointer")(
            (*dst).plane[0 as c_int as usize],
            (*dst).i_stride[0 as c_int as usize] as intptr_t,
            (*dst).plane[1 as c_int as usize],
            (*dst).i_stride[1 as c_int as usize] as intptr_t,
            pix[0 as c_int as usize] as *mut uint32_t,
            (stride[0 as c_int as usize] / ::core::mem::size_of::<uint32_t>() as c_int) as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else if i_csp >= X264_CSP_BGR {
        stride[0 as c_int as usize] = (*src).img.i_stride[0 as c_int as usize];
        pix[0 as c_int as usize] = (*src).img.plane[0 as c_int as usize];
        if (*src).img.i_csp & X264_CSP_VFLIP != 0 {
            pix[0 as c_int as usize] = pix[0 as c_int as usize].offset(
                (((*h).param.i_height - 1 as c_int) * stride[0 as c_int as usize]) as isize,
            );
            stride[0 as c_int as usize] = -stride[0 as c_int as usize];
        }
        let mut b: c_int = (i_csp == X264_CSP_RGB) as c_int;
        (*h).mc
            .plane_copy_deinterleave_rgb
            .expect("non-null function pointer")(
            (*dst).plane[(1 as c_int + b) as usize],
            (*dst).i_stride[(1 as c_int + b) as usize] as intptr_t,
            (*dst).plane[0 as c_int as usize],
            (*dst).i_stride[0 as c_int as usize] as intptr_t,
            (*dst).plane[(2 as c_int - b) as usize],
            (*dst).i_stride[(2 as c_int - b) as usize] as intptr_t,
            pix[0 as c_int as usize] as *mut pixel,
            (stride[0 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
            if i_csp == X264_CSP_BGRA {
                4 as c_int
            } else {
                3 as c_int
            },
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else {
        let mut v_shift: c_int = (*h).mb.chroma_v_shift;
        if get_plane_ptr(
            h,
            src,
            &mut *pix.as_mut_ptr().offset(0 as c_int as isize),
            &mut *stride.as_mut_ptr().offset(0 as c_int as isize),
            0 as c_int,
            0 as c_int,
            0 as c_int,
        ) < 0 as c_int
        {
            return -(1 as c_int);
        }
        (*h).mc.plane_copy.expect("non-null function pointer")(
            (*dst).plane[0 as c_int as usize],
            (*dst).i_stride[0 as c_int as usize] as intptr_t,
            pix[0 as c_int as usize] as *mut pixel,
            (stride[0 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
        if i_csp == X264_CSP_NV12 || i_csp == X264_CSP_NV16 {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as c_int as isize),
                1 as c_int,
                0 as c_int,
                v_shift,
            ) < 0 as c_int
            {
                return -(1 as c_int);
            }
            (*h).mc.plane_copy.expect("non-null function pointer")(
                (*dst).plane[1 as c_int as usize],
                (*dst).i_stride[1 as c_int as usize] as intptr_t,
                pix[1 as c_int as usize] as *mut pixel,
                (stride[1 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == X264_CSP_NV21 {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as c_int as isize),
                1 as c_int,
                0 as c_int,
                v_shift,
            ) < 0 as c_int
            {
                return -(1 as c_int);
            }
            (*h).mc.plane_copy_swap.expect("non-null function pointer")(
                (*dst).plane[1 as c_int as usize],
                (*dst).i_stride[1 as c_int as usize] as intptr_t,
                pix[1 as c_int as usize] as *mut pixel,
                (stride[1 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width >> 1 as c_int,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == X264_CSP_I420
            || i_csp == X264_CSP_I422
            || i_csp == X264_CSP_YV12
            || i_csp == X264_CSP_YV16
        {
            let mut uv_swap: c_int = (i_csp == X264_CSP_YV12 || i_csp == X264_CSP_YV16) as c_int;
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as c_int as isize),
                if uv_swap != 0 { 2 as c_int } else { 1 as c_int },
                1 as c_int,
                v_shift,
            ) < 0 as c_int
            {
                return -(1 as c_int);
            }
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(2 as c_int as isize),
                &mut *stride.as_mut_ptr().offset(2 as c_int as isize),
                if uv_swap != 0 { 1 as c_int } else { 2 as c_int },
                1 as c_int,
                v_shift,
            ) < 0 as c_int
            {
                return -(1 as c_int);
            }
            (*h).mc
                .plane_copy_interleave
                .expect("non-null function pointer")(
                (*dst).plane[1 as c_int as usize],
                (*dst).i_stride[1 as c_int as usize] as intptr_t,
                pix[1 as c_int as usize] as *mut pixel,
                (stride[1 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                pix[2 as c_int as usize] as *mut pixel,
                (stride[2 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width >> 1 as c_int,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == X264_CSP_I444 || i_csp == X264_CSP_YV24 {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as c_int as isize),
                if i_csp == 0xc as c_int {
                    1 as c_int
                } else {
                    2 as c_int
                },
                0 as c_int,
                0 as c_int,
            ) < 0 as c_int
            {
                return -(1 as c_int);
            }
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(2 as c_int as isize),
                &mut *stride.as_mut_ptr().offset(2 as c_int as isize),
                if i_csp == 0xc as c_int {
                    2 as c_int
                } else {
                    1 as c_int
                },
                0 as c_int,
                0 as c_int,
            ) < 0 as c_int
            {
                return -(1 as c_int);
            }
            (*h).mc.plane_copy.expect("non-null function pointer")(
                (*dst).plane[1 as c_int as usize],
                (*dst).i_stride[1 as c_int as usize] as intptr_t,
                pix[1 as c_int as usize] as *mut pixel,
                (stride[1 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
            (*h).mc.plane_copy.expect("non-null function pointer")(
                (*dst).plane[2 as c_int as usize],
                (*dst).i_stride[2 as c_int as usize] as intptr_t,
                pix[2 as c_int as usize] as *mut pixel,
                (stride[2 as c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
        }
    }
    return 0 as c_int;
}
#[inline(always)]
#[c2rust::src_loc = "483:1"]
unsafe extern "C" fn pixel_memset(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut len: c_int,
    mut size: c_int,
) {
    let mut dstp: *mut uint8_t = dst as *mut uint8_t;
    let mut v1: uint32_t = *src as uint32_t;
    let mut v2: uint32_t = if size == 1 as c_int {
        v1.wrapping_add(v1 << 8 as c_int)
    } else {
        (*(src as *mut x264_union16_t)).i as uint32_t
    };
    let mut v4: uint32_t = if size <= 2 as c_int {
        v2.wrapping_add(v2 << 16 as c_int)
    } else {
        (*(src as *mut x264_union32_t)).i
    };
    let mut i: c_int = 0 as c_int;
    len *= size;
    if dstp as intptr_t as uint64_t & WORD_SIZE.wrapping_sub(1 as uint64_t) != 0 {
        if size <= 2 as c_int && dstp as intptr_t & 3 as intptr_t != 0 {
            if size == 1 as c_int && dstp as intptr_t & 1 as intptr_t != 0 {
                let fresh0 = i;
                i = i + 1;
                *dstp.offset(fresh0 as isize) = v1 as uint8_t;
            }
            if dstp as intptr_t & 2 as intptr_t != 0 {
                (*(dstp.offset(i as isize) as *mut x264_union16_t)).i = v2 as uint16_t;
                i += 2 as c_int;
            }
        }
        if WORD_SIZE == 8 as uint64_t && dstp as intptr_t & 4 as intptr_t != 0 {
            (*(dstp.offset(i as isize) as *mut x264_union32_t)).i = v4;
            i += 4 as c_int;
        }
    }
    if WORD_SIZE == 8 as uint64_t {
        let mut v8: uint64_t = (v4 as uint64_t).wrapping_add((v4 as uint64_t) << 32 as c_int);
        while i < len - 7 as c_int {
            (*(dstp.offset(i as isize) as *mut x264_union64_t)).i = v8;
            i += 8 as c_int;
        }
    }
    while i < len - 3 as c_int {
        (*(dstp.offset(i as isize) as *mut x264_union32_t)).i = v4;
        i += 4 as c_int;
    }
    if size <= 2 as c_int {
        if i < len - 1 as c_int {
            (*(dstp.offset(i as isize) as *mut x264_union16_t)).i = v2 as uint16_t;
            i += 2 as c_int;
        }
        if size == 1 as c_int && i != len {
            *dstp.offset(i as isize) = v1 as uint8_t;
        }
    }
}
#[inline(always)]
#[c2rust::src_loc = "535:1"]
unsafe extern "C" fn plane_expand_border(
    mut pix: *mut pixel,
    mut i_stride: c_int,
    mut i_width: c_int,
    mut i_height: c_int,
    mut i_padh: c_int,
    mut i_padv: c_int,
    mut b_pad_top: c_int,
    mut b_pad_bottom: c_int,
    mut b_chroma: c_int,
) {
    let mut y: c_int = 0 as c_int;
    while y < i_height {
        pixel_memset(
            pix.offset(-i_padh as isize).offset((y * i_stride) as isize),
            pix.offset(0 as c_int as isize)
                .offset((y * i_stride) as isize),
            i_padh >> b_chroma,
            SIZEOF_PIXEL << b_chroma,
        );
        pixel_memset(
            pix.offset(i_width as isize).offset((y * i_stride) as isize),
            pix.offset((i_width - 1 as c_int - b_chroma) as isize)
                .offset((y * i_stride) as isize),
            i_padh >> b_chroma,
            SIZEOF_PIXEL << b_chroma,
        );
        y += 1;
    }
    if b_pad_top != 0 {
        let mut y_0: c_int = 0 as c_int;
        while y_0 < i_padv {
            memcpy(
                pix.offset(-i_padh as isize)
                    .offset(((-y_0 - 1 as c_int) * i_stride) as isize)
                    as *mut c_void,
                pix.offset(-i_padh as isize)
                    .offset((0 as c_int * i_stride) as isize) as *const c_void,
                ((i_width + 2 as c_int * i_padh) * SIZEOF_PIXEL) as size_t,
            );
            y_0 += 1;
        }
    }
    if b_pad_bottom != 0 {
        let mut y_1: c_int = 0 as c_int;
        while y_1 < i_padv {
            memcpy(
                pix.offset(-i_padh as isize)
                    .offset(((i_height + y_1) * i_stride) as isize) as *mut c_void,
                pix.offset(-i_padh as isize)
                    .offset(((i_height - 1 as c_int) * i_stride) as isize)
                    as *const c_void,
                ((i_width + 2 as c_int * i_padh) * SIZEOF_PIXEL) as size_t,
            );
            y_1 += 1;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "556:1"]
unsafe extern "C" fn x264_10_frame_expand_border(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: c_int,
) {
    let mut pad_top: c_int = (mb_y == 0 as c_int) as c_int;
    let mut pad_bot: c_int =
        (mb_y == (*h).mb.i_mb_height - ((1 as c_int) << (*h).sh.b_mbaff)) as c_int;
    let mut b_start: c_int = (mb_y == (*h).i_threadslice_start) as c_int;
    let mut b_end: c_int =
        (mb_y == (*h).i_threadslice_end - ((1 as c_int) << (*h).sh.b_mbaff)) as c_int;
    if mb_y & (*h).sh.b_mbaff != 0 {
        return;
    }
    let mut i: c_int = 0 as c_int;
    while i < (*frame).i_plane {
        let mut h_shift: c_int = (i != 0 && (*h).mb.chroma_h_shift != 0) as c_int;
        let mut v_shift: c_int = (i != 0 && (*h).mb.chroma_v_shift != 0) as c_int;
        let mut stride: c_int = (*frame).i_stride[i as usize];
        let mut width: c_int = 16 as c_int * (*h).mb.i_mb_width;
        let mut height: c_int = (if pad_bot != 0 {
            16 as c_int * ((*h).mb.i_mb_height - mb_y) >> (*h).sh.b_mbaff
        } else {
            16 as c_int
        }) >> v_shift;
        let mut padh: c_int = PADH;
        let mut padv: c_int = PADV >> v_shift;
        if b_end != 0 && b_start == 0 {
            height += 4 as c_int >> v_shift + (*h).sh.b_mbaff;
        }
        let mut pix: *mut pixel = 0 as *mut pixel;
        let mut starty: c_int = 16 as c_int * mb_y - 4 as c_int * (b_start == 0) as c_int;
        if (*h).sh.b_mbaff != 0 {
            pix = (*frame).plane_fld[i as usize].offset((starty * stride >> v_shift) as isize);
            plane_expand_border(
                pix,
                stride * 2 as c_int,
                width,
                height,
                padh,
                padv,
                pad_top,
                pad_bot,
                h_shift,
            );
            plane_expand_border(
                pix.offset(stride as isize),
                stride * 2 as c_int,
                width,
                height,
                padh,
                padv,
                pad_top,
                pad_bot,
                h_shift,
            );
            height = (if pad_bot != 0 {
                16 as c_int * ((*h).mb.i_mb_height - mb_y)
            } else {
                32 as c_int
            }) >> v_shift;
            if b_end != 0 && b_start == 0 {
                height += 4 as c_int >> v_shift;
            }
            pix = (*frame).plane[i as usize].offset((starty * stride >> v_shift) as isize);
            plane_expand_border(
                pix, stride, width, height, padh, padv, pad_top, pad_bot, h_shift,
            );
        } else {
            pix = (*frame).plane[i as usize].offset((starty * stride >> v_shift) as isize);
            plane_expand_border(
                pix, stride, width, height, padh, padv, pad_top, pad_bot, h_shift,
            );
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "599:1"]
unsafe extern "C" fn x264_10_frame_expand_border_filtered(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: c_int,
    mut b_end: c_int,
) {
    let mut b_start: c_int = (mb_y == 0) as c_int;
    let mut width: c_int = 16 as c_int * (*h).mb.i_mb_width + 8 as c_int;
    let mut height: c_int = if b_end != 0 {
        (16 as c_int * ((*h).mb.i_mb_height - mb_y) >> (*h).sh.b_mbaff) + 16 as c_int
    } else {
        16 as c_int
    };
    let mut padh: c_int = PADH - 4 as c_int;
    let mut padv: c_int = PADV - 8 as c_int;
    let mut p: c_int = 0 as c_int;
    while p
        < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            3 as c_int
        } else {
            1 as c_int
        })
    {
        let mut i: c_int = 1 as c_int;
        while i < 4 as c_int {
            let mut stride: c_int = (*frame).i_stride[p as usize];
            let mut pix: *mut pixel = 0 as *mut pixel;
            if (*h).sh.b_mbaff != 0 {
                pix = (*frame).filtered_fld[p as usize][i as usize]
                    .offset(((16 as c_int * mb_y - 16 as c_int) * stride) as isize)
                    .offset(-(4 as c_int as isize));
                plane_expand_border(
                    pix,
                    stride * 2 as c_int,
                    width,
                    height,
                    padh,
                    padv,
                    b_start,
                    b_end,
                    0 as c_int,
                );
                plane_expand_border(
                    pix.offset(stride as isize),
                    stride * 2 as c_int,
                    width,
                    height,
                    padh,
                    padv,
                    b_start,
                    b_end,
                    0 as c_int,
                );
            }
            pix = (*frame).filtered[p as usize][i as usize]
                .offset(((16 as c_int * mb_y - 8 as c_int) * stride) as isize)
                .offset(-(4 as c_int as isize));
            plane_expand_border(
                pix,
                stride,
                width,
                height << (*h).sh.b_mbaff,
                padh,
                padv,
                b_start,
                b_end,
                0 as c_int,
            );
            i += 1;
        }
        p += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "627:1"]
unsafe extern "C" fn x264_10_frame_expand_border_lowres(mut frame: *mut x264_frame_t) {
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        plane_expand_border(
            (*frame).lowres[i as usize],
            (*frame).i_stride_lowres,
            (*frame).i_width_lowres,
            (*frame).i_lines_lowres,
            PADH,
            PADV,
            1 as c_int,
            1 as c_int,
            0 as c_int,
        );
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "633:1"]
unsafe extern "C" fn x264_10_frame_expand_border_chroma(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut plane: c_int,
) {
    let mut v_shift: c_int = (*h).mb.chroma_v_shift;
    plane_expand_border(
        (*frame).plane[plane as usize],
        (*frame).i_stride[plane as usize],
        16 as c_int * (*h).mb.i_mb_width,
        16 as c_int * (*h).mb.i_mb_height >> v_shift,
        PADH,
        PADV >> v_shift,
        1 as c_int,
        1 as c_int,
        (*h).mb.chroma_h_shift,
    );
}
#[no_mangle]
#[c2rust::src_loc = "640:1"]
unsafe extern "C" fn x264_10_frame_expand_border_mod16(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*frame).i_plane {
        let mut i_width: c_int = (*h).param.i_width;
        let mut h_shift: c_int = (i != 0 && (*h).mb.chroma_h_shift != 0) as c_int;
        let mut v_shift: c_int = (i != 0 && (*h).mb.chroma_v_shift != 0) as c_int;
        let mut i_height: c_int = (*h).param.i_height >> v_shift;
        let mut i_padx: c_int = (*h).mb.i_mb_width * 16 as c_int - (*h).param.i_width;
        let mut i_pady: c_int = (*h).mb.i_mb_height * 16 as c_int - (*h).param.i_height >> v_shift;
        if i_padx != 0 {
            let mut y: c_int = 0 as c_int;
            while y < i_height {
                pixel_memset(
                    &mut *(*(*frame).plane.as_mut_ptr().offset(i as isize)).offset(
                        (y * *(*frame).i_stride.as_mut_ptr().offset(i as isize) + i_width) as isize,
                    ),
                    &mut *(*(*frame).plane.as_mut_ptr().offset(i as isize)).offset(
                        (y * *(*frame).i_stride.as_mut_ptr().offset(i as isize) + i_width
                            - 1 as c_int
                            - h_shift) as isize,
                    ),
                    i_padx >> h_shift,
                    SIZEOF_PIXEL << h_shift,
                );
                y += 1;
            }
        }
        if i_pady != 0 {
            let mut y_0: c_int = i_height;
            while y_0 < i_height + i_pady {
                memcpy(
                    &mut *(*(*frame).plane.as_mut_ptr().offset(i as isize))
                        .offset((y_0 * *(*frame).i_stride.as_mut_ptr().offset(i as isize)) as isize)
                        as *mut pixel as *mut c_void,
                    &mut *(*(*frame).plane.as_mut_ptr().offset(i as isize)).offset(
                        ((i_height - (!y_0 & (*h).param.b_interlaced) - 1 as c_int)
                            * *(*frame).i_stride.as_mut_ptr().offset(i as isize))
                            as isize,
                    ) as *mut pixel as *const c_void,
                    ((i_width + i_padx) * SIZEOF_PIXEL) as size_t,
                );
                y_0 += 1;
            }
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "668:1"]
unsafe extern "C" fn x264_10_expand_border_mbpair(
    mut h: *mut x264_t,
    mut mb_x: c_int,
    mut _mb_y: c_int,
) {
    let mut i: c_int = 0 as c_int;
    while i < (*(*h).fenc).i_plane {
        let mut v_shift: c_int = (i != 0 && (*h).mb.chroma_v_shift != 0) as c_int;
        let mut stride: c_int = (*(*h).fenc).i_stride[i as usize];
        let mut height: c_int = (*h).param.i_height >> v_shift;
        let mut pady: c_int = (*h).mb.i_mb_height * 16 as c_int - (*h).param.i_height >> v_shift;
        let mut fenc: *mut pixel =
            (*(*h).fenc).plane[i as usize].offset((16 as c_int * mb_x) as isize);
        let mut y: c_int = height;
        while y < height + pady {
            memcpy(
                fenc.offset((y * stride) as isize) as *mut c_void,
                fenc.offset(((height - 1 as c_int) * stride) as isize) as *const c_void,
                (16 as c_int * SIZEOF_PIXEL) as size_t,
            );
            y += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "683:1"]
unsafe extern "C" fn x264_10_frame_cond_broadcast(
    mut frame: *mut x264_frame_t,
    mut i_lines_completed: c_int,
) {
    pthread_mutex_lock(&mut (*frame).mutex);
    (*frame).i_lines_completed = i_lines_completed;
    pthread_cond_broadcast(&mut (*frame).cv);
    pthread_mutex_unlock(&mut (*frame).mutex);
}
#[no_mangle]
#[c2rust::src_loc = "691:1"]
unsafe extern "C" fn x264_10_frame_cond_wait(
    mut frame: *mut x264_frame_t,
    mut i_lines_completed: c_int,
) -> c_int {
    let mut completed: c_int = 0;
    pthread_mutex_lock(&mut (*frame).mutex);
    loop {
        completed = (*frame).i_lines_completed;
        if !(completed < i_lines_completed && i_lines_completed >= 0 as c_int) {
            break;
        }
        pthread_cond_wait(&mut (*frame).cv, &mut (*frame).mutex);
    }
    pthread_mutex_unlock(&mut (*frame).mutex);
    return completed;
}
#[no_mangle]
#[c2rust::src_loc = "701:1"]
unsafe extern "C" fn x264_10_threadslice_cond_broadcast(mut h: *mut x264_t, mut pass: c_int) {
    pthread_mutex_lock(&mut (*h).mutex);
    (*h).i_threadslice_pass = pass;
    if pass > 0 as c_int {
        pthread_cond_broadcast(&mut (*h).cv);
    }
    pthread_mutex_unlock(&mut (*h).mutex);
}
#[no_mangle]
#[c2rust::src_loc = "710:1"]
unsafe extern "C" fn x264_10_threadslice_cond_wait(mut h: *mut x264_t, mut pass: c_int) {
    pthread_mutex_lock(&mut (*h).mutex);
    while (*h).i_threadslice_pass < pass {
        pthread_cond_wait(&mut (*h).cv, &mut (*h).mutex);
    }
    pthread_mutex_unlock(&mut (*h).mutex);
}
#[no_mangle]
#[c2rust::src_loc = "718:1"]
unsafe extern "C" fn x264_10_frame_new_slice(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) -> c_int {
    if (*h).param.i_slice_count_max != 0 {
        let mut slice_count: c_int = 0;
        if (*h).param.b_sliced_threads != 0 {
            slice_count = x264_pthread_fetch_and_add(
                &mut (*frame).i_slice_count,
                1 as c_int,
                &mut (*frame).mutex,
            );
        } else {
            let fresh1 = (*frame).i_slice_count;
            (*frame).i_slice_count = (*frame).i_slice_count + 1;
            slice_count = fresh1;
        }
        if slice_count >= (*h).param.i_slice_count_max {
            return -(1 as c_int);
        }
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "735:1"]
unsafe extern "C" fn x264_10_frame_push(
    mut list: *mut *mut x264_frame_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: c_int = 0 as c_int;
    while !(*list.offset(i as isize)).is_null() {
        i += 1;
    }
    let ref mut fresh2 = *list.offset(i as isize);
    *fresh2 = frame;
}
#[no_mangle]
#[c2rust::src_loc = "742:1"]
unsafe extern "C" fn x264_10_frame_pop(mut list: *mut *mut x264_frame_t) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    let mut i: c_int = 0 as c_int;
    if !(*list.offset(0 as c_int as isize)).is_null() {
    } else {
        __assert_fail(
            b"list[0]\0" as *const u8 as *const c_char,
            b"common/frame.c\0" as *const u8 as *const c_char,
            746 as c_uint,
            ::core::mem::transmute::<[u8; 49], [c_char; 49]>(
                *b"x264_frame_t *x264_10_frame_pop(x264_frame_t **)\0",
            )
            .as_ptr(),
        );
    }
    while !(*list.offset((i + 1 as c_int) as isize)).is_null() {
        i += 1;
    }
    frame = *list.offset(i as isize);
    let ref mut fresh3 = *list.offset(i as isize);
    *fresh3 = 0 as *mut x264_frame_t;
    return frame;
}
#[no_mangle]
#[c2rust::src_loc = "753:1"]
unsafe extern "C" fn x264_10_frame_unshift(
    mut list: *mut *mut x264_frame_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: c_int = 0 as c_int;
    while !(*list.offset(i as isize)).is_null() {
        i += 1;
    }
    loop {
        let fresh4 = i;
        i = i - 1;
        if !(fresh4 != 0) {
            break;
        }
        let ref mut fresh5 = *list.offset((i + 1 as c_int) as isize);
        *fresh5 = *list.offset(i as isize);
    }
    let ref mut fresh6 = *list.offset(0 as c_int as isize);
    *fresh6 = frame;
}
#[no_mangle]
#[c2rust::src_loc = "762:1"]
unsafe extern "C" fn x264_10_frame_shift(mut list: *mut *mut x264_frame_t) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = *list.offset(0 as c_int as isize);
    let mut i: c_int = 0;
    i = 0 as c_int;
    while !(*list.offset(i as isize)).is_null() {
        let ref mut fresh7 = *list.offset(i as isize);
        *fresh7 = *list.offset((i + 1 as c_int) as isize);
        i += 1;
    }
    if !frame.is_null() {
    } else {
        __assert_fail(
            b"frame\0" as *const u8 as *const c_char,
            b"common/frame.c\0" as *const u8 as *const c_char,
            768 as c_uint,
            ::core::mem::transmute::<[u8; 51], [c_char; 51]>(
                *b"x264_frame_t *x264_10_frame_shift(x264_frame_t **)\0",
            )
            .as_ptr(),
        );
    }
    return frame;
}
#[no_mangle]
#[c2rust::src_loc = "772:1"]
unsafe extern "C" fn x264_10_frame_push_unused(mut h: *mut x264_t, mut frame: *mut x264_frame_t) {
    if (*frame).i_reference_count > 0 as c_int {
    } else {
        __assert_fail(
            b"frame->i_reference_count > 0\0" as *const u8 as *const c_char,
            b"common/frame.c\0" as *const u8 as *const c_char,
            774 as c_uint,
            ::core::mem::transmute::<[u8; 57], [c_char; 57]>(
                *b"void x264_10_frame_push_unused(x264_t *, x264_frame_t *)\0",
            )
            .as_ptr(),
        );
    }
    (*frame).i_reference_count -= 1;
    if (*frame).i_reference_count == 0 as c_int {
        x264_10_frame_push((*h).frames.unused[(*frame).b_fdec as usize], frame);
    }
}
#[no_mangle]
#[c2rust::src_loc = "780:1"]
unsafe extern "C" fn x264_10_frame_pop_unused(
    mut h: *mut x264_t,
    mut b_fdec: c_int,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    if !(*(*h).frames.unused[b_fdec as usize].offset(0 as c_int as isize)).is_null() {
        frame = x264_10_frame_pop((*h).frames.unused[b_fdec as usize]);
    } else {
        frame = frame_new(h, b_fdec);
    }
    if frame.is_null() {
        return 0 as *mut x264_frame_t;
    }
    (*frame).b_last_minigop_bframe = 0 as uint8_t;
    (*frame).i_reference_count = 1 as c_int;
    (*frame).b_intra_calculated = 0 as c_int;
    (*frame).b_scenecut = 1 as c_int;
    (*frame).b_keyframe = 0 as c_int;
    (*frame).b_corrupt = 0 as c_int;
    (*frame).i_slice_count = if (*h).param.b_sliced_threads != 0 {
        (*h).param.i_threads
    } else {
        1 as c_int
    };
    memset(
        (*frame).weight.as_mut_ptr() as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[[x264_weight_t; 3]; 16]>() as size_t,
    );
    memset(
        (*frame).f_weighted_cost_delta.as_mut_ptr() as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[c_float; 18]>() as size_t,
    );
    return frame;
}
#[no_mangle]
#[c2rust::src_loc = "803:1"]
unsafe extern "C" fn x264_10_frame_push_blank_unused(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    if (*frame).i_reference_count > 0 as c_int {
    } else {
        __assert_fail(
            b"frame->i_reference_count > 0\0" as *const u8 as *const c_char,
            b"common/frame.c\0" as *const u8 as *const c_char,
            805 as c_uint,
            ::core::mem::transmute::<[u8; 63], [c_char; 63]>(
                *b"void x264_10_frame_push_blank_unused(x264_t *, x264_frame_t *)\0",
            )
            .as_ptr(),
        );
    }
    (*frame).i_reference_count -= 1;
    if (*frame).i_reference_count == 0 as c_int {
        x264_10_frame_push((*h).frames.blank_unused, frame);
    }
}
#[no_mangle]
#[c2rust::src_loc = "811:1"]
unsafe extern "C" fn x264_10_frame_pop_blank_unused(mut h: *mut x264_t) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    if !(*(*h).frames.blank_unused.offset(0 as c_int as isize)).is_null() {
        frame = x264_10_frame_pop((*h).frames.blank_unused);
    } else {
        frame = x264_malloc(::core::mem::size_of::<x264_frame_t>() as int64_t) as *mut x264_frame_t;
    }
    if frame.is_null() {
        return 0 as *mut x264_frame_t;
    }
    (*frame).b_duplicate = 1 as c_int;
    (*frame).i_reference_count = 1 as c_int;
    return frame;
}
#[no_mangle]
#[c2rust::src_loc = "825:1"]
unsafe extern "C" fn x264_10_weight_scale_plane(
    mut _h: *mut x264_t,
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut i_width: c_int,
    mut i_height: c_int,
    mut w: *mut x264_weight_t,
) {
    while i_height > 0 as c_int {
        let mut x: c_int = 0;
        x = 0 as c_int;
        while x < i_width - 8 as c_int {
            (*(*w).weightfn.offset((16 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                dst.offset(x as isize),
                i_dst_stride,
                src.offset(x as isize),
                i_src_stride,
                w,
                if i_height < 16 as c_int {
                    i_height
                } else {
                    16 as c_int
                },
            );
            x += 16 as c_int;
        }
        if x < i_width {
            (*(*w).weightfn.offset((8 as c_int >> 2 as c_int) as isize))
                .expect("non-null function pointer")(
                dst.offset(x as isize),
                i_dst_stride,
                src.offset(x as isize),
                i_src_stride,
                w,
                if i_height < 16 as c_int {
                    i_height
                } else {
                    16 as c_int
                },
            );
        }
        i_height -= 16 as c_int;
        dst = dst.offset((16 as intptr_t * i_dst_stride) as isize);
        src = src.offset((16 as intptr_t * i_src_stride) as isize);
    }
}
#[no_mangle]
#[c2rust::src_loc = "843:1"]
unsafe extern "C" fn x264_10_frame_delete_list(mut list: *mut *mut x264_frame_t) {
    let mut i: c_int = 0 as c_int;
    if list.is_null() {
        return;
    }
    while !(*list.offset(i as isize)).is_null() {
        let fresh35 = i;
        i = i + 1;
        x264_10_frame_delete(*list.offset(fresh35 as isize));
    }
    x264_free(list as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "853:1"]
unsafe extern "C" fn x264_10_sync_frame_list_init(
    mut slist: *mut x264_sync_frame_list_t,
    mut max_size: c_int,
) -> c_int {
    if max_size < 0 as c_int {
        return -(1 as c_int);
    }
    (*slist).i_max_size = max_size;
    (*slist).i_size = 0 as c_int;
    (*slist).list = x264_malloc(
        ((max_size + 1 as c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<*mut x264_frame_t>() as usize) as int64_t,
    ) as *mut *mut x264_frame_t;
    if (*slist).list.is_null() {
        return -(1 as c_int);
    } else {
        memset(
            (*slist).list as *mut c_void,
            0 as c_int,
            ((max_size + 1 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut x264_frame_t>() as size_t),
        );
        if pthread_mutex_init(&mut (*slist).mutex, 0 as *const pthread_mutexattr_t) != 0
            || pthread_cond_init(&mut (*slist).cv_fill, 0 as *const pthread_condattr_t) != 0
            || pthread_cond_init(&mut (*slist).cv_empty, 0 as *const pthread_condattr_t) != 0
        {
            return -(1 as c_int);
        }
        return 0 as c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "869:1"]
unsafe extern "C" fn x264_10_sync_frame_list_delete(mut slist: *mut x264_sync_frame_list_t) {
    pthread_mutex_destroy(&mut (*slist).mutex);
    pthread_cond_destroy(&mut (*slist).cv_fill);
    pthread_cond_destroy(&mut (*slist).cv_empty);
    x264_10_frame_delete_list((*slist).list);
}
#[no_mangle]
#[c2rust::src_loc = "877:1"]
unsafe extern "C" fn x264_10_sync_frame_list_push(
    mut slist: *mut x264_sync_frame_list_t,
    mut frame: *mut x264_frame_t,
) {
    pthread_mutex_lock(&mut (*slist).mutex);
    while (*slist).i_size == (*slist).i_max_size {
        pthread_cond_wait(&mut (*slist).cv_empty, &mut (*slist).mutex);
    }
    let fresh36 = (*slist).i_size;
    (*slist).i_size = (*slist).i_size + 1;
    let ref mut fresh37 = *(*slist).list.offset(fresh36 as isize);
    *fresh37 = frame;
    pthread_mutex_unlock(&mut (*slist).mutex);
    pthread_cond_broadcast(&mut (*slist).cv_fill);
}
#[no_mangle]
#[c2rust::src_loc = "887:1"]
unsafe extern "C" fn x264_10_sync_frame_list_pop(
    mut slist: *mut x264_sync_frame_list_t,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    pthread_mutex_lock(&mut (*slist).mutex);
    while (*slist).i_size == 0 {
        pthread_cond_wait(&mut (*slist).cv_fill, &mut (*slist).mutex);
    }
    (*slist).i_size -= 1;
    frame = *(*slist).list.offset((*slist).i_size as isize);
    let ref mut fresh38 = *(*slist).list.offset((*slist).i_size as isize);
    *fresh38 = 0 as *mut x264_frame_t;
    pthread_cond_broadcast(&mut (*slist).cv_empty);
    pthread_mutex_unlock(&mut (*slist).mutex);
    return frame;
}
