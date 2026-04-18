use crate::src::common::macroblock::MacroblockType;
use crate::src::common::macroblock::Partition;
use crate::src::encoder::analyse::base_h::x264_clip3;
use crate::src::encoder::analyse::base_h::x264_clip3f;
use crate::src::encoder::analyse::base_h::x264_exp2fix8;
use crate::src::encoder::analyse::base_h::x264_log2;
use crate::src::encoder::analyse::base_h::x264_median_mv;
use crate::src::encoder::analyse::bitstream_h::bs_size_se;
use crate::src::encoder::analyse::bitstream_h::bs_size_ue;
use crate::src::encoder::analyse::mb_analyse_load_costs;
use crate::src::encoder::analyse::x264_h::x264_b_pyramid_names;
use crate::src::encoder::analyse::x264_mb_analysis_list_t;
use crate::src::encoder::analyse::x264_mb_analysis_t;
pub static mut delta_tfi_divisor: [crate::stdlib::uint8_t; 10] =
    [0u8, 2u8, 1u8, 1u8, 2u8, 2u8, 3u8, 3u8, 4u8, 6u8];
pub unsafe extern "C" fn lowres_context_init(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
) {
    unsafe {
        (*a).i_qp = crate::src::common::common::X264_LOOKAHEAD_QP;
        (*a).i_lambda =
            crate::src::common::tables::x264_lambda_tab[(*a).i_qp as usize] as ::core::ffi::c_int;
        mb_analyse_load_costs(h, a);
        if (*h).param.analyse.i_subpel_refine > 1i32 {
            (*h).mb.i_me_method = if (1i32) < (*h).param.analyse.i_me_method {
                1i32
            } else {
                (*h).param.analyse.i_me_method
            };
            (*h).mb.i_subpel_refine = 4i32;
        } else {
            (*h).mb.i_me_method = crate::x264_h::X264_ME_DIA;
            (*h).mb.i_subpel_refine = 2i32;
        }
        (*h).mb.chroma_me = false;
    }
}
pub unsafe extern "C" fn weight_get_h264(
    mut weight_nonh264: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut w: *mut crate::src::common::mc::x264_weight_t,
) {
    unsafe {
        (*w).i_offset = offset;
        (*w).i_denom = 7i32;
        (*w).i_scale = weight_nonh264;
        while (*w).i_denom > 0i32 && (*w).i_scale > 127i32 {
            (*w).i_denom -= 1;
            (*w).i_scale >>= 1i32;
        }
        (*w).i_scale = if (*w).i_scale < 127i32 {
            (*w).i_scale
        } else {
            127i32
        };
    }
}
#[inline(never)]
pub unsafe extern "C" fn weight_cost_init_luma(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut ref_0: *mut crate::src::common::frame::x264_frame_t,
    mut dest: *mut crate::src::common::common::pixel,
) -> *mut crate::src::common::common::pixel {
    unsafe {
        let mut ref0_distance = (*fenc).i_frame - (*ref_0).i_frame - 1i32;
        if (*(*fenc).lowres_mvs[0usize][ref0_distance as usize].offset(0isize))[0usize]
            as ::core::ffi::c_int
            != 0x7fffi32
        {
            let mut y = 0i32;
            let mut i_stride = (*fenc).i_stride_lowres;
            let mut i_lines = (*fenc).i_lines_lowres;
            let mut i_width = (*fenc).i_width_lowres;
            let mut p = dest;
            while y < i_lines {
                let mut x = 0i32;
                while x < i_width {
                    let mut i_mb_xy = 0i32;
                    let mut mvx = (*(*fenc).lowres_mvs[0usize][ref0_distance as usize]
                        .offset(i_mb_xy as isize))[0usize]
                        as ::core::ffi::c_int;
                    let mut mvy = (*(*fenc).lowres_mvs[0usize][ref0_distance as usize]
                        .offset(i_mb_xy as isize))[1usize]
                        as ::core::ffi::c_int;
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        p.offset(x as isize),
                        i_stride as crate::stdlib::intptr_t,
                        &raw mut (*ref_0).lowres as *mut *mut crate::src::common::common::pixel,
                        i_stride as crate::stdlib::intptr_t,
                        mvx + (x << 2i32),
                        mvy + (y << 2i32),
                        8i32,
                        8i32,
                        &raw mut crate::src::common::tables::x264_zero
                            as *const crate::src::common::mc::x264_weight_t,
                    );
                    x += 8i32;
                    i_mb_xy += 1;
                }
                y += 8i32;
                p = p.offset((i_stride * 8i32) as isize);
            }
            return dest;
        }
        (*ref_0).lowres[0usize]
    }
}
#[inline(never)]
pub unsafe extern "C" fn weight_cost_init_chroma(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut ref_0: *mut crate::src::common::frame::x264_frame_t,
    mut dstu: *mut crate::src::common::common::pixel,
    mut dstv: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut ref0_distance = (*fenc).i_frame - (*ref_0).i_frame - 1i32;
        let mut i_stride = (*fenc).i_stride[1usize];
        let mut i_lines = (*fenc).i_lines[1usize];
        let mut i_width = (*fenc).i_width[1usize];
        let mut v_shift = ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
        let mut cw = 8i32 * (*h).mb.i_mb_width;
        let mut ch = (16i32 * (*h).mb.i_mb_height) >> v_shift;
        let mut height = 16i32 >> v_shift;
        if (*(*fenc).lowres_mvs[0usize][ref0_distance as usize].offset(0isize))[0usize]
            as ::core::ffi::c_int
            != 0x7fffi32
        {
            let mut y = 0i32;
            crate::src::common::frame::x264_8_frame_expand_border_chroma(h, ref_0, 1i32);
            while y < i_lines {
                let mut pel_offset_y = 0i32;
                let mut x = 0i32;
                while x < i_width {
                    let mut mb_xy = 0i32;
                    let mut pel_offset_x = 0i32;
                    let mut pixu = dstu
                        .offset(pel_offset_y as isize)
                        .offset(pel_offset_x as isize);
                    let mut pixv = dstv
                        .offset(pel_offset_y as isize)
                        .offset(pel_offset_x as isize);
                    let mut src1 = (*ref_0).plane[1usize]
                        .offset(pel_offset_y as isize)
                        .offset((pel_offset_x * 2i32) as isize);
                    let mut mvx = (*(*fenc).lowres_mvs[0usize][ref0_distance as usize]
                        .offset(mb_xy as isize))[0usize]
                        as ::core::ffi::c_int;
                    let mut mvy = (*(*fenc).lowres_mvs[0usize][ref0_distance as usize]
                        .offset(mb_xy as isize))[1usize]
                        as ::core::ffi::c_int;
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        pixu,
                        pixv,
                        i_stride as crate::stdlib::intptr_t,
                        src1,
                        i_stride as crate::stdlib::intptr_t,
                        mvx,
                        (2i32 * mvy) >> v_shift,
                        8i32,
                        height,
                    );
                    x += 8i32;
                    mb_xy += 1;
                    pel_offset_x += 8i32;
                }
                y += height;
                pel_offset_y = y * i_stride;
            }
        } else {
            (*h).mc
                .plane_copy_deinterleave
                .expect("non-null function pointer")(
                dstu,
                i_stride as crate::stdlib::intptr_t,
                dstv,
                i_stride as crate::stdlib::intptr_t,
                (*ref_0).plane[1usize],
                i_stride as crate::stdlib::intptr_t,
                cw,
                ch,
            );
        }
        (*h).mc
            .plane_copy_deinterleave
            .expect("non-null function pointer")(
            dstu.offset(i_width as isize),
            i_stride as crate::stdlib::intptr_t,
            dstv.offset(i_width as isize),
            i_stride as crate::stdlib::intptr_t,
            (*fenc).plane[1usize],
            i_stride as crate::stdlib::intptr_t,
            cw,
            ch,
        );
    }
}
#[inline(never)]
pub unsafe extern "C" fn weight_cost_init_chroma444(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut ref_0: *mut crate::src::common::frame::x264_frame_t,
    mut dst: *mut crate::src::common::common::pixel,
    mut p: ::core::ffi::c_int,
) -> *mut crate::src::common::common::pixel {
    unsafe {
        let mut ref0_distance = (*fenc).i_frame - (*ref_0).i_frame - 1i32;
        let mut i_stride = (*fenc).i_stride[p as usize];
        let mut i_lines = (*fenc).i_lines[p as usize];
        let mut i_width = (*fenc).i_width[p as usize];
        if (*(*fenc).lowres_mvs[0usize][ref0_distance as usize].offset(0isize))[0usize]
            as ::core::ffi::c_int
            != 0x7fffi32
        {
            let mut y = 0i32;
            crate::src::common::frame::x264_8_frame_expand_border_chroma(h, ref_0, p);
            while y < i_lines {
                let mut pel_offset_y = 0i32;
                let mut x = 0i32;
                while x < i_width {
                    let mut mb_xy = 0i32;
                    let mut pel_offset_x = 0i32;
                    let mut pix = dst
                        .offset(pel_offset_y as isize)
                        .offset(pel_offset_x as isize);
                    let mut src = (*ref_0).plane[p as usize]
                        .offset(pel_offset_y as isize)
                        .offset(pel_offset_x as isize);
                    let mut mvx = (*(*fenc).lowres_mvs[0usize][ref0_distance as usize]
                        .offset(mb_xy as isize))[0usize]
                        as ::core::ffi::c_int
                        / 2i32;
                    let mut mvy = (*(*fenc).lowres_mvs[0usize][ref0_distance as usize]
                        .offset(mb_xy as isize))[1usize]
                        as ::core::ffi::c_int
                        / 2i32;
                    (*h).mc
                        .copy_16x16_unaligned
                        .expect("non-null function pointer")(
                        pix,
                        i_stride as crate::stdlib::intptr_t,
                        src.offset(mvx as isize).offset((mvy * i_stride) as isize),
                        i_stride as crate::stdlib::intptr_t,
                        16i32,
                    );
                    x += 16i32;
                    mb_xy += 1;
                    pel_offset_x += 16i32;
                }
                y += 16i32;
                pel_offset_y = y * i_stride;
            }
            return dst;
        }
        (*ref_0).plane[p as usize]
    }
}
pub unsafe extern "C" fn weight_slice_header_cost(
    mut h: *mut crate::src::common::common::x264_t,
    mut w: *mut crate::src::common::mc::x264_weight_t,
    mut b_chroma: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut numslices = 0;
        let mut lambda = crate::src::common::tables::x264_lambda_tab
            [crate::src::common::common::X264_LOOKAHEAD_QP as usize]
            as ::core::ffi::c_int;
        if b_chroma != 0 {
            lambda *= 4i32;
        }
        if (*h).param.i_slice_count != 0 {
            numslices = (*h).param.i_slice_count;
        } else if (*h).param.i_slice_max_mbs != 0 {
            numslices = ((*h).mb.i_mb_width * (*h).mb.i_mb_height + (*h).param.i_slice_max_mbs
                - 1i32)
                / (*h).param.i_slice_max_mbs;
        } else {
            numslices = 1i32;
        }
        let mut denom_cost =
            bs_size_ue((*w.offset(0isize)).i_denom as ::core::ffi::c_uint) * (2i32 - b_chroma);
        lambda
            * numslices
            * (10i32
                + denom_cost
                + 2i32
                    * (bs_size_se((*w.offset(0isize)).i_scale)
                        + bs_size_se((*w.offset(0isize)).i_offset)))
    }
}
#[inline(never)]
pub unsafe extern "C" fn weight_cost_luma(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut src: *mut crate::src::common::common::pixel,
    mut w: *mut crate::src::common::mc::x264_weight_t,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut cost = 0u32;
        let mut pixoff = 0i32;
        let mut i_mb = 0i32;
        let mut i_stride = (*fenc).i_stride_lowres;
        let mut i_lines = (*fenc).i_lines_lowres;
        let mut i_width = (*fenc).i_width_lowres;
        let mut fenc_plane = (*fenc).lowres[0usize];
        if !w.is_null() {
            let mut y = 0i32;
            while y < i_lines {
                let mut x = 0i32;
                while x < i_width {
                    let mut buf = [0; 64];
                    (*(*w).weightfn.offset((8i32 >> 2i32) as isize))
                        .expect("non-null function pointer")(
                        &raw mut buf as *mut crate::src::common::common::pixel,
                        8isize,
                        src.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                        w,
                        8i32,
                    );
                    let mut cmp = (*h).pixf.mbcmp
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        &raw mut buf as *mut crate::src::common::common::pixel,
                        8isize,
                        fenc_plane.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                    );
                    cost = cost.wrapping_add(
                        (if cmp < *(*fenc).i_intra_cost.offset(i_mb as isize) as ::core::ffi::c_int
                        {
                            cmp
                        } else {
                            *(*fenc).i_intra_cost.offset(i_mb as isize) as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    x += 8i32;
                    i_mb += 1;
                    pixoff += 8i32;
                }
                y += 8i32;
                pixoff = y * i_stride;
            }
            cost = cost.wrapping_add(weight_slice_header_cost(h, w, 0i32) as ::core::ffi::c_uint);
        } else {
            let mut y_0 = 0i32;
            while y_0 < i_lines {
                let mut x_0 = 0i32;
                while x_0 < i_width {
                    let mut cmp_0 = (*h).pixf.mbcmp
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        src.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                        fenc_plane.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                    );
                    cost = cost.wrapping_add(
                        (if cmp_0
                            < *(*fenc).i_intra_cost.offset(i_mb as isize) as ::core::ffi::c_int
                        {
                            cmp_0
                        } else {
                            *(*fenc).i_intra_cost.offset(i_mb as isize) as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    );
                    x_0 += 8i32;
                    i_mb += 1;
                    pixoff += 8i32;
                }
                y_0 += 8i32;
                pixoff = y_0 * i_stride;
            }
        }
        cost
    }
}
#[inline(never)]
pub unsafe extern "C" fn weight_cost_chroma(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut ref_0: *mut crate::src::common::common::pixel,
    mut w: *mut crate::src::common::mc::x264_weight_t,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut cost = 0u32;
        let mut pixoff = 0i32;
        let mut i_stride = (*fenc).i_stride[1usize];
        let mut i_lines = (*fenc).i_lines[1usize];
        let mut i_width = (*fenc).i_width[1usize];
        let mut src = ref_0.offset(i_width as isize);
        let mut height = 16i32 >> ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
        if !w.is_null() {
            let mut y = 0i32;
            while y < i_lines {
                let mut x = 0i32;
                while x < i_width {
                    let mut buf = [0; 128];
                    (*(*w).weightfn.offset((8i32 >> 2i32) as isize))
                        .expect("non-null function pointer")(
                        &raw mut buf as *mut crate::src::common::common::pixel,
                        8isize,
                        ref_0.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                        w,
                        height,
                    );
                    cost = cost.wrapping_add((*h).pixf.asd8.expect("non-null function pointer")(
                        &raw mut buf as *mut crate::src::common::common::pixel,
                        8isize,
                        src.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                        height,
                    ) as ::core::ffi::c_uint);
                    x += 8i32;
                    pixoff += 8i32;
                }
                y += height;
                pixoff = y * i_stride;
            }
            cost = cost.wrapping_add(weight_slice_header_cost(h, w, 1i32) as ::core::ffi::c_uint);
        } else {
            let mut y_0 = 0i32;
            while y_0 < i_lines {
                let mut x_0 = 0i32;
                while x_0 < i_width {
                    cost = cost.wrapping_add((*h).pixf.asd8.expect("non-null function pointer")(
                        ref_0.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                        src.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                        height,
                    ) as ::core::ffi::c_uint);
                    x_0 += 8i32;
                    pixoff += 8i32;
                }
                y_0 += height;
                pixoff = y_0 * i_stride;
            }
        }
        cost
    }
}
#[inline(never)]
pub unsafe extern "C" fn weight_cost_chroma444(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut ref_0: *mut crate::src::common::common::pixel,
    mut w: *mut crate::src::common::mc::x264_weight_t,
    mut p: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut cost = 0u32;
        let mut pixoff = 0i32;
        let mut i_stride = (*fenc).i_stride[p as usize];
        let mut i_lines = (*fenc).i_lines[p as usize];
        let mut i_width = (*fenc).i_width[p as usize];
        let mut src = (*fenc).plane[p as usize];
        if !w.is_null() {
            let mut y = 0i32;
            while y < i_lines {
                let mut x = 0i32;
                while x < i_width {
                    let mut buf = [0; 256];
                    (*(*w).weightfn.offset((16i32 >> 2i32) as isize))
                        .expect("non-null function pointer")(
                        &raw mut buf as *mut crate::src::common::common::pixel,
                        16isize,
                        ref_0.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                        w,
                        16i32,
                    );
                    cost = cost.wrapping_add((*h).pixf.mbcmp
                        [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        &raw mut buf as *mut crate::src::common::common::pixel,
                        16isize,
                        src.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                    ) as ::core::ffi::c_uint);
                    x += 16i32;
                    pixoff += 16i32;
                }
                y += 16i32;
                pixoff = y * i_stride;
            }
            cost = cost.wrapping_add(weight_slice_header_cost(h, w, 1i32) as ::core::ffi::c_uint);
        } else {
            let mut y_0 = 0i32;
            while y_0 < i_lines {
                let mut x_0 = 0i32;
                while x_0 < i_width {
                    cost = cost.wrapping_add((*h).pixf.mbcmp
                        [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        ref_0.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                        src.offset(pixoff as isize),
                        i_stride as crate::stdlib::intptr_t,
                    ) as ::core::ffi::c_uint);
                    x_0 += 16i32;
                    pixoff += 16i32;
                }
                y_0 += 16i32;
                pixoff = y_0 * i_stride;
            }
        }
        cost
    }
}
pub unsafe extern "C" fn x264_8_weights_analyse(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut ref_0: *mut crate::src::common::frame::x264_frame_t,
    mut b_lookahead: ::core::ffi::c_int,
) {
    unsafe {
        let mut guess_scale = [0.; 3];
        let mut fenc_mean = [0.; 3];
        let mut ref_mean = [0.; 3];
        let mut plane = 0i32;
        let mut chroma_denom = 7i32;
        let mut plane_0 = 0i32;
        let mut i_0 = 1i32;
        let mut i_delta_index = (*fenc).i_frame - (*ref_0).i_frame - 1i32;
        let epsilon = 1.0 / 128.0;
        let mut weights = &raw mut *(&raw mut (*fenc).weight
            as *mut [crate::src::common::mc::x264_weight_t; 3])
            .offset(0isize) as *mut crate::src::common::mc::x264_weight_t;
        (*weights.offset(0isize)).i_scale = 1i32;
        (*weights.offset(0isize)).i_denom = 0i32;
        (*weights.offset(0isize)).i_offset = 0i32;
        let ref mut c2rust_fresh8 = (*weights.offset(0isize)).weightfn;
        *c2rust_fresh8 = ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
        (*weights.offset(1isize)).i_scale = 1i32;
        (*weights.offset(1isize)).i_denom = 0i32;
        (*weights.offset(1isize)).i_offset = 0i32;
        let ref mut c2rust_fresh9 = (*weights.offset(1isize)).weightfn;
        *c2rust_fresh9 = ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
        (*weights.offset(2isize)).i_scale = 1i32;
        (*weights.offset(2isize)).i_denom = 0i32;
        (*weights.offset(2isize)).i_offset = 0i32;
        let ref mut c2rust_fresh10 = (*weights.offset(2isize)).weightfn;
        *c2rust_fresh10 = ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
        while plane <= 2i32 * (b_lookahead == 0) as ::core::ffi::c_int {
            if plane == 0 || !(*h).sps.i_chroma_format_idc.is_400() {
                let mut zero_bias =
                    ((*ref_0).i_pixel_ssd[plane as usize] == 0) as ::core::ffi::c_int;
                let mut fenc_var = (*fenc).i_pixel_ssd[plane as usize]
                    .wrapping_add(zero_bias as crate::stdlib::uint64_t)
                    as ::core::ffi::c_float;
                let mut ref_var = (*ref_0).i_pixel_ssd[plane as usize]
                    .wrapping_add(zero_bias as crate::stdlib::uint64_t)
                    as ::core::ffi::c_float;
                guess_scale[plane as usize] = crate::stdlib::sqrtf(fenc_var / ref_var);
                fenc_mean[plane as usize] = (*fenc).i_pixel_sum[plane as usize]
                    .wrapping_add(zero_bias as crate::stdlib::uint32_t)
                    as ::core::ffi::c_float
                    / ((*fenc).i_lines[(plane != 0) as ::core::ffi::c_int as usize]
                        * (*fenc).i_width[(plane != 0) as ::core::ffi::c_int as usize])
                        as ::core::ffi::c_float
                    / ((1i32) << (crate::internal::BIT_DEPTH - 8i32)) as ::core::ffi::c_float;
                ref_mean[plane as usize] = (*ref_0).i_pixel_sum[plane as usize]
                    .wrapping_add(zero_bias as crate::stdlib::uint32_t)
                    as ::core::ffi::c_float
                    / ((*fenc).i_lines[(plane != 0) as ::core::ffi::c_int as usize]
                        * (*fenc).i_width[(plane != 0) as ::core::ffi::c_int as usize])
                        as ::core::ffi::c_float
                    / ((1i32) << (crate::internal::BIT_DEPTH - 8i32)) as ::core::ffi::c_float;
            } else {
                guess_scale[plane as usize] = 1f32;
                fenc_mean[plane as usize] = 0f32;
                ref_mean[plane as usize] = 0f32;
            }
            plane += 1;
        }
        if b_lookahead == 0 {
            while chroma_denom > 0i32 {
                let mut thresh = 127.0 / ((1i32) << chroma_denom) as ::core::ffi::c_float;
                if guess_scale[1usize] < thresh && guess_scale[2usize] < thresh {
                    break;
                }
                chroma_denom -= 1;
            }
        }
        while plane_0
            < (if !(*h).sps.i_chroma_format_idc.is_400() {
                3i32
            } else {
                1i32
            })
            && !(plane_0 != 0 && ((*weights.offset(0isize)).weightfn.is_null() || b_lookahead != 0))
        {
            if crate::stdlib::fabsf(ref_mean[plane_0 as usize] - fenc_mean[plane_0 as usize]) < 0.5
                && crate::stdlib::fabsf(1.0 - guess_scale[plane_0 as usize]) < epsilon
            {
                (*weights.offset(plane_0 as isize)).i_scale = 1i32;
                (*weights.offset(plane_0 as isize)).i_denom = 0i32;
                (*weights.offset(plane_0 as isize)).i_offset = 0i32;
                let ref mut c2rust_fresh11 = (*weights.offset(plane_0 as isize)).weightfn;
                *c2rust_fresh11 = ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
            } else {
                let mut minoff = 0;
                let mut minscale = 0;
                let mut mindenom = 0;
                let mut minscore = 0;
                let mut origscore = 0;
                let mut found = 0;
                let mut mcbuf = ::core::ptr::null_mut::<crate::src::common::common::pixel>();
                if plane_0 != 0 {
                    (*weights.offset(plane_0 as isize)).i_denom = chroma_denom;
                    (*weights.offset(plane_0 as isize)).i_scale = x264_clip3(
                        crate::stdlib::round(
                            (guess_scale[plane_0 as usize]
                                * ((1i32) << chroma_denom) as ::core::ffi::c_float)
                                as ::core::ffi::c_double,
                        ) as ::core::ffi::c_int,
                        0i32,
                        255i32,
                    );
                    if (*weights.offset(plane_0 as isize)).i_scale > 127i32 {
                        let ref mut c2rust_fresh12 = (*weights.offset(2isize)).weightfn;
                        *c2rust_fresh12 =
                            ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
                        let ref mut c2rust_fresh13 = (*weights.offset(1isize)).weightfn;
                        *c2rust_fresh13 = *c2rust_fresh12;
                        break;
                    }
                } else {
                    weight_get_h264(
                        crate::stdlib::round(
                            (guess_scale[plane_0 as usize] * 128f32) as ::core::ffi::c_double,
                        ) as ::core::ffi::c_int,
                        0i32,
                        weights.offset(plane_0 as isize),
                    );
                }
                found = 0i32;
                mindenom = (*weights.offset(plane_0 as isize)).i_denom;
                minscale = (*weights.offset(plane_0 as isize)).i_scale;
                minoff = 0i32;
                if plane_0 == 0 {
                    if !(*fenc).intra_calculated {
                        let mut a = x264_mb_analysis_t {
                            i_lambda: 0,
                            i_lambda2: 0,
                            i_qp: 0,
                            p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                            p_cost_ref: [::core::ptr::null_mut::<crate::stdlib::uint16_t>(); 2],
                            i_mbrd: 0,
                            fast_intra: false,
                            force_intra: false,
                            avoid_topright: false,
                            try_skip: false,
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
                                me16x16: crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                },
                                bi16x16: crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                },
                                me8x8: [crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 4],
                                me4x4: [[crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 4]; 4],
                                me8x4: [[crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 2]; 4],
                                me4x8: [[crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 2]; 4],
                                me16x8: [crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 2],
                                me8x16: [crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
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
                                me16x16: crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                },
                                bi16x16: crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                },
                                me8x8: [crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 4],
                                me4x4: [[crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 4]; 4],
                                me8x4: [[crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 2]; 4],
                                me4x8: [[crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 2]; 4],
                                me16x8: [crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_stride: [0; 3],
                                    mvp: [0; 2],
                                    cost_mv: 0,
                                    cost: 0,
                                    mv: [0; 2],
                                }; 2],
                                me8x16: [crate::src::encoder::me::x264_me_t {
                                    i_pixel: 0,
                                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                                    i_ref_cost: 0,
                                    i_ref: 0,
                                    weight: ::core::ptr::null::<
                                        crate::src::common::mc::x264_weight_t,
                                    >(),
                                    p_fref: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 12],
                                    p_fref_w: ::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(),
                                    p_fenc: [::core::ptr::null_mut::<
                                        crate::src::common::common::pixel,
                                    >(); 3],
                                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
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
                            i_mb_partition16x8: [Partition::D_16x16; 2],
                            i_mb_partition8x16: [Partition::D_16x16; 2],
                            i_mb_type16x8: MacroblockType::I_4x4,
                            i_mb_type8x16: MacroblockType::I_4x4,
                            direct_available: false,
                            early_terminate: false,
                        };
                        lowres_context_init(h, &raw mut a);
                        slicetype_frame_cost(h, &raw mut a, &raw mut fenc, 0i32, 0i32, 0i32);
                    }
                    mcbuf = weight_cost_init_luma(h, fenc, ref_0, (*h).mb.p_weight_buf[0usize]);
                    minscore = weight_cost_luma(
                        h,
                        fenc,
                        mcbuf,
                        ::core::ptr::null_mut::<crate::src::common::mc::x264_weight_t>(),
                    );
                    origscore = minscore;
                } else if (*h).sps.i_chroma_format_idc.is_444() {
                    mcbuf = weight_cost_init_chroma444(
                        h,
                        fenc,
                        ref_0,
                        (*h).mb.p_weight_buf[0usize],
                        plane_0,
                    );
                    minscore = weight_cost_chroma444(
                        h,
                        fenc,
                        mcbuf,
                        ::core::ptr::null_mut::<crate::src::common::mc::x264_weight_t>(),
                        plane_0,
                    );
                    origscore = minscore;
                } else {
                    let mut chroma_initted = 0i32;
                    let mut dstu = (*h).mb.p_weight_buf[0usize];
                    let mut dstv = (*h).mb.p_weight_buf[0usize]
                        .offset(((*fenc).i_stride[1usize] * (*fenc).i_lines[1usize]) as isize);
                    let c2rust_fresh14 = chroma_initted;
                    chroma_initted += 1;
                    if c2rust_fresh14 == 0 {
                        weight_cost_init_chroma(h, fenc, ref_0, dstu, dstv);
                    }
                    mcbuf = if plane_0 == 1i32 { dstu } else { dstv };
                    minscore = weight_cost_chroma(
                        h,
                        fenc,
                        mcbuf,
                        ::core::ptr::null_mut::<crate::src::common::mc::x264_weight_t>(),
                    );
                    origscore = minscore;
                }
                if minscore != 0 {
                    pub static mut weight_check_distance: [[crate::stdlib::uint8_t; 2]; 12] = [
                        [0u8, 0u8],
                        [0u8, 0u8],
                        [0u8, 1u8],
                        [0u8, 1u8],
                        [0u8, 1u8],
                        [0u8, 1u8],
                        [0u8, 1u8],
                        [1u8, 1u8],
                        [1u8, 1u8],
                        [2u8, 1u8],
                        [2u8, 1u8],
                        [4u8, 2u8],
                    ];
                    let mut scale_dist = if b_lookahead != 0 {
                        0i32
                    } else {
                        weight_check_distance[(*h).param.analyse.i_subpel_refine as usize][0usize]
                            as ::core::ffi::c_int
                    };
                    let mut offset_dist = if b_lookahead != 0 {
                        0i32
                    } else {
                        weight_check_distance[(*h).param.analyse.i_subpel_refine as usize][1usize]
                            as ::core::ffi::c_int
                    };
                    let mut start_scale = x264_clip3(minscale - scale_dist, 0i32, 127i32);
                    let mut end_scale = x264_clip3(minscale + scale_dist, 0i32, 127i32);
                    let mut i_scale = start_scale;
                    while i_scale <= end_scale {
                        let mut cur_scale = i_scale;
                        let mut cur_offset = (fenc_mean[plane_0 as usize]
                            - ref_mean[plane_0 as usize] * cur_scale as ::core::ffi::c_float
                                / ((1i32) << mindenom) as ::core::ffi::c_float
                            + 0.5 * b_lookahead as ::core::ffi::c_float)
                            as ::core::ffi::c_int;
                        if cur_offset < -(128i32) || cur_offset > 127i32 {
                            cur_offset = x264_clip3(cur_offset, -(128i32), 127i32);
                            cur_scale = x264_clip3f(
                                (((1i32) << mindenom) as ::core::ffi::c_float
                                    * (fenc_mean[plane_0 as usize]
                                        - cur_offset as ::core::ffi::c_float)
                                    / ref_mean[plane_0 as usize]
                                    + 0.5f32)
                                    as ::core::ffi::c_double,
                                0f64,
                                127f64,
                            ) as ::core::ffi::c_int;
                        }
                        let mut start_offset =
                            x264_clip3(cur_offset - offset_dist, -(128i32), 127i32);
                        let mut end_offset =
                            x264_clip3(cur_offset + offset_dist, -(128i32), 127i32);
                        let mut i_off = start_offset;
                        while i_off <= end_offset {
                            let mut s = 0;
                            (*weights.offset(plane_0 as isize)).i_scale = cur_scale;
                            (*weights.offset(plane_0 as isize)).i_denom = mindenom;
                            (*weights.offset(plane_0 as isize)).i_offset = i_off;
                            (*h).mc.weight_cache.expect("non-null function pointer")(
                                h,
                                weights.offset(plane_0 as isize),
                            );
                            if plane_0 != 0 {
                                if (*h).sps.i_chroma_format_idc.is_444() {
                                    s = weight_cost_chroma444(
                                        h,
                                        fenc,
                                        mcbuf,
                                        weights.offset(plane_0 as isize),
                                        plane_0,
                                    );
                                } else {
                                    s = weight_cost_chroma(
                                        h,
                                        fenc,
                                        mcbuf,
                                        weights.offset(plane_0 as isize),
                                    );
                                }
                            } else {
                                s = weight_cost_luma(
                                    h,
                                    fenc,
                                    mcbuf,
                                    weights.offset(plane_0 as isize),
                                );
                            }
                            if s < minscore {
                                minscore = s;
                                minscale = cur_scale;
                                minoff = i_off;
                                found = 1i32;
                            }
                            if minoff == start_offset && i_off != start_offset {
                                break;
                            }
                            i_off += 1;
                        }
                        i_scale += 1;
                    }
                    if plane_0 == 0 {
                        while mindenom > 0i32 && minscale & 1i32 == 0 {
                            mindenom -= 1;
                            minscale >>= 1i32;
                        }
                    }
                    if found == 0
                        || minscale == (1i32) << mindenom && minoff == 0i32
                        || minscore as ::core::ffi::c_float / origscore as ::core::ffi::c_float
                            > 0.998
                    {
                        (*weights.offset(plane_0 as isize)).i_scale = 1i32;
                        (*weights.offset(plane_0 as isize)).i_denom = 0i32;
                        (*weights.offset(plane_0 as isize)).i_offset = 0i32;
                        let ref mut c2rust_fresh16 = (*weights.offset(plane_0 as isize)).weightfn;
                        *c2rust_fresh16 =
                            ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
                    } else {
                        (*weights.offset(plane_0 as isize)).i_scale = minscale;
                        (*weights.offset(plane_0 as isize)).i_denom = mindenom;
                        (*weights.offset(plane_0 as isize)).i_offset = minoff;
                        (*h).mc.weight_cache.expect("non-null function pointer")(
                            h,
                            weights.offset(plane_0 as isize),
                        );
                        if (*h).param.analyse.i_weighted_pred
                            == crate::src::common::base::X264_WEIGHTP_FAKE
                            && !(*weights.offset(0isize)).weightfn.is_null()
                            && plane_0 == 0
                        {
                            (*fenc).f_weighted_cost_delta[i_delta_index as usize] = minscore
                                as ::core::ffi::c_float
                                / origscore as ::core::ffi::c_float;
                        }
                    }
                }
            }
            plane_0 += 1;
        }
        if !(*weights.offset(1isize)).weightfn.is_null()
            || !(*weights.offset(2isize)).weightfn.is_null()
        {
            let mut denom = if !(*weights.offset(1isize)).weightfn.is_null() {
                (*weights.offset(1isize)).i_denom
            } else {
                (*weights.offset(2isize)).i_denom
            };
            let mut both_weighted = (!(*weights.offset(1isize)).weightfn.is_null()
                && !(*weights.offset(2isize)).weightfn.is_null())
                as ::core::ffi::c_int;
            while both_weighted == 0 && denom == 7i32
                || denom > 0i32
                    && !(!(*weights.offset(1isize)).weightfn.is_null()
                        && (*weights.offset(1isize)).i_scale & 1i32 != 0)
                    && !(!(*weights.offset(2isize)).weightfn.is_null()
                        && (*weights.offset(2isize)).i_scale & 1i32 != 0)
            {
                let mut i = 1i32;
                denom -= 1;
                while i <= 2i32 {
                    if !(*weights.offset(i as isize)).weightfn.is_null() {
                        (*weights.offset(i as isize)).i_scale >>= 1i32;
                        (*weights.offset(i as isize)).i_denom = denom;
                    }
                    i += 1;
                }
            }
        }
        while i_0 <= 2i32 {
            if !(*weights.offset(i_0 as isize)).weightfn.is_null() {
                (*h).mc.weight_cache.expect("non-null function pointer")(
                    h,
                    weights.offset(i_0 as isize),
                );
            }
            i_0 += 1;
        }
        if !(*weights.offset(0isize)).weightfn.is_null() && b_lookahead != 0 {
            let mut src = (*ref_0).buffer_lowres;
            let mut dst = (*h).mb.p_weight_buf[0usize];
            let mut width = (*ref_0).i_width_lowres
                + ((if 32i32
                    > 64i32
                        / ::core::mem::size_of::<crate::src::common::common::pixel>()
                            as ::core::ffi::c_int
                {
                    32i32
                } else {
                    64i32
                        / ::core::mem::size_of::<crate::src::common::common::pixel>()
                            as ::core::ffi::c_int
                }) + crate::src::common::frame::PADH);
            let mut height = (*ref_0).i_lines_lowres + crate::src::common::frame::PADV * 2i32;
            crate::src::common::frame::x264_8_weight_scale_plane(
                h,
                dst,
                (*ref_0).i_stride_lowres as crate::stdlib::intptr_t,
                src,
                (*ref_0).i_stride_lowres as crate::stdlib::intptr_t,
                width,
                height,
                weights.offset(0isize),
            );
            (*fenc).weighted[0usize] = (*h).mb.p_weight_buf[0usize]
                .offset(
                    (if 32i32
                        > 64i32
                            / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                    {
                        32i32
                    } else {
                        64i32
                            / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                    }) as isize,
                )
                .offset(((*ref_0).i_stride_lowres * crate::src::common::frame::PADV) as isize);
        }
    }
}
pub unsafe extern "C" fn slicetype_mb_cost(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut p0: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut dist_scale_factor: ::core::ffi::c_int,
    mut do_search: *mut ::core::ffi::c_int,
    mut w: *const crate::src::common::mc::x264_weight_t,
    mut output_inter: *mut ::core::ffi::c_int,
    mut output_intra: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut pix1 = [0; 288];
        let mut list_used = 0i32;
        let mut lowres_penalty = 4i32;
        let mut fref0 = *frames.offset(p0 as isize);
        let mut fref1 = *frames.offset(p1 as isize);
        let mut fenc = *frames.offset(b as isize);
        let b_bidir = (b < p1) as ::core::ffi::c_int;
        let i_mb_x = (*h).mb.i_mb_x;
        let i_mb_y = (*h).mb.i_mb_y;
        let i_mb_stride = (*h).mb.i_mb_width;
        let i_mb_xy = i_mb_x + i_mb_y * i_mb_stride;
        let i_stride = (*fenc).i_stride_lowres;
        let i_pel_offset = 8i32 * (i_mb_x + i_mb_y * i_stride);
        let i_bipred_weight = if (*h).param.analyse.weighted_bipred {
            64i32 - (dist_scale_factor >> 2i32)
        } else {
            32i32
        };
        let mut fenc_mvs = [
            if b != p0 {
                (*(&raw mut *(&raw mut (*fenc).lowres_mvs
                    as *mut [*mut [crate::stdlib::int16_t; 2]; 17])
                    .offset(0isize) as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset((b - p0 - 1i32) as isize))
                .offset(i_mb_xy as isize)
            } else {
                ::core::ptr::null_mut::<[crate::stdlib::int16_t; 2]>()
            },
            if b != p1 {
                (*(&raw mut *(&raw mut (*fenc).lowres_mvs
                    as *mut [*mut [crate::stdlib::int16_t; 2]; 17])
                    .offset(1isize) as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset((p1 - b - 1i32) as isize))
                .offset(i_mb_xy as isize)
            } else {
                ::core::ptr::null_mut::<[crate::stdlib::int16_t; 2]>()
            },
        ];
        let mut fenc_costs = [
            if b != p0 {
                (*(&raw mut *(&raw mut (*fenc).lowres_mv_costs
                    as *mut [*mut ::core::ffi::c_int; 17])
                    .offset(0isize) as *mut *mut ::core::ffi::c_int)
                    .offset((b - p0 - 1i32) as isize))
                .offset(i_mb_xy as isize)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_int>()
            },
            if b != p1 {
                (*(&raw mut *(&raw mut (*fenc).lowres_mv_costs
                    as *mut [*mut ::core::ffi::c_int; 17])
                    .offset(1isize) as *mut *mut ::core::ffi::c_int)
                    .offset((p1 - b - 1i32) as isize))
                .offset(i_mb_xy as isize)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_int>()
            },
        ];
        let mut b_frame_score_mb = (i_mb_x > 0i32
            && i_mb_x < (*h).mb.i_mb_width - 1i32
            && i_mb_y > 0i32
            && i_mb_y < (*h).mb.i_mb_height - 1i32
            || (*h).mb.i_mb_width <= 2i32
            || (*h).mb.i_mb_height <= 2i32)
            as ::core::ffi::c_int;
        let mut pix2 = (&raw mut pix1 as *mut crate::src::common::common::pixel).offset(8isize);
        let mut i_bcost = crate::src::encoder::me::COST_MAX;
        (*h).mb.pic.p_fenc[0usize] =
            &raw mut (*h).mb.pic.fenc_buf as *mut crate::src::common::common::pixel;
        (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[0usize],
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            (*(&raw mut (*fenc).lowres as *mut *mut crate::src::common::common::pixel)
                .offset(0isize))
            .offset(i_pel_offset as isize),
            i_stride as crate::stdlib::intptr_t,
            8i32,
        );
        if p0 != p1 {
            let mut mv_range = 0;
            let mut m = [crate::src::encoder::me::x264_me_t {
                i_pixel: 0,
                p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                i_ref_cost: 0,
                i_ref: 0,
                weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                i_stride: [0; 3],
                mvp: [0; 2],
                cost_mv: 0,
                cost: 0,
                mv: [0; 2],
            }; 2];
            let mut l = 0i32;
            mv_range = 2i32 * (*h).param.analyse.i_mv_range;
            (*h).mb.mv_min_spel[0usize] = if 4i32 * (-(8i32) * (*h).mb.i_mb_x - 12i32) > -mv_range {
                4i32 * (-(8i32) * (*h).mb.i_mb_x - 12i32)
            } else {
                -mv_range
            };
            (*h).mb.mv_max_spel[0usize] = if 4i32
                * (8i32 * ((*h).mb.i_mb_width - (*h).mb.i_mb_x - 1i32) + 12i32)
                < mv_range - 1i32
            {
                4i32 * (8i32 * ((*h).mb.i_mb_width - (*h).mb.i_mb_x - 1i32) + 12i32)
            } else {
                mv_range - 1i32
            };
            (*h).mb.mv_limit_fpel[0usize][0usize] =
                ((*h).mb.mv_min_spel[0usize] >> 2i32) as crate::stdlib::int16_t;
            (*h).mb.mv_limit_fpel[1usize][0usize] =
                ((*h).mb.mv_max_spel[0usize] >> 2i32) as crate::stdlib::int16_t;
            if (*h).mb.i_mb_x >= (*h).mb.i_mb_width - 2i32 {
                (*h).mb.mv_min_spel[1usize] =
                    if 4i32 * (-(8i32) * (*h).mb.i_mb_y - 12i32) > -mv_range {
                        4i32 * (-(8i32) * (*h).mb.i_mb_y - 12i32)
                    } else {
                        -mv_range
                    };
                (*h).mb.mv_max_spel[1usize] = if 4i32
                    * (8i32 * ((*h).mb.i_mb_height - (*h).mb.i_mb_y - 1i32) + 12i32)
                    < mv_range - 1i32
                {
                    4i32 * (8i32 * ((*h).mb.i_mb_height - (*h).mb.i_mb_y - 1i32) + 12i32)
                } else {
                    mv_range - 1i32
                };
                (*h).mb.mv_limit_fpel[0usize][1usize] =
                    ((*h).mb.mv_min_spel[1usize] >> 2i32) as crate::stdlib::int16_t;
                (*h).mb.mv_limit_fpel[1usize][1usize] =
                    ((*h).mb.mv_max_spel[1usize] >> 2i32) as crate::stdlib::int16_t;
            }
            m[0usize].i_pixel = crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int;
            m[0usize].p_cost_mv = (*a).p_cost_mv;
            m[0usize].i_stride[0usize] = i_stride;
            m[0usize].p_fenc[0usize] = (*h).mb.pic.p_fenc[0usize];
            m[0usize].weight = w;
            m[0usize].i_ref = 0i32;
            m[0usize].p_fref[0usize] = (*(&raw mut (*fref0).lowres
                as *mut *mut crate::src::common::common::pixel)
                .offset(0isize))
            .offset(i_pel_offset as isize);
            m[0usize].p_fref[1usize] = (*(&raw mut (*fref0).lowres
                as *mut *mut crate::src::common::common::pixel)
                .offset(1isize))
            .offset(i_pel_offset as isize);
            m[0usize].p_fref[2usize] = (*(&raw mut (*fref0).lowres
                as *mut *mut crate::src::common::common::pixel)
                .offset(2isize))
            .offset(i_pel_offset as isize);
            m[0usize].p_fref[3usize] = (*(&raw mut (*fref0).lowres
                as *mut *mut crate::src::common::common::pixel)
                .offset(3isize))
            .offset(i_pel_offset as isize);
            m[0usize].p_fref_w = m[0usize].p_fref[0usize];
            if !(*w.offset(0isize)).weightfn.is_null() {
                m[0usize].p_fref_w = (*(&raw mut (*fenc).weighted
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(0isize))
                .offset(i_pel_offset as isize);
            }
            if b_bidir != 0 {
                let mut dmv = [[0; 2]; 2];
                m[1usize].i_pixel = crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int;
                m[1usize].p_cost_mv = (*a).p_cost_mv;
                m[1usize].i_stride[0usize] = i_stride;
                m[1usize].p_fenc[0usize] = (*h).mb.pic.p_fenc[0usize];
                m[1usize].i_ref = 0i32;
                m[1usize].weight = &raw mut crate::src::common::tables::x264_zero
                    as *const crate::src::common::mc::x264_weight_t;
                m[1usize].p_fref[0usize] = (*(&raw mut (*fref1).lowres
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(0isize))
                .offset(i_pel_offset as isize);
                m[1usize].p_fref[1usize] = (*(&raw mut (*fref1).lowres
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset(i_pel_offset as isize);
                m[1usize].p_fref[2usize] = (*(&raw mut (*fref1).lowres
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(2isize))
                .offset(i_pel_offset as isize);
                m[1usize].p_fref[3usize] = (*(&raw mut (*fref1).lowres
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(3isize))
                .offset(i_pel_offset as isize);
                m[1usize].p_fref_w = m[1usize].p_fref[0usize];
                if (*(*fref1).lowres_mvs[0usize][(p1 - p0 - 1i32) as usize].offset(0isize))[0usize]
                    as ::core::ffi::c_int
                    != 0x7fffi32
                {
                    let mut mvr = &raw mut *(*(&raw mut *(&raw mut (*fref1).lowres_mvs
                        as *mut [*mut [crate::stdlib::int16_t; 2]; 17])
                        .offset(0isize)
                        as *mut *mut [crate::stdlib::int16_t; 2])
                        .offset((p1 - p0 - 1i32) as isize))
                    .offset(i_mb_xy as isize)
                        as *mut crate::stdlib::int16_t;
                    dmv[0usize][0usize] =
                        ((*mvr.offset(0isize) as ::core::ffi::c_int * dist_scale_factor + 128i32)
                            >> 8i32) as crate::stdlib::int16_t;
                    dmv[0usize][1usize] =
                        ((*mvr.offset(1isize) as ::core::ffi::c_int * dist_scale_factor + 128i32)
                            >> 8i32) as crate::stdlib::int16_t;
                    dmv[1usize][0usize] = (dmv[0usize][0usize] as ::core::ffi::c_int
                        - *mvr.offset(0isize) as ::core::ffi::c_int)
                        as crate::stdlib::int16_t;
                    dmv[1usize][1usize] = (dmv[0usize][1usize] as ::core::ffi::c_int
                        - *mvr.offset(1isize) as ::core::ffi::c_int)
                        as crate::stdlib::int16_t;
                    dmv[0usize][0usize] = x264_clip3(
                        dmv[0usize][0usize] as ::core::ffi::c_int,
                        (*h).mb.mv_min_spel[0usize],
                        (*h).mb.mv_max_spel[0usize],
                    ) as crate::stdlib::int16_t;
                    dmv[0usize][1usize] = x264_clip3(
                        dmv[0usize][1usize] as ::core::ffi::c_int,
                        (*h).mb.mv_min_spel[1usize],
                        (*h).mb.mv_max_spel[1usize],
                    ) as crate::stdlib::int16_t;
                    dmv[1usize][0usize] = x264_clip3(
                        dmv[1usize][0usize] as ::core::ffi::c_int,
                        (*h).mb.mv_min_spel[0usize],
                        (*h).mb.mv_max_spel[0usize],
                    ) as crate::stdlib::int16_t;
                    dmv[1usize][1usize] = x264_clip3(
                        dmv[1usize][1usize] as ::core::ffi::c_int,
                        (*h).mb.mv_min_spel[1usize],
                        (*h).mb.mv_max_spel[1usize],
                    ) as crate::stdlib::int16_t;
                    if (*h).param.analyse.i_subpel_refine <= 1i32 {
                        let ref mut c2rust_fresh20 =
                            (*(&raw mut dmv as *mut crate::src::common::base::x264_union64_t)).i;
                        *c2rust_fresh20 &= !(0x1000100010001u64);
                    }
                } else {
                    (*(&raw mut dmv as *mut crate::src::common::base::x264_union64_t)).i = 0u64;
                }
                if (*h).param.analyse.i_subpel_refine <= 1i32 {
                    let mut hpel_idx1 = ((dmv[0usize][0usize] as ::core::ffi::c_int & 2i32)
                        >> 1i32)
                        + (dmv[0usize][1usize] as ::core::ffi::c_int & 2i32);
                    let mut hpel_idx2 = ((dmv[1usize][0usize] as ::core::ffi::c_int & 2i32)
                        >> 1i32)
                        + (dmv[1usize][1usize] as ::core::ffi::c_int & 2i32);
                    let mut src1 = m[0usize].p_fref[hpel_idx1 as usize]
                        .offset((dmv[0usize][0usize] as ::core::ffi::c_int >> 2i32) as isize)
                        .offset(
                            ((dmv[0usize][1usize] as ::core::ffi::c_int >> 2i32)
                                * m[0usize].i_stride[0usize]) as isize,
                        );
                    let mut src2 = m[1usize].p_fref[hpel_idx2 as usize]
                        .offset((dmv[1usize][0usize] as ::core::ffi::c_int >> 2i32) as isize)
                        .offset(
                            ((dmv[1usize][1usize] as ::core::ffi::c_int >> 2i32)
                                * m[1usize].i_stride[0usize]) as isize,
                        );
                    (*h).mc.avg
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        16isize,
                        src1,
                        m[0usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        src2,
                        m[1usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        i_bipred_weight,
                    );
                } else {
                    let mut stride1 = 16isize;
                    let mut stride2 = 16isize;
                    let mut src1_0 = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        &raw mut stride1,
                        &raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                            .offset(0isize))
                        .p_fref
                            as *mut *mut crate::src::common::common::pixel,
                        m[0usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        dmv[0usize][0usize] as ::core::ffi::c_int,
                        dmv[0usize][1usize] as ::core::ffi::c_int,
                        8i32,
                        8i32,
                        w,
                    );
                    let mut src2_0 = (*h).mc.get_ref.expect("non-null function pointer")(
                        pix2,
                        &raw mut stride2,
                        &raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                            .offset(1isize))
                        .p_fref
                            as *mut *mut crate::src::common::common::pixel,
                        m[1usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        dmv[1usize][0usize] as ::core::ffi::c_int,
                        dmv[1usize][1usize] as ::core::ffi::c_int,
                        8i32,
                        8i32,
                        w,
                    );
                    (*h).mc.avg
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        16isize,
                        src1_0,
                        stride1,
                        src2_0,
                        stride2,
                        i_bipred_weight,
                    );
                }
                let mut i_cost = 0i32 * (*a).i_lambda
                    + (*h).pixf.mbcmp
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        m[0usize].p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        16isize,
                    );
                if i_cost < i_bcost {
                    i_bcost = i_cost;
                    list_used = 3i32;
                }
                if (*(&raw mut dmv as *mut crate::src::common::base::x264_union64_t)).i != 0 {
                    (*h).mc.avg
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        16isize,
                        m[0usize].p_fref[0usize],
                        m[0usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        m[1usize].p_fref[0usize],
                        m[1usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        i_bipred_weight,
                    );
                    let mut i_cost_0 = (*h).pixf.mbcmp
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        m[0usize].p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        16isize,
                    );
                    if i_cost_0 < i_bcost {
                        i_bcost = i_cost_0;
                        list_used = 3i32;
                    }
                }
            }
            while l < 1i32 + b_bidir {
                if *do_search.offset(l as isize) != 0 {
                    let mut c2rust_current_block_117: u64;
                    let mut i_mvc = 0i32;
                    let mut mvc = [[0; 2]; 4];
                    let mut fenc_mv = fenc_mvs[l as usize];
                    (*(&raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                            .offset(0isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                    (*(&raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                            .offset(2isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                    if i_mb_x < (*h).mb.i_mb_width - 1i32 {
                        (*(&raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                            .offset(i_mvc as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*(&raw mut *fenc_mv.offset(1isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                        i_mvc += 1;
                    }
                    if i_mb_y < (*h).i_threadslice_end - 1i32 {
                        (*(&raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                            .offset(i_mvc as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*(&raw mut *fenc_mv.offset(i_mb_stride as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                        i_mvc += 1;
                        if i_mb_x > 0i32 {
                            (*(&raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                                .offset(i_mvc as isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i = (*(&raw mut *fenc_mv.offset((i_mb_stride - 1i32) as isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i;
                            i_mvc += 1;
                        }
                        if i_mb_x < (*h).mb.i_mb_width - 1i32 {
                            (*(&raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                                .offset(i_mvc as isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i = (*(&raw mut *fenc_mv.offset((i_mb_stride + 1i32) as isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i;
                            i_mvc += 1;
                        }
                    }
                    if i_mvc <= 1i32 {
                        (*(&raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                            .offset(l as isize))
                        .mvp
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*(&raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                            .offset(0isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                    } else {
                        x264_median_mv(
                            &raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                                .offset(l as isize))
                            .mvp as *mut crate::stdlib::int16_t,
                            &raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                                .offset(0isize)
                                as *mut crate::stdlib::int16_t,
                            &raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                                .offset(1isize)
                                as *mut crate::stdlib::int16_t,
                            &raw mut *(&raw mut mvc as *mut [crate::stdlib::int16_t; 2])
                                .offset(2isize)
                                as *mut crate::stdlib::int16_t,
                        );
                    }
                    if (*(&raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                        .offset(l as isize))
                    .mvp
                        as *mut crate::src::common::base::x264_union32_t))
                        .i
                        == 0
                    {
                        m[l as usize].cost = (*h).pixf.mbcmp
                            [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                            .expect("non-null function pointer")(
                            m[l as usize].p_fenc[0usize],
                            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                            m[l as usize].p_fref[0usize],
                            m[l as usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        );
                        if m[l as usize].cost < 64i32 {
                            (*(&raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                                .offset(l as isize))
                            .mv
                                as *mut crate::src::common::base::x264_union32_t))
                                .i = 0u32;
                            c2rust_current_block_117 = 9949278710472276570;
                        } else {
                            c2rust_current_block_117 = 17958840340921835115;
                        }
                    } else {
                        c2rust_current_block_117 = 17958840340921835115;
                    }
                    match c2rust_current_block_117 {
                        17958840340921835115 => {
                            crate::src::encoder::me::x264_8_me_search_ref(
                                h,
                                (&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                                    .offset(l as isize),
                                &raw mut mvc as *mut [crate::stdlib::int16_t; 2],
                                i_mvc,
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            );
                            m[l as usize].cost -=
                                *(*a).p_cost_mv.offset(0isize) as ::core::ffi::c_int;
                            if (*(&raw mut (*(&raw mut m
                                as *mut crate::src::encoder::me::x264_me_t)
                                .offset(l as isize))
                            .mv
                                as *mut crate::src::common::base::x264_union32_t))
                                .i
                                != 0
                            {
                                m[l as usize].cost += 5i32 * (*a).i_lambda;
                            }
                        }
                        _ => {}
                    }
                    (*(fenc_mvs[l as usize] as *mut crate::src::common::base::x264_union32_t)).i =
                        (*(&raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                            .offset(l as isize))
                        .mv
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                    *fenc_costs[l as usize] = m[l as usize].cost;
                } else {
                    (*(&raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                        .offset(l as isize))
                    .mv as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(fenc_mvs[l as usize]
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    m[l as usize].cost = *fenc_costs[l as usize];
                }
                if m[l as usize].cost < i_bcost {
                    i_bcost = m[l as usize].cost;
                    list_used = l + 1i32;
                }
                l += 1;
            }
            if b_bidir != 0
                && ((*(&raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                    .offset(0isize))
                .mv as *mut crate::src::common::base::x264_union32_t))
                    .i
                    != 0
                    || (*(&raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                        .offset(1isize))
                    .mv
                        as *mut crate::src::common::base::x264_union32_t))
                        .i
                        != 0)
            {
                if (*h).param.analyse.i_subpel_refine <= 1i32 {
                    let mut hpel_idx1_0 = ((m[0usize].mv[0usize] as ::core::ffi::c_int & 2i32)
                        >> 1i32)
                        + (m[0usize].mv[1usize] as ::core::ffi::c_int & 2i32);
                    let mut hpel_idx2_0 = ((m[1usize].mv[0usize] as ::core::ffi::c_int & 2i32)
                        >> 1i32)
                        + (m[1usize].mv[1usize] as ::core::ffi::c_int & 2i32);
                    let mut src1_1 = m[0usize].p_fref[hpel_idx1_0 as usize]
                        .offset((m[0usize].mv[0usize] as ::core::ffi::c_int >> 2i32) as isize)
                        .offset(
                            ((m[0usize].mv[1usize] as ::core::ffi::c_int >> 2i32)
                                * m[0usize].i_stride[0usize]) as isize,
                        );
                    let mut src2_1 = m[1usize].p_fref[hpel_idx2_0 as usize]
                        .offset((m[1usize].mv[0usize] as ::core::ffi::c_int >> 2i32) as isize)
                        .offset(
                            ((m[1usize].mv[1usize] as ::core::ffi::c_int >> 2i32)
                                * m[1usize].i_stride[0usize]) as isize,
                        );
                    (*h).mc.avg
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        16isize,
                        src1_1,
                        m[0usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        src2_1,
                        m[1usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        i_bipred_weight,
                    );
                } else {
                    let mut stride1_0 = 16isize;
                    let mut stride2_0 = 16isize;
                    let mut src1_2 = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        &raw mut stride1_0,
                        &raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                            .offset(0isize))
                        .p_fref
                            as *mut *mut crate::src::common::common::pixel,
                        m[0usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        m[0usize].mv[0usize] as ::core::ffi::c_int,
                        m[0usize].mv[1usize] as ::core::ffi::c_int,
                        8i32,
                        8i32,
                        w,
                    );
                    let mut src2_2 = (*h).mc.get_ref.expect("non-null function pointer")(
                        pix2,
                        &raw mut stride2_0,
                        &raw mut (*(&raw mut m as *mut crate::src::encoder::me::x264_me_t)
                            .offset(1isize))
                        .p_fref
                            as *mut *mut crate::src::common::common::pixel,
                        m[1usize].i_stride[0usize] as crate::stdlib::intptr_t,
                        m[1usize].mv[0usize] as ::core::ffi::c_int,
                        m[1usize].mv[1usize] as ::core::ffi::c_int,
                        8i32,
                        8i32,
                        w,
                    );
                    (*h).mc.avg
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        16isize,
                        src1_2,
                        stride1_0,
                        src2_2,
                        stride2_0,
                        i_bipred_weight,
                    );
                }
                let mut i_cost_1 = 5i32 * (*a).i_lambda
                    + (*h).pixf.mbcmp
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        m[0usize].p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        &raw mut pix1 as *mut crate::src::common::common::pixel,
                        16isize,
                    );
                if i_cost_1 < i_bcost {
                    i_bcost = i_cost_1;
                    list_used = 3i32;
                }
            }
        }
        if !(*fenc).intra_calculated {
            let mut satds = [0; 3];
            let mut pix = (&raw mut pix1 as *mut crate::src::common::common::pixel)
                .offset((8i32 + crate::src::common::common::FDEC_STRIDE) as isize);
            let mut src = (*(&raw mut (*fenc).lowres
                as *mut *mut crate::src::common::common::pixel)
                .offset(0isize))
            .offset(i_pel_offset as isize);
            let intra_penalty = 5i32 * (*a).i_lambda;
            let mut pixoff = 4i32 / crate::src::common::common::SIZEOF_PIXEL;
            crate::stdlib::memcpy(
                pix.offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *mut ::core::ffi::c_void,
                src.offset(-(i_stride as isize)) as *const ::core::ffi::c_void,
                (16i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            let mut i = -(1i32);
            while i < 8i32 {
                (*(pix.offset((i * 32i32 - pixoff) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(src.offset((i * i_stride - pixoff) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
                i += 1;
            }
            (*h).pixf
                .intra_mbcmp_x3_8x8c
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[0usize],
                pix,
                &raw mut satds as *mut ::core::ffi::c_int,
            );
            let mut i_icost = if satds[0usize]
                < (if satds[1usize] < satds[2usize] {
                    satds[1usize]
                } else {
                    satds[2usize]
                }) {
                satds[0usize]
            } else if satds[1usize] < satds[2usize] {
                satds[1usize]
            } else {
                satds[2usize]
            };
            if (*h).param.analyse.i_subpel_refine > 1i32 {
                let mut edge = [0; 36];
                let mut i_0 = 3i32;
                (*h).predict_8x8c
                    [crate::src::common::predict::I_PRED_CHROMA_P as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(pix);
                let mut satd = (*h).pixf.mbcmp
                    [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fenc[0usize],
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    pix,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                );
                i_icost = if i_icost < satd { i_icost } else { satd };
                (*h).predict_8x8_filter.expect("non-null function pointer")(
                    pix,
                    &raw mut edge as *mut crate::src::common::common::pixel,
                    crate::src::common::macroblock::ALL_NEIGHBORS as ::core::ffi::c_int,
                    crate::src::common::macroblock::ALL_NEIGHBORS as ::core::ffi::c_int,
                );
                while i_0 < 9i32 {
                    (*h).predict_8x8[i_0 as usize].expect("non-null function pointer")(
                        pix,
                        &raw mut edge as *mut crate::src::common::common::pixel,
                    );
                    satd = (*h).pixf.mbcmp
                        [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        (*h).mb.pic.p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        pix,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    );
                    i_icost = if i_icost < satd { i_icost } else { satd };
                    i_0 += 1;
                }
            }
            i_icost =
                ((i_icost + intra_penalty) >> (crate::internal::BIT_DEPTH - 8i32)) + lowres_penalty;
            *(*fenc).i_intra_cost.offset(i_mb_xy as isize) = i_icost as crate::stdlib::uint16_t;
            let mut i_icost_aq = i_icost;
            if (*h).param.rc.i_aq_mode != 0 {
                i_icost_aq = (i_icost_aq
                    * *(*fenc).i_inv_qscale_factor.offset(i_mb_xy as isize) as ::core::ffi::c_int
                    + 128i32)
                    >> 8i32;
            }
            *output_intra.offset(
                (crate::slicetype_c::NUM_INTS + ((*h).mb.i_mb_y - (*h).i_threadslice_start))
                    as isize,
            ) += i_icost_aq;
            if b_frame_score_mb != 0 {
                *output_intra.offset(crate::slicetype_c::COST_EST as isize) += i_icost;
                *output_intra.offset(crate::slicetype_c::COST_EST_AQ as isize) += i_icost_aq;
            }
        }
        i_bcost = (i_bcost >> (crate::internal::BIT_DEPTH - 8i32)) + lowres_penalty;
        if b_bidir == 0 {
            let mut i_icost_0 =
                *(*fenc).i_intra_cost.offset(i_mb_xy as isize) as ::core::ffi::c_int;
            let mut b_intra = (i_icost_0 < i_bcost) as ::core::ffi::c_int;
            if b_intra != 0 {
                i_bcost = i_icost_0;
                list_used = 0i32;
            }
            if b_frame_score_mb != 0 {
                *output_inter.offset(crate::slicetype_c::INTRA_MBS as isize) += b_intra;
            }
        }
        if p0 != p1 {
            let mut i_bcost_aq = i_bcost;
            if (*h).param.rc.i_aq_mode != 0 {
                i_bcost_aq = (i_bcost_aq
                    * *(*fenc).i_inv_qscale_factor.offset(i_mb_xy as isize) as ::core::ffi::c_int
                    + 128i32)
                    >> 8i32;
            }
            *output_inter.offset(
                (crate::slicetype_c::NUM_INTS + ((*h).mb.i_mb_y - (*h).i_threadslice_start))
                    as isize,
            ) += i_bcost_aq;
            if b_frame_score_mb != 0 {
                *output_inter.offset(crate::slicetype_c::COST_EST as isize) += i_bcost;
                *output_inter.offset(crate::slicetype_c::COST_EST_AQ as isize) += i_bcost_aq;
            }
        }
        *(*fenc).lowres_costs[(b - p0) as usize][(p1 - b) as usize].offset(i_mb_xy as isize) =
            ((if i_bcost < ((1i32) << 14i32) - 1i32 {
                i_bcost
            } else {
                ((1i32) << 14i32) - 1i32
            }) + (list_used << crate::src::common::frame::LOWRES_COST_SHIFT))
                as crate::stdlib::uint16_t;
    }
}
pub unsafe extern "C" fn slicetype_slice_cost(
    mut s: *mut crate::slicetype_c::x264_slicetype_slice_t,
) {
    unsafe {
        let mut h = (*s).h;
        let mut do_edges = ((*h).param.rc.mb_tree
            || (*h).param.rc.i_vbv_buffer_size != 0
            || (*h).mb.i_mb_width <= 2i32
            || (*h).mb.i_mb_height <= 2i32) as ::core::ffi::c_int;
        let mut start_y = if ((*h).i_threadslice_end - 1i32) < (*h).mb.i_mb_height - 2i32 + do_edges
        {
            (*h).i_threadslice_end - 1i32
        } else {
            (*h).mb.i_mb_height - 2i32 + do_edges
        };
        let mut end_y = if (*h).i_threadslice_start > 1i32 - do_edges {
            (*h).i_threadslice_start
        } else {
            1i32 - do_edges
        };
        let mut start_x = (*h).mb.i_mb_width - 2i32 + do_edges;
        let mut end_x = 1i32 - do_edges;
        (*h).mb.i_mb_y = start_y;
        while (*h).mb.i_mb_y >= end_y {
            (*h).mb.i_mb_x = start_x;
            while (*h).mb.i_mb_x >= end_x {
                slicetype_mb_cost(
                    h,
                    (*s).a,
                    (*s).frames,
                    (*s).p0,
                    (*s).p1,
                    (*s).b,
                    (*s).dist_scale_factor,
                    (*s).do_search,
                    (*s).w,
                    (*s).output_inter,
                    (*s).output_intra,
                );
                (*h).mb.i_mb_x -= 1;
            }
            (*h).mb.i_mb_y -= 1;
        }
    }
}
pub unsafe extern "C" fn slicetype_frame_cost(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut p0: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_score = 0i32;
        let mut w = &raw mut crate::src::common::tables::x264_zero
            as *const crate::src::common::mc::x264_weight_t;
        let mut fenc = *frames.offset(b as isize);
        if (*fenc).i_cost_est[(b - p0) as usize][(p1 - b) as usize] >= 0i32
            && ((*h).param.rc.i_vbv_buffer_size == 0
                || *(*fenc).i_row_satds[(b - p0) as usize][(p1 - b) as usize].offset(0isize)
                    != -(1i32))
        {
            i_score = (*fenc).i_cost_est[(b - p0) as usize][(p1 - b) as usize];
        } else {
            let mut do_search = [0; 2];
            let mut dist_scale_factor = 128i32;
            let mut output_inter = [::core::ptr::null_mut::<::core::ffi::c_int>(); 17];
            let mut output_intra = [::core::ptr::null_mut::<::core::ffi::c_int>(); 17];
            let mut i_1 = 0i32;
            do_search[0usize] = (b != p0
                && (*(*fenc).lowres_mvs[0usize][(b - p0 - 1i32) as usize].offset(0isize))[0usize]
                    as ::core::ffi::c_int
                    == 0x7fffi32) as ::core::ffi::c_int;
            do_search[1usize] = (b != p1
                && (*(*fenc).lowres_mvs[1usize][(p1 - b - 1i32) as usize].offset(0isize))[0usize]
                    as ::core::ffi::c_int
                    == 0x7fffi32) as ::core::ffi::c_int;
            if do_search[0usize] != 0 {
                if (*h).param.analyse.i_weighted_pred != 0 && b == p1 {
                    x264_8_weights_analyse(h, fenc, *frames.offset(p0 as isize), 1i32);
                    w = &raw mut *(&raw mut (*fenc).weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(0isize)
                        as *mut crate::src::common::mc::x264_weight_t;
                }
                (*(*fenc).lowres_mvs[0usize][(b - p0 - 1i32) as usize].offset(0isize))[0usize] =
                    0i16;
            }
            if do_search[1usize] != 0 {
                (*(*fenc).lowres_mvs[1usize][(p1 - b - 1i32) as usize].offset(0isize))[0usize] =
                    0i16;
            }
            if p1 != p0 {
                dist_scale_factor = (((b - p0) << 8i32) + ((p1 - p0) >> 1i32)) / (p1 - p0);
            }
            let mut output_buf_size = (*h).mb.i_mb_height
                + (crate::slicetype_c::NUM_INTS + crate::slicetype_c::PAD_SIZE)
                    * (*h).param.i_lookahead_threads;
            output_inter[0usize] = (*h).scratch_buffer2 as *mut ::core::ffi::c_int;
            output_intra[0usize] = output_inter[0usize].offset(output_buf_size as isize);
            if (*h).param.i_lookahead_threads > 1i32 {
                let mut s = [crate::slicetype_c::x264_slicetype_slice_t {
                    h: ::core::ptr::null_mut::<crate::src::common::common::x264_t>(),
                    a: ::core::ptr::null_mut::<x264_mb_analysis_t>(),
                    frames: ::core::ptr::null_mut::<*mut crate::src::common::frame::x264_frame_t>(),
                    p0: 0,
                    p1: 0,
                    b: 0,
                    dist_scale_factor: 0,
                    do_search: ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    w: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    output_inter: ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    output_intra: ::core::ptr::null_mut::<::core::ffi::c_int>(),
                }; 16];
                let mut i = 0i32;
                let mut i_0 = 0i32;
                while i < (*h).param.i_lookahead_threads {
                    let mut t = (*h).lookahead_thread[i as usize];
                    (*t).mb.i_me_method = (*h).mb.i_me_method;
                    (*t).mb.i_subpel_refine = (*h).mb.i_subpel_refine;
                    (*t).mb.chroma_me = (*h).mb.chroma_me;
                    s[i as usize] = crate::slicetype_c::x264_slicetype_slice_t {
                        h: t,
                        a: a,
                        frames: frames,
                        p0: p0,
                        p1: p1,
                        b: b,
                        dist_scale_factor: dist_scale_factor,
                        do_search: &raw mut do_search as *mut ::core::ffi::c_int,
                        w: w,
                        output_inter: output_inter[i as usize],
                        output_intra: output_intra[i as usize],
                    };
                    (*t).i_threadslice_start = ((*h).mb.i_mb_height * i
                        + (*h).param.i_lookahead_threads / 2i32)
                        / (*h).param.i_lookahead_threads;
                    (*t).i_threadslice_end = ((*h).mb.i_mb_height * (i + 1i32)
                        + (*h).param.i_lookahead_threads / 2i32)
                        / (*h).param.i_lookahead_threads;
                    let mut thread_height = (*t).i_threadslice_end - (*t).i_threadslice_start;
                    let mut thread_output_size = thread_height + crate::slicetype_c::NUM_INTS;
                    crate::stdlib::memset(
                        output_inter[i as usize] as *mut ::core::ffi::c_void,
                        0i32,
                        (thread_output_size as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
                    );
                    crate::stdlib::memset(
                        output_intra[i as usize] as *mut ::core::ffi::c_void,
                        0i32,
                        (thread_output_size as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
                    );
                    let ref mut c2rust_fresh18 =
                        *output_intra[i as usize].offset(crate::slicetype_c::NUM_ROWS as isize);
                    *c2rust_fresh18 = thread_height;
                    *output_inter[i as usize].offset(crate::slicetype_c::NUM_ROWS as isize) =
                        *c2rust_fresh18;
                    output_inter[(i + 1i32) as usize] = output_inter[i as usize]
                        .offset(thread_output_size as isize)
                        .offset(crate::slicetype_c::PAD_SIZE as isize);
                    output_intra[(i + 1i32) as usize] = output_intra[i as usize]
                        .offset(thread_output_size as isize)
                        .offset(crate::slicetype_c::PAD_SIZE as isize);
                    crate::src::common::threadpool::x264_8_threadpool_run(
                        (*h).lookaheadpool,
                        ::core::mem::transmute::<
                            *mut ::core::ffi::c_void,
                            Option<
                                unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                )
                                    -> *mut ::core::ffi::c_void,
                            >,
                        >(::core::mem::transmute::<
                            Option<
                                unsafe extern "C" fn(
                                    *mut crate::slicetype_c::x264_slicetype_slice_t,
                                ) -> (),
                            >,
                            *mut ::core::ffi::c_void,
                        >(Some(
                            slicetype_slice_cost
                                as unsafe extern "C" fn(
                                    *mut crate::slicetype_c::x264_slicetype_slice_t,
                                ) -> (),
                        ))),
                        (&raw mut s as *mut crate::slicetype_c::x264_slicetype_slice_t)
                            .offset(i as isize) as *mut ::core::ffi::c_void,
                    );
                    i += 1;
                }
                while i_0 < (*h).param.i_lookahead_threads {
                    crate::src::common::threadpool::x264_8_threadpool_wait(
                        (*h).lookaheadpool,
                        (&raw mut s as *mut crate::slicetype_c::x264_slicetype_slice_t)
                            .offset(i_0 as isize)
                            as *mut ::core::ffi::c_void,
                    );
                    i_0 += 1;
                }
            } else {
                (*h).i_threadslice_start = 0i32;
                (*h).i_threadslice_end = (*h).mb.i_mb_height;
                crate::stdlib::memset(
                    output_inter[0usize] as *mut ::core::ffi::c_void,
                    0i32,
                    ((output_buf_size - crate::slicetype_c::PAD_SIZE)
                        as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
                );
                crate::stdlib::memset(
                    output_intra[0usize] as *mut ::core::ffi::c_void,
                    0i32,
                    ((output_buf_size - crate::slicetype_c::PAD_SIZE)
                        as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
                );
                let ref mut c2rust_fresh19 =
                    *output_intra[0usize].offset(crate::slicetype_c::NUM_ROWS as isize);
                *c2rust_fresh19 = (*h).mb.i_mb_height;
                *output_inter[0usize].offset(crate::slicetype_c::NUM_ROWS as isize) =
                    *c2rust_fresh19;
                let mut s_0 = crate::slicetype_c::x264_slicetype_slice_t {
                    h: h,
                    a: a,
                    frames: frames,
                    p0: p0,
                    p1: p1,
                    b: b,
                    dist_scale_factor: dist_scale_factor,
                    do_search: &raw mut do_search as *mut ::core::ffi::c_int,
                    w: w,
                    output_inter: output_inter[0usize],
                    output_intra: output_intra[0usize],
                };
                slicetype_slice_cost(&raw mut s_0);
            }
            if b == p1 {
                (*fenc).i_intra_mbs[(b - p0) as usize] = 0i32;
            }
            if !(*fenc).intra_calculated {
                (*fenc).i_cost_est[0usize][0usize] = 0i32;
                (*fenc).i_cost_est_aq[0usize][0usize] = 0i32;
            }
            (*fenc).i_cost_est[(b - p0) as usize][(p1 - b) as usize] = 0i32;
            (*fenc).i_cost_est_aq[(b - p0) as usize][(p1 - b) as usize] = 0i32;
            let mut row_satd_inter = (*fenc).i_row_satds[(b - p0) as usize][(p1 - b) as usize];
            let mut row_satd_intra = (*fenc).i_row_satds[0usize][0usize];
            while i_1 < (*h).param.i_lookahead_threads {
                if b == p1 {
                    (*fenc).i_intra_mbs[(b - p0) as usize] +=
                        *output_inter[i_1 as usize].offset(crate::slicetype_c::INTRA_MBS as isize);
                }
                if !(*fenc).intra_calculated {
                    (*fenc).i_cost_est[0usize][0usize] +=
                        *output_intra[i_1 as usize].offset(crate::slicetype_c::COST_EST as isize);
                    (*fenc).i_cost_est_aq[0usize][0usize] += *output_intra[i_1 as usize]
                        .offset(crate::slicetype_c::COST_EST_AQ as isize);
                }
                (*fenc).i_cost_est[(b - p0) as usize][(p1 - b) as usize] +=
                    *output_inter[i_1 as usize].offset(crate::slicetype_c::COST_EST as isize);
                (*fenc).i_cost_est_aq[(b - p0) as usize][(p1 - b) as usize] +=
                    *output_inter[i_1 as usize].offset(crate::slicetype_c::COST_EST_AQ as isize);
                if (*h).param.rc.i_vbv_buffer_size != 0 {
                    let mut row_count =
                        *output_inter[i_1 as usize].offset(crate::slicetype_c::NUM_ROWS as isize);
                    crate::stdlib::memcpy(
                        row_satd_inter as *mut ::core::ffi::c_void,
                        output_inter[i_1 as usize].offset(crate::slicetype_c::NUM_INTS as isize)
                            as *const ::core::ffi::c_void,
                        (row_count as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
                    );
                    if !(*fenc).intra_calculated {
                        crate::stdlib::memcpy(
                            row_satd_intra as *mut ::core::ffi::c_void,
                            output_intra[i_1 as usize].offset(crate::slicetype_c::NUM_INTS as isize)
                                as *const ::core::ffi::c_void,
                            (row_count as crate::__stddef_size_t_h::size_t)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
                        );
                    }
                    row_satd_inter = row_satd_inter.offset(row_count as isize);
                    row_satd_intra = row_satd_intra.offset(row_count as isize);
                }
                i_1 += 1;
            }
            i_score = (*fenc).i_cost_est[(b - p0) as usize][(p1 - b) as usize];
            if b != p1 {
                i_score = (i_score as crate::stdlib::uint64_t)
                    .wrapping_mul(100u64)
                    .wrapping_div((120i32 + (*h).param.i_bframe_bias) as crate::stdlib::uint64_t)
                    as ::core::ffi::c_int;
            } else {
                (*fenc).intra_calculated = true;
            }
            (*fenc).i_cost_est[(b - p0) as usize][(p1 - b) as usize] = i_score;
        }
        i_score
    }
}
pub unsafe extern "C" fn slicetype_frame_cost_recalculate(
    mut h: *mut crate::src::common::common::x264_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut p0: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_score = 0i32;
        let mut row_satd =
            (**frames.offset(b as isize)).i_row_satds[(b - p0) as usize][(p1 - b) as usize];
        let mut qp_offset = if (**frames.offset(b as isize)).i_type == crate::x264_h::X264_TYPE_B
            || (**frames.offset(b as isize)).i_type == crate::x264_h::X264_TYPE_BREF
        {
            (**frames.offset(b as isize)).f_qp_offset_aq
        } else {
            (**frames.offset(b as isize)).f_qp_offset
        };
        (*h).mb.i_mb_y = (*h).mb.i_mb_height - 1i32;
        while (*h).mb.i_mb_y >= 0i32 {
            *row_satd.offset((*h).mb.i_mb_y as isize) = 0i32;
            (*h).mb.i_mb_x = (*h).mb.i_mb_width - 1i32;
            while (*h).mb.i_mb_x >= 0i32 {
                let mut i_mb_xy = (*h).mb.i_mb_x + (*h).mb.i_mb_y * (*h).mb.i_mb_stride;
                let mut i_mb_cost = *(**frames.offset(b as isize)).lowres_costs[(b - p0) as usize]
                    [(p1 - b) as usize]
                    .offset(i_mb_xy as isize)
                    as ::core::ffi::c_int
                    & crate::src::common::frame::LOWRES_COST_MASK;
                let mut qp_adj = *qp_offset.offset(i_mb_xy as isize);
                i_mb_cost = (i_mb_cost * x264_exp2fix8(qp_adj) + 128i32) >> 8i32;
                *row_satd.offset((*h).mb.i_mb_y as isize) += i_mb_cost;
                if (*h).mb.i_mb_y > 0i32
                    && (*h).mb.i_mb_y < (*h).mb.i_mb_height - 1i32
                    && (*h).mb.i_mb_x > 0i32
                    && (*h).mb.i_mb_x < (*h).mb.i_mb_width - 1i32
                    || (*h).mb.i_mb_width <= 2i32
                    || (*h).mb.i_mb_height <= 2i32
                {
                    i_score += i_mb_cost;
                }
                (*h).mb.i_mb_x -= 1;
            }
            (*h).mb.i_mb_y -= 1;
        }
        i_score
    }
}
pub unsafe extern "C" fn macroblock_tree_finish(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut average_duration: ::core::ffi::c_float,
    mut ref0_distance: ::core::ffi::c_int,
) {
    unsafe {
        let mut weightdelta = 0.0;
        let mut mb_index = 0i32;
        let mut fps_factor = crate::stdlib::round(
            x264_clip3f(
                average_duration as ::core::ffi::c_double,
                (0.01
                    / (((*h).param.i_frame_packing == 5i32) as ::core::ffi::c_int + 1i32)
                        as ::core::ffi::c_float) as ::core::ffi::c_double,
                (1.00
                    / (((*h).param.i_frame_packing == 5i32) as ::core::ffi::c_int + 1i32)
                        as ::core::ffi::c_float) as ::core::ffi::c_double,
            ) / x264_clip3f(
                (*frame).f_duration as ::core::ffi::c_double,
                (0.01
                    / (((*h).param.i_frame_packing == 5i32) as ::core::ffi::c_int + 1i32)
                        as ::core::ffi::c_float) as ::core::ffi::c_double,
                (1.00
                    / (((*h).param.i_frame_packing == 5i32) as ::core::ffi::c_int + 1i32)
                        as ::core::ffi::c_float) as ::core::ffi::c_double,
            ) * 256f64
                / crate::slicetype_c::MBTREE_PRECISION as ::core::ffi::c_double,
        ) as ::core::ffi::c_int;
        if ref0_distance != 0
            && (*frame).f_weighted_cost_delta[(ref0_distance - 1i32) as usize] > 0f32
        {
            weightdelta = (1.0f64
                - (*frame).f_weighted_cost_delta[(ref0_distance - 1i32) as usize]
                    as ::core::ffi::c_double) as ::core::ffi::c_float;
        }
        let mut strength = 5.0 * (1.0 - (*h).param.rc.f_qcompress);
        while mb_index < (*h).mb.i_mb_count {
            let mut intra_cost = (*(*frame).i_intra_cost.offset(mb_index as isize)
                as ::core::ffi::c_int
                * *(*frame).i_inv_qscale_factor.offset(mb_index as isize) as ::core::ffi::c_int
                + 128i32)
                >> 8i32;
            if intra_cost != 0 {
                let mut propagate_cost = (*(*frame).i_propagate_cost.offset(mb_index as isize)
                    as ::core::ffi::c_int
                    * fps_factor
                    + 128i32)
                    >> 8i32;
                let mut log2_ratio =
                    x264_log2((intra_cost + propagate_cost) as crate::stdlib::uint32_t)
                        - x264_log2(intra_cost as crate::stdlib::uint32_t)
                        + weightdelta;
                *(*frame).f_qp_offset.offset(mb_index as isize) =
                    *(*frame).f_qp_offset_aq.offset(mb_index as isize) - strength * log2_ratio;
            }
            mb_index += 1;
        }
    }
}
pub unsafe extern "C" fn macroblock_tree_propagate(
    mut h: *mut crate::src::common::common::x264_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut average_duration: ::core::ffi::c_float,
    mut p0: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut referenced: ::core::ffi::c_int,
) {
    unsafe {
        let mut ref_costs = [
            (**frames.offset(p0 as isize)).i_propagate_cost,
            (**frames.offset(p1 as isize)).i_propagate_cost,
        ];
        let mut dist_scale_factor = (((b - p0) << 8i32) + ((p1 - p0) >> 1i32)) / (p1 - p0);
        let mut i_bipred_weight = if (*h).param.analyse.weighted_bipred {
            64i32 - (dist_scale_factor >> 2i32)
        } else {
            32i32
        };
        let mut mvs = [
            if b != p0 {
                (**frames.offset(b as isize)).lowres_mvs[0usize][(b - p0 - 1i32) as usize]
            } else {
                ::core::ptr::null_mut::<[crate::stdlib::int16_t; 2]>()
            },
            if b != p1 {
                (**frames.offset(b as isize)).lowres_mvs[1usize][(p1 - b - 1i32) as usize]
            } else {
                ::core::ptr::null_mut::<[crate::stdlib::int16_t; 2]>()
            },
        ];
        let mut bipred_weights = [i_bipred_weight, 64i32 - i_bipred_weight];
        let mut buf = (*h).scratch_buffer as *mut crate::stdlib::int16_t;
        let mut propagate_cost = (**frames.offset(b as isize)).i_propagate_cost;
        let mut lowres_costs =
            (**frames.offset(b as isize)).lowres_costs[(b - p0) as usize][(p1 - b) as usize];
        let mut fps_factor = (x264_clip3f(
            (**frames.offset(b as isize)).f_duration as ::core::ffi::c_double,
            (0.01
                / (((*h).param.i_frame_packing == 5i32) as ::core::ffi::c_int + 1i32)
                    as ::core::ffi::c_float) as ::core::ffi::c_double,
            (1.00
                / (((*h).param.i_frame_packing == 5i32) as ::core::ffi::c_int + 1i32)
                    as ::core::ffi::c_float) as ::core::ffi::c_double,
        ) / (x264_clip3f(
            average_duration as ::core::ffi::c_double,
            (0.01
                / (((*h).param.i_frame_packing == 5i32) as ::core::ffi::c_int + 1i32)
                    as ::core::ffi::c_float) as ::core::ffi::c_double,
            (1.00
                / (((*h).param.i_frame_packing == 5i32) as ::core::ffi::c_int + 1i32)
                    as ::core::ffi::c_float) as ::core::ffi::c_double,
        ) * 256.0)
            * crate::slicetype_c::MBTREE_PRECISION as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        if referenced == 0 {
            crate::stdlib::memset(
                (**frames.offset(b as isize)).i_propagate_cost as *mut ::core::ffi::c_void,
                0i32,
                ((*h).mb.i_mb_width as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>()),
            );
        }
        (*h).mb.i_mb_y = 0i32;
        while (*h).mb.i_mb_y < (*h).mb.i_mb_height {
            let mut mb_index = (*h).mb.i_mb_y * (*h).mb.i_mb_stride;
            (*h).mc
                .mbtree_propagate_cost
                .expect("non-null function pointer")(
                buf,
                propagate_cost,
                (**frames.offset(b as isize))
                    .i_intra_cost
                    .offset(mb_index as isize),
                lowres_costs.offset(mb_index as isize),
                (**frames.offset(b as isize))
                    .i_inv_qscale_factor
                    .offset(mb_index as isize),
                &raw mut fps_factor,
                (*h).mb.i_mb_width,
            );
            if referenced != 0 {
                propagate_cost = propagate_cost.offset((*h).mb.i_mb_width as isize);
            }
            (*h).mc
                .mbtree_propagate_list
                .expect("non-null function pointer")(
                h,
                ref_costs[0usize],
                (*(&raw mut mvs as *mut *mut [crate::stdlib::int16_t; 2]).offset(0isize))
                    .offset(mb_index as isize),
                buf,
                lowres_costs.offset(mb_index as isize),
                bipred_weights[0usize],
                (*h).mb.i_mb_y,
                (*h).mb.i_mb_width,
                0i32,
            );
            if b != p1 {
                (*h).mc
                    .mbtree_propagate_list
                    .expect("non-null function pointer")(
                    h,
                    ref_costs[1usize],
                    (*(&raw mut mvs as *mut *mut [crate::stdlib::int16_t; 2]).offset(1isize))
                        .offset(mb_index as isize),
                    buf,
                    lowres_costs.offset(mb_index as isize),
                    bipred_weights[1usize],
                    (*h).mb.i_mb_y,
                    (*h).mb.i_mb_width,
                    1i32,
                );
            }
            (*h).mb.i_mb_y += 1;
        }
        if (*h).param.rc.i_vbv_buffer_size != 0 && (*h).param.rc.i_lookahead != 0 && referenced != 0
        {
            macroblock_tree_finish(
                h,
                *frames.offset(b as isize),
                average_duration,
                if b == p1 { b - p0 } else { 0i32 },
            );
        }
    }
}
pub unsafe extern "C" fn macroblock_tree(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut num_frames: ::core::ffi::c_int,
    mut b_intra: ::core::ffi::c_int,
) {
    unsafe {
        let mut bframes = 0i32;
        let mut total_duration = 0.0;
        let mut j = 0i32;
        let mut idx = (b_intra == 0) as ::core::ffi::c_int;
        while j <= num_frames {
            total_duration += (**frames.offset(j as isize)).f_duration;
            j += 1;
        }
        let mut average_duration = total_duration / (num_frames + 1i32) as ::core::ffi::c_float;
        let mut i = num_frames;
        if b_intra != 0 {
            slicetype_frame_cost(h, a, frames, 0i32, 0i32, 0i32);
        }
        while i > 0i32
            && ((**frames.offset(i as isize)).i_type == crate::x264_h::X264_TYPE_B
                || (**frames.offset(i as isize)).i_type == crate::x264_h::X264_TYPE_BREF)
        {
            i -= 1;
        }
        let mut last_nonb = i;
        if (*h).param.rc.i_lookahead == 0 {
            if b_intra != 0 {
                crate::stdlib::memset(
                    (**frames.offset(0isize)).i_propagate_cost as *mut ::core::ffi::c_void,
                    0i32,
                    ((*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>()),
                );
                crate::stdlib::memcpy(
                    (**frames.offset(0isize)).f_qp_offset as *mut ::core::ffi::c_void,
                    (**frames.offset(0isize)).f_qp_offset_aq as *const ::core::ffi::c_void,
                    ((*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()),
                );
                return;
            }
            let mut t = (**frames.offset(last_nonb as isize)).i_propagate_cost;
            let ref mut c2rust_fresh21 = (**frames.offset(last_nonb as isize)).i_propagate_cost;
            *c2rust_fresh21 = (**frames.offset(0isize)).i_propagate_cost;
            let ref mut c2rust_fresh22 = (**frames.offset(0isize)).i_propagate_cost;
            *c2rust_fresh22 = t;
            crate::stdlib::memset(
                (**frames.offset(0isize)).i_propagate_cost as *mut ::core::ffi::c_void,
                0i32,
                ((*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>()),
            );
        } else {
            if last_nonb < idx {
                return;
            }
            crate::stdlib::memset(
                (**frames.offset(last_nonb as isize)).i_propagate_cost as *mut ::core::ffi::c_void,
                0i32,
                ((*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>()),
            );
        }
        loop {
            let mut cur_nonb = 1i32;
            let c2rust_fresh23 = i;
            i -= 1;
            if c2rust_fresh23 <= idx {
                break;
            }
            cur_nonb = i;
            while ((**frames.offset(cur_nonb as isize)).i_type == crate::x264_h::X264_TYPE_B
                || (**frames.offset(cur_nonb as isize)).i_type == crate::x264_h::X264_TYPE_BREF)
                && cur_nonb > 0i32
            {
                cur_nonb -= 1;
            }
            if cur_nonb < idx {
                break;
            }
            slicetype_frame_cost(h, a, frames, cur_nonb, last_nonb, last_nonb);
            crate::stdlib::memset(
                (**frames.offset(cur_nonb as isize)).i_propagate_cost as *mut ::core::ffi::c_void,
                0i32,
                ((*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>()),
            );
            bframes = last_nonb - cur_nonb - 1i32;
            if (*h).param.i_bframe_pyramid != 0 && bframes > 1i32 {
                let mut middle = (bframes + 1i32) / 2i32 + cur_nonb;
                slicetype_frame_cost(h, a, frames, cur_nonb, last_nonb, middle);
                crate::stdlib::memset(
                    (**frames.offset(middle as isize)).i_propagate_cost as *mut ::core::ffi::c_void,
                    0i32,
                    ((*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>()),
                );
                while i > cur_nonb {
                    let mut p0 = if i > middle { middle } else { cur_nonb };
                    let mut p1 = if i < middle { middle } else { last_nonb };
                    if i != middle {
                        slicetype_frame_cost(h, a, frames, p0, p1, i);
                        macroblock_tree_propagate(h, frames, average_duration, p0, p1, i, 0i32);
                    }
                    i -= 1;
                }
                macroblock_tree_propagate(
                    h,
                    frames,
                    average_duration,
                    cur_nonb,
                    last_nonb,
                    middle,
                    1i32,
                );
            } else {
                while i > cur_nonb {
                    slicetype_frame_cost(h, a, frames, cur_nonb, last_nonb, i);
                    macroblock_tree_propagate(
                        h,
                        frames,
                        average_duration,
                        cur_nonb,
                        last_nonb,
                        i,
                        0i32,
                    );
                    i -= 1;
                }
            }
            macroblock_tree_propagate(
                h,
                frames,
                average_duration,
                cur_nonb,
                last_nonb,
                last_nonb,
                1i32,
            );
            last_nonb = cur_nonb;
        }
        if (*h).param.rc.i_lookahead == 0 {
            slicetype_frame_cost(h, a, frames, 0i32, last_nonb, last_nonb);
            macroblock_tree_propagate(
                h,
                frames,
                average_duration,
                0i32,
                last_nonb,
                last_nonb,
                1i32,
            );
            let mut t_0 = (**frames.offset(last_nonb as isize)).i_propagate_cost;
            let ref mut c2rust_fresh24 = (**frames.offset(last_nonb as isize)).i_propagate_cost;
            *c2rust_fresh24 = (**frames.offset(0isize)).i_propagate_cost;
            let ref mut c2rust_fresh25 = (**frames.offset(0isize)).i_propagate_cost;
            *c2rust_fresh25 = t_0;
        }
        macroblock_tree_finish(
            h,
            *frames.offset(last_nonb as isize),
            average_duration,
            last_nonb,
        );
        if (*h).param.i_bframe_pyramid != 0
            && bframes > 1i32
            && (*h).param.rc.i_vbv_buffer_size == 0
        {
            macroblock_tree_finish(
                h,
                *frames.offset((last_nonb + (bframes + 1i32) / 2i32) as isize),
                average_duration,
                0i32,
            );
        }
    }
}
pub unsafe extern "C" fn vbv_frame_cost(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut p0: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut cost = slicetype_frame_cost(h, a, frames, p0, p1, b);
        if (*h).param.rc.i_aq_mode != 0 {
            if (*h).param.rc.mb_tree {
                return slicetype_frame_cost_recalculate(h, frames, p0, p1, b);
            } else {
                return (**frames.offset(b as isize)).i_cost_est_aq[(b - p0) as usize]
                    [(p1 - b) as usize];
            }
        }
        cost
    }
}
pub unsafe extern "C" fn calculate_durations(
    mut h: *mut crate::src::common::common::x264_t,
    mut cur_frame: *mut crate::src::common::frame::x264_frame_t,
    mut prev_frame: *mut crate::src::common::frame::x264_frame_t,
    mut i_cpb_delay: *mut crate::stdlib::int64_t,
    mut i_coded_fields: *mut crate::stdlib::int64_t,
) {
    unsafe {
        (*cur_frame).i_cpb_delay = *i_cpb_delay;
        (*cur_frame).i_dpb_output_delay = (*cur_frame).i_field_cnt - *i_coded_fields;
        (*cur_frame).i_dpb_output_delay +=
            ((*h).sps.vui.i_num_reorder_frames * 2i32) as crate::stdlib::int64_t;
        if (*cur_frame).i_dpb_output_delay < 0i64 {
            (*cur_frame).i_cpb_delay += (*cur_frame).i_dpb_output_delay;
            (*cur_frame).i_dpb_output_delay = 0i64;
            if !prev_frame.is_null() {
                (*prev_frame).i_cpb_duration += (*cur_frame).i_dpb_output_delay;
            }
        }
        if (*cur_frame).keyframe && !(*h).param.intra_refresh {
            *i_cpb_delay = 0i64;
        }
        *i_cpb_delay += (*cur_frame).i_duration;
        *i_coded_fields += (*cur_frame).i_duration;
        (*cur_frame).i_cpb_duration = (*cur_frame).i_duration;
    }
}
pub unsafe extern "C" fn vbv_lookahead(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut num_frames: ::core::ffi::c_int,
    mut keyframe: ::core::ffi::c_int,
) {
    unsafe {
        let mut last_nonb = 0i32;
        let mut cur_nonb = 1i32;
        let mut idx = 0i32;
        while cur_nonb < num_frames
            && ((**frames.offset(cur_nonb as isize)).i_type == crate::x264_h::X264_TYPE_B
                || (**frames.offset(cur_nonb as isize)).i_type == crate::x264_h::X264_TYPE_BREF)
        {
            cur_nonb += 1;
        }
        let mut next_nonb = if keyframe != 0 { last_nonb } else { cur_nonb };
        if (**frames.offset(cur_nonb as isize)).i_coded_fields_lookahead >= 0i64 {
            (*h).i_coded_fields_lookahead =
                (**frames.offset(cur_nonb as isize)).i_coded_fields_lookahead;
            (*h).i_cpb_delay_lookahead = (**frames.offset(cur_nonb as isize)).i_cpb_delay_lookahead;
        }
        while cur_nonb < num_frames {
            let mut prev_frame = ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
            let mut prev_frame_idx = 0i32;
            if next_nonb != cur_nonb {
                let mut p0 = if (**frames.offset(cur_nonb as isize)).i_type
                    == crate::x264_h::X264_TYPE_I
                    || (**frames.offset(cur_nonb as isize)).i_type == crate::x264_h::X264_TYPE_IDR
                    || (**frames.offset(cur_nonb as isize)).i_type
                        == crate::x264_h::X264_TYPE_KEYFRAME
                {
                    cur_nonb
                } else {
                    last_nonb
                };
                (**frames.offset(next_nonb as isize)).i_planned_satd[idx as usize] =
                    vbv_frame_cost(h, a, frames, p0, cur_nonb, cur_nonb);
                (**frames.offset(next_nonb as isize)).i_planned_type[idx as usize] =
                    (**frames.offset(cur_nonb as isize)).i_type as crate::stdlib::uint8_t;
                (**frames.offset(cur_nonb as isize)).i_coded_fields_lookahead =
                    (*h).i_coded_fields_lookahead;
                (**frames.offset(cur_nonb as isize)).i_cpb_delay_lookahead =
                    (*h).i_cpb_delay_lookahead;
                calculate_durations(
                    h,
                    *frames.offset(cur_nonb as isize),
                    prev_frame,
                    &raw mut (*h).i_cpb_delay_lookahead,
                    &raw mut (*h).i_coded_fields_lookahead,
                );
                if !prev_frame.is_null() {
                    (**frames.offset(next_nonb as isize)).f_planned_cpb_duration
                        [prev_frame_idx as usize] = (*prev_frame).i_cpb_duration
                        as ::core::ffi::c_double
                        * (*h).sps.vui.i_num_units_in_tick as ::core::ffi::c_double
                        / (*h).sps.vui.i_time_scale as ::core::ffi::c_double;
                }
                (**frames.offset(next_nonb as isize)).f_planned_cpb_duration[idx as usize] =
                    (**frames.offset(cur_nonb as isize)).i_cpb_duration as ::core::ffi::c_double
                        * (*h).sps.vui.i_num_units_in_tick as ::core::ffi::c_double
                        / (*h).sps.vui.i_time_scale as ::core::ffi::c_double;
                prev_frame = *frames.offset(cur_nonb as isize);
                prev_frame_idx = idx;
                idx += 1;
            }
            let mut i = last_nonb + 1i32;
            while i < cur_nonb {
                (**frames.offset(next_nonb as isize)).i_planned_satd[idx as usize] =
                    vbv_frame_cost(h, a, frames, last_nonb, cur_nonb, i);
                (**frames.offset(next_nonb as isize)).i_planned_type[idx as usize] =
                    crate::x264_h::X264_TYPE_B as crate::stdlib::uint8_t;
                (**frames.offset(i as isize)).i_coded_fields_lookahead =
                    (*h).i_coded_fields_lookahead;
                (**frames.offset(i as isize)).i_cpb_delay_lookahead = (*h).i_cpb_delay_lookahead;
                calculate_durations(
                    h,
                    *frames.offset(i as isize),
                    prev_frame,
                    &raw mut (*h).i_cpb_delay_lookahead,
                    &raw mut (*h).i_coded_fields_lookahead,
                );
                if !prev_frame.is_null() {
                    (**frames.offset(next_nonb as isize)).f_planned_cpb_duration
                        [prev_frame_idx as usize] = (*prev_frame).i_cpb_duration
                        as ::core::ffi::c_double
                        * (*h).sps.vui.i_num_units_in_tick as ::core::ffi::c_double
                        / (*h).sps.vui.i_time_scale as ::core::ffi::c_double;
                }
                (**frames.offset(next_nonb as isize)).f_planned_cpb_duration[idx as usize] =
                    (**frames.offset(i as isize)).i_cpb_duration as ::core::ffi::c_double
                        * (*h).sps.vui.i_num_units_in_tick as ::core::ffi::c_double
                        / (*h).sps.vui.i_time_scale as ::core::ffi::c_double;
                prev_frame = *frames.offset(i as isize);
                prev_frame_idx = idx;
                i += 1;
                idx += 1;
            }
            last_nonb = cur_nonb;
            cur_nonb += 1;
            while cur_nonb <= num_frames
                && ((**frames.offset(cur_nonb as isize)).i_type == crate::x264_h::X264_TYPE_B
                    || (**frames.offset(cur_nonb as isize)).i_type == crate::x264_h::X264_TYPE_BREF)
            {
                cur_nonb += 1;
            }
        }
        (**frames.offset(next_nonb as isize)).i_planned_type[idx as usize] =
            crate::x264_h::X264_TYPE_AUTO as crate::stdlib::uint8_t;
    }
}
pub unsafe extern "C" fn slicetype_path_cost(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut path: *mut ::core::ffi::c_char,
    mut threshold: crate::stdlib::uint64_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut cost = 0u64;
        let mut loc = 1i32;
        path = path.offset(-1);
        while *path.offset(loc as isize) != 0 {
            let mut cur_nonb = 0i32;
            let mut next_nonb = loc;
            while *path.offset(next_nonb as isize) as ::core::ffi::c_int == 'B' as i32 {
                next_nonb += 1;
            }
            if *path.offset(next_nonb as isize) as ::core::ffi::c_int == 'P' as i32 {
                cost = cost.wrapping_add(slicetype_frame_cost(
                    h, a, frames, cur_nonb, next_nonb, next_nonb,
                ) as crate::stdlib::uint64_t);
            } else {
                cost = cost.wrapping_add(slicetype_frame_cost(
                    h, a, frames, next_nonb, next_nonb, next_nonb,
                ) as crate::stdlib::uint64_t);
            }
            if cost > threshold {
                break;
            }
            if (*h).param.i_bframe_pyramid != 0 && next_nonb - cur_nonb > 2i32 {
                let mut middle = cur_nonb + (next_nonb - cur_nonb) / 2i32;
                cost = cost.wrapping_add(slicetype_frame_cost(
                    h, a, frames, cur_nonb, next_nonb, middle,
                ) as crate::stdlib::uint64_t);
                let mut next_b = loc;
                while next_b < middle && cost < threshold {
                    cost = cost
                        .wrapping_add(slicetype_frame_cost(h, a, frames, cur_nonb, middle, next_b)
                            as crate::stdlib::uint64_t);
                    next_b += 1;
                }
                let mut next_b_0 = middle + 1i32;
                while next_b_0 < next_nonb && cost < threshold {
                    cost = cost.wrapping_add(slicetype_frame_cost(
                        h, a, frames, middle, next_nonb, next_b_0,
                    ) as crate::stdlib::uint64_t);
                    next_b_0 += 1;
                }
            } else {
                let mut next_b_1 = loc;
                while next_b_1 < next_nonb && cost < threshold {
                    cost = cost.wrapping_add(slicetype_frame_cost(
                        h, a, frames, cur_nonb, next_nonb, next_b_1,
                    ) as crate::stdlib::uint64_t);
                    next_b_1 += 1;
                }
            }
            loc = next_nonb + 1i32;
            cur_nonb = next_nonb;
        }
        cost
    }
}
pub unsafe extern "C" fn slicetype_path(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut length: ::core::ffi::c_int,
    mut best_paths: *mut [::core::ffi::c_char; 251],
) {
    unsafe {
        let mut paths = [[0; 251]; 2];
        let mut idx = 0i32;
        let mut path = 0i32;
        let mut num_paths = if ((*h).param.i_bframe + 1i32) < length {
            (*h).param.i_bframe + 1i32
        } else {
            length
        };
        let mut best_cost = crate::src::encoder::me::COST_MAX64;
        while path < num_paths {
            let mut best_possible = 0i32;
            let mut possible = 1i32;
            let mut i = 1i32;
            let mut len = length - (path + 1i32);
            crate::stdlib::memcpy(
                &raw mut *(&raw mut paths as *mut [::core::ffi::c_char; 251]).offset(idx as isize)
                    as *mut ::core::ffi::c_void,
                &raw mut *best_paths
                    .offset((len % (crate::src::common::base::X264_BFRAME_MAX + 1i32)) as isize)
                    as *const ::core::ffi::c_void,
                len as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memset(
                (&raw mut *(&raw mut paths as *mut [::core::ffi::c_char; 251]).offset(idx as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(len as isize) as *mut ::core::ffi::c_void,
                'B' as i32,
                path as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::strcpy(
                (&raw mut *(&raw mut paths as *mut [::core::ffi::c_char; 251]).offset(idx as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(len as isize)
                    .offset(path as isize),
                b"P\0".as_ptr() as *const ::core::ffi::c_char,
            );
            while i <= length {
                let mut i_type = (**frames.offset(i as isize)).i_type;
                if i_type != crate::x264_h::X264_TYPE_AUTO {
                    if i_type == crate::x264_h::X264_TYPE_B
                        || i_type == crate::x264_h::X264_TYPE_BREF
                    {
                        possible = (possible != 0
                            && (i < len
                                || i == length
                                || paths[idx as usize][(i - 1i32) as usize] as ::core::ffi::c_int
                                    == 'B' as i32))
                            as ::core::ffi::c_int;
                    } else {
                        possible = (possible != 0
                            && (i < len
                                || paths[idx as usize][(i - 1i32) as usize] as ::core::ffi::c_int
                                    != 'B' as i32))
                            as ::core::ffi::c_int;
                        paths[idx as usize][(i - 1i32) as usize] =
                            (if i_type == crate::x264_h::X264_TYPE_I
                                || i_type == crate::x264_h::X264_TYPE_IDR
                                || i_type == crate::x264_h::X264_TYPE_KEYFRAME
                            {
                                'I' as i32
                            } else {
                                'P' as i32
                            }) as ::core::ffi::c_char;
                    }
                }
                i += 1;
            }
            if possible != 0 || best_possible == 0 {
                if possible != 0 && best_possible == 0 {
                    best_cost = crate::src::encoder::me::COST_MAX64;
                }
                let mut cost = slicetype_path_cost(
                    h,
                    a,
                    frames,
                    &raw mut *(&raw mut paths as *mut [::core::ffi::c_char; 251])
                        .offset(idx as isize) as *mut ::core::ffi::c_char,
                    best_cost,
                );
                if cost < best_cost {
                    best_cost = cost;
                    best_possible = possible;
                    idx ^= 1i32;
                }
            }
            path += 1;
        }
        crate::stdlib::memcpy(
            &raw mut *best_paths
                .offset((length % (crate::src::common::base::X264_BFRAME_MAX + 1i32)) as isize)
                as *mut ::core::ffi::c_void,
            &raw mut *(&raw mut paths as *mut [::core::ffi::c_char; 251])
                .offset((idx ^ 1i32) as isize) as *const ::core::ffi::c_void,
            length as crate::__stddef_size_t_h::size_t,
        );
    }
}
pub unsafe extern "C" fn scenecut_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut p0: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut real_scenecut: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut f_bias = 0.;
        let mut frame = *frames.offset(p1 as isize);
        if real_scenecut != 0 && (*h).param.i_frame_packing == 5i32 && (*frame).i_frame & 1i32 != 0
        {
            return 0i32;
        }
        slicetype_frame_cost(h, a, frames, p0, p1, p1);
        let mut icost = (*frame).i_cost_est[0usize][0usize];
        let mut pcost = (*frame).i_cost_est[(p1 - p0) as usize][0usize];
        let mut i_gop_size = (*frame).i_frame - (*(*h).lookahead).i_last_keyframe;
        let mut f_thresh_max = ((*h).param.i_scenecut_threshold as ::core::ffi::c_double / 100.0)
            as ::core::ffi::c_float;
        let mut f_thresh_min =
            (f_thresh_max as ::core::ffi::c_double * 0.25) as ::core::ffi::c_float;
        if (*h).param.i_keyint_min == (*h).param.i_keyint_max {
            f_thresh_min = f_thresh_max;
        }
        if i_gop_size <= (*h).param.i_keyint_min / 4i32 || (*h).param.intra_refresh {
            f_bias = f_thresh_min / 4f32;
        } else if i_gop_size <= (*h).param.i_keyint_min {
            f_bias = f_thresh_min * i_gop_size as ::core::ffi::c_float
                / (*h).param.i_keyint_min as ::core::ffi::c_float;
        } else {
            f_bias = f_thresh_min
                + (f_thresh_max - f_thresh_min)
                    * (i_gop_size - (*h).param.i_keyint_min) as ::core::ffi::c_float
                    / ((*h).param.i_keyint_max - (*h).param.i_keyint_min) as ::core::ffi::c_float;
        }
        let mut res = (pcost as ::core::ffi::c_double
            >= (1.0 - f_bias as ::core::ffi::c_double) * icost as ::core::ffi::c_double)
            as ::core::ffi::c_int;
        if res != 0 && real_scenecut != 0 {
            let mut imb = (*frame).i_intra_mbs[(p1 - p0) as usize];
            let mut pmb = (if (*h).mb.i_mb_width > 2i32 && (*h).mb.i_mb_height > 2i32 {
                ((*h).mb.i_mb_width - 2i32) * ((*h).mb.i_mb_height - 2i32)
            } else {
                (*h).mb.i_mb_width * (*h).mb.i_mb_height
            }) - imb;
            log::debug!(
                "scene cut at {} Icost:{icost} Pcost:{pcost} ratio:{:.4} bias:{f_bias:.4} gop:{i_gop_size} (imb:{imb} pmb:{pmb})",
                (*frame).i_frame,
                1.0 - pcost as f64 / icost as f64,
            );
        }
        res
    }
}
pub unsafe extern "C" fn scenecut(
    mut h: *mut crate::src::common::common::x264_t,
    mut a: *mut x264_mb_analysis_t,
    mut frames: *mut *mut crate::src::common::frame::x264_frame_t,
    mut p0: ::core::ffi::c_int,
    mut p1: ::core::ffi::c_int,
    mut real_scenecut: ::core::ffi::c_int,
    mut num_frames: ::core::ffi::c_int,
    mut i_max_search: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if real_scenecut != 0 && (*h).param.i_bframe != 0 {
            let mut origmaxp1 = p0 + 1i32;
            if (*h).param.i_bframe_adaptive == crate::x264_h::X264_B_ADAPT_TRELLIS {
                origmaxp1 += (*h).param.i_bframe;
            } else {
                origmaxp1 += 1;
            }
            let mut maxp1 = if origmaxp1 < num_frames {
                origmaxp1
            } else {
                num_frames
            };
            let mut curp1 = p1;
            while curp1 <= maxp1 {
                if scenecut_internal(h, a, frames, p0, curp1, 0i32) == 0 {
                    let mut i = curp1;
                    while i > p0 {
                        (**frames.offset(i as isize)).scenecut = false;
                        i -= 1;
                    }
                }
                curp1 += 1;
            }
            let mut curp0 = p0;
            while curp0 <= maxp1 {
                if origmaxp1 > i_max_search
                    || curp0 < maxp1 && scenecut_internal(h, a, frames, curp0, maxp1, 0i32) != 0
                {
                    (**frames.offset(curp0 as isize)).scenecut = false;
                }
                curp0 += 1;
            }
        }
        if !(**frames.offset(p1 as isize)).scenecut {
            return 0i32;
        }
        scenecut_internal(h, a, frames, p0, p1, real_scenecut)
    }
}
pub unsafe extern "C" fn x264_8_slicetype_analyse(
    mut h: *mut crate::src::common::common::x264_t,
    mut intra_minigop: ::core::ffi::c_int,
) {
    unsafe {
        let mut a = x264_mb_analysis_t {
            i_lambda: 0,
            i_lambda2: 0,
            i_qp: 0,
            p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
            p_cost_ref: [::core::ptr::null_mut::<crate::stdlib::uint16_t>(); 2],
            i_mbrd: 0,
            fast_intra: false,
            force_intra: false,
            avoid_topright: false,
            try_skip: false,
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
                me16x16: crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                },
                bi16x16: crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                },
                me8x8: [crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 4],
                me4x4: [[crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 4]; 4],
                me8x4: [[crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 2]; 4],
                me4x8: [[crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 2]; 4],
                me16x8: [crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 2],
                me8x16: [crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
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
                me16x16: crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                },
                bi16x16: crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                },
                me8x8: [crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 4],
                me4x4: [[crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 4]; 4],
                me8x4: [[crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 2]; 4],
                me4x8: [[crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 2]; 4],
                me16x8: [crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_stride: [0; 3],
                    mvp: [0; 2],
                    cost_mv: 0,
                    cost: 0,
                    mv: [0; 2],
                }; 2],
                me8x16: [crate::src::encoder::me::x264_me_t {
                    i_pixel: 0,
                    p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                    i_ref_cost: 0,
                    i_ref: 0,
                    weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                    p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                    p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                    p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                    integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
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
            i_mb_partition16x8: [Partition::D_L0_4x4; 2],
            i_mb_partition8x16: [Partition::D_L0_4x4; 2],
            i_mb_type16x8: MacroblockType::I_4x4,
            i_mb_type8x16: MacroblockType::I_4x4,
            direct_available: false,
            early_terminate: false,
        };
        let mut frames = [
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
        ];
        let mut framecnt = 0i32;
        let mut j = 1i32;
        let mut j_0 = 2i32;
        let mut reset_start = 0;
        let mut i_max_search = if (*(*h).lookahead).next.i_size < 250i32 {
            (*(*h).lookahead).next.i_size
        } else {
            250i32
        };
        let mut b_vbv_lookahead = ((*h).param.rc.i_vbv_buffer_size != 0
            && (*h).param.rc.i_lookahead != 0)
            as ::core::ffi::c_int;
        if (*h).param.deterministic {
            i_max_search =
                if i_max_search < (*(*h).lookahead).i_slicetype_length + 1i32 - intra_minigop {
                    i_max_search
                } else {
                    (*(*h).lookahead).i_slicetype_length + 1i32 - intra_minigop
                };
        }
        let mut keyframe = (intra_minigop != 0) as ::core::ffi::c_int;
        '_c2rust_label: {
            if (*h).frames.have_lowres {
            } else {
                crate::stdlib::__assert_fail(
                    b"h->frames.have_lowres\0".as_ptr() as *const ::core::ffi::c_char,
                    b"encoder/slicetype.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1488u32,
                    b"void x264_8_slicetype_analyse(x264_t *, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if (*(*h).lookahead).last_nonb.is_null() {
            return;
        }
        frames[0usize] = (*(*h).lookahead).last_nonb;
        while framecnt < i_max_search {
            frames[(framecnt + 1i32) as usize] =
                *(*(*h).lookahead).next.list.offset(framecnt as isize);
            framecnt += 1;
        }
        lowres_context_init(h, &raw mut a);
        if framecnt == 0 {
            if (*h).param.rc.mb_tree {
                macroblock_tree(
                    h,
                    &raw mut a,
                    &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                    0i32,
                    keyframe,
                );
            }
            return;
        }
        let mut keyint_limit = (*h).param.i_keyint_max - (*frames[0usize]).i_frame
            + (*(*h).lookahead).i_last_keyframe
            - 1i32;
        let mut num_frames = if (*h).param.intra_refresh {
            framecnt
        } else if framecnt < keyint_limit {
            framecnt
        } else {
            keyint_limit
        };
        let mut orig_num_frames = num_frames;
        if (*h).param.analyse.psy && (*h).param.rc.mb_tree || b_vbv_lookahead != 0 {
            num_frames = framecnt;
        } else if (*h).param.open_gop && num_frames < framecnt {
            num_frames += 1;
        } else if num_frames == 0i32 {
            (*frames[1usize]).i_type = crate::x264_h::X264_TYPE_I;
            return;
        }
        if ((*frames[1usize]).i_type == crate::x264_h::X264_TYPE_AUTO
            || ((*frames[1usize]).i_type == crate::x264_h::X264_TYPE_I
                || (*frames[1usize]).i_type == crate::x264_h::X264_TYPE_IDR
                || (*frames[1usize]).i_type == crate::x264_h::X264_TYPE_KEYFRAME))
            && (*h).param.i_scenecut_threshold != 0
            && scenecut(
                h,
                &raw mut a,
                &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                0i32,
                1i32,
                1i32,
                orig_num_frames,
                i_max_search,
            ) != 0
        {
            if (*frames[1usize]).i_type == crate::x264_h::X264_TYPE_AUTO {
                (*frames[1usize]).i_type = crate::x264_h::X264_TYPE_I;
            }
            return;
        }
        while j <= num_frames {
            if (*frames[j as usize]).i_type == crate::x264_h::X264_TYPE_KEYFRAME {
                (*frames[j as usize]).i_type = if (*h).param.open_gop {
                    crate::x264_h::X264_TYPE_I
                } else {
                    crate::x264_h::X264_TYPE_IDR
                };
            }
            j += 1;
        }
        while j_0 <= num_frames {
            if (*frames[j_0 as usize]).i_type == crate::x264_h::X264_TYPE_IDR
                && ((*frames[(j_0 - 1i32) as usize]).i_type == crate::x264_h::X264_TYPE_AUTO
                    || ((*frames[(j_0 - 1i32) as usize]).i_type == crate::x264_h::X264_TYPE_B
                        || (*frames[(j_0 - 1i32) as usize]).i_type
                            == crate::x264_h::X264_TYPE_BREF))
            {
                (*frames[(j_0 - 1i32) as usize]).i_type = crate::x264_h::X264_TYPE_P;
            }
            j_0 += 1;
        }
        let mut num_analysed_frames = num_frames;
        if (*h).param.i_bframe != 0 {
            let mut num_bframes_1 = 0i32;
            let mut j_5 = 1i32;
            if (*h).param.i_bframe_adaptive == crate::x264_h::X264_B_ADAPT_TRELLIS {
                if num_frames > 1i32 {
                    let mut j_1 = 2i32;
                    let mut j_2 = 1i32;
                    let mut best_paths =
     [
                            ::core::mem::transmute::<
                                [u8; 251],
                                [::core::ffi::c_char; 251],
                            >(
                                *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                            ),
                            ::core::mem::transmute::<
                                [u8; 251],
                                [::core::ffi::c_char; 251],
                            >(
                                *b"P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                            ),
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                            [0; 251],
                        ];
                    let mut best_path_index =
                        num_frames % (crate::src::common::base::X264_BFRAME_MAX + 1i32);
                    while j_1 <= num_frames {
                        slicetype_path(
                            h,
                            &raw mut a,
                            &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                            j_1,
                            &raw mut best_paths as *mut [::core::ffi::c_char; 251],
                        );
                        j_1 += 1;
                    }
                    while j_2 < num_frames {
                        if best_paths[best_path_index as usize][(j_2 - 1i32) as usize]
                            as ::core::ffi::c_int
                            != 'B' as i32
                        {
                            if (*frames[j_2 as usize]).i_type == crate::x264_h::X264_TYPE_AUTO
                                || ((*frames[j_2 as usize]).i_type == crate::x264_h::X264_TYPE_B
                                    || (*frames[j_2 as usize]).i_type
                                        == crate::x264_h::X264_TYPE_BREF)
                            {
                                (*frames[j_2 as usize]).i_type = crate::x264_h::X264_TYPE_P;
                            }
                        } else if (*frames[j_2 as usize]).i_type == crate::x264_h::X264_TYPE_AUTO {
                            (*frames[j_2 as usize]).i_type = crate::x264_h::X264_TYPE_B;
                        }
                        j_2 += 1;
                    }
                }
            } else if (*h).param.i_bframe_adaptive == crate::x264_h::X264_B_ADAPT_FAST {
                let mut j_3 = 1i32;
                let mut num_bframes = (*h).param.i_bframe;
                while j_3 < num_frames {
                    let mut last_nonb = 0i32;
                    if j_3 - 1i32 > 0i32
                        && ((*frames[(j_3 - 1i32) as usize]).i_type == crate::x264_h::X264_TYPE_B
                            || (*frames[(j_3 - 1i32) as usize]).i_type
                                == crate::x264_h::X264_TYPE_BREF)
                    {
                        num_bframes -= 1;
                    } else {
                        last_nonb = j_3 - 1i32;
                        num_bframes = (*h).param.i_bframe;
                    }
                    if num_bframes == 0 {
                        if (*frames[j_3 as usize]).i_type == crate::x264_h::X264_TYPE_AUTO
                            || ((*frames[j_3 as usize]).i_type == crate::x264_h::X264_TYPE_B
                                || (*frames[j_3 as usize]).i_type == crate::x264_h::X264_TYPE_BREF)
                        {
                            (*frames[j_3 as usize]).i_type = crate::x264_h::X264_TYPE_P;
                        }
                    } else if (*frames[j_3 as usize]).i_type == crate::x264_h::X264_TYPE_AUTO {
                        if (*frames[(j_3 + 1i32) as usize]).i_type == crate::x264_h::X264_TYPE_B
                            || (*frames[(j_3 + 1i32) as usize]).i_type
                                == crate::x264_h::X264_TYPE_BREF
                        {
                            (*frames[j_3 as usize]).i_type = crate::x264_h::X264_TYPE_P;
                        } else {
                            let mut path = [0; 251];
                            let mut bframes = j_3 - last_nonb - 1i32;
                            crate::stdlib::memset(
                                &raw mut path as *mut ::core::ffi::c_void,
                                'B' as i32,
                                bframes as crate::__stddef_size_t_h::size_t,
                            );
                            crate::stdlib::strcpy(
                                (&raw mut path as *mut ::core::ffi::c_char)
                                    .offset(bframes as isize),
                                b"PP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            let mut cost_p = slicetype_path_cost(
                                h,
                                &raw mut a,
                                (&raw mut frames
                                    as *mut *mut crate::src::common::frame::x264_frame_t)
                                    .offset(last_nonb as isize),
                                &raw mut path as *mut ::core::ffi::c_char,
                                crate::src::encoder::me::COST_MAX64,
                            );
                            crate::stdlib::strcpy(
                                (&raw mut path as *mut ::core::ffi::c_char)
                                    .offset(bframes as isize),
                                b"BP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            let mut cost_b = slicetype_path_cost(
                                h,
                                &raw mut a,
                                (&raw mut frames
                                    as *mut *mut crate::src::common::frame::x264_frame_t)
                                    .offset(last_nonb as isize),
                                &raw mut path as *mut ::core::ffi::c_char,
                                cost_p,
                            );
                            if cost_b < cost_p {
                                (*frames[j_3 as usize]).i_type = crate::x264_h::X264_TYPE_B;
                            } else {
                                (*frames[j_3 as usize]).i_type = crate::x264_h::X264_TYPE_P;
                            }
                        }
                    }
                    j_3 += 1;
                }
            } else {
                let mut j_4 = 1i32;
                let mut num_bframes_0 = (*h).param.i_bframe;
                while j_4 < num_frames {
                    if num_bframes_0 == 0 {
                        if (*frames[j_4 as usize]).i_type == crate::x264_h::X264_TYPE_AUTO
                            || ((*frames[j_4 as usize]).i_type == crate::x264_h::X264_TYPE_B
                                || (*frames[j_4 as usize]).i_type == crate::x264_h::X264_TYPE_BREF)
                        {
                            (*frames[j_4 as usize]).i_type = crate::x264_h::X264_TYPE_P;
                        }
                    } else if (*frames[j_4 as usize]).i_type == crate::x264_h::X264_TYPE_AUTO {
                        if (*frames[(j_4 + 1i32) as usize]).i_type == crate::x264_h::X264_TYPE_B
                            || (*frames[(j_4 + 1i32) as usize]).i_type
                                == crate::x264_h::X264_TYPE_BREF
                        {
                            (*frames[j_4 as usize]).i_type = crate::x264_h::X264_TYPE_P;
                        } else {
                            (*frames[j_4 as usize]).i_type = crate::x264_h::X264_TYPE_B;
                        }
                    }
                    if (*frames[j_4 as usize]).i_type == crate::x264_h::X264_TYPE_B
                        || (*frames[j_4 as usize]).i_type == crate::x264_h::X264_TYPE_BREF
                    {
                        num_bframes_0 -= 1;
                    } else {
                        num_bframes_0 = (*h).param.i_bframe;
                    }
                    j_4 += 1;
                }
            }
            if (*frames[num_frames as usize]).i_type == crate::x264_h::X264_TYPE_AUTO
                || ((*frames[num_frames as usize]).i_type == crate::x264_h::X264_TYPE_B
                    || (*frames[num_frames as usize]).i_type == crate::x264_h::X264_TYPE_BREF)
            {
                (*frames[num_frames as usize]).i_type = crate::x264_h::X264_TYPE_P;
            }
            while num_bframes_1 < num_frames
                && ((*frames[(num_bframes_1 + 1i32) as usize]).i_type == crate::x264_h::X264_TYPE_B
                    || (*frames[(num_bframes_1 + 1i32) as usize]).i_type
                        == crate::x264_h::X264_TYPE_BREF)
            {
                num_bframes_1 += 1;
            }
            while j_5 < num_bframes_1 + 1i32 {
                if (*frames[j_5 as usize]).i_forced_type == crate::x264_h::X264_TYPE_AUTO
                    && ((*frames[(j_5 + 1i32) as usize]).i_forced_type
                        == crate::x264_h::X264_TYPE_AUTO
                        || ((*frames[(j_5 + 1i32) as usize]).i_forced_type
                            == crate::x264_h::X264_TYPE_I
                            || (*frames[(j_5 + 1i32) as usize]).i_forced_type
                                == crate::x264_h::X264_TYPE_IDR
                            || (*frames[(j_5 + 1i32) as usize]).i_forced_type
                                == crate::x264_h::X264_TYPE_KEYFRAME))
                    && (*h).param.i_scenecut_threshold != 0
                    && scenecut(
                        h,
                        &raw mut a,
                        &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                        j_5,
                        j_5 + 1i32,
                        0i32,
                        orig_num_frames,
                        i_max_search,
                    ) != 0
                {
                    (*frames[j_5 as usize]).i_type = crate::x264_h::X264_TYPE_P;
                    num_analysed_frames = j_5;
                    break;
                } else {
                    j_5 += 1;
                }
            }
            reset_start = if keyframe != 0 {
                1i32
            } else if (num_bframes_1 + 2i32) < num_analysed_frames + 1i32 {
                num_bframes_1 + 2i32
            } else {
                num_analysed_frames + 1i32
            };
        } else {
            let mut j_6 = 1i32;
            while j_6 <= num_frames {
                if (*frames[j_6 as usize]).i_type == crate::x264_h::X264_TYPE_AUTO
                    || ((*frames[j_6 as usize]).i_type == crate::x264_h::X264_TYPE_B
                        || (*frames[j_6 as usize]).i_type == crate::x264_h::X264_TYPE_BREF)
                {
                    (*frames[j_6 as usize]).i_type = crate::x264_h::X264_TYPE_P;
                }
                j_6 += 1;
            }
            reset_start = (keyframe == 0) as ::core::ffi::c_int + 1i32;
        }
        if (*h).param.rc.mb_tree {
            macroblock_tree(
                h,
                &raw mut a,
                &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                if num_frames < (*h).param.i_keyint_max {
                    num_frames
                } else {
                    (*h).param.i_keyint_max
                },
                keyframe,
            );
        }
        if !(*h).param.intra_refresh {
            let mut j_7 = 1i32;
            let mut last_keyframe = (*(*h).lookahead).i_last_keyframe;
            while j_7 <= num_frames {
                let mut last_possible = 0i32;
                let mut frm = frames[j_7 as usize];
                let mut keyframe_dist = (*frm).i_frame - last_keyframe;
                if ((*frm).i_forced_type == crate::x264_h::X264_TYPE_AUTO
                    || ((*frm).i_forced_type == crate::x264_h::X264_TYPE_I
                        || (*frm).i_forced_type == crate::x264_h::X264_TYPE_IDR
                        || (*frm).i_forced_type == crate::x264_h::X264_TYPE_KEYFRAME))
                    && ((*h).param.open_gop
                        || !((*frames[(j_7 - 1i32) as usize]).i_forced_type
                            == crate::x264_h::X264_TYPE_B
                            || (*frames[(j_7 - 1i32) as usize]).i_forced_type
                                == crate::x264_h::X264_TYPE_BREF))
                {
                    last_possible = j_7;
                }
                if keyframe_dist >= (*h).param.i_keyint_max {
                    if last_possible != 0i32 && last_possible != j_7 {
                        j_7 = last_possible;
                        frm = frames[j_7 as usize];
                        keyframe_dist = (*frm).i_frame - last_keyframe;
                    }
                    last_possible = 0i32;
                    if (*frm).i_type != crate::x264_h::X264_TYPE_IDR {
                        (*frm).i_type = if (*h).param.open_gop {
                            crate::x264_h::X264_TYPE_I
                        } else {
                            crate::x264_h::X264_TYPE_IDR
                        };
                    }
                }
                if (*frm).i_type == crate::x264_h::X264_TYPE_I
                    && keyframe_dist >= (*h).param.i_keyint_min
                {
                    if (*h).param.open_gop {
                        last_keyframe = (*frm).i_frame;
                        if (*h).param.bluray_compat {
                            let mut bframes_0 = 0i32;
                            while bframes_0 < j_7 - 1i32
                                && ((*frames[(j_7 - 1i32 - bframes_0) as usize]).i_type
                                    == crate::x264_h::X264_TYPE_B
                                    || (*frames[(j_7 - 1i32 - bframes_0) as usize]).i_type
                                        == crate::x264_h::X264_TYPE_BREF)
                            {
                                bframes_0 += 1;
                            }
                            last_keyframe -= bframes_0;
                        }
                    } else if (*frm).i_forced_type != crate::x264_h::X264_TYPE_I {
                        (*frm).i_type = crate::x264_h::X264_TYPE_IDR;
                    }
                }
                if (*frm).i_type == crate::x264_h::X264_TYPE_IDR {
                    last_keyframe = (*frm).i_frame;
                    if j_7 > 1i32
                        && ((*frames[(j_7 - 1i32) as usize]).i_type == crate::x264_h::X264_TYPE_B
                            || (*frames[(j_7 - 1i32) as usize]).i_type
                                == crate::x264_h::X264_TYPE_BREF)
                    {
                        (*frames[(j_7 - 1i32) as usize]).i_type = crate::x264_h::X264_TYPE_P;
                    }
                }
                j_7 += 1;
            }
        }
        if b_vbv_lookahead != 0 {
            vbv_lookahead(
                h,
                &raw mut a,
                &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                num_frames,
                keyframe,
            );
        }
        let mut j_8 = reset_start;
        while j_8 <= num_frames {
            (*frames[j_8 as usize]).i_type = (*frames[j_8 as usize]).i_forced_type;
            j_8 += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_slicetype_decide(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut frames = [::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(); 18];
        let mut i = 0i32;
        let mut bframes = 0i32;
        let mut brefs = 0i32;
        let mut i_2 = 0i32;
        if (*(*h).lookahead).next.i_size == 0 {
            return;
        }
        let mut lookahead_size = (*(*h).lookahead).next.i_size;
        while i < (*(*h).lookahead).next.i_size {
            if (*h).param.vfr_input {
                let c2rust_fresh5 = lookahead_size;
                lookahead_size -= 1;
                if c2rust_fresh5 > 1i32 {
                    (**(*(*h).lookahead).next.list.offset(i as isize)).i_duration = 2i64
                        * ((**(*(*h).lookahead).next.list.offset((i + 1i32) as isize)).i_pts
                            - (**(*(*h).lookahead).next.list.offset(i as isize)).i_pts);
                } else {
                    (**(*(*h).lookahead).next.list.offset(i as isize)).i_duration =
                        (*h).i_prev_duration;
                }
            } else {
                (**(*(*h).lookahead).next.list.offset(i as isize)).i_duration = delta_tfi_divisor
                    [(**(*(*h).lookahead).next.list.offset(i as isize)).i_pic_struct as usize]
                    as crate::stdlib::int64_t;
            }
            (*h).i_prev_duration = (**(*(*h).lookahead).next.list.offset(i as isize)).i_duration;
            (**(*(*h).lookahead).next.list.offset(i as isize)).f_duration =
                ((**(*(*h).lookahead).next.list.offset(i as isize)).i_duration
                    as ::core::ffi::c_double
                    * (*h).sps.vui.i_num_units_in_tick as ::core::ffi::c_double
                    / (*h).sps.vui.i_time_scale as ::core::ffi::c_double)
                    as ::core::ffi::c_float;
            if (**(*(*h).lookahead).next.list.offset(i as isize)).i_frame
                > (*h).i_disp_fields_last_frame
                && lookahead_size > 0i32
            {
                (**(*(*h).lookahead).next.list.offset(i as isize)).i_field_cnt = (*h).i_disp_fields;
                (*h).i_disp_fields += (**(*(*h).lookahead).next.list.offset(i as isize)).i_duration;
                (*h).i_disp_fields_last_frame =
                    (**(*(*h).lookahead).next.list.offset(i as isize)).i_frame;
            } else if lookahead_size == 0i32 {
                (**(*(*h).lookahead).next.list.offset(i as isize)).i_field_cnt = (*h).i_disp_fields;
                (**(*(*h).lookahead).next.list.offset(i as isize)).i_duration =
                    (*h).i_prev_duration;
            }
            i += 1;
        }
        if (*h).param.rc.stat_read {
            let mut i_0 = 0i32;
            while i_0 < (*(*h).lookahead).next.i_size {
                (**(*(*h).lookahead).next.list.offset(i_0 as isize)).i_type =
                    crate::src::encoder::ratecontrol::x264_8_ratecontrol_slice_type(
                        h,
                        (**(*(*h).lookahead).next.list.offset(i_0 as isize)).i_frame,
                    );
                i_0 += 1;
            }
        } else if (*h).param.i_bframe != 0 && (*h).param.i_bframe_adaptive != 0
            || (*h).param.i_scenecut_threshold != 0
            || (*h).param.rc.mb_tree
            || (*h).param.rc.i_vbv_buffer_size != 0 && (*h).param.rc.i_lookahead != 0
        {
            x264_8_slicetype_analyse(h, 0i32);
        }
        loop {
            let mut frm = ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
            frm = *(*(*h).lookahead).next.list.offset(bframes as isize);
            if (*frm).i_forced_type != crate::x264_h::X264_TYPE_AUTO
                && (*frm).i_type != (*frm).i_forced_type
                && !((*frm).i_forced_type == crate::x264_h::X264_TYPE_KEYFRAME
                    && ((*frm).i_type == crate::x264_h::X264_TYPE_I
                        || (*frm).i_type == crate::x264_h::X264_TYPE_IDR
                        || (*frm).i_type == crate::x264_h::X264_TYPE_KEYFRAME))
            {
                log::warn!(
                    "forced frame type ({}) at {} was changed to frame type ({})",
                    (*frm).i_forced_type,
                    (*frm).i_frame,
                    (*frm).i_type
                );
            }
            if (*frm).i_type == crate::x264_h::X264_TYPE_BREF
                && (*h).param.i_bframe_pyramid < crate::x264_h::X264_B_PYRAMID_NORMAL
                && brefs == (*h).param.i_bframe_pyramid
            {
                (*frm).i_type = crate::x264_h::X264_TYPE_B;
                log::warn!(
                    "B-ref at frame {} incompatible with B-pyramid {}",
                    (*frm).i_frame,
                    x264_b_pyramid_names[(*h).param.i_bframe_pyramid as usize]
                );
            } else if (*frm).i_type == crate::x264_h::X264_TYPE_BREF
                && (*h).param.i_bframe_pyramid == crate::x264_h::X264_B_PYRAMID_NORMAL
                && brefs != 0
                && (*h).param.i_frame_reference <= brefs + 3i32
            {
                (*frm).i_type = crate::x264_h::X264_TYPE_B;
                log::warn!(
                    "B-ref at frame {} incompatible with B-pyramid {} and {} reference frames",
                    (*frm).i_frame,
                    x264_b_pyramid_names[(*h).param.i_bframe_pyramid as usize],
                    (*h).param.i_frame_reference
                );
            }
            if (*frm).i_type == crate::x264_h::X264_TYPE_KEYFRAME {
                (*frm).i_type = if (*h).param.open_gop {
                    crate::x264_h::X264_TYPE_I
                } else {
                    crate::x264_h::X264_TYPE_IDR
                };
            }
            if (!(*h).param.intra_refresh || (*frm).i_frame == 0i32)
                && (*frm).i_frame - (*(*h).lookahead).i_last_keyframe >= (*h).param.i_keyint_max
            {
                if (*frm).i_type == crate::x264_h::X264_TYPE_AUTO
                    || (*frm).i_type == crate::x264_h::X264_TYPE_I
                {
                    (*frm).i_type =
                        if (*h).param.open_gop && (*(*h).lookahead).i_last_keyframe >= 0i32 {
                            crate::x264_h::X264_TYPE_I
                        } else {
                            crate::x264_h::X264_TYPE_IDR
                        };
                }
                let mut warn =
                    ((*frm).i_type != crate::x264_h::X264_TYPE_IDR) as ::core::ffi::c_int;
                if warn != 0 && (*h).param.open_gop {
                    warn &= ((*frm).i_type != crate::x264_h::X264_TYPE_I) as ::core::ffi::c_int;
                }
                if warn != 0 {
                    log::warn!(
                        "specified frame type ({}) at {} is not compatible with keyframe interval",
                        (*frm).i_type,
                        (*frm).i_frame
                    );
                    (*frm).i_type =
                        if (*h).param.open_gop && (*(*h).lookahead).i_last_keyframe >= 0i32 {
                            crate::x264_h::X264_TYPE_I
                        } else {
                            crate::x264_h::X264_TYPE_IDR
                        };
                }
            }
            if (*frm).i_type == crate::x264_h::X264_TYPE_I
                && (*frm).i_frame - (*(*h).lookahead).i_last_keyframe >= (*h).param.i_keyint_min
            {
                if (*h).param.open_gop {
                    (*(*h).lookahead).i_last_keyframe = (*frm).i_frame;
                    if (*h).param.bluray_compat {
                        (*(*h).lookahead).i_last_keyframe -= bframes;
                    }
                    (*frm).keyframe = true;
                } else {
                    (*frm).i_type = crate::x264_h::X264_TYPE_IDR;
                }
            }
            if (*frm).i_type == crate::x264_h::X264_TYPE_IDR {
                (*(*h).lookahead).i_last_keyframe = (*frm).i_frame;
                (*frm).keyframe = true;
                if bframes > 0i32 {
                    bframes -= 1;
                    (**(*(*h).lookahead).next.list.offset(bframes as isize)).i_type =
                        crate::x264_h::X264_TYPE_P;
                }
            }
            if bframes == (*h).param.i_bframe
                || (*(*(*h).lookahead)
                    .next
                    .list
                    .offset((bframes + 1i32) as isize))
                .is_null()
            {
                if (*frm).i_type == crate::x264_h::X264_TYPE_B
                    || (*frm).i_type == crate::x264_h::X264_TYPE_BREF
                {
                    log::warn!("specified frame type is not compatible with max B-frames");
                }
                if (*frm).i_type == crate::x264_h::X264_TYPE_AUTO
                    || ((*frm).i_type == crate::x264_h::X264_TYPE_B
                        || (*frm).i_type == crate::x264_h::X264_TYPE_BREF)
                {
                    (*frm).i_type = crate::x264_h::X264_TYPE_P;
                }
            }
            if (*frm).i_type == crate::x264_h::X264_TYPE_BREF {
                brefs += 1;
            }
            if (*frm).i_type == crate::x264_h::X264_TYPE_AUTO {
                (*frm).i_type = crate::x264_h::X264_TYPE_B;
            } else if !((*frm).i_type == crate::x264_h::X264_TYPE_B
                || (*frm).i_type == crate::x264_h::X264_TYPE_BREF)
            {
                break;
            }
            bframes += 1;
        }
        if bframes != 0 {
            (**(*(*h).lookahead)
                .next
                .list
                .offset((bframes - 1i32) as isize))
            .b_last_minigop_bframe = 1u8;
        }
        (**(*(*h).lookahead).next.list.offset(bframes as isize)).i_bframes =
            bframes as crate::stdlib::uint8_t;
        if (*h).param.i_bframe_pyramid != 0 && bframes > 1i32 && brefs == 0 {
            (**(*(*h).lookahead)
                .next
                .list
                .offset(((bframes - 1i32) / 2i32) as isize))
            .i_type = crate::x264_h::X264_TYPE_BREF;
            brefs += 1;
        }
        if (*h).param.rc.i_rc_method != crate::x264_h::X264_RC_CQP {
            let mut a = x264_mb_analysis_t {
                i_lambda: 0,
                i_lambda2: 0,
                i_qp: 0,
                p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                p_cost_ref: [::core::ptr::null_mut::<crate::stdlib::uint16_t>(); 2],
                i_mbrd: 0,
                fast_intra: false,
                force_intra: false,
                avoid_topright: false,
                try_skip: false,
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
                    me16x16: crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    },
                    bi16x16: crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    },
                    me8x8: [crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 4],
                    me4x4: [[crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 4]; 4],
                    me8x4: [[crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 2]; 4],
                    me4x8: [[crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 2]; 4],
                    me16x8: [crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 2],
                    me8x16: [crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
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
                    me16x16: crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    },
                    bi16x16: crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    },
                    me8x8: [crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 4],
                    me4x4: [[crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 4]; 4],
                    me8x4: [[crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 2]; 4],
                    me4x8: [[crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 2]; 4],
                    me16x8: [crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_stride: [0; 3],
                        mvp: [0; 2],
                        cost_mv: 0,
                        cost: 0,
                        mv: [0; 2],
                    }; 2],
                    me8x16: [crate::src::encoder::me::x264_me_t {
                        i_pixel: 0,
                        p_cost_mv: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
                        i_ref_cost: 0,
                        i_ref: 0,
                        weight: ::core::ptr::null::<crate::src::common::mc::x264_weight_t>(),
                        p_fref: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 12],
                        p_fref_w: ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        p_fenc: [::core::ptr::null_mut::<crate::src::common::common::pixel>(); 3],
                        integral: ::core::ptr::null_mut::<crate::stdlib::uint16_t>(),
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
                i_mb_partition16x8: [Partition::D_16x16; 2],
                i_mb_partition8x16: [Partition::D_16x16; 2],
                i_mb_type16x8: MacroblockType::I_4x4,
                i_mb_type8x16: MacroblockType::I_4x4,
                direct_available: false,
                early_terminate: false,
            };
            let mut p0 = 0;
            let mut b = bframes + 1i32;
            let mut p1 = b;
            lowres_context_init(h, &raw mut a);
            frames[0usize] = (*(*h).lookahead).last_nonb;
            crate::stdlib::memcpy(
                (&raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t)
                    .offset(1isize) as *mut ::core::ffi::c_void,
                (*(*h).lookahead).next.list as *const ::core::ffi::c_void,
                ((bframes + 1i32) as crate::__stddef_size_t_h::size_t).wrapping_mul(
                    ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>(),
                ),
            );
            if (**(*(*h).lookahead).next.list.offset(bframes as isize)).i_type
                == crate::x264_h::X264_TYPE_I
                || (**(*(*h).lookahead).next.list.offset(bframes as isize)).i_type
                    == crate::x264_h::X264_TYPE_IDR
                || (**(*(*h).lookahead).next.list.offset(bframes as isize)).i_type
                    == crate::x264_h::X264_TYPE_KEYFRAME
            {
                p0 = bframes + 1i32;
            } else {
                p0 = 0i32;
            }
            slicetype_frame_cost(
                h,
                &raw mut a,
                &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                p0,
                p1,
                b,
            );
            if (p0 != p1 || bframes != 0) && (*h).param.rc.i_vbv_buffer_size != 0 {
                slicetype_frame_cost(
                    h,
                    &raw mut a,
                    &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                    b,
                    b,
                    b,
                );
                p0 = 0i32;
                b = 1i32;
                while b <= bframes {
                    if (*frames[b as usize]).i_type == crate::x264_h::X264_TYPE_B {
                        p1 = b;
                        while (*frames[p1 as usize]).i_type == crate::x264_h::X264_TYPE_B {
                            p1 += 1;
                        }
                    } else {
                        p1 = bframes + 1i32;
                    }
                    slicetype_frame_cost(
                        h,
                        &raw mut a,
                        &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t,
                        p0,
                        p1,
                        b,
                    );
                    if (*frames[b as usize]).i_type == crate::x264_h::X264_TYPE_BREF {
                        p0 = b;
                    }
                    b += 1;
                }
            }
        }
        if !(*h).param.rc.stat_read
            && (**(*(*h).lookahead).next.list.offset(bframes as isize)).i_type
                == crate::x264_h::X264_TYPE_P
            && (*h).param.analyse.i_weighted_pred >= crate::x264_h::X264_WEIGHTP_SIMPLE
        {
            x264_8_weights_analyse(
                h,
                *(*(*h).lookahead).next.list.offset(bframes as isize),
                (*(*h).lookahead).last_nonb,
                0i32,
            );
        }
        let mut i_coded = (**(*(*h).lookahead).next.list.offset(0isize)).i_frame;
        if bframes != 0 {
            let mut i_1 = 0i32;
            let mut idx_list = [brefs + 1i32, 1i32];
            while i_1 < bframes {
                let c2rust_fresh6 = idx_list[((**(*(*h).lookahead).next.list.offset(i_1 as isize))
                    .i_type
                    == crate::x264_h::X264_TYPE_BREF)
                    as ::core::ffi::c_int as usize];
                idx_list[((**(*(*h).lookahead).next.list.offset(i_1 as isize)).i_type
                    == crate::x264_h::X264_TYPE_BREF) as ::core::ffi::c_int
                    as usize] = idx_list[((**(*(*h).lookahead).next.list.offset(i_1 as isize))
                    .i_type
                    == crate::x264_h::X264_TYPE_BREF)
                    as ::core::ffi::c_int as usize]
                    + 1;
                let mut idx = c2rust_fresh6;
                frames[idx as usize] = *(*(*h).lookahead).next.list.offset(i_1 as isize);
                (*frames[idx as usize]).i_reordered_pts =
                    (**(*(*h).lookahead).next.list.offset(idx as isize)).i_pts;
                i_1 += 1;
            }
            frames[0usize] = *(*(*h).lookahead).next.list.offset(bframes as isize);
            (*frames[0usize]).i_reordered_pts =
                (**(*(*h).lookahead).next.list.offset(0isize)).i_pts;
            crate::stdlib::memcpy(
                (*(*h).lookahead).next.list as *mut ::core::ffi::c_void,
                &raw mut frames as *const ::core::ffi::c_void,
                ((bframes + 1i32) as crate::__stddef_size_t_h::size_t).wrapping_mul(
                    ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>(),
                ),
            );
        }
        while i_2 <= bframes {
            let c2rust_fresh7 = i_coded;
            i_coded += 1;
            (**(*(*h).lookahead).next.list.offset(i_2 as isize)).i_coded = c2rust_fresh7;
            if i_2 != 0 {
                calculate_durations(
                    h,
                    *(*(*h).lookahead).next.list.offset(i_2 as isize),
                    *(*(*h).lookahead).next.list.offset((i_2 - 1i32) as isize),
                    &raw mut (*h).i_cpb_delay,
                    &raw mut (*h).i_coded_fields,
                );
                (**(*(*h).lookahead).next.list.offset(0isize)).f_planned_cpb_duration
                    [(i_2 - 1i32) as usize] = (**(*(*h).lookahead).next.list.offset(i_2 as isize))
                    .i_cpb_duration
                    as ::core::ffi::c_double
                    * (*h).sps.vui.i_num_units_in_tick as ::core::ffi::c_double
                    / (*h).sps.vui.i_time_scale as ::core::ffi::c_double;
            } else {
                calculate_durations(
                    h,
                    *(*(*h).lookahead).next.list.offset(i_2 as isize),
                    ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(),
                    &raw mut (*h).i_cpb_delay,
                    &raw mut (*h).i_coded_fields,
                );
            }
            i_2 += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_rc_analyse_slice(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p0 = 0i32;
        let mut p1 = 0;
        let mut b = 0;
        if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_I
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_KEYFRAME
        {
            b = 0i32;
            p1 = b;
        } else if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_P {
            b = (*(*h).fenc).i_bframes as ::core::ffi::c_int + 1i32;
            p1 = b;
        } else {
            p1 = ((*(*h).fref_nearest[1usize]).i_poc - (*(*h).fref_nearest[0usize]).i_poc) / 2i32;
            b = ((*(*h).fenc).i_poc - (*(*h).fref_nearest[0usize]).i_poc) / 2i32;
        }
        let mut frames = (&raw mut (*h).fenc).offset(-(b as isize));
        let mut cost =
            (**frames.offset(b as isize)).i_cost_est[(b - p0) as usize][(p1 - b) as usize];
        '_c2rust_label: {
            if cost >= 0i32 {
            } else {
                crate::stdlib::__assert_fail(
                    b"cost >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"encoder/slicetype.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1996u32,
                    b"int x264_8_rc_analyse_slice(x264_t *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if (*h).param.rc.mb_tree && !(*h).param.rc.stat_read {
            cost = slicetype_frame_cost_recalculate(h, frames, p0, p1, b);
            if b != 0 && (*h).param.rc.i_vbv_buffer_size != 0 {
                slicetype_frame_cost_recalculate(h, frames, b, b, b);
            }
        } else if (*h).param.rc.i_aq_mode != 0 {
            cost =
                (**frames.offset(b as isize)).i_cost_est_aq[(b - p0) as usize][(p1 - b) as usize];
        }
        (*(*h).fenc).i_row_satd = (*(*h).fenc).i_row_satds[(b - p0) as usize][(p1 - b) as usize];
        (*(*h).fdec).i_row_satd = (*(*h).fdec).i_row_satds[(b - p0) as usize][(p1 - b) as usize];
        (*(*h).fdec).i_satd = cost;
        crate::stdlib::memcpy(
            (*(*h).fdec).i_row_satd as *mut ::core::ffi::c_void,
            (*(*h).fenc).i_row_satd as *const ::core::ffi::c_void,
            ((*h).mb.i_mb_height as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
        );
        if !((*(*h).fenc).i_type == crate::x264_h::X264_TYPE_I
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_KEYFRAME)
        {
            crate::stdlib::memcpy(
                (*(*h).fdec).i_row_satds[0usize][0usize] as *mut ::core::ffi::c_void,
                (*(*h).fenc).i_row_satds[0usize][0usize] as *const ::core::ffi::c_void,
                ((*h).mb.i_mb_height as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
            );
        }
        if (*h).param.intra_refresh
            && (*h).param.rc.i_vbv_buffer_size != 0
            && (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_P
        {
            let mut y = 0i32;
            let mut ip_factor = (256f32 * (*h).param.rc.f_ip_factor) as ::core::ffi::c_int;
            while y < (*h).mb.i_mb_height {
                let mut mb_xy = y * (*h).mb.i_mb_stride + (*(*h).fdec).i_pir_start_col;
                let mut x = (*(*h).fdec).i_pir_start_col;
                while x <= (*(*h).fdec).i_pir_end_col {
                    let mut intra_cost = (*(*(*h).fenc).i_intra_cost.offset(mb_xy as isize)
                        as ::core::ffi::c_int
                        * ip_factor
                        + 128i32)
                        >> 8i32;
                    let mut inter_cost =
                        *(*(*h).fenc).lowres_costs[(b - p0) as usize][(p1 - b) as usize]
                            .offset(mb_xy as isize) as ::core::ffi::c_int
                            & crate::src::common::frame::LOWRES_COST_MASK;
                    let mut diff = intra_cost - inter_cost;
                    if (*h).param.rc.i_aq_mode != 0 {
                        *(*(*h).fdec).i_row_satd.offset(y as isize) += (diff
                            * *(**frames.offset(b as isize))
                                .i_inv_qscale_factor
                                .offset(mb_xy as isize)
                                as ::core::ffi::c_int
                            + 128i32)
                            >> 8i32;
                    } else {
                        *(*(*h).fdec).i_row_satd.offset(y as isize) += diff;
                    }
                    cost += diff;
                    x += 1;
                    mb_xy += 1;
                }
                y += 1;
            }
        }
        cost
    }
}
