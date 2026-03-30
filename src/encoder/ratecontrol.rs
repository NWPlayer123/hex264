pub mod base_h {
    pub static mut slice_type_to_char: [::core::ffi::c_char; 3] = [
        'P' as i32 as ::core::ffi::c_char,
        'B' as i32 as ::core::ffi::c_char,
        'I' as i32 as ::core::ffi::c_char,
    ];
    #[inline(always)]
    pub extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return if v < i_min {
            i_min
        } else if v > i_max {
            i_max
        } else {
            v
        };
    }
    #[inline(always)]
    pub extern "C" fn x264_clip3f(
        mut v: ::core::ffi::c_double,
        mut f_min: ::core::ffi::c_double,
        mut f_max: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double {
        return if v < f_min {
            f_min
        } else if v > f_max {
            f_max
        } else {
            v
        };
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_exp2fix8(mut x: ::core::ffi::c_float) -> ::core::ffi::c_int {
        unsafe {
            let mut i: ::core::ffi::c_int =
                (x * (-64.0f32 / 6.0f32) + 512.5f32) as ::core::ffi::c_int;
            if i < 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if i > 1023 as ::core::ffi::c_int {
                return 0xffff as ::core::ffi::c_int;
            }
            return (crate::src::common::tables::x264_exp2_lut
                [(i & 63 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + 256 as ::core::ffi::c_int)
                << (i >> 6 as ::core::ffi::c_int)
                >> 8 as ::core::ffi::c_int;
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_log2(mut x: crate::stdlib::uint32_t) -> ::core::ffi::c_float {
        unsafe {
            let mut lz: ::core::ffi::c_int = x.leading_zeros() as i32;
            return crate::src::common::tables::x264_log2_lut
                [(x << lz >> 24 as ::core::ffi::c_int & 0x7f as crate::stdlib::uint32_t) as usize]
                + crate::src::common::tables::x264_log2_lz_lut[lz as usize];
        }
    }
}
pub mod osdep_h {
    #[inline]
    pub unsafe extern "C" fn x264_is_regular_file(
        mut filehandle: *mut crate::stdlib::FILE,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut file_stat: crate::stdlib::stat = crate::stdlib::stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: crate::stdlib::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: crate::stdlib::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: crate::stdlib::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            if crate::stdlib::fstat(crate::stdlib::fileno(filehandle), &raw mut file_stat) != 0 {
                return 1 as ::core::ffi::c_int;
            }
            return (file_stat.st_mode & crate::stdlib::__S_IFMT as crate::stdlib::__mode_t
                == 0o100000 as crate::stdlib::__mode_t) as ::core::ffi::c_int;
        }
    }
}
use crate::src::encoder::ratecontrol::base_h::slice_type_to_char;
use crate::src::encoder::ratecontrol::base_h::x264_clip3;
use crate::src::encoder::ratecontrol::base_h::x264_clip3f;
use crate::src::encoder::ratecontrol::base_h::x264_exp2fix8;
use crate::src::encoder::ratecontrol::base_h::x264_log2;
use crate::src::encoder::ratecontrol::osdep_h::x264_is_regular_file;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_ratecontrol_t {
    pub b_abr: ::core::ffi::c_int,
    pub b_2pass: ::core::ffi::c_int,
    pub b_vbv: ::core::ffi::c_int,
    pub b_vbv_min_rate: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_double,
    pub bitrate: ::core::ffi::c_double,
    pub rate_tolerance: ::core::ffi::c_double,
    pub qcompress: ::core::ffi::c_double,
    pub nmb: ::core::ffi::c_int,
    pub qp_constant: [::core::ffi::c_int; 3],
    pub rce: *mut ratecontrol_entry_t,
    pub qpm: ::core::ffi::c_float,
    pub qpa_rc: ::core::ffi::c_float,
    pub qpa_rc_prev: ::core::ffi::c_float,
    pub qpa_aq: ::core::ffi::c_int,
    pub qpa_aq_prev: ::core::ffi::c_int,
    pub qp_novbv: ::core::ffi::c_float,
    pub buffer_size: ::core::ffi::c_double,
    pub buffer_fill_final: crate::stdlib::int64_t,
    pub buffer_fill_final_min: crate::stdlib::int64_t,
    pub buffer_fill: ::core::ffi::c_double,
    pub buffer_rate: ::core::ffi::c_double,
    pub vbv_max_rate: ::core::ffi::c_double,
    pub pred: *mut predictor_t,
    pub single_frame_vbv: ::core::ffi::c_int,
    pub rate_factor_max_increment: ::core::ffi::c_float,
    pub last_satd: ::core::ffi::c_int,
    pub last_rceq: ::core::ffi::c_double,
    pub cplxr_sum: ::core::ffi::c_double,
    pub expected_bits_sum: ::core::ffi::c_double,
    pub filler_bits_sum: crate::stdlib::int64_t,
    pub wanted_bits_window: ::core::ffi::c_double,
    pub cbr_decay: ::core::ffi::c_double,
    pub short_term_cplxsum: ::core::ffi::c_double,
    pub short_term_cplxcount: ::core::ffi::c_double,
    pub rate_factor_constant: ::core::ffi::c_double,
    pub ip_offset: ::core::ffi::c_double,
    pub pb_offset: ::core::ffi::c_double,
    pub p_stat_file_out: *mut crate::stdlib::FILE,
    pub psz_stat_file_tmpname: *mut ::core::ffi::c_char,
    pub p_mbtree_stat_file_out: *mut crate::stdlib::FILE,
    pub psz_mbtree_stat_file_tmpname: *mut ::core::ffi::c_char,
    pub psz_mbtree_stat_file_name: *mut ::core::ffi::c_char,
    pub p_mbtree_stat_file_in: *mut crate::stdlib::FILE,
    pub num_entries: ::core::ffi::c_int,
    pub entry: *mut ratecontrol_entry_t,
    pub entry_out: *mut *mut ratecontrol_entry_t,
    pub last_qscale: ::core::ffi::c_double,
    pub last_qscale_for: [::core::ffi::c_double; 3],
    pub last_non_b_pict_type: ::core::ffi::c_int,
    pub accum_p_qp: ::core::ffi::c_double,
    pub accum_p_norm: ::core::ffi::c_double,
    pub last_accum_p_norm: ::core::ffi::c_double,
    pub lmin: [::core::ffi::c_double; 3],
    pub lmax: [::core::ffi::c_double; 3],
    pub lstep: ::core::ffi::c_double,
    pub mbtree: C2Rust_Unnamed_23,
    pub frame_size_estimated: ::core::ffi::c_float,
    pub bits_so_far: ::core::ffi::c_float,
    pub frame_size_maximum: ::core::ffi::c_double,
    pub frame_size_planned: ::core::ffi::c_double,
    pub slice_size_planned: ::core::ffi::c_double,
    pub row_pred: *mut predictor_t,
    pub row_preds: [[predictor_t; 2]; 3],
    pub pred_b_from_p: *mut predictor_t,
    pub bframes: ::core::ffi::c_int,
    pub bframe_bits: ::core::ffi::c_int,
    pub i_zones: ::core::ffi::c_int,
    pub zones: *mut crate::x264_h::x264_zone_t,
    pub prev_zone: *mut crate::x264_h::x264_zone_t,
    pub initial_cpb_removal_delay: ::core::ffi::c_int,
    pub initial_cpb_removal_delay_offset: ::core::ffi::c_int,
    pub nrt_first_access_unit: ::core::ffi::c_double,
    pub previous_cpb_final_arrival_time: ::core::ffi::c_double,
    pub hrd_multiply_denom: crate::stdlib::uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predictor_t {
    pub coeff_min: ::core::ffi::c_float,
    pub coeff: ::core::ffi::c_float,
    pub count: ::core::ffi::c_float,
    pub decay: ::core::ffi::c_float,
    pub offset: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub qp_buffer: [*mut crate::stdlib::uint16_t; 2],
    pub qpbuf_pos: ::core::ffi::c_int,
    pub src_mb_count: ::core::ffi::c_int,
    pub rescale_enabled: ::core::ffi::c_int,
    pub scale_buffer: [*mut ::core::ffi::c_float; 2],
    pub filtersize: [::core::ffi::c_int; 2],
    pub coeffs: [*mut ::core::ffi::c_float; 2],
    pub pos: [*mut ::core::ffi::c_int; 2],
    pub srcdim: [::core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ratecontrol_entry_t {
    pub pict_type: ::core::ffi::c_int,
    pub frame_type: ::core::ffi::c_int,
    pub kept_as_ref: ::core::ffi::c_int,
    pub qscale: ::core::ffi::c_double,
    pub mv_bits: ::core::ffi::c_int,
    pub tex_bits: ::core::ffi::c_int,
    pub misc_bits: ::core::ffi::c_int,
    pub expected_bits: ::core::ffi::c_double,
    pub expected_vbv: ::core::ffi::c_double,
    pub new_qscale: ::core::ffi::c_double,
    pub new_qp: ::core::ffi::c_float,
    pub i_count: ::core::ffi::c_int,
    pub p_count: ::core::ffi::c_int,
    pub s_count: ::core::ffi::c_int,
    pub blurred_complexity: ::core::ffi::c_float,
    pub direct_mode: ::core::ffi::c_char,
    pub weight: [[crate::stdlib::int16_t; 2]; 3],
    pub i_weight_denom: [crate::stdlib::int16_t; 2],
    pub refcount: [::core::ffi::c_int; 16],
    pub refs: ::core::ffi::c_int,
    pub i_duration: crate::stdlib::int64_t,
    pub i_cpb_duration: crate::stdlib::int64_t,
    pub out_num: ::core::ffi::c_int,
}
#[inline]
unsafe extern "C" fn qp2qscale(mut qp: ::core::ffi::c_float) -> ::core::ffi::c_float {
    unsafe {
        return 0.85f32
            * crate::stdlib::powf(
                2.0f32,
                (qp - (12.0f32 + crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_float))
                    / 6.0f32,
            );
    }
}
#[inline]
unsafe extern "C" fn qscale2qp(mut qscale: ::core::ffi::c_float) -> ::core::ffi::c_float {
    unsafe {
        return 12.0f32
            + crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_float
            + 6.0f32 * crate::stdlib::log2f(qscale / 0.85f32);
    }
}
#[inline]
unsafe extern "C" fn qscale2bits(
    mut rce: *mut ratecontrol_entry_t,
    mut qscale: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    unsafe {
        if qscale < 0.1f64 {
            qscale = 0.1f64;
        }
        return ((*rce).tex_bits as ::core::ffi::c_double + 0.1f64)
            * crate::stdlib::pow((*rce).qscale / qscale, 1.1f64)
            + (*rce).mv_bits as ::core::ffi::c_double
                * crate::stdlib::pow(
                    (if (*rce).qscale > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                        (*rce).qscale
                    } else {
                        1 as ::core::ffi::c_int as ::core::ffi::c_double
                    }) / (if qscale > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                        qscale
                    } else {
                        1 as ::core::ffi::c_int as ::core::ffi::c_double
                    }),
                    0.5f64,
                )
            + (*rce).misc_bits as ::core::ffi::c_double;
    }
}
#[inline(always)]
unsafe extern "C" fn ac_energy_var(
    mut sum_ssd: crate::stdlib::uint64_t,
    mut shift: ::core::ffi::c_int,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut i: ::core::ffi::c_int,
    mut b_store: ::core::ffi::c_int,
) -> crate::stdlib::uint32_t {
    unsafe {
        let mut sum: crate::stdlib::uint32_t = sum_ssd as crate::stdlib::uint32_t;
        let mut ssd: crate::stdlib::uint32_t =
            (sum_ssd >> 32 as ::core::ffi::c_int) as crate::stdlib::uint32_t;
        if b_store != 0 {
            (*frame).i_pixel_sum[i as usize] = (*frame).i_pixel_sum[i as usize].wrapping_add(sum);
            (*frame).i_pixel_ssd[i as usize] =
                (*frame).i_pixel_ssd[i as usize].wrapping_add(ssd as crate::stdlib::uint64_t);
        }
        return (ssd as crate::stdlib::uint64_t).wrapping_sub(
            (sum as crate::stdlib::uint64_t).wrapping_mul(sum as crate::stdlib::uint64_t) >> shift,
        ) as crate::stdlib::uint32_t;
    }
}
#[inline(always)]
unsafe extern "C" fn ac_energy_plane(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut i: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
    mut b_field: ::core::ffi::c_int,
    mut b_store: ::core::ffi::c_int,
) -> crate::stdlib::uint32_t {
    unsafe {
        let mut height: ::core::ffi::c_int = if b_chroma != 0 {
            16 as ::core::ffi::c_int
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        };
        let mut stride: ::core::ffi::c_int = (*frame).i_stride[i as usize];
        let mut offset: ::core::ffi::c_int = if b_field != 0 {
            16 as ::core::ffi::c_int * mb_x
                + height * (mb_y & !(1 as ::core::ffi::c_int)) * stride
                + (mb_y & 1 as ::core::ffi::c_int) * stride
        } else {
            16 as ::core::ffi::c_int * mb_x + height * mb_y * stride
        };
        stride <<= b_field;
        if b_chroma != 0 {
            let mut pix: [crate::src::common::common::pixel; 256] = [0; 256];
            let mut chromapix: ::core::ffi::c_int = (*h).luma2chroma_pixel
                [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
            let mut shift: ::core::ffi::c_int = 7 as ::core::ffi::c_int
                - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            (*h).mc
                .load_deinterleave_chroma_fenc
                .expect("non-null function pointer")(
                &raw mut pix as *mut crate::src::common::common::pixel,
                (*frame).plane[1 as ::core::ffi::c_int as usize].offset(offset as isize),
                stride as crate::stdlib::intptr_t,
                height,
            );
            return ac_energy_var(
                (*h).pixf.var[chromapix as usize].expect("non-null function pointer")(
                    &raw mut pix as *mut crate::src::common::common::pixel,
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                ),
                shift,
                frame,
                1 as ::core::ffi::c_int,
                b_store,
            )
            .wrapping_add(ac_energy_var(
                (*h).pixf.var[chromapix as usize].expect("non-null function pointer")(
                    (&raw mut pix as *mut crate::src::common::common::pixel).offset(
                        (crate::src::common::common::FENC_STRIDE / 2 as ::core::ffi::c_int)
                            as isize,
                    ),
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                ),
                shift,
                frame,
                2 as ::core::ffi::c_int,
                b_store,
            ));
        } else {
            return ac_energy_var(
                (*h).pixf.var
                    [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    (*frame).plane[i as usize].offset(offset as isize),
                    stride as crate::stdlib::intptr_t,
                ),
                8 as ::core::ffi::c_int,
                frame,
                i,
                b_store,
            );
        };
    }
}
#[inline(never)]
unsafe extern "C" fn ac_energy_mb(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) -> crate::stdlib::uint32_t {
    unsafe {
        let mut var: crate::stdlib::uint32_t = 0;
        crate::src::common::macroblock::x264_8_prefetch_fenc(
            h as *mut crate::src::common::common::x264_t,
            frame as *mut crate::src::common::frame::x264_frame,
            mb_x,
            mb_y,
        );
        if (*h).mb.b_adaptive_mbaff != 0 {
            let mut var_interlaced: crate::stdlib::uint32_t = ac_energy_plane(
                h,
                mb_x,
                mb_y,
                frame,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            let mut var_progressive: crate::stdlib::uint32_t = ac_energy_plane(
                h,
                mb_x,
                mb_y,
                frame,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                var_interlaced = var_interlaced.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                ));
                var_progressive = var_progressive.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                ));
                var_interlaced = var_interlaced.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                ));
                var_progressive = var_progressive.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                ));
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                var_interlaced = var_interlaced.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                ));
                var_progressive = var_progressive.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
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
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*h).param.b_interlaced,
                1 as ::core::ffi::c_int,
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                var = var.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    (*h).param.b_interlaced,
                    1 as ::core::ffi::c_int,
                ));
                var = var.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    (*h).param.b_interlaced,
                    1 as ::core::ffi::c_int,
                ));
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                var = var.wrapping_add(ac_energy_plane(
                    h,
                    mb_x,
                    mb_y,
                    frame,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    (*h).param.b_interlaced,
                    1 as ::core::ffi::c_int,
                ));
            }
        }
        return var;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_adaptive_quant_frame(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut quant_offsets: *mut ::core::ffi::c_float,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            (*frame).i_pixel_sum[i as usize] = 0 as crate::stdlib::uint32_t;
            (*frame).i_pixel_ssd[i as usize] = 0 as crate::stdlib::uint64_t;
            i += 1;
        }
        if (*h).param.rc.i_aq_mode == crate::x264_h::X264_AQ_NONE
            || (*h).param.rc.f_aq_strength == 0 as ::core::ffi::c_int as ::core::ffi::c_float
        {
            if (*h).param.rc.i_aq_mode != 0
                && (*h).param.rc.f_aq_strength == 0 as ::core::ffi::c_int as ::core::ffi::c_float
            {
                if !quant_offsets.is_null() {
                    let mut mb_xy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while mb_xy < (*h).mb.i_mb_count {
                        let ref mut c2rust_fresh6 = *(*frame).f_qp_offset_aq.offset(mb_xy as isize);
                        *c2rust_fresh6 = *quant_offsets.offset(mb_xy as isize);
                        *(*frame).f_qp_offset.offset(mb_xy as isize) = *c2rust_fresh6;
                        mb_xy += 1;
                    }
                    if (*h).frames.b_have_lowres != 0 {
                        let mut mb_xy_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while mb_xy_0 < (*h).mb.i_mb_count {
                            *(*frame).i_inv_qscale_factor.offset(mb_xy_0 as isize) =
                                x264_exp2fix8(*(*frame).f_qp_offset.offset(mb_xy_0 as isize))
                                    as crate::stdlib::uint16_t;
                            mb_xy_0 += 1;
                        }
                    }
                } else {
                    crate::stdlib::memset(
                        (*frame).f_qp_offset as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ((*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()
                                as crate::__stddef_size_t_h::size_t),
                    );
                    crate::stdlib::memset(
                        (*frame).f_qp_offset_aq as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ((*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()
                                as crate::__stddef_size_t_h::size_t),
                    );
                    if (*h).frames.b_have_lowres != 0 {
                        let mut mb_xy_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while mb_xy_1 < (*h).mb.i_mb_count {
                            *(*frame).i_inv_qscale_factor.offset(mb_xy_1 as isize) =
                                256 as crate::stdlib::uint16_t;
                            mb_xy_1 += 1;
                        }
                    }
                }
            }
            if (*h).param.analyse.i_weighted_pred != 0 {
                let mut mb_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while mb_y < (*h).mb.i_mb_height {
                    let mut mb_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
            let mut strength: ::core::ffi::c_float = 0.;
            let mut avg_adj: ::core::ffi::c_float = 0.0f32;
            let mut bias_strength: ::core::ffi::c_float = 0.0f32;
            if (*h).param.rc.i_aq_mode == crate::x264_h::X264_AQ_AUTOVARIANCE
                || (*h).param.rc.i_aq_mode == crate::x264_h::X264_AQ_AUTOVARIANCE_BIASED
            {
                let mut bit_depth_correction: ::core::ffi::c_float = 1.0f32
                    / ((1 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int
                            * (crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int))
                        as ::core::ffi::c_float;
                let mut avg_adj_pow2: ::core::ffi::c_float = 0.0f32;
                let mut mb_y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while mb_y_0 < (*h).mb.i_mb_height {
                    let mut mb_x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while mb_x_0 < (*h).mb.i_mb_width {
                        let mut energy: crate::stdlib::uint32_t =
                            ac_energy_mb(h, mb_x_0, mb_y_0, frame);
                        let mut qp_adj: ::core::ffi::c_float = crate::stdlib::powf(
                            energy as ::core::ffi::c_float * bit_depth_correction
                                + 1 as ::core::ffi::c_int as ::core::ffi::c_float,
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
                avg_adj /= (*h).mb.i_mb_count as ::core::ffi::c_float;
                avg_adj_pow2 /= (*h).mb.i_mb_count as ::core::ffi::c_float;
                strength = (*h).param.rc.f_aq_strength * avg_adj;
                avg_adj = avg_adj - 0.5f32 * (avg_adj_pow2 - 14.0f32) / avg_adj;
                bias_strength = (*h).param.rc.f_aq_strength;
            } else {
                strength = (*h).param.rc.f_aq_strength * 1.0397f32;
            }
            let mut mb_y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while mb_y_1 < (*h).mb.i_mb_height {
                let mut mb_x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while mb_x_1 < (*h).mb.i_mb_width {
                    let mut qp_adj_0: ::core::ffi::c_float = 0.;
                    let mut mb_xy_2: ::core::ffi::c_int = mb_x_1 + mb_y_1 * (*h).mb.i_mb_stride;
                    if (*h).param.rc.i_aq_mode == crate::x264_h::X264_AQ_AUTOVARIANCE_BIASED {
                        qp_adj_0 = *(*frame).f_qp_offset.offset(mb_xy_2 as isize);
                        qp_adj_0 = strength * (qp_adj_0 - avg_adj)
                            + bias_strength * (1.0f32 - 14.0f32 / (qp_adj_0 * qp_adj_0));
                    } else if (*h).param.rc.i_aq_mode == crate::x264_h::X264_AQ_AUTOVARIANCE {
                        qp_adj_0 = *(*frame).f_qp_offset.offset(mb_xy_2 as isize);
                        qp_adj_0 = strength * (qp_adj_0 - avg_adj);
                    } else {
                        let mut energy_0: crate::stdlib::uint32_t =
                            ac_energy_mb(h, mb_x_1, mb_y_1, frame);
                        qp_adj_0 = strength
                            * (x264_log2(if energy_0 > 1 as crate::stdlib::uint32_t {
                                energy_0
                            } else {
                                1 as crate::stdlib::uint32_t
                            }) - (14.427f32
                                + (2 as ::core::ffi::c_int
                                    * (crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_float));
                    }
                    if !quant_offsets.is_null() {
                        qp_adj_0 += *quant_offsets.offset(mb_xy_2 as isize);
                    }
                    let ref mut c2rust_fresh7 = *(*frame).f_qp_offset_aq.offset(mb_xy_2 as isize);
                    *c2rust_fresh7 = qp_adj_0;
                    *(*frame).f_qp_offset.offset(mb_xy_2 as isize) = *c2rust_fresh7;
                    if (*h).frames.b_have_lowres != 0 {
                        *(*frame).i_inv_qscale_factor.offset(mb_xy_2 as isize) =
                            x264_exp2fix8(qp_adj_0) as crate::stdlib::uint16_t;
                    }
                    mb_x_1 += 1;
                }
                mb_y_1 += 1;
            }
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 3 as ::core::ffi::c_int {
            let mut ssd: crate::stdlib::uint64_t = (*frame).i_pixel_ssd[i_0 as usize];
            let mut sum: crate::stdlib::uint64_t =
                (*frame).i_pixel_sum[i_0 as usize] as crate::stdlib::uint64_t;
            let mut width: ::core::ffi::c_int = 16 as ::core::ffi::c_int * (*h).mb.i_mb_width
                >> (i_0 != 0
                    && (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int))
                    as ::core::ffi::c_int;
            let mut height: ::core::ffi::c_int = 16 as ::core::ffi::c_int * (*h).mb.i_mb_height
                >> (i_0 != 0
                    && crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            (*frame).i_pixel_ssd[i_0 as usize] = ssd.wrapping_sub(
                sum.wrapping_mul(sum)
                    .wrapping_add(
                        (width * height / 2 as ::core::ffi::c_int) as crate::stdlib::uint64_t,
                    )
                    .wrapping_div((width * height) as crate::stdlib::uint64_t),
            );
            i_0 += 1;
        }
    }
}
unsafe extern "C" fn macroblock_tree_rescale_init(
    mut h: *mut crate::src::common::common::x264_t,
    mut rc: *mut x264_ratecontrol_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut srcdim: [::core::ffi::c_float; 2] = [
            (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_float / 16.0f32,
            (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_float / 16.0f32,
        ];
        let mut dstdim: [::core::ffi::c_float; 2] = [
            (*h).param.i_width as ::core::ffi::c_float / 16.0f32,
            (*h).param.i_height as ::core::ffi::c_float / 16.0f32,
        ];
        let mut srcdimi: [::core::ffi::c_int; 2] = [
            crate::stdlib::ceil(srcdim[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double)
                as ::core::ffi::c_int,
            crate::stdlib::ceil(srcdim[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double)
                as ::core::ffi::c_int,
        ];
        let mut dstdimi: [::core::ffi::c_int; 2] = [
            crate::stdlib::ceil(dstdim[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double)
                as ::core::ffi::c_int,
            crate::stdlib::ceil(dstdim[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double)
                as ::core::ffi::c_int,
        ];
        if (*h).param.b_interlaced != 0 || (*h).param.b_fake_interlaced != 0 {
            srcdimi[1 as ::core::ffi::c_int as usize] = srcdimi[1 as ::core::ffi::c_int as usize]
                + 1 as ::core::ffi::c_int
                & !(1 as ::core::ffi::c_int);
            dstdimi[1 as ::core::ffi::c_int as usize] = dstdimi[1 as ::core::ffi::c_int as usize]
                + 1 as ::core::ffi::c_int
                & !(1 as ::core::ffi::c_int);
        }
        (*rc).mbtree.src_mb_count =
            srcdimi[0 as ::core::ffi::c_int as usize] * srcdimi[1 as ::core::ffi::c_int as usize];
        (*rc).mbtree.qp_buffer[0 as ::core::ffi::c_int as usize] =
            crate::src::common::base::x264_malloc(
                ((*rc).mbtree.src_mb_count as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>() as usize)
                    as crate::stdlib::int64_t,
            ) as *mut crate::stdlib::uint16_t;
        if !(*rc).mbtree.qp_buffer[0 as ::core::ffi::c_int as usize].is_null() {
            if (*h).param.i_bframe_pyramid != 0 && (*h).param.rc.b_stat_read != 0 {
                (*rc).mbtree.qp_buffer[1 as ::core::ffi::c_int as usize] =
                    crate::src::common::base::x264_malloc(
                        ((*rc).mbtree.src_mb_count as usize).wrapping_mul(::core::mem::size_of::<
                            crate::stdlib::uint16_t,
                        >(
                        )
                            as usize) as crate::stdlib::int64_t,
                    ) as *mut crate::stdlib::uint16_t;
                if (*rc).mbtree.qp_buffer[1 as ::core::ffi::c_int as usize].is_null() {
                    c2rust_current_block = 15546540007840481838;
                } else {
                    c2rust_current_block = 5399440093318478209;
                }
            } else {
                c2rust_current_block = 5399440093318478209;
            }
            match c2rust_current_block {
                15546540007840481838 => {}
                _ => {
                    (*rc).mbtree.qpbuf_pos = -(1 as ::core::ffi::c_int);
                    if srcdimi[0 as ::core::ffi::c_int as usize]
                        == dstdimi[0 as ::core::ffi::c_int as usize]
                        && srcdimi[1 as ::core::ffi::c_int as usize]
                            == dstdimi[1 as ::core::ffi::c_int as usize]
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    (*rc).mbtree.rescale_enabled = 1 as ::core::ffi::c_int;
                    (*rc).mbtree.scale_buffer[0 as ::core::ffi::c_int as usize] =
                        crate::src::common::base::x264_malloc(
                            ((srcdimi[0 as ::core::ffi::c_int as usize]
                                * srcdimi[1 as ::core::ffi::c_int as usize])
                                as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_float>() as usize
                                ) as crate::stdlib::int64_t,
                        ) as *mut ::core::ffi::c_float;
                    if !(*rc).mbtree.scale_buffer[0 as ::core::ffi::c_int as usize].is_null() {
                        (*rc).mbtree.scale_buffer[1 as ::core::ffi::c_int as usize] =
                            crate::src::common::base::x264_malloc(
                                ((dstdimi[0 as ::core::ffi::c_int as usize]
                                    * srcdimi[1 as ::core::ffi::c_int as usize])
                                    as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<::core::ffi::c_float>() as usize
                                    ) as crate::stdlib::int64_t,
                            ) as *mut ::core::ffi::c_float;
                        if !(*rc).mbtree.scale_buffer[1 as ::core::ffi::c_int as usize].is_null() {
                            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            loop {
                                if !(i < 2 as ::core::ffi::c_int) {
                                    c2rust_current_block = 3160140712158701372;
                                    break;
                                }
                                if srcdim[i as usize] > dstdim[i as usize] {
                                    (*rc).mbtree.filtersize[i as usize] = 1 as ::core::ffi::c_int
                                        + (2 as ::core::ffi::c_int * srcdimi[i as usize]
                                            + dstdimi[i as usize]
                                            - 1 as ::core::ffi::c_int)
                                            / dstdimi[i as usize];
                                } else {
                                    (*rc).mbtree.filtersize[i as usize] = 3 as ::core::ffi::c_int;
                                }
                                (*rc).mbtree.coeffs[i as usize] =
                                    crate::src::common::base::x264_malloc(
                                        (((*rc).mbtree.filtersize[i as usize] * dstdimi[i as usize])
                                            as usize)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<::core::ffi::c_float>()
                                                    as usize,
                                            )
                                            as crate::stdlib::int64_t,
                                    )
                                        as *mut ::core::ffi::c_float;
                                if (*rc).mbtree.coeffs[i as usize].is_null() {
                                    c2rust_current_block = 15546540007840481838;
                                    break;
                                }
                                (*rc).mbtree.pos[i as usize] = crate::src::common::base::x264_malloc(
                                    (dstdimi[i as usize] as usize).wrapping_mul(
                                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                    ) as crate::stdlib::int64_t,
                                )
                                    as *mut ::core::ffi::c_int;
                                if (*rc).mbtree.pos[i as usize].is_null() {
                                    c2rust_current_block = 15546540007840481838;
                                    break;
                                }
                                let mut inc: ::core::ffi::c_float =
                                    srcdim[i as usize] / dstdim[i as usize];
                                let mut dmul: ::core::ffi::c_float = if inc > 1.0f32 {
                                    dstdim[i as usize] / srcdim[i as usize]
                                } else {
                                    1.0f32
                                };
                                let mut dstinsrc: ::core::ffi::c_float = 0.5f32 * inc - 0.5f32;
                                let mut filtersize: ::core::ffi::c_int =
                                    (*rc).mbtree.filtersize[i as usize];
                                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while j < dstdimi[i as usize] {
                                    let mut pos: ::core::ffi::c_int = (dstinsrc
                                        - (filtersize as ::core::ffi::c_float - 2.0f32) * 0.5f32)
                                        as ::core::ffi::c_int;
                                    let mut sum: ::core::ffi::c_float = 0.0f32;
                                    *(*rc).mbtree.pos[i as usize].offset(j as isize) = pos;
                                    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    while k < filtersize {
                                        let mut d: ::core::ffi::c_float = (crate::stdlib::fabs(
                                            ((pos + k) as ::core::ffi::c_float - dstinsrc)
                                                as ::core::ffi::c_double,
                                        ) * dmul
                                            as ::core::ffi::c_double)
                                            as ::core::ffi::c_float;
                                        let mut coeff: ::core::ffi::c_float = if 1.0f32 - d
                                            > 0 as ::core::ffi::c_int as ::core::ffi::c_float
                                        {
                                            1.0f32 - d
                                        } else {
                                            0 as ::core::ffi::c_int as ::core::ffi::c_float
                                        };
                                        *(*rc).mbtree.coeffs[i as usize]
                                            .offset((j * filtersize + k) as isize) = coeff;
                                        sum += coeff;
                                        k += 1;
                                    }
                                    sum = 1.0f32 / sum;
                                    let mut k_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
                            match c2rust_current_block {
                                15546540007840481838 => {}
                                _ => {
                                    (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize] =
                                        srcdimi[0 as ::core::ffi::c_int as usize];
                                    (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize] =
                                        srcdimi[1 as ::core::ffi::c_int as usize];
                                    return 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        return -(1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn macroblock_tree_rescale_destroy(mut rc: *mut x264_ratecontrol_t) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int {
            crate::src::common::base::x264_free(
                (*rc).mbtree.qp_buffer[i as usize] as *mut ::core::ffi::c_void,
            );
            crate::src::common::base::x264_free(
                (*rc).mbtree.scale_buffer[i as usize] as *mut ::core::ffi::c_void,
            );
            crate::src::common::base::x264_free(
                (*rc).mbtree.coeffs[i as usize] as *mut ::core::ffi::c_void,
            );
            crate::src::common::base::x264_free(
                (*rc).mbtree.pos[i as usize] as *mut ::core::ffi::c_void,
            );
            i += 1;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn tapfilter(
    mut src: *mut ::core::ffi::c_float,
    mut pos: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut coeff: *mut ::core::ffi::c_float,
    mut filtersize: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    unsafe {
        let mut sum: ::core::ffi::c_float = 0.0f32;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < filtersize {
            sum += *src.offset(
                (x264_clip3(pos, 0 as ::core::ffi::c_int, max - 1 as ::core::ffi::c_int) * stride)
                    as isize,
            ) * *coeff.offset(i as isize);
            i += 1;
            pos += 1;
        }
        return sum;
    }
}
unsafe extern "C" fn macroblock_tree_rescale(
    mut h: *mut crate::src::common::common::x264_t,
    mut rc: *mut x264_ratecontrol_t,
    mut dst: *mut ::core::ffi::c_float,
) {
    unsafe {
        let mut input: *mut ::core::ffi::c_float =
            (*rc).mbtree.scale_buffer[0 as ::core::ffi::c_int as usize];
        let mut output: *mut ::core::ffi::c_float =
            (*rc).mbtree.scale_buffer[1 as ::core::ffi::c_int as usize];
        let mut filtersize: ::core::ffi::c_int =
            (*rc).mbtree.filtersize[0 as ::core::ffi::c_int as usize];
        let mut stride: ::core::ffi::c_int = (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize];
        let mut height: ::core::ffi::c_int = (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize];
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < height {
            let mut coeff: *mut ::core::ffi::c_float =
                (*rc).mbtree.coeffs[0 as ::core::ffi::c_int as usize];
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < (*h).mb.i_mb_width {
                *output.offset(x as isize) = tapfilter(
                    input,
                    *(*rc).mbtree.pos[0 as ::core::ffi::c_int as usize].offset(x as isize),
                    stride,
                    1 as ::core::ffi::c_int,
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
        input = (*rc).mbtree.scale_buffer[1 as ::core::ffi::c_int as usize];
        output = dst;
        filtersize = (*rc).mbtree.filtersize[1 as ::core::ffi::c_int as usize];
        stride = (*h).mb.i_mb_width;
        height = (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize];
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < (*h).mb.i_mb_width {
            let mut coeff_0: *mut ::core::ffi::c_float =
                (*rc).mbtree.coeffs[1 as ::core::ffi::c_int as usize];
            let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_0 < (*h).mb.i_mb_height {
                *output.offset((y_0 * stride) as isize) = tapfilter(
                    input,
                    *(*rc).mbtree.pos[1 as ::core::ffi::c_int as usize].offset(y_0 as isize),
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
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_tree_read(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut quant_offsets: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut i_type_actual: crate::stdlib::uint8_t =
            (*(*rc).entry.offset((*frame).i_frame as isize)).pict_type as crate::stdlib::uint8_t;
        if (*(*rc).entry.offset((*frame).i_frame as isize)).kept_as_ref != 0 {
            let mut i_type: crate::stdlib::uint8_t = 0;
            if (*rc).mbtree.qpbuf_pos < 0 as ::core::ffi::c_int {
                c2rust_current_block = 12675440807659640239;
            } else {
                c2rust_current_block = 11812396948646013369;
            }
            loop {
                match c2rust_current_block {
                    12675440807659640239 => {
                        (*rc).mbtree.qpbuf_pos += 1;
                        if crate::stdlib::fread(
                            &raw mut i_type as *mut ::core::ffi::c_void,
                            1 as crate::__stddef_size_t_h::size_t,
                            1 as crate::__stddef_size_t_h::size_t,
                            (*rc).p_mbtree_stat_file_in,
                        ) == 0
                        {
                            c2rust_current_block = 6381850087970354452;
                            break;
                        }
                        if crate::stdlib::fread(
                            (*rc).mbtree.qp_buffer[(*rc).mbtree.qpbuf_pos as usize]
                                as *mut ::core::ffi::c_void,
                            ::core::mem::size_of::<crate::stdlib::uint16_t>()
                                as crate::__stddef_size_t_h::size_t,
                            (*rc).mbtree.src_mb_count as crate::__stddef_size_t_h::size_t,
                            (*rc).p_mbtree_stat_file_in,
                        ) != (*rc).mbtree.src_mb_count as ::core::ffi::c_uint
                            as ::core::ffi::c_ulong
                        {
                            c2rust_current_block = 6381850087970354452;
                            break;
                        }
                        if i_type as ::core::ffi::c_int != i_type_actual as ::core::ffi::c_int
                            && (*rc).mbtree.qpbuf_pos == 1 as ::core::ffi::c_int
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"MB-tree frametype %d doesn't match actual frametype %d.\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                i_type as ::core::ffi::c_int,
                                i_type_actual as ::core::ffi::c_int,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        if i_type as ::core::ffi::c_int != i_type_actual as ::core::ffi::c_int {
                            c2rust_current_block = 12675440807659640239;
                        } else {
                            c2rust_current_block = 11812396948646013369;
                        }
                    }
                    _ => {
                        let mut dst: *mut ::core::ffi::c_float =
                            if (*rc).mbtree.rescale_enabled != 0 {
                                (*rc).mbtree.scale_buffer[0 as ::core::ffi::c_int as usize]
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
                            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i < (*h).mb.i_mb_count {
                                *(*frame).i_inv_qscale_factor.offset(i as isize) =
                                    x264_exp2fix8(*(*frame).f_qp_offset.offset(i as isize))
                                        as crate::stdlib::uint16_t;
                                i += 1;
                            }
                        }
                        (*rc).mbtree.qpbuf_pos -= 1;
                        c2rust_current_block = 5689001924483802034;
                        break;
                    }
                }
            }
            match c2rust_current_block {
                5689001924483802034 => {}
                _ => {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_ERROR_1,
                        b"Incomplete MB-tree stats file.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
            }
        } else {
            x264_8_adaptive_quant_frame(h, frame, quant_offsets);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_reference_build_list_optimal(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rce: *mut ratecontrol_entry_t = (*(*h).rc).rce;
        let mut frames: [*mut crate::src::common::frame::x264_frame_t; 16] =
            [::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>(); 16];
        let mut weights: [[crate::src::common::mc::x264_weight_t; 3]; 16] =
            [[crate::src::common::mc::x264_weight_t {
                cachea: [0; 8],
                cacheb: [0; 8],
                i_denom: 0,
                i_scale: 0,
                i_offset: 0,
                weightfn: ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>(),
            }; 3]; 16];
        let mut refcount: [::core::ffi::c_int; 16] = [0; 16];
        if (*rce).refs != (*h).i_ref[0 as ::core::ffi::c_int as usize] {
            return -(1 as ::core::ffi::c_int);
        }
        crate::stdlib::memcpy(
            &raw mut frames as *mut *mut crate::src::common::frame::x264_frame_t
                as *mut ::core::ffi::c_void,
            &raw mut *(&raw mut (*h).fref
                as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut crate::src::common::frame::x264_frame_t
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[*mut crate::src::common::frame::x264_frame_t; 16]>()
                as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut refcount as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
            &raw mut (*rce).refcount as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_int; 16]>() as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut weights as *mut [crate::src::common::mc::x264_weight_t; 3]
                as *mut ::core::ffi::c_void,
            &raw mut (*(*h).fenc).weight as *mut [crate::src::common::mc::x264_weight_t; 3]
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[[crate::src::common::mc::x264_weight_t; 3]; 16]>()
                as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memset(
            (&raw mut *(&raw mut (*(*h).fenc).weight
                as *mut [crate::src::common::mc::x264_weight_t; 3])
                .offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::mc::x264_weight_t)
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::mc::x264_weight_t
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[[crate::src::common::mc::x264_weight_t; 3]; 15]>()
                as crate::__stddef_size_t_h::size_t,
        );
        let mut ref_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while ref_0 < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
            let mut max: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            let mut bestref: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
                if refcount[i as usize] > max {
                    max = refcount[i as usize];
                    bestref = i;
                }
                i += 1;
            }
            refcount[bestref as usize] = -(1 as ::core::ffi::c_int);
            (*h).fref[0 as ::core::ffi::c_int as usize][ref_0 as usize] = frames[bestref as usize];
            crate::stdlib::memcpy(
                &raw mut *(&raw mut (*(*h).fenc).weight
                    as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(ref_0 as isize)
                    as *mut crate::src::common::mc::x264_weight_t
                    as *mut ::core::ffi::c_void,
                &raw mut *(&raw mut weights as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(bestref as isize)
                    as *mut crate::src::common::mc::x264_weight_t
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[crate::src::common::mc::x264_weight_t; 3]>()
                    as crate::__stddef_size_t_h::size_t,
            );
            ref_0 += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn strcat_filename(
    mut input: *mut ::core::ffi::c_char,
    mut suffix: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut output: *mut ::core::ffi::c_char = crate::src::common::base::x264_malloc(
            crate::stdlib::strlen(input)
                .wrapping_add(crate::stdlib::strlen(suffix))
                .wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                as crate::stdlib::int64_t,
        ) as *mut ::core::ffi::c_char;
        if output.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        crate::stdlib::strcpy(output, input);
        crate::stdlib::strcat(output, suffix);
        return output;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_init_reconfigurable(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_init: ::core::ffi::c_int,
) {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        if b_init == 0 && (*rc).b_2pass != 0 {
            return;
        }
        if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF {
            let mut base_cplx: ::core::ffi::c_double = ((*h).mb.i_mb_count
                * (if (*h).param.i_bframe != 0 {
                    120 as ::core::ffi::c_int
                } else {
                    80 as ::core::ffi::c_int
                })) as ::core::ffi::c_double;
            let mut mbtree_offset: ::core::ffi::c_double = if (*h).param.rc.b_mb_tree != 0 {
                (1.0f64 - (*h).param.rc.f_qcompress as ::core::ffi::c_double) * 13.5f64
            } else {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            };
            (*rc).rate_factor_constant = crate::stdlib::pow(
                base_cplx,
                1 as ::core::ffi::c_int as ::core::ffi::c_double - (*rc).qcompress,
            ) / qp2qscale(
                ((*h).param.rc.f_rf_constant as ::core::ffi::c_double
                    + mbtree_offset
                    + crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_double)
                    as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
        }
        if (*h).param.rc.i_vbv_max_bitrate > 0 as ::core::ffi::c_int
            && (*h).param.rc.i_vbv_buffer_size > 0 as ::core::ffi::c_int
        {
            if (*rc).b_vbv_min_rate != 0 {
                (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
            }
            if (*h).param.rc.i_vbv_buffer_size
                < ((*h).param.rc.i_vbv_max_bitrate as ::core::ffi::c_double / (*rc).fps)
                    as ::core::ffi::c_int
            {
                (*h).param.rc.i_vbv_buffer_size =
                    ((*h).param.rc.i_vbv_max_bitrate as ::core::ffi::c_double / (*rc).fps)
                        as ::core::ffi::c_int;
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"VBV buffer size cannot be smaller than one frame, using %d kbit\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.rc.i_vbv_buffer_size,
                );
            }
            let mut kilobit_size: ::core::ffi::c_int = if (*h).param.i_avcintra_class != 0 {
                1024 as ::core::ffi::c_int
            } else {
                1000 as ::core::ffi::c_int
            };
            let mut vbv_buffer_size: ::core::ffi::c_int =
                (*h).param.rc.i_vbv_buffer_size * kilobit_size;
            let mut vbv_max_bitrate: ::core::ffi::c_int =
                (*h).param.rc.i_vbv_max_bitrate * kilobit_size;
            if (*h).param.i_nal_hrd != 0 && b_init != 0 {
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_cpb_cnt = 1 as ::core::ffi::c_int;
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .b_cbr_hrd =
                    ((*h).param.i_nal_hrd == crate::x264_h::X264_NAL_HRD_CBR) as ::core::ffi::c_int;
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_time_offset_length = 0 as ::core::ffi::c_int;
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_bit_rate_scale = x264_clip3(
                    (vbv_max_bitrate as ::core::ffi::c_uint).trailing_zeros() as i32 - BR_SHIFT,
                    0 as ::core::ffi::c_int,
                    15 as ::core::ffi::c_int,
                );
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_bit_rate_value = vbv_max_bitrate
                    >> (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .hrd
                        .i_bit_rate_scale
                        + BR_SHIFT;
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_bit_rate_unscaled = (*(&raw mut (*h).sps
                    as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_bit_rate_value
                    << (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .hrd
                        .i_bit_rate_scale
                        + BR_SHIFT;
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_cpb_size_scale = x264_clip3(
                    (vbv_buffer_size as ::core::ffi::c_uint).trailing_zeros() as i32 - CPB_SHIFT,
                    0 as ::core::ffi::c_int,
                    15 as ::core::ffi::c_int,
                );
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_cpb_size_value = vbv_buffer_size
                    >> (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .hrd
                        .i_cpb_size_scale
                        + CPB_SHIFT;
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_cpb_size_unscaled = (*(&raw mut (*h).sps
                    as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_cpb_size_value
                    << (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .hrd
                        .i_cpb_size_scale
                        + CPB_SHIFT;
                let mut max_cpb_output_delay: ::core::ffi::c_int = (if ((*h).param.i_keyint_max
                    as ::core::ffi::c_double
                    * 0.5f64
                    * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as ::core::ffi::c_double
                    / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_num_units_in_tick as ::core::ffi::c_double)
                    < 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    (*h).param.i_keyint_max as ::core::ffi::c_double
                        * 0.5f64
                        * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_time_scale as ::core::ffi::c_double
                        / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_num_units_in_tick as ::core::ffi::c_double
                } else {
                    2147483647 as ::core::ffi::c_int as ::core::ffi::c_double
                })
                    as ::core::ffi::c_int;
                let mut max_dpb_output_delay: ::core::ffi::c_int =
                    ((*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_max_dec_frame_buffering as ::core::ffi::c_double
                        * MAX_DURATION
                        * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_time_scale as ::core::ffi::c_double
                        / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_num_units_in_tick as ::core::ffi::c_double)
                        as ::core::ffi::c_int;
                let mut max_delay: ::core::ffi::c_int = (90000.0f64
                    * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .hrd
                        .i_cpb_size_unscaled as ::core::ffi::c_double
                    / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .hrd
                        .i_bit_rate_unscaled as ::core::ffi::c_double
                    + 0.5f64)
                    as ::core::ffi::c_int;
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_initial_cpb_removal_delay_length = 2 as ::core::ffi::c_int
                    + x264_clip3(
                        32 as ::core::ffi::c_int
                            - (max_delay as ::core::ffi::c_uint).leading_zeros() as i32,
                        4 as ::core::ffi::c_int,
                        22 as ::core::ffi::c_int,
                    );
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_cpb_removal_delay_length = x264_clip3(
                    32 as ::core::ffi::c_int
                        - (max_cpb_output_delay as ::core::ffi::c_uint).leading_zeros() as i32,
                    4 as ::core::ffi::c_int,
                    31 as ::core::ffi::c_int,
                );
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_dpb_output_delay_length = x264_clip3(
                    32 as ::core::ffi::c_int
                        - (max_dpb_output_delay as ::core::ffi::c_uint).leading_zeros() as i32,
                    4 as ::core::ffi::c_int,
                    31 as ::core::ffi::c_int,
                );
                vbv_buffer_size = (*(&raw mut (*h).sps
                    as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_cpb_size_unscaled;
                vbv_max_bitrate = (*(&raw mut (*h).sps
                    as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_bit_rate_unscaled;
            } else if (*h).param.i_nal_hrd != 0 && b_init == 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"VBV parameters cannot be changed when NAL HRD is in use\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .hrd
                .i_bit_rate_unscaled = vbv_max_bitrate;
            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .hrd
                .i_cpb_size_unscaled = vbv_buffer_size;
            if (*rc).b_vbv_min_rate != 0 {
                (*rc).bitrate = (*h).param.rc.i_bitrate as ::core::ffi::c_double
                    * kilobit_size as ::core::ffi::c_double;
            }
            (*rc).buffer_rate = vbv_max_bitrate as ::core::ffi::c_double / (*rc).fps;
            (*rc).vbv_max_rate = vbv_max_bitrate as ::core::ffi::c_double;
            (*rc).buffer_size = vbv_buffer_size as ::core::ffi::c_double;
            (*rc).single_frame_vbv =
                ((*rc).buffer_rate * 1.1f64 > (*rc).buffer_size) as ::core::ffi::c_int;
            if (*rc).b_abr != 0 && (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR {
                (*rc).cbr_decay = 1.0f64
                    - (*rc).buffer_rate / (*rc).buffer_size
                        * 0.5f64
                        * (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            > 1.5f64 - (*rc).buffer_rate * (*rc).fps / (*rc).bitrate
                        {
                            0 as ::core::ffi::c_int as ::core::ffi::c_double
                        } else {
                            1.5f64 - (*rc).buffer_rate * (*rc).fps / (*rc).bitrate
                        });
            }
            if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF
                && (*h).param.rc.f_rf_constant_max != 0.
            {
                (*rc).rate_factor_max_increment =
                    (*h).param.rc.f_rf_constant_max - (*h).param.rc.f_rf_constant;
                if (*rc).rate_factor_max_increment
                    <= 0 as ::core::ffi::c_int as ::core::ffi::c_float
                {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_WARNING_1,
                        b"CRF max must be greater than CRF\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*rc).rate_factor_max_increment =
                        0 as ::core::ffi::c_int as ::core::ffi::c_float;
                }
            }
            if b_init != 0 {
                if (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double > 1.0f64 {
                    (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
                        ((*h).param.rc.f_vbv_buffer_init
                            / (*h).param.rc.i_vbv_buffer_size as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        0 as ::core::ffi::c_int as ::core::ffi::c_double,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                    ) as ::core::ffi::c_float;
                }
                (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
                    if (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double
                        > (*rc).buffer_rate / (*rc).buffer_size
                    {
                        (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double
                    } else {
                        (*rc).buffer_rate / (*rc).buffer_size
                    },
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                ) as ::core::ffi::c_float;
                (*rc).buffer_fill_final_min = ((*rc).buffer_size
                    * (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double
                    * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as ::core::ffi::c_double)
                    as crate::stdlib::int64_t;
                (*rc).buffer_fill_final = (*rc).buffer_fill_final_min;
                (*rc).b_vbv = 1 as ::core::ffi::c_int;
                (*rc).b_vbv_min_rate = ((*rc).b_2pass == 0
                    && (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
                    && (*h).param.rc.i_vbv_max_bitrate <= (*h).param.rc.i_bitrate)
                    as ::core::ffi::c_int;
            }
        }
    }
}
pub const BR_SHIFT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CPB_SHIFT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_DURATION: ::core::ffi::c_double = 0.5f64;
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_new(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut w: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut num_preds: ::core::ffi::c_int = 0;
        static mut pred_coeff_table: [::core::ffi::c_float; 3] = [
            1.0f64 as ::core::ffi::c_float,
            1.0f64 as ::core::ffi::c_float,
            1.5f64 as ::core::ffi::c_float,
        ];
        let mut c2rust_current_block: u64;
        let mut rc: *mut x264_ratecontrol_t = ::core::ptr::null_mut::<x264_ratecontrol_t>();
        (*h).rc = crate::src::common::base::x264_malloc(
            ((*h).param.i_threads as usize)
                .wrapping_mul(::core::mem::size_of::<x264_ratecontrol_t>() as usize)
                as crate::stdlib::int64_t,
        ) as *mut x264_ratecontrol_t;
        if !(*h).rc.is_null() {
            crate::stdlib::memset(
                (*h).rc as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*h).param.i_threads as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<x264_ratecontrol_t>()
                        as crate::__stddef_size_t_h::size_t),
            );
            rc = (*h).rc;
            (*rc).b_abr = ((*h).param.rc.i_rc_method != crate::x264_h::X264_RC_CQP
                && (*h).param.rc.b_stat_read == 0) as ::core::ffi::c_int;
            (*rc).b_2pass = ((*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
                && (*h).param.rc.b_stat_read != 0)
                as ::core::ffi::c_int;
            if (*h).param.i_fps_num > 0 as crate::stdlib::uint32_t
                && (*h).param.i_fps_den > 0 as crate::stdlib::uint32_t
            {
                (*rc).fps = ((*h).param.i_fps_num as ::core::ffi::c_float
                    / (*h).param.i_fps_den as ::core::ffi::c_float)
                    as ::core::ffi::c_double;
            } else {
                (*rc).fps = 25.0f64;
            }
            if (*h).param.rc.b_mb_tree != 0 {
                (*h).param.rc.f_pb_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
                (*rc).qcompress = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                (*rc).qcompress = (*h).param.rc.f_qcompress as ::core::ffi::c_double;
            }
            (*rc).bitrate = (*h).param.rc.i_bitrate as ::core::ffi::c_double
                * (if (*h).param.i_avcintra_class != 0 {
                    1024.0f64
                } else {
                    1000.0f64
                });
            (*rc).rate_tolerance = (*h).param.rc.f_rate_tolerance as ::core::ffi::c_double;
            (*rc).nmb = (*h).mb.i_mb_count;
            (*rc).last_non_b_pict_type = -(1 as ::core::ffi::c_int);
            (*rc).cbr_decay = 1.0f64;
            if (*h).param.rc.i_rc_method != crate::x264_h::X264_RC_ABR
                && (*h).param.rc.b_stat_read != 0
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"CRF/CQP is incompatible with 2pass.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            x264_8_ratecontrol_init_reconfigurable(h, 1 as ::core::ffi::c_int);
            if (*h).param.i_nal_hrd != 0 {
                let mut denom: crate::stdlib::uint64_t = ((*(&raw mut (*h).sps
                    as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .i_bit_rate_unscaled
                    as crate::stdlib::uint64_t)
                    .wrapping_mul(
                        (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_time_scale as crate::stdlib::uint64_t,
                    );
                let mut num: crate::stdlib::uint64_t = 90000 as crate::stdlib::uint64_t;
                crate::src::common::base::x264_reduce_fraction64(&raw mut num, &raw mut denom);
                (*rc).hrd_multiply_denom = (90000 as crate::stdlib::uint64_t).wrapping_div(num);
                let mut bits_required: ::core::ffi::c_double =
                    crate::stdlib::log2(num as ::core::ffi::c_double)
                        + crate::stdlib::log2(
                            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                .vui
                                .i_time_scale as ::core::ffi::c_double,
                        )
                        + crate::stdlib::log2(
                            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                .vui
                                .hrd
                                .i_cpb_size_unscaled
                                as ::core::ffi::c_double,
                        );
                if bits_required >= 63 as ::core::ffi::c_int as ::core::ffi::c_double {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_ERROR_1,
                        b"HRD with very large timescale and bufsize not supported\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
            }
            if (*rc).rate_tolerance < 0.01f64 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"bitrate tolerance too small, using .01\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*rc).rate_tolerance = 0.01f64;
            }
            (*h).mb.b_variable_qp =
                ((*rc).b_vbv != 0 || (*h).param.rc.i_aq_mode != 0) as ::core::ffi::c_int;
            if (*rc).b_abr != 0 {
                (*rc).accum_p_norm = 0.01f64;
                (*rc).accum_p_qp = ((if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF {
                    (*h).param.rc.f_rf_constant
                } else {
                    24 as ::core::ffi::c_int as ::core::ffi::c_float
                }) + crate::src::common::common::QP_BD_OFFSET
                    as ::core::ffi::c_float)
                    as ::core::ffi::c_double
                    * (*rc).accum_p_norm;
                (*rc).cplxr_sum = 0.01f64
                    * crate::stdlib::pow(7.0e5f64, (*rc).qcompress)
                    * crate::stdlib::pow((*h).mb.i_mb_count as ::core::ffi::c_double, 0.5f64);
                (*rc).wanted_bits_window = 1.0f64 * (*rc).bitrate / (*rc).fps;
                (*rc).last_non_b_pict_type =
                    crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int;
            }
            (*rc).ip_offset =
                6.0f64 * crate::stdlib::log2f((*h).param.rc.f_ip_factor) as ::core::ffi::c_double;
            (*rc).pb_offset =
                6.0f64 * crate::stdlib::log2f((*h).param.rc.f_pb_factor) as ::core::ffi::c_double;
            (*rc).qp_constant
                [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize] =
                (*h).param.rc.i_qp_constant;
            (*rc).qp_constant
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize] =
                x264_clip3(
                    ((*h).param.rc.i_qp_constant as ::core::ffi::c_double - (*rc).ip_offset
                        + 0.5f64) as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    crate::src::common::common::QP_MAX,
                );
            (*rc).qp_constant
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize] =
                x264_clip3(
                    ((*h).param.rc.i_qp_constant as ::core::ffi::c_double
                        + (*rc).pb_offset
                        + 0.5f64) as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    crate::src::common::common::QP_MAX,
                );
            (*h).mb.ip_offset = ((*rc).ip_offset + 0.5f64) as ::core::ffi::c_int;
            (*rc).lstep = crate::stdlib::pow(
                2 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*h).param.rc.i_qp_step as ::core::ffi::c_double / 6.0f64,
            );
            (*rc).last_qscale = qp2qscale(
                (26 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
                    as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
            num_preds =
                (*h).param.b_sliced_threads * (*h).param.i_threads + 1 as ::core::ffi::c_int;
            (*rc).pred = crate::src::common::base::x264_malloc(
                (5 as usize)
                    .wrapping_mul(::core::mem::size_of::<predictor_t>() as usize)
                    .wrapping_mul(num_preds as usize) as crate::stdlib::int64_t,
            ) as *mut predictor_t;
            if !(*rc).pred.is_null() {
                (*rc).pred_b_from_p =
                    crate::src::common::base::x264_malloc(
                        ::core::mem::size_of::<predictor_t>() as crate::stdlib::int64_t
                    ) as *mut predictor_t;
                if !(*rc).pred_b_from_p.is_null() {
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i < 3 as ::core::ffi::c_int {
                        (*rc).last_qscale_for[i as usize] = qp2qscale(
                            (if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF {
                                (*h).param.rc.f_rf_constant
                            } else {
                                24 as ::core::ffi::c_int as ::core::ffi::c_float
                            }) + crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_float,
                        )
                            as ::core::ffi::c_double;
                        (*rc).lmin[i as usize] =
                            qp2qscale((*h).param.rc.i_qp_min as ::core::ffi::c_float)
                                as ::core::ffi::c_double;
                        (*rc).lmax[i as usize] =
                            qp2qscale((*h).param.rc.i_qp_max as ::core::ffi::c_float)
                                as ::core::ffi::c_double;
                        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while j < num_preds {
                            (*(*rc)
                                .pred
                                .offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .coeff_min = pred_coeff_table[i as usize]
                                / 2 as ::core::ffi::c_int as ::core::ffi::c_float;
                            (*(*rc)
                                .pred
                                .offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .coeff = pred_coeff_table[i as usize];
                            (*(*rc)
                                .pred
                                .offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .count = 1.0f32;
                            (*(*rc)
                                .pred
                                .offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .decay = 0.5f32;
                            (*(*rc)
                                .pred
                                .offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .offset = 0.0f32;
                            j += 1;
                        }
                        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while j_0 < 2 as ::core::ffi::c_int {
                            (*rc).row_preds[i as usize][j_0 as usize].coeff_min = (0.25f64
                                / 4 as ::core::ffi::c_int as ::core::ffi::c_double)
                                as ::core::ffi::c_float;
                            (*rc).row_preds[i as usize][j_0 as usize].coeff = 0.25f32;
                            (*rc).row_preds[i as usize][j_0 as usize].count = 1.0f32;
                            (*rc).row_preds[i as usize][j_0 as usize].decay = 0.5f32;
                            (*rc).row_preds[i as usize][j_0 as usize].offset = 0.0f32;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                    (*(*rc).pred_b_from_p).coeff_min = (0.5f64
                        / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                        as ::core::ffi::c_float;
                    (*(*rc).pred_b_from_p).coeff = 0.5f32;
                    (*(*rc).pred_b_from_p).count = 1.0f32;
                    (*(*rc).pred_b_from_p).decay = 0.5f32;
                    (*(*rc).pred_b_from_p).offset = 0.0f32;
                    if parse_zones(h) < 0 as ::core::ffi::c_int {
                        crate::src::common::common::x264_8_log(
                            h as *mut crate::src::common::common::x264_t,
                            crate::x264_h::X264_LOG_ERROR_1,
                            b"failed to parse zones\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    if (*h).param.rc.b_stat_read != 0 {
                        '_c2rust_label: {
                            if !(*h).param.rc.psz_stat_in.is_null() {
                            } else {
                                crate::stdlib::__assert_fail(
                                    b"h->param.rc.psz_stat_in\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"encoder/ratecontrol.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    874 as ::core::ffi::c_uint,
                                    b"int x264_8_ratecontrol_new(x264_t *)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        let mut stats_in: *mut ::core::ffi::c_char =
                            crate::src::common::base::x264_slurp_file((*h).param.rc.psz_stat_in);
                        let mut stats_buf: *mut ::core::ffi::c_char = stats_in;
                        if stats_buf.is_null() {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"ratecontrol_init: can't open stats file\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        if (*h).param.rc.b_mb_tree != 0 {
                            let mut mbtree_stats_in: *mut ::core::ffi::c_char = strcat_filename(
                                (*h).param.rc.psz_stat_in,
                                b".mbtree\0".as_ptr() as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                            if mbtree_stats_in.is_null() {
                                return -(1 as ::core::ffi::c_int);
                            }
                            (*rc).p_mbtree_stat_file_in = crate::stdlib::fopen(
                                mbtree_stats_in,
                                b"rb\0".as_ptr() as *const ::core::ffi::c_char,
                            )
                                as *mut crate::stdlib::FILE;
                            crate::src::common::base::x264_free(
                                mbtree_stats_in as *mut ::core::ffi::c_void,
                            );
                            if (*rc).p_mbtree_stat_file_in.is_null() {
                                crate::src::common::common::x264_8_log(
                                    h as *mut crate::src::common::common::x264_t,
                                    crate::x264_h::X264_LOG_ERROR_1,
                                    b"ratecontrol_init: can't open mbtree stats file\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                        }
                        if crate::stdlib::strncmp(
                            stats_buf,
                            b"#options:\0".as_ptr() as *const ::core::ffi::c_char,
                            9 as crate::__stddef_size_t_h::size_t,
                        ) != 0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"options list in stats file not valid\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        let mut i_0: ::core::ffi::c_int = 0;
                        let mut j_1: ::core::ffi::c_int = 0;
                        let mut k: crate::stdlib::uint32_t = 0;
                        let mut l: crate::stdlib::uint32_t = 0;
                        let mut opts: *mut ::core::ffi::c_char = stats_buf;
                        stats_in = crate::stdlib::strchr(stats_buf, '\n' as i32);
                        if stats_in.is_null() {
                            return -(1 as ::core::ffi::c_int);
                        }
                        *stats_in = '\0' as i32 as ::core::ffi::c_char;
                        stats_in = stats_in.offset(1);
                        if crate::stdlib::sscanf(
                            opts,
                            b"#options: %dx%d\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut i_0,
                            &raw mut j_1,
                        ) != 2 as ::core::ffi::c_int
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"resolution specified in stats file not valid\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        } else if (*h).param.rc.b_mb_tree != 0 {
                            (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize] = i_0;
                            (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize] = j_1;
                        }
                        let mut res_factor: ::core::ffi::c_float = (*h).param.i_width
                            as ::core::ffi::c_float
                            * (*h).param.i_height as ::core::ffi::c_float
                            / (i_0 * j_1) as ::core::ffi::c_float;
                        let mut res_factor_bits: ::core::ffi::c_float =
                            crate::stdlib::powf(res_factor, 0.7f32);
                        let mut p: *mut ::core::ffi::c_char = crate::stdlib::strstr(
                            opts,
                            b"timebase=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if p.is_null()
                            || crate::stdlib::sscanf(
                                p,
                                b"timebase=%u/%u\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut k,
                                &raw mut l,
                            ) != 2 as ::core::ffi::c_int
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"timebase specified in stats file not valid\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        if k != (*h).param.i_timebase_num || l != (*h).param.i_timebase_den {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"timebase mismatch with 1st pass (%u/%u vs %u/%u)\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.i_timebase_num,
                                (*h).param.i_timebase_den,
                                k,
                                l,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"bitdepth=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"bitdepth=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && 8 as ::core::ffi::c_int != i_0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"different bitdepth setting than first pass (%d vs %d)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                8 as ::core::ffi::c_int,
                                i_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"weightp=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"weightp=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && (if 0 as ::core::ffi::c_int > (*h).param.analyse.i_weighted_pred {
                                0 as ::core::ffi::c_int
                            } else {
                                (*h).param.analyse.i_weighted_pred
                            }) != i_0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"different weightp setting than first pass (%d vs %d)\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                if 0 as ::core::ffi::c_int > (*h).param.analyse.i_weighted_pred {
                                    0 as ::core::ffi::c_int
                                } else {
                                    (*h).param.analyse.i_weighted_pred
                                },
                                i_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"bframes=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"bframes=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && (*h).param.i_bframe != i_0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"different bframes setting than first pass (%d vs %d)\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.i_bframe,
                                i_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"b_pyramid=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"b_pyramid=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && (*h).param.i_bframe_pyramid != i_0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"different b_pyramid setting than first pass (%d vs %d)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.i_bframe_pyramid,
                                i_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"intra_refresh=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"intra_refresh=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && (*h).param.b_intra_refresh != i_0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"different intra_refresh setting than first pass (%d vs %d)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.b_intra_refresh,
                                i_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"open_gop=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"open_gop=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && (*h).param.b_open_gop != i_0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"different open_gop setting than first pass (%d vs %d)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.b_open_gop,
                                i_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"bluray_compat=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"bluray_compat=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && (*h).param.b_bluray_compat != i_0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"different bluray_compat setting than first pass (%d vs %d)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.b_bluray_compat,
                                i_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"mbtree=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"mbtree=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && (*h).param.rc.b_mb_tree != i_0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"different mbtree setting than first pass (%d vs %d)\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.rc.b_mb_tree,
                                i_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"interlaced=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null() {
                            let mut current: *mut ::core::ffi::c_char =
                                (if (*h).param.b_interlaced != 0 {
                                    if (*h).param.b_tff != 0 {
                                        b"tff\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"bff\0".as_ptr() as *const ::core::ffi::c_char
                                    }
                                } else if (*h).param.b_fake_interlaced != 0 {
                                    b"fake\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"0\0".as_ptr() as *const ::core::ffi::c_char
                                }) as *mut ::core::ffi::c_char;
                            let mut buf: [::core::ffi::c_char; 5] = [0; 5];
                            crate::stdlib::sscanf(
                                p,
                                b"interlaced=%4s\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut buf as *mut ::core::ffi::c_char,
                            );
                            if crate::stdlib::strcmp(
                                current,
                                &raw mut buf as *mut ::core::ffi::c_char,
                            ) != 0
                            {
                                crate::src::common::common::x264_8_log(
                                    h as *mut crate::src::common::common::x264_t,
                                    crate::x264_h::X264_LOG_ERROR_1,
                                    b"different interlaced setting than first pass (%s vs %s)\n\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    current,
                                    &raw mut buf as *mut ::core::ffi::c_char,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"keyint=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null() {
                            p = p.offset(7 as ::core::ffi::c_int as isize);
                            let mut buf_0: [::core::ffi::c_char; 13] =
                                ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(
                                    *b"infinite \0\0\0\0",
                                );
                            if (*h).param.i_keyint_max != crate::x264_h::X264_KEYINT_MAX_INFINITE {
                                crate::stdlib::sprintf(
                                    &raw mut buf_0 as *mut ::core::ffi::c_char,
                                    b"%d \0".as_ptr() as *const ::core::ffi::c_char,
                                    (*h).param.i_keyint_max,
                                );
                            }
                            if crate::stdlib::strncmp(
                                p,
                                &raw mut buf_0 as *mut ::core::ffi::c_char,
                                crate::stdlib::strlen(&raw mut buf_0 as *mut ::core::ffi::c_char),
                            ) != 0
                            {
                                crate::src::common::common::x264_8_log(
                                    h as *mut crate::src::common::common::x264_t,
                                    crate::x264_h::X264_LOG_ERROR_1,
                                    b"different keyint setting than first pass (%.*s vs %.*s)\n\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    crate::stdlib::strlen(
                                        &raw mut buf_0 as *mut ::core::ffi::c_char,
                                    )
                                    .wrapping_sub(1 as crate::__stddef_size_t_h::size_t),
                                    &raw mut buf_0 as *mut ::core::ffi::c_char,
                                    crate::stdlib::strcspn(
                                        p,
                                        b" \0".as_ptr() as *const ::core::ffi::c_char,
                                    ),
                                    p,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                        }
                        if !crate::stdlib::strstr(
                            opts,
                            b"qp=0\0".as_ptr() as *const ::core::ffi::c_char,
                        )
                        .is_null()
                            && (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_WARNING_1,
                                b"1st pass was lossless, bitrate prediction will be inaccurate\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        if crate::stdlib::strstr(
                            opts,
                            b"direct=3\0".as_ptr() as *const ::core::ffi::c_char,
                        )
                        .is_null()
                            && (*h).param.analyse.i_direct_mv_pred
                                == crate::x264_h::X264_DIRECT_PRED_AUTO
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_WARNING_1,
                                b"direct=auto not used on the first pass\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            (*h).mb.b_direct_auto_write = 1 as ::core::ffi::c_int;
                        }
                        p = crate::stdlib::strstr(
                            opts,
                            b"b_adapt=\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !p.is_null()
                            && crate::stdlib::sscanf(
                                p,
                                b"b_adapt=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                            && i_0 >= crate::x264_h::X264_B_ADAPT_NONE
                            && i_0 <= crate::x264_h::X264_B_ADAPT_TRELLIS
                        {
                            (*h).param.i_bframe_adaptive = i_0;
                        } else if (*h).param.i_bframe != 0 {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"b_adapt method specified in stats file not valid\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        if ((*h).param.rc.b_mb_tree != 0 || (*h).param.rc.i_vbv_buffer_size != 0)
                            && {
                                p = crate::stdlib::strstr(
                                    opts,
                                    b"rc_lookahead=\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                !p.is_null()
                            }
                            && crate::stdlib::sscanf(
                                p,
                                b"rc_lookahead=%d\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut i_0,
                            ) != 0
                        {
                            (*h).param.rc.i_lookahead = i_0;
                        }
                        p = stats_in;
                        let mut num_entries: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
                        while !p.is_null() {
                            p = crate::stdlib::strchr(
                                p.offset(1 as ::core::ffi::c_int as isize),
                                ';' as i32,
                            );
                            num_entries += 1;
                        }
                        if num_entries == 0 {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"empty stats file\n\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        (*rc).num_entries = num_entries;
                        if (*h).param.i_frame_total < (*rc).num_entries
                            && (*h).param.i_frame_total > 0 as ::core::ffi::c_int
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_WARNING_1,
                                b"2nd pass has fewer frames than 1st pass (%d vs %d)\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.i_frame_total,
                                (*rc).num_entries,
                            );
                        }
                        if (*h).param.i_frame_total > (*rc).num_entries {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"2nd pass has more frames than 1st pass (%d vs %d)\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*h).param.i_frame_total,
                                (*rc).num_entries,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        (*rc).entry = crate::src::common::base::x264_malloc(
                            ((*rc).num_entries as usize).wrapping_mul(::core::mem::size_of::<
                                ratecontrol_entry_t,
                            >(
                            )
                                as usize) as crate::stdlib::int64_t,
                        ) as *mut ratecontrol_entry_t;
                        if (*rc).entry.is_null() {
                            c2rust_current_block = 2822377315416241595;
                        } else {
                            crate::stdlib::memset(
                                (*rc).entry as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ((*rc).num_entries as crate::__stddef_size_t_h::size_t)
                                    .wrapping_mul(::core::mem::size_of::<ratecontrol_entry_t>()
                                        as crate::__stddef_size_t_h::size_t),
                            );
                            (*rc).entry_out =
                                crate::src::common::base::x264_malloc(
                                    ((*rc).num_entries as usize).wrapping_mul(
                                        ::core::mem::size_of::<*mut ratecontrol_entry_t>() as usize,
                                    ) as crate::stdlib::int64_t,
                                ) as *mut *mut ratecontrol_entry_t;
                            if (*rc).entry_out.is_null() {
                                c2rust_current_block = 2822377315416241595;
                            } else {
                                let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while i_1 < (*rc).num_entries {
                                    let mut rce: *mut ratecontrol_entry_t =
                                        (*rc).entry.offset(i_1 as isize)
                                            as *mut ratecontrol_entry_t;
                                    (*rce).pict_type = crate::src::common::base::SLICE_TYPE_P
                                        as ::core::ffi::c_int;
                                    (*rce).new_qscale = qp2qscale(
                                        (20 as ::core::ffi::c_int
                                            + crate::src::common::common::QP_BD_OFFSET)
                                            as ::core::ffi::c_float,
                                    )
                                        as ::core::ffi::c_double;
                                    (*rce).qscale = (*rce).new_qscale;
                                    (*rce).misc_bits = (*rc).nmb + 10 as ::core::ffi::c_int;
                                    (*rce).new_qp = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
                                    let ref mut c2rust_fresh0 =
                                        *(*rc).entry_out.offset(i_1 as isize);
                                    *c2rust_fresh0 = rce;
                                    i_1 += 1;
                                }
                                p = stats_in;
                                let mut total_qp_aq: ::core::ffi::c_double =
                                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
                                let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while i_2 < (*rc).num_entries {
                                    let mut frame_number: ::core::ffi::c_int =
                                        0 as ::core::ffi::c_int;
                                    let mut frame_out_number: ::core::ffi::c_int =
                                        0 as ::core::ffi::c_int;
                                    let mut pict_type: ::core::ffi::c_char =
                                        0 as ::core::ffi::c_char;
                                    let mut qp_rc: ::core::ffi::c_float = 0.;
                                    let mut qp_aq: ::core::ffi::c_float = 0.;
                                    let mut ref_0: ::core::ffi::c_int = 0;
                                    let mut next: *mut ::core::ffi::c_char =
                                        crate::stdlib::strchr(p, ';' as i32);
                                    if !next.is_null() {
                                        let c2rust_fresh1 = next;
                                        next = next.offset(1);
                                        *c2rust_fresh1 = 0 as ::core::ffi::c_char;
                                    }
                                    let mut e: ::core::ffi::c_int = crate::stdlib::sscanf(
                                        p,
                                        b" in:%d out:%d \0".as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut frame_number,
                                        &raw mut frame_out_number,
                                    );
                                    if frame_number < 0 as ::core::ffi::c_int
                                        || frame_number >= (*rc).num_entries
                                    {
                                        crate::src::common::common::x264_8_log(
                                            h as *mut crate::src::common::common::x264_t,
                                            crate::x264_h::X264_LOG_ERROR_1,
                                            b"bad frame number (%d) at stats line %d\n\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            frame_number,
                                            i_2,
                                        );
                                        return -(1 as ::core::ffi::c_int);
                                    }
                                    if frame_out_number < 0 as ::core::ffi::c_int
                                        || frame_out_number >= (*rc).num_entries
                                    {
                                        crate::src::common::common::x264_8_log(
                                            h as *mut crate::src::common::common::x264_t,
                                            crate::x264_h::X264_LOG_ERROR_1,
                                            b"bad frame output number (%d) at stats line %d\n\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            frame_out_number,
                                            i_2,
                                        );
                                        return -(1 as ::core::ffi::c_int);
                                    }
                                    let mut rce_0: *mut ratecontrol_entry_t =
                                        (*rc).entry.offset(frame_number as isize)
                                            as *mut ratecontrol_entry_t;
                                    let ref mut c2rust_fresh2 =
                                        *(*rc).entry_out.offset(frame_out_number as isize);
                                    *c2rust_fresh2 = rce_0;
                                    (*rce_0).direct_mode = 0 as ::core::ffi::c_char;
                                    e
                                        += crate::stdlib::sscanf(
                                            p,
                                            b" in:%*d out:%*d type:%c dur:%ld cpbdur:%ld q:%f aq:%f tex:%d mv:%d misc:%d imb:%d pmb:%d smb:%d d:%c\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            &raw mut pict_type,
                                            &raw mut (*rce_0).i_duration,
                                            &raw mut (*rce_0).i_cpb_duration,
                                            &raw mut qp_rc,
                                            &raw mut qp_aq,
                                            &raw mut (*rce_0).tex_bits,
                                            &raw mut (*rce_0).mv_bits,
                                            &raw mut (*rce_0).misc_bits,
                                            &raw mut (*rce_0).i_count,
                                            &raw mut (*rce_0).p_count,
                                            &raw mut (*rce_0).s_count,
                                            &raw mut (*rce_0).direct_mode,
                                        );
                                    (*rce_0).tex_bits = ((*rce_0).tex_bits as ::core::ffi::c_float
                                        * res_factor_bits)
                                        as ::core::ffi::c_int;
                                    (*rce_0).mv_bits = ((*rce_0).mv_bits as ::core::ffi::c_float
                                        * res_factor_bits)
                                        as ::core::ffi::c_int;
                                    (*rce_0).misc_bits = ((*rce_0).misc_bits
                                        as ::core::ffi::c_float
                                        * res_factor_bits)
                                        as ::core::ffi::c_int;
                                    (*rce_0).i_count = ((*rce_0).i_count as ::core::ffi::c_float
                                        * res_factor)
                                        as ::core::ffi::c_int;
                                    (*rce_0).p_count = ((*rce_0).p_count as ::core::ffi::c_float
                                        * res_factor)
                                        as ::core::ffi::c_int;
                                    (*rce_0).s_count = ((*rce_0).s_count as ::core::ffi::c_float
                                        * res_factor)
                                        as ::core::ffi::c_int;
                                    p = crate::stdlib::strstr(
                                        p,
                                        b"ref:\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    if !p.is_null() {
                                        p = p.offset(4 as ::core::ffi::c_int as isize);
                                        ref_0 = 0 as ::core::ffi::c_int;
                                        loop {
                                            if !(ref_0 < 16 as ::core::ffi::c_int) {
                                                c2rust_current_block = 9728093949049737828;
                                                break;
                                            }
                                            if crate::stdlib::sscanf(
                                                p,
                                                b" %d\0".as_ptr() as *const ::core::ffi::c_char,
                                                (&raw mut (*rce_0).refcount
                                                    as *mut ::core::ffi::c_int)
                                                    .offset(ref_0 as isize)
                                                    as *mut ::core::ffi::c_int,
                                            ) != 1 as ::core::ffi::c_int
                                            {
                                                c2rust_current_block = 9728093949049737828;
                                                break;
                                            }
                                            p = crate::stdlib::strchr(
                                                p.offset(1 as ::core::ffi::c_int as isize),
                                                ' ' as i32,
                                            );
                                            if p.is_null() {
                                                c2rust_current_block = 5991871592215859566;
                                                break;
                                            }
                                            ref_0 += 1;
                                        }
                                        match c2rust_current_block {
                                            5991871592215859566 => {}
                                            _ => {
                                                (*rce_0).refs = ref_0;
                                                (*rce_0).i_weight_denom
                                                    [1 as ::core::ffi::c_int as usize] = -(1
                                                    as ::core::ffi::c_int)
                                                    as crate::stdlib::int16_t;
                                                (*rce_0).i_weight_denom
                                                    [0 as ::core::ffi::c_int as usize] = (*rce_0)
                                                    .i_weight_denom
                                                    [1 as ::core::ffi::c_int as usize];
                                                w = crate::stdlib::strchr(p, 'w' as i32);
                                                if !w.is_null() {
                                                    let mut count: ::core::ffi::c_int =
                                                        crate::stdlib::sscanf(
                                                            w,
                                                            b"w:%hd,%hd,%hd,%hd,%hd,%hd,%hd,%hd\0"
                                                                .as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                            (&raw mut (*rce_0).i_weight_denom
                                                                as *mut crate::stdlib::int16_t)
                                                                .offset(
                                                                    0 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t,
                                                            (&raw mut *(&raw mut (*rce_0).weight
                                                                as *mut [crate::stdlib::int16_t; 2])
                                                                .offset(
                                                                    0 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t)
                                                                .offset(
                                                                    0 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t,
                                                            (&raw mut *(&raw mut (*rce_0).weight
                                                                as *mut [crate::stdlib::int16_t; 2])
                                                                .offset(
                                                                    0 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t)
                                                                .offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t,
                                                            (&raw mut (*rce_0).i_weight_denom
                                                                as *mut crate::stdlib::int16_t)
                                                                .offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t,
                                                            (&raw mut *(&raw mut (*rce_0).weight
                                                                as *mut [crate::stdlib::int16_t; 2])
                                                                .offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t)
                                                                .offset(
                                                                    0 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t,
                                                            (&raw mut *(&raw mut (*rce_0).weight
                                                                as *mut [crate::stdlib::int16_t; 2])
                                                                .offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t)
                                                                .offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t,
                                                            (&raw mut *(&raw mut (*rce_0).weight
                                                                as *mut [crate::stdlib::int16_t; 2])
                                                                .offset(
                                                                    2 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t)
                                                                .offset(
                                                                    0 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t,
                                                            (&raw mut *(&raw mut (*rce_0).weight
                                                                as *mut [crate::stdlib::int16_t; 2])
                                                                .offset(
                                                                    2 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t)
                                                                .offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut crate::stdlib::int16_t,
                                                        );
                                                    if count == 3 as ::core::ffi::c_int {
                                                        (*rce_0).i_weight_denom
                                                            [1 as ::core::ffi::c_int as usize] = -(1
                                                            as ::core::ffi::c_int)
                                                            as crate::stdlib::int16_t;
                                                    } else if count != 8 as ::core::ffi::c_int {
                                                        (*rce_0).i_weight_denom
                                                            [1 as ::core::ffi::c_int as usize] = -(1
                                                            as ::core::ffi::c_int)
                                                            as crate::stdlib::int16_t;
                                                        (*rce_0).i_weight_denom
                                                            [0 as ::core::ffi::c_int as usize] =
                                                            (*rce_0).i_weight_denom
                                                                [1 as ::core::ffi::c_int as usize];
                                                    }
                                                }
                                                if pict_type as ::core::ffi::c_int != 'b' as i32 {
                                                    (*rce_0).kept_as_ref = 1 as ::core::ffi::c_int;
                                                }
                                                match pict_type as ::core::ffi::c_int {
                                                    73 => {
                                                        (*rce_0).frame_type =
                                                            crate::x264_h::X264_TYPE_IDR;
                                                        (*rce_0).pict_type =
                                                            crate::src::common::base::SLICE_TYPE_I
                                                                as ::core::ffi::c_int;
                                                    }
                                                    105 => {
                                                        (*rce_0).frame_type =
                                                            crate::x264_h::X264_TYPE_I;
                                                        (*rce_0).pict_type =
                                                            crate::src::common::base::SLICE_TYPE_I
                                                                as ::core::ffi::c_int;
                                                    }
                                                    80 => {
                                                        (*rce_0).frame_type =
                                                            crate::x264_h::X264_TYPE_P;
                                                        (*rce_0).pict_type =
                                                            crate::src::common::base::SLICE_TYPE_P
                                                                as ::core::ffi::c_int;
                                                    }
                                                    66 => {
                                                        (*rce_0).frame_type =
                                                            crate::x264_h::X264_TYPE_BREF;
                                                        (*rce_0).pict_type =
                                                            crate::src::common::base::SLICE_TYPE_B
                                                                as ::core::ffi::c_int;
                                                    }
                                                    98 => {
                                                        (*rce_0).frame_type =
                                                            crate::x264_h::X264_TYPE_B;
                                                        (*rce_0).pict_type =
                                                            crate::src::common::base::SLICE_TYPE_B
                                                                as ::core::ffi::c_int;
                                                    }
                                                    _ => {
                                                        e = -(1 as ::core::ffi::c_int);
                                                    }
                                                }
                                                if !(e < 14 as ::core::ffi::c_int) {
                                                    (*rce_0).qscale =
                                                        qp2qscale(qp_rc) as ::core::ffi::c_double;
                                                    total_qp_aq += qp_aq as ::core::ffi::c_double;
                                                    p = next;
                                                    i_2 += 1;
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    crate::src::common::common::x264_8_log(
                                        h as *mut crate::src::common::common::x264_t,
                                        crate::x264_h::X264_LOG_ERROR_1,
                                        b"statistics are damaged at line %d, parser out=%d\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        i_2,
                                        e,
                                    );
                                    return -(1 as ::core::ffi::c_int);
                                }
                                if (*h).param.b_stitchable == 0 {
                                    (*(&raw mut (*h).pps
                                        as *mut crate::src::common::set::x264_pps_t))
                                        .i_pic_init_qp = if ((total_qp_aq
                                        / (*rc).num_entries as ::core::ffi::c_double
                                        + 0.5f64)
                                        as ::core::ffi::c_int)
                                        < 51 as ::core::ffi::c_int
                                            + 6 as ::core::ffi::c_int
                                                * (8 as ::core::ffi::c_int
                                                    - 8 as ::core::ffi::c_int)
                                    {
                                        (total_qp_aq / (*rc).num_entries as ::core::ffi::c_double
                                            + 0.5f64)
                                            as ::core::ffi::c_int
                                    } else {
                                        51 as ::core::ffi::c_int
                                            + 6 as ::core::ffi::c_int
                                                * (8 as ::core::ffi::c_int
                                                    - 8 as ::core::ffi::c_int)
                                    };
                                }
                                crate::src::common::base::x264_free(
                                    stats_buf as *mut ::core::ffi::c_void,
                                );
                                if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR {
                                    if init_pass2(h) < 0 as ::core::ffi::c_int {
                                        return -(1 as ::core::ffi::c_int);
                                    }
                                }
                                c2rust_current_block = 2756754640271984560;
                            }
                        }
                    } else {
                        c2rust_current_block = 2756754640271984560;
                    }
                    match c2rust_current_block {
                        2822377315416241595 => {}
                        _ => {
                            if (*h).param.rc.b_stat_write != 0 {
                                (*rc).psz_stat_file_tmpname = strcat_filename(
                                    (*h).param.rc.psz_stat_out,
                                    b".temp\0".as_ptr() as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                if (*rc).psz_stat_file_tmpname.is_null() {
                                    return -(1 as ::core::ffi::c_int);
                                }
                                (*rc).p_stat_file_out = crate::stdlib::fopen(
                                    (*rc).psz_stat_file_tmpname,
                                    b"wb\0".as_ptr() as *const ::core::ffi::c_char,
                                )
                                    as *mut crate::stdlib::FILE;
                                if (*rc).p_stat_file_out.is_null() {
                                    crate::src::common::common::x264_8_log(
                                        h as *mut crate::src::common::common::x264_t,
                                        crate::x264_h::X264_LOG_ERROR_1,
                                        b"ratecontrol_init: can't open stats file\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    return -(1 as ::core::ffi::c_int);
                                }
                                let mut p_0: *mut ::core::ffi::c_char =
                                    crate::src::common::base::x264_param2string(
                                        &raw mut (*h).param as *mut _
                                            as *mut crate::x264_h::x264_param_t,
                                        1 as ::core::ffi::c_int,
                                    );
                                if !p_0.is_null() {
                                    crate::stdlib::fprintf(
                                        (*rc).p_stat_file_out,
                                        b"#options: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                        p_0,
                                    );
                                }
                                crate::src::common::base::x264_free(
                                    p_0 as *mut ::core::ffi::c_void,
                                );
                                if (*h).param.rc.b_mb_tree != 0 && (*h).param.rc.b_stat_read == 0 {
                                    (*rc).psz_mbtree_stat_file_tmpname = strcat_filename(
                                        (*h).param.rc.psz_stat_out,
                                        b".mbtree.temp\0".as_ptr() as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                    (*rc).psz_mbtree_stat_file_name = strcat_filename(
                                        (*h).param.rc.psz_stat_out,
                                        b".mbtree\0".as_ptr() as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                    );
                                    if (*rc).psz_mbtree_stat_file_tmpname.is_null()
                                        || (*rc).psz_mbtree_stat_file_name.is_null()
                                    {
                                        return -(1 as ::core::ffi::c_int);
                                    }
                                    (*rc).p_mbtree_stat_file_out = crate::stdlib::fopen(
                                        (*rc).psz_mbtree_stat_file_tmpname,
                                        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
                                    )
                                        as *mut crate::stdlib::FILE;
                                    if (*rc).p_mbtree_stat_file_out.is_null() {
                                        crate::src::common::common::x264_8_log(
                                            h as *mut crate::src::common::common::x264_t,
                                            crate::x264_h::X264_LOG_ERROR_1,
                                            b"ratecontrol_init: can't open mbtree stats file\n\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        return -(1 as ::core::ffi::c_int);
                                    }
                                }
                            }
                            if (*h).param.rc.b_mb_tree != 0
                                && ((*h).param.rc.b_stat_read != 0
                                    || (*h).param.rc.b_stat_write != 0)
                            {
                                if (*h).param.rc.b_stat_read == 0 {
                                    (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize] =
                                        (*h).param.i_width;
                                    (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize] =
                                        (*h).param.i_height;
                                }
                                if macroblock_tree_rescale_init(h, rc) < 0 as ::core::ffi::c_int {
                                    return -(1 as ::core::ffi::c_int);
                                }
                            }
                            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_3 < (*h).param.i_threads {
                                (*(*h).thread[i_3 as usize]).rc = rc.offset(i_3 as isize);
                                if i_3 != 0 {
                                    *rc.offset(i_3 as isize) =
                                        *rc.offset(0 as ::core::ffi::c_int as isize);
                                    (*(*h).thread[i_3 as usize]).param = (*h).param;
                                    (*(*h).thread[i_3 as usize]).mb.b_variable_qp =
                                        (*h).mb.b_variable_qp;
                                    (*(*h).thread[i_3 as usize]).mb.ip_offset = (*h).mb.ip_offset;
                                }
                                i_3 += 1;
                            }
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
        return -(1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn parse_zone(
    mut h: *mut crate::src::common::common::x264_t,
    mut z: *mut crate::x264_h::x264_zone_t,
    mut p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut tok: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*z).param = ::core::ptr::null_mut::<crate::x264_h::x264_param_t>();
        (*z).f_bitrate_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
        if 3 as ::core::ffi::c_int
            <= crate::stdlib::sscanf(
                p,
                b"%d,%d,q=%d%n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*z).i_start,
                &raw mut (*z).i_end,
                &raw mut (*z).i_qp,
                &raw mut len,
            )
        {
            (*z).b_force_qp = 1 as ::core::ffi::c_int;
        } else if 3 as ::core::ffi::c_int
            <= crate::stdlib::sscanf(
                p,
                b"%d,%d,b=%f%n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*z).i_start,
                &raw mut (*z).i_end,
                &raw mut (*z).f_bitrate_factor,
                &raw mut len,
            )
        {
            (*z).b_force_qp = 0 as ::core::ffi::c_int;
        } else if 2 as ::core::ffi::c_int
            <= crate::stdlib::sscanf(
                p,
                b"%d,%d%n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*z).i_start,
                &raw mut (*z).i_end,
                &raw mut len,
            )
        {
            (*z).b_force_qp = 0 as ::core::ffi::c_int;
        } else {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"invalid zone: \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                p,
            );
            return -(1 as ::core::ffi::c_int);
        }
        p = p.offset(len as isize);
        if *p == 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*z).param = crate::src::common::base::x264_malloc(::core::mem::size_of::<
            crate::x264_h::x264_param_t,
        >() as crate::stdlib::int64_t) as *mut crate::x264_h::x264_param_t;
        if (*z).param.is_null() {
            return -(1 as ::core::ffi::c_int);
        } else {
            crate::stdlib::memcpy(
                (*z).param as *mut ::core::ffi::c_void,
                &raw mut (*h).param as *const ::core::ffi::c_void,
                ::core::mem::size_of::<crate::x264_h::x264_param_t>()
                    as crate::__stddef_size_t_h::size_t,
            );
            (*(*z).param).opaque = crate::__stddef_null_h::NULL;
            (*(*z).param).param_free = Some(
                crate::src::common::base::x264_free
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            )
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
            loop {
                tok = crate::stdlib::strtok_r(
                    p,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut saveptr,
                );
                if tok.is_null() {
                    break;
                }
                let mut val: *mut ::core::ffi::c_char = crate::stdlib::strchr(tok, '=' as i32);
                if !val.is_null() {
                    *val = '\0' as i32 as ::core::ffi::c_char;
                    val = val.offset(1);
                }
                if crate::src::common::base::x264_param_parse(
                    (*z).param as *mut crate::x264_h::x264_param_t
                        as *mut crate::x264_h::x264_param_t,
                    tok,
                    val,
                ) != 0
                {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_ERROR_1,
                        b"invalid zone param: %s = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        tok,
                        val,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                p = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            return 0 as ::core::ffi::c_int;
        };
    }
}
unsafe extern "C" fn parse_zones(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        if !(*h).param.rc.psz_zones.is_null() && (*h).param.rc.i_zones == 0 {
            let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut psz_zones: *mut ::core::ffi::c_char = crate::src::common::base::x264_malloc(
                crate::stdlib::strlen((*h).param.rc.psz_zones)
                    .wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                    as crate::stdlib::int64_t,
            ) as *mut ::core::ffi::c_char;
            if psz_zones.is_null() {
                c2rust_current_block = 17294107561870585226;
            } else {
                crate::stdlib::strcpy(psz_zones, (*h).param.rc.psz_zones);
                (*h).param.rc.i_zones = 1 as ::core::ffi::c_int;
                p = psz_zones;
                while *p != 0 {
                    (*h).param.rc.i_zones +=
                        (*p as ::core::ffi::c_int == '/' as i32) as ::core::ffi::c_int;
                    p = p.offset(1);
                }
                (*h).param.rc.zones = crate::src::common::base::x264_malloc(
                    ((*h).param.rc.i_zones as usize)
                        .wrapping_mul(::core::mem::size_of::<crate::x264_h::x264_zone_t>() as usize)
                        as crate::stdlib::int64_t,
                ) as *mut crate::x264_h::x264_zone_t;
                if (*h).param.rc.zones.is_null() {
                    c2rust_current_block = 17294107561870585226;
                } else {
                    p = psz_zones;
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i < (*h).param.rc.i_zones {
                        let mut i_tok: ::core::ffi::c_int = crate::stdlib::strcspn(
                            p,
                            b"/\0".as_ptr() as *const ::core::ffi::c_char,
                        )
                            as ::core::ffi::c_int;
                        *p.offset(i_tok as isize) = 0 as ::core::ffi::c_char;
                        if parse_zone(
                            h,
                            (*h).param.rc.zones.offset(i as isize)
                                as *mut crate::x264_h::x264_zone_t,
                            p,
                        ) != 0
                        {
                            crate::src::common::base::x264_free(
                                psz_zones as *mut ::core::ffi::c_void,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        p = p.offset((i_tok + 1 as ::core::ffi::c_int) as isize);
                        i += 1;
                    }
                    crate::src::common::base::x264_free(psz_zones as *mut ::core::ffi::c_void);
                    c2rust_current_block = 11584701595673473500;
                }
            }
        } else {
            c2rust_current_block = 11584701595673473500;
        }
        match c2rust_current_block {
            11584701595673473500 => {
                if (*h).param.rc.i_zones > 0 as ::core::ffi::c_int {
                    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_0 < (*h).param.rc.i_zones {
                        let mut z: crate::x264_h::x264_zone_t =
                            *(*h).param.rc.zones.offset(i_0 as isize);
                        if z.i_start < 0 as ::core::ffi::c_int || z.i_start > z.i_end {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"invalid zone: start=%d end=%d\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                z.i_start,
                                z.i_end,
                            );
                            return -(1 as ::core::ffi::c_int);
                        } else if z.b_force_qp == 0
                            && z.f_bitrate_factor <= 0 as ::core::ffi::c_int as ::core::ffi::c_float
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_ERROR_1,
                                b"invalid zone: bitrate_factor=%f\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                z.f_bitrate_factor as ::core::ffi::c_double,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        i_0 += 1;
                    }
                    (*rc).i_zones = (*h).param.rc.i_zones + 1 as ::core::ffi::c_int;
                    (*rc).zones = crate::src::common::base::x264_malloc(
                        ((*rc).i_zones as usize).wrapping_mul(::core::mem::size_of::<
                            crate::x264_h::x264_zone_t,
                        >() as usize) as crate::stdlib::int64_t,
                    ) as *mut crate::x264_h::x264_zone_t;
                    if (*rc).zones.is_null() {
                        c2rust_current_block = 17294107561870585226;
                    } else {
                        crate::stdlib::memcpy(
                            (*rc).zones.offset(1 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_void,
                            (*h).param.rc.zones as *const ::core::ffi::c_void,
                            (((*rc).i_zones - 1 as ::core::ffi::c_int)
                                as crate::__stddef_size_t_h::size_t)
                                .wrapping_mul(::core::mem::size_of::<crate::x264_h::x264_zone_t>()
                                    as crate::__stddef_size_t_h::size_t),
                        );
                        (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).i_start =
                            0 as ::core::ffi::c_int;
                        (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).i_end =
                            crate::limits_h::INT_MAX;
                        (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).b_force_qp =
                            0 as ::core::ffi::c_int;
                        (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).f_bitrate_factor =
                            1 as ::core::ffi::c_int as ::core::ffi::c_float;
                        let ref mut c2rust_fresh3 =
                            (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param;
                        *c2rust_fresh3 =
                            crate::src::common::base::x264_malloc(::core::mem::size_of::<
                                crate::x264_h::x264_param_t,
                            >()
                                as crate::stdlib::int64_t)
                                as *mut crate::x264_h::x264_param_t;
                        if (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize))
                            .param
                            .is_null()
                        {
                            c2rust_current_block = 17294107561870585226;
                        } else {
                            crate::stdlib::memcpy(
                                (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*h).param as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<crate::x264_h::x264_param_t>()
                                    as crate::__stddef_size_t_h::size_t,
                            );
                            let ref mut c2rust_fresh4 =
                                (*(*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param)
                                    .opaque;
                            *c2rust_fresh4 = crate::__stddef_null_h::NULL;
                            let mut i_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                            while i_1 < (*rc).i_zones {
                                if (*(*rc).zones.offset(i_1 as isize)).param.is_null() {
                                    let ref mut c2rust_fresh5 =
                                        (*(*rc).zones.offset(i_1 as isize)).param;
                                    *c2rust_fresh5 =
                                        (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize))
                                            .param;
                                }
                                i_1 += 1;
                            }
                            c2rust_current_block = 2122094917359643297;
                        }
                    }
                } else {
                    c2rust_current_block = 2122094917359643297;
                }
                match c2rust_current_block {
                    17294107561870585226 => {}
                    _ => return 0 as ::core::ffi::c_int,
                }
            }
            _ => {}
        }
        return -(1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn get_zone(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame_num: ::core::ffi::c_int,
) -> *mut crate::x264_h::x264_zone_t {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut i: ::core::ffi::c_int = (*rc).i_zones - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            let mut z: *mut crate::x264_h::x264_zone_t =
                (*rc).zones.offset(i as isize) as *mut crate::x264_h::x264_zone_t;
            if frame_num >= (*z).i_start && frame_num <= (*z).i_end {
                return z;
            }
            i -= 1;
        }
        return ::core::ptr::null_mut::<crate::x264_h::x264_zone_t>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_summary(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        if (*rc).b_abr != 0
            && (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
            && (*rc).cbr_decay > 0.9999f64
        {
            let mut base_cplx: ::core::ffi::c_double = ((*h).mb.i_mb_count
                * (if (*h).param.i_bframe != 0 {
                    120 as ::core::ffi::c_int
                } else {
                    80 as ::core::ffi::c_int
                })) as ::core::ffi::c_double;
            let mut mbtree_offset: ::core::ffi::c_double = if (*h).param.rc.b_mb_tree != 0 {
                (1.0f64 - (*h).param.rc.f_qcompress as ::core::ffi::c_double) * 13.5f64
            } else {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            };
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_INFO,
                b"final ratefactor: %.2f\n\0".as_ptr() as *const ::core::ffi::c_char,
                qscale2qp(
                    (crate::stdlib::pow(
                        base_cplx,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double - (*rc).qcompress,
                    ) * (*rc).cplxr_sum
                        / (*rc).wanted_bits_window) as ::core::ffi::c_float,
                ) as ::core::ffi::c_double
                    - mbtree_offset
                    - crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_double,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_delete(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut b_regular_file: ::core::ffi::c_int = 0;
        if !(*rc).p_stat_file_out.is_null() {
            b_regular_file = x264_is_regular_file((*rc).p_stat_file_out);
            crate::stdlib::fclose((*rc).p_stat_file_out);
            if (*h).i_frame >= (*rc).num_entries && b_regular_file != 0 {
                if crate::stdlib::rename((*rc).psz_stat_file_tmpname, (*h).param.rc.psz_stat_out)
                    != 0 as ::core::ffi::c_int
                {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_ERROR_1,
                        b"failed to rename \"%s\" to \"%s\"\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*rc).psz_stat_file_tmpname,
                        (*h).param.rc.psz_stat_out,
                    );
                }
            }
            crate::src::common::base::x264_free(
                (*rc).psz_stat_file_tmpname as *mut ::core::ffi::c_void,
            );
        }
        if !(*rc).p_mbtree_stat_file_out.is_null() {
            b_regular_file = x264_is_regular_file((*rc).p_mbtree_stat_file_out);
            crate::stdlib::fclose((*rc).p_mbtree_stat_file_out);
            if (*h).i_frame >= (*rc).num_entries && b_regular_file != 0 {
                if crate::stdlib::rename(
                    (*rc).psz_mbtree_stat_file_tmpname,
                    (*rc).psz_mbtree_stat_file_name,
                ) != 0 as ::core::ffi::c_int
                {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_ERROR_1,
                        b"failed to rename \"%s\" to \"%s\"\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*rc).psz_mbtree_stat_file_tmpname,
                        (*rc).psz_mbtree_stat_file_name,
                    );
                }
            }
            crate::src::common::base::x264_free(
                (*rc).psz_mbtree_stat_file_tmpname as *mut ::core::ffi::c_void,
            );
            crate::src::common::base::x264_free(
                (*rc).psz_mbtree_stat_file_name as *mut ::core::ffi::c_void,
            );
        }
        if !(*rc).p_mbtree_stat_file_in.is_null() {
            crate::stdlib::fclose((*rc).p_mbtree_stat_file_in);
        }
        crate::src::common::base::x264_free((*rc).pred as *mut ::core::ffi::c_void);
        crate::src::common::base::x264_free((*rc).pred_b_from_p as *mut ::core::ffi::c_void);
        crate::src::common::base::x264_free((*rc).entry as *mut ::core::ffi::c_void);
        crate::src::common::base::x264_free((*rc).entry_out as *mut ::core::ffi::c_void);
        macroblock_tree_rescale_destroy(rc);
        if !(*rc).zones.is_null() {
            crate::src::common::base::x264_param_cleanup(
                (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param
                    as *mut crate::x264_h::x264_param_t
                    as *mut crate::x264_h::x264_param_t,
            );
            crate::src::common::base::x264_free(
                (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param
                    as *mut ::core::ffi::c_void,
            );
            let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i < (*rc).i_zones {
                if (*(*rc).zones.offset(i as isize)).param
                    != (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param
                    && (*(*(*rc).zones.offset(i as isize)).param)
                        .param_free
                        .is_some()
                {
                    crate::src::common::base::x264_param_cleanup(
                        (*(*rc).zones.offset(i as isize)).param as *mut crate::x264_h::x264_param_t
                            as *mut crate::x264_h::x264_param_t,
                    );
                    (*(*(*rc).zones.offset(i as isize)).param)
                        .param_free
                        .expect("non-null function pointer")(
                        (*(*rc).zones.offset(i as isize)).param as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
            crate::src::common::base::x264_free((*rc).zones as *mut ::core::ffi::c_void);
        }
        crate::src::common::base::x264_free(rc as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn accum_p_qp_update(
    mut h: *mut crate::src::common::common::x264_t,
    mut qp: ::core::ffi::c_float,
) {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        (*rc).accum_p_qp *= 0.95f64;
        (*rc).accum_p_norm *= 0.95f64;
        (*rc).accum_p_norm += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            (*rc).accum_p_qp += qp as ::core::ffi::c_double + (*rc).ip_offset;
        } else {
            (*rc).accum_p_qp += qp as ::core::ffi::c_double;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_zone_init(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut zone: *mut crate::x264_h::x264_zone_t = get_zone(h, (*(*h).fenc).i_frame);
        if !zone.is_null()
            && ((*rc).prev_zone.is_null() || (*zone).param != (*(*rc).prev_zone).param)
        {
            crate::src::encoder::encoder::x264_8_encoder_reconfig_apply(
                h as *mut crate::src::common::common::x264_t,
                (*zone).param as *mut crate::x264_h::x264_param_t
                    as *mut crate::x264_h::x264_param_t,
            );
        }
        (*rc).prev_zone = zone;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_start(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_force_qp: ::core::ffi::c_int,
    mut overhead: ::core::ffi::c_int,
) {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut rce: *mut ratecontrol_entry_t = ::core::ptr::null_mut::<ratecontrol_entry_t>();
        let mut zone: *mut crate::x264_h::x264_zone_t = get_zone(h, (*(*h).fenc).i_frame);
        let mut q: ::core::ffi::c_float = 0.;
        if (*h).param.rc.b_stat_read != 0 {
            let mut frame: ::core::ffi::c_int = (*(*h).fenc).i_frame;
            '_c2rust_label: {
                if frame >= 0 as ::core::ffi::c_int && frame < (*rc).num_entries {
                } else {
                    crate::stdlib::__assert_fail(
                        b"frame >= 0 && frame < rc->num_entries\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"encoder/ratecontrol.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1444 as ::core::ffi::c_uint,
                        b"void x264_8_ratecontrol_start(x264_t *, int, int)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            (*rc).rce = (*rc).entry.offset(frame as isize) as *mut ratecontrol_entry_t;
            rce = (*rc).rce;
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
                && (*h).param.analyse.i_direct_mv_pred == crate::x264_h::X264_DIRECT_PRED_AUTO
            {
                (*h).sh.b_direct_spatial_mv_pred =
                    ((*rce).direct_mode as ::core::ffi::c_int == 's' as i32) as ::core::ffi::c_int;
                (*h).mb.b_direct_auto_read = ((*rce).direct_mode as ::core::ffi::c_int
                    == 's' as i32
                    || (*rce).direct_mode as ::core::ffi::c_int == 't' as i32)
                    as ::core::ffi::c_int;
            }
        }
        if (*rc).b_vbv != 0 {
            crate::stdlib::memset(
                (*(*h).fdec).i_row_bits as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*h).mb.i_mb_height as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                        as crate::__stddef_size_t_h::size_t),
            );
            crate::stdlib::memset(
                (*(*h).fdec).f_row_qp as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*h).mb.i_mb_height as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()
                        as crate::__stddef_size_t_h::size_t),
            );
            crate::stdlib::memset(
                (*(*h).fdec).f_row_qscale as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*h).mb.i_mb_height as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()
                        as crate::__stddef_size_t_h::size_t),
            );
            (*rc).row_pred = &raw mut *(&raw mut (*rc).row_preds as *mut [predictor_t; 2])
                .offset((*h).sh.i_type as isize) as *mut predictor_t;
            (*rc).buffer_rate = (*(*h).fenc).i_cpb_duration as ::core::ffi::c_double
                * (*rc).vbv_max_rate
                * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_num_units_in_tick as ::core::ffi::c_double
                / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as ::core::ffi::c_double;
            update_vbv_plan(h, overhead);
            let mut l: *const crate::x264_h::x264_level_t =
                &raw const crate::src::common::tables::x264_levels
                    as *const crate::x264_h::x264_level_t;
            while (*l).level_idc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && (*l).level_idc as ::core::ffi::c_int != (*h).param.i_level_idc
            {
                l = l.offset(1);
            }
            let mut mincr: ::core::ffi::c_int = (*l).mincr as ::core::ffi::c_int;
            if (*h).param.b_bluray_compat != 0 {
                mincr = 4 as ::core::ffi::c_int;
            }
            if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_profile_idc
                > crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
            {
                (*rc).frame_size_maximum = 1e9f64;
            } else if (*h).i_frame == 0 as ::core::ffi::c_int {
                let mut fr: ::core::ffi::c_double = 1.0f64
                    / (if (*h).param.i_level_idc >= 60 as ::core::ffi::c_int {
                        300 as ::core::ffi::c_int
                    } else {
                        172 as ::core::ffi::c_int
                    }) as ::core::ffi::c_double;
                let mut pic_size_in_mbs: ::core::ffi::c_int =
                    (*h).mb.i_mb_width * (*h).mb.i_mb_height;
                (*rc).frame_size_maximum = (384 as ::core::ffi::c_int * crate::internal::BIT_DEPTH)
                    as ::core::ffi::c_double
                    * (if pic_size_in_mbs as ::core::ffi::c_double
                        > fr * (*l).mbps as ::core::ffi::c_double
                    {
                        pic_size_in_mbs as ::core::ffi::c_double
                    } else {
                        fr * (*l).mbps as ::core::ffi::c_double
                    })
                    / mincr as ::core::ffi::c_double;
            } else {
                (*rc).frame_size_maximum = (384 as ::core::ffi::c_int * crate::internal::BIT_DEPTH)
                    as ::core::ffi::c_double
                    * ((*(*h).fenc).i_cpb_duration as ::core::ffi::c_double
                        * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_num_units_in_tick
                            as ::core::ffi::c_double
                        / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_time_scale as ::core::ffi::c_double)
                    * (*l).mbps as ::core::ffi::c_double
                    / mincr as ::core::ffi::c_double;
            }
        }
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            (*rc).bframes = (*(*h).fenc).i_bframes as ::core::ffi::c_int;
        }
        if (*rc).b_abr != 0 {
            q = qscale2qp(rate_estimate_qscale(h));
        } else if (*rc).b_2pass != 0 {
            (*rce).new_qscale = rate_estimate_qscale(h) as ::core::ffi::c_double;
            q = qscale2qp((*rce).new_qscale as ::core::ffi::c_float);
        } else {
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
                && (*(*h).fdec).b_kept_as_ref != 0
            {
                q = (((*rc).qp_constant
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                    + (*rc).qp_constant
                        [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize])
                    / 2 as ::core::ffi::c_int) as ::core::ffi::c_float;
            } else {
                q = (*rc).qp_constant[(*h).sh.i_type as usize] as ::core::ffi::c_float;
            }
            if !zone.is_null() {
                if (*zone).b_force_qp != 0 {
                    q += ((*zone).i_qp
                        - (*rc).qp_constant
                            [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize])
                        as ::core::ffi::c_float;
                } else {
                    q -= 6 as ::core::ffi::c_int as ::core::ffi::c_float
                        * crate::stdlib::log2f((*zone).f_bitrate_factor);
                }
            }
        }
        if i_force_qp != crate::x264_h::X264_QP_AUTO {
            q = (i_force_qp - 1 as ::core::ffi::c_int) as ::core::ffi::c_float;
        }
        q = x264_clip3f(
            q as ::core::ffi::c_double,
            (*h).param.rc.i_qp_min as ::core::ffi::c_double,
            (*h).param.rc.i_qp_max as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        (*rc).qpa_aq_prev = 0 as ::core::ffi::c_int;
        (*rc).qpa_aq = (*rc).qpa_aq_prev;
        (*rc).qpa_rc_prev = (*rc).qpa_aq as ::core::ffi::c_float;
        (*rc).qpa_rc = (*rc).qpa_rc_prev;
        (*rc).qpm = q;
        (*(*h).fdec).f_qp_avg_aq = (*rc).qpm;
        (*(*h).fdec).f_qp_avg_rc = (*(*h).fdec).f_qp_avg_aq;
        if !rce.is_null() {
            (*rce).new_qp = q;
        }
        accum_p_qp_update(h, (*rc).qpm);
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            (*rc).last_non_b_pict_type = (*h).sh.i_type;
        }
    }
}
unsafe extern "C" fn predict_row_size(
    mut h: *mut crate::src::common::common::x264_t,
    mut y: ::core::ffi::c_int,
    mut qscale: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut pred_s: ::core::ffi::c_float = predict_size(
            (*rc).row_pred.offset(0 as ::core::ffi::c_int as isize) as *mut predictor_t,
            qscale,
            *(*(*h).fdec).i_row_satd.offset(y as isize) as ::core::ffi::c_float,
        );
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
            || qscale
                >= *(*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .f_row_qscale
                    .offset(y as isize)
        {
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                && (*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_type
                    == (*(*h).fdec).i_type
                && *(*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .f_row_qscale
                    .offset(y as isize)
                    > 0 as ::core::ffi::c_int as ::core::ffi::c_float
                && *(*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_row_satd
                    .offset(y as isize)
                    > 0 as ::core::ffi::c_int
                && crate::stdlib::abs(
                    *(*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .i_row_satd
                        .offset(y as isize)
                        - *(*(*h).fdec).i_row_satd.offset(y as isize),
                ) < *(*(*h).fdec).i_row_satd.offset(y as isize) / 2 as ::core::ffi::c_int
            {
                let mut pred_t: ::core::ffi::c_float = (*(*(*h).fref
                    [0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_row_bits
                    .offset(y as isize)
                    * *(*(*h).fdec).i_row_satd.offset(y as isize)
                    / *(*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .i_row_satd
                        .offset(y as isize))
                    as ::core::ffi::c_float
                    * *(*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .f_row_qscale
                        .offset(y as isize)
                    / qscale;
                return (pred_s + pred_t) * 0.5f32;
            }
            return pred_s;
        } else {
            let mut pred_intra: ::core::ffi::c_float = predict_size(
                (*rc).row_pred.offset(1 as ::core::ffi::c_int as isize) as *mut predictor_t,
                qscale,
                *(*(*h).fdec).i_row_satds[0 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize]
                    .offset(y as isize) as ::core::ffi::c_float,
            );
            return pred_intra + pred_s;
        };
    }
}
unsafe extern "C" fn row_bits_so_far(
    mut h: *mut crate::src::common::common::x264_t,
    mut y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = (*h).i_threadslice_start;
        while i <= y {
            bits += *(*(*h).fdec).i_row_bits.offset(i as isize);
            i += 1;
        }
        return bits;
    }
}
unsafe extern "C" fn predict_row_size_to_end(
    mut h: *mut crate::src::common::common::x264_t,
    mut y: ::core::ffi::c_int,
    mut qp: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    unsafe {
        let mut qscale: ::core::ffi::c_float = qp2qscale(qp);
        let mut bits: ::core::ffi::c_float = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        let mut i: ::core::ffi::c_int = y + 1 as ::core::ffi::c_int;
        while i < (*h).i_threadslice_end {
            bits += predict_row_size(h, i, qscale);
            i += 1;
        }
        return bits;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_mb(
    mut h: *mut crate::src::common::common::x264_t,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let y: ::core::ffi::c_int = (*h).mb.i_mb_y;
        *(*(*h).fdec).i_row_bits.offset(y as isize) += bits;
        (*rc).qpa_aq += (*h).mb.i_qp;
        if (*h).mb.i_mb_x != (*h).mb.i_mb_width - 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        (*rc).qpa_rc += (*rc).qpm * (*h).mb.i_mb_width as ::core::ffi::c_float;
        if (*rc).b_vbv == 0 {
            return 0 as ::core::ffi::c_int;
        }
        let mut qscale: ::core::ffi::c_float = qp2qscale((*rc).qpm);
        *(*(*h).fdec).f_row_qp.offset(y as isize) = (*rc).qpm;
        *(*(*h).fdec).f_row_qscale.offset(y as isize) = qscale;
        update_predictor(
            (*rc).row_pred.offset(0 as ::core::ffi::c_int as isize) as *mut predictor_t,
            qscale,
            *(*(*h).fdec).i_row_satd.offset(y as isize) as ::core::ffi::c_float,
            *(*(*h).fdec).i_row_bits.offset(y as isize) as ::core::ffi::c_float,
        );
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
            && (*rc).qpm
                < *(*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .f_row_qp
                    .offset(y as isize)
        {
            update_predictor(
                (*rc).row_pred.offset(1 as ::core::ffi::c_int as isize) as *mut predictor_t,
                qscale,
                *(*(*h).fdec).i_row_satds[0 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize]
                    .offset(y as isize) as ::core::ffi::c_float,
                *(*(*h).fdec).i_row_bits.offset(y as isize) as ::core::ffi::c_float,
            );
        }
        if (*h).sh.b_mbaff != 0 && y & 1 as ::core::ffi::c_int == 0 {
            return 0 as ::core::ffi::c_int;
        }
        let mut can_reencode_row: ::core::ffi::c_int = ((*h).sh.i_first_mb
            <= ((*h).mb.i_mb_y - (*h).sh.b_mbaff) * (*h).mb.i_mb_stride)
            as ::core::ffi::c_int;
        let mut prev_row_qp: ::core::ffi::c_float = *(*(*h).fdec).f_row_qp.offset(y as isize);
        let mut qp_absolute_max: ::core::ffi::c_float =
            (*h).param.rc.i_qp_max as ::core::ffi::c_float;
        if (*rc).rate_factor_max_increment != 0. {
            qp_absolute_max = if qp_absolute_max < (*rc).qp_novbv + (*rc).rate_factor_max_increment
            {
                qp_absolute_max
            } else {
                (*rc).qp_novbv + (*rc).rate_factor_max_increment
            };
        }
        let mut qp_max: ::core::ffi::c_float =
            if (prev_row_qp + (*h).param.rc.i_qp_step as ::core::ffi::c_float) < qp_absolute_max {
                prev_row_qp + (*h).param.rc.i_qp_step as ::core::ffi::c_float
            } else {
                qp_absolute_max
            };
        let mut qp_min: ::core::ffi::c_float = if prev_row_qp
            - (*h).param.rc.i_qp_step as ::core::ffi::c_float
            > (*h).param.rc.i_qp_min as ::core::ffi::c_float
        {
            prev_row_qp - (*h).param.rc.i_qp_step as ::core::ffi::c_float
        } else {
            (*h).param.rc.i_qp_min as ::core::ffi::c_float
        };
        let mut step_size: ::core::ffi::c_float = 0.5f32;
        let mut slice_size_planned: ::core::ffi::c_float = (if (*h).param.b_sliced_threads != 0 {
            (*rc).slice_size_planned
        } else {
            (*rc).frame_size_planned
        }) as ::core::ffi::c_float;
        let mut bits_so_far: ::core::ffi::c_float = row_bits_so_far(h, y) as ::core::ffi::c_float;
        ::core::ptr::write_volatile(
            &mut (*rc).bits_so_far as *mut ::core::ffi::c_float,
            bits_so_far,
        );
        let mut max_frame_error: ::core::ffi::c_float = x264_clip3f(
            1.0f64 / (*h).mb.i_mb_height as ::core::ffi::c_double,
            0.05f64,
            0.25f64,
        ) as ::core::ffi::c_float;
        let mut max_frame_size: ::core::ffi::c_float = ((*rc).frame_size_maximum
            - (*rc).frame_size_maximum * max_frame_error as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        max_frame_size = (if (max_frame_size as ::core::ffi::c_double)
            < (*rc).buffer_fill - (*rc).buffer_rate * max_frame_error as ::core::ffi::c_double
        {
            max_frame_size as ::core::ffi::c_double
        } else {
            (*rc).buffer_fill - (*rc).buffer_rate * max_frame_error as ::core::ffi::c_double
        }) as ::core::ffi::c_float;
        let mut size_of_other_slices: ::core::ffi::c_float =
            0 as ::core::ffi::c_int as ::core::ffi::c_float;
        if (*h).param.b_sliced_threads != 0 {
            let mut bits_so_far_of_other_slices: ::core::ffi::c_float =
                0 as ::core::ffi::c_int as ::core::ffi::c_float;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*h).param.i_threads {
                if h != (*h).thread[i as usize] {
                    size_of_other_slices += (*(*(*h).thread[i as usize]).rc).frame_size_estimated;
                    bits_so_far_of_other_slices += (*(*(*h).thread[i as usize]).rc).bits_so_far;
                }
                i += 1;
            }
            let mut weight: ::core::ffi::c_float = x264_clip3f(
                ((bits_so_far_of_other_slices + (*rc).frame_size_estimated)
                    / (size_of_other_slices + (*rc).frame_size_estimated))
                    as ::core::ffi::c_double,
                0.0f64,
                1.0f64,
            ) as ::core::ffi::c_float;
            let mut frame_size_planned: ::core::ffi::c_float = ((*rc).frame_size_planned
                - (*rc).frame_size_planned * max_frame_error as ::core::ffi::c_double)
                as ::core::ffi::c_float;
            let mut size_of_other_slices_planned: ::core::ffi::c_float =
                ((if frame_size_planned < max_frame_size {
                    frame_size_planned
                } else {
                    max_frame_size
                }) as ::core::ffi::c_double
                    - (*rc).slice_size_planned) as ::core::ffi::c_float;
            size_of_other_slices_planned =
                if size_of_other_slices_planned > bits_so_far_of_other_slices {
                    size_of_other_slices_planned
                } else {
                    bits_so_far_of_other_slices
                };
            size_of_other_slices = (size_of_other_slices - size_of_other_slices_planned) * weight
                + size_of_other_slices_planned;
        }
        if y < (*h).i_threadslice_end - 1 as ::core::ffi::c_int {
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                qp_min = if qp_min
                    > (if *(*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as ::core::ffi::c_int) as isize)
                        > *(*(*h).fref[1 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize])
                            .f_row_qp
                            .offset((y + 1 as ::core::ffi::c_int) as isize)
                    {
                        *(*(*h).fref[0 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize])
                            .f_row_qp
                            .offset((y + 1 as ::core::ffi::c_int) as isize)
                    } else {
                        *(*(*h).fref[1 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize])
                            .f_row_qp
                            .offset((y + 1 as ::core::ffi::c_int) as isize)
                    }) {
                    qp_min
                } else if *(*(*h).fref[0 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as ::core::ffi::c_int) as isize)
                    > *(*(*h).fref[1 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as ::core::ffi::c_int) as isize)
                {
                    *(*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as ::core::ffi::c_int) as isize)
                } else {
                    *(*(*h).fref[1 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as ::core::ffi::c_int) as isize)
                };
                (*rc).qpm = if (*rc).qpm > qp_min {
                    (*rc).qpm
                } else {
                    qp_min
                };
            }
            let mut buffer_left_planned: ::core::ffi::c_float =
                ((*rc).buffer_fill - (*rc).frame_size_planned) as ::core::ffi::c_float;
            buffer_left_planned = if buffer_left_planned > 0.0f32 {
                buffer_left_planned
            } else {
                0.0f32
            };
            let mut rc_tol: ::core::ffi::c_float =
                ((buffer_left_planned / (*h).param.i_threads as ::core::ffi::c_float)
                    as ::core::ffi::c_double
                    * (*rc).rate_tolerance) as ::core::ffi::c_float;
            let mut b1: ::core::ffi::c_float =
                bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
            let mut trust_coeff: ::core::ffi::c_float = x264_clip3f(
                (bits_so_far / slice_size_planned) as ::core::ffi::c_double,
                0.0f64,
                1.0f64,
            ) as ::core::ffi::c_float;
            if trust_coeff < 0.05f32 {
                qp_absolute_max = prev_row_qp;
                qp_max = qp_absolute_max;
            }
            if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
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
                && (b1 as ::core::ffi::c_double
                    > (*rc).frame_size_planned + rc_tol as ::core::ffi::c_double
                    || b1 as ::core::ffi::c_double > (*rc).frame_size_planned
                        && (*rc).qpm < (*rc).qp_novbv
                    || b1 as ::core::ffi::c_double
                        > (*rc).buffer_fill
                            - (buffer_left_planned * 0.5f32) as ::core::ffi::c_double)
            {
                (*rc).qpm += step_size;
                b1 = bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
            }
            let mut b_max: ::core::ffi::c_float = (b1 as ::core::ffi::c_double
                + (((*rc).buffer_fill - (*rc).buffer_size + (*rc).buffer_rate) * 0.90f64
                    - b1 as ::core::ffi::c_double)
                    * trust_coeff as ::core::ffi::c_double)
                as ::core::ffi::c_float;
            (*rc).qpm -= step_size;
            let mut b2: ::core::ffi::c_float =
                bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
            while (*rc).qpm > qp_min
                && (*rc).qpm < prev_row_qp
                && ((*rc).qpm
                    > *(*(*h).fdec)
                        .f_row_qp
                        .offset(0 as ::core::ffi::c_int as isize)
                    || (*rc).single_frame_vbv != 0)
                && b2 < max_frame_size
                && ((b2 as ::core::ffi::c_double) < (*rc).frame_size_planned * 0.8f64 || b2 < b_max)
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
                &mut (*rc).frame_size_estimated as *mut ::core::ffi::c_float,
                b1 - size_of_other_slices,
            );
            if (*rc).qpm > qp_max && prev_row_qp < qp_max && can_reencode_row != 0 {
                (*rc).qpm = x264_clip3f(
                    ((prev_row_qp + (*rc).qpm) * 0.5f32) as ::core::ffi::c_double,
                    (prev_row_qp + 1.0f32) as ::core::ffi::c_double,
                    qp_max as ::core::ffi::c_double,
                ) as ::core::ffi::c_float;
                (*rc).qpa_rc = (*rc).qpa_rc_prev;
                (*rc).qpa_aq = (*rc).qpa_aq_prev;
                *(*(*h).fdec).i_row_bits.offset(y as isize) = 0 as ::core::ffi::c_int;
                *(*(*h).fdec)
                    .i_row_bits
                    .offset((y - (*h).sh.b_mbaff) as isize) = 0 as ::core::ffi::c_int;
                return -(1 as ::core::ffi::c_int);
            }
        } else {
            ::core::ptr::write_volatile(
                &mut (*rc).frame_size_estimated as *mut ::core::ffi::c_float,
                bits_so_far,
            );
            if (*rc).qpm < qp_max
                && can_reencode_row != 0
                && (bits_so_far + size_of_other_slices) as ::core::ffi::c_double
                    > (if (*rc).frame_size_maximum < (*rc).buffer_fill {
                        (*rc).frame_size_maximum
                    } else {
                        (*rc).buffer_fill
                    })
            {
                (*rc).qpm = qp_max;
                (*rc).qpa_rc = (*rc).qpa_rc_prev;
                (*rc).qpa_aq = (*rc).qpa_aq_prev;
                *(*(*h).fdec).i_row_bits.offset(y as isize) = 0 as ::core::ffi::c_int;
                *(*(*h).fdec)
                    .i_row_bits
                    .offset((y - (*h).sh.b_mbaff) as isize) = 0 as ::core::ffi::c_int;
                return -(1 as ::core::ffi::c_int);
            }
        }
        (*rc).qpa_rc_prev = (*rc).qpa_rc;
        (*rc).qpa_aq_prev = (*rc).qpa_aq;
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_qp(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        return x264_clip3(
            ((*(*h).rc).qpm + 0.5f32) as ::core::ffi::c_int,
            (*h).param.rc.i_qp_min,
            (*h).param.rc.i_qp_max,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_mb_qp(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut qp: ::core::ffi::c_float = (*(*h).rc).qpm;
        if (*h).param.rc.i_aq_mode != 0 {
            let mut qp_offset: ::core::ffi::c_float = if (*(*h).fdec).b_kept_as_ref != 0 {
                *(*(*h).fenc).f_qp_offset.offset((*h).mb.i_mb_xy as isize)
            } else {
                *(*(*h).fenc).f_qp_offset_aq.offset((*h).mb.i_mb_xy as isize)
            };
            if qp > crate::src::common::common::QP_MAX_SPEC as ::core::ffi::c_float {
                qp_offset *= (crate::src::common::common::QP_MAX as ::core::ffi::c_float - qp)
                    / (crate::src::common::common::QP_MAX - crate::src::common::common::QP_MAX_SPEC)
                        as ::core::ffi::c_float;
            }
            qp += qp_offset;
        }
        return x264_clip3(
            (qp + 0.5f32) as ::core::ffi::c_int,
            (*h).param.rc.i_qp_min,
            (*h).param.rc.i_qp_max,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_slice_type(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame_num: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        if (*h).param.rc.b_stat_read != 0 {
            if frame_num >= (*rc).num_entries {
                (*h).param.rc.i_qp_constant = (if (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    == 0 as ::core::ffi::c_int
                {
                    (24 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
                        as ::core::ffi::c_double
                } else {
                    1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + (*h).stat.f_frame_qp
                            [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                            / (*h).stat.i_frame_count[crate::src::common::base::SLICE_TYPE_P
                                as ::core::ffi::c_int
                                as usize] as ::core::ffi::c_double
                }) as ::core::ffi::c_int;
                (*rc).qp_constant
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize] =
                    x264_clip3(
                        (*h).param.rc.i_qp_constant,
                        0 as ::core::ffi::c_int,
                        crate::src::common::common::QP_MAX,
                    );
                (*rc).qp_constant
                    [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize] =
                    x264_clip3(
                        (qscale2qp(
                            qp2qscale((*h).param.rc.i_qp_constant as ::core::ffi::c_float)
                                / (*h).param.rc.f_ip_factor,
                        ) as ::core::ffi::c_double
                            + 0.5f64) as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        crate::src::common::common::QP_MAX,
                    );
                (*rc).qp_constant
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize] =
                    x264_clip3(
                        (qscale2qp(
                            qp2qscale((*h).param.rc.i_qp_constant as ::core::ffi::c_float)
                                * (*h).param.rc.f_pb_factor,
                        ) as ::core::ffi::c_double
                            + 0.5f64) as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        crate::src::common::common::QP_MAX,
                    );
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"2nd pass has more frames than 1st pass (%d)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*rc).num_entries,
                );
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"continuing anyway, at constant QP=%d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.rc.i_qp_constant,
                );
                if (*h).param.i_bframe_adaptive != 0 {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_ERROR_1,
                        b"disabling adaptive B-frames\n\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*h).param.i_threads {
                    (*(*(*h).thread[i as usize]).rc).b_abr = 0 as ::core::ffi::c_int;
                    (*(*(*h).thread[i as usize]).rc).b_2pass = 0 as ::core::ffi::c_int;
                    (*(*h).thread[i as usize]).param.rc.i_rc_method = crate::x264_h::X264_RC_CQP;
                    (*(*h).thread[i as usize]).param.rc.b_stat_read = 0 as ::core::ffi::c_int;
                    (*(*h).thread[i as usize]).param.i_bframe_adaptive = 0 as ::core::ffi::c_int;
                    (*(*h).thread[i as usize]).param.i_scenecut_threshold = 0 as ::core::ffi::c_int;
                    (*(*h).thread[i as usize]).param.rc.b_mb_tree = 0 as ::core::ffi::c_int;
                    if (*(*h).thread[i as usize]).param.i_bframe > 1 as ::core::ffi::c_int {
                        (*(*h).thread[i as usize]).param.i_bframe = 1 as ::core::ffi::c_int;
                    }
                    i += 1;
                }
                return crate::x264_h::X264_TYPE_AUTO;
            }
            return (*(*rc).entry.offset(frame_num as isize)).frame_type;
        } else {
            return crate::x264_h::X264_TYPE_AUTO;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_set_weights(
    mut h: *mut crate::src::common::common::x264_t,
    mut frm: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        let mut rce: *mut ratecontrol_entry_t =
            (*(*h).rc).entry.offset((*frm).i_frame as isize) as *mut ratecontrol_entry_t;
        if (*h).param.analyse.i_weighted_pred <= 0 as ::core::ffi::c_int {
            return;
        }
        if (*rce).i_weight_denom[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >= 0 as ::core::ffi::c_int
        {
            (*frm).weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                .i_scale = (*rce).weight[0 as ::core::ffi::c_int as usize]
                [0 as ::core::ffi::c_int as usize]
                as crate::stdlib::int32_t;
            (*frm).weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                .i_denom =
                (*rce).i_weight_denom[0 as ::core::ffi::c_int as usize] as crate::stdlib::int32_t;
            (*frm).weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                .i_offset = (*rce).weight[0 as ::core::ffi::c_int as usize]
                [1 as ::core::ffi::c_int as usize]
                as crate::stdlib::int32_t;
            (*h).mc.weight_cache.expect("non-null function pointer")(
                h,
                (&raw mut *(&raw mut (*frm).weight
                    as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::mc::x264_weight_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::mc::x264_weight_t,
            );
        }
        if (*rce).i_weight_denom[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >= 0 as ::core::ffi::c_int
        {
            (*frm).weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                .i_scale = (*rce).weight[1 as ::core::ffi::c_int as usize]
                [0 as ::core::ffi::c_int as usize]
                as crate::stdlib::int32_t;
            (*frm).weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                .i_denom =
                (*rce).i_weight_denom[1 as ::core::ffi::c_int as usize] as crate::stdlib::int32_t;
            (*frm).weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                .i_offset = (*rce).weight[1 as ::core::ffi::c_int as usize]
                [1 as ::core::ffi::c_int as usize]
                as crate::stdlib::int32_t;
            (*h).mc.weight_cache.expect("non-null function pointer")(
                h,
                (&raw mut *(&raw mut (*frm).weight
                    as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::mc::x264_weight_t)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::mc::x264_weight_t,
            );
            (*frm).weight[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize]
                .i_scale = (*rce).weight[2 as ::core::ffi::c_int as usize]
                [0 as ::core::ffi::c_int as usize]
                as crate::stdlib::int32_t;
            (*frm).weight[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize]
                .i_denom =
                (*rce).i_weight_denom[1 as ::core::ffi::c_int as usize] as crate::stdlib::int32_t;
            (*frm).weight[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize]
                .i_offset = (*rce).weight[2 as ::core::ffi::c_int as usize]
                [1 as ::core::ffi::c_int as usize]
                as crate::stdlib::int32_t;
            (*h).mc.weight_cache.expect("non-null function pointer")(
                h,
                (&raw mut *(&raw mut (*frm).weight
                    as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::mc::x264_weight_t)
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::mc::x264_weight_t,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_ratecontrol_end(
    mut h: *mut crate::src::common::common::x264_t,
    mut bits: ::core::ffi::c_int,
    mut filler: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut mbs: *const ::core::ffi::c_int =
            &raw mut (*h).stat.frame.i_mb_count as *mut ::core::ffi::c_int;
        (*h).stat.frame.i_mb_count_skip = *mbs
            .offset(crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int as isize)
            + *mbs.offset(crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int as isize);
        (*h).stat.frame.i_mb_count_i = *mbs
            .offset(crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int as isize)
            + *mbs.offset(crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int as isize)
            + *mbs.offset(crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int as isize)
            + *mbs.offset(crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as isize);
        (*h).stat.frame.i_mb_count_p = *mbs
            .offset(crate::src::common::macroblock::P_L0 as ::core::ffi::c_int as isize)
            + *mbs.offset(crate::src::common::macroblock::P_8x8 as ::core::ffi::c_int as isize);
        let mut i: ::core::ffi::c_int =
            crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int;
        while i <= crate::src::common::macroblock::B_8x8 as ::core::ffi::c_int {
            (*h).stat.frame.i_mb_count_p += *mbs.offset(i as isize);
            i += 1;
        }
        (*rc).qpa_rc /= (*h).mb.i_mb_count as ::core::ffi::c_float;
        (*(*h).fdec).f_qp_avg_rc = (*rc).qpa_rc;
        (*(*h).fdec).f_qp_avg_aq =
            (*rc).qpa_aq as ::core::ffi::c_float / (*h).mb.i_mb_count as ::core::ffi::c_float;
        (*(*h).fdec).f_crf_avg =
            (*h).param.rc.f_rf_constant + (*(*h).fdec).f_qp_avg_rc - (*rc).qp_novbv;
        if (*h).param.rc.b_stat_write != 0 {
            let mut c_type: ::core::ffi::c_char = (if (*h).sh.i_type
                == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
            {
                if (*(*h).fenc).i_poc == 0 as ::core::ffi::c_int {
                    'I' as i32
                } else {
                    'i' as i32
                }
            } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            {
                'P' as i32
            } else if (*(*h).fenc).b_kept_as_ref != 0 {
                'B' as i32
            } else {
                'b' as i32
            }) as ::core::ffi::c_char;
            let mut dir_frame: ::core::ffi::c_int = (*h).stat.frame.i_direct_score
                [1 as ::core::ffi::c_int as usize]
                - (*h).stat.frame.i_direct_score[0 as ::core::ffi::c_int as usize];
            let mut dir_avg: ::core::ffi::c_int = (*h).stat.i_direct_score
                [1 as ::core::ffi::c_int as usize]
                - (*h).stat.i_direct_score[0 as ::core::ffi::c_int as usize];
            let mut c_direct: ::core::ffi::c_char = (if (*h).mb.b_direct_auto_write != 0 {
                if dir_frame > 0 as ::core::ffi::c_int {
                    's' as i32
                } else if dir_frame < 0 as ::core::ffi::c_int {
                    't' as i32
                } else if dir_avg > 0 as ::core::ffi::c_int {
                    's' as i32
                } else if dir_avg < 0 as ::core::ffi::c_int {
                    't' as i32
                } else {
                    '-' as i32
                }
            } else {
                '-' as i32
            }) as ::core::ffi::c_char;
            if crate::stdlib::fprintf(
                (*rc).p_stat_file_out,
                b"in:%d out:%d type:%c dur:%ld cpbdur:%ld q:%.2f aq:%.2f tex:%d mv:%d misc:%d imb:%d pmb:%d smb:%d d:%c ref:\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*(*h).fenc).i_frame,
                (*h).i_frame,
                c_type as ::core::ffi::c_int,
                (*(*h).fenc).i_duration,
                (*(*h).fenc).i_cpb_duration,
                (*rc).qpa_rc as ::core::ffi::c_double,
                (*(*h).fdec).f_qp_avg_aq as ::core::ffi::c_double,
                (*h).stat.frame.i_tex_bits,
                (*h).stat.frame.i_mv_bits,
                (*h).stat.frame.i_misc_bits,
                (*h).stat.frame.i_mb_count_i,
                (*h).stat.frame.i_mb_count_p,
                (*h).stat.frame.i_mb_count_skip,
                c_direct as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                c2rust_current_block = 12447577463904507897;
            } else {
                let mut use_old_stats: ::core::ffi::c_int = ((*h).param.rc.b_stat_read
                    != 0 && (*(*rc).rce).refs > 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                loop {
                    if !(i_0
                        < (if use_old_stats != 0 {
                            (*(*rc).rce).refs
                        } else {
                            (*h).i_ref[0 as ::core::ffi::c_int as usize]
                        }))
                    {
                        c2rust_current_block = 8236137900636309791;
                        break;
                    }
                    let mut refcount: ::core::ffi::c_int = if use_old_stats != 0 {
                        (*(*rc).rce).refcount[i_0 as usize]
                    } else if (*h).param.b_interlaced != 0 {
                        (*h)
                            .stat
                            .frame
                            .i_mb_count_ref[0 as ::core::ffi::c_int
                            as usize][(i_0 * 2 as ::core::ffi::c_int) as usize]
                            + (*h)
                                .stat
                                .frame
                                .i_mb_count_ref[0 as ::core::ffi::c_int
                                as usize][(i_0 * 2 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as usize]
                    } else {
                        (*h)
                            .stat
                            .frame
                            .i_mb_count_ref[0 as ::core::ffi::c_int
                            as usize][i_0 as usize]
                    };
                    if crate::stdlib::fprintf(
                        (*rc).p_stat_file_out,
                        b"%d \0".as_ptr() as *const ::core::ffi::c_char,
                        refcount,
                    ) < 0 as ::core::ffi::c_int
                    {
                        c2rust_current_block = 12447577463904507897;
                        break;
                    }
                    i_0 += 1;
                }
                match c2rust_current_block {
                    12447577463904507897 => {}
                    _ => {
                        if (*h).param.analyse.i_weighted_pred >= crate::x264_h::X264_WEIGHTP_SIMPLE
                            && !(*h)
                                .sh
                                .weight[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int as usize]
                                .weightfn
                                .is_null()
                        {
                            if crate::stdlib::fprintf(
                                (*rc).p_stat_file_out,
                                b"w:%d,%d,%d\0".as_ptr() as *const ::core::ffi::c_char,
                                (*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int as usize]
                                    .i_denom,
                                (*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int as usize]
                                    .i_scale,
                                (*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int as usize]
                                    .i_offset,
                            ) < 0 as ::core::ffi::c_int
                            {
                                c2rust_current_block = 12447577463904507897;
                            } else if !(*h)
                                .sh
                                .weight[0 as ::core::ffi::c_int
                                    as usize][1 as ::core::ffi::c_int as usize]
                                .weightfn
                                .is_null()
                                || !(*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][2 as ::core::ffi::c_int as usize]
                                    .weightfn
                                    .is_null()
                            {
                                if crate::stdlib::fprintf(
                                    (*rc).p_stat_file_out,
                                    b",%d,%d,%d,%d,%d \0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*h)
                                        .sh
                                        .weight[0 as ::core::ffi::c_int
                                            as usize][1 as ::core::ffi::c_int as usize]
                                        .i_denom,
                                    (*h)
                                        .sh
                                        .weight[0 as ::core::ffi::c_int
                                            as usize][1 as ::core::ffi::c_int as usize]
                                        .i_scale,
                                    (*h)
                                        .sh
                                        .weight[0 as ::core::ffi::c_int
                                            as usize][1 as ::core::ffi::c_int as usize]
                                        .i_offset,
                                    (*h)
                                        .sh
                                        .weight[0 as ::core::ffi::c_int
                                            as usize][2 as ::core::ffi::c_int as usize]
                                        .i_scale,
                                    (*h)
                                        .sh
                                        .weight[0 as ::core::ffi::c_int
                                            as usize][2 as ::core::ffi::c_int as usize]
                                        .i_offset,
                                ) < 0 as ::core::ffi::c_int
                                {
                                    c2rust_current_block = 12447577463904507897;
                                } else {
                                    c2rust_current_block = 12147880666119273379;
                                }
                            } else if crate::stdlib::fprintf(
                                (*rc).p_stat_file_out,
                                b" \0".as_ptr() as *const ::core::ffi::c_char,
                            ) < 0 as ::core::ffi::c_int
                            {
                                c2rust_current_block = 12447577463904507897;
                            } else {
                                c2rust_current_block = 12147880666119273379;
                            }
                        } else {
                            c2rust_current_block = 12147880666119273379;
                        }
                        match c2rust_current_block {
                            12447577463904507897 => {}
                            _ => {
                                if crate::stdlib::fprintf(
                                    (*rc).p_stat_file_out,
                                    b";\n\0".as_ptr() as *const ::core::ffi::c_char,
                                ) < 0 as ::core::ffi::c_int
                                {
                                    c2rust_current_block = 12447577463904507897;
                                } else if (*h).param.rc.b_mb_tree != 0
                                    && (*(*h).fenc).b_kept_as_ref != 0
                                    && (*h).param.rc.b_stat_read == 0
                                {
                                    let mut i_type: crate::stdlib::uint8_t = (*h).sh.i_type as crate::stdlib::uint8_t;
                                    (*h)
                                        .mc
                                        .mbtree_fix8_pack
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        (*rc).mbtree.qp_buffer[0 as ::core::ffi::c_int as usize],
                                        (*(*h).fenc).f_qp_offset,
                                        (*h).mb.i_mb_count,
                                    );
                                    if crate::stdlib::fwrite(
                                        &raw mut i_type as *const ::core::ffi::c_void,
                                        1 as crate::__stddef_size_t_h::size_t,
                                        1 as crate::__stddef_size_t_h::size_t,
                                        (*rc).p_mbtree_stat_file_out,
                                    ) < 1 as ::core::ffi::c_ulong
                                    {
                                        c2rust_current_block = 12447577463904507897;
                                    } else if crate::stdlib::fwrite(
                                        (*rc).mbtree.qp_buffer[0 as ::core::ffi::c_int as usize]
                                            as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<crate::stdlib::uint16_t>() as crate::__stddef_size_t_h::size_t,
                                        (*h).mb.i_mb_count as crate::__stddef_size_t_h::size_t,
                                        (*rc).p_mbtree_stat_file_out,
                                    )
                                        < (*h).mb.i_mb_count as ::core::ffi::c_uint
                                            as ::core::ffi::c_ulong
                                    {
                                        c2rust_current_block = 12447577463904507897;
                                    } else {
                                        c2rust_current_block = 15125582407903384992;
                                    }
                                } else {
                                    c2rust_current_block = 15125582407903384992;
                                }
                            }
                        }
                    }
                }
            }
            match c2rust_current_block {
                15125582407903384992 => {}
                _ => {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_ERROR_1,
                        b"ratecontrol_end: stats file could not be written to\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
            }
        }
        if (*rc).b_abr != 0 {
            if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                (*rc).cplxr_sum += (bits as ::core::ffi::c_float * qp2qscale((*rc).qpa_rc))
                    as ::core::ffi::c_double
                    / (*rc).last_rceq;
            } else {
                (*rc).cplxr_sum += (bits as ::core::ffi::c_float * qp2qscale((*rc).qpa_rc))
                    as ::core::ffi::c_double
                    / ((*rc).last_rceq * (*h).param.rc.f_pb_factor as ::core::ffi::c_double);
            }
            (*rc).cplxr_sum *= (*rc).cbr_decay;
            (*rc).wanted_bits_window +=
                (*(*h).fenc).f_duration as ::core::ffi::c_double * (*rc).bitrate;
            (*rc).wanted_bits_window *= (*rc).cbr_decay;
        }
        if (*rc).b_2pass != 0 {
            (*rc).expected_bits_sum += qscale2bits(
                (*rc).rce,
                qp2qscale((*(*rc).rce).new_qp) as ::core::ffi::c_double,
            );
        }
        if (*h).mb.b_variable_qp != 0 {
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                (*rc).bframe_bits += bits;
                if (*(*h).fenc).b_last_minigop_bframe != 0 {
                    update_predictor(
                        (*rc).pred_b_from_p,
                        qp2qscale((*rc).qpa_rc),
                        (*(*h).fref[1 as ::core::ffi::c_int as usize][((*h).i_ref
                            [1 as ::core::ffi::c_int as usize]
                            - 1 as ::core::ffi::c_int)
                            as usize])
                            .i_satd as ::core::ffi::c_float,
                        ((*rc).bframe_bits / (*rc).bframes) as ::core::ffi::c_float,
                    );
                    (*rc).bframe_bits = 0 as ::core::ffi::c_int;
                }
            }
        }
        *filler = update_vbv(h, bits);
        (*rc).filler_bits_sum += (*filler * 8 as ::core::ffi::c_int) as crate::stdlib::int64_t;
        if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
            .vui
            .b_nal_hrd_parameters_present
            != 0
        {
            if (*(*h).fenc).i_frame == 0 as ::core::ffi::c_int {
                (*(*h).fenc).hrd_timing.cpb_initial_arrival_time =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
                (*rc).initial_cpb_removal_delay = (*h).initial_cpb_removal_delay;
                (*rc).initial_cpb_removal_delay_offset = (*h).initial_cpb_removal_delay_offset;
                (*rc).nrt_first_access_unit = (*rc).initial_cpb_removal_delay
                    as ::core::ffi::c_double
                    / 90000 as ::core::ffi::c_int as ::core::ffi::c_double;
                (*(*h).fenc).hrd_timing.cpb_removal_time = (*rc).nrt_first_access_unit;
            } else {
                (*(*h).fenc).hrd_timing.cpb_removal_time = (*rc).nrt_first_access_unit
                    + ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset)
                        as ::core::ffi::c_double
                        * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_num_units_in_tick as ::core::ffi::c_double
                        / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .i_time_scale as ::core::ffi::c_double;
                if (*(*h).fenc).b_keyframe != 0 {
                    (*rc).nrt_first_access_unit = (*(*h).fenc).hrd_timing.cpb_removal_time;
                    (*rc).initial_cpb_removal_delay = (*h).initial_cpb_removal_delay;
                    (*rc).initial_cpb_removal_delay_offset = (*h).initial_cpb_removal_delay_offset;
                }
                let mut cpb_earliest_arrival_time: ::core::ffi::c_double =
                    (*(*h).fenc).hrd_timing.cpb_removal_time
                        - (*rc).initial_cpb_removal_delay as ::core::ffi::c_double
                            / 90000 as ::core::ffi::c_int as ::core::ffi::c_double;
                if (*(*h).fenc).b_keyframe == 0 {
                    cpb_earliest_arrival_time -= (*rc).initial_cpb_removal_delay_offset
                        as ::core::ffi::c_double
                        / 90000 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .hrd
                    .b_cbr_hrd
                    != 0
                {
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
            let mut filler_bits: ::core::ffi::c_int = if *filler != 0 {
                (if 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int - (*h).param.b_annexb
                    > *filler
                {
                    5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int - (*h).param.b_annexb
                } else {
                    *filler
                }) * 8 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            (*rc).previous_cpb_final_arrival_time =
                (*(*h).fenc).hrd_timing.cpb_initial_arrival_time
                    + (bits + filler_bits) as ::core::ffi::c_double
                        / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .vui
                            .hrd
                            .i_bit_rate_unscaled as ::core::ffi::c_double;
            (*(*h).fenc).hrd_timing.cpb_final_arrival_time = (*rc).previous_cpb_final_arrival_time;
            (*(*h).fenc).hrd_timing.dpb_output_time = (*(*h).fenc).i_dpb_output_delay
                as ::core::ffi::c_double
                * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_num_units_in_tick as ::core::ffi::c_double
                / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as ::core::ffi::c_double
                + (*(*h).fenc).hrd_timing.cpb_removal_time;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn get_qscale(
    mut h: *mut crate::src::common::common::x264_t,
    mut rce: *mut ratecontrol_entry_t,
    mut rate_factor: ::core::ffi::c_double,
    mut frame_num: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let mut zone: *mut crate::x264_h::x264_zone_t = get_zone(h, frame_num);
        let mut q: ::core::ffi::c_double = 0.;
        if (*h).param.rc.b_mb_tree != 0 {
            let mut timescale: ::core::ffi::c_double =
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_num_units_in_tick as ::core::ffi::c_double
                    / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as ::core::ffi::c_double;
            q = crate::stdlib::pow(
                (0.04f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_float)
                    as ::core::ffi::c_double
                    / x264_clip3f(
                        (*rce).i_duration as ::core::ffi::c_double * timescale,
                        (0.01f32
                            / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        (1.00f32
                            / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                    ),
                (1 as ::core::ffi::c_int as ::core::ffi::c_float - (*h).param.rc.f_qcompress)
                    as ::core::ffi::c_double,
            );
        } else {
            q = crate::stdlib::pow(
                (*rce).blurred_complexity as ::core::ffi::c_double,
                1 as ::core::ffi::c_int as ::core::ffi::c_double - (*rcc).qcompress,
            );
        }
        if q.is_finite() as i32 == 0 || (*rce).tex_bits + (*rce).mv_bits == 0 as ::core::ffi::c_int
        {
            q = (*rcc).last_qscale_for[(*rce).pict_type as usize];
        } else {
            (*rcc).last_rceq = q;
            q /= rate_factor;
            (*rcc).last_qscale = q;
        }
        if !zone.is_null() {
            if (*zone).b_force_qp != 0 {
                q = qp2qscale((*zone).i_qp as ::core::ffi::c_float) as ::core::ffi::c_double;
            } else {
                q /= (*zone).f_bitrate_factor as ::core::ffi::c_double;
            }
        }
        return q;
    }
}
unsafe extern "C" fn get_diff_limited_q(
    mut h: *mut crate::src::common::common::x264_t,
    mut rce: *mut ratecontrol_entry_t,
    mut q: ::core::ffi::c_double,
    mut frame_num: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let pict_type: ::core::ffi::c_int = (*rce).pict_type;
        let mut zone: *mut crate::x264_h::x264_zone_t = get_zone(h, frame_num);
        if pict_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            let mut iq: ::core::ffi::c_double = q;
            let mut pq: ::core::ffi::c_double =
                qp2qscale(((*rcc).accum_p_qp / (*rcc).accum_p_norm) as ::core::ffi::c_float)
                    as ::core::ffi::c_double;
            let mut ip_factor: ::core::ffi::c_double =
                (*h).param.rc.f_ip_factor as ::core::ffi::c_double;
            if (*rcc).accum_p_norm <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                q = iq;
            } else if (*rcc).accum_p_norm >= 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                q = pq / ip_factor;
            } else {
                q = (*rcc).accum_p_norm * pq / ip_factor
                    + (1 as ::core::ffi::c_int as ::core::ffi::c_double - (*rcc).accum_p_norm) * iq;
            }
        } else if pict_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            q = (*rcc).last_qscale_for[(*rcc).last_non_b_pict_type as usize];
            if (*rce).kept_as_ref == 0 {
                q *= (*h).param.rc.f_pb_factor as ::core::ffi::c_double;
            }
        } else if pict_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            && (*rcc).last_non_b_pict_type
                == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            && (*rce).tex_bits == 0 as ::core::ffi::c_int
        {
            q = (*rcc).last_qscale_for
                [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize];
        }
        if (*rcc).last_non_b_pict_type == pict_type
            && (pict_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                || (*rcc).last_accum_p_norm < 1 as ::core::ffi::c_int as ::core::ffi::c_double)
        {
            let mut last_q: ::core::ffi::c_double = (*rcc).last_qscale_for[pict_type as usize];
            let mut max_qscale: ::core::ffi::c_double = last_q * (*rcc).lstep;
            let mut min_qscale: ::core::ffi::c_double = last_q / (*rcc).lstep;
            if q > max_qscale {
                q = max_qscale;
            } else if q < min_qscale {
                q = min_qscale;
            }
        }
        (*rcc).last_qscale_for[pict_type as usize] = q;
        if pict_type != crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            (*rcc).last_non_b_pict_type = pict_type;
        }
        if pict_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            (*rcc).last_accum_p_norm = (*rcc).accum_p_norm;
            (*rcc).accum_p_norm = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*rcc).accum_p_qp = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if pict_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            let mut mask: ::core::ffi::c_float = (1 as ::core::ffi::c_int as ::core::ffi::c_double
                - crate::stdlib::pow(
                    ((*rce).i_count as ::core::ffi::c_float / (*rcc).nmb as ::core::ffi::c_float)
                        as ::core::ffi::c_double,
                    2 as ::core::ffi::c_int as ::core::ffi::c_double,
                )) as ::core::ffi::c_float;
            (*rcc).accum_p_qp = mask as ::core::ffi::c_double
                * (qscale2qp(q as ::core::ffi::c_float) as ::core::ffi::c_double
                    + (*rcc).accum_p_qp);
            (*rcc).accum_p_norm = mask as ::core::ffi::c_double
                * (1 as ::core::ffi::c_int as ::core::ffi::c_double + (*rcc).accum_p_norm);
        }
        if !zone.is_null() {
            if (*zone).b_force_qp != 0 {
                q = qp2qscale((*zone).i_qp as ::core::ffi::c_float) as ::core::ffi::c_double;
            } else {
                q /= (*zone).f_bitrate_factor as ::core::ffi::c_double;
            }
        }
        return q;
    }
}
unsafe extern "C" fn predict_size(
    mut p: *mut predictor_t,
    mut q: ::core::ffi::c_float,
    mut var: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    unsafe {
        return ((*p).coeff * var + (*p).offset) / (q * (*p).count);
    }
}
unsafe extern "C" fn update_predictor(
    mut p: *mut predictor_t,
    mut q: ::core::ffi::c_float,
    mut var: ::core::ffi::c_float,
    mut bits: ::core::ffi::c_float,
) {
    unsafe {
        let mut range: ::core::ffi::c_float = 1.5f32;
        if var < 10 as ::core::ffi::c_int as ::core::ffi::c_float {
            return;
        }
        let mut old_coeff: ::core::ffi::c_float = (*p).coeff / (*p).count;
        let mut old_offset: ::core::ffi::c_float = (*p).offset / (*p).count;
        let mut new_coeff: ::core::ffi::c_float = if (bits * q - old_offset) / var > (*p).coeff_min
        {
            (bits * q - old_offset) / var
        } else {
            (*p).coeff_min
        };
        let mut new_coeff_clipped: ::core::ffi::c_float = x264_clip3f(
            new_coeff as ::core::ffi::c_double,
            (old_coeff / range) as ::core::ffi::c_double,
            (old_coeff * range) as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        let mut new_offset: ::core::ffi::c_float = bits * q - new_coeff_clipped * var;
        if new_offset >= 0 as ::core::ffi::c_int as ::core::ffi::c_float {
            new_coeff = new_coeff_clipped;
        } else {
            new_offset = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        }
        (*p).count *= (*p).decay;
        (*p).coeff *= (*p).decay;
        (*p).offset *= (*p).decay;
        (*p).count += 1.;
        (*p).coeff += new_coeff;
        (*p).offset += new_offset;
    }
}
unsafe extern "C" fn update_vbv(
    mut h: *mut crate::src::common::common::x264_t,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut filler: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bitrate: ::core::ffi::c_int = (*(&raw mut (*h).sps
            as *mut crate::src::common::set::x264_sps_t))
            .vui
            .hrd
            .i_bit_rate_unscaled;
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let mut rct: *mut x264_ratecontrol_t = (*(*h).thread[0 as ::core::ffi::c_int as usize]).rc;
        let mut buffer_size: crate::stdlib::int64_t =
            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .hrd
                .i_cpb_size_unscaled as crate::stdlib::int64_t
                * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as crate::stdlib::int64_t;
        if (*rcc).last_satd >= (*h).mb.i_mb_count {
            update_predictor(
                (*rct).pred.offset((*h).sh.i_type as isize) as *mut predictor_t,
                qp2qscale((*rcc).qpa_rc),
                (*rcc).last_satd as ::core::ffi::c_float,
                bits as ::core::ffi::c_float,
            );
        }
        if (*rcc).b_vbv == 0 {
            return filler;
        }
        let mut buffer_diff: crate::stdlib::uint64_t = (bits as crate::stdlib::uint64_t)
            .wrapping_mul(
                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as crate::stdlib::uint64_t,
            );
        (*rct).buffer_fill_final = ((*rct).buffer_fill_final as crate::stdlib::uint64_t)
            .wrapping_sub(buffer_diff) as crate::stdlib::int64_t
            as crate::stdlib::int64_t;
        (*rct).buffer_fill_final_min =
            ((*rct).buffer_fill_final_min as crate::stdlib::uint64_t).wrapping_sub(buffer_diff)
                as crate::stdlib::int64_t as crate::stdlib::int64_t;
        if (*rct).buffer_fill_final_min < 0 as crate::stdlib::int64_t {
            let mut underflow: ::core::ffi::c_double = (*rct).buffer_fill_final_min
                as ::core::ffi::c_double
                / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as ::core::ffi::c_double;
            if (*rcc).rate_factor_max_increment != 0.
                && (*rcc).qpm >= (*rcc).qp_novbv + (*rcc).rate_factor_max_increment
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_DEBUG_1,
                    b"VBV underflow due to CRF-max (frame %d, %.0f bits)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).i_frame,
                    underflow,
                );
            } else {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"VBV underflow (frame %d, %.0f bits)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).i_frame,
                    underflow,
                );
            }
            (*rct).buffer_fill_final_min = 0 as crate::stdlib::int64_t;
            (*rct).buffer_fill_final = (*rct).buffer_fill_final_min;
        }
        if (*h).param.i_avcintra_class != 0 {
            buffer_diff = buffer_size as crate::stdlib::uint64_t;
        } else {
            buffer_diff = (bitrate as crate::stdlib::uint64_t)
                .wrapping_mul(
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_num_units_in_tick as crate::stdlib::uint64_t,
                )
                .wrapping_mul((*(*h).fenc).i_cpb_duration as crate::stdlib::uint64_t);
        }
        (*rct).buffer_fill_final = ((*rct).buffer_fill_final as crate::stdlib::uint64_t)
            .wrapping_add(buffer_diff) as crate::stdlib::int64_t
            as crate::stdlib::int64_t;
        (*rct).buffer_fill_final_min =
            ((*rct).buffer_fill_final_min as crate::stdlib::uint64_t).wrapping_add(buffer_diff)
                as crate::stdlib::int64_t as crate::stdlib::int64_t;
        if (*rct).buffer_fill_final > buffer_size {
            if (*h).param.rc.b_filler != 0 {
                let mut scale: crate::stdlib::int64_t =
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as crate::stdlib::int64_t
                        * 8 as crate::stdlib::int64_t;
                filler = (((*rct).buffer_fill_final - buffer_size + scale
                    - 1 as crate::stdlib::int64_t)
                    / scale) as ::core::ffi::c_int;
                bits = if (*h).param.i_avcintra_class != 0 {
                    filler * 8 as ::core::ffi::c_int
                } else {
                    (if 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int - (*h).param.b_annexb
                        > filler
                    {
                        5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int - (*h).param.b_annexb
                    } else {
                        filler
                    }) * 8 as ::core::ffi::c_int
                };
                buffer_diff = (bits as crate::stdlib::uint64_t).wrapping_mul(
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as crate::stdlib::uint64_t,
                );
                (*rct).buffer_fill_final =
                    ((*rct).buffer_fill_final as crate::stdlib::uint64_t).wrapping_sub(buffer_diff)
                        as crate::stdlib::int64_t as crate::stdlib::int64_t;
                (*rct).buffer_fill_final_min =
                    ((*rct).buffer_fill_final_min as crate::stdlib::uint64_t)
                        .wrapping_sub(buffer_diff) as crate::stdlib::int64_t
                        as crate::stdlib::int64_t;
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
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_hrd_fullness(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut rct: *mut x264_ratecontrol_t = (*(*h).thread[0 as ::core::ffi::c_int as usize]).rc;
        let mut denom: crate::stdlib::uint64_t =
            ((*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .hrd
                .i_bit_rate_unscaled as crate::stdlib::uint64_t)
                .wrapping_mul(
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as crate::stdlib::uint64_t,
                )
                .wrapping_div((*rct).hrd_multiply_denom);
        let mut cpb_state: crate::stdlib::uint64_t =
            (*rct).buffer_fill_final as crate::stdlib::uint64_t;
        let mut cpb_size: crate::stdlib::uint64_t =
            ((*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .hrd
                .i_cpb_size_unscaled as crate::stdlib::uint64_t)
                .wrapping_mul(
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as crate::stdlib::uint64_t,
                );
        let mut multiply_factor: crate::stdlib::uint64_t =
            (90000 as crate::stdlib::uint64_t).wrapping_div((*rct).hrd_multiply_denom);
        if (*rct).buffer_fill_final < 0 as crate::stdlib::int64_t
            || (*rct).buffer_fill_final > cpb_size as crate::stdlib::int64_t
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"CPB %s: %.0f bits in a %.0f-bit buffer\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if (*rct).buffer_fill_final < 0 as crate::stdlib::int64_t {
                    b"underflow\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"overflow\0".as_ptr() as *const ::core::ffi::c_char
                },
                (*rct).buffer_fill_final as ::core::ffi::c_double
                    / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as ::core::ffi::c_double,
                cpb_size as ::core::ffi::c_double
                    / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_time_scale as ::core::ffi::c_double,
            );
        }
        (*h).initial_cpb_removal_delay =
            multiply_factor.wrapping_mul(cpb_state).wrapping_div(denom) as ::core::ffi::c_int;
        (*h).initial_cpb_removal_delay_offset = multiply_factor
            .wrapping_mul(cpb_size)
            .wrapping_div(denom)
            .wrapping_sub((*h).initial_cpb_removal_delay as crate::stdlib::uint64_t)
            as ::core::ffi::c_int;
        let mut decoder_buffer_fill: crate::stdlib::int64_t =
            ((*h).initial_cpb_removal_delay as crate::stdlib::uint64_t)
                .wrapping_mul(denom)
                .wrapping_div(multiply_factor) as crate::stdlib::int64_t;
        (*rct).buffer_fill_final_min = if (*rct).buffer_fill_final_min < decoder_buffer_fill {
            (*rct).buffer_fill_final_min
        } else {
            decoder_buffer_fill
        };
    }
}
unsafe extern "C" fn update_vbv_plan(
    mut h: *mut crate::src::common::common::x264_t,
    mut overhead: ::core::ffi::c_int,
) {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        (*rcc).buffer_fill =
            ((*(*(*h).thread[0 as ::core::ffi::c_int as usize]).rc).buffer_fill_final_min
                / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as crate::stdlib::int64_t) as ::core::ffi::c_double;
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            let mut j: ::core::ffi::c_int =
                rcc.offset_from((*(*h).thread[0 as ::core::ffi::c_int as usize]).rc)
                    as ::core::ffi::c_long as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i < (*h).i_thread_frames {
                let mut t: *mut crate::src::common::common::x264_t =
                    (*h).thread[((j + i) % (*h).i_thread_frames) as usize];
                let mut bits: ::core::ffi::c_double = (*(*t).rc).frame_size_planned;
                if !((*t).b_thread_active == 0) {
                    bits = if bits > (*(*t).rc).frame_size_estimated as ::core::ffi::c_double {
                        bits
                    } else {
                        (*(*t).rc).frame_size_estimated as ::core::ffi::c_double
                    };
                    (*rcc).buffer_fill -= bits;
                    (*rcc).buffer_fill =
                        if (*rcc).buffer_fill > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            (*rcc).buffer_fill
                        } else {
                            0 as ::core::ffi::c_int as ::core::ffi::c_double
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
        (*rcc).buffer_fill -= overhead as ::core::ffi::c_double;
    }
}
unsafe extern "C" fn clip_qscale(
    mut h: *mut crate::src::common::common::x264_t,
    mut pict_type: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let mut lmin: ::core::ffi::c_double = (*rcc).lmin[pict_type as usize];
        let mut lmax: ::core::ffi::c_double = (*rcc).lmax[pict_type as usize];
        if (*rcc).rate_factor_max_increment != 0. {
            lmax = if lmax
                < qp2qscale((*rcc).qp_novbv + (*rcc).rate_factor_max_increment)
                    as ::core::ffi::c_double
            {
                lmax
            } else {
                qp2qscale((*rcc).qp_novbv + (*rcc).rate_factor_max_increment)
                    as ::core::ffi::c_double
            };
        }
        if lmin == lmax {
            return lmin;
        } else if (*rcc).b_2pass != 0 {
            let mut min2: ::core::ffi::c_double = crate::stdlib::log(lmin);
            let mut max2: ::core::ffi::c_double = crate::stdlib::log(lmax);
            q = (crate::stdlib::log(q) - min2) / (max2 - min2) - 0.5f64;
            q = 1.0f64
                / (1.0f64
                    + crate::stdlib::exp(-(4 as ::core::ffi::c_int) as ::core::ffi::c_double * q));
            q = q * (max2 - min2) + min2;
            return crate::stdlib::exp(q);
        } else {
            return x264_clip3f(q, lmin, lmax);
        };
    }
}
unsafe extern "C" fn vbv_pass1(
    mut h: *mut crate::src::common::common::x264_t,
    mut pict_type: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        if (*rcc).b_vbv != 0 && (*rcc).last_satd > 0 as ::core::ffi::c_int {
            let mut q0: ::core::ffi::c_double = q;
            let mut fenc_cpb_duration: ::core::ffi::c_double = (*(*h).fenc).i_cpb_duration
                as ::core::ffi::c_double
                * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_num_units_in_tick as ::core::ffi::c_double
                / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as ::core::ffi::c_double;
            if (*h).param.rc.i_lookahead != 0 {
                let mut terminate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iterations: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while iterations < 1000 as ::core::ffi::c_int
                    && terminate != 3 as ::core::ffi::c_int
                {
                    let mut frame_q: [::core::ffi::c_double; 3] = [0.; 3];
                    let mut cur_bits: ::core::ffi::c_double = predict_size(
                        (*rcc).pred.offset((*h).sh.i_type as isize) as *mut predictor_t,
                        q as ::core::ffi::c_float,
                        (*rcc).last_satd as ::core::ffi::c_float,
                    )
                        as ::core::ffi::c_double;
                    let mut buffer_fill_cur: ::core::ffi::c_double = (*rcc).buffer_fill - cur_bits;
                    let mut total_duration: ::core::ffi::c_double =
                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    let mut last_duration: ::core::ffi::c_double = fenc_cpb_duration;
                    frame_q[0 as ::core::ffi::c_int as usize] = if (*h).sh.i_type
                        == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                    {
                        q * (*h).param.rc.f_ip_factor as ::core::ffi::c_double
                    } else {
                        q
                    };
                    frame_q[1 as ::core::ffi::c_int as usize] = frame_q
                        [0 as ::core::ffi::c_int as usize]
                        * (*h).param.rc.f_pb_factor as ::core::ffi::c_double;
                    frame_q[2 as ::core::ffi::c_int as usize] = frame_q
                        [0 as ::core::ffi::c_int as usize]
                        / (*h).param.rc.f_ip_factor as ::core::ffi::c_double;
                    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while buffer_fill_cur >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && buffer_fill_cur <= (*rcc).buffer_size
                    {
                        total_duration += last_duration;
                        buffer_fill_cur += (*rcc).vbv_max_rate * last_duration;
                        let mut i_type: ::core::ffi::c_int =
                            (*(*h).fenc).i_planned_type[j as usize] as ::core::ffi::c_int;
                        let mut i_satd: ::core::ffi::c_int =
                            (*(*h).fenc).i_planned_satd[j as usize];
                        if i_type == crate::x264_h::X264_TYPE_AUTO {
                            break;
                        }
                        i_type = if i_type == crate::x264_h::X264_TYPE_I
                            || i_type == crate::x264_h::X264_TYPE_IDR
                            || i_type == crate::x264_h::X264_TYPE_KEYFRAME
                        {
                            crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                        } else if i_type == crate::x264_h::X264_TYPE_B
                            || i_type == crate::x264_h::X264_TYPE_BREF
                        {
                            crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
                        } else {
                            crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                        };
                        cur_bits = predict_size(
                            (*rcc).pred.offset(i_type as isize) as *mut predictor_t,
                            frame_q[i_type as usize] as ::core::ffi::c_float,
                            i_satd as ::core::ffi::c_float,
                        ) as ::core::ffi::c_double;
                        buffer_fill_cur -= cur_bits;
                        last_duration = (*(*h).fenc).f_planned_cpb_duration[j as usize];
                        j += 1;
                    }
                    let mut target_fill: ::core::ffi::c_double = if (*rcc).buffer_fill
                        + total_duration * (*rcc).vbv_max_rate * 0.5f64
                        < (*rcc).buffer_size * 0.5f64
                    {
                        (*rcc).buffer_fill + total_duration * (*rcc).vbv_max_rate * 0.5f64
                    } else {
                        (*rcc).buffer_size * 0.5f64
                    };
                    if buffer_fill_cur < target_fill {
                        q *= 1.01f64;
                        terminate |= 1 as ::core::ffi::c_int;
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
                        terminate |= 2 as ::core::ffi::c_int;
                    }
                    iterations += 1;
                }
            } else {
                if (pict_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                    || pict_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                        && (*rcc).last_non_b_pict_type
                            == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int)
                    && (*rcc).buffer_fill / (*rcc).buffer_size < 0.5f64
                {
                    q /= x264_clip3f(
                        2.0f64 * (*rcc).buffer_fill / (*rcc).buffer_size,
                        0.5f64,
                        1.0f64,
                    );
                }
                let mut bits: ::core::ffi::c_double = predict_size(
                    (*rcc).pred.offset((*h).sh.i_type as isize) as *mut predictor_t,
                    q as ::core::ffi::c_float,
                    (*rcc).last_satd as ::core::ffi::c_float,
                ) as ::core::ffi::c_double;
                let mut max_fill_factor: ::core::ffi::c_double =
                    (if (*h).param.rc.i_vbv_buffer_size as ::core::ffi::c_double
                        >= (5 as ::core::ffi::c_int * (*h).param.rc.i_vbv_max_bitrate)
                            as ::core::ffi::c_double
                            / (*rcc).fps
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as ::core::ffi::c_double;
                let mut min_fill_factor: ::core::ffi::c_double = (if (*rcc).single_frame_vbv != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                })
                    as ::core::ffi::c_double;
                if bits > (*rcc).buffer_fill / max_fill_factor {
                    let mut qf: ::core::ffi::c_double = x264_clip3f(
                        (*rcc).buffer_fill / (max_fill_factor * bits),
                        0.2f64,
                        1.0f64,
                    );
                    q /= qf;
                    bits *= qf;
                }
                if bits < (*rcc).buffer_rate / min_fill_factor {
                    let mut qf_0: ::core::ffi::c_double = x264_clip3f(
                        bits * min_fill_factor / (*rcc).buffer_rate,
                        0.001f64,
                        1.0f64,
                    );
                    q *= qf_0;
                }
                q = if q0 > q { q0 } else { q };
            }
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                && (*rcc).single_frame_vbv == 0
            {
                let mut nb: ::core::ffi::c_int = (*rcc).bframes;
                let mut bits_0: ::core::ffi::c_double = predict_size(
                    (*rcc).pred.offset((*h).sh.i_type as isize) as *mut predictor_t,
                    q as ::core::ffi::c_float,
                    (*rcc).last_satd as ::core::ffi::c_float,
                ) as ::core::ffi::c_double;
                let mut pbbits: ::core::ffi::c_double = bits_0;
                let mut bbits: ::core::ffi::c_double = predict_size(
                    (*rcc).pred_b_from_p,
                    (q * (*h).param.rc.f_pb_factor as ::core::ffi::c_double)
                        as ::core::ffi::c_float,
                    (*rcc).last_satd as ::core::ffi::c_float,
                ) as ::core::ffi::c_double;
                let mut bframe_cpb_duration: ::core::ffi::c_double =
                    0 as ::core::ffi::c_int as ::core::ffi::c_double;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < nb {
                    bframe_cpb_duration += (*(*h).fenc).f_planned_cpb_duration[i as usize];
                    i += 1;
                }
                if bbits * nb as ::core::ffi::c_double > bframe_cpb_duration * (*rcc).vbv_max_rate {
                    nb = 0 as ::core::ffi::c_int;
                    bframe_cpb_duration = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                pbbits += nb as ::core::ffi::c_double * bbits;
                let mut minigop_cpb_duration: ::core::ffi::c_double =
                    bframe_cpb_duration + fenc_cpb_duration;
                let mut space: ::core::ffi::c_double = (*rcc).buffer_fill
                    + minigop_cpb_duration * (*rcc).vbv_max_rate
                    - (*rcc).buffer_size;
                if pbbits < space {
                    q *= if pbbits / space > bits_0 / (0.5f64 * (*rcc).buffer_size) {
                        pbbits / space
                    } else {
                        bits_0 / (0.5f64 * (*rcc).buffer_size)
                    };
                }
                q = if q0 / 2 as ::core::ffi::c_int as ::core::ffi::c_double > q {
                    q0 / 2 as ::core::ffi::c_int as ::core::ffi::c_double
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
}
unsafe extern "C" fn rate_estimate_qscale(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_float {
    unsafe {
        let mut q: ::core::ffi::c_float = 0.;
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let mut rce: ratecontrol_entry_t = ratecontrol_entry_t {
            pict_type: 0 as ::core::ffi::c_int,
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
        let mut pict_type: ::core::ffi::c_int = (*h).sh.i_type;
        let mut total_bits: crate::stdlib::int64_t = 8 as crate::stdlib::int64_t
            * ((*h).stat.i_frame_size
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_size
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_size
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize])
            - (*rcc).filler_bits_sum;
        if (*rcc).b_2pass != 0 {
            rce = *(*rcc).rce;
            if pict_type != rce.pict_type {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"slice=%c but 2pass stats say %c\n\0".as_ptr() as *const ::core::ffi::c_char,
                    slice_type_to_char[pict_type as usize] as ::core::ffi::c_int,
                    slice_type_to_char[rce.pict_type as usize] as ::core::ffi::c_int,
                );
            }
        }
        if pict_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            let mut i0: ::core::ffi::c_int =
                ((*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_type
                    == crate::x264_h::X264_TYPE_I
                    || (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_type
                        == crate::x264_h::X264_TYPE_IDR
                    || (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_type
                        == crate::x264_h::X264_TYPE_KEYFRAME) as ::core::ffi::c_int;
            let mut i1: ::core::ffi::c_int =
                ((*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_type
                    == crate::x264_h::X264_TYPE_I
                    || (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_type
                        == crate::x264_h::X264_TYPE_IDR
                    || (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_type
                        == crate::x264_h::X264_TYPE_KEYFRAME) as ::core::ffi::c_int;
            let mut dt0: ::core::ffi::c_int = crate::stdlib::abs(
                (*(*h).fenc).i_poc - (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_poc,
            );
            let mut dt1: ::core::ffi::c_int = crate::stdlib::abs(
                (*(*h).fenc).i_poc - (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_poc,
            );
            let mut q0: ::core::ffi::c_float =
                (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).f_qp_avg_rc;
            let mut q1: ::core::ffi::c_float =
                (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).f_qp_avg_rc;
            if (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_type
                == crate::x264_h::X264_TYPE_BREF
            {
                q0 = (q0 as ::core::ffi::c_double
                    - (*rcc).pb_offset / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_float;
            }
            if (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_type
                == crate::x264_h::X264_TYPE_BREF
            {
                q1 = (q1 as ::core::ffi::c_double
                    - (*rcc).pb_offset / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_float;
            }
            if i0 != 0 && i1 != 0 {
                q = (((q0 + q1) / 2 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double
                    + (*rcc).ip_offset) as ::core::ffi::c_float;
            } else if i0 != 0 {
                q = q1;
            } else if i1 != 0 {
                q = q0;
            } else {
                q = (q0 * dt1 as ::core::ffi::c_float + q1 * dt0 as ::core::ffi::c_float)
                    / (dt0 + dt1) as ::core::ffi::c_float;
            }
            if (*(*h).fenc).b_kept_as_ref != 0 {
                q = (q as ::core::ffi::c_double
                    + (*rcc).pb_offset / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_float;
            } else {
                q = (q as ::core::ffi::c_double + (*rcc).pb_offset) as ::core::ffi::c_float;
            }
            (*rcc).qp_novbv = q;
            q = qp2qscale(q);
            if (*rcc).b_2pass != 0 {
                (*rcc).frame_size_planned = qscale2bits(&raw mut rce, q as ::core::ffi::c_double);
            } else {
                (*rcc).frame_size_planned = predict_size(
                    (*rcc).pred_b_from_p,
                    q,
                    (*(*h).fref[1 as ::core::ffi::c_int as usize][((*h).i_ref
                        [1 as ::core::ffi::c_int as usize]
                        - 1 as ::core::ffi::c_int)
                        as usize])
                        .i_satd as ::core::ffi::c_float,
                ) as ::core::ffi::c_double;
            }
            if (*rcc).b_vbv != 0 {
                let mut frame_size_maximum: ::core::ffi::c_double = if (*rcc).frame_size_maximum
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
                    q = (q as ::core::ffi::c_double
                        * ((*rcc).frame_size_planned / frame_size_maximum))
                        as ::core::ffi::c_float;
                    (*rcc).frame_size_planned = frame_size_maximum;
                }
            }
            ::core::ptr::write_volatile(
                &mut (*rcc).frame_size_estimated as *mut ::core::ffi::c_float,
                (*rcc).frame_size_planned as ::core::ffi::c_float,
            );
            if (*rcc).b_vbv != 0 {
                (*rcc).last_satd =
                    crate::src::encoder::analyse::slicetype_c::x264_8_rc_analyse_slice(
                        h as *mut crate::src::common::common::x264_t,
                    );
            }
            return q;
        } else {
            let mut abr_buffer: ::core::ffi::c_double = 2 as ::core::ffi::c_int
                as ::core::ffi::c_double
                * (*rcc).rate_tolerance
                * (*rcc).bitrate;
            let mut predicted_bits: ::core::ffi::c_double = total_bits as ::core::ffi::c_double;
            if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
                let mut j: ::core::ffi::c_int =
                    rcc.offset_from((*(*h).thread[0 as ::core::ffi::c_int as usize]).rc)
                        as ::core::ffi::c_long as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while i < (*h).i_thread_frames {
                    let mut t: *mut crate::src::common::common::x264_t =
                        (*h).thread[((j + i) % (*h).i_thread_frames) as usize];
                    let mut bits: ::core::ffi::c_double = (*(*t).rc).frame_size_planned;
                    if !((*t).b_thread_active == 0) {
                        bits = if bits > (*(*t).rc).frame_size_estimated as ::core::ffi::c_double {
                            bits
                        } else {
                            (*(*t).rc).frame_size_estimated as ::core::ffi::c_double
                        };
                        predicted_bits += bits;
                    }
                    i += 1;
                }
            }
            if (*rcc).b_2pass != 0 {
                let mut lmin: ::core::ffi::c_double = (*rcc).lmin[pict_type as usize];
                let mut lmax: ::core::ffi::c_double = (*rcc).lmax[pict_type as usize];
                if (*rcc).num_entries > (*h).i_frame {
                    let mut final_bits: ::core::ffi::c_double = (**(*rcc)
                        .entry_out
                        .offset(((*rcc).num_entries - 1 as ::core::ffi::c_int) as isize))
                    .expected_bits;
                    let mut video_pos: ::core::ffi::c_double = rce.expected_bits / final_bits;
                    let mut scale_factor: ::core::ffi::c_double = crate::stdlib::sqrt(
                        (1 as ::core::ffi::c_int as ::core::ffi::c_double - video_pos)
                            * (*rcc).num_entries as ::core::ffi::c_double,
                    );
                    abr_buffer *= 0.5f64
                        * (if scale_factor > 0.5f64 {
                            scale_factor
                        } else {
                            0.5f64
                        });
                }
                let mut diff: ::core::ffi::c_double = predicted_bits - rce.expected_bits;
                q = rce.new_qscale as ::core::ffi::c_float;
                q = (q as ::core::ffi::c_double
                    / x264_clip3f(
                        (abr_buffer - diff) / abr_buffer,
                        0.5f64,
                        2 as ::core::ffi::c_int as ::core::ffi::c_double,
                    )) as ::core::ffi::c_float;
                if (*h).i_frame as ::core::ffi::c_double >= (*rcc).fps
                    && (*rcc).expected_bits_sum >= 1 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    let mut cur_time: ::core::ffi::c_double = (*h).i_frame as ::core::ffi::c_double
                        / (*rcc).num_entries as ::core::ffi::c_double;
                    let mut w: ::core::ffi::c_double = x264_clip3f(
                        cur_time * 100 as ::core::ffi::c_int as ::core::ffi::c_double,
                        0.0f64,
                        1.0f64,
                    );
                    q = (q as ::core::ffi::c_double
                        * crate::stdlib::pow(
                            total_bits as ::core::ffi::c_double / (*rcc).expected_bits_sum,
                            w,
                        )) as ::core::ffi::c_float;
                }
                (*rcc).qp_novbv = qscale2qp(q);
                if (*rcc).b_vbv != 0 {
                    let mut expected_size: ::core::ffi::c_double =
                        qscale2bits(&raw mut rce, q as ::core::ffi::c_double);
                    let mut expected_vbv: ::core::ffi::c_double =
                        (*rcc).buffer_fill + (*rcc).buffer_rate - expected_size;
                    let mut expected_fullness: ::core::ffi::c_double =
                        rce.expected_vbv / (*rcc).buffer_size;
                    let mut qmax: ::core::ffi::c_double = q as ::core::ffi::c_double
                        * (2 as ::core::ffi::c_int as ::core::ffi::c_double - expected_fullness);
                    let mut size_constraint: ::core::ffi::c_double =
                        1 as ::core::ffi::c_int as ::core::ffi::c_double + expected_fullness;
                    qmax = if qmax > rce.new_qscale {
                        qmax
                    } else {
                        rce.new_qscale
                    };
                    if expected_fullness < 0.05f64 {
                        qmax = lmax;
                    }
                    qmax = if qmax < lmax { qmax } else { lmax };
                    while expected_vbv < rce.expected_vbv / size_constraint
                        && (q as ::core::ffi::c_double) < qmax
                        || expected_vbv < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            && (q as ::core::ffi::c_double) < lmax
                    {
                        q = (q as ::core::ffi::c_double * 1.05f64) as ::core::ffi::c_float;
                        expected_size = qscale2bits(&raw mut rce, q as ::core::ffi::c_double);
                        expected_vbv = (*rcc).buffer_fill + (*rcc).buffer_rate - expected_size;
                    }
                    (*rcc).last_satd =
                        crate::src::encoder::analyse::slicetype_c::x264_8_rc_analyse_slice(
                            h as *mut crate::src::common::common::x264_t,
                        );
                }
                q = x264_clip3f(q as ::core::ffi::c_double, lmin, lmax) as ::core::ffi::c_float;
            } else {
                let mut wanted_bits: ::core::ffi::c_double = 0.;
                let mut overflow: ::core::ffi::c_double =
                    1 as ::core::ffi::c_int as ::core::ffi::c_double;
                (*rcc).last_satd =
                    crate::src::encoder::analyse::slicetype_c::x264_8_rc_analyse_slice(
                        h as *mut crate::src::common::common::x264_t,
                    );
                (*rcc).short_term_cplxsum *= 0.5f64;
                (*rcc).short_term_cplxcount *= 0.5f64;
                (*rcc).short_term_cplxsum += (*rcc).last_satd as ::core::ffi::c_double
                    / (x264_clip3f(
                        (*(*h).fenc).f_duration as ::core::ffi::c_double,
                        (0.01f32
                            / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        (1.00f32
                            / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                    ) / (0.04f32
                        / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_float)
                        as ::core::ffi::c_double);
                (*rcc).short_term_cplxcount += 1.;
                rce.tex_bits = (*rcc).last_satd;
                rce.blurred_complexity = ((*rcc).short_term_cplxsum / (*rcc).short_term_cplxcount)
                    as ::core::ffi::c_float;
                rce.mv_bits = 0 as ::core::ffi::c_int;
                rce.p_count = (*rcc).nmb;
                rce.i_count = 0 as ::core::ffi::c_int;
                rce.s_count = 0 as ::core::ffi::c_int;
                rce.qscale = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                rce.pict_type = pict_type;
                rce.i_duration = (*(*h).fenc).i_duration;
                if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF {
                    q = get_qscale(
                        h,
                        &raw mut rce,
                        (*rcc).rate_factor_constant,
                        (*(*h).fenc).i_frame,
                    ) as ::core::ffi::c_float;
                } else {
                    q = get_qscale(
                        h,
                        &raw mut rce,
                        (*rcc).wanted_bits_window / (*rcc).cplxr_sum,
                        (*(*h).fenc).i_frame,
                    ) as ::core::ffi::c_float;
                    if (*rcc).b_vbv_min_rate == 0 && (*rcc).last_satd != 0 {
                        let mut i_frame_done: ::core::ffi::c_int = (*h).i_frame;
                        let mut time_done: ::core::ffi::c_double =
                            i_frame_done as ::core::ffi::c_double / (*rcc).fps;
                        if (*h).param.b_vfr_input != 0 && i_frame_done > 0 as ::core::ffi::c_int {
                            time_done = ((*(*h).fenc).i_reordered_pts - (*h).i_reordered_pts_delay)
                                as ::core::ffi::c_double
                                * (*h).param.i_timebase_num as ::core::ffi::c_double
                                / (*h).param.i_timebase_den as ::core::ffi::c_double;
                        }
                        wanted_bits = time_done * (*rcc).bitrate;
                        if wanted_bits > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            abr_buffer *= if 1 as ::core::ffi::c_int as ::core::ffi::c_double
                                > crate::stdlib::sqrt(time_done)
                            {
                                1 as ::core::ffi::c_int as ::core::ffi::c_double
                            } else {
                                crate::stdlib::sqrt(time_done)
                            };
                            overflow = x264_clip3f(
                                1.0f64 + (predicted_bits - wanted_bits) / abr_buffer,
                                0.5f64,
                                2 as ::core::ffi::c_int as ::core::ffi::c_double,
                            );
                            q = (q as ::core::ffi::c_double * overflow) as ::core::ffi::c_float;
                        }
                    }
                }
                if pict_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                    && (*h).param.i_keyint_max > 1 as ::core::ffi::c_int
                    && (*rcc).last_non_b_pict_type
                        != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                {
                    q = qp2qscale(
                        ((*rcc).accum_p_qp / (*rcc).accum_p_norm) as ::core::ffi::c_float,
                    );
                    q /= (*h).param.rc.f_ip_factor;
                } else if (*h).i_frame > 0 as ::core::ffi::c_int {
                    if (*h).param.rc.i_rc_method != crate::x264_h::X264_RC_CRF {
                        let mut lmin_0: ::core::ffi::c_double =
                            (*rcc).last_qscale_for[pict_type as usize] / (*rcc).lstep;
                        let mut lmax_0: ::core::ffi::c_double =
                            (*rcc).last_qscale_for[pict_type as usize] * (*rcc).lstep;
                        if overflow > 1.1f64 && (*h).i_frame > 3 as ::core::ffi::c_int {
                            lmax_0 *= (*rcc).lstep;
                        } else if overflow < 0.9f64 {
                            lmin_0 /= (*rcc).lstep;
                        }
                        q = x264_clip3f(q as ::core::ffi::c_double, lmin_0, lmax_0)
                            as ::core::ffi::c_float;
                    }
                } else if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF
                    && (*rcc).qcompress != 1 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    q = qp2qscale(
                        (if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF {
                            (*h).param.rc.f_rf_constant
                        } else {
                            24 as ::core::ffi::c_int as ::core::ffi::c_float
                        }) + crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_float,
                    ) / (*h).param.rc.f_ip_factor;
                }
                (*rcc).qp_novbv = qscale2qp(q);
                q = vbv_pass1(h, pict_type, q as ::core::ffi::c_double) as ::core::ffi::c_float;
            }
            (*rcc).last_qscale = q as ::core::ffi::c_double;
            (*rcc).last_qscale_for[pict_type as usize] = (*rcc).last_qscale;
            if !((*rcc).b_2pass != 0 && (*rcc).b_vbv == 0)
                && (*(*h).fenc).i_frame == 0 as ::core::ffi::c_int
            {
                (*rcc).last_qscale_for
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize] =
                    (q * (*h).param.rc.f_ip_factor) as ::core::ffi::c_double;
            }
            if (*rcc).b_2pass != 0 {
                (*rcc).frame_size_planned = qscale2bits(&raw mut rce, q as ::core::ffi::c_double);
            } else {
                (*rcc).frame_size_planned = predict_size(
                    (*rcc).pred.offset((*h).sh.i_type as isize) as *mut predictor_t,
                    q,
                    (*rcc).last_satd as ::core::ffi::c_float,
                ) as ::core::ffi::c_double;
            }
            if (*rcc).b_vbv != 0 {
                let mut frame_size_maximum_0: ::core::ffi::c_double = if (*rcc).frame_size_maximum
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
                    q = (q as ::core::ffi::c_double
                        * ((*rcc).frame_size_planned / frame_size_maximum_0))
                        as ::core::ffi::c_float;
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
                &mut (*rcc).frame_size_estimated as *mut ::core::ffi::c_float,
                (*rcc).frame_size_planned as ::core::ffi::c_float,
            );
            return q;
        };
    }
}
unsafe extern "C" fn threads_normalize_predictors(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut totalsize: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).param.i_threads {
            totalsize += (*(*(*h).thread[i as usize]).rc).slice_size_planned;
            i += 1;
        }
        let mut factor: ::core::ffi::c_double = (*(*h).rc).frame_size_planned / totalsize;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*h).param.i_threads {
            (*(*(*h).thread[i_0 as usize]).rc).slice_size_planned *= factor;
            i_0 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_threads_distribute_ratecontrol(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut row: ::core::ffi::c_int = 0;
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut qscale: ::core::ffi::c_float = qp2qscale((*rc).qpm);
        if (*h).i_frame == 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*h).param.i_threads {
                let mut t: *mut crate::src::common::common::x264_t = (*h).thread[i as usize];
                if t != h {
                    crate::stdlib::memcpy(
                        &raw mut (*(*t).rc).row_preds as *mut [predictor_t; 2]
                            as *mut ::core::ffi::c_void,
                        &raw mut (*rc).row_preds as *mut [predictor_t; 2]
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[[predictor_t; 2]; 3]>()
                            as crate::__stddef_size_t_h::size_t,
                    );
                }
                i += 1;
            }
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*h).param.i_threads {
            let mut t_0: *mut crate::src::common::common::x264_t = (*h).thread[i_0 as usize];
            if t_0 != h {
                crate::stdlib::memcpy(
                    (*t_0).rc as *mut ::core::ffi::c_void,
                    rc as *const ::core::ffi::c_void,
                    576 as crate::__stddef_size_t_h::size_t,
                );
            }
            (*(*t_0).rc).row_pred =
                &raw mut *(&raw mut (*(*t_0).rc).row_preds as *mut [predictor_t; 2])
                    .offset((*h).sh.i_type as isize) as *mut predictor_t;
            if (*rc).b_vbv != 0 && (*rc).frame_size_planned != 0. {
                let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                row = (*t_0).i_threadslice_start;
                while row < (*t_0).i_threadslice_end {
                    size += *(*(*h).fdec).i_row_satd.offset(row as isize);
                    row += 1;
                }
                (*(*t_0).rc).slice_size_planned = predict_size(
                    (*rc).pred.offset(
                        ((*h).sh.i_type + (i_0 + 1 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut predictor_t,
                    qscale,
                    size as ::core::ffi::c_float,
                ) as ::core::ffi::c_double;
            } else {
                (*(*t_0).rc).slice_size_planned = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            i_0 += 1;
        }
        if (*rc).b_vbv != 0 && (*rc).frame_size_planned != 0. {
            threads_normalize_predictors(h);
            if (*rc).single_frame_vbv != 0 {
                let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_1 < (*h).param.i_threads {
                    let mut t_1: *mut crate::src::common::common::x264_t =
                        (*h).thread[i_1 as usize];
                    let mut max_frame_error: ::core::ffi::c_float = x264_clip3f(
                        1.0f64
                            / ((*t_1).i_threadslice_end - (*t_1).i_threadslice_start)
                                as ::core::ffi::c_double,
                        0.05f64,
                        0.25f64,
                    )
                        as ::core::ffi::c_float;
                    (*(*t_1).rc).slice_size_planned +=
                        (2 as ::core::ffi::c_int as ::core::ffi::c_float * max_frame_error)
                            as ::core::ffi::c_double
                            * (*rc).frame_size_planned;
                    i_1 += 1;
                }
                threads_normalize_predictors(h);
            }
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < (*h).param.i_threads {
                ::core::ptr::write_volatile(
                    &mut (*(*(*h).thread[i_2 as usize]).rc).frame_size_estimated
                        as *mut ::core::ffi::c_float,
                    (*(*(*h).thread[i_2 as usize]).rc).slice_size_planned as ::core::ffi::c_float,
                );
                i_2 += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_threads_merge_ratecontrol(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut rc: *mut x264_ratecontrol_t = (*h).rc;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).param.i_threads {
            let mut t: *mut crate::src::common::common::x264_t = (*h).thread[i as usize];
            let mut rct: *mut x264_ratecontrol_t = (*(*h).thread[i as usize]).rc;
            if (*h).param.rc.i_vbv_buffer_size != 0 {
                let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut row: ::core::ffi::c_int = (*t).i_threadslice_start;
                while row < (*t).i_threadslice_end {
                    size += *(*(*h).fdec).i_row_satd.offset(row as isize);
                    row += 1;
                }
                let mut bits: ::core::ffi::c_int = (*t).stat.frame.i_mv_bits
                    + (*t).stat.frame.i_tex_bits
                    + (*t).stat.frame.i_misc_bits;
                let mut mb_count: ::core::ffi::c_int =
                    ((*t).i_threadslice_end - (*t).i_threadslice_start) * (*h).mb.i_mb_width;
                update_predictor(
                    (*rc).pred.offset(
                        ((*h).sh.i_type + (i + 1 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut predictor_t,
                    qp2qscale((*rct).qpa_rc / mb_count as ::core::ffi::c_float),
                    size as ::core::ffi::c_float,
                    bits as ::core::ffi::c_float,
                );
            }
            if !(i == 0) {
                (*rc).qpa_rc += (*rct).qpa_rc;
                (*rc).qpa_aq += (*rct).qpa_aq;
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_thread_sync_ratecontrol(
    mut cur: *mut crate::src::common::common::x264_t,
    mut prev: *mut crate::src::common::common::x264_t,
    mut next: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        if cur != prev {
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).accum_p_qp as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).accum_p_qp as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).accum_p_norm as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).accum_p_norm as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).last_satd as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).last_satd as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).last_rceq as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).last_rceq as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).last_qscale_for as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).last_qscale_for as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_double; 3]>()
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).last_non_b_pict_type as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).last_non_b_pict_type as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).short_term_cplxsum as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).short_term_cplxsum as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).short_term_cplxcount as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).short_term_cplxcount as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).bframes as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).bframes as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).prev_zone as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).prev_zone as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut crate::x264_h::x264_zone_t>()
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).mbtree.qpbuf_pos as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).mbtree.qpbuf_pos as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).bitrate as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).bitrate as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).buffer_size as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).buffer_size as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).buffer_rate as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).buffer_rate as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).vbv_max_rate as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).vbv_max_rate as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).single_frame_vbv as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).single_frame_vbv as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).cbr_decay as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).cbr_decay as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).rate_factor_constant as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).rate_factor_constant as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_double>() as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*(*cur).rc).rate_factor_max_increment as *mut ::core::ffi::c_void,
                &raw mut (*(*prev).rc).rate_factor_max_increment as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_float>() as crate::__stddef_size_t_h::size_t,
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
}
unsafe extern "C" fn find_underflow(
    mut h: *mut crate::src::common::common::x264_t,
    mut fills: *mut ::core::ffi::c_double,
    mut t0: *mut ::core::ffi::c_int,
    mut t1: *mut ::core::ffi::c_int,
    mut over: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let buffer_min: ::core::ffi::c_double = 0.1f64 * (*rcc).buffer_size;
        let buffer_max: ::core::ffi::c_double = 0.9f64 * (*rcc).buffer_size;
        let mut fill: ::core::ffi::c_double =
            *fills.offset((*t0 - 1 as ::core::ffi::c_int) as isize);
        let mut parity: ::core::ffi::c_double = if over != 0 { 1.0f64 } else { -1.0f64 };
        let mut start: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut end: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut i: ::core::ffi::c_int = *t0;
        while i < (*rcc).num_entries {
            fill += ((**(*rcc).entry_out.offset(i as isize)).i_cpb_duration
                as ::core::ffi::c_double
                * (*rcc).vbv_max_rate
                * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_num_units_in_tick as ::core::ffi::c_double
                / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as ::core::ffi::c_double
                - qscale2bits(
                    *(*rcc).entry_out.offset(i as isize),
                    (**(*rcc).entry_out.offset(i as isize)).new_qscale,
                ))
                * parity;
            fill = x264_clip3f(
                fill,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*rcc).buffer_size,
            );
            *fills.offset(i as isize) = fill;
            if fill <= buffer_min || i == 0 as ::core::ffi::c_int {
                if end >= 0 as ::core::ffi::c_int {
                    break;
                }
                start = i;
            } else if fill >= buffer_max && start >= 0 as ::core::ffi::c_int {
                end = i;
            }
            i += 1;
        }
        *t0 = start;
        *t1 = end;
        return (start >= 0 as ::core::ffi::c_int && end >= 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fix_underflow(
    mut h: *mut crate::src::common::common::x264_t,
    mut t0: ::core::ffi::c_int,
    mut t1: ::core::ffi::c_int,
    mut adjustment: ::core::ffi::c_double,
    mut qscale_min: ::core::ffi::c_double,
    mut qscale_max: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let mut qscale_orig: ::core::ffi::c_double = 0.;
        let mut qscale_new: ::core::ffi::c_double = 0.;
        let mut adjusted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if t0 > 0 as ::core::ffi::c_int {
            t0 += 1;
        }
        let mut i: ::core::ffi::c_int = t0;
        while i <= t1 {
            qscale_orig = (**(*rcc).entry_out.offset(i as isize)).new_qscale;
            qscale_orig = x264_clip3f(qscale_orig, qscale_min, qscale_max);
            qscale_new = qscale_orig * adjustment;
            qscale_new = x264_clip3f(qscale_new, qscale_min, qscale_max);
            (**(*rcc).entry_out.offset(i as isize)).new_qscale = qscale_new;
            adjusted = (adjusted != 0 || qscale_new != qscale_orig) as ::core::ffi::c_int;
            i += 1;
        }
        return adjusted;
    }
}
unsafe extern "C" fn count_expected_bits(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_double {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let mut expected_bits: ::core::ffi::c_double =
            0 as ::core::ffi::c_int as ::core::ffi::c_double;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*rcc).num_entries {
            let mut rce: *mut ratecontrol_entry_t = *(*rcc).entry_out.offset(i as isize);
            (*rce).expected_bits = expected_bits;
            expected_bits += qscale2bits(rce, (*rce).new_qscale);
            i += 1;
        }
        return expected_bits;
    }
}
unsafe extern "C" fn vbv_pass2(
    mut h: *mut crate::src::common::common::x264_t,
    mut all_available_bits: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let mut expected_bits: ::core::ffi::c_double =
            0 as ::core::ffi::c_int as ::core::ffi::c_double;
        let mut adjustment: ::core::ffi::c_double = 0.;
        let mut prev_bits: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        let mut t0: ::core::ffi::c_int = 0;
        let mut t1: ::core::ffi::c_int = 0;
        let mut qscale_min: ::core::ffi::c_double =
            qp2qscale((*h).param.rc.i_qp_min as ::core::ffi::c_float) as ::core::ffi::c_double;
        let mut qscale_max: ::core::ffi::c_double =
            qp2qscale((*h).param.rc.i_qp_max as ::core::ffi::c_float) as ::core::ffi::c_double;
        let mut _iterations: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut adj_min: ::core::ffi::c_int = 0;
        let mut adj_max: ::core::ffi::c_int = 0;
        let mut fills: *mut ::core::ffi::c_double = crate::src::common::base::x264_malloc(
            (((*rcc).num_entries + 1 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
                as crate::stdlib::int64_t,
        ) as *mut ::core::ffi::c_double;
        if fills.is_null() {
            return -(1 as ::core::ffi::c_int);
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
                    *fills.offset(-(1 as ::core::ffi::c_int) as isize) = (*rcc).buffer_size
                        * (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double;
                    t0 = 0 as ::core::ffi::c_int;
                    adj_min = 1 as ::core::ffi::c_int;
                    while adj_min != 0
                        && find_underflow(
                            h,
                            fills,
                            &raw mut t0,
                            &raw mut t1,
                            1 as ::core::ffi::c_int,
                        ) != 0
                    {
                        adj_min = fix_underflow(h, t0, t1, adjustment, qscale_min, qscale_max);
                        t0 = t1;
                    }
                }
                *fills.offset(-(1 as ::core::ffi::c_int) as isize) = (*rcc).buffer_size
                    * (1.0f64 - (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double);
                t0 = 0 as ::core::ffi::c_int;
                adj_max = 1 as ::core::ffi::c_int;
                while adj_max != 0
                    && find_underflow(h, fills, &raw mut t0, &raw mut t1, 0 as ::core::ffi::c_int)
                        != 0
                {
                    adj_max = fix_underflow(h, t0, t1, 1.001f64, qscale_min, qscale_max);
                }
                expected_bits = count_expected_bits(h);
                if !(expected_bits < 0.995f64 * all_available_bits
                    && (expected_bits + 0.5f64) as crate::stdlib::int64_t
                        > (prev_bits + 0.5f64) as crate::stdlib::int64_t)
                {
                    break;
                }
            }
            if adj_max == 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"vbv-maxrate issue, qpmax or vbv-maxrate too low\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*rcc).num_entries {
                (**(*rcc).entry_out.offset(i as isize)).expected_vbv =
                    (*rcc).buffer_size - *fills.offset(i as isize);
                i += 1;
            }
            crate::src::common::base::x264_free(
                fills.offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void
            );
            return 0 as ::core::ffi::c_int;
        };
    }
}
unsafe extern "C" fn init_pass2(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
        let mut all_const_bits: crate::stdlib::uint64_t = 0 as crate::stdlib::uint64_t;
        let mut timescale: ::core::ffi::c_double =
            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .i_num_units_in_tick as ::core::ffi::c_double
                / (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .i_time_scale as ::core::ffi::c_double;
        let mut duration: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*rcc).num_entries {
            duration += (*(*rcc).entry.offset(i as isize)).i_duration as ::core::ffi::c_double;
            i += 1;
        }
        duration *= timescale;
        let mut all_available_bits: crate::stdlib::uint64_t =
            ((*h).param.rc.i_bitrate as ::core::ffi::c_double * 1000.0f64 * duration)
                as crate::stdlib::uint64_t;
        let mut rate_factor: ::core::ffi::c_double = 0.;
        let mut step_mult: ::core::ffi::c_double = 0.;
        let mut qblur: ::core::ffi::c_double = (*h).param.rc.f_qblur as ::core::ffi::c_double;
        let mut cplxblur: ::core::ffi::c_double =
            (*h).param.rc.f_complexity_blur as ::core::ffi::c_double;
        let filter_size: ::core::ffi::c_int =
            (qblur * 4 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int;
        let mut expected_bits: ::core::ffi::c_double = 0.;
        let mut blurred_qscale: *mut ::core::ffi::c_double =
            ::core::ptr::null_mut::<::core::ffi::c_double>();
        let mut base_cplx: ::core::ffi::c_double = ((*h).mb.i_mb_count
            * (if (*h).param.i_bframe != 0 {
                120 as ::core::ffi::c_int
            } else {
                80 as ::core::ffi::c_int
            })) as ::core::ffi::c_double;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*rcc).num_entries {
            let mut rce: *mut ratecontrol_entry_t =
                (*rcc).entry.offset(i_0 as isize) as *mut ratecontrol_entry_t;
            all_const_bits =
                all_const_bits.wrapping_add((*rce).misc_bits as crate::stdlib::uint64_t);
            i_0 += 1;
        }
        if all_available_bits < all_const_bits {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"requested bitrate is too low. estimated minimum is %d kbps\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (all_const_bits as ::core::ffi::c_double * (*rcc).fps
                    / ((*rcc).num_entries as ::core::ffi::c_double * 1000.0f64))
                    as ::core::ffi::c_int,
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*rcc).num_entries {
            let mut rce_0: *mut ratecontrol_entry_t =
                (*rcc).entry.offset(i_1 as isize) as *mut ratecontrol_entry_t;
            let mut weight_sum: ::core::ffi::c_double =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
            let mut cplx_sum: ::core::ffi::c_double =
                0 as ::core::ffi::c_int as ::core::ffi::c_double;
            let mut weight: ::core::ffi::c_double = 1.0f64;
            let mut gaussian_weight: ::core::ffi::c_double = 0.;
            let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while (j as ::core::ffi::c_double)
                < cplxblur * 2 as ::core::ffi::c_int as ::core::ffi::c_double
                && j < (*rcc).num_entries - i_1
            {
                let mut rcj: *mut ratecontrol_entry_t =
                    (*rcc).entry.offset((i_1 + j) as isize) as *mut ratecontrol_entry_t;
                let mut frame_duration: ::core::ffi::c_double = x264_clip3f(
                    (*rcj).i_duration as ::core::ffi::c_double * timescale,
                    (0.01f32
                        / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_float) as ::core::ffi::c_double,
                    (1.00f32
                        / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_float) as ::core::ffi::c_double,
                ) / (0.04f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_float)
                    as ::core::ffi::c_double;
                weight *= 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - crate::stdlib::pow(
                        ((*rcj).i_count as ::core::ffi::c_float
                            / (*rcc).nmb as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        2 as ::core::ffi::c_int as ::core::ffi::c_double,
                    );
                if weight < 0.0001f64 {
                    break;
                }
                gaussian_weight =
                    weight * crate::stdlib::exp((-j * j) as ::core::ffi::c_double / 200.0f64);
                weight_sum += gaussian_weight;
                cplx_sum += gaussian_weight
                    * (qscale2bits(rcj, 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                        - (*rcj).misc_bits as ::core::ffi::c_double)
                    / frame_duration;
                j += 1;
            }
            weight = 1.0f64;
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 as ::core::ffi::c_double
                <= cplxblur * 2 as ::core::ffi::c_int as ::core::ffi::c_double
                && j_0 <= i_1
            {
                let mut rcj_0: *mut ratecontrol_entry_t =
                    (*rcc).entry.offset((i_1 - j_0) as isize) as *mut ratecontrol_entry_t;
                let mut frame_duration_0: ::core::ffi::c_double = x264_clip3f(
                    (*rcj_0).i_duration as ::core::ffi::c_double * timescale,
                    (0.01f32
                        / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_float) as ::core::ffi::c_double,
                    (1.00f32
                        / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_float) as ::core::ffi::c_double,
                ) / (0.04f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_float)
                    as ::core::ffi::c_double;
                gaussian_weight =
                    weight * crate::stdlib::exp((-j_0 * j_0) as ::core::ffi::c_double / 200.0f64);
                weight_sum += gaussian_weight;
                cplx_sum += gaussian_weight
                    * (qscale2bits(rcj_0, 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                        - (*rcj_0).misc_bits as ::core::ffi::c_double)
                    / frame_duration_0;
                weight *= 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - crate::stdlib::pow(
                        ((*rcj_0).i_count as ::core::ffi::c_float
                            / (*rcc).nmb as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        2 as ::core::ffi::c_int as ::core::ffi::c_double,
                    );
                if weight < 0.0001f64 {
                    break;
                }
                j_0 += 1;
            }
            (*rce_0).blurred_complexity = (cplx_sum / weight_sum) as ::core::ffi::c_float;
            i_1 += 1;
        }
        let mut qscale: *mut ::core::ffi::c_double = crate::src::common::base::x264_malloc(
            (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                .wrapping_mul((*rcc).num_entries as usize) as crate::stdlib::int64_t,
        ) as *mut ::core::ffi::c_double;
        if !qscale.is_null() {
            if filter_size > 1 as ::core::ffi::c_int {
                blurred_qscale = crate::src::common::base::x264_malloc(
                    (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                        .wrapping_mul((*rcc).num_entries as usize)
                        as crate::stdlib::int64_t,
                ) as *mut ::core::ffi::c_double;
                if blurred_qscale.is_null() {
                    c2rust_current_block = 13029897179669627124;
                } else {
                    c2rust_current_block = 12199444798915819164;
                }
            } else {
                blurred_qscale = qscale;
                c2rust_current_block = 12199444798915819164;
            }
            match c2rust_current_block {
                13029897179669627124 => {}
                _ => {
                    expected_bits = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_2 < (*rcc).num_entries {
                        let mut q: ::core::ffi::c_double = get_qscale(
                            h,
                            (*rcc).entry.offset(i_2 as isize) as *mut ratecontrol_entry_t,
                            1.0f64,
                            i_2,
                        );
                        expected_bits += qscale2bits(
                            (*rcc).entry.offset(i_2 as isize) as *mut ratecontrol_entry_t,
                            q,
                        );
                        (*rcc).last_qscale_for
                            [(*(*rcc).entry.offset(i_2 as isize)).pict_type as usize] = q;
                        i_2 += 1;
                    }
                    step_mult = all_available_bits as ::core::ffi::c_double / expected_bits;
                    rate_factor = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    let mut step: ::core::ffi::c_double = 1E4f64 * step_mult;
                    while step > 1E-7f64 * step_mult {
                        expected_bits = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        rate_factor += step;
                        (*rcc).last_non_b_pict_type = -(1 as ::core::ffi::c_int);
                        (*rcc).last_accum_p_norm = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                        (*rcc).accum_p_norm = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        (*rcc).last_qscale_for[2 as ::core::ffi::c_int as usize] =
                            crate::stdlib::pow(
                                base_cplx,
                                1 as ::core::ffi::c_int as ::core::ffi::c_double - (*rcc).qcompress,
                            ) / rate_factor;
                        (*rcc).last_qscale_for[1 as ::core::ffi::c_int as usize] =
                            (*rcc).last_qscale_for[2 as ::core::ffi::c_int as usize];
                        (*rcc).last_qscale_for[0 as ::core::ffi::c_int as usize] =
                            (*rcc).last_qscale_for[1 as ::core::ffi::c_int as usize];
                        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_3 < (*rcc).num_entries {
                            *qscale.offset(i_3 as isize) = get_qscale(
                                h,
                                (*rcc).entry.offset(i_3 as isize) as *mut ratecontrol_entry_t,
                                rate_factor,
                                -(1 as ::core::ffi::c_int),
                            );
                            (*rcc).last_qscale_for
                                [(*(*rcc).entry.offset(i_3 as isize)).pict_type as usize] =
                                *qscale.offset(i_3 as isize);
                            i_3 += 1;
                        }
                        let mut i_4: ::core::ffi::c_int =
                            (*rcc).num_entries - 1 as ::core::ffi::c_int;
                        while i_4 >= 0 as ::core::ffi::c_int {
                            *qscale.offset(i_4 as isize) = get_diff_limited_q(
                                h,
                                (*rcc).entry.offset(i_4 as isize) as *mut ratecontrol_entry_t,
                                *qscale.offset(i_4 as isize),
                                i_4,
                            );
                            '_c2rust_label: {
                                if *qscale.offset(i_4 as isize)
                                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                } else {
                                    crate::stdlib::__assert_fail(
                                        b"qscale[i] >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"encoder/ratecontrol.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        3050 as ::core::ffi::c_uint,
                                        b"int init_pass2(x264_t *)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            i_4 -= 1;
                        }
                        if filter_size > 1 as ::core::ffi::c_int {
                            '_c2rust_label_0: {
                                if filter_size % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                {
                                } else {
                                    crate::stdlib::__assert_fail(
                                        b"filter_size%2 == 1\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"encoder/ratecontrol.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        3056 as ::core::ffi::c_uint,
                                        b"int init_pass2(x264_t *)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_5 < (*rcc).num_entries {
                                let mut rce_1: *mut ratecontrol_entry_t =
                                    (*rcc).entry.offset(i_5 as isize) as *mut ratecontrol_entry_t;
                                let mut q_0: ::core::ffi::c_double = 0.0f64;
                                let mut sum: ::core::ffi::c_double = 0.0f64;
                                let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while j_1 < filter_size {
                                    let mut idx: ::core::ffi::c_int =
                                        i_5 + j_1 - filter_size / 2 as ::core::ffi::c_int;
                                    let mut d: ::core::ffi::c_double =
                                        (idx - i_5) as ::core::ffi::c_double;
                                    let mut coeff: ::core::ffi::c_double = if qblur
                                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                    {
                                        1.0f64
                                    } else {
                                        crate::stdlib::exp(-d * d / (qblur * qblur))
                                    };
                                    if !(idx < 0 as ::core::ffi::c_int || idx >= (*rcc).num_entries)
                                    {
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
                        let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_6 < (*rcc).num_entries {
                            let mut rce_2: *mut ratecontrol_entry_t =
                                (*rcc).entry.offset(i_6 as isize) as *mut ratecontrol_entry_t;
                            (*rce_2).new_qscale = clip_qscale(
                                h,
                                (*rce_2).pict_type,
                                *blurred_qscale.offset(i_6 as isize),
                            );
                            '_c2rust_label_1: {
                                if (*rce_2).new_qscale
                                    >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                } else {
                                    crate::stdlib::__assert_fail(
                                        b"rce->new_qscale >= 0\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"encoder/ratecontrol.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        3083 as ::core::ffi::c_uint,
                                        b"int init_pass2(x264_t *)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            expected_bits += qscale2bits(rce_2, (*rce_2).new_qscale);
                            i_6 += 1;
                        }
                        if expected_bits > all_available_bits as ::core::ffi::c_double {
                            rate_factor -= step;
                        }
                        step *= 0.5f64;
                    }
                    crate::src::common::base::x264_free(qscale as *mut ::core::ffi::c_void);
                    if filter_size > 1 as ::core::ffi::c_int {
                        crate::src::common::base::x264_free(
                            blurred_qscale as *mut ::core::ffi::c_void,
                        );
                    }
                    if (*rcc).b_vbv != 0 {
                        if vbv_pass2(h, all_available_bits as ::core::ffi::c_double) != 0 {
                            return -(1 as ::core::ffi::c_int);
                        }
                    }
                    expected_bits = count_expected_bits(h);
                    if crate::stdlib::fabs(
                        expected_bits / all_available_bits as ::core::ffi::c_double - 1.0f64,
                    ) > 0.01f64
                    {
                        let mut avgq: ::core::ffi::c_double =
                            0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_7 < (*rcc).num_entries {
                            avgq += (*(*rcc).entry.offset(i_7 as isize)).new_qscale;
                            i_7 += 1;
                        }
                        avgq = qscale2qp(
                            (avgq / (*rcc).num_entries as ::core::ffi::c_double)
                                as ::core::ffi::c_float,
                        ) as ::core::ffi::c_double;
                        if expected_bits > all_available_bits as ::core::ffi::c_double
                            || (*rcc).b_vbv == 0
                        {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_WARNING_1,
                                b"Error: 2pass curve failed to converge\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        crate::src::common::common::x264_8_log(
                            h as *mut crate::src::common::common::x264_t,
                            crate::x264_h::X264_LOG_WARNING_1,
                            b"target: %.2f kbit/s, expected: %.2f kbit/s, avg QP: %.4f\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*h).param.rc.i_bitrate as ::core::ffi::c_float
                                as ::core::ffi::c_double,
                            expected_bits * (*rcc).fps
                                / ((*rcc).num_entries as ::core::ffi::c_double * 1000.0f64),
                            avgq,
                        );
                        if expected_bits < all_available_bits as ::core::ffi::c_double
                            && avgq
                                < ((*h).param.rc.i_qp_min + 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_double
                        {
                            if (*h).param.rc.i_qp_min > 0 as ::core::ffi::c_int {
                                crate::src::common::common::x264_8_log(
                                    h as *mut crate::src::common::common::x264_t,
                                    crate::x264_h::X264_LOG_WARNING_1,
                                    b"try reducing target bitrate or reducing qp_min (currently %d)\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*h).param.rc.i_qp_min,
                                );
                            } else {
                                crate::src::common::common::x264_8_log(
                                    h as *mut crate::src::common::common::x264_t,
                                    crate::x264_h::X264_LOG_WARNING_1,
                                    b"try reducing target bitrate\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        } else if expected_bits > all_available_bits as ::core::ffi::c_double
                            && avgq
                                > ((*h).param.rc.i_qp_max - 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_double
                        {
                            if (*h).param.rc.i_qp_max < crate::src::common::common::QP_MAX {
                                crate::src::common::common::x264_8_log(
                                    h as *mut crate::src::common::common::x264_t,
                                    crate::x264_h::X264_LOG_WARNING_1,
                                    b"try increasing target bitrate or increasing qp_max (currently %d)\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*h).param.rc.i_qp_max,
                                );
                            } else {
                                crate::src::common::common::x264_8_log(
                                    h as *mut crate::src::common::common::x264_t,
                                    crate::x264_h::X264_LOG_WARNING_1,
                                    b"try increasing target bitrate\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        } else if !((*rcc).b_2pass != 0 && (*rcc).b_vbv != 0) {
                            crate::src::common::common::x264_8_log(
                                h as *mut crate::src::common::common::x264_t,
                                crate::x264_h::X264_LOG_WARNING_1,
                                b"internal error\n\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        return -(1 as ::core::ffi::c_int);
    }
}
