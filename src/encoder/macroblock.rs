pub mod base_h {
    pub static mut x264_scan8: [crate::stdlib::uint8_t; 51] = [
        (4i32 + 1i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 1i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 2i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 2i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 1i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 1i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 2i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 2i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 3i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 3i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 4i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 4i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 3i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 3i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 4i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 4i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 6i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 6i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 7i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 7i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 6i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 6i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 7i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 7i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 8i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 8i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 9i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 9i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 8i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 8i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 9i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 9i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 11i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 11i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 12i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 12i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 11i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 11i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 12i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 12i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 13i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 13i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 14i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 14i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 13i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 13i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 14i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 14i32 * 8i32) as crate::stdlib::uint8_t,
        (0i32 + 0i32 * 8i32) as crate::stdlib::uint8_t,
        (0i32 + 5i32 * 8i32) as crate::stdlib::uint8_t,
        (0i32 + 10i32 * 8i32) as crate::stdlib::uint8_t,
    ];
    #[inline(always)]
    pub extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        if v < i_min {
            i_min
        } else if v > i_max {
            i_max
        } else {
            v
        }
    }
}
pub mod macroblock_h {
    pub static mut x264_pred_i4x4_neighbors: [crate::stdlib::uint8_t; 12] = [
        crate::src::common::macroblock::MB_TOP as crate::stdlib::uint8_t,
        crate::src::common::macroblock::MB_LEFT as crate::stdlib::uint8_t,
        (crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        crate::src::common::macroblock::MB_LEFT as crate::stdlib::uint8_t,
        crate::src::common::macroblock::MB_LEFT as crate::stdlib::uint8_t,
        crate::src::common::macroblock::MB_TOP as crate::stdlib::uint8_t,
        0u8,
    ];
    pub static mut block_idx_x: [crate::stdlib::uint8_t; 16] = [
        0u8, 1u8, 0u8, 1u8, 2u8, 3u8, 2u8, 3u8, 0u8, 1u8, 0u8, 1u8, 2u8, 3u8, 2u8, 3u8,
    ];
    pub static mut block_idx_y: [crate::stdlib::uint8_t; 16] = [
        0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 1u8, 1u8, 2u8, 2u8, 3u8, 3u8, 2u8, 2u8, 3u8, 3u8,
    ];
    pub static mut block_idx_xy_1d: [crate::stdlib::uint8_t; 16] = [
        0u8, 1u8, 4u8, 5u8, 2u8, 3u8, 6u8, 7u8, 8u8, 9u8, 12u8, 13u8, 10u8, 11u8, 14u8, 15u8,
    ];
    pub static mut block_idx_yx_1d: [crate::stdlib::uint8_t; 16] = [
        0u8, 4u8, 1u8, 5u8, 8u8, 12u8, 9u8, 13u8, 2u8, 6u8, 3u8, 7u8, 10u8, 14u8, 11u8, 15u8,
    ];
    pub static mut block_idx_xy_fenc: [crate::stdlib::uint8_t; 16] = [
        (0i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (1i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (0i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (1i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (2i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (3i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (2i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (3i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (0i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (1i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (0i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (1i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (2i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (3i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (2i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
        (3i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FENC_STRIDE)
            as crate::stdlib::uint8_t,
    ];
    pub static mut block_idx_xy_fdec: [crate::stdlib::uint16_t; 16] = [
        (0i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (1i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (0i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (1i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (2i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (3i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (2i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (3i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (0i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (1i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (0i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (1i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (2i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (3i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (2i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
        (3i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FDEC_STRIDE)
            as crate::stdlib::uint16_t,
    ];
    pub static mut ctx_cat_plane: [[crate::stdlib::uint8_t; 3]; 6] = [
        [
            crate::src::common::macroblock::DCT_LUMA_DC as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAU_DC as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAV_DC as crate::stdlib::uint8_t,
        ],
        [
            crate::src::common::macroblock::DCT_LUMA_AC as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAU_AC as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAV_AC as crate::stdlib::uint8_t,
        ],
        [
            crate::src::common::macroblock::DCT_LUMA_4x4 as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAU_4x4 as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAV_4x4 as crate::stdlib::uint8_t,
        ],
        [0u8, 0, 0],
        [0u8, 0, 0],
        [
            crate::src::common::macroblock::DCT_LUMA_8x8 as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAU_8x8 as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAV_8x8 as crate::stdlib::uint8_t,
        ],
    ];
}
pub mod osdep_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_ctz_4bit(mut x: crate::stdlib::uint32_t) -> ::core::ffi::c_int {
        unsafe {
            pub static mut lut: [crate::stdlib::uint8_t; 16] = [
                4u8, 0u8, 1u8, 0u8, 2u8, 0u8, 1u8, 0u8, 3u8, 0u8, 1u8, 0u8, 2u8, 0u8, 1u8, 0u8,
            ];
            lut[x as usize] as ::core::ffi::c_int
        }
    }
}
pub mod encoder_macroblock_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_quant_4x4(
        mut h: *mut crate::src::common::common::x264_t,
        mut dct: *mut crate::src::common::common::dctcoef,
        mut i_qp: ::core::ffi::c_int,
        mut ctx_block_cat: ::core::ffi::c_int,
        mut b_intra: ::core::ffi::c_int,
        mut p: ::core::ffi::c_int,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut i_quant_cat = if b_intra != 0 {
                if p != 0 {
                    crate::src::common::set::CQM_4IC as ::core::ffi::c_int
                } else {
                    crate::src::common::set::CQM_4IY as ::core::ffi::c_int
                }
            } else if p != 0 {
                crate::src::common::set::CQM_4PC as ::core::ffi::c_int
            } else {
                crate::src::common::set::CQM_4PY as ::core::ffi::c_int
            };
            if (*h).mb.noise_reduction {
                (*h).quantf.denoise_dct.expect("non-null function pointer")(
                    dct,
                    &raw mut *(*h)
                        .nr_residual_sum
                        .offset((0i32 + (p != 0) as ::core::ffi::c_int * 2i32) as isize)
                        as *mut crate::stdlib::uint32_t,
                    &raw mut *(*h)
                        .nr_offset
                        .offset((0i32 + (p != 0) as ::core::ffi::c_int * 2i32) as isize)
                        as *mut crate::src::common::common::udctcoef,
                    16i32,
                );
            }
            if (*h).mb.trellis {
                crate::src::encoder::analyse::rdo_c::x264_8_quant_4x4_trellis(
                    h,
                    dct,
                    i_quant_cat,
                    i_qp,
                    ctx_block_cat,
                    b_intra,
                    (p != 0) as ::core::ffi::c_int,
                    idx + p * 16i32,
                )
            } else {
                (*h).quantf.quant_4x4.expect("non-null function pointer")(
                    dct,
                    &raw mut *(*(&raw mut (*h).quant4_mf
                        as *mut *mut [crate::src::common::common::udctcoef; 16])
                        .offset(i_quant_cat as isize))
                    .offset(i_qp as isize)
                        as *mut crate::src::common::common::udctcoef,
                    &raw mut *(*(&raw mut (*h).quant4_bias
                        as *mut *mut [crate::src::common::common::udctcoef; 16])
                        .offset(i_quant_cat as isize))
                    .offset(i_qp as isize)
                        as *mut crate::src::common::common::udctcoef,
                )
            }
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_quant_8x8(
        mut h: *mut crate::src::common::common::x264_t,
        mut dct: *mut crate::src::common::common::dctcoef,
        mut i_qp: ::core::ffi::c_int,
        mut ctx_block_cat: ::core::ffi::c_int,
        mut b_intra: ::core::ffi::c_int,
        mut p: ::core::ffi::c_int,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut i_quant_cat = if b_intra != 0 {
                if p != 0 {
                    crate::src::common::set::CQM_8IC as ::core::ffi::c_int
                } else {
                    crate::src::common::set::CQM_8IY as ::core::ffi::c_int
                }
            } else if p != 0 {
                crate::src::common::set::CQM_8PC as ::core::ffi::c_int
            } else {
                crate::src::common::set::CQM_8PY as ::core::ffi::c_int
            };
            if (*h).mb.noise_reduction {
                (*h).quantf.denoise_dct.expect("non-null function pointer")(
                    dct,
                    &raw mut *(*h)
                        .nr_residual_sum
                        .offset((1i32 + (p != 0) as ::core::ffi::c_int * 2i32) as isize)
                        as *mut crate::stdlib::uint32_t,
                    &raw mut *(*h)
                        .nr_offset
                        .offset((1i32 + (p != 0) as ::core::ffi::c_int * 2i32) as isize)
                        as *mut crate::src::common::common::udctcoef,
                    64i32,
                );
            }
            if (*h).mb.trellis {
                crate::src::encoder::analyse::rdo_c::x264_8_quant_8x8_trellis(
                    h,
                    dct,
                    i_quant_cat,
                    i_qp,
                    ctx_block_cat,
                    b_intra,
                    (p != 0) as ::core::ffi::c_int,
                    idx + p * 4i32,
                )
            } else {
                (*h).quantf.quant_8x8.expect("non-null function pointer")(
                    dct,
                    &raw mut *(*(&raw mut (*h).quant8_mf
                        as *mut *mut [crate::src::common::common::udctcoef; 64])
                        .offset(i_quant_cat as isize))
                    .offset(i_qp as isize)
                        as *mut crate::src::common::common::udctcoef,
                    &raw mut *(*(&raw mut (*h).quant8_bias
                        as *mut *mut [crate::src::common::common::udctcoef; 64])
                        .offset(i_quant_cat as isize))
                    .offset(i_qp as isize)
                        as *mut crate::src::common::common::udctcoef,
                )
            }
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_encode_i4x4(
        mut h: *mut crate::src::common::common::x264_t,
        mut p: ::core::ffi::c_int,
        mut idx: ::core::ffi::c_int,
        mut i_qp: ::core::ffi::c_int,
        mut i_mode: ::core::ffi::c_int,
        mut b_predict: ::core::ffi::c_int,
    ) {
        unsafe {
            let mut nz = 0;
            let mut dct4x4 = [0; 16];
            let mut p_src = (*(&raw mut (*h).mb.pic.p_fenc
                as *mut *mut crate::src::common::common::pixel)
                .offset(p as isize))
            .offset(
                *(&raw const block_idx_xy_fenc as *const crate::stdlib::uint8_t)
                    .offset(idx as isize) as isize,
            );
            let mut p_dst = (*(&raw mut (*h).mb.pic.p_fdec
                as *mut *mut crate::src::common::common::pixel)
                .offset(p as isize))
            .offset(
                *(&raw const block_idx_xy_fdec as *const crate::stdlib::uint16_t)
                    .offset(idx as isize) as isize,
            );
            if b_predict != 0 {
                if (*h).mb.lossless {
                    x264_8_predict_lossless_4x4(h, p_dst, p, idx, i_mode);
                } else {
                    (*h).predict_4x4[i_mode as usize].expect("non-null function pointer")(p_dst);
                }
            }
            if (*h).mb.lossless {
                nz = (*h).zigzagf.sub_4x4.expect("non-null function pointer")(
                    &raw mut *(&raw mut (*h).dct.luma4x4
                        as *mut [crate::src::common::common::dctcoef; 16])
                        .offset((p * 16i32 + idx) as isize)
                        as *mut crate::src::common::common::dctcoef,
                    p_src,
                    p_dst,
                );
                (*h).mb.cache.non_zero_count[x264_scan8[(p * 16i32 + idx) as usize] as usize] =
                    nz as crate::stdlib::uint8_t;
                (*h).mb.i_cbp_luma |= nz << (idx >> 2i32);
                return;
            }
            (*h).dctf.sub4x4_dct.expect("non-null function pointer")(
                &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                p_src,
                p_dst,
            );
            nz = x264_quant_4x4(
                h,
                &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                i_qp,
                ctx_cat_plane
                    [crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int as usize]
                    [p as usize] as ::core::ffi::c_int,
                1i32,
                p,
                idx,
            );
            (*h).mb.cache.non_zero_count[x264_scan8[(p * 16i32 + idx) as usize] as usize] =
                nz as crate::stdlib::uint8_t;
            if nz != 0 {
                (*h).mb.i_cbp_luma |= (1i32) << (idx >> 2i32);
                (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                    &raw mut *(&raw mut (*h).dct.luma4x4
                        as *mut [crate::src::common::common::dctcoef; 16])
                        .offset((p * 16i32 + idx) as isize)
                        as *mut crate::src::common::common::dctcoef,
                    &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                );
                (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                    &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                    (*h).dequant4_mf[(if p != 0 {
                        crate::src::common::set::CQM_4IC as ::core::ffi::c_int
                    } else {
                        crate::src::common::set::CQM_4IY as ::core::ffi::c_int
                    }) as usize],
                    i_qp,
                );
                (*h).dctf.add4x4_idct.expect("non-null function pointer")(
                    p_dst,
                    &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                );
            }
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_encode_i8x8(
        mut h: *mut crate::src::common::common::x264_t,
        mut p: ::core::ffi::c_int,
        mut idx: ::core::ffi::c_int,
        mut i_qp: ::core::ffi::c_int,
        mut i_mode: ::core::ffi::c_int,
        mut edge: *mut crate::src::common::common::pixel,
        mut b_predict: ::core::ffi::c_int,
    ) {
        unsafe {
            let mut nz = 0;
            let mut dct8x8 = [0; 64];
            let mut x = idx & 1i32;
            let mut y = idx >> 1i32;
            let mut p_src = (*(&raw mut (*h).mb.pic.p_fenc
                as *mut *mut crate::src::common::common::pixel)
                .offset(p as isize))
            .offset((8i32 * x + 8i32 * y * crate::src::common::common::FENC_STRIDE) as isize);
            let mut p_dst = (*(&raw mut (*h).mb.pic.p_fdec
                as *mut *mut crate::src::common::common::pixel)
                .offset(p as isize))
            .offset((8i32 * x + 8i32 * y * crate::src::common::common::FDEC_STRIDE) as isize);
            if b_predict != 0 {
                if edge.is_null() {
                    let mut edge_buf = [0; 36];
                    (*h).predict_8x8_filter.expect("non-null function pointer")(
                        p_dst,
                        &raw mut edge_buf as *mut crate::src::common::common::pixel,
                        (*h).mb.i_neighbour8[idx as usize] as ::core::ffi::c_int,
                        x264_pred_i4x4_neighbors[i_mode as usize] as ::core::ffi::c_int,
                    );
                    edge = &raw mut edge_buf as *mut crate::src::common::common::pixel;
                }
                if (*h).mb.lossless {
                    x264_8_predict_lossless_8x8(h, p_dst, p, idx, i_mode, edge);
                } else {
                    (*h).predict_8x8[i_mode as usize].expect("non-null function pointer")(
                        p_dst, edge,
                    );
                }
            }
            if (*h).mb.lossless {
                nz = (*h).zigzagf.sub_8x8.expect("non-null function pointer")(
                    &raw mut *(&raw mut (*h).dct.luma8x8
                        as *mut [crate::src::common::common::dctcoef; 64])
                        .offset((p * 4i32 + idx) as isize)
                        as *mut crate::src::common::common::dctcoef,
                    p_src,
                    p_dst,
                );
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((p * 16i32 + idx * 4i32) as isize)
                        as ::core::ffi::c_int
                        + 0i32) as isize,
                ) as *mut crate::src::common::base::x264_union16_t))
                    .i = (nz * 0x101i32) as crate::stdlib::uint16_t;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((p * 16i32 + idx * 4i32) as isize)
                        as ::core::ffi::c_int
                        + 8i32) as isize,
                ) as *mut crate::src::common::base::x264_union16_t))
                    .i = (nz * 0x101i32) as crate::stdlib::uint16_t;
                (*h).mb.i_cbp_luma |= nz << idx;
                return;
            }
            (*h).dctf.sub8x8_dct8.expect("non-null function pointer")(
                &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                p_src,
                p_dst,
            );
            nz = x264_quant_8x8(
                h,
                &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                i_qp,
                ctx_cat_plane
                    [crate::src::common::macroblock::DCT_LUMA_8x8 as ::core::ffi::c_int as usize]
                    [p as usize] as ::core::ffi::c_int,
                1i32,
                p,
                idx,
            );
            if nz != 0 {
                (*h).mb.i_cbp_luma |= (1i32) << idx;
                (*h).zigzagf.scan_8x8.expect("non-null function pointer")(
                    &raw mut *(&raw mut (*h).dct.luma8x8
                        as *mut [crate::src::common::common::dctcoef; 64])
                        .offset((p * 4i32 + idx) as isize)
                        as *mut crate::src::common::common::dctcoef,
                    &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                );
                (*h).quantf.dequant_8x8.expect("non-null function pointer")(
                    &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                    (*h).dequant8_mf[(if p != 0 {
                        crate::src::common::set::CQM_8IC as ::core::ffi::c_int
                    } else {
                        crate::src::common::set::CQM_8IY as ::core::ffi::c_int
                    }) as usize],
                    i_qp,
                );
                (*h).dctf.add8x8_idct8.expect("non-null function pointer")(
                    p_dst,
                    &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                );
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((p * 16i32 + idx * 4i32) as isize)
                        as ::core::ffi::c_int
                        + 0i32) as isize,
                ) as *mut crate::src::common::base::x264_union16_t))
                    .i = (1i32 * 0x101i32) as crate::stdlib::uint16_t;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((p * 16i32 + idx * 4i32) as isize)
                        as ::core::ffi::c_int
                        + 8i32) as isize,
                ) as *mut crate::src::common::base::x264_union16_t))
                    .i = (1i32 * 0x101i32) as crate::stdlib::uint16_t;
            } else {
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((p * 16i32 + idx * 4i32) as isize)
                        as ::core::ffi::c_int
                        + 0i32) as isize,
                ) as *mut crate::src::common::base::x264_union16_t))
                    .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((p * 16i32 + idx * 4i32) as isize)
                        as ::core::ffi::c_int
                        + 8i32) as isize,
                ) as *mut crate::src::common::base::x264_union16_t))
                    .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
            };
        }
    }
    use crate::src::encoder::macroblock::base_h::x264_scan8;
    use crate::src::encoder::macroblock::macroblock_h::block_idx_xy_fdec;
    use crate::src::encoder::macroblock::macroblock_h::block_idx_xy_fenc;
    use crate::src::encoder::macroblock::macroblock_h::ctx_cat_plane;
    use crate::src::encoder::macroblock::macroblock_h::x264_pred_i4x4_neighbors;
    use crate::src::encoder::macroblock::x264_8_predict_lossless_4x4;
    use crate::src::encoder::macroblock::x264_8_predict_lossless_8x8;
}
use crate::src::common::base::ChromaFormat;
use crate::src::common::macroblock::{MacroblockType, Partition};
use crate::src::encoder::macroblock::base_h::x264_clip3;
use crate::src::encoder::macroblock::base_h::x264_scan8;
use crate::src::encoder::macroblock::encoder_macroblock_h::x264_mb_encode_i4x4;
use crate::src::encoder::macroblock::encoder_macroblock_h::x264_mb_encode_i8x8;
use crate::src::encoder::macroblock::encoder_macroblock_h::x264_quant_4x4;
use crate::src::encoder::macroblock::encoder_macroblock_h::x264_quant_8x8;
use crate::src::encoder::macroblock::macroblock_h::block_idx_x;
use crate::src::encoder::macroblock::macroblock_h::block_idx_xy_1d;
use crate::src::encoder::macroblock::macroblock_h::block_idx_xy_fdec;
use crate::src::encoder::macroblock::macroblock_h::block_idx_xy_fenc;
use crate::src::encoder::macroblock::macroblock_h::block_idx_y;
use crate::src::encoder::macroblock::macroblock_h::block_idx_yx_1d;
use crate::src::encoder::macroblock::macroblock_h::ctx_cat_plane;
use crate::src::encoder::macroblock::osdep_h::x264_ctz_4bit;
#[inline]
unsafe extern "C" fn zigzag_scan_2x2_dc(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        *level.offset(0isize) = *dct.offset((0i32 * 2i32 + 0i32) as isize);
        *level.offset(1isize) = *dct.offset((1i32 * 2i32 + 0i32) as isize);
        *level.offset(2isize) = *dct.offset((0i32 * 2i32 + 1i32) as isize);
        *level.offset(3isize) = *dct.offset((1i32 * 2i32 + 1i32) as isize);
    }
}
#[inline]
unsafe extern "C" fn zigzag_scan_2x4_dc(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        *level.offset(0isize) = *dct.offset(0isize);
        *level.offset(1isize) = *dct.offset(2isize);
        *level.offset(2isize) = *dct.offset(1isize);
        *level.offset(3isize) = *dct.offset(4isize);
        *level.offset(4isize) = *dct.offset(6isize);
        *level.offset(5isize) = *dct.offset(3isize);
        *level.offset(6isize) = *dct.offset(5isize);
        *level.offset(7isize) = *dct.offset(7isize);
    }
}
#[inline]
unsafe extern "C" fn idct_dequant_2x2_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dct4x4: *mut [crate::src::common::common::dctcoef; 16],
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let mut d0 =
            *dct.offset(0isize) as ::core::ffi::c_int + *dct.offset(1isize) as ::core::ffi::c_int;
        let mut d1 =
            *dct.offset(2isize) as ::core::ffi::c_int + *dct.offset(3isize) as ::core::ffi::c_int;
        let mut d2 =
            *dct.offset(0isize) as ::core::ffi::c_int - *dct.offset(1isize) as ::core::ffi::c_int;
        let mut d3 =
            *dct.offset(2isize) as ::core::ffi::c_int - *dct.offset(3isize) as ::core::ffi::c_int;
        let mut dmf = (*dequant_mf.offset((i_qp % 6i32) as isize))[0usize] << (i_qp / 6i32);
        (*dct4x4.offset(0isize))[0usize] =
            (((d0 + d1) * dmf) >> 5i32) as crate::src::common::common::dctcoef;
        (*dct4x4.offset(1isize))[0usize] =
            (((d0 - d1) * dmf) >> 5i32) as crate::src::common::common::dctcoef;
        (*dct4x4.offset(2isize))[0usize] =
            (((d2 + d3) * dmf) >> 5i32) as crate::src::common::common::dctcoef;
        (*dct4x4.offset(3isize))[0usize] =
            (((d2 - d3) * dmf) >> 5i32) as crate::src::common::common::dctcoef;
    }
}
#[inline]
unsafe extern "C" fn idct_dequant_2x2_dconly(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let mut d0 =
            *dct.offset(0isize) as ::core::ffi::c_int + *dct.offset(1isize) as ::core::ffi::c_int;
        let mut d1 =
            *dct.offset(2isize) as ::core::ffi::c_int + *dct.offset(3isize) as ::core::ffi::c_int;
        let mut d2 =
            *dct.offset(0isize) as ::core::ffi::c_int - *dct.offset(1isize) as ::core::ffi::c_int;
        let mut d3 =
            *dct.offset(2isize) as ::core::ffi::c_int - *dct.offset(3isize) as ::core::ffi::c_int;
        let mut dmf = (*dequant_mf.offset((i_qp % 6i32) as isize))[0usize] << (i_qp / 6i32);
        *dct.offset(0isize) = (((d0 + d1) * dmf) >> 5i32) as crate::src::common::common::dctcoef;
        *dct.offset(1isize) = (((d0 - d1) * dmf) >> 5i32) as crate::src::common::common::dctcoef;
        *dct.offset(2isize) = (((d2 + d3) * dmf) >> 5i32) as crate::src::common::common::dctcoef;
        *dct.offset(3isize) = (((d2 - d3) * dmf) >> 5i32) as crate::src::common::common::dctcoef;
    }
}
#[inline]
unsafe extern "C" fn dct2x2dc(
    mut d: *mut crate::src::common::common::dctcoef,
    mut dct4x4: *mut [crate::src::common::common::dctcoef; 16],
) {
    unsafe {
        let mut d0 = (*dct4x4.offset(0isize))[0usize] as ::core::ffi::c_int
            + (*dct4x4.offset(1isize))[0usize] as ::core::ffi::c_int;
        let mut d1 = (*dct4x4.offset(2isize))[0usize] as ::core::ffi::c_int
            + (*dct4x4.offset(3isize))[0usize] as ::core::ffi::c_int;
        let mut d2 = (*dct4x4.offset(0isize))[0usize] as ::core::ffi::c_int
            - (*dct4x4.offset(1isize))[0usize] as ::core::ffi::c_int;
        let mut d3 = (*dct4x4.offset(2isize))[0usize] as ::core::ffi::c_int
            - (*dct4x4.offset(3isize))[0usize] as ::core::ffi::c_int;
        *d.offset(0isize) = (d0 + d1) as crate::src::common::common::dctcoef;
        *d.offset(2isize) = (d2 + d3) as crate::src::common::common::dctcoef;
        *d.offset(1isize) = (d0 - d1) as crate::src::common::common::dctcoef;
        *d.offset(3isize) = (d2 - d3) as crate::src::common::common::dctcoef;
        (*dct4x4.offset(0isize))[0usize] = 0i16;
        (*dct4x4.offset(1isize))[0usize] = 0i16;
        (*dct4x4.offset(2isize))[0usize] = 0i16;
        (*dct4x4.offset(3isize))[0usize] = 0i16;
    }
}
#[inline(always)]
unsafe extern "C" fn array_non_zero(
    mut v: *mut crate::src::common::common::dctcoef,
    mut i_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if crate::osdep_h::WORD_SIZE == 8i32 {
            let mut i = 0i32;
            while i < i_count {
                if (*(v.offset(i as isize) as *mut crate::src::common::base::x264_union64_t)).i != 0
                {
                    return 1i32;
                }
                i = (i as ::core::ffi::c_ulong).wrapping_add(
                    (8usize)
                        .wrapping_div(::core::mem::size_of::<crate::src::common::common::dctcoef>())
                        as ::core::ffi::c_ulong,
                ) as ::core::ffi::c_int;
            }
        } else {
            let mut i_0 = 0i32;
            while i_0 < i_count {
                if (*(v.offset(i_0 as isize) as *mut crate::src::common::base::x264_union32_t)).i
                    != 0
                {
                    return 1i32;
                }
                i_0 = (i_0 as ::core::ffi::c_ulong).wrapping_add(
                    (4usize)
                        .wrapping_div(::core::mem::size_of::<crate::src::common::common::dctcoef>())
                        as ::core::ffi::c_ulong,
                ) as ::core::ffi::c_int;
            }
        }
        0i32
    }
}
unsafe extern "C" fn mb_encode_i16x16(
    mut h: *mut crate::src::common::common::x264_t,
    mut p: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let mut dct4x4 = [[0; 16]; 16];
        let mut dct_dc4x4 = [0; 16];
        let mut nz = 0;
        let mut block_cbp = 0i32;
        let mut idx_0 = 0i32;
        let mut p_src = (*h).mb.pic.p_fenc[p as usize];
        let mut p_dst = (*h).mb.pic.p_fdec[p as usize];
        let mut decimate_score = if (*h).mb.dct_decimate { 0i32 } else { 9i32 };
        let mut i_quant_cat = if p != 0 {
            crate::src::common::set::CQM_4IC as ::core::ffi::c_int
        } else {
            crate::src::common::set::CQM_4IY as ::core::ffi::c_int
        };
        let mut i_mode = (*h).mb.i_intra16x16_pred_mode;
        if (*h).mb.lossless {
            x264_8_predict_lossless_16x16(h, p, i_mode);
        } else {
            (*h).predict_16x16[i_mode as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[p as usize],
            );
        }
        if (*h).mb.lossless {
            let mut i = 0i32;
            while i < 16i32 {
                let mut oe = block_idx_xy_fenc[i as usize] as ::core::ffi::c_int;
                let mut od = block_idx_xy_fdec[i as usize] as ::core::ffi::c_int;
                nz = (*h).zigzagf.sub_4x4ac.expect("non-null function pointer")(
                    &raw mut *(&raw mut (*h).dct.luma4x4
                        as *mut [crate::src::common::common::dctcoef; 16])
                        .offset((16i32 * p + i) as isize)
                        as *mut crate::src::common::common::dctcoef,
                    p_src.offset(oe as isize),
                    p_dst.offset(od as isize),
                    (&raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef).offset(
                        *(&raw const block_idx_yx_1d as *const crate::stdlib::uint8_t)
                            .offset(i as isize) as isize,
                    ),
                );
                (*h).mb.cache.non_zero_count[x264_scan8[(16i32 * p + i) as usize] as usize] =
                    nz as crate::stdlib::uint8_t;
                block_cbp |= nz;
                i += 1;
            }
            (*h).mb.i_cbp_luma |= block_cbp * 0xfi32;
            (*h).mb.cache.non_zero_count
                [x264_scan8[(crate::src::common::base::LUMA_DC + p) as usize] as usize] =
                array_non_zero(
                    &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
                    16i32,
                ) as crate::stdlib::uint8_t;
            (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                &raw mut *(&raw mut (*h).dct.luma16x16_dc
                    as *mut [crate::src::common::common::dctcoef; 16])
                    .offset(p as isize) as *mut crate::src::common::common::dctcoef,
                &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
            );
            return;
        }
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset((16i32 * p) as isize)
                as ::core::ffi::c_int
                + 0i32 * 8i32) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset((16i32 * p) as isize)
                as ::core::ffi::c_int
                + 1i32 * 8i32) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset((16i32 * p) as isize)
                as ::core::ffi::c_int
                + 2i32 * 8i32) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset((16i32 * p) as isize)
                as ::core::ffi::c_int
                + 3i32 * 8i32) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*h).dctf.sub16x16_dct.expect("non-null function pointer")(
            &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
            p_src,
            p_dst,
        );
        if (*h).mb.noise_reduction {
            let mut idx = 0i32;
            while idx < 16i32 {
                (*h).quantf.denoise_dct.expect("non-null function pointer")(
                    &raw mut *(&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                        .offset(idx as isize)
                        as *mut crate::src::common::common::dctcoef,
                    &raw mut *(*h).nr_residual_sum.offset(0isize) as *mut crate::stdlib::uint32_t,
                    &raw mut *(*h).nr_offset.offset(0isize)
                        as *mut crate::src::common::common::udctcoef,
                    16i32,
                );
                idx += 1;
            }
        }
        while idx_0 < 16i32 {
            dct_dc4x4[block_idx_xy_1d[idx_0 as usize] as usize] = dct4x4[idx_0 as usize][0usize];
            dct4x4[idx_0 as usize][0usize] = 0i16;
            idx_0 += 1;
        }
        if (*h).mb.trellis {
            let mut idx_1 = 0i32;
            while idx_1 < 16i32 {
                if crate::src::encoder::analyse::rdo_c::x264_8_quant_4x4_trellis(
                    h,
                    &raw mut *(&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                        .offset(idx_1 as isize)
                        as *mut crate::src::common::common::dctcoef,
                    i_quant_cat,
                    i_qp,
                    ctx_cat_plane
                        [crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int as usize]
                        [p as usize] as ::core::ffi::c_int,
                    1i32,
                    (p != 0) as ::core::ffi::c_int,
                    idx_1,
                ) != 0
                {
                    block_cbp = 0xfi32;
                    (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                        &raw mut *(&raw mut (*h).dct.luma4x4
                            as *mut [crate::src::common::common::dctcoef; 16])
                            .offset((16i32 * p + idx_1) as isize)
                            as *mut crate::src::common::common::dctcoef,
                        &raw mut *(&raw mut dct4x4
                            as *mut [crate::src::common::common::dctcoef; 16])
                            .offset(idx_1 as isize)
                            as *mut crate::src::common::common::dctcoef,
                    );
                    (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                        &raw mut *(&raw mut dct4x4
                            as *mut [crate::src::common::common::dctcoef; 16])
                            .offset(idx_1 as isize)
                            as *mut crate::src::common::common::dctcoef,
                        (*h).dequant4_mf[i_quant_cat as usize],
                        i_qp,
                    );
                    if decimate_score < 6i32 {
                        decimate_score += (*h)
                            .quantf
                            .decimate_score15
                            .expect("non-null function pointer")(
                            &raw mut *(&raw mut (*h).dct.luma4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset((16i32 * p + idx_1) as isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    }
                    (*h).mb.cache.non_zero_count
                        [x264_scan8[(16i32 * p + idx_1) as usize] as usize] = 1u8;
                }
                idx_1 += 1;
            }
        } else {
            let mut i8x8 = 0i32;
            while i8x8 < 4i32 {
                nz = (*h).quantf.quant_4x4x4.expect("non-null function pointer")(
                    (&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                        .offset((i8x8 * 4i32) as isize),
                    &raw mut *(*(&raw mut (*h).quant4_mf
                        as *mut *mut [crate::src::common::common::udctcoef; 16])
                        .offset(i_quant_cat as isize))
                    .offset(i_qp as isize)
                        as *mut crate::src::common::common::udctcoef,
                    &raw mut *(*(&raw mut (*h).quant4_bias
                        as *mut *mut [crate::src::common::common::udctcoef; 16])
                        .offset(i_quant_cat as isize))
                    .offset(i_qp as isize)
                        as *mut crate::src::common::common::udctcoef,
                );
                if nz != 0 {
                    block_cbp = 0xfi32;
                    let mut idx_2 = i8x8 * 4i32;
                    let mut msk = nz;
                    while msk != 0 && {
                        let mut skip = 0;
                        skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                        idx_2 += skip;
                        msk >>= skip + 1i32;
                        1i32 != 0
                    } {
                        (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                            &raw mut *(&raw mut (*h).dct.luma4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset((16i32 * p + idx_2) as isize)
                                as *mut crate::src::common::common::dctcoef,
                            &raw mut *(&raw mut dct4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset(idx_2 as isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                        (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                            &raw mut *(&raw mut dct4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset(idx_2 as isize)
                                as *mut crate::src::common::common::dctcoef,
                            (*h).dequant4_mf[i_quant_cat as usize],
                            i_qp,
                        );
                        if decimate_score < 6i32 {
                            decimate_score += (*h)
                                .quantf
                                .decimate_score15
                                .expect("non-null function pointer")(
                                &raw mut *(&raw mut (*h).dct.luma4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((16i32 * p + idx_2) as isize)
                                    as *mut crate::src::common::common::dctcoef,
                            );
                        }
                        (*h).mb.cache.non_zero_count
                            [x264_scan8[(16i32 * p + idx_2) as usize] as usize] = 1u8;
                        idx_2 += 1;
                    }
                }
                i8x8 += 1;
            }
        }
        if decimate_score < 6i32 {
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset((16i32 * p) as isize) as ::core::ffi::c_int
                    + 0i32 * 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0u32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset((16i32 * p) as isize) as ::core::ffi::c_int
                    + 1i32 * 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0u32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset((16i32 * p) as isize) as ::core::ffi::c_int
                    + 2i32 * 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0u32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset((16i32 * p) as isize) as ::core::ffi::c_int
                    + 3i32 * 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0u32;
            block_cbp = 0i32;
        } else {
            (*h).mb.i_cbp_luma |= block_cbp;
        }
        (*h).dctf.dct4x4dc.expect("non-null function pointer")(
            &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
        );
        if (*h).mb.trellis {
            nz = crate::src::encoder::analyse::rdo_c::x264_8_quant_luma_dc_trellis(
                h,
                &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
                i_quant_cat,
                i_qp,
                ctx_cat_plane
                    [crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int as usize]
                    [p as usize] as ::core::ffi::c_int,
                1i32,
                crate::src::common::base::LUMA_DC + p,
            );
        } else {
            nz = (*h).quantf.quant_4x4_dc.expect("non-null function pointer")(
                &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
                (*(*h).quant4_mf[i_quant_cat as usize].offset(i_qp as isize))[0usize]
                    as ::core::ffi::c_int
                    >> 1i32,
                ((*(*h).quant4_bias[i_quant_cat as usize].offset(i_qp as isize))[0usize]
                    as ::core::ffi::c_int)
                    << 1i32,
            );
        }
        (*h).mb.cache.non_zero_count
            [x264_scan8[(crate::src::common::base::LUMA_DC + p) as usize] as usize] =
            nz as crate::stdlib::uint8_t;
        if nz != 0 {
            (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                &raw mut *(&raw mut (*h).dct.luma16x16_dc
                    as *mut [crate::src::common::common::dctcoef; 16])
                    .offset(p as isize) as *mut crate::src::common::common::dctcoef,
                &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
            );
            (*h).dctf.idct4x4dc.expect("non-null function pointer")(
                &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
            );
            (*h).quantf
                .dequant_4x4_dc
                .expect("non-null function pointer")(
                &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
                (*h).dequant4_mf[i_quant_cat as usize],
                i_qp,
            );
            if block_cbp != 0 {
                let mut i_0 = 0i32;
                while i_0 < 16i32 {
                    dct4x4[i_0 as usize][0usize] =
                        dct_dc4x4[block_idx_xy_1d[i_0 as usize] as usize];
                    i_0 += 1;
                }
            }
        }
        if block_cbp != 0 {
            (*h).dctf.add16x16_idct.expect("non-null function pointer")(
                p_dst,
                &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
            );
        } else if nz != 0 {
            (*h).dctf
                .add16x16_idct_dc
                .expect("non-null function pointer")(
                p_dst,
                &raw mut dct_dc4x4 as *mut crate::src::common::common::dctcoef,
            );
        }
    }
}
#[inline(always)]
unsafe extern "C" fn mb_optimize_chroma_dc(
    mut h: *mut crate::src::common::common::x264_t,
    mut dct_dc: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
    mut chroma422: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dmf = (*dequant_mf.offset((i_qp % 6i32) as isize))[0usize] << (i_qp / 6i32);
        if dmf > 32i32 * 64i32 {
            return 1i32;
        }
        if chroma422 != 0 {
            (*h).quantf
                .optimize_chroma_2x4_dc
                .expect("non-null function pointer")(dct_dc, dmf)
        } else {
            (*h).quantf
                .optimize_chroma_2x2_dc
                .expect("non-null function pointer")(dct_dc, dmf)
        }
    }
}
#[inline(always)]
unsafe extern "C" fn mb_encode_chroma_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_inter: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
    mut chroma422: ::core::ffi::c_int,
) {
    unsafe {
        let mut nz_dc = 0;
        let mut dct_dc = [0; 8];
        let mut ch_0 = 0i32;
        let mut b_decimate = b_inter != 0 && (*h).mb.dct_decimate;
        let mut dequant_mf = (*h).dequant4_mf
            [(crate::src::common::set::CQM_4IC as ::core::ffi::c_int + b_inter) as usize];
        (*h).mb.i_cbp_chroma = 0i32;
        let ref mut c2rust_fresh2 = *(*h).nr_count.offset(2isize);
        *c2rust_fresh2 = (*c2rust_fresh2).wrapping_add(
            ((*h).mb.noise_reduction as ::core::ffi::c_int * 4i32) as crate::stdlib::uint32_t,
        );
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(16isize) as isize,
        ) as *mut crate::src::common::base::x264_union16_t))
            .i = 0u16;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(18isize) as isize,
        ) as *mut crate::src::common::base::x264_union16_t))
            .i = 0u16;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(32isize) as isize,
        ) as *mut crate::src::common::base::x264_union16_t))
            .i = 0u16;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(34isize) as isize,
        ) as *mut crate::src::common::base::x264_union16_t))
            .i = 0u16;
        if chroma422 != 0 {
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(24isize) as isize,
            ) as *mut crate::src::common::base::x264_union16_t))
                .i = 0u16;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(26isize) as isize,
            ) as *mut crate::src::common::base::x264_union16_t))
                .i = 0u16;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(40isize) as isize,
            ) as *mut crate::src::common::base::x264_union16_t))
                .i = 0u16;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(42isize) as isize,
            ) as *mut crate::src::common::base::x264_union16_t))
                .i = 0u16;
        }
        if b_decimate
            && i_qp >= (if (*h).mb.trellis { 12i32 } else { 18i32 })
            && !(*h).mb.noise_reduction
        {
            let mut ssd = [0; 2];
            let mut thresh = if chroma422 != 0 {
                (crate::src::common::tables::x264_lambda2_tab[i_qp as usize] + 16i32) >> 5i32
            } else {
                (crate::src::common::tables::x264_lambda2_tab[i_qp as usize] + 32i32) >> 6i32
            };
            let mut chromapix = if chroma422 != 0 {
                crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int
            } else {
                crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
            };
            if (*h).pixf.var2[chromapix as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[1usize],
                (*h).mb.pic.p_fdec[1usize],
                &raw mut ssd as *mut ::core::ffi::c_int,
            ) < thresh * 4i32
            {
                let mut ch = 0i32;
                (*h).mb.cache.non_zero_count
                    [x264_scan8[(crate::src::common::base::CHROMA_DC + 0i32) as usize] as usize] =
                    0u8;
                (*h).mb.cache.non_zero_count
                    [x264_scan8[(crate::src::common::base::CHROMA_DC + 1i32) as usize] as usize] =
                    0u8;
                while ch < 2i32 {
                    if ssd[ch as usize] > thresh {
                        let mut p_src = (*h).mb.pic.p_fenc[(1i32 + ch) as usize];
                        let mut p_dst = (*h).mb.pic.p_fdec[(1i32 + ch) as usize];
                        if chroma422 != 0 {
                            (*h).dctf.sub8x16_dct_dc.expect("non-null function pointer")(
                                &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                p_src,
                                p_dst,
                            );
                        } else {
                            (*h).dctf.sub8x8_dct_dc.expect("non-null function pointer")(
                                &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                p_src,
                                p_dst,
                            );
                        }
                        if (*h).mb.trellis {
                            nz_dc =
                                crate::src::encoder::analyse::rdo_c::x264_8_quant_chroma_dc_trellis(
                                    h,
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                    i_qp + 3i32 * chroma422,
                                    (b_inter == 0) as ::core::ffi::c_int,
                                    crate::src::common::base::CHROMA_DC + ch,
                                );
                        } else {
                            let mut i = 0i32;
                            nz_dc = 0i32;
                            while i <= chroma422 {
                                nz_dc |=
                                    (*h).quantf.quant_2x2_dc.expect("non-null function pointer")(
                                        (&raw mut dct_dc
                                            as *mut crate::src::common::common::dctcoef)
                                            .offset((4i32 * i) as isize),
                                        (*(*h).quant4_mf[(crate::src::common::set::CQM_4IC
                                            as ::core::ffi::c_int
                                            + b_inter)
                                            as usize]
                                            .offset((i_qp + 3i32 * chroma422) as isize))[0usize]
                                            as ::core::ffi::c_int
                                            >> 1i32,
                                        ((*(*h).quant4_bias[(crate::src::common::set::CQM_4IC
                                            as ::core::ffi::c_int
                                            + b_inter)
                                            as usize]
                                            .offset((i_qp + 3i32 * chroma422) as isize))[0usize]
                                            as ::core::ffi::c_int)
                                            << 1i32,
                                    );
                                i += 1;
                            }
                        }
                        if nz_dc != 0
                            && (mb_optimize_chroma_dc(
                                h,
                                &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                dequant_mf,
                                i_qp + 3i32 * chroma422,
                                chroma422,
                            ) != 0)
                        {
                            let mut i_0 = 0i32;
                            (*h).mb.cache.non_zero_count[x264_scan8
                                [(crate::src::common::base::CHROMA_DC + ch) as usize]
                                as usize] = 1u8;
                            if chroma422 != 0 {
                                zigzag_scan_2x4_dc(
                                    &raw mut *(&raw mut (*h).dct.chroma_dc
                                        as *mut [crate::src::common::common::dctcoef; 8])
                                        .offset(ch as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                );
                                (*h).quantf
                                    .idct_dequant_2x4_dconly
                                    .expect("non-null function pointer")(
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                    dequant_mf,
                                    i_qp + 3i32,
                                );
                            } else {
                                zigzag_scan_2x2_dc(
                                    &raw mut *(&raw mut (*h).dct.chroma_dc
                                        as *mut [crate::src::common::common::dctcoef; 8])
                                        .offset(ch as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                );
                                idct_dequant_2x2_dconly(
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                    dequant_mf,
                                    i_qp,
                                );
                            }
                            while i_0 <= chroma422 {
                                (*h).dctf.add8x8_idct_dc.expect("non-null function pointer")(
                                    p_dst.offset(
                                        (8i32 * i_0 * crate::src::common::common::FDEC_STRIDE)
                                            as isize,
                                    ),
                                    (&raw mut dct_dc as *mut crate::src::common::common::dctcoef)
                                        .offset((4i32 * i_0) as isize),
                                );
                                i_0 += 1;
                            }
                            (*h).mb.i_cbp_chroma = 1i32;
                        }
                    }
                    ch += 1;
                }
                return;
            }
        }
        while ch_0 < 2i32 {
            let mut nz = 0;
            let mut p_src_0 = (*h).mb.pic.p_fenc[(1i32 + ch_0) as usize];
            let mut p_dst_0 = (*h).mb.pic.p_fdec[(1i32 + ch_0) as usize];
            let mut i_decimate_score = if b_decimate { 0i32 } else { 7i32 };
            if (*h).mb.lossless {
                let mut i_1 = 0i32;
                static mut chroma422_scan: [crate::stdlib::uint8_t; 8] =
                    [0u8, 2u8, 1u8, 5u8, 3u8, 6u8, 4u8, 7u8];
                while i_1 < (if chroma422 != 0 { 8i32 } else { 4i32 }) {
                    let mut oe = 4i32 * (i_1 & 1i32)
                        + 4i32 * (i_1 >> 1i32) * crate::src::common::common::FENC_STRIDE;
                    let mut od = 4i32 * (i_1 & 1i32)
                        + 4i32 * (i_1 >> 1i32) * crate::src::common::common::FDEC_STRIDE;
                    nz = (*h).zigzagf.sub_4x4ac.expect("non-null function pointer")(
                        &raw mut *(&raw mut (*h).dct.luma4x4
                            as *mut [crate::src::common::common::dctcoef; 16])
                            .offset(
                                (16i32
                                    + i_1
                                    + (if chroma422 != 0 { i_1 & 4i32 } else { 0i32 })
                                    + ch_0 * 16i32) as isize,
                            ) as *mut crate::src::common::common::dctcoef,
                        p_src_0.offset(oe as isize),
                        p_dst_0.offset(od as isize),
                        (&raw mut *(&raw mut (*h).dct.chroma_dc
                            as *mut [crate::src::common::common::dctcoef; 8])
                            .offset(ch_0 as isize)
                            as *mut crate::src::common::common::dctcoef)
                            .offset(
                                (if chroma422 != 0 {
                                    *(&raw const chroma422_scan as *const crate::stdlib::uint8_t)
                                        .offset(i_1 as isize)
                                        as ::core::ffi::c_int
                                } else {
                                    i_1
                                }) as isize,
                            ),
                    );
                    (*h).mb.cache.non_zero_count[x264_scan8[(16i32
                        + i_1
                        + (if chroma422 != 0 { i_1 & 4i32 } else { 0i32 })
                        + ch_0 * 16i32)
                        as usize] as usize] = nz as crate::stdlib::uint8_t;
                    (*h).mb.i_cbp_chroma |= nz;
                    i_1 += 1;
                }
                (*h).mb.cache.non_zero_count
                    [x264_scan8[(crate::src::common::base::CHROMA_DC + ch_0) as usize] as usize] =
                    array_non_zero(
                        &raw mut *(&raw mut (*h).dct.chroma_dc
                            as *mut [crate::src::common::common::dctcoef; 8])
                            .offset(ch_0 as isize)
                            as *mut crate::src::common::common::dctcoef,
                        if chroma422 != 0 { 8i32 } else { 4i32 },
                    ) as crate::stdlib::uint8_t;
            } else {
                let mut nz_ac = 0i32;
                let mut dct4x4 = [[0; 16]; 8];
                let mut i_2 = 0i32;
                let mut i8x8 = 0i32;
                while i_2 <= chroma422 {
                    (*h).dctf.sub8x8_dct.expect("non-null function pointer")(
                        (&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                            .offset((4i32 * i_2) as isize),
                        p_src_0.offset(
                            (8i32 * i_2 * crate::src::common::common::FENC_STRIDE) as isize,
                        ),
                        p_dst_0.offset(
                            (8i32 * i_2 * crate::src::common::common::FDEC_STRIDE) as isize,
                        ),
                    );
                    i_2 += 1;
                }
                if (*h).mb.noise_reduction {
                    let mut i_3 = 0i32;
                    while i_3 < (if chroma422 != 0 { 8i32 } else { 4i32 }) {
                        (*h).quantf.denoise_dct.expect("non-null function pointer")(
                            &raw mut *(&raw mut dct4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset(i_3 as isize)
                                as *mut crate::src::common::common::dctcoef,
                            &raw mut *(*h).nr_residual_sum.offset(2isize)
                                as *mut crate::stdlib::uint32_t,
                            &raw mut *(*h).nr_offset.offset(2isize)
                                as *mut crate::src::common::common::udctcoef,
                            16i32,
                        );
                        i_3 += 1;
                    }
                }
                if chroma422 != 0 {
                    (*h).dctf.dct2x4dc.expect("non-null function pointer")(
                        &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                        &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                    );
                } else {
                    dct2x2dc(
                        &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                        &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                    );
                }
                while i8x8 < (if chroma422 != 0 { 2i32 } else { 1i32 }) {
                    if (*h).mb.trellis {
                        let mut i4x4 = 0i32;
                        while i4x4 < 4i32 {
                            if crate::src::encoder::analyse::rdo_c::x264_8_quant_4x4_trellis(
                                h,
                                &raw mut *(&raw mut dct4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((i8x8 * 4i32 + i4x4) as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                crate::src::common::set::CQM_4IC as ::core::ffi::c_int + b_inter,
                                i_qp,
                                crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int,
                                (b_inter == 0) as ::core::ffi::c_int,
                                1i32,
                                0i32,
                            ) != 0
                            {
                                let mut idx = 16i32 + ch_0 * 16i32 + i8x8 * 8i32 + i4x4;
                                (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                                    &raw mut *(&raw mut (*h).dct.luma4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(idx as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    &raw mut *(&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset((i8x8 * 4i32 + i4x4) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                                (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                                    &raw mut *(&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset((i8x8 * 4i32 + i4x4) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    dequant_mf,
                                    i_qp,
                                );
                                if i_decimate_score < 7i32 {
                                    i_decimate_score += (*h)
                                        .quantf
                                        .decimate_score15
                                        .expect("non-null function pointer")(
                                        &raw mut *(&raw mut (*h).dct.luma4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset(idx as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                    );
                                }
                                (*h).mb.cache.non_zero_count[x264_scan8[idx as usize] as usize] =
                                    1u8;
                                nz_ac = 1i32;
                            }
                            i4x4 += 1;
                        }
                    } else {
                        let mut i4x4_0 = 0i32;
                        nz = (*h).quantf.quant_4x4x4.expect("non-null function pointer")(
                            (&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                                .offset((i8x8 * 4i32) as isize),
                            &raw mut *(*(&raw mut (*h).quant4_mf
                                as *mut *mut [crate::src::common::common::udctcoef; 16])
                                .offset(
                                    (crate::src::common::set::CQM_4IC as ::core::ffi::c_int
                                        + b_inter) as isize,
                                ))
                            .offset(i_qp as isize)
                                as *mut crate::src::common::common::udctcoef,
                            &raw mut *(*(&raw mut (*h).quant4_bias
                                as *mut *mut [crate::src::common::common::udctcoef; 16])
                                .offset(
                                    (crate::src::common::set::CQM_4IC as ::core::ffi::c_int
                                        + b_inter) as isize,
                                ))
                            .offset(i_qp as isize)
                                as *mut crate::src::common::common::udctcoef,
                        );
                        nz_ac |= nz;
                        let mut msk = nz;
                        while msk != 0 && {
                            let mut skip = 0;
                            skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                            i4x4_0 += skip;
                            msk >>= skip + 1i32;
                            1i32 != 0
                        } {
                            let mut idx_0 = 16i32 + ch_0 * 16i32 + i8x8 * 8i32 + i4x4_0;
                            (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                                &raw mut *(&raw mut (*h).dct.luma4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(idx_0 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut *(&raw mut dct4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((i8x8 * 4i32 + i4x4_0) as isize)
                                    as *mut crate::src::common::common::dctcoef,
                            );
                            (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                                &raw mut *(&raw mut dct4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((i8x8 * 4i32 + i4x4_0) as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                dequant_mf,
                                i_qp,
                            );
                            if i_decimate_score < 7i32 {
                                i_decimate_score += (*h)
                                    .quantf
                                    .decimate_score15
                                    .expect("non-null function pointer")(
                                    &raw mut *(&raw mut (*h).dct.luma4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(idx_0 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                            }
                            (*h).mb.cache.non_zero_count[x264_scan8[idx_0 as usize] as usize] = 1u8;
                            i4x4_0 += 1;
                        }
                    }
                    i8x8 += 1;
                }
                if (*h).mb.trellis {
                    nz_dc = crate::src::encoder::analyse::rdo_c::x264_8_quant_chroma_dc_trellis(
                        h,
                        &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                        i_qp + 3i32 * chroma422,
                        (b_inter == 0) as ::core::ffi::c_int,
                        crate::src::common::base::CHROMA_DC + ch_0,
                    );
                } else {
                    let mut i_4 = 0i32;
                    nz_dc = 0i32;
                    while i_4 <= chroma422 {
                        nz_dc |= (*h).quantf.quant_2x2_dc.expect("non-null function pointer")(
                            (&raw mut dct_dc as *mut crate::src::common::common::dctcoef)
                                .offset((4i32 * i_4) as isize),
                            (*(*h).quant4_mf[(crate::src::common::set::CQM_4IC
                                as ::core::ffi::c_int
                                + b_inter) as usize]
                                .offset((i_qp + 3i32 * chroma422) as isize))[0usize]
                                as ::core::ffi::c_int
                                >> 1i32,
                            ((*(*h).quant4_bias[(crate::src::common::set::CQM_4IC
                                as ::core::ffi::c_int
                                + b_inter)
                                as usize]
                                .offset((i_qp + 3i32 * chroma422) as isize))[0usize]
                                as ::core::ffi::c_int)
                                << 1i32,
                        );
                        i_4 += 1;
                    }
                }
                (*h).mb.cache.non_zero_count
                    [x264_scan8[(crate::src::common::base::CHROMA_DC + ch_0) as usize] as usize] =
                    nz_dc as crate::stdlib::uint8_t;
                if i_decimate_score < 7i32 || nz_ac == 0 {
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 + 16i32 * ch_0) as isize)
                                as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0u16;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((18i32 + 16i32 * ch_0) as isize)
                                as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0u16;
                    if chroma422 != 0 {
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((24i32 + 16i32 * ch_0) as isize)
                                    as isize,
                            )
                            as *mut crate::src::common::base::x264_union16_t))
                            .i = 0u16;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((26i32 + 16i32 * ch_0) as isize)
                                    as isize,
                            )
                            as *mut crate::src::common::base::x264_union16_t))
                            .i = 0u16;
                    }
                    if nz_dc != 0 {
                        if mb_optimize_chroma_dc(
                            h,
                            &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                            dequant_mf,
                            i_qp + 3i32 * chroma422,
                            chroma422,
                        ) == 0
                        {
                            (*h).mb.cache.non_zero_count[x264_scan8
                                [(crate::src::common::base::CHROMA_DC + ch_0) as usize]
                                as usize] = 0u8;
                        } else {
                            let mut i_5 = 0i32;
                            if chroma422 != 0 {
                                zigzag_scan_2x4_dc(
                                    &raw mut *(&raw mut (*h).dct.chroma_dc
                                        as *mut [crate::src::common::common::dctcoef; 8])
                                        .offset(ch_0 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                );
                                (*h).quantf
                                    .idct_dequant_2x4_dconly
                                    .expect("non-null function pointer")(
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                    dequant_mf,
                                    i_qp + 3i32,
                                );
                            } else {
                                zigzag_scan_2x2_dc(
                                    &raw mut *(&raw mut (*h).dct.chroma_dc
                                        as *mut [crate::src::common::common::dctcoef; 8])
                                        .offset(ch_0 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                );
                                idct_dequant_2x2_dconly(
                                    &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                    dequant_mf,
                                    i_qp,
                                );
                            }
                            while i_5 <= chroma422 {
                                (*h).dctf.add8x8_idct_dc.expect("non-null function pointer")(
                                    p_dst_0.offset(
                                        (8i32 * i_5 * crate::src::common::common::FDEC_STRIDE)
                                            as isize,
                                    ),
                                    (&raw mut dct_dc as *mut crate::src::common::common::dctcoef)
                                        .offset((4i32 * i_5) as isize),
                                );
                                i_5 += 1;
                            }
                        }
                    }
                } else {
                    let mut i_6 = 0i32;
                    (*h).mb.i_cbp_chroma = 1i32;
                    if nz_dc != 0 {
                        if chroma422 != 0 {
                            zigzag_scan_2x4_dc(
                                &raw mut *(&raw mut (*h).dct.chroma_dc
                                    as *mut [crate::src::common::common::dctcoef; 8])
                                    .offset(ch_0 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                            );
                            (*h).quantf
                                .idct_dequant_2x4_dc
                                .expect("non-null function pointer")(
                                &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                                dequant_mf,
                                i_qp + 3i32,
                            );
                        } else {
                            zigzag_scan_2x2_dc(
                                &raw mut *(&raw mut (*h).dct.chroma_dc
                                    as *mut [crate::src::common::common::dctcoef; 8])
                                    .offset(ch_0 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                            );
                            idct_dequant_2x2_dc(
                                &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                                &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                                dequant_mf,
                                i_qp,
                            );
                        }
                    }
                    while i_6 <= chroma422 {
                        (*h).dctf.add8x8_idct.expect("non-null function pointer")(
                            p_dst_0.offset(
                                (8i32 * i_6 * crate::src::common::common::FDEC_STRIDE) as isize,
                            ),
                            (&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                                .offset((4i32 * i_6) as isize),
                        );
                        i_6 += 1;
                    }
                }
            }
            ch_0 += 1;
        }
        (*h).mb.i_cbp_chroma += (*h).mb.cache.non_zero_count
            [x264_scan8[(crate::src::common::base::CHROMA_DC + 0i32) as usize] as usize]
            as ::core::ffi::c_int
            | (*h).mb.cache.non_zero_count
                [x264_scan8[(crate::src::common::base::CHROMA_DC + 1i32) as usize] as usize]
                as ::core::ffi::c_int
            | (*h).mb.i_cbp_chroma;
    }
}
pub unsafe extern "C" fn x264_8_mb_encode_chroma(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_inter: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        if (*h).sps.i_chroma_format_idc.is_420() {
            mb_encode_chroma_internal(h, b_inter, i_qp, 0i32);
        } else {
            mb_encode_chroma_internal(h, b_inter, i_qp, 1i32);
        };
    }
}
unsafe extern "C" fn macroblock_encode_skip(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(2isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(8isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(10isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((16i32 + 0i32) as isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((16i32 + 2i32) as isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((32i32 + 0i32) as isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((32i32 + 2i32) as isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i = 0u32;
        if (*h).sps.i_chroma_format_idc.is_422() || (*h).sps.i_chroma_format_idc.is_444() {
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset((16i32 + 8i32) as isize) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0u32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset((16i32 + 10i32) as isize) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0u32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset((32i32 + 8i32) as isize) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0u32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset((32i32 + 10i32) as isize) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0u32;
        }
        (*h).mb.i_cbp_luma = 0i32;
        (*h).mb.i_cbp_chroma = 0i32;
        *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) = 0i16;
    }
}
pub unsafe extern "C" fn x264_8_predict_lossless_chroma(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_mode: ::core::ffi::c_int,
) {
    unsafe {
        let mut height = 16i32 >> ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
        if i_mode == crate::src::common::predict::I_PRED_CHROMA_V as ::core::ffi::c_int {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[1usize],
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fenc[1usize]
                    .offset(-(crate::src::common::common::FENC_STRIDE as isize)),
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                height,
            );
            (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[2usize],
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fenc[2usize]
                    .offset(-(crate::src::common::common::FENC_STRIDE as isize)),
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                height,
            );
            crate::stdlib::memcpy(
                (*h).mb.pic.p_fdec[1usize] as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[1usize]
                    .offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *const ::core::ffi::c_void,
                (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                (*h).mb.pic.p_fdec[2usize] as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[2usize]
                    .offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *const ::core::ffi::c_void,
                (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
        } else if i_mode == crate::src::common::predict::I_PRED_CHROMA_H as ::core::ffi::c_int {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[1usize],
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fenc[1usize].offset(-(1isize)),
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                height,
            );
            (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[2usize],
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fenc[2usize].offset(-(1isize)),
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                height,
            );
            crate::src::common::macroblock::x264_8_copy_column8(
                (*h).mb.pic.p_fdec[1usize]
                    .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[1usize]
                    .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    .offset(-(1isize)),
            );
            crate::src::common::macroblock::x264_8_copy_column8(
                (*h).mb.pic.p_fdec[2usize]
                    .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[2usize]
                    .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    .offset(-(1isize)),
            );
            if (*h).sps.i_chroma_format_idc.is_422() {
                crate::src::common::macroblock::x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[1usize]
                        .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[1usize]
                        .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                        .offset(-(1isize)),
                );
                crate::src::common::macroblock::x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[2usize]
                        .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[2usize]
                        .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                        .offset(-(1isize)),
                );
            }
        } else {
            (*h).predict_chroma[i_mode as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[1usize],
            );
            (*h).predict_chroma[i_mode as usize].expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[2usize],
            );
        };
    }
}
pub unsafe extern "C" fn x264_8_predict_lossless_4x4(
    mut h: *mut crate::src::common::common::x264_t,
    mut p_dst: *mut crate::src::common::common::pixel,
    mut p: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
) {
    unsafe {
        let mut stride =
            (*(*h).fenc).i_stride[p as usize] << (*h).mb.interlaced as ::core::ffi::c_int;
        let mut p_src = (*h).mb.pic.p_fenc_plane[p as usize]
            .offset((block_idx_x[idx as usize] as ::core::ffi::c_int * 4i32) as isize)
            .offset((block_idx_y[idx as usize] as ::core::ffi::c_int * 4i32 * stride) as isize);
        if i_mode == crate::src::common::predict::I_PRED_4x4_V as ::core::ffi::c_int {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                p_dst,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                p_src.offset(-(stride as isize)),
                stride as crate::stdlib::intptr_t,
                4i32,
            );
            crate::stdlib::memcpy(
                p_dst as *mut ::core::ffi::c_void,
                p_dst.offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *const ::core::ffi::c_void,
                (4i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
        } else if i_mode == crate::src::common::predict::I_PRED_4x4_H as ::core::ffi::c_int {
            let mut i = 0i32;
            (*h).mc.copy[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                p_dst,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                p_src.offset(-(1isize)),
                stride as crate::stdlib::intptr_t,
                4i32,
            );
            while i < 4i32 {
                *p_dst.offset((i * crate::src::common::common::FDEC_STRIDE) as isize) =
                    *p_dst.offset((i * crate::src::common::common::FDEC_STRIDE - 1i32) as isize);
                i += 1;
            }
        } else {
            (*h).predict_4x4[i_mode as usize].expect("non-null function pointer")(p_dst);
        };
    }
}
pub unsafe extern "C" fn x264_8_predict_lossless_8x8(
    mut h: *mut crate::src::common::common::x264_t,
    mut p_dst: *mut crate::src::common::common::pixel,
    mut p: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut stride =
            (*(*h).fenc).i_stride[p as usize] << (*h).mb.interlaced as ::core::ffi::c_int;
        let mut p_src = (*h).mb.pic.p_fenc_plane[p as usize]
            .offset(((idx & 1i32) * 8i32) as isize)
            .offset(((idx >> 1i32) * 8i32 * stride) as isize);
        if i_mode == crate::src::common::predict::I_PRED_8x8_V as ::core::ffi::c_int {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                p_dst,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                p_src.offset(-(stride as isize)),
                stride as crate::stdlib::intptr_t,
                8i32,
            );
            crate::stdlib::memcpy(
                p_dst as *mut ::core::ffi::c_void,
                edge.offset(16isize) as *const ::core::ffi::c_void,
                (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
        } else if i_mode == crate::src::common::predict::I_PRED_8x8_H as ::core::ffi::c_int {
            let mut i = 0i32;
            (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                p_dst,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                p_src.offset(-(1isize)),
                stride as crate::stdlib::intptr_t,
                8i32,
            );
            while i < 8i32 {
                *p_dst.offset((i * crate::src::common::common::FDEC_STRIDE) as isize) =
                    *edge.offset((14i32 - i) as isize);
                i += 1;
            }
        } else {
            (*h).predict_8x8[i_mode as usize].expect("non-null function pointer")(p_dst, edge);
        };
    }
}
pub unsafe extern "C" fn x264_8_predict_lossless_16x16(
    mut h: *mut crate::src::common::common::x264_t,
    mut p: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
) {
    unsafe {
        let mut stride =
            (*(*h).fenc).i_stride[p as usize] << (*h).mb.interlaced as ::core::ffi::c_int;
        let mut p_dst = (*h).mb.pic.p_fdec[p as usize];
        if i_mode == crate::src::common::predict::I_PRED_16x16_V as ::core::ffi::c_int {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                p_dst,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fenc_plane[p as usize].offset(-(stride as isize)),
                stride as crate::stdlib::intptr_t,
                16i32,
            );
            crate::stdlib::memcpy(
                p_dst as *mut ::core::ffi::c_void,
                p_dst.offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *const ::core::ffi::c_void,
                (16i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
        } else if i_mode == crate::src::common::predict::I_PRED_16x16_H as ::core::ffi::c_int {
            let mut i = 0i32;
            (*h).mc
                .copy_16x16_unaligned
                .expect("non-null function pointer")(
                p_dst,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fenc_plane[p as usize].offset(-(1isize)),
                stride as crate::stdlib::intptr_t,
                16i32,
            );
            while i < 16i32 {
                *p_dst.offset((i * crate::src::common::common::FDEC_STRIDE) as isize) =
                    *p_dst.offset((i * crate::src::common::common::FDEC_STRIDE - 1i32) as isize);
                i += 1;
            }
        } else {
            (*h).predict_16x16[i_mode as usize].expect("non-null function pointer")(p_dst);
        };
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut plane_count: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut b_force_no_skip = 0i32;
        let mut p = 0i32;
        let mut i_qp = (*h).mb.i_qp;
        let mut b_decimate = (*h).mb.dct_decimate;
        (*h).mb.i_cbp_luma = 0i32;
        while p < plane_count {
            (*h).mb.cache.non_zero_count
                [x264_scan8[(crate::src::common::base::LUMA_DC + p) as usize] as usize] = 0u8;
            p += 1;
        }
        if (*h).mb.ty == MacroblockType::I_PCM {
            let mut p_0 = 0i32;
            while p_0 < plane_count {
                (*h).mc.copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[p_0 as usize],
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (*h).mb.pic.p_fenc[p_0 as usize],
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    16i32,
                );
                p_0 += 1;
            }
            if chroma != 0 {
                let mut height =
                    16i32 >> ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
                (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[1usize],
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (*h).mb.pic.p_fenc[1usize],
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    height,
                );
                (*h).mc.copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[2usize],
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (*h).mb.pic.p_fenc[2usize],
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    height,
                );
            }
            return;
        }
        if !(*h).mb.allow_skip {
            b_force_no_skip = 1i32;
            if (*h).mb.ty == MacroblockType::P_SKIP || (*h).mb.ty == MacroblockType::B_SKIP {
                if (*h).mb.ty == MacroblockType::P_SKIP {
                    (*h).mb.ty = MacroblockType::P_L0;
                } else if (*h).mb.ty == MacroblockType::B_SKIP {
                    (*h).mb.ty = MacroblockType::B_DIRECT;
                }
            }
        }
        if (*h).mb.ty == MacroblockType::P_SKIP {
            if !(*h).mb.skip_mc {
                let mut p_1 = 0i32;
                let mut mvx = x264_clip3(
                    (*h).mb.cache.mv[0usize][x264_scan8[0usize] as usize][0usize]
                        as ::core::ffi::c_int,
                    (*h).mb.mv_min[0usize],
                    (*h).mb.mv_max[0usize],
                );
                let mut mvy = x264_clip3(
                    (*h).mb.cache.mv[0usize][x264_scan8[0usize] as usize][1usize]
                        as ::core::ffi::c_int,
                    (*h).mb.mv_min[1usize],
                    (*h).mb.mv_max[1usize],
                );
                while p_1 < plane_count {
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[p_1 as usize],
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                            as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                            .offset(0isize)
                            as *mut [*mut crate::src::common::common::pixel; 12])
                            .offset(0isize)
                            as *mut *mut crate::src::common::common::pixel)
                            .offset((p_1 * 4i32) as isize),
                        (*h).mb.pic.i_stride[p_1 as usize] as crate::stdlib::intptr_t,
                        mvx,
                        mvy,
                        16i32,
                        16i32,
                        (&raw mut *(&raw mut (*h).sh.weight
                            as *mut [crate::src::common::mc::x264_weight_t; 3])
                            .offset(0isize)
                            as *mut crate::src::common::mc::x264_weight_t)
                            .offset(p_1 as isize),
                    );
                    p_1 += 1;
                }
                if chroma != 0 {
                    let mut v_shift = ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
                    let mut height_0 = 16i32 >> v_shift;
                    if mvx | mvy != 0 {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            (*h).mb.pic.p_fdec[1usize],
                            (*h).mb.pic.p_fdec[2usize],
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*h).mb.pic.p_fref[0usize][0usize][4usize],
                            (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                            mvx,
                            (2i32 * mvy) >> v_shift,
                            8i32,
                            height_0,
                        );
                    } else {
                        (*h).mc
                            .load_deinterleave_chroma_fdec
                            .expect("non-null function pointer")(
                            (*h).mb.pic.p_fdec[1usize],
                            (*h).mb.pic.p_fref[0usize][0usize][4usize],
                            (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                            height_0,
                        );
                    }
                    if !(*h).sh.weight[0usize][1usize].weightfn.is_null() {
                        (*(*h).sh.weight[0usize][1usize]
                            .weightfn
                            .offset((8i32 >> 2i32) as isize))
                        .expect("non-null function pointer")(
                            (*h).mb.pic.p_fdec[1usize],
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*h).mb.pic.p_fdec[1usize],
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (&raw mut *(&raw mut (*h).sh.weight
                                as *mut [crate::src::common::mc::x264_weight_t; 3])
                                .offset(0isize)
                                as *mut crate::src::common::mc::x264_weight_t)
                                .offset(1isize),
                            height_0,
                        );
                    }
                    if !(*h).sh.weight[0usize][2usize].weightfn.is_null() {
                        (*(*h).sh.weight[0usize][2usize]
                            .weightfn
                            .offset((8i32 >> 2i32) as isize))
                        .expect("non-null function pointer")(
                            (*h).mb.pic.p_fdec[2usize],
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*h).mb.pic.p_fdec[2usize],
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (&raw mut *(&raw mut (*h).sh.weight
                                as *mut [crate::src::common::mc::x264_weight_t; 3])
                                .offset(0isize)
                                as *mut crate::src::common::mc::x264_weight_t)
                                .offset(2isize),
                            height_0,
                        );
                    }
                }
            }
            macroblock_encode_skip(h);
            return;
        }
        if (*h).mb.ty == MacroblockType::B_SKIP {
            if !(*h).mb.skip_mc {
                crate::src::common::macroblock::x264_8_mb_mc(h);
            }
            macroblock_encode_skip(h);
            return;
        }
        if (*h).mb.ty == MacroblockType::I_16x16 {
            let mut p_2 = 0i32;
            (*h).mb.transform_8x8 = false;
            while p_2 < plane_count {
                mb_encode_i16x16(h, p_2, i_qp);
                p_2 += 1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else if (*h).mb.ty == MacroblockType::I_8x8 {
            let mut p_3 = 0i32;
            (*h).mb.transform_8x8 = true;
            if (*h).mb.i_skip_intra != 0 {
                (*h).mc.copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[0usize],
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut (*h).mb.pic.i8x8_fdec_buf as *mut crate::src::common::common::pixel,
                    16isize,
                    16i32,
                );
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*h).mb.pic.i8x8_nnz_buf[0usize];
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(2isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*h).mb.pic.i8x8_nnz_buf[1usize];
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(8isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*h).mb.pic.i8x8_nnz_buf[2usize];
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(10isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*h).mb.pic.i8x8_nnz_buf[3usize];
                (*h).mb.i_cbp_luma = (*h).mb.pic.i8x8_cbp;
                if (*h).mb.i_skip_intra == 2i32 {
                    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                        &raw mut (*h).dct.luma8x8 as *mut ::core::ffi::c_void,
                        &raw mut (*h).mb.pic.i8x8_dct_buf as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[[crate::src::common::common::dctcoef; 64]; 3]>(),
                    );
                }
            }
            while p_3 < plane_count {
                let mut i = if p_3 == 0i32 && (*h).mb.i_skip_intra != 0 {
                    3i32
                } else {
                    0i32
                };
                while i < 4i32 {
                    let mut i_mode = (*h).mb.cache.intra4x4_pred_mode
                        [x264_scan8[(4i32 * i) as usize] as usize]
                        as ::core::ffi::c_int;
                    x264_mb_encode_i8x8(
                        h,
                        p_3,
                        i,
                        i_qp,
                        i_mode,
                        ::core::ptr::null_mut::<crate::src::common::common::pixel>(),
                        1i32,
                    );
                    i += 1;
                }
                p_3 += 1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else if (*h).mb.ty == MacroblockType::I_4x4 {
            let mut p_4 = 0i32;
            (*h).mb.transform_8x8 = false;
            if (*h).mb.i_skip_intra != 0 {
                (*h).mc.copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[0usize],
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut (*h).mb.pic.i4x4_fdec_buf as *mut crate::src::common::common::pixel,
                    16isize,
                    16i32,
                );
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*h).mb.pic.i4x4_nnz_buf[0usize];
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(2isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*h).mb.pic.i4x4_nnz_buf[1usize];
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(8isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*h).mb.pic.i4x4_nnz_buf[2usize];
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(10isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*h).mb.pic.i4x4_nnz_buf[3usize];
                (*h).mb.i_cbp_luma = (*h).mb.pic.i4x4_cbp;
                if (*h).mb.i_skip_intra == 2i32 {
                    (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                        &raw mut (*h).dct.luma4x4 as *mut ::core::ffi::c_void,
                        &raw mut (*h).mb.pic.i4x4_dct_buf as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[[crate::src::common::common::dctcoef; 16]; 15]>(),
                    );
                }
            }
            while p_4 < plane_count {
                let mut i_0 = if p_4 == 0i32 && (*h).mb.i_skip_intra != 0 {
                    15i32
                } else {
                    0i32
                };
                while i_0 < 16i32 {
                    let mut p_dst = (*(&raw mut (*h).mb.pic.p_fdec
                        as *mut *mut crate::src::common::common::pixel)
                        .offset(p_4 as isize))
                    .offset(
                        *(&raw const block_idx_xy_fdec as *const crate::stdlib::uint16_t)
                            .offset(i_0 as isize) as isize,
                    );
                    let mut i_mode_0 = (*h).mb.cache.intra4x4_pred_mode
                        [x264_scan8[i_0 as usize] as usize]
                        as ::core::ffi::c_int;
                    if (*h).mb.i_neighbour4[i_0 as usize]
                        & (crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
                            | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                        == crate::src::common::macroblock::MB_TOP
                    {
                        (*(p_dst.offset((4i32 - 32i32) as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*p_dst.offset((3i32 - 32i32) as isize) as ::core::ffi::c_uint)
                            .wrapping_mul(0x1010101u32);
                    }
                    x264_mb_encode_i4x4(h, p_4, i_0, i_qp, i_mode_0, 1i32);
                    i_0 += 1;
                }
                p_4 += 1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else {
            let mut nz = 0;
            let mut i_decimate_mb = 0i32;
            if !(*h).mb.skip_mc {
                crate::src::common::macroblock::x264_8_mb_mc(h);
            }
            if (*h).mb.lossless {
                if (*h).mb.transform_8x8 {
                    let mut p_5 = 0i32;
                    while p_5 < plane_count {
                        let mut i8x8 = 0i32;
                        while i8x8 < 4i32 {
                            let mut x = i8x8 & 1i32;
                            let mut y = i8x8 >> 1i32;
                            nz = (*h).zigzagf.sub_8x8.expect("non-null function pointer")(
                                &raw mut *(&raw mut (*h).dct.luma8x8
                                    as *mut [crate::src::common::common::dctcoef; 64])
                                    .offset((p_5 * 4i32 + i8x8) as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                (*h).mb.pic.p_fenc[p_5 as usize]
                                    .offset((8i32 * x) as isize)
                                    .offset(
                                        (8i32 * y * crate::src::common::common::FENC_STRIDE)
                                            as isize,
                                    ),
                                (*h).mb.pic.p_fdec[p_5 as usize]
                                    .offset((8i32 * x) as isize)
                                    .offset(
                                        (8i32 * y * crate::src::common::common::FDEC_STRIDE)
                                            as isize,
                                    ),
                            );
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_5 * 16i32 + i8x8 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 0i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (nz * 0x101i32) as crate::stdlib::uint16_t;
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_5 * 16i32 + i8x8 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 8i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (nz * 0x101i32) as crate::stdlib::uint16_t;
                            (*h).mb.i_cbp_luma |= nz << i8x8;
                            i8x8 += 1;
                        }
                        p_5 += 1;
                    }
                } else {
                    let mut p_6 = 0i32;
                    while p_6 < plane_count {
                        let mut i4x4 = 0i32;
                        while i4x4 < 16i32 {
                            nz = (*h).zigzagf.sub_4x4.expect("non-null function pointer")(
                                &raw mut *(&raw mut (*h).dct.luma4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((p_6 * 16i32 + i4x4) as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                (*h).mb.pic.p_fenc[p_6 as usize].offset(
                                    block_idx_xy_fenc[i4x4 as usize] as ::core::ffi::c_int as isize,
                                ),
                                (*h).mb.pic.p_fdec[p_6 as usize].offset(
                                    block_idx_xy_fdec[i4x4 as usize] as ::core::ffi::c_int as isize,
                                ),
                            );
                            (*h).mb.cache.non_zero_count
                                [x264_scan8[(p_6 * 16i32 + i4x4) as usize] as usize] =
                                nz as crate::stdlib::uint8_t;
                            (*h).mb.i_cbp_luma |= nz << (i4x4 >> 2i32);
                            i4x4 += 1;
                        }
                        p_6 += 1;
                    }
                }
            } else if (*h).mb.transform_8x8 {
                let mut p_7 = 0i32;
                b_decimate &= !(*h).mb.trellis || !(*h).param.cabac;
                while p_7 < plane_count {
                    let mut dct8x8 = [[0; 64]; 4];
                    let mut plane_cbp = 0i32;
                    let mut idx = 0i32;
                    let mut quant_cat = if p_7 != 0 {
                        crate::src::common::set::CQM_8PC as ::core::ffi::c_int
                    } else {
                        crate::src::common::set::CQM_8PY as ::core::ffi::c_int
                    };
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 * p_7) as isize)
                                as ::core::ffi::c_int
                                + 0i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 * p_7) as isize)
                                as ::core::ffi::c_int
                                + 1i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 * p_7) as isize)
                                as ::core::ffi::c_int
                                + 2i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 * p_7) as isize)
                                as ::core::ffi::c_int
                                + 3i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*h).dctf.sub16x16_dct8.expect("non-null function pointer")(
                        &raw mut dct8x8 as *mut [crate::src::common::common::dctcoef; 64],
                        (*h).mb.pic.p_fenc[p_7 as usize],
                        (*h).mb.pic.p_fdec[p_7 as usize],
                    );
                    let ref mut c2rust_fresh0 = *(*h)
                        .nr_count
                        .offset((1i32 + (p_7 != 0) as ::core::ffi::c_int * 2i32) as isize);
                    *c2rust_fresh0 = (*c2rust_fresh0).wrapping_add(
                        ((*h).mb.noise_reduction as ::core::ffi::c_int * 4i32)
                            as crate::stdlib::uint32_t,
                    );
                    while idx < 4i32 {
                        nz = x264_quant_8x8(
                            h,
                            &raw mut *(&raw mut dct8x8
                                as *mut [crate::src::common::common::dctcoef; 64])
                                .offset(idx as isize)
                                as *mut crate::src::common::common::dctcoef,
                            i_qp,
                            ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_8x8
                                as ::core::ffi::c_int
                                as usize][p_7 as usize]
                                as ::core::ffi::c_int,
                            0i32,
                            p_7,
                            idx,
                        );
                        if nz != 0 {
                            (*h).zigzagf.scan_8x8.expect("non-null function pointer")(
                                &raw mut *(&raw mut (*h).dct.luma8x8
                                    as *mut [crate::src::common::common::dctcoef; 64])
                                    .offset((p_7 * 4i32 + idx) as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut *(&raw mut dct8x8
                                    as *mut [crate::src::common::common::dctcoef; 64])
                                    .offset(idx as isize)
                                    as *mut crate::src::common::common::dctcoef,
                            );
                            if b_decimate {
                                let mut i_decimate_8x8 = (*h)
                                    .quantf
                                    .decimate_score64
                                    .expect("non-null function pointer")(
                                    &raw mut *(&raw mut (*h).dct.luma8x8
                                        as *mut [crate::src::common::common::dctcoef; 64])
                                        .offset((p_7 * 4i32 + idx) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                                i_decimate_mb += i_decimate_8x8;
                                if i_decimate_8x8 >= 4i32 {
                                    plane_cbp |= (1i32) << idx;
                                }
                            } else {
                                plane_cbp |= (1i32) << idx;
                            }
                        }
                        idx += 1;
                    }
                    if i_decimate_mb >= 6i32 || !b_decimate {
                        let mut idx_0 = 0i32;
                        (*h).mb.i_cbp_luma |= plane_cbp;
                        let mut msk = plane_cbp;
                        while msk != 0 && {
                            let mut skip = 0;
                            skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                            idx_0 += skip;
                            msk >>= skip + 1i32;
                            1i32 != 0
                        } {
                            (*h).quantf.dequant_8x8.expect("non-null function pointer")(
                                &raw mut *(&raw mut dct8x8
                                    as *mut [crate::src::common::common::dctcoef; 64])
                                    .offset(idx_0 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                (*h).dequant8_mf[quant_cat as usize],
                                i_qp,
                            );
                            (*h).dctf.add8x8_idct8.expect("non-null function pointer")(
                                (*(&raw mut (*h).mb.pic.p_fdec
                                    as *mut *mut crate::src::common::common::pixel)
                                    .offset(p_7 as isize))
                                .offset(
                                    (8i32 * (idx_0 & 1i32)
                                        + 8i32
                                            * (idx_0 >> 1i32)
                                            * crate::src::common::common::FDEC_STRIDE)
                                        as isize,
                                ),
                                &raw mut *(&raw mut dct8x8
                                    as *mut [crate::src::common::common::dctcoef; 64])
                                    .offset(idx_0 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                            );
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_7 * 16i32 + idx_0 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 0i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (1i32 * 0x101i32) as crate::stdlib::uint16_t;
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_7 * 16i32 + idx_0 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 8i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (1i32 * 0x101i32) as crate::stdlib::uint16_t;
                            idx_0 += 1;
                        }
                    }
                    p_7 += 1;
                    i_qp = (*h).mb.i_chroma_qp;
                }
            } else {
                let mut p_8 = 0i32;
                while p_8 < plane_count {
                    let mut dct4x4 = [[0; 16]; 16];
                    let mut plane_cbp_0 = 0i32;
                    let mut i8x8_0 = 0i32;
                    let mut quant_cat_0 = if p_8 != 0 {
                        crate::src::common::set::CQM_4PC as ::core::ffi::c_int
                    } else {
                        crate::src::common::set::CQM_4PY as ::core::ffi::c_int
                    };
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 * p_8) as isize)
                                as ::core::ffi::c_int
                                + 0i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 * p_8) as isize)
                                as ::core::ffi::c_int
                                + 1i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 * p_8) as isize)
                                as ::core::ffi::c_int
                                + 2i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((16i32 * p_8) as isize)
                                as ::core::ffi::c_int
                                + 3i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*h).dctf.sub16x16_dct.expect("non-null function pointer")(
                        &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                        (*h).mb.pic.p_fenc[p_8 as usize],
                        (*h).mb.pic.p_fdec[p_8 as usize],
                    );
                    if (*h).mb.noise_reduction {
                        let mut idx_1 = 0i32;
                        let ref mut c2rust_fresh1 = *(*h)
                            .nr_count
                            .offset((0i32 + (p_8 != 0) as ::core::ffi::c_int * 2i32) as isize);
                        *c2rust_fresh1 = (*c2rust_fresh1).wrapping_add(16u32);
                        while idx_1 < 16i32 {
                            (*h).quantf.denoise_dct.expect("non-null function pointer")(
                                &raw mut *(&raw mut dct4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(idx_1 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut *(*h).nr_residual_sum.offset(
                                    (0i32 + (p_8 != 0) as ::core::ffi::c_int * 2i32) as isize,
                                ) as *mut crate::stdlib::uint32_t,
                                &raw mut *(*h).nr_offset.offset(
                                    (0i32 + (p_8 != 0) as ::core::ffi::c_int * 2i32) as isize,
                                )
                                    as *mut crate::src::common::common::udctcoef,
                                16i32,
                            );
                            idx_1 += 1;
                        }
                    }
                    while i8x8_0 < 4i32 {
                        let mut nnz8x8 = 0i32;
                        let mut i_decimate_8x8_0 = if b_decimate { 0i32 } else { 6i32 };
                        if (*h).mb.trellis {
                            let mut i4x4_0 = 0i32;
                            while i4x4_0 < 4i32 {
                                let mut idx_2 = i8x8_0 * 4i32 + i4x4_0;
                                if crate::src::encoder::analyse::rdo_c::x264_8_quant_4x4_trellis(
                                    h,
                                    &raw mut *(&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(idx_2 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    quant_cat_0,
                                    i_qp,
                                    ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_4x4
                                        as ::core::ffi::c_int
                                        as usize][p_8 as usize]
                                        as ::core::ffi::c_int,
                                    0i32,
                                    (p_8 != 0) as ::core::ffi::c_int,
                                    p_8 * 16i32 + idx_2,
                                ) != 0
                                {
                                    (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                                        &raw mut *(&raw mut (*h).dct.luma4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset((p_8 * 16i32 + idx_2) as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                        &raw mut *(&raw mut dct4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset(idx_2 as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                    );
                                    (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                                        &raw mut *(&raw mut dct4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset(idx_2 as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                        (*h).dequant4_mf[quant_cat_0 as usize],
                                        i_qp,
                                    );
                                    if i_decimate_8x8_0 < 6i32 {
                                        i_decimate_8x8_0 += (*h)
                                            .quantf
                                            .decimate_score16
                                            .expect("non-null function pointer")(
                                            &raw mut *(&raw mut (*h).dct.luma4x4
                                                as *mut [crate::src::common::common::dctcoef; 16])
                                                .offset((p_8 * 16i32 + idx_2) as isize)
                                                as *mut crate::src::common::common::dctcoef,
                                        );
                                    }
                                    (*h).mb.cache.non_zero_count
                                        [x264_scan8[(p_8 * 16i32 + idx_2) as usize] as usize] = 1u8;
                                    nnz8x8 = 1i32;
                                }
                                i4x4_0 += 1;
                            }
                        } else {
                            nz = (*h).quantf.quant_4x4x4.expect("non-null function pointer")(
                                (&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((i8x8_0 * 4i32) as isize),
                                &raw mut *(*(&raw mut (*h).quant4_mf
                                    as *mut *mut [crate::src::common::common::udctcoef; 16])
                                    .offset(quant_cat_0 as isize))
                                .offset(i_qp as isize)
                                    as *mut crate::src::common::common::udctcoef,
                                &raw mut *(*(&raw mut (*h).quant4_bias
                                    as *mut *mut [crate::src::common::common::udctcoef; 16])
                                    .offset(quant_cat_0 as isize))
                                .offset(i_qp as isize)
                                    as *mut crate::src::common::common::udctcoef,
                            );
                            nnz8x8 = nz;
                            if nz != 0 {
                                let mut idx_3 = i8x8_0 * 4i32;
                                let mut msk_0 = nz;
                                while msk_0 != 0 && {
                                    let mut skip_0 = 0;
                                    skip_0 = x264_ctz_4bit(msk_0 as crate::stdlib::uint32_t);
                                    idx_3 += skip_0;
                                    msk_0 >>= skip_0 + 1i32;
                                    1i32 != 0
                                } {
                                    (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                                        &raw mut *(&raw mut (*h).dct.luma4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset((p_8 * 16i32 + idx_3) as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                        &raw mut *(&raw mut dct4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset(idx_3 as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                    );
                                    (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                                        &raw mut *(&raw mut dct4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset(idx_3 as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                        (*h).dequant4_mf[quant_cat_0 as usize],
                                        i_qp,
                                    );
                                    if i_decimate_8x8_0 < 6i32 {
                                        i_decimate_8x8_0 += (*h)
                                            .quantf
                                            .decimate_score16
                                            .expect("non-null function pointer")(
                                            &raw mut *(&raw mut (*h).dct.luma4x4
                                                as *mut [crate::src::common::common::dctcoef; 16])
                                                .offset((p_8 * 16i32 + idx_3) as isize)
                                                as *mut crate::src::common::common::dctcoef,
                                        );
                                    }
                                    (*h).mb.cache.non_zero_count
                                        [x264_scan8[(p_8 * 16i32 + idx_3) as usize] as usize] = 1u8;
                                    idx_3 += 1;
                                }
                            }
                        }
                        if nnz8x8 != 0 {
                            i_decimate_mb += i_decimate_8x8_0;
                            if i_decimate_8x8_0 < 4i32 {
                                (*((&raw mut (*h).mb.cache.non_zero_count
                                    as *mut crate::stdlib::uint8_t)
                                    .offset(
                                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                            .offset((p_8 * 16i32 + i8x8_0 * 4i32) as isize)
                                            as ::core::ffi::c_int
                                            + 0i32)
                                            as isize,
                                    )
                                    as *mut crate::src::common::base::x264_union16_t))
                                    .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                                (*((&raw mut (*h).mb.cache.non_zero_count
                                    as *mut crate::stdlib::uint8_t)
                                    .offset(
                                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                            .offset((p_8 * 16i32 + i8x8_0 * 4i32) as isize)
                                            as ::core::ffi::c_int
                                            + 8i32)
                                            as isize,
                                    )
                                    as *mut crate::src::common::base::x264_union16_t))
                                    .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                            } else {
                                plane_cbp_0 |= (1i32) << i8x8_0;
                            }
                        }
                        i8x8_0 += 1;
                    }
                    if i_decimate_mb < 6i32 {
                        plane_cbp_0 = 0i32;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * p_8) as isize)
                                    as ::core::ffi::c_int
                                    + 0i32 * 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * p_8) as isize)
                                    as ::core::ffi::c_int
                                    + 1i32 * 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * p_8) as isize)
                                    as ::core::ffi::c_int
                                    + 2i32 * 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * p_8) as isize)
                                    as ::core::ffi::c_int
                                    + 3i32 * 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                    } else {
                        let mut i8x8_1 = 0i32;
                        (*h).mb.i_cbp_luma |= plane_cbp_0;
                        let mut msk_1 = plane_cbp_0;
                        while msk_1 != 0 && {
                            let mut skip_1 = 0;
                            skip_1 = x264_ctz_4bit(msk_1 as crate::stdlib::uint32_t);
                            i8x8_1 += skip_1;
                            msk_1 >>= skip_1 + 1i32;
                            1i32 != 0
                        } {
                            (*h).dctf.add8x8_idct.expect("non-null function pointer")(
                                (*(&raw mut (*h).mb.pic.p_fdec
                                    as *mut *mut crate::src::common::common::pixel)
                                    .offset(p_8 as isize))
                                .offset(
                                    ((i8x8_1 & 1i32) * 8i32
                                        + (i8x8_1 >> 1i32)
                                            * 8i32
                                            * crate::src::common::common::FDEC_STRIDE)
                                        as isize,
                                ),
                                (&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((i8x8_1 * 4i32) as isize),
                            );
                            i8x8_1 += 1;
                        }
                    }
                    p_8 += 1;
                    i_qp = (*h).mb.i_chroma_qp;
                }
            }
        }
        if chroma != 0 {
            if (*h).mb.ty == MacroblockType::I_4x4
                || (*h).mb.ty == MacroblockType::I_8x8
                || (*h).mb.ty == MacroblockType::I_16x16
                || (*h).mb.ty == MacroblockType::I_PCM
            {
                let mut i_mode_1 = (*h).mb.i_chroma_pred_mode;
                if (*h).mb.lossless {
                    x264_8_predict_lossless_chroma(h, i_mode_1);
                } else {
                    (*h).predict_chroma[i_mode_1 as usize].expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[1usize],
                    );
                    (*h).predict_chroma[i_mode_1 as usize].expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[2usize],
                    );
                }
            }
            x264_8_mb_encode_chroma(
                h,
                !((*h).mb.ty == MacroblockType::I_4x4
                    || (*h).mb.ty == MacroblockType::I_8x8
                    || (*h).mb.ty == MacroblockType::I_16x16
                    || (*h).mb.ty == MacroblockType::I_PCM) as ::core::ffi::c_int,
                (*h).mb.i_chroma_qp,
            );
        } else {
            (*h).mb.i_cbp_chroma = 0i32;
        }
        let mut cbp = (*h).mb.i_cbp_chroma << 4i32 | (*h).mb.i_cbp_luma;
        if (*h).param.cabac {
            cbp |= ((*h).mb.cache.non_zero_count
                [x264_scan8[crate::src::common::base::LUMA_DC as usize] as usize]
                as ::core::ffi::c_int)
                << 8i32
                | ((*h).mb.cache.non_zero_count
                    [x264_scan8[(crate::src::common::base::CHROMA_DC + 0i32) as usize] as usize]
                    as ::core::ffi::c_int)
                    << 9i32
                | ((*h).mb.cache.non_zero_count
                    [x264_scan8[(crate::src::common::base::CHROMA_DC + 1i32) as usize] as usize]
                    as ::core::ffi::c_int)
                    << 10i32;
        }
        *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) = cbp as crate::stdlib::int16_t;
        if b_force_no_skip == 0 {
            if (*h).mb.ty == MacroblockType::P_L0
                && (*h).mb.i_partition == Partition::D_16x16
                && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
                && (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as isize,
                    ) as *mut crate::src::common::base::x264_union32_t))
                    .i
                    == (*(&raw mut (*h).mb.cache.pskip_mv
                        as *mut crate::src::common::base::x264_union32_t))
                        .i
                && (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int
                    == 0i32
            {
                (*h).mb.ty = MacroblockType::P_SKIP;
            }
            if (*h).mb.ty == MacroblockType::B_DIRECT
                && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
            {
                (*h).mb.ty = MacroblockType::B_SKIP;
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_macroblock_encode(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        if (*h).sps.i_chroma_format_idc.is_444() {
            macroblock_encode_internal(h, 3i32, 0i32);
        } else if !(*h).sps.i_chroma_format_idc.is_400() {
            macroblock_encode_internal(h, 1i32, 1i32);
        } else {
            macroblock_encode_internal(h, 1i32, 0i32);
        };
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_probe_skip_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_bidir: ::core::ffi::c_int,
    mut plane_count: ::core::ffi::c_int,
    mut chroma: ChromaFormat,
) -> bool {
    unsafe {
        let mut dct4x4 = [[0; 16]; 8];
        let mut dctscan = [0; 16];
        let mut mvp = [0; 2];
        let mut p = 0i32;
        let mut i_qp = (*h).mb.i_qp;
        while p < plane_count {
            let mut i8x8 = 0i32;
            let mut quant_cat = if p != 0 {
                crate::src::common::set::CQM_4PC as ::core::ffi::c_int
            } else {
                crate::src::common::set::CQM_4PY as ::core::ffi::c_int
            };
            if b_bidir == 0 {
                mvp[0usize] = x264_clip3(
                    (*h).mb.cache.pskip_mv[0usize] as ::core::ffi::c_int,
                    (*h).mb.mv_min[0usize],
                    (*h).mb.mv_max[0usize],
                ) as crate::stdlib::int16_t;
                mvp[1usize] = x264_clip3(
                    (*h).mb.cache.pskip_mv[1usize] as ::core::ffi::c_int,
                    (*h).mb.mv_min[1usize],
                    (*h).mb.mv_max[1usize],
                ) as crate::stdlib::int16_t;
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[p as usize],
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                        as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                        .offset(0isize)
                        as *mut [*mut crate::src::common::common::pixel; 12])
                        .offset(0isize)
                        as *mut *mut crate::src::common::common::pixel)
                        .offset((p * 4i32) as isize),
                    (*h).mb.pic.i_stride[p as usize] as crate::stdlib::intptr_t,
                    mvp[0usize] as ::core::ffi::c_int,
                    mvp[1usize] as ::core::ffi::c_int,
                    16i32,
                    16i32,
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(0isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(p as isize),
                );
            }
            while i8x8 < 4i32 {
                let mut idx = 0i32;
                let mut fenc_offset = (i8x8 & 1i32) * 8i32
                    + (i8x8 >> 1i32) * crate::src::common::common::FENC_STRIDE * 8i32;
                let mut fdec_offset = (i8x8 & 1i32) * 8i32
                    + (i8x8 >> 1i32) * crate::src::common::common::FDEC_STRIDE * 8i32;
                (*h).dctf.sub8x8_dct.expect("non-null function pointer")(
                    &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                    (*h).mb.pic.p_fenc[p as usize].offset(fenc_offset as isize),
                    (*h).mb.pic.p_fdec[p as usize].offset(fdec_offset as isize),
                );
                if (*h).mb.noise_reduction {
                    let mut i4x4 = 0i32;
                    while i4x4 < 4i32 {
                        (*h).quantf.denoise_dct.expect("non-null function pointer")(
                            &raw mut *(&raw mut dct4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset(i4x4 as isize)
                                as *mut crate::src::common::common::dctcoef,
                            &raw mut *(*h)
                                .nr_residual_sum
                                .offset((0i32 + (p != 0) as ::core::ffi::c_int * 2i32) as isize)
                                as *mut crate::stdlib::uint32_t,
                            &raw mut *(*h)
                                .nr_offset
                                .offset((0i32 + (p != 0) as ::core::ffi::c_int * 2i32) as isize)
                                as *mut crate::src::common::common::udctcoef,
                            16i32,
                        );
                        i4x4 += 1;
                    }
                }
                let mut nz = (*h).quantf.quant_4x4x4.expect("non-null function pointer")(
                    &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                    &raw mut *(*(&raw mut (*h).quant4_mf
                        as *mut *mut [crate::src::common::common::udctcoef; 16])
                        .offset(quant_cat as isize))
                    .offset(i_qp as isize)
                        as *mut crate::src::common::common::udctcoef,
                    &raw mut *(*(&raw mut (*h).quant4_bias
                        as *mut *mut [crate::src::common::common::udctcoef; 16])
                        .offset(quant_cat as isize))
                    .offset(i_qp as isize)
                        as *mut crate::src::common::common::udctcoef,
                );
                let mut msk = nz;
                while msk != 0 && {
                    let mut skip = 0;
                    skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                    idx += skip;
                    msk >>= skip + 1i32;
                    1i32 != 0
                } {
                    let mut i_decimate_mb = 0i32;
                    (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                        &raw mut dctscan as *mut crate::src::common::common::dctcoef,
                        &raw mut *(&raw mut dct4x4
                            as *mut [crate::src::common::common::dctcoef; 16])
                            .offset(idx as isize)
                            as *mut crate::src::common::common::dctcoef,
                    );
                    i_decimate_mb += (*h)
                        .quantf
                        .decimate_score16
                        .expect("non-null function pointer")(
                        &raw mut dctscan as *mut crate::src::common::common::dctcoef,
                    );
                    if i_decimate_mb >= 6i32 {
                        return false;
                    }
                    idx += 1;
                }
                i8x8 += 1;
            }
            p += 1;
            i_qp = (*h).mb.i_chroma_qp;
        }
        if chroma.is_420() || chroma.is_422() {
            let mut ch = 0i32;
            i_qp = (*h).mb.i_chroma_qp;
            let mut chroma422 = chroma.is_422() as ::core::ffi::c_int;
            let mut thresh = if chroma422 != 0 {
                (crate::src::common::tables::x264_lambda2_tab[i_qp as usize] + 16i32) >> 5i32
            } else {
                (crate::src::common::tables::x264_lambda2_tab[i_qp as usize] + 32i32) >> 6i32
            };
            if b_bidir == 0 {
                if (*(&raw mut mvp as *mut crate::src::common::base::x264_union32_t)).i != 0 {
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[1usize],
                        (*h).mb.pic.p_fdec[2usize],
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (*h).mb.pic.p_fref[0usize][0usize][4usize],
                        (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                        mvp[0usize] as ::core::ffi::c_int,
                        mvp[1usize] as ::core::ffi::c_int * ((1i32) << chroma422),
                        8i32,
                        if chroma422 != 0 { 16i32 } else { 8i32 },
                    );
                } else {
                    (*h).mc
                        .load_deinterleave_chroma_fdec
                        .expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[1usize],
                        (*h).mb.pic.p_fref[0usize][0usize][4usize],
                        (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                        if chroma422 != 0 { 16i32 } else { 8i32 },
                    );
                }
            }
            while ch < 2i32 {
                let mut ssd = 0;
                let mut p_src = (*h).mb.pic.p_fenc[(1i32 + ch) as usize];
                let mut p_dst = (*h).mb.pic.p_fdec[(1i32 + ch) as usize];
                if b_bidir == 0
                    && !(*h).sh.weight[0usize][(1i32 + ch) as usize]
                        .weightfn
                        .is_null()
                {
                    (*(*h).sh.weight[0usize][(1i32 + ch) as usize]
                        .weightfn
                        .offset((8i32 >> 2i32) as isize))
                    .expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[(1i32 + ch) as usize],
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (*h).mb.pic.p_fdec[(1i32 + ch) as usize],
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut *(&raw mut (*h).sh.weight
                            as *mut [crate::src::common::mc::x264_weight_t; 3])
                            .offset(0isize)
                            as *mut crate::src::common::mc::x264_weight_t)
                            .offset((1i32 + ch) as isize),
                        if chroma422 != 0 { 16i32 } else { 8i32 },
                    );
                }
                ssd = (*h).pixf.ssd[(if chroma422 != 0 {
                    crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int
                } else {
                    crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
                }) as usize]
                    .expect("non-null function pointer")(
                    p_dst,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    p_src,
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                );
                if ssd >= thresh {
                    let mut dct_dc = [0; 8];
                    let mut i_0 = 0i32;
                    if (*h).mb.noise_reduction {
                        let mut i = 0i32;
                        let mut i4x4_0 = 0i32;
                        while i <= chroma422 {
                            (*h).dctf.sub8x8_dct.expect("non-null function pointer")(
                                (&raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((4i32 * i) as isize),
                                p_src.offset(
                                    (8i32 * i * crate::src::common::common::FENC_STRIDE) as isize,
                                ),
                                p_dst.offset(
                                    (8i32 * i * crate::src::common::common::FDEC_STRIDE) as isize,
                                ),
                            );
                            i += 1;
                        }
                        while i4x4_0 < (if chroma422 != 0 { 8i32 } else { 4i32 }) {
                            (*h).quantf.denoise_dct.expect("non-null function pointer")(
                                &raw mut *(&raw mut dct4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i4x4_0 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut *(*h).nr_residual_sum.offset(2isize)
                                    as *mut crate::stdlib::uint32_t,
                                &raw mut *(*h).nr_offset.offset(2isize)
                                    as *mut crate::src::common::common::udctcoef,
                                16i32,
                            );
                            dct_dc[i4x4_0 as usize] = dct4x4[i4x4_0 as usize][0usize];
                            dct4x4[i4x4_0 as usize][0usize] = 0i16;
                            i4x4_0 += 1;
                        }
                    } else if chroma422 != 0 {
                        (*h).dctf.sub8x16_dct_dc.expect("non-null function pointer")(
                            &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                            p_src,
                            p_dst,
                        );
                    } else {
                        (*h).dctf.sub8x8_dct_dc.expect("non-null function pointer")(
                            &raw mut dct_dc as *mut crate::src::common::common::dctcoef,
                            p_src,
                            p_dst,
                        );
                    }
                    while i_0 <= chroma422 {
                        if (*h).quantf.quant_2x2_dc.expect("non-null function pointer")(
                            (&raw mut dct_dc as *mut crate::src::common::common::dctcoef)
                                .offset((4i32 * i_0) as isize),
                            (*(*h).quant4_mf
                                [crate::src::common::set::CQM_4PC as ::core::ffi::c_int as usize]
                                .offset((i_qp + 3i32 * chroma422) as isize))[0usize]
                                as ::core::ffi::c_int
                                >> 1i32,
                            ((*(*h).quant4_bias
                                [crate::src::common::set::CQM_4PC as ::core::ffi::c_int as usize]
                                .offset((i_qp + 3i32 * chroma422) as isize))[0usize]
                                as ::core::ffi::c_int)
                                << 1i32,
                        ) != 0
                        {
                            return false;
                        }
                        i_0 += 1;
                    }
                    if ssd >= thresh * 4i32 {
                        let mut i8x8_0 = 0i32;
                        if !(*h).mb.noise_reduction {
                            let mut i_1 = 0i32;
                            while i_1 <= chroma422 {
                                (*h).dctf.sub8x8_dct.expect("non-null function pointer")(
                                    (&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset((4i32 * i_1) as isize),
                                    p_src.offset(
                                        (8i32 * i_1 * crate::src::common::common::FENC_STRIDE)
                                            as isize,
                                    ),
                                    p_dst.offset(
                                        (8i32 * i_1 * crate::src::common::common::FDEC_STRIDE)
                                            as isize,
                                    ),
                                );
                                dct4x4[(i_1 * 4i32 + 0i32) as usize][0usize] = 0i16;
                                dct4x4[(i_1 * 4i32 + 1i32) as usize][0usize] = 0i16;
                                dct4x4[(i_1 * 4i32 + 2i32) as usize][0usize] = 0i16;
                                dct4x4[(i_1 * 4i32 + 3i32) as usize][0usize] = 0i16;
                                i_1 += 1;
                            }
                        }
                        while i8x8_0 < (if chroma422 != 0 { 2i32 } else { 1i32 }) {
                            let mut nz_0 =
                                (*h).quantf.quant_4x4x4.expect("non-null function pointer")(
                                    (&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset((i8x8_0 * 4i32) as isize),
                                    &raw mut *(*(&raw mut (*h).quant4_mf
                                        as *mut *mut [crate::src::common::common::udctcoef; 16])
                                        .offset(
                                            crate::src::common::set::CQM_4PC as ::core::ffi::c_int
                                                as isize,
                                        ))
                                    .offset(i_qp as isize)
                                        as *mut crate::src::common::common::udctcoef,
                                    &raw mut *(*(&raw mut (*h).quant4_bias
                                        as *mut *mut [crate::src::common::common::udctcoef; 16])
                                        .offset(
                                            crate::src::common::set::CQM_4PC as ::core::ffi::c_int
                                                as isize,
                                        ))
                                    .offset(i_qp as isize)
                                        as *mut crate::src::common::common::udctcoef,
                                );
                            let mut idx_0 = i8x8_0 * 4i32;
                            let mut msk_0 = nz_0;
                            while msk_0 != 0 && {
                                let mut skip_0 = 0;
                                skip_0 = x264_ctz_4bit(msk_0 as crate::stdlib::uint32_t);
                                idx_0 += skip_0;
                                msk_0 >>= skip_0 + 1i32;
                                1i32 != 0
                            } {
                                let mut i_decimate_mb_0 = 0i32;
                                (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                                    &raw mut dctscan as *mut crate::src::common::common::dctcoef,
                                    &raw mut *(&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(idx_0 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                                i_decimate_mb_0 += (*h)
                                    .quantf
                                    .decimate_score15
                                    .expect("non-null function pointer")(
                                    &raw mut dctscan as *mut crate::src::common::common::dctcoef,
                                );
                                if i_decimate_mb_0 >= 7i32 {
                                    return false;
                                }
                                idx_0 += 1;
                            }
                            i8x8_0 += 1;
                        }
                    }
                }
                ch += 1;
            }
        }
        (*h).mb.skip_mc = true;
        true
    }
}
pub unsafe extern "C" fn x264_8_macroblock_probe_skip(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_bidir: ::core::ffi::c_int,
) -> bool {
    unsafe {
        if (*h).sps.i_chroma_format_idc.is_420() {
            macroblock_probe_skip_internal(h, b_bidir, 1i32, ChromaFormat::Chroma420)
        } else if (*h).sps.i_chroma_format_idc.is_422() {
            macroblock_probe_skip_internal(h, b_bidir, 1i32, ChromaFormat::Chroma422)
        } else if (*h).sps.i_chroma_format_idc.is_444() {
            macroblock_probe_skip_internal(h, b_bidir, 3i32, ChromaFormat::Chroma444)
        } else {
            macroblock_probe_skip_internal(h, b_bidir, 1i32, ChromaFormat::Chroma400)
        }
    }
}
pub unsafe extern "C" fn x264_8_noise_reduction_update(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut cat = 0i32;
        (*h).nr_offset =
            &raw mut (*h).nr_offset_denoise as *mut [crate::src::common::common::udctcoef; 64];
        (*h).nr_residual_sum = &raw mut *(&raw mut (*h).nr_residual_sum_buf
            as *mut [[crate::stdlib::uint32_t; 64]; 4])
            .offset(0isize) as *mut [crate::stdlib::uint32_t; 64];
        (*h).nr_count = &raw mut *(&raw mut (*h).nr_count_buf as *mut [crate::stdlib::uint32_t; 4])
            .offset(0isize) as *mut crate::stdlib::uint32_t;
        while cat < 3i32 + ((*h).sps.i_chroma_format_idc.is_444()) as ::core::ffi::c_int {
            let mut i_0 = 0i32;
            let mut dct8x8 = cat & 1i32;
            let mut size = if dct8x8 != 0 { 64i32 } else { 16i32 };
            let mut weight = if dct8x8 != 0 {
                &raw const crate::src::common::tables::x264_dct8_weight2_tab
                    as *const crate::stdlib::uint32_t
            } else {
                &raw const crate::src::common::tables::x264_dct4_weight2_tab
                    as *const crate::stdlib::uint32_t
            };
            if *(*h).nr_count.offset(cat as isize)
                > (if dct8x8 != 0 {
                    (1i32) << 16i32
                } else {
                    (1i32) << 18i32
                }) as crate::stdlib::uint32_t
            {
                let mut i = 0i32;
                while i < size {
                    (*(*h).nr_residual_sum.offset(cat as isize))[i as usize] >>= 1i32;
                    i += 1;
                }
                *(*h).nr_count.offset(cat as isize) >>= 1i32;
            }
            while i_0 < size {
                (*(*h).nr_offset.offset(cat as isize))[i_0 as usize] = ((*h)
                    .param
                    .analyse
                    .i_noise_reduction
                    as crate::stdlib::uint64_t)
                    .wrapping_mul(*(*h).nr_count.offset(cat as isize) as crate::stdlib::uint64_t)
                    .wrapping_add(
                        (*(*h).nr_residual_sum.offset(cat as isize))[i_0 as usize]
                            .wrapping_div(2u32) as crate::stdlib::uint64_t,
                    )
                    .wrapping_div(
                        ((*(*h).nr_residual_sum.offset(cat as isize))[i_0 as usize]
                            as crate::stdlib::uint64_t)
                            .wrapping_mul(*weight.offset(i_0 as isize) as crate::stdlib::uint64_t)
                            .wrapping_div(256u64)
                            .wrapping_add(1u64),
                    )
                    as crate::src::common::common::udctcoef;
                i_0 += 1;
            }
            (*(*h).nr_offset.offset(cat as isize))[0usize] = 0u16;
            cat += 1;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_p8x8_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut i8: ::core::ffi::c_int,
    mut plane_count: ::core::ffi::c_int,
    mut chroma: ChromaFormat,
) {
    unsafe {
        let mut nz = 0;
        let mut b_decimate = (*h).mb.dct_decimate;
        let mut i_qp = (*h).mb.i_qp;
        let mut x = i8 & 1i32;
        let mut y = i8 >> 1i32;
        let mut chroma422 = chroma.is_422() as ::core::ffi::c_int;
        (*h).mb.i_cbp_chroma = 0i32;
        (*h).mb.i_cbp_luma &= !((1i32) << i8);
        if !(*h).mb.skip_mc {
            crate::src::common::macroblock::x264_8_mb_mc_8x8(h, i8);
        }
        if (*h).mb.lossless {
            let mut p = 0i32;
            while p < plane_count {
                let mut nnz8x8 = 0i32;
                let mut p_fenc = (*h).mb.pic.p_fenc[p as usize]
                    .offset((8i32 * x) as isize)
                    .offset((8i32 * y * crate::src::common::common::FENC_STRIDE) as isize);
                let mut p_fdec = (*h).mb.pic.p_fdec[p as usize]
                    .offset((8i32 * x) as isize)
                    .offset((8i32 * y * crate::src::common::common::FDEC_STRIDE) as isize);
                if (*h).mb.transform_8x8 {
                    nnz8x8 = (*h).zigzagf.sub_8x8.expect("non-null function pointer")(
                        &raw mut *(&raw mut (*h).dct.luma8x8
                            as *mut [crate::src::common::common::dctcoef; 64])
                            .offset((4i32 * p + i8) as isize)
                            as *mut crate::src::common::common::dctcoef,
                        p_fenc,
                        p_fdec,
                    );
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((p * 16i32 + i8 * 4i32) as isize)
                                as ::core::ffi::c_int
                                + 0i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (nnz8x8 * 0x101i32) as crate::stdlib::uint16_t;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((p * 16i32 + i8 * 4i32) as isize)
                                as ::core::ffi::c_int
                                + 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (nnz8x8 * 0x101i32) as crate::stdlib::uint16_t;
                } else {
                    let mut i4 = i8 * 4i32;
                    while i4 < i8 * 4i32 + 4i32 {
                        nz = (*h).zigzagf.sub_4x4.expect("non-null function pointer")(
                            &raw mut *(&raw mut (*h).dct.luma4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset((16i32 * p + i4) as isize)
                                as *mut crate::src::common::common::dctcoef,
                            (*h).mb.pic.p_fenc[p as usize].offset(
                                block_idx_xy_fenc[i4 as usize] as ::core::ffi::c_int as isize,
                            ),
                            (*h).mb.pic.p_fdec[p as usize].offset(
                                block_idx_xy_fdec[i4 as usize] as ::core::ffi::c_int as isize,
                            ),
                        );
                        (*h).mb.cache.non_zero_count
                            [x264_scan8[(16i32 * p + i4) as usize] as usize] =
                            nz as crate::stdlib::uint8_t;
                        nnz8x8 |= nz;
                        i4 += 1;
                    }
                }
                (*h).mb.i_cbp_luma |= nnz8x8 << i8;
                p += 1;
            }
            if chroma.is_420() || chroma.is_422() {
                let mut ch = 0i32;
                while ch < 2i32 {
                    let mut i4x4 = 0i32;
                    let mut p_fenc_0 = (*h).mb.pic.p_fenc[(1i32 + ch) as usize]
                        .offset((4i32 * x) as isize)
                        .offset(
                            ((if chroma422 != 0 { 8i32 } else { 4i32 })
                                * y
                                * crate::src::common::common::FENC_STRIDE)
                                as isize,
                        );
                    let mut p_fdec_0 = (*h).mb.pic.p_fdec[(1i32 + ch) as usize]
                        .offset((4i32 * x) as isize)
                        .offset(
                            ((if chroma422 != 0 { 8i32 } else { 4i32 })
                                * y
                                * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        );
                    while i4x4 <= chroma422 {
                        let mut dc = 0;
                        let mut offset = if chroma422 != 0 {
                            8i32 * y + 2i32 * i4x4 + x
                        } else {
                            i8
                        };
                        nz = (*h).zigzagf.sub_4x4ac.expect("non-null function pointer")(
                            &raw mut *(&raw mut (*h).dct.luma4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset((16i32 + offset + ch * 16i32) as isize)
                                as *mut crate::src::common::common::dctcoef,
                            p_fenc_0.offset(
                                (4i32 * i4x4 * crate::src::common::common::FENC_STRIDE) as isize,
                            ),
                            p_fdec_0.offset(
                                (4i32 * i4x4 * crate::src::common::common::FDEC_STRIDE) as isize,
                            ),
                            &raw mut dc,
                        );
                        (*h).mb.cache.non_zero_count
                            [x264_scan8[(16i32 + offset + ch * 16i32) as usize] as usize] =
                            nz as crate::stdlib::uint8_t;
                        i4x4 += 1;
                    }
                    ch += 1;
                }
                (*h).mb.i_cbp_chroma = 0x2i32;
            }
        } else {
            if (*h).mb.transform_8x8 {
                let mut p_0 = 0i32;
                while p_0 < plane_count {
                    let mut dct8x8 = [0; 64];
                    let mut quant_cat = if p_0 != 0 {
                        crate::src::common::set::CQM_8PC as ::core::ffi::c_int
                    } else {
                        crate::src::common::set::CQM_8PY as ::core::ffi::c_int
                    };
                    let mut p_fenc_1 = (*h).mb.pic.p_fenc[p_0 as usize]
                        .offset((8i32 * x) as isize)
                        .offset((8i32 * y * crate::src::common::common::FENC_STRIDE) as isize);
                    let mut p_fdec_1 = (*h).mb.pic.p_fdec[p_0 as usize]
                        .offset((8i32 * x) as isize)
                        .offset((8i32 * y * crate::src::common::common::FDEC_STRIDE) as isize);
                    (*h).dctf.sub8x8_dct8.expect("non-null function pointer")(
                        &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                        p_fenc_1,
                        p_fdec_1,
                    );
                    let mut nnz8x8_0 = x264_quant_8x8(
                        h,
                        &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                        i_qp,
                        ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_8x8
                            as ::core::ffi::c_int as usize][p_0 as usize]
                            as ::core::ffi::c_int,
                        0i32,
                        p_0,
                        i8,
                    );
                    if nnz8x8_0 != 0 {
                        (*h).zigzagf.scan_8x8.expect("non-null function pointer")(
                            &raw mut *(&raw mut (*h).dct.luma8x8
                                as *mut [crate::src::common::common::dctcoef; 64])
                                .offset((4i32 * p_0 + i8) as isize)
                                as *mut crate::src::common::common::dctcoef,
                            &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                        );
                        if b_decimate && !(*h).mb.trellis {
                            nnz8x8_0 = (4i32
                                <= (*h)
                                    .quantf
                                    .decimate_score64
                                    .expect("non-null function pointer")(
                                    &raw mut *(&raw mut (*h).dct.luma8x8
                                        as *mut [crate::src::common::common::dctcoef; 64])
                                        .offset((4i32 * p_0 + i8) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                )) as ::core::ffi::c_int;
                        }
                        if nnz8x8_0 != 0 {
                            (*h).quantf.dequant_8x8.expect("non-null function pointer")(
                                &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                                (*h).dequant8_mf[quant_cat as usize],
                                i_qp,
                            );
                            (*h).dctf.add8x8_idct8.expect("non-null function pointer")(
                                p_fdec_1,
                                &raw mut dct8x8 as *mut crate::src::common::common::dctcoef,
                            );
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_0 * 16i32 + i8 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 0i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (1i32 * 0x101i32) as crate::stdlib::uint16_t;
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_0 * 16i32 + i8 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 8i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (1i32 * 0x101i32) as crate::stdlib::uint16_t;
                            (*h).mb.i_cbp_luma |= (1i32) << i8;
                        } else {
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_0 * 16i32 + i8 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 0i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_0 * 16i32 + i8 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 8i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                        }
                    } else {
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((p_0 * 16i32 + i8 * 4i32) as isize)
                                    as ::core::ffi::c_int
                                    + 0i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union16_t))
                            .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((p_0 * 16i32 + i8 * 4i32) as isize)
                                    as ::core::ffi::c_int
                                    + 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union16_t))
                            .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                    }
                    p_0 += 1;
                    i_qp = (*h).mb.i_chroma_qp;
                }
            } else {
                let mut p_1 = 0i32;
                while p_1 < plane_count {
                    let mut dct4x4 = [[0; 16]; 4];
                    let mut nnz8x8_1 = 0i32;
                    let mut quant_cat_0 = if p_1 != 0 {
                        crate::src::common::set::CQM_4PC as ::core::ffi::c_int
                    } else {
                        crate::src::common::set::CQM_4PY as ::core::ffi::c_int
                    };
                    let mut p_fenc_2 = (*h).mb.pic.p_fenc[p_1 as usize]
                        .offset((8i32 * x) as isize)
                        .offset((8i32 * y * crate::src::common::common::FENC_STRIDE) as isize);
                    let mut p_fdec_2 = (*h).mb.pic.p_fdec[p_1 as usize]
                        .offset((8i32 * x) as isize)
                        .offset((8i32 * y * crate::src::common::common::FDEC_STRIDE) as isize);
                    let mut i_decimate_8x8 = if b_decimate { 0i32 } else { 4i32 };
                    (*h).dctf.sub8x8_dct.expect("non-null function pointer")(
                        &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                        p_fenc_2,
                        p_fdec_2,
                    );
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((p_1 * 16i32 + i8 * 4i32) as isize)
                                as ::core::ffi::c_int
                                + 0i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((p_1 * 16i32 + i8 * 4i32) as isize)
                                as ::core::ffi::c_int
                                + 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                    if (*h).mb.noise_reduction {
                        let mut idx = 0i32;
                        while idx < 4i32 {
                            (*h).quantf.denoise_dct.expect("non-null function pointer")(
                                &raw mut *(&raw mut dct4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(idx as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut *(*h).nr_residual_sum.offset(
                                    (0i32 + (p_1 != 0) as ::core::ffi::c_int * 2i32) as isize,
                                ) as *mut crate::stdlib::uint32_t,
                                &raw mut *(*h).nr_offset.offset(
                                    (0i32 + (p_1 != 0) as ::core::ffi::c_int * 2i32) as isize,
                                )
                                    as *mut crate::src::common::common::udctcoef,
                                16i32,
                            );
                            idx += 1;
                        }
                    }
                    if (*h).mb.trellis {
                        let mut i4x4_0 = 0i32;
                        while i4x4_0 < 4i32 {
                            if crate::src::encoder::analyse::rdo_c::x264_8_quant_4x4_trellis(
                                h,
                                &raw mut *(&raw mut dct4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i4x4_0 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                quant_cat_0,
                                i_qp,
                                ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_4x4
                                    as ::core::ffi::c_int
                                    as usize][p_1 as usize]
                                    as ::core::ffi::c_int,
                                0i32,
                                (p_1 != 0) as ::core::ffi::c_int,
                                i8 * 4i32 + i4x4_0 + p_1 * 16i32,
                            ) != 0
                            {
                                (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                                    &raw mut *(&raw mut (*h).dct.luma4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset((p_1 * 16i32 + i8 * 4i32 + i4x4_0) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    &raw mut *(&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(i4x4_0 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                                (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                                    &raw mut *(&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(i4x4_0 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    (*h).dequant4_mf[quant_cat_0 as usize],
                                    i_qp,
                                );
                                if i_decimate_8x8 < 4i32 {
                                    i_decimate_8x8 += (*h)
                                        .quantf
                                        .decimate_score16
                                        .expect("non-null function pointer")(
                                        &raw mut *(&raw mut (*h).dct.luma4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset((p_1 * 16i32 + i8 * 4i32 + i4x4_0) as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                    );
                                }
                                (*h).mb.cache.non_zero_count[x264_scan8
                                    [(p_1 * 16i32 + i8 * 4i32 + i4x4_0) as usize]
                                    as usize] = 1u8;
                                nnz8x8_1 = 1i32;
                            }
                            i4x4_0 += 1;
                        }
                    } else {
                        nz = (*h).quantf.quant_4x4x4.expect("non-null function pointer")(
                            &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                            &raw mut *(*(&raw mut (*h).quant4_mf
                                as *mut *mut [crate::src::common::common::udctcoef; 16])
                                .offset(quant_cat_0 as isize))
                            .offset(i_qp as isize)
                                as *mut crate::src::common::common::udctcoef,
                            &raw mut *(*(&raw mut (*h).quant4_bias
                                as *mut *mut [crate::src::common::common::udctcoef; 16])
                                .offset(quant_cat_0 as isize))
                            .offset(i_qp as isize)
                                as *mut crate::src::common::common::udctcoef,
                        );
                        nnz8x8_1 = nz;
                        if nz != 0 {
                            let mut i4x4_1 = 0i32;
                            let mut msk = nz;
                            while msk != 0 && {
                                let mut skip = 0;
                                skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                                i4x4_1 += skip;
                                msk >>= skip + 1i32;
                                1i32 != 0
                            } {
                                (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                                    &raw mut *(&raw mut (*h).dct.luma4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset((p_1 * 16i32 + i8 * 4i32 + i4x4_1) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    &raw mut *(&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(i4x4_1 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                                (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                                    &raw mut *(&raw mut dct4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(i4x4_1 as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                    (*h).dequant4_mf[quant_cat_0 as usize],
                                    i_qp,
                                );
                                if i_decimate_8x8 < 4i32 {
                                    i_decimate_8x8 += (*h)
                                        .quantf
                                        .decimate_score16
                                        .expect("non-null function pointer")(
                                        &raw mut *(&raw mut (*h).dct.luma4x4
                                            as *mut [crate::src::common::common::dctcoef; 16])
                                            .offset((p_1 * 16i32 + i8 * 4i32 + i4x4_1) as isize)
                                            as *mut crate::src::common::common::dctcoef,
                                    );
                                }
                                (*h).mb.cache.non_zero_count[x264_scan8
                                    [(p_1 * 16i32 + i8 * 4i32 + i4x4_1) as usize]
                                    as usize] = 1u8;
                                i4x4_1 += 1;
                            }
                        }
                    }
                    if nnz8x8_1 != 0 {
                        if i_decimate_8x8 < 4i32 {
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_1 * 16i32 + i8 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 0i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                            (*((&raw mut (*h).mb.cache.non_zero_count
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset((p_1 * 16i32 + i8 * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + 8i32) as isize,
                                )
                                as *mut crate::src::common::base::x264_union16_t))
                                .i = (0i32 * 0x101i32) as crate::stdlib::uint16_t;
                        } else {
                            (*h).dctf.add8x8_idct.expect("non-null function pointer")(
                                p_fdec_2,
                                &raw mut dct4x4 as *mut [crate::src::common::common::dctcoef; 16],
                            );
                            (*h).mb.i_cbp_luma |= (1i32) << i8;
                        }
                    }
                    p_1 += 1;
                    i_qp = (*h).mb.i_chroma_qp;
                }
            }
            if chroma.is_420() || chroma.is_422() {
                let mut ch_0 = 0i32;
                i_qp = (*h).mb.i_chroma_qp;
                while ch_0 < 2i32 {
                    let mut i4x4_2 = 0i32;
                    let mut p_fenc_3 = (*h).mb.pic.p_fenc[(1i32 + ch_0) as usize]
                        .offset((4i32 * x) as isize)
                        .offset(
                            ((if chroma422 != 0 { 8i32 } else { 4i32 })
                                * y
                                * crate::src::common::common::FENC_STRIDE)
                                as isize,
                        );
                    let mut p_fdec_3 = (*h).mb.pic.p_fdec[(1i32 + ch_0) as usize]
                        .offset((4i32 * x) as isize)
                        .offset(
                            ((if chroma422 != 0 { 8i32 } else { 4i32 })
                                * y
                                * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        );
                    while i4x4_2 <= chroma422 {
                        let mut dct4x4_0 = [[0; 16]; 2];
                        (*h).dctf.sub4x4_dct.expect("non-null function pointer")(
                            &raw mut *(&raw mut dct4x4_0
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset(i4x4_2 as isize)
                                as *mut crate::src::common::common::dctcoef,
                            p_fenc_3.offset(
                                (4i32 * i4x4_2 * crate::src::common::common::FENC_STRIDE) as isize,
                            ),
                            p_fdec_3.offset(
                                (4i32 * i4x4_2 * crate::src::common::common::FDEC_STRIDE) as isize,
                            ),
                        );
                        if (*h).mb.noise_reduction {
                            (*h).quantf.denoise_dct.expect("non-null function pointer")(
                                &raw mut *(&raw mut dct4x4_0
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i4x4_2 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut *(*h).nr_residual_sum.offset(2isize)
                                    as *mut crate::stdlib::uint32_t,
                                &raw mut *(*h).nr_offset.offset(2isize)
                                    as *mut crate::src::common::common::udctcoef,
                                16i32,
                            );
                        }
                        dct4x4_0[i4x4_2 as usize][0usize] = 0i16;
                        if (*h).mb.trellis {
                            nz = crate::src::encoder::analyse::rdo_c::x264_8_quant_4x4_trellis(
                                h,
                                &raw mut *(&raw mut dct4x4_0
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i4x4_2 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                crate::src::common::set::CQM_4PC as ::core::ffi::c_int,
                                i_qp,
                                crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int,
                                0i32,
                                1i32,
                                0i32,
                            );
                        } else {
                            nz = (*h).quantf.quant_4x4.expect("non-null function pointer")(
                                &raw mut *(&raw mut dct4x4_0
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i4x4_2 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut *(*(&raw mut (*h).quant4_mf
                                    as *mut *mut [crate::src::common::common::udctcoef; 16])
                                    .offset(
                                        crate::src::common::set::CQM_4PC as ::core::ffi::c_int
                                            as isize,
                                    ))
                                .offset(i_qp as isize)
                                    as *mut crate::src::common::common::udctcoef,
                                &raw mut *(*(&raw mut (*h).quant4_bias
                                    as *mut *mut [crate::src::common::common::udctcoef; 16])
                                    .offset(
                                        crate::src::common::set::CQM_4PC as ::core::ffi::c_int
                                            as isize,
                                    ))
                                .offset(i_qp as isize)
                                    as *mut crate::src::common::common::udctcoef,
                            );
                        }
                        let mut offset_0 = if chroma422 != 0 {
                            ((5i32 * i8) & 0x9i32) + 2i32 * i4x4_2
                        } else {
                            i8
                        };
                        (*h).mb.cache.non_zero_count
                            [x264_scan8[(16i32 + offset_0 + ch_0 * 16i32) as usize] as usize] =
                            nz as crate::stdlib::uint8_t;
                        if nz != 0 {
                            (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                                &raw mut *(&raw mut (*h).dct.luma4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset((16i32 + offset_0 + ch_0 * 16i32) as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                &raw mut *(&raw mut dct4x4_0
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i4x4_2 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                            );
                            (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                                &raw mut *(&raw mut dct4x4_0
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i4x4_2 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                                (*h).dequant4_mf[crate::src::common::set::CQM_4PC
                                    as ::core::ffi::c_int
                                    as usize],
                                i_qp,
                            );
                            (*h).dctf.add4x4_idct.expect("non-null function pointer")(
                                p_fdec_3.offset(
                                    (4i32 * i4x4_2 * crate::src::common::common::FDEC_STRIDE)
                                        as isize,
                                ),
                                &raw mut *(&raw mut dct4x4_0
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i4x4_2 as isize)
                                    as *mut crate::src::common::common::dctcoef,
                            );
                        }
                        i4x4_2 += 1;
                    }
                    ch_0 += 1;
                }
                (*h).mb.i_cbp_chroma = 0x2i32;
            }
        };
    }
}
pub unsafe extern "C" fn x264_8_macroblock_encode_p8x8(
    mut h: *mut crate::src::common::common::x264_t,
    mut i8: ::core::ffi::c_int,
) {
    unsafe {
        if (*h).sps.i_chroma_format_idc.is_420() {
            macroblock_encode_p8x8_internal(h, i8, 1i32, ChromaFormat::Chroma420);
        } else if (*h).sps.i_chroma_format_idc.is_422() {
            macroblock_encode_p8x8_internal(h, i8, 1i32, ChromaFormat::Chroma422);
        } else if (*h).sps.i_chroma_format_idc.is_444() {
            macroblock_encode_p8x8_internal(h, i8, 3i32, ChromaFormat::Chroma444);
        } else {
            macroblock_encode_p8x8_internal(h, i8, 1i32, ChromaFormat::Chroma400);
        };
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_p4x4_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut i4: ::core::ffi::c_int,
    mut plane_count: ::core::ffi::c_int,
) {
    unsafe {
        let mut p = 0i32;
        let mut i_qp = (*h).mb.i_qp;
        while p < plane_count {
            let mut nz = 0;
            let mut quant_cat = if p != 0 {
                crate::src::common::set::CQM_4PC as ::core::ffi::c_int
            } else {
                crate::src::common::set::CQM_4PY as ::core::ffi::c_int
            };
            let mut p_fenc = (*(&raw mut (*h).mb.pic.p_fenc
                as *mut *mut crate::src::common::common::pixel)
                .offset(p as isize))
            .offset(
                *(&raw const block_idx_xy_fenc as *const crate::stdlib::uint8_t).offset(i4 as isize)
                    as isize,
            );
            let mut p_fdec = (*(&raw mut (*h).mb.pic.p_fdec
                as *mut *mut crate::src::common::common::pixel)
                .offset(p as isize))
            .offset(
                *(&raw const block_idx_xy_fdec as *const crate::stdlib::uint16_t)
                    .offset(i4 as isize) as isize,
            );
            if (*h).mb.lossless {
                nz = (*h).zigzagf.sub_4x4.expect("non-null function pointer")(
                    &raw mut *(&raw mut (*h).dct.luma4x4
                        as *mut [crate::src::common::common::dctcoef; 16])
                        .offset((p * 16i32 + i4) as isize)
                        as *mut crate::src::common::common::dctcoef,
                    p_fenc,
                    p_fdec,
                );
                (*h).mb.cache.non_zero_count[x264_scan8[(p * 16i32 + i4) as usize] as usize] =
                    nz as crate::stdlib::uint8_t;
            } else {
                let mut dct4x4 = [0; 16];
                (*h).dctf.sub4x4_dct.expect("non-null function pointer")(
                    &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                    p_fenc,
                    p_fdec,
                );
                nz = x264_quant_4x4(
                    h,
                    &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                    i_qp,
                    ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int
                        as usize][p as usize] as ::core::ffi::c_int,
                    0i32,
                    p,
                    i4,
                );
                (*h).mb.cache.non_zero_count[x264_scan8[(p * 16i32 + i4) as usize] as usize] =
                    nz as crate::stdlib::uint8_t;
                if nz != 0 {
                    (*h).zigzagf.scan_4x4.expect("non-null function pointer")(
                        &raw mut *(&raw mut (*h).dct.luma4x4
                            as *mut [crate::src::common::common::dctcoef; 16])
                            .offset((p * 16i32 + i4) as isize)
                            as *mut crate::src::common::common::dctcoef,
                        &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                    );
                    (*h).quantf.dequant_4x4.expect("non-null function pointer")(
                        &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                        (*h).dequant4_mf[quant_cat as usize],
                        i_qp,
                    );
                    (*h).dctf.add4x4_idct.expect("non-null function pointer")(
                        p_fdec,
                        &raw mut dct4x4 as *mut crate::src::common::common::dctcoef,
                    );
                }
            }
            p += 1;
            i_qp = (*h).mb.i_chroma_qp;
        }
    }
}
pub unsafe extern "C" fn x264_8_macroblock_encode_p4x4(
    mut h: *mut crate::src::common::common::x264_t,
    mut i8: ::core::ffi::c_int,
) {
    unsafe {
        if (*h).sps.i_chroma_format_idc.is_444() {
            macroblock_encode_p4x4_internal(h, i8, 3i32);
        } else {
            macroblock_encode_p4x4_internal(h, i8, 1i32);
        };
    }
}
