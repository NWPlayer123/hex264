pub mod bitstream_h {
    #[inline]
    pub unsafe extern "C" fn bs_init(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut p_data: *mut ::core::ffi::c_void,
        mut i_data: ::core::ffi::c_int,
    ) {
        unsafe {
            let mut offset = (p_data as crate::stdlib::intptr_t & 3isize) as ::core::ffi::c_int;
            (*s).p_start = (p_data as *mut crate::stdlib::uint8_t).offset(-(offset as isize));
            (*s).p = (*s).p_start;
            (*s).p_end = (p_data as *mut crate::stdlib::uint8_t).offset(i_data as isize);
            (*s).i_left = (crate::osdep_h::WORD_SIZE - offset) * 8i32;
            if offset != 0 {
                (*s).cur_bits =
                    endian_fix32((*((*s).p as *mut crate::src::common::base::x264_union32_t)).i)
                        as crate::stdlib::uintptr_t;
                (*s).cur_bits >>= (4i32 - offset) * 8i32;
            } else {
                (*s).cur_bits = 0usize;
            };
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_flush(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            (*((*s).p as *mut crate::src::common::base::x264_union32_t)).i =
                endian_fix32(((*s).cur_bits << ((*s).i_left & 31i32)) as crate::stdlib::uint32_t);
            (*s).p = (*s)
                .p
                .offset((crate::osdep_h::WORD_SIZE - ((*s).i_left >> 3i32)) as isize);
            (*s).i_left = crate::osdep_h::WORD_SIZE * 8i32;
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_write(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut i_count: ::core::ffi::c_int,
        mut i_bits: crate::stdlib::uint32_t,
    ) {
        unsafe {
            if crate::osdep_h::WORD_SIZE == 8i32 {
                (*s).cur_bits = (*s).cur_bits << i_count | i_bits as crate::stdlib::uintptr_t;
                (*s).i_left -= i_count;
                if (*s).i_left <= 32i32 {
                    (*((*s).p as *mut crate::src::common::base::x264_union32_t)).i =
                        endian_fix((*s).cur_bits << (*s).i_left) as crate::stdlib::uint32_t;
                    (*s).i_left += 32i32;
                    (*s).p = (*s).p.offset(4isize);
                }
            } else if i_count < (*s).i_left {
                (*s).cur_bits = (*s).cur_bits << i_count | i_bits as crate::stdlib::uintptr_t;
                (*s).i_left -= i_count;
            } else {
                i_count -= (*s).i_left;
                (*s).cur_bits =
                    (*s).cur_bits << (*s).i_left | (i_bits >> i_count) as crate::stdlib::uintptr_t;
                (*((*s).p as *mut crate::src::common::base::x264_union32_t)).i =
                    endian_fix((*s).cur_bits) as crate::stdlib::uint32_t;
                (*s).p = (*s).p.offset(4isize);
                (*s).cur_bits = i_bits as crate::stdlib::uintptr_t;
                (*s).i_left = 32i32 - i_count;
            };
        }
    }
    use crate::src::encoder::cabac::osdep_h::endian_fix;
    use crate::src::encoder::cabac::osdep_h::endian_fix32;
}
pub mod cabac_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_cabac_pos(
        mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            return (((*cb).p.offset_from((*cb).p_start) as ::core::ffi::c_long
                + (*cb).i_bytes_outstanding as ::core::ffi::c_long)
                * 8i64
                + (*cb).i_queue as ::core::ffi::c_long) as ::core::ffi::c_int;
        }
    }
}
pub mod predict_h {
    pub static mut x264_mb_chroma_pred_mode_fix: [crate::stdlib::uint8_t; 7] = [
        crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_H as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_V as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_P as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::uint8_t,
    ];
    pub static mut x264_mb_pred_mode16x16_fix: [crate::stdlib::uint8_t; 7] = [
        crate::src::common::predict::I_PRED_16x16_V as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_H as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_P as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_DC as crate::stdlib::uint8_t,
    ];
    pub static mut x264_mb_pred_mode4x4_fix: [crate::stdlib::int8_t; 13] = [
        -1i8,
        crate::src::common::predict::I_PRED_4x4_V as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_H as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DC as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DDL as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DDR as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_VR as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_HD as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_VL as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_HU as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DC as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DC as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DC as crate::stdlib::int8_t,
    ];
}
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
    pub unsafe extern "C" fn x264_cabac_mvd_sum(
        mut mvdleft: *mut crate::stdlib::uint8_t,
        mut mvdtop: *mut crate::stdlib::uint8_t,
    ) -> crate::stdlib::uint16_t {
        unsafe {
            let mut amvd0 = *mvdleft.offset(0isize) as ::core::ffi::c_int
                + *mvdtop.offset(0isize) as ::core::ffi::c_int;
            let mut amvd1 = *mvdleft.offset(1isize) as ::core::ffi::c_int
                + *mvdtop.offset(1isize) as ::core::ffi::c_int;
            amvd0 = (amvd0 > 2i32) as ::core::ffi::c_int + (amvd0 > 32i32) as ::core::ffi::c_int;
            amvd1 = (amvd1 > 2i32) as ::core::ffi::c_int + (amvd1 > 32i32) as ::core::ffi::c_int;
            return (amvd0 + (amvd1 << 8i32)) as crate::stdlib::uint16_t;
        }
    }
}
pub mod macroblock_h {
    pub static mut x264_mb_type_list_table: [[[crate::stdlib::uint8_t; 2]; 2]; 19] = [
        [[0u8, 0u8], [0u8, 0u8]],
        [[0u8, 0u8], [0u8, 0u8]],
        [[0u8, 0u8], [0u8, 0u8]],
        [[0u8, 0u8], [0u8, 0u8]],
        [[1u8, 1u8], [0u8, 0u8]],
        [[0u8, 0u8], [0u8, 0u8]],
        [[1u8, 1u8], [0u8, 0u8]],
        [[0u8, 0u8], [0u8, 0u8]],
        [[1u8, 1u8], [0u8, 0u8]],
        [[1u8, 0u8], [0u8, 1u8]],
        [[1u8, 1u8], [0u8, 1u8]],
        [[0u8, 1u8], [1u8, 0u8]],
        [[0u8, 0u8], [1u8, 1u8]],
        [[0u8, 1u8], [1u8, 1u8]],
        [[1u8, 1u8], [1u8, 0u8]],
        [[1u8, 0u8], [1u8, 1u8]],
        [[1u8, 1u8], [1u8, 1u8]],
        [[0u8, 0u8], [0u8, 0u8]],
        [[0u8, 0u8], [0u8, 0u8]],
    ];
    pub static mut x264_mb_partition_listX_table: [[crate::stdlib::uint8_t; 17]; 2] = [
        [
            1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
        [
            0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
    ];
    pub static mut block_idx_x: [crate::stdlib::uint8_t; 16] = [
        0u8, 1u8, 0u8, 1u8, 2u8, 3u8, 2u8, 3u8, 0u8, 1u8, 0u8, 1u8, 2u8, 3u8, 2u8, 3u8,
    ];
    pub static mut block_idx_y: [crate::stdlib::uint8_t; 16] = [
        0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 1u8, 1u8, 2u8, 2u8, 3u8, 3u8, 2u8, 2u8, 3u8, 3u8,
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
    #[inline(always)]
    pub extern "C" fn pack8to16(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
        return a.wrapping_add(b << 8i32);
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_predict_intra4x4_mode(
        mut h: *mut crate::src::common::common::x264_t,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            let ma = (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[idx as usize] as ::core::ffi::c_int - 1i32) as usize]
                as ::core::ffi::c_int;
            let mb = (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[idx as usize] as ::core::ffi::c_int - 8i32) as usize]
                as ::core::ffi::c_int;
            let m = if (x264_mb_pred_mode4x4_fix[(ma + 1i32) as usize] as ::core::ffi::c_int)
                < x264_mb_pred_mode4x4_fix[(mb + 1i32) as usize] as ::core::ffi::c_int
            {
                x264_mb_pred_mode4x4_fix[(ma + 1i32) as usize] as ::core::ffi::c_int
            } else {
                x264_mb_pred_mode4x4_fix[(mb + 1i32) as usize] as ::core::ffi::c_int
            };
            if m < 0i32 {
                return crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int;
            }
            return m;
        }
    }
    pub static mut x264_transform_allowed: [crate::stdlib::uint8_t; 19] = [
        0u8, 0u8, 0u8, 0u8, 1u8, 2u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
        0u8,
    ];
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_transform_8x8_allowed(
        mut h: *mut crate::src::common::common::x264_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            if !(*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                .transform_8x8_mode
            {
                return 0i32;
            }
            if (*h).mb.i_type != crate::src::common::macroblock::P_8x8 as ::core::ffi::c_int {
                return x264_transform_allowed[(*h).mb.i_type as usize] as ::core::ffi::c_int;
            }
            return ((*(&raw mut (*h).mb.i_sub_partition
                as *mut crate::src::common::base::x264_union32_t))
                .i
                == (crate::src::common::macroblock::D_L0_8x8 as ::core::ffi::c_int * 0x1010101i32)
                    as crate::stdlib::uint32_t) as ::core::ffi::c_int;
        }
    }
    use crate::src::encoder::cabac::base_h::x264_scan8;
    use crate::src::encoder::cabac::predict_h::x264_mb_pred_mode4x4_fix;
}
pub mod osdep_h {
    #[inline(always)]
    pub extern "C" fn endian_fix32(mut x: crate::stdlib::uint32_t) -> crate::stdlib::uint32_t {
        return (x << 24i32)
            .wrapping_add(x << 8i32 & 0xff0000u32)
            .wrapping_add(x >> 8i32 & 0xff00u32)
            .wrapping_add(x >> 24i32);
    }
    #[inline(always)]
    pub extern "C" fn endian_fix64(mut x: crate::stdlib::uint64_t) -> crate::stdlib::uint64_t {
        return (endian_fix32((x >> 32i32) as crate::stdlib::uint32_t) as crate::stdlib::uint64_t)
            .wrapping_add(
                (endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uint64_t) << 32i32,
            );
    }
    #[inline(always)]
    pub extern "C" fn endian_fix(mut x: crate::stdlib::uintptr_t) -> crate::stdlib::uintptr_t {
        return if crate::osdep_h::WORD_SIZE == 8i32 {
            endian_fix64(x as crate::stdlib::uint64_t) as crate::stdlib::uintptr_t
        } else {
            endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uintptr_t
        };
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_ctz_4bit(mut x: crate::stdlib::uint32_t) -> ::core::ffi::c_int {
        unsafe {
            pub static mut lut: [crate::stdlib::uint8_t; 16] = [
                4u8, 0u8, 1u8, 0u8, 2u8, 0u8, 1u8, 0u8, 3u8, 0u8, 1u8, 0u8, 2u8, 0u8, 1u8, 0u8,
            ];
            return lut[x as usize] as ::core::ffi::c_int;
        }
    }
}
pub mod rectangle_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_macroblock_cache_rect(
        mut dst: *mut ::core::ffi::c_void,
        mut w: ::core::ffi::c_int,
        mut h: ::core::ffi::c_int,
        mut s: ::core::ffi::c_int,
        mut v: crate::stdlib::uint32_t,
    ) {
        unsafe {
            let mut d = dst as *mut crate::stdlib::uint8_t;
            let mut v2 = (if s >= 2i32 {
                v
            } else {
                v.wrapping_mul(0x101u32)
            }) as crate::stdlib::uint16_t;
            let mut v4 = if s >= 4i32 {
                v
            } else if s >= 2i32 {
                v.wrapping_mul(0x10001u32)
            } else {
                v.wrapping_mul(0x1010101u32)
            };
            let mut v8 = (v4 as crate::stdlib::uint64_t)
                .wrapping_add((v4 as crate::stdlib::uint64_t) << 32i32);
            s *= 8i32;
            if w == 2i32 {
                (*(d.offset((s * 0i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                if h == 1i32 {
                    return;
                }
                (*(d.offset((s * 1i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                if h == 2i32 {
                    return;
                }
                (*(d.offset((s * 2i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                (*(d.offset((s * 3i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
            } else if w == 4i32 {
                (*(d.offset((s * 0i32) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                if h == 1i32 {
                    return;
                }
                (*(d.offset((s * 1i32) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                if h == 2i32 {
                    return;
                }
                (*(d.offset((s * 2i32) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 3i32) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
            } else if w == 8i32 {
                if crate::osdep_h::WORD_SIZE == 8i32 {
                    (*(d.offset((s * 0i32) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    if h == 1i32 {
                        return;
                    }
                    (*(d.offset((s * 1i32) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    if h == 2i32 {
                        return;
                    }
                    (*(d.offset((s * 2i32) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 3i32) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                } else {
                    (*(d.offset((s * 0i32) as isize).offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 0i32) as isize).offset(4isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    if h == 1i32 {
                        return;
                    }
                    (*(d.offset((s * 1i32) as isize).offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 1i32) as isize).offset(4isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    if h == 2i32 {
                        return;
                    }
                    (*(d.offset((s * 2i32) as isize).offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 2i32) as isize).offset(4isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 3i32) as isize).offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 3i32) as isize).offset(4isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                }
            } else if w == 16i32 {
                '_c2rust_label: {
                    if h != 1i32 {
                    } else {
                        crate::stdlib::__assert_fail(
                            b"h != 1\0".as_ptr() as *const ::core::ffi::c_char,
                            b"./common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
                            82u32,
                            b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                if crate::osdep_h::WORD_SIZE == 8i32 {
                    loop {
                        (*(d.offset((s * 0i32) as isize).offset(0isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 0i32) as isize).offset(8isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 1i32) as isize).offset(0isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 1i32) as isize).offset(8isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        h -= 2i32;
                        d = d.offset((s * 2i32) as isize);
                        if !(h != 0) {
                            break;
                        }
                    }
                } else {
                    loop {
                        (*(d.offset(0isize) as *mut crate::src::common::base::x264_union32_t)).i =
                            v4;
                        (*(d.offset(4isize) as *mut crate::src::common::base::x264_union32_t)).i =
                            v4;
                        (*(d.offset(8isize) as *mut crate::src::common::base::x264_union32_t)).i =
                            v4;
                        (*(d.offset(12isize) as *mut crate::src::common::base::x264_union32_t)).i =
                            v4;
                        d = d.offset(s as isize);
                        h -= 1;
                        if !(h != 0) {
                            break;
                        }
                    }
                }
            } else {
                '_c2rust_label_0: {
                    crate::stdlib::__assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"./common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
                        118u32,
                        b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            };
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_macroblock_cache_mvd(
        mut h: *mut crate::src::common::common::x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut i_list: ::core::ffi::c_int,
        mut mvd: crate::stdlib::uint16_t,
    ) {
        unsafe {
            let mut mvd_cache =
                (&raw mut *(&raw mut (*h).mb.cache.mvd as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(i_list as isize) as *mut [crate::stdlib::uint8_t; 2])
                    .offset((crate::src::common::base::X264_SCAN8_0 + x + 8i32 * y) as isize)
                    as *mut ::core::ffi::c_void;
            if 0 == 0 || 0 == 0 {
                crate::src::common::rectangle::x264_8_cache_mvd_func_table
                    [(width + (height << 1i32) - 3i32) as usize]
                    .expect("non-null function pointer")(
                    mvd_cache, mvd as crate::stdlib::uint32_t
                );
            } else {
                x264_macroblock_cache_rect(
                    mvd_cache,
                    width * 2i32,
                    height,
                    2i32,
                    mvd as crate::stdlib::uint32_t,
                );
            };
        }
    }
}
use crate::src::encoder::cabac::base_h::x264_cabac_mvd_sum;
use crate::src::encoder::cabac::base_h::x264_scan8;
use crate::src::encoder::cabac::bitstream_h::bs_flush;
use crate::src::encoder::cabac::bitstream_h::bs_init;
use crate::src::encoder::cabac::bitstream_h::bs_write;
use crate::src::encoder::cabac::cabac_h::x264_cabac_pos;
use crate::src::encoder::cabac::macroblock_h::block_idx_x;
use crate::src::encoder::cabac::macroblock_h::block_idx_y;
use crate::src::encoder::cabac::macroblock_h::ctx_cat_plane;
use crate::src::encoder::cabac::macroblock_h::pack8to16;
use crate::src::encoder::cabac::macroblock_h::x264_mb_partition_listX_table;
use crate::src::encoder::cabac::macroblock_h::x264_mb_predict_intra4x4_mode;
use crate::src::encoder::cabac::macroblock_h::x264_mb_transform_8x8_allowed;
use crate::src::encoder::cabac::macroblock_h::x264_mb_type_list_table;
use crate::src::encoder::cabac::osdep_h::x264_ctz_4bit;
use crate::src::encoder::cabac::predict_h::x264_mb_chroma_pred_mode_fix;
use crate::src::encoder::cabac::predict_h::x264_mb_pred_mode4x4_fix;
use crate::src::encoder::cabac::predict_h::x264_mb_pred_mode16x16_fix;
use crate::src::encoder::cabac::rectangle_h::x264_macroblock_cache_mvd;
#[inline]
unsafe extern "C" fn cabac_mb_type_intra(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut ctx0: ::core::ffi::c_int,
    mut ctx1: ::core::ffi::c_int,
    mut ctx2: ::core::ffi::c_int,
    mut ctx3: ::core::ffi::c_int,
    mut ctx4: ::core::ffi::c_int,
    mut ctx5: ::core::ffi::c_int,
) {
    unsafe {
        if i_mb_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
        {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx0, 0i32);
        } else if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx0, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_flush(h, cb);
        } else {
            let mut i_pred = x264_mb_pred_mode16x16_fix[(*h).mb.i_intra16x16_pred_mode as usize]
                as ::core::ffi::c_int;
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx0, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_terminal_c(cb);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb,
                ctx1,
                ((*h).mb.i_cbp_luma != 0) as ::core::ffi::c_int,
            );
            if (*h).mb.i_cbp_chroma == 0i32 {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx2, 0i32);
            } else {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx2, 1i32);
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb,
                    ctx3,
                    (*h).mb.i_cbp_chroma >> 1i32,
                );
            }
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx4, i_pred >> 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx5, i_pred & 1i32);
        };
    }
}
unsafe extern "C" fn cabac_field_decoding_flag(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut ctx = 0i32;
        ctx += (*h).mb.field_decoding_flag & ((*h).mb.i_mb_x != 0) as ::core::ffi::c_int;
        ctx += ((*h).mb.i_mb_top_mbpair_xy >= 0i32
            && *(*h)
                .mb
                .slice_table
                .offset((*h).mb.i_mb_top_mbpair_xy as isize)
                == (*h).sh.i_first_mb
            && *(*h).mb.field.offset((*h).mb.i_mb_top_mbpair_xy as isize) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb,
            70i32 + ctx,
            (*h).mb.interlaced as ::core::ffi::c_int,
        );
        (*h).mb.field_decoding_flag = (*h).mb.interlaced as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn cabac_intra4x4_pred_mode(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_pred: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
) {
    unsafe {
        if i_pred == i_mode {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 68i32, 1i32);
        } else {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 68i32, 0i32);
            if i_mode > i_pred {
                i_mode -= 1;
            }
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 69i32, i_mode & 0x1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb,
                69i32,
                i_mode >> 1i32 & 0x1i32,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 69i32, i_mode >> 2i32);
        };
    }
}
unsafe extern "C" fn cabac_intra_chroma_pred_mode(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut ctx = 0i32;
        let mut i_mode =
            x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as ::core::ffi::c_int;
        if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
            && *(*h)
                .mb
                .chroma_pred_mode
                .offset((*h).mb.i_mb_left_xy[0usize] as isize) as ::core::ffi::c_int
                != 0i32
        {
            ctx += 1;
        }
        if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
            && *(*h)
                .mb
                .chroma_pred_mode
                .offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                != 0i32
        {
            ctx += 1;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb,
            64i32 + ctx,
            (i_mode > 0i32) as ::core::ffi::c_int,
        );
        if i_mode > 0i32 {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb,
                64i32 + 3i32,
                (i_mode > 1i32) as ::core::ffi::c_int,
            );
            if i_mode > 1i32 {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb,
                    64i32 + 3i32,
                    (i_mode > 2i32) as ::core::ffi::c_int,
                );
            }
        }
    }
}
unsafe extern "C" fn cabac_cbp_luma(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut cbp = (*h).mb.i_cbp_luma;
        let mut cbp_l = (*h).mb.cache.i_cbp_left;
        let mut cbp_t = (*h).mb.cache.i_cbp_top;
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb,
            76i32 - (cbp_l >> 1i32 & 1i32) - (cbp_t >> 1i32 & 2i32),
            cbp >> 0i32 & 1i32,
        );
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb,
            76i32 - (cbp >> 0i32 & 1i32) - (cbp_t >> 2i32 & 2i32),
            cbp >> 1i32 & 1i32,
        );
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb,
            76i32 - (cbp_l >> 3i32 & 1i32) - (cbp << 1i32 & 2i32),
            cbp >> 2i32 & 1i32,
        );
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb,
            76i32 - (cbp >> 2i32 & 1i32) - (cbp >> 0i32 & 2i32),
            cbp >> 3i32 & 1i32,
        );
    }
}
unsafe extern "C" fn cabac_cbp_chroma(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut ctx = 0i32;
        let mut cbp_a = (*h).mb.cache.i_cbp_left & 0x30i32;
        let mut cbp_b = (*h).mb.cache.i_cbp_top & 0x30i32;
        if cbp_a != 0 && (*h).mb.cache.i_cbp_left != -(1i32) {
            ctx += 1;
        }
        if cbp_b != 0 && (*h).mb.cache.i_cbp_top != -(1i32) {
            ctx += 2i32;
        }
        if (*h).mb.i_cbp_chroma == 0i32 {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 77i32 + ctx, 0i32);
        } else {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 77i32 + ctx, 1i32);
            ctx = 4i32;
            if cbp_a == 0x20i32 {
                ctx += 1;
            }
            if cbp_b == 0x20i32 {
                ctx += 2i32;
            }
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb,
                77i32 + ctx,
                (*h).mb.i_cbp_chroma >> 1i32,
            );
        };
    }
}
unsafe extern "C" fn cabac_qp_delta(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut i_dqp = (*h).mb.i_qp - (*h).mb.i_last_qp;
        if (*h).mb.i_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
            && *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) == 0
            && (*h).mb.i_qp > (*h).mb.i_last_qp
        {
            (*h).mb.i_qp = (*h).mb.i_last_qp;
            i_dqp = 0i32;
        }
        let mut ctx = ((*h).mb.i_last_dqp != 0
            && (*(*h).mb.type_0.offset((*h).mb.i_mb_prev_xy as isize) as ::core::ffi::c_int
                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || *(*h).mb.cbp.offset((*h).mb.i_mb_prev_xy as isize) as ::core::ffi::c_int
                    & 0x3fi32
                    != 0)) as ::core::ffi::c_int;
        if i_dqp != 0i32 {
            i_dqp *= 2i32;
            let mut val = 1i32 - i_dqp;
            if val < 0i32 {
                val = i_dqp;
            }
            val -= 1;
            if val >= crate::src::common::common::QP_MAX_SPEC
                && val != crate::src::common::common::QP_MAX_SPEC + 1i32
            {
                val = 2i32 * crate::src::common::common::QP_MAX_SPEC + 1i32 - val;
            }
            loop {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 60i32 + ctx, 1i32);
                ctx = 2i32 + (ctx >> 1i32);
                val -= 1;
                if !(val != 0) {
                    break;
                }
            }
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 60i32 + ctx, 0i32);
    }
}
pub unsafe extern "C" fn x264_8_cabac_mb_skip(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_skip: ::core::ffi::c_int,
) {
    unsafe {
        let mut ctx = (*h).mb.cache.i_neighbour_skip + 11i32;
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            ctx += 13i32;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(&raw mut (*h).cabac, ctx, b_skip);
    }
}
#[inline]
unsafe extern "C" fn cabac_subpartition_p(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_sub: ::core::ffi::c_int,
) {
    unsafe {
        if i_sub == crate::src::common::macroblock::D_L0_8x8 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 21i32, 1i32);
            return;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 21i32, 0i32);
        if i_sub == crate::src::common::macroblock::D_L0_8x4 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 22i32, 0i32);
        } else {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 22i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb,
                23i32,
                (i_sub == crate::src::common::macroblock::D_L0_4x8 as ::core::ffi::c_int)
                    as ::core::ffi::c_int,
            );
        };
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_subpartition_b(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_sub: ::core::ffi::c_int,
) {
    unsafe {
        if i_sub == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 36i32, 0i32);
            return;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 36i32, 1i32);
        if i_sub == crate::src::common::macroblock::D_BI_8x8 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 37i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 38i32, 0i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 39i32, 0i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 39i32, 0i32);
            return;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 37i32, 0i32);
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb,
            39i32,
            (i_sub == crate::src::common::macroblock::D_L1_8x8 as ::core::ffi::c_int)
                as ::core::ffi::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_transform_size(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut ctx = 399i32 + (*h).mb.cache.i_neighbour_transform_size;
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb,
            ctx,
            (*h).mb.transform_8x8 as ::core::ffi::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_ref_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut bframe: ::core::ffi::c_int,
) {
    unsafe {
        let mut ctx = 0i32;
        let i8 = x264_scan8[idx as usize] as ::core::ffi::c_int;
        let i_refa =
            (*h).mb.cache.ref_0[i_list as usize][(i8 - 1i32) as usize] as ::core::ffi::c_int;
        let i_refb =
            (*h).mb.cache.ref_0[i_list as usize][(i8 - 8i32) as usize] as ::core::ffi::c_int;
        if i_refa > 0i32 && (bframe == 0 || (*h).mb.cache.skip[(i8 - 1i32) as usize] == 0) {
            ctx += 1;
        }
        if i_refb > 0i32 && (bframe == 0 || (*h).mb.cache.skip[(i8 - 8i32) as usize] == 0) {
            ctx += 2i32;
        }
        let mut i_ref = (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as ::core::ffi::c_int;
        while i_ref > 0i32 {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 54i32 + ctx, 1i32);
            ctx = (ctx >> 2i32) + 4i32;
            i_ref -= 1;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 54i32 + ctx, 0i32);
    }
}
#[inline(never)]
unsafe extern "C" fn cabac_ref_p(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut idx: ::core::ffi::c_int,
) {
    unsafe {
        cabac_ref_internal(h, cb, 0i32, idx, 0i32);
    }
}
#[inline(never)]
unsafe extern "C" fn cabac_ref_b(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
) {
    unsafe {
        cabac_ref_internal(h, cb, i_list, idx, 1i32);
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_mvd_cpn(
    mut _h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut _i_list: ::core::ffi::c_int,
    mut _idx: ::core::ffi::c_int,
    mut l: ::core::ffi::c_int,
    mut mvd: ::core::ffi::c_int,
    mut ctx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ctxbase = if l != 0 { 47i32 } else { 40i32 };
        if mvd == 0i32 {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctxbase + ctx, 0i32);
            return 0i32;
        }
        let mut i_abs = crate::stdlib::abs(mvd);
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctxbase + ctx, 1i32);
        static mut ctxes: [crate::stdlib::uint8_t; 8] = [3u8, 4u8, 5u8, 6u8, 6u8, 6u8, 6u8, 6u8];
        if i_abs < 9i32 {
            let mut i = 1i32;
            while i < i_abs {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb,
                    ctxbase + ctxes[(i - 1i32) as usize] as ::core::ffi::c_int,
                    1i32,
                );
                i += 1;
            }
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb,
                ctxbase + ctxes[(i_abs - 1i32) as usize] as ::core::ffi::c_int,
                0i32,
            );
        } else {
            let mut i_0 = 1i32;
            while i_0 < 9i32 {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb,
                    ctxbase + ctxes[(i_0 - 1i32) as usize] as ::core::ffi::c_int,
                    1i32,
                );
                i_0 += 1;
            }
            crate::src::common::cabac::x264_8_cabac_encode_ue_bypass(cb, 3i32, i_abs - 9i32);
        }
        crate::src::common::cabac::x264_8_cabac_encode_bypass_c(cb, mvd >> 31i32);
        return if i_abs < 66i32 { i_abs } else { 66i32 };
    }
}
#[inline(never)]
unsafe extern "C" fn cabac_mvd(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) -> crate::stdlib::uint16_t {
    unsafe {
        let mut mvp = [0; 2];
        crate::src::common::mvpred::x264_8_mb_predict_mv(
            h,
            i_list,
            idx,
            width,
            &raw mut mvp as *mut crate::stdlib::int16_t,
        );
        let mut mdx = (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize][0usize]
            as ::core::ffi::c_int
            - mvp[0usize];
        let mut mdy = (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize][1usize]
            as ::core::ffi::c_int
            - mvp[1usize];
        let mut amvd = x264_cabac_mvd_sum(
            &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                as *mut [[crate::stdlib::uint8_t; 2]; 40])
                .offset(i_list as isize) as *mut [crate::stdlib::uint8_t; 2])
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(idx as isize)
                        as ::core::ffi::c_int
                        - 1i32) as isize,
                ) as *mut crate::stdlib::uint8_t,
            &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                as *mut [[crate::stdlib::uint8_t; 2]; 40])
                .offset(i_list as isize) as *mut [crate::stdlib::uint8_t; 2])
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(idx as isize)
                        as ::core::ffi::c_int
                        - 8i32) as isize,
                ) as *mut crate::stdlib::uint8_t,
        );
        mdx = cabac_mvd_cpn(
            h,
            cb,
            i_list,
            idx,
            0i32,
            mdx,
            amvd as ::core::ffi::c_int & 0xffi32,
        );
        mdy = cabac_mvd_cpn(
            h,
            cb,
            i_list,
            idx,
            1i32,
            mdy,
            amvd as ::core::ffi::c_int >> 8i32,
        );
        return pack8to16(
            mdx as crate::stdlib::uint32_t,
            mdy as crate::stdlib::uint32_t,
        ) as crate::stdlib::uint16_t;
    }
}
#[inline]
unsafe extern "C" fn cabac_8x8_mvd(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i: ::core::ffi::c_int,
) {
    unsafe {
        match (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int {
            3 => {
                let mut mvd = cabac_mvd(h, cb, 0i32, 4i32 * i, 2i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i) as usize] as ::core::ffi::c_int,
                    2i32,
                    2i32,
                    0i32,
                    mvd,
                );
            }
            1 => {
                let mut mvd_0 = cabac_mvd(h, cb, 0i32, 4i32 * i + 0i32, 2i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i + 0i32) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i + 0i32) as usize] as ::core::ffi::c_int,
                    2i32,
                    1i32,
                    0i32,
                    mvd_0,
                );
                let mut mvd_1 = cabac_mvd(h, cb, 0i32, 4i32 * i + 2i32, 2i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i + 2i32) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i + 2i32) as usize] as ::core::ffi::c_int,
                    2i32,
                    1i32,
                    0i32,
                    mvd_1,
                );
            }
            2 => {
                let mut mvd_2 = cabac_mvd(h, cb, 0i32, 4i32 * i + 0i32, 1i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i + 0i32) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i + 0i32) as usize] as ::core::ffi::c_int,
                    1i32,
                    2i32,
                    0i32,
                    mvd_2,
                );
                let mut mvd_3 = cabac_mvd(h, cb, 0i32, 4i32 * i + 1i32, 1i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i + 1i32) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i + 1i32) as usize] as ::core::ffi::c_int,
                    1i32,
                    2i32,
                    0i32,
                    mvd_3,
                );
            }
            0 => {
                let mut mvd_4 = cabac_mvd(h, cb, 0i32, 4i32 * i + 0i32, 1i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i + 0i32) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i + 0i32) as usize] as ::core::ffi::c_int,
                    1i32,
                    1i32,
                    0i32,
                    mvd_4,
                );
                let mut mvd_5 = cabac_mvd(h, cb, 0i32, 4i32 * i + 1i32, 1i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i + 1i32) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i + 1i32) as usize] as ::core::ffi::c_int,
                    1i32,
                    1i32,
                    0i32,
                    mvd_5,
                );
                let mut mvd_6 = cabac_mvd(h, cb, 0i32, 4i32 * i + 2i32, 1i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i + 2i32) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i + 2i32) as usize] as ::core::ffi::c_int,
                    1i32,
                    1i32,
                    0i32,
                    mvd_6,
                );
                let mut mvd_7 = cabac_mvd(h, cb, 0i32, 4i32 * i + 3i32, 1i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4i32 * i + 3i32) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4i32 * i + 3i32) as usize] as ::core::ffi::c_int,
                    1i32,
                    1i32,
                    0i32,
                    mvd_7,
                );
            }
            _ => {
                '_c2rust_label: {
                    crate::stdlib::__assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"encoder/cabac.c\0".as_ptr() as *const ::core::ffi::c_char,
                        377u32,
                        b"void cabac_8x8_mvd(x264_t *, x264_cabac_t *, int)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
            }
        };
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_mb_header_i(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut slice_type: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        if slice_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            let mut ctx = 0i32;
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                && (*h).mb.i_mb_type_left[0usize]
                    != crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            {
                ctx += 1;
            }
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
                && (*h).mb.i_mb_type_top
                    != crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            {
                ctx += 1;
            }
            cabac_mb_type_intra(
                h,
                cb,
                i_mb_type,
                3i32 + ctx,
                3i32 + 3i32,
                3i32 + 4i32,
                3i32 + 5i32,
                3i32 + 6i32,
                3i32 + 7i32,
            );
        } else if slice_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 14i32, 1i32);
            cabac_mb_type_intra(
                h,
                cb,
                i_mb_type,
                17i32 + 0i32,
                17i32 + 1i32,
                17i32 + 2i32,
                17i32 + 2i32,
                17i32 + 3i32,
                17i32 + 3i32,
            );
        } else if slice_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 3i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 4i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 5i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 5i32, 0i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 5i32, 1i32);
            cabac_mb_type_intra(
                h,
                cb,
                i_mb_type,
                32i32 + 0i32,
                32i32 + 1i32,
                32i32 + 2i32,
                32i32 + 2i32,
                32i32 + 3i32,
                32i32 + 3i32,
            );
        }
        if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            return;
        }
        if i_mb_type != crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int {
            let mut i = 0i32;
            if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).transform_8x8_mode
            {
                cabac_transform_size(h, cb);
            }
            let mut di = if (*h).mb.transform_8x8 { 4i32 } else { 1i32 };
            while i < 16i32 {
                let i_pred = x264_mb_predict_intra4x4_mode(h, i);
                let i_mode = x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                    [x264_scan8[i as usize] as usize]
                    as ::core::ffi::c_int
                    + 1i32) as usize] as ::core::ffi::c_int;
                cabac_intra4x4_pred_mode(cb, i_pred, i_mode);
                i += di;
            }
        }
        if chroma != 0 {
            cabac_intra_chroma_pred_mode(h, cb);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_mb_header_p(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        if i_mb_type == crate::src::common::macroblock::P_L0 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 14i32, 0i32);
            if (*h).mb.i_partition == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 15i32, 0i32);
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 16i32, 0i32);
                if (*h).mb.pic.i_fref[0usize] > 1i32 {
                    cabac_ref_p(h, cb, 0i32);
                }
                let mut mvd = cabac_mvd(h, cb, 0i32, 0i32, 4i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[0usize] as ::core::ffi::c_int,
                    block_idx_y[0usize] as ::core::ffi::c_int,
                    4i32,
                    4i32,
                    0i32,
                    mvd,
                );
            } else if (*h).mb.i_partition
                == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
            {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 15i32, 1i32);
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 17i32, 1i32);
                if (*h).mb.pic.i_fref[0usize] > 1i32 {
                    cabac_ref_p(h, cb, 0i32);
                    cabac_ref_p(h, cb, 8i32);
                }
                let mut mvd_0 = cabac_mvd(h, cb, 0i32, 0i32, 4i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[0usize] as ::core::ffi::c_int,
                    block_idx_y[0usize] as ::core::ffi::c_int,
                    4i32,
                    2i32,
                    0i32,
                    mvd_0,
                );
                let mut mvd_1 = cabac_mvd(h, cb, 0i32, 8i32, 4i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[8usize] as ::core::ffi::c_int,
                    block_idx_y[8usize] as ::core::ffi::c_int,
                    4i32,
                    2i32,
                    0i32,
                    mvd_1,
                );
            } else {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 15i32, 1i32);
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 17i32, 0i32);
                if (*h).mb.pic.i_fref[0usize] > 1i32 {
                    cabac_ref_p(h, cb, 0i32);
                    cabac_ref_p(h, cb, 4i32);
                }
                let mut mvd_2 = cabac_mvd(h, cb, 0i32, 0i32, 2i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[0usize] as ::core::ffi::c_int,
                    block_idx_y[0usize] as ::core::ffi::c_int,
                    2i32,
                    4i32,
                    0i32,
                    mvd_2,
                );
                let mut mvd_3 = cabac_mvd(h, cb, 0i32, 4i32, 2i32);
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[4usize] as ::core::ffi::c_int,
                    block_idx_y[4usize] as ::core::ffi::c_int,
                    2i32,
                    4i32,
                    0i32,
                    mvd_3,
                );
            }
        } else if i_mb_type == crate::src::common::macroblock::P_8x8 as ::core::ffi::c_int {
            let mut i = 0i32;
            let mut i_0 = 0i32;
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 14i32, 0i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 15i32, 0i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 16i32, 1i32);
            while i < 4i32 {
                cabac_subpartition_p(
                    cb,
                    (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int,
                );
                i += 1;
            }
            if (*h).mb.pic.i_fref[0usize] > 1i32 {
                cabac_ref_p(h, cb, 0i32);
                cabac_ref_p(h, cb, 4i32);
                cabac_ref_p(h, cb, 8i32);
                cabac_ref_p(h, cb, 12i32);
            }
            while i_0 < 4i32 {
                cabac_8x8_mvd(h, cb, i_0);
                i_0 += 1;
            }
        } else {
            cabac_mb_header_i(
                h,
                cb,
                i_mb_type,
                crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int,
                chroma,
            );
        };
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_mb_header_b(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut ctx = 0i32;
        if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
            && (*h).mb.i_mb_type_left[0usize]
                != crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
            && (*h).mb.i_mb_type_left[0usize]
                != crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int
        {
            ctx += 1;
        }
        if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
            && (*h).mb.i_mb_type_top != crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
            && (*h).mb.i_mb_type_top
                != crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int
        {
            ctx += 1;
        }
        if i_mb_type == crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + ctx, 0i32);
            return;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + ctx, 1i32);
        if i_mb_type == crate::src::common::macroblock::B_8x8 as ::core::ffi::c_int {
            let mut i = 0i32;
            let mut i_2 = 0i32;
            let mut i_3 = 0i32;
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 3i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 4i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 5i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 5i32, 1i32);
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, 27i32 + 5i32, 1i32);
            while i < 4i32 {
                cabac_subpartition_b(
                    cb,
                    (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int,
                );
                i += 1;
            }
            if (*h).mb.pic.i_fref[0usize] > 1i32 {
                let mut i_0 = 0i32;
                while i_0 < 4i32 {
                    if x264_mb_partition_listX_table[0usize]
                        [(*h).mb.i_sub_partition[i_0 as usize] as usize]
                        != 0
                    {
                        cabac_ref_b(h, cb, 0i32, 4i32 * i_0);
                    }
                    i_0 += 1;
                }
            }
            if (*h).mb.pic.i_fref[1usize] > 1i32 {
                let mut i_1 = 0i32;
                while i_1 < 4i32 {
                    if x264_mb_partition_listX_table[1usize]
                        [(*h).mb.i_sub_partition[i_1 as usize] as usize]
                        != 0
                    {
                        cabac_ref_b(h, cb, 1i32, 4i32 * i_1);
                    }
                    i_1 += 1;
                }
            }
            while i_2 < 4i32 {
                if x264_mb_partition_listX_table[0usize]
                    [(*h).mb.i_sub_partition[i_2 as usize] as usize]
                    != 0
                {
                    let mut mvd = cabac_mvd(h, cb, 0i32, 4i32 * i_2, 2i32);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[(4i32 * i_2) as usize] as ::core::ffi::c_int,
                        block_idx_y[(4i32 * i_2) as usize] as ::core::ffi::c_int,
                        2i32,
                        2i32,
                        0i32,
                        mvd,
                    );
                }
                i_2 += 1;
            }
            while i_3 < 4i32 {
                if x264_mb_partition_listX_table[1usize]
                    [(*h).mb.i_sub_partition[i_3 as usize] as usize]
                    != 0
                {
                    let mut mvd_0 = cabac_mvd(h, cb, 1i32, 4i32 * i_3, 2i32);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[(4i32 * i_3) as usize] as ::core::ffi::c_int,
                        block_idx_y[(4i32 * i_3) as usize] as ::core::ffi::c_int,
                        2i32,
                        2i32,
                        1i32,
                        mvd_0,
                    );
                }
                i_3 += 1;
            }
        } else if i_mb_type >= crate::src::common::macroblock::B_L0_L0 as ::core::ffi::c_int
            && i_mb_type <= crate::src::common::macroblock::B_BI_BI as ::core::ffi::c_int
        {
            let mut i_list = 0i32;
            static mut i_mb_bits: [crate::stdlib::uint8_t; 27] = [
                0x31u8, 0x29u8, 0x4u8, 0x35u8, 0x2du8, 0u8, 0x43u8, 0x63u8, 0u8, 0x3du8, 0x2fu8,
                0u8, 0x39u8, 0x25u8, 0x6u8, 0x53u8, 0x73u8, 0u8, 0x4bu8, 0x6bu8, 0u8, 0x5bu8,
                0x7bu8, 0u8, 0x47u8, 0x67u8, 0x21u8,
            ];
            let idx = (i_mb_type - crate::src::common::macroblock::B_L0_L0 as ::core::ffi::c_int)
                * 3i32
                + ((*h).mb.i_partition
                    - crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int);
            let mut bits = i_mb_bits[idx as usize] as ::core::ffi::c_int;
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb,
                27i32 + 3i32,
                bits & 1i32,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb,
                27i32 + 5i32 - (bits & 1i32),
                bits >> 1i32 & 1i32,
            );
            bits >>= 2i32;
            if bits != 1i32 {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb,
                    27i32 + 5i32,
                    bits & 1i32,
                );
                bits >>= 1i32;
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb,
                    27i32 + 5i32,
                    bits & 1i32,
                );
                bits >>= 1i32;
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb,
                    27i32 + 5i32,
                    bits & 1i32,
                );
                bits >>= 1i32;
                if bits != 1i32 {
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                        cb,
                        27i32 + 5i32,
                        bits & 1i32,
                    );
                }
            }
            let mut b_list = &raw const *(&raw const x264_mb_type_list_table
                as *const [[crate::stdlib::uint8_t; 2]; 2])
                .offset(i_mb_type as isize)
                as *const [crate::stdlib::uint8_t; 2];
            if (*h).mb.pic.i_fref[0usize] > 1i32 {
                if (*b_list.offset(0isize))[0usize] != 0 {
                    cabac_ref_b(h, cb, 0i32, 0i32);
                }
                if (*b_list.offset(0isize))[1usize] as ::core::ffi::c_int != 0
                    && (*h).mb.i_partition
                        != crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                {
                    cabac_ref_b(
                        h,
                        cb,
                        0i32,
                        8i32 >> ((*h).mb.i_partition
                            == crate::src::common::macroblock::D_8x16 as ::core::ffi::c_int)
                            as ::core::ffi::c_int,
                    );
                }
            }
            if (*h).mb.pic.i_fref[1usize] > 1i32 {
                if (*b_list.offset(1isize))[0usize] != 0 {
                    cabac_ref_b(h, cb, 1i32, 0i32);
                }
                if (*b_list.offset(1isize))[1usize] as ::core::ffi::c_int != 0
                    && (*h).mb.i_partition
                        != crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                {
                    cabac_ref_b(
                        h,
                        cb,
                        1i32,
                        8i32 >> ((*h).mb.i_partition
                            == crate::src::common::macroblock::D_8x16 as ::core::ffi::c_int)
                            as ::core::ffi::c_int,
                    );
                }
            }
            while i_list < 2i32 {
                if (*h).mb.i_partition
                    == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                {
                    if (*b_list.offset(i_list as isize))[0usize] != 0 {
                        let mut mvd_1 = cabac_mvd(h, cb, i_list, 0i32, 4i32);
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[0usize] as ::core::ffi::c_int,
                            block_idx_y[0usize] as ::core::ffi::c_int,
                            4i32,
                            4i32,
                            i_list,
                            mvd_1,
                        );
                    }
                } else if (*h).mb.i_partition
                    == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
                {
                    if (*b_list.offset(i_list as isize))[0usize] != 0 {
                        let mut mvd_2 = cabac_mvd(h, cb, i_list, 0i32, 4i32);
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[0usize] as ::core::ffi::c_int,
                            block_idx_y[0usize] as ::core::ffi::c_int,
                            4i32,
                            2i32,
                            i_list,
                            mvd_2,
                        );
                    }
                    if (*b_list.offset(i_list as isize))[1usize] != 0 {
                        let mut mvd_3 = cabac_mvd(h, cb, i_list, 8i32, 4i32);
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[8usize] as ::core::ffi::c_int,
                            block_idx_y[8usize] as ::core::ffi::c_int,
                            4i32,
                            2i32,
                            i_list,
                            mvd_3,
                        );
                    }
                } else {
                    if (*b_list.offset(i_list as isize))[0usize] != 0 {
                        let mut mvd_4 = cabac_mvd(h, cb, i_list, 0i32, 2i32);
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[0usize] as ::core::ffi::c_int,
                            block_idx_y[0usize] as ::core::ffi::c_int,
                            2i32,
                            4i32,
                            i_list,
                            mvd_4,
                        );
                    }
                    if (*b_list.offset(i_list as isize))[1usize] != 0 {
                        let mut mvd_5 = cabac_mvd(h, cb, i_list, 4i32, 2i32);
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[4usize] as ::core::ffi::c_int,
                            block_idx_y[4usize] as ::core::ffi::c_int,
                            2i32,
                            4i32,
                            i_list,
                            mvd_5,
                        );
                    }
                }
                i_list += 1;
            }
        } else {
            cabac_mb_header_i(
                h,
                cb,
                i_mb_type,
                crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int,
                chroma,
            );
        };
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_cbf_ctxidxinc(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_cat: ::core::ffi::c_int,
    mut i_idx: ::core::ffi::c_int,
    mut b_intra: ::core::ffi::c_int,
    mut b_dc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        static mut base_ctx: [crate::stdlib::uint16_t; 14] = [
            85u16, 89u16, 93u16, 97u16, 101u16, 1012u16, 460u16, 464u16, 468u16, 1016u16, 472u16,
            476u16, 480u16, 1020u16,
        ];
        if b_dc != 0 {
            i_idx -= crate::src::common::base::LUMA_DC;
            if i_cat == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int {
                let mut i_nza = if (*h).mb.cache.i_cbp_left != -(1i32) {
                    (*h).mb.cache.i_cbp_left >> 8i32 + i_idx & 1i32
                } else {
                    b_intra
                };
                let mut i_nzb = if (*h).mb.cache.i_cbp_top != -(1i32) {
                    (*h).mb.cache.i_cbp_top >> 8i32 + i_idx & 1i32
                } else {
                    b_intra
                };
                return base_ctx[i_cat as usize] as ::core::ffi::c_int + 2i32 * i_nzb + i_nza;
            } else {
                let mut i_nza_0 = (*h).mb.cache.i_cbp_left >> 8i32 + i_idx & 1i32;
                let mut i_nzb_0 = (*h).mb.cache.i_cbp_top >> 8i32 + i_idx & 1i32;
                return base_ctx[i_cat as usize] as ::core::ffi::c_int + 2i32 * i_nzb_0 + i_nza_0;
            }
        } else {
            let mut i_nza_1 = (*h).mb.cache.non_zero_count
                [(x264_scan8[i_idx as usize] as ::core::ffi::c_int - 1i32) as usize]
                as ::core::ffi::c_int;
            let mut i_nzb_1 = (*h).mb.cache.non_zero_count
                [(x264_scan8[i_idx as usize] as ::core::ffi::c_int - 8i32) as usize]
                as ::core::ffi::c_int;
            if 0 != 0 && b_intra == 0 {
                return base_ctx[i_cat as usize] as ::core::ffi::c_int
                    + (2i32 * i_nzb_1 + i_nza_1 & 0x7fi32);
            } else {
                i_nza_1 &= 0x7fi32 + (b_intra << 7i32);
                i_nzb_1 &= 0x7fi32 + (b_intra << 7i32);
                return base_ctx[i_cat as usize] as ::core::ffi::c_int
                    + 2i32 * (i_nzb_1 != 0) as ::core::ffi::c_int
                    + (i_nza_1 != 0) as ::core::ffi::c_int;
            }
        };
    }
}
static mut coeff_abs_level1_ctx: [crate::stdlib::uint8_t; 8] =
    [1u8, 2u8, 3u8, 4u8, 0u8, 0u8, 0u8, 0u8];
static mut coeff_abs_levelgt1_ctx: [crate::stdlib::uint8_t; 8] =
    [5u8, 5u8, 5u8, 5u8, 6u8, 7u8, 8u8, 9u8];
static mut coeff_abs_levelgt1_ctx_chroma_dc: [crate::stdlib::uint8_t; 8] =
    [5u8, 5u8, 5u8, 5u8, 6u8, 7u8, 8u8, 8u8];
static mut coeff_abs_level_transition: [[crate::stdlib::uint8_t; 8]; 2] = [
    [1u8, 2u8, 3u8, 3u8, 4u8, 5u8, 6u8, 7u8],
    [4u8, 4u8, 4u8, 4u8, 5u8, 6u8, 7u8, 7u8],
];
#[inline(always)]
unsafe extern "C" fn cabac_block_residual_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut crate::src::common::common::dctcoef,
    mut chroma422dc: ::core::ffi::c_int,
) {
    unsafe {
        let mut coeffs = [0; 64];
        let mut ctx_sig = crate::src::common::tables::x264_significant_coeff_flag_offset
            [(*h).mb.interlaced as usize][ctx_block_cat as usize]
            as ::core::ffi::c_int;
        let mut ctx_last = crate::src::common::tables::x264_last_coeff_flag_offset
            [(*h).mb.interlaced as usize][ctx_block_cat as usize]
            as ::core::ffi::c_int;
        let mut ctx_level = crate::src::common::tables::x264_coeff_abs_level_m1_offset
            [ctx_block_cat as usize] as ::core::ffi::c_int;
        let mut coeff_idx = -(1i32);
        let mut last =
            (*h).quantf.coeff_last[ctx_block_cat as usize].expect("non-null function pointer")(l);
        let mut levelgt1_ctx = if chroma422dc != 0 {
            &raw const coeff_abs_levelgt1_ctx_chroma_dc as *const crate::stdlib::uint8_t
        } else {
            &raw const coeff_abs_levelgt1_ctx as *const crate::stdlib::uint8_t
        };
        if chroma422dc != 0 {
            loop {
                let mut count_m1 = 7i32;
                let mut i = 0i32;
                if *l.offset(i as isize) != 0 {
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i as isize);
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                        cb,
                        ctx_sig
                            + crate::src::common::tables::x264_coeff_flag_offset_chroma_422_dc
                                [i as usize] as ::core::ffi::c_int,
                        1i32,
                    );
                    if i == last {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctx_last
                                + crate::src::common::tables::x264_coeff_flag_offset_chroma_422_dc
                                    [i as usize]
                                    as ::core::ffi::c_int,
                            1i32,
                        );
                        break;
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctx_last
                                + crate::src::common::tables::x264_coeff_flag_offset_chroma_422_dc
                                    [i as usize]
                                    as ::core::ffi::c_int,
                            0i32,
                        );
                    }
                } else {
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                        cb,
                        ctx_sig
                            + crate::src::common::tables::x264_coeff_flag_offset_chroma_422_dc
                                [i as usize] as ::core::ffi::c_int,
                        0i32,
                    );
                }
                i += 1;
                if !(i == count_m1) {
                    continue;
                }
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i as isize);
                break;
            }
        } else {
            let mut count_m1_0 = crate::src::common::tables::x264_count_cat_m1
                [ctx_block_cat as usize] as ::core::ffi::c_int;
            if count_m1_0 == 63i32 {
                let mut sig_offset =
                    &raw const *(&raw const crate::src::common::tables::x264_significant_coeff_flag_offset_8x8
                        as *const [crate::stdlib::uint8_t; 64])
                        .offset((*h).mb.interlaced as isize)
                        as *const crate::stdlib::uint8_t;
                loop {
                    let mut i_0 = 0i32;
                    if *l.offset(i_0 as isize) != 0 {
                        coeff_idx += 1;
                        coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctx_sig + *sig_offset.offset(i_0 as isize) as ::core::ffi::c_int,
                            1i32,
                        );
                        if i_0 == last {
                            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                cb,
                                ctx_last
                                    + crate::src::common::tables::x264_last_coeff_flag_offset_8x8
                                        [i_0 as usize]
                                        as ::core::ffi::c_int,
                                1i32,
                            );
                            break;
                        } else {
                            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                cb,
                                ctx_last
                                    + crate::src::common::tables::x264_last_coeff_flag_offset_8x8
                                        [i_0 as usize]
                                        as ::core::ffi::c_int,
                                0i32,
                            );
                        }
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctx_sig + *sig_offset.offset(i_0 as isize) as ::core::ffi::c_int,
                            0i32,
                        );
                    }
                    i_0 += 1;
                    if !(i_0 == count_m1_0) {
                        continue;
                    }
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                    break;
                }
            } else {
                loop {
                    let mut i_1 = 0i32;
                    if *l.offset(i_1 as isize) != 0 {
                        coeff_idx += 1;
                        coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctx_sig + i_1,
                            1i32,
                        );
                        if i_1 == last {
                            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                cb,
                                ctx_last + i_1,
                                1i32,
                            );
                            break;
                        } else {
                            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                cb,
                                ctx_last + i_1,
                                0i32,
                            );
                        }
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctx_sig + i_1,
                            0i32,
                        );
                    }
                    i_1 += 1;
                    if !(i_1 == count_m1_0) {
                        continue;
                    }
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                    break;
                }
            }
        }
        loop {
            let mut node_ctx = 0i32;
            let mut coeff = coeffs[coeff_idx as usize] as ::core::ffi::c_int;
            let mut abs_coeff = crate::stdlib::abs(coeff);
            let mut coeff_sign = coeff >> 31i32;
            let mut ctx = coeff_abs_level1_ctx[node_ctx as usize] as ::core::ffi::c_int + ctx_level;
            if abs_coeff > 1i32 {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx, 1i32);
                ctx = *levelgt1_ctx.offset(node_ctx as isize) as ::core::ffi::c_int + ctx_level;
                let mut i_2 = (if abs_coeff < 15i32 { abs_coeff } else { 15i32 }) - 2i32;
                while i_2 > 0i32 {
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx, 1i32);
                    i_2 -= 1;
                }
                if abs_coeff < 15i32 {
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx, 0i32);
                } else {
                    crate::src::common::cabac::x264_8_cabac_encode_ue_bypass(
                        cb,
                        0i32,
                        abs_coeff - 15i32,
                    );
                }
                node_ctx =
                    coeff_abs_level_transition[1usize][node_ctx as usize] as ::core::ffi::c_int;
            } else {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(cb, ctx, 0i32);
                node_ctx =
                    coeff_abs_level_transition[0usize][node_ctx as usize] as ::core::ffi::c_int;
            }
            crate::src::common::cabac::x264_8_cabac_encode_bypass_c(cb, coeff_sign);
            coeff_idx -= 1;
            if !(coeff_idx >= 0i32) {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_cabac_block_residual_c(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        cabac_block_residual_internal(h, cb, ctx_block_cat, l, 0i32);
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_block_residual(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        x264_8_cabac_block_residual_c(h, cb, ctx_block_cat, l);
    }
}
unsafe extern "C" fn cabac_block_residual_422_dc(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut _ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        cabac_block_residual_internal(
            h,
            cb,
            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
            l,
            1i32,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_write_cabac_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut plane_count: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let i_mb_type = (*h).mb.i_type;
        let i_mb_pos_start = x264_cabac_pos(cb);
        if (*h).sh.mbaff
            && ((*h).mb.i_mb_y & 1i32 == 0
                || (*(*h)
                    .mb
                    .type_0
                    .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                    as ::core::ffi::c_int
                    == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                    || *(*h)
                        .mb
                        .type_0
                        .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int))
        {
            cabac_field_decoding_flag(h, cb);
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            cabac_mb_header_p(h, cb, i_mb_type, chroma);
        } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            cabac_mb_header_b(h, cb, i_mb_type, chroma);
        } else {
            cabac_mb_header_i(
                h,
                cb,
                i_mb_type,
                crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int,
                chroma,
            );
        }
        let mut i_mb_pos_tex = x264_cabac_pos(cb);
        (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
        if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            let mut s = crate::src::common::bitstream::bs_s {
                p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                cur_bits: 0,
                i_left: 0,
                i_bits_encoded: 0,
            };
            let mut p = 0i32;
            bs_init(
                &raw mut s,
                (*cb).p as *mut ::core::ffi::c_void,
                (*cb).p_end.offset_from((*cb).p) as ::core::ffi::c_int,
            );
            while p < plane_count {
                let mut i = 0i32;
                while i < 256i32 {
                    bs_write(
                        &raw mut s,
                        crate::internal::BIT_DEPTH,
                        *(*h).mb.pic.p_fenc[p as usize].offset(i as isize)
                            as crate::stdlib::uint32_t,
                    );
                    i += 1;
                }
                p += 1;
            }
            if chroma != 0 {
                let mut ch = 1i32;
                while ch < 3i32 {
                    let mut i_0 = 0i32;
                    while i_0
                        < 16i32
                            >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                    {
                        let mut j = 0i32;
                        while j < 8i32 {
                            bs_write(
                                &raw mut s,
                                crate::internal::BIT_DEPTH,
                                *(*h).mb.pic.p_fenc[ch as usize].offset(
                                    (i_0 * crate::src::common::common::FENC_STRIDE + j) as isize,
                                ) as crate::stdlib::uint32_t,
                            );
                            j += 1;
                        }
                        i_0 += 1;
                    }
                    ch += 1;
                }
            }
            bs_flush(&raw mut s);
            (*cb).p = s.p;
            crate::src::common::cabac::x264_8_cabac_encode_init_core(cb);
            (*h).stat.frame.i_tex_bits += x264_cabac_pos(cb) - i_mb_pos_tex;
            return;
        }
        if i_mb_type != crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int {
            cabac_cbp_luma(h, cb);
            if chroma != 0 {
                cabac_cbp_chroma(h, cb);
            }
        }
        if x264_mb_transform_8x8_allowed(h) != 0 && (*h).mb.i_cbp_luma != 0 {
            cabac_transform_size(h, cb);
        }
        if (*h).mb.i_cbp_luma != 0
            || chroma != 0 && (*h).mb.i_cbp_chroma != 0
            || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
        {
            let b_intra = (i_mb_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            cabac_qp_delta(h, cb);
            if i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int {
                let mut p_0 = 0i32;
                while p_0 < plane_count {
                    let mut ctxidxinc = cabac_cbf_ctxidxinc(
                        h,
                        ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_DC
                            as ::core::ffi::c_int as usize][p_0 as usize]
                            as ::core::ffi::c_int,
                        48i32 + p_0,
                        1i32,
                        1i32,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8[(48i32 + p_0) as usize] as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb, ctxidxinc, 1i32,
                        );
                        cabac_block_residual(
                            h,
                            cb,
                            ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_DC
                                as ::core::ffi::c_int
                                as usize][p_0 as usize]
                                as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.luma16x16_dc
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset(p_0 as isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb, ctxidxinc, 0i32,
                        );
                    }
                    if (*h).mb.i_cbp_luma != 0 {
                        let mut i_1 = p_0 * 16i32;
                        while i_1 < p_0 * 16i32 + 16i32 {
                            let mut ctxidxinc_0 = cabac_cbf_ctxidxinc(
                                h,
                                ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_AC
                                    as ::core::ffi::c_int
                                    as usize][p_0 as usize]
                                    as ::core::ffi::c_int,
                                i_1,
                                1i32,
                                0i32,
                            );
                            if (*h).mb.cache.non_zero_count[x264_scan8[i_1 as usize] as usize] != 0
                            {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb,
                                    ctxidxinc_0,
                                    1i32,
                                );
                                cabac_block_residual(
                                    h,
                                    cb,
                                    ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_AC
                                        as ::core::ffi::c_int
                                        as usize][p_0 as usize]
                                        as ::core::ffi::c_int,
                                    (&raw mut *(&raw mut (*h).dct.luma4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(i_1 as isize)
                                        as *mut crate::src::common::common::dctcoef)
                                        .offset(1isize),
                                );
                            } else {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb,
                                    ctxidxinc_0,
                                    0i32,
                                );
                            }
                            i_1 += 1;
                        }
                    }
                    p_0 += 1;
                }
            } else if (*h).mb.transform_8x8 {
                if plane_count == 3i32 {
                    let mut nnzbak = [[0; 8]; 3];
                    let mut p_1 = 0i32;
                    if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_left_xy[0usize] as isize)
                            == 0
                        && *(*h).mb.cbp.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                            as ::core::ffi::c_int
                            & 0x1000i32
                            == 0
                    {
                        nnzbak[0usize][0usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 0i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 0i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[0usize][1usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 0i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 0i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[1usize][0usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 1i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 1i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[1usize][1usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 1i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 1i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[2usize][0usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 2i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 2i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[2usize][1usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 2i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 2i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                    }
                    if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_left_xy[1usize] as isize)
                            == 0
                        && *(*h).mb.cbp.offset((*h).mb.i_mb_left_xy[1usize] as isize)
                            as ::core::ffi::c_int
                            & 0x1000i32
                            == 0
                    {
                        nnzbak[0usize][2usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 0i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 0i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[0usize][3usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 0i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 0i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[1usize][2usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 1i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 1i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[1usize][3usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 1i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 1i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[2usize][2usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 2i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 2i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                        nnzbak[2usize][3usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16i32 * 2i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 2i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = 0u8;
                    }
                    if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_top_xy as isize)
                            == 0
                        && *(*h).mb.cbp.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                            & 0x1000i32
                            == 0
                    {
                        (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                            .offset(0isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(4isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*((&raw mut (*h).mb.cache.non_zero_count
                            as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 0i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 0i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                        (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                            .offset(1isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(4isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*((&raw mut (*h).mb.cache.non_zero_count
                            as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 1i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 1i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                        (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                            .offset(2isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(4isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*((&raw mut (*h).mb.cache.non_zero_count
                            as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 2i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 2i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0u32;
                    }
                    while p_1 < 3i32 {
                        let mut i_2 = 0i32;
                        let mut msk = (*h).mb.i_cbp_luma;
                        while msk != 0 && {
                            let mut skip = 0;
                            skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                            i_2 += skip;
                            msk >>= skip + 1i32;
                            1i32 != 0
                        } {
                            let mut ctxidxinc_1 = cabac_cbf_ctxidxinc(
                                h,
                                ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_8x8
                                    as ::core::ffi::c_int
                                    as usize][p_1 as usize]
                                    as ::core::ffi::c_int,
                                i_2 * 4i32 + p_1 * 16i32,
                                b_intra,
                                0i32,
                            );
                            if (*h).mb.cache.non_zero_count
                                [x264_scan8[(i_2 * 4i32 + p_1 * 16i32) as usize] as usize]
                                != 0
                            {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb,
                                    ctxidxinc_1,
                                    1i32,
                                );
                                cabac_block_residual(
                                    h,
                                    cb,
                                    ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_8x8
                                        as ::core::ffi::c_int
                                        as usize][p_1 as usize]
                                        as ::core::ffi::c_int,
                                    &raw mut *(&raw mut (*h).dct.luma8x8
                                        as *mut [crate::src::common::common::dctcoef; 64])
                                        .offset((i_2 + p_1 * 4i32) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                            } else {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb,
                                    ctxidxinc_1,
                                    0i32,
                                );
                            }
                            i_2 += 1;
                        }
                        p_1 += 1;
                    }
                    if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_left_xy[0usize] as isize)
                            == 0
                        && *(*h).mb.cbp.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                            as ::core::ffi::c_int
                            & 0x1000i32
                            == 0
                    {
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 0i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[0usize][0usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 0i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[0usize][1usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 1i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[1usize][0usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 1i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[1usize][1usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 2i32 + 0i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[2usize][0usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 2i32 + 2i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[2usize][1usize];
                    }
                    if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_left_xy[1usize] as isize)
                            == 0
                        && *(*h).mb.cbp.offset((*h).mb.i_mb_left_xy[1usize] as isize)
                            as ::core::ffi::c_int
                            & 0x1000i32
                            == 0
                    {
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 0i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[0usize][2usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 0i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[0usize][3usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 1i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[1usize][2usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 1i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[1usize][3usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 2i32 + 8i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[2usize][2usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 * 2i32 + 10i32) as usize]
                            as ::core::ffi::c_int
                            - 1i32)
                            as usize] = nnzbak[2usize][3usize];
                    }
                    if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_top_xy as isize)
                            == 0
                        && *(*h).mb.cbp.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                            & 0x1000i32
                            == 0
                    {
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 0i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i =
                            (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                                .offset(0isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(4isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 1i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i =
                            (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                                .offset(1isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(4isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((16i32 * 2i32) as isize)
                                    as ::core::ffi::c_int
                                    - 8i32) as isize,
                            )
                            as *mut crate::src::common::base::x264_union32_t))
                            .i =
                            (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                                .offset(2isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(4isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i;
                    }
                } else {
                    let mut i_3 = 0i32;
                    let mut msk_0 = (*h).mb.i_cbp_luma;
                    while msk_0 != 0 && {
                        let mut skip_0 = 0;
                        skip_0 = x264_ctz_4bit(msk_0 as crate::stdlib::uint32_t);
                        i_3 += skip_0;
                        msk_0 >>= skip_0 + 1i32;
                        1i32 != 0
                    } {
                        cabac_block_residual(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_LUMA_8x8 as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.luma8x8
                                as *mut [crate::src::common::common::dctcoef; 64])
                                .offset(i_3 as isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                        i_3 += 1;
                    }
                }
            } else {
                let mut p_2 = 0i32;
                while p_2 < plane_count {
                    let mut i8x8 = 0i32;
                    let mut msk_1 = (*h).mb.i_cbp_luma;
                    while msk_1 != 0 && {
                        let mut skip_1 = 0;
                        skip_1 = x264_ctz_4bit(msk_1 as crate::stdlib::uint32_t);
                        i8x8 += skip_1;
                        msk_1 >>= skip_1 + 1i32;
                        1i32 != 0
                    } {
                        let mut i_4 = 0i32;
                        while i_4 < 4i32 {
                            let mut ctxidxinc_2 = cabac_cbf_ctxidxinc(
                                h,
                                ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_4x4
                                    as ::core::ffi::c_int
                                    as usize][p_2 as usize]
                                    as ::core::ffi::c_int,
                                i_4 + i8x8 * 4i32 + p_2 * 16i32,
                                b_intra,
                                0i32,
                            );
                            if (*h).mb.cache.non_zero_count
                                [x264_scan8[(i_4 + i8x8 * 4i32 + p_2 * 16i32) as usize] as usize]
                                != 0
                            {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb,
                                    ctxidxinc_2,
                                    1i32,
                                );
                                cabac_block_residual(
                                    h,
                                    cb,
                                    ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_4x4
                                        as ::core::ffi::c_int
                                        as usize][p_2 as usize]
                                        as ::core::ffi::c_int,
                                    &raw mut *(&raw mut (*h).dct.luma4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset((i_4 + i8x8 * 4i32 + p_2 * 16i32) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                            } else {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb,
                                    ctxidxinc_2,
                                    0i32,
                                );
                            }
                            i_4 += 1;
                        }
                        i8x8 += 1;
                    }
                    p_2 += 1;
                }
            }
            if chroma != 0 && (*h).mb.i_cbp_chroma != 0 {
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                {
                    let mut ctxidxinc_3 = cabac_cbf_ctxidxinc(
                        h,
                        crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                        49i32 + 0i32,
                        b_intra,
                        1i32,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8[(49i32 + 0i32) as usize] as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctxidxinc_3,
                            1i32,
                        );
                        cabac_block_residual_422_dc(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.chroma_dc
                                as *mut [crate::src::common::common::dctcoef; 8])
                                .offset(0isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctxidxinc_3,
                            0i32,
                        );
                    }
                    let mut ctxidxinc_4 = cabac_cbf_ctxidxinc(
                        h,
                        crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                        49i32 + 1i32,
                        b_intra,
                        1i32,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8[(49i32 + 1i32) as usize] as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctxidxinc_4,
                            1i32,
                        );
                        cabac_block_residual_422_dc(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.chroma_dc
                                as *mut [crate::src::common::common::dctcoef; 8])
                                .offset(1isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctxidxinc_4,
                            0i32,
                        );
                    }
                } else {
                    let mut ctxidxinc_5 = cabac_cbf_ctxidxinc(
                        h,
                        crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                        49i32 + 0i32,
                        b_intra,
                        1i32,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8[(49i32 + 0i32) as usize] as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctxidxinc_5,
                            1i32,
                        );
                        cabac_block_residual(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.chroma_dc
                                as *mut [crate::src::common::common::dctcoef; 8])
                                .offset(0isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctxidxinc_5,
                            0i32,
                        );
                    }
                    let mut ctxidxinc_6 = cabac_cbf_ctxidxinc(
                        h,
                        crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                        49i32 + 1i32,
                        b_intra,
                        1i32,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8[(49i32 + 1i32) as usize] as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctxidxinc_6,
                            1i32,
                        );
                        cabac_block_residual(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.chroma_dc
                                as *mut [crate::src::common::common::dctcoef; 8])
                                .offset(1isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb,
                            ctxidxinc_6,
                            0i32,
                        );
                    }
                }
                if (*h).mb.i_cbp_chroma == 2i32 {
                    let mut i_5 = 16i32;
                    let mut step = (8i32)
                        << (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                    while i_5 < 3i32 * 16i32 {
                        let mut j_0 = i_5;
                        while j_0 < i_5 + 4i32 {
                            let mut ctxidxinc_7 = cabac_cbf_ctxidxinc(
                                h,
                                crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int,
                                j_0,
                                b_intra,
                                0i32,
                            );
                            if (*h).mb.cache.non_zero_count[x264_scan8[j_0 as usize] as usize] != 0
                            {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb,
                                    ctxidxinc_7,
                                    1i32,
                                );
                                cabac_block_residual(
                                    h,
                                    cb,
                                    crate::src::common::macroblock::DCT_CHROMA_AC
                                        as ::core::ffi::c_int,
                                    (&raw mut *(&raw mut (*h).dct.luma4x4
                                        as *mut [crate::src::common::common::dctcoef; 16])
                                        .offset(j_0 as isize)
                                        as *mut crate::src::common::common::dctcoef)
                                        .offset(1isize),
                                );
                            } else {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb,
                                    ctxidxinc_7,
                                    0i32,
                                );
                            }
                            j_0 += 1;
                        }
                        i_5 += step;
                    }
                }
            }
        }
        (*h).stat.frame.i_tex_bits += x264_cabac_pos(cb) - i_mb_pos_tex;
    }
}
pub unsafe extern "C" fn x264_8_macroblock_write_cabac(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            macroblock_write_cabac_internal(h, cb, 3i32, 0i32);
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            macroblock_write_cabac_internal(h, cb, 1i32, 1i32);
        } else {
            macroblock_write_cabac_internal(h, cb, 1i32, 0i32);
        };
    }
}
