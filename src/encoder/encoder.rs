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
    pub unsafe extern "C" fn bs_pos(
        mut s: *mut crate::src::common::bitstream::bs_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            return (8i64 * (*s).p.offset_from((*s).p_start) as ::core::ffi::c_long
                + (crate::osdep_h::WORD_SIZE * 8i32) as ::core::ffi::c_long
                - (*s).i_left as ::core::ffi::c_long) as ::core::ffi::c_int;
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
    pub unsafe extern "C" fn bs_realign(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            let mut offset = ((*s).p as crate::stdlib::intptr_t & 3isize) as ::core::ffi::c_int;
            if offset != 0 {
                (*s).p = (*s).p.offset(-(offset as isize));
                (*s).i_left = (crate::osdep_h::WORD_SIZE - offset) * 8i32;
                (*s).cur_bits =
                    endian_fix32((*((*s).p as *mut crate::src::common::base::x264_union32_t)).i)
                        as crate::stdlib::uintptr_t;
                (*s).cur_bits >>= (4i32 - offset) * 8i32;
            }
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
    #[inline]
    pub unsafe extern "C" fn bs_write1(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut i_bit: crate::stdlib::uint32_t,
    ) {
        unsafe {
            (*s).cur_bits <<= 1i32;
            (*s).cur_bits |= i_bit as crate::stdlib::uintptr_t;
            (*s).i_left -= 1;
            if (*s).i_left == crate::osdep_h::WORD_SIZE * 8i32 - 32i32 {
                (*((*s).p as *mut crate::src::common::base::x264_union32_t)).i =
                    endian_fix32((*s).cur_bits as crate::stdlib::uint32_t);
                (*s).p = (*s).p.offset(4isize);
                (*s).i_left = crate::osdep_h::WORD_SIZE * 8i32;
            }
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_align_1(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            bs_write(
                s,
                (*s).i_left & 7i32,
                (((1i32) << ((*s).i_left & 7i32)) - 1i32) as crate::stdlib::uint32_t,
            );
            bs_flush(s);
        }
    }
    pub static mut x264_ue_size_tab: [crate::stdlib::uint8_t; 256] = [
        1u8, 1u8, 3u8, 3u8, 5u8, 5u8, 5u8, 5u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 9u8, 9u8,
        9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 11u8, 11u8, 11u8,
        11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8,
        11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 13u8,
        13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
        13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
        13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
        13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
        13u8, 13u8, 13u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
    ];
    #[inline]
    pub unsafe extern "C" fn bs_write_ue_big(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut val: ::core::ffi::c_uint,
    ) {
        unsafe {
            let mut size = 0i32;
            val = val.wrapping_add(1);
            let mut tmp = val as ::core::ffi::c_int;
            if tmp >= 0x10000i32 {
                size = 32i32;
                tmp >>= 16i32;
            }
            if tmp >= 0x100i32 {
                size += 16i32;
                tmp >>= 8i32;
            }
            size += x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
            bs_write(s, size >> 1i32, 0u32);
            bs_write(s, (size >> 1i32) + 1i32, val);
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_write_se(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut val: ::core::ffi::c_int,
    ) {
        unsafe {
            let mut size = 0i32;
            let mut tmp = 1i32 - val * 2i32;
            if tmp < 0i32 {
                tmp = val * 2i32;
            }
            val = tmp;
            if tmp >= 0x100i32 {
                size = 16i32;
                tmp >>= 8i32;
            }
            size += x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
            bs_write(s, size, val as crate::stdlib::uint32_t);
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_rbsp_trailing(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            bs_write1(s, 1u32);
            bs_write(s, (*s).i_left & 7i32, 0u32);
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn bs_size_ue_big(mut val: ::core::ffi::c_uint) -> ::core::ffi::c_int {
        unsafe {
            if val < 255u32 {
                return x264_ue_size_tab[val.wrapping_add(1u32) as usize] as ::core::ffi::c_int;
            } else {
                return x264_ue_size_tab[(val.wrapping_add(1u32) >> 8i32) as usize]
                    as ::core::ffi::c_int
                    + 16i32;
            };
        }
    }
    use crate::src::encoder::encoder::osdep_h::endian_fix;
    use crate::src::encoder::encoder::osdep_h::endian_fix32;
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
pub mod pixel_h {
    pub static mut x264_luma2chroma_pixel: [[crate::stdlib::uint8_t; 7]; 4] = [
        [0u8, 0, 0, 0, 0, 0, 0],
        [
            crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x4 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x4 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x2 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_2x4 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_2x2 as crate::stdlib::uint8_t,
        ],
        [
            crate::src::common::pixel::PIXEL_8x16 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x16 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x4 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_2x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_2x4 as crate::stdlib::uint8_t,
        ],
        [
            crate::src::common::pixel::PIXEL_16x16 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_16x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x16 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x4 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x4 as crate::stdlib::uint8_t,
        ],
    ];
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
    pub const slice_type_to_char: [char; 3] = ['P', 'B', 'I'];
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
    pub static mut x264_mb_partition_pixel_table: [crate::stdlib::uint8_t; 17] = [
        crate::src::common::pixel::PIXEL_4x4 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x4 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x4 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x4 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x4 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x4 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_16x8 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x16 as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_16x16 as crate::stdlib::uint8_t,
    ];
    pub static mut i_chroma_qp_table: [crate::stdlib::uint8_t; 94] = [
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        (0i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (1i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (2i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (3i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (4i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (5i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (6i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (7i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (8i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (9i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (10i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (11i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (12i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (13i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (14i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (15i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (16i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (17i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (18i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (19i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (20i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (21i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (22i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (23i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (24i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (25i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (26i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (27i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (28i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (29i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (29i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (30i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (31i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (32i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (32i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (33i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (34i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (34i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (35i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (35i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (36i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (36i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (37i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (37i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (37i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (38i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (38i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (38i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        (39i32 + crate::src::common::common::QP_BD_OFFSET) as crate::stdlib::uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
}
pub mod osdep_h {
    #[inline]
    pub unsafe extern "C" fn x264_is_regular_file(
        mut filehandle: *mut crate::stdlib::FILE,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut file_stat = crate::stdlib::stat {
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
                return 1i32;
            }
            return (file_stat.st_mode & crate::stdlib::__S_IFMT as crate::stdlib::__mode_t
                == 0o100000u32) as ::core::ffi::c_int;
        }
    }
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
}
use crate::src::common::base::ChromaFormat;
use crate::src::common::base::x264_free;
use crate::src::common::base::x264_param_strdup;
use crate::src::common::common::x264_t;
use crate::src::common::cpu::X264_CPU_NAMES;
use crate::src::encoder::encoder::base_h::slice_type_to_char;
use crate::src::encoder::encoder::base_h::x264_clip3;
use crate::src::encoder::encoder::base_h::x264_clip3f;
use crate::src::encoder::encoder::base_h::x264_scan8;
use crate::src::encoder::encoder::bitstream_h::bs_align_1;
use crate::src::encoder::encoder::bitstream_h::bs_flush;
use crate::src::encoder::encoder::bitstream_h::bs_init;
use crate::src::encoder::encoder::bitstream_h::bs_pos;
use crate::src::encoder::encoder::bitstream_h::bs_rbsp_trailing;
use crate::src::encoder::encoder::bitstream_h::bs_realign;
use crate::src::encoder::encoder::bitstream_h::bs_size_ue_big;
use crate::src::encoder::encoder::bitstream_h::bs_write;
use crate::src::encoder::encoder::bitstream_h::bs_write_se;
use crate::src::encoder::encoder::bitstream_h::bs_write_ue_big;
use crate::src::encoder::encoder::bitstream_h::bs_write1;
use crate::src::encoder::encoder::cabac_h::x264_cabac_pos;
use crate::src::encoder::encoder::macroblock_h::i_chroma_qp_table;
use crate::src::encoder::encoder::macroblock_h::x264_mb_partition_pixel_table;
use crate::src::encoder::encoder::macroblock_h::x264_mb_type_list_table;
use crate::src::encoder::encoder::osdep_h::x264_is_regular_file;
use crate::src::encoder::encoder::pixel_h::x264_luma2chroma_pixel;
use crate::src::encoder::encoder::predict_h::x264_mb_chroma_pred_mode_fix;
use crate::src::encoder::encoder::predict_h::x264_mb_pred_mode4x4_fix;
use crate::src::encoder::encoder::predict_h::x264_mb_pred_mode16x16_fix;
use crate::x264_h::X264_CPU_BMI1;
use crate::x264_h::X264_CPU_BMI2;
use crate::x264_h::X264_CPU_CACHELINE_64;
use crate::x264_h::X264_CPU_FMA3;
use crate::x264_h::X264_CPU_SSE2;
use crate::x264_h::X264_CPU_SSE2_IS_FAST;
use crate::x264_h::X264_CPU_SSE2_IS_SLOW;
use crate::x264_h::X264_CPU_SSE42;
use crate::x264_h::X264_CPU_SSSE3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub fps_num: crate::stdlib::uint16_t,
    pub fps_den: crate::stdlib::uint16_t,
    pub interlaced: crate::stdlib::uint8_t,
    pub frame_size: crate::stdlib::uint16_t,
    pub cqm_4iy: *const crate::stdlib::uint8_t,
    pub cqm_4ic: *const crate::stdlib::uint8_t,
    pub cqm_8iy: *const crate::stdlib::uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_bs_bak_t {
    pub skip: ::core::ffi::c_int,
    pub cabac_prevbyte: crate::stdlib::uint8_t,
    pub bs: crate::src::common::bitstream::bs_t,
    pub cabac: crate::src::common::cabac::x264_cabac_t,
    pub stat: crate::src::common::common::x264_frame_stat_t,
    pub last_qp: ::core::ffi::c_int,
    pub last_dqp: ::core::ffi::c_int,
    pub field_decoding_flag: ::core::ffi::c_int,
}
unsafe extern "C" fn calc_psnr(
    mut sqe: ::core::ffi::c_double,
    mut size: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    unsafe {
        let mut mse = sqe
            / ((crate::src::common::common::PIXEL_MAX * crate::src::common::common::PIXEL_MAX)
                as ::core::ffi::c_double
                * size);
        if mse <= 0.0000000001 {
            return 100f64;
        }
        return -10.0f64 * crate::stdlib::log10(mse);
    }
}
unsafe extern "C" fn calc_ssim_db(mut ssim: ::core::ffi::c_double) -> ::core::ffi::c_double {
    unsafe {
        let mut inv_ssim = 1f64 - ssim;
        if inv_ssim <= 0.0000000001 {
            return 100f64;
        }
        return -10.0f64 * crate::stdlib::log10(inv_ssim);
    }
}
unsafe extern "C" fn threadpool_wait_all(mut h: *mut x264_t) -> ::core::ffi::c_int {
    unsafe {
        let mut i = 0i32;
        while i < (*h).param.i_threads {
            if (*(*h).thread[i as usize]).thread_active {
                (*(*h).thread[i as usize]).thread_active = false;
                if (crate::src::common::threadpool::x264_8_threadpool_wait(
                    (*h).threadpool,
                    (*h).thread[i as usize] as *mut ::core::ffi::c_void,
                ) as crate::stdlib::intptr_t)
                    < 0isize
                {
                    return -(1i32);
                }
            }
            i += 1;
        }
        return 0i32;
    }
}
unsafe extern "C" fn frame_dump(mut h: *mut x264_t) {
    unsafe {
        let mut f = crate::stdlib::fopen(
            (*h).param.psz_dump_yuv,
            b"r+b\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if f.is_null() {
            return;
        }
        if (*h).param.sliced_threads {
            threadpool_wait_all(h);
        }
        let mut frame_size = (*h).param.i_height
            * (*h).param.i_width
            * ::core::mem::size_of::<crate::src::common::common::pixel>() as ::core::ffi::c_int
            + 2i32
                * (if !(*h).sps.i_chroma_format_idc.is_400() {
                    ((*h).param.i_height
                        * (*h).param.i_width
                        * ::core::mem::size_of::<crate::src::common::common::pixel>()
                            as ::core::ffi::c_int)
                        >> (((*h).sps.i_chroma_format_idc.is_420()
                            || (*h).sps.i_chroma_format_idc.is_422())
                            as ::core::ffi::c_int
                            + ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int)
                } else {
                    0i32
                });
        if crate::stdlib::fseeko(
            f,
            (*(*h).fdec).i_frame as crate::stdlib::__off64_t
                * frame_size as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_SET,
        ) == 0
        {
            let mut p = 0i32;
            while p
                < (if (*h).sps.i_chroma_format_idc.is_444() {
                    3i32
                } else {
                    1i32
                })
            {
                let mut y = 0i32;
                while y < (*h).param.i_height {
                    crate::stdlib::fwrite(
                        (*(&raw mut (*(*h).fdec).plane
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(p as isize))
                        .offset(
                            (y * *(&raw mut (*(*h).fdec).i_stride as *mut ::core::ffi::c_int)
                                .offset(p as isize)) as isize,
                        ) as *const ::core::ffi::c_void,
                        crate::src::common::common::SIZEOF_PIXEL
                            as crate::__stddef_size_t_h::size_t,
                        (*h).param.i_width as crate::__stddef_size_t_h::size_t,
                        f,
                    );
                    y += 1;
                }
                p += 1;
            }
            if (*h).sps.i_chroma_format_idc.is_420() || (*h).sps.i_chroma_format_idc.is_422() {
                let mut cw = (*h).param.i_width >> 1i32;
                let mut ch = (*h).param.i_height
                    >> ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
                let mut planeu = crate::src::common::base::x264_malloc(
                    (2i32 * (cw * ch * crate::src::common::common::SIZEOF_PIXEL + 32i32))
                        as crate::stdlib::int64_t,
                ) as *mut crate::src::common::common::pixel;
                if !planeu.is_null() {
                    let mut planev = planeu
                        .offset((cw * ch) as isize)
                        .offset((32i32 / crate::src::common::common::SIZEOF_PIXEL) as isize);
                    (*h).mc
                        .plane_copy_deinterleave
                        .expect("non-null function pointer")(
                        planeu,
                        cw as crate::stdlib::intptr_t,
                        planev,
                        cw as crate::stdlib::intptr_t,
                        (*(*h).fdec).plane[1usize],
                        (*(*h).fdec).i_stride[1usize] as crate::stdlib::intptr_t,
                        cw,
                        ch,
                    );
                    crate::stdlib::fwrite(
                        planeu as *const ::core::ffi::c_void,
                        1usize,
                        (cw * ch * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                        f,
                    );
                    crate::stdlib::fwrite(
                        planev as *const ::core::ffi::c_void,
                        1usize,
                        (cw * ch * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                        f,
                    );
                    x264_free(planeu as *mut ::core::ffi::c_void);
                }
            }
        }
        crate::stdlib::fclose(f);
    }
}
unsafe extern "C" fn slice_header_init<'a>(
    mut h: *mut x264_t,
    mut sh: *mut crate::src::common::common::x264_slice_header_t<'a>,
    sps: &'a crate::src::common::set::x264_sps_t,
    mut pps: *mut crate::src::common::set::x264_pps_t,
    mut i_idr_pic_id: ::core::ffi::c_int,
    mut i_frame: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let mut list = 0i32;
        let mut param = &raw mut (*h).param;
        (*sh).sps = sps;
        (*sh).pps = pps;
        (*sh).i_first_mb = 0i32;
        (*sh).i_last_mb = (*h).mb.i_mb_count - 1i32;
        (*sh).i_pps_id = (*pps).i_id;
        (*sh).i_frame_num = i_frame;
        (*sh).mbaff = (*h).param.interlaced;
        (*sh).field_pic = false;
        (*sh).bottom_field = false;
        (*sh).i_idr_pic_id = i_idr_pic_id;
        (*sh).i_poc = 0i32;
        (*sh).i_delta_poc_bottom = 0i32;
        (*sh).i_delta_poc[0usize] = 0i32;
        (*sh).i_delta_poc[1usize] = 0i32;
        (*sh).i_redundant_pic_cnt = 0i32;
        (*h).mb.direct_auto_write = (*h).param.analyse.i_direct_mv_pred
            == crate::x264_h::X264_DIRECT_PRED_AUTO
            && (*h).param.i_bframe != 0
            && ((*h).param.rc.stat_write || !(*h).param.rc.stat_read);
        if !(*h).mb.direct_auto_read
            && (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
        {
            if (*(*h).fref[1usize][0usize]).i_poc_l0ref0 == (*(*h).fref[0usize][0usize]).i_poc {
                if (*h).mb.direct_auto_write {
                    (*sh).direct_spatial_mv_pred =
                        (*h).stat.i_direct_score[1usize] > (*h).stat.i_direct_score[0usize];
                } else {
                    (*sh).direct_spatial_mv_pred = (*param).analyse.i_direct_mv_pred
                        == crate::x264_h::X264_DIRECT_PRED_SPATIAL;
                }
            } else {
                (*h).mb.direct_auto_write = false;
                (*sh).direct_spatial_mv_pred = true;
            }
        }
        (*sh).num_ref_idx_override = false;
        (*sh).i_num_ref_idx_l0_active = 1i32;
        (*sh).i_num_ref_idx_l1_active = 1i32;
        (*sh).ref_pic_list_reordering[0usize] = (*h).ref_reorder[0usize];
        (*sh).ref_pic_list_reordering[1usize] = (*h).ref_reorder[1usize];
        while list < 2i32 {
            if (*sh).ref_pic_list_reordering[list as usize] {
                let mut i = 0i32;
                let mut pred_frame_num = i_frame;
                while i < (*h).i_ref[list as usize] {
                    let mut diff =
                        (*(*h).fref[list as usize][i as usize]).i_frame_num - pred_frame_num;
                    (*sh).ref_pic_list_order[list as usize][i as usize].idc =
                        (diff > 0i32) as ::core::ffi::c_int;
                    (*sh).ref_pic_list_order[list as usize][i as usize].arg =
                        (crate::stdlib::abs(diff) - 1i32)
                            & (((1i32) << sps.i_log2_max_frame_num) - 1i32);
                    pred_frame_num = (*(*h).fref[list as usize][i as usize]).i_frame_num;
                    i += 1;
                }
            }
            list += 1;
        }
        (*sh).i_cabac_init_idc = (*param).i_cabac_init_idc;
        (*sh).i_qp = if i_qp < 51i32 + 6i32 * (8i32 - 8i32) {
            i_qp
        } else {
            51i32 + 6i32 * (8i32 - 8i32)
        };
        (*sh).i_qp_delta = (*sh).i_qp - (*pps).i_pic_init_qp;
        (*sh).sp_for_swidth = false;
        (*sh).i_qs_delta = 0i32;
        let mut deblock_thresh = i_qp
            + 2i32
                * (if (*param).i_deblocking_filter_alphac0 < (*param).i_deblocking_filter_beta {
                    (*param).i_deblocking_filter_alphac0
                } else {
                    (*param).i_deblocking_filter_beta
                });
        if (*param).deblocking_filter && ((*h).mb.variable_qp || (15i32) < deblock_thresh) {
            (*sh).i_disable_deblocking_filter_idc =
                if (*param).sliced_threads { 2i32 } else { 0i32 };
        } else {
            (*sh).i_disable_deblocking_filter_idc = 1i32;
        }
        (*sh).i_alpha_c0_offset = (*param).i_deblocking_filter_alphac0 * 2i32;
        (*sh).i_beta_offset = (*param).i_deblocking_filter_beta * 2i32;
    }
}
unsafe extern "C" fn slice_header_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut sh: *mut crate::src::common::common::x264_slice_header_t,
    mut i_nal_ref_idc: ::core::ffi::c_int,
) {
    unsafe {
        if (*sh).mbaff {
            let mut first_x = (*sh).i_first_mb % (*sh).sps.i_mb_width;
            let mut first_y = (*sh).i_first_mb / (*sh).sps.i_mb_width;
            '_c2rust_label: {
                if first_y & 1i32 == 0i32 {
                } else {
                    crate::stdlib::__assert_fail(
                        b"(first_y&1) == 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                        219u32,
                        b"void slice_header_write(bs_t *, x264_slice_header_t *, int)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            bs_write_ue_big(
                s,
                ((2i32 * first_x + (*sh).sps.i_mb_width * (first_y & !(1i32)) + (first_y & 1i32))
                    >> 1i32) as ::core::ffi::c_uint,
            );
        } else {
            bs_write_ue_big(s, (*sh).i_first_mb as ::core::ffi::c_uint);
        }
        bs_write_ue_big(s, ((*sh).i_type + 5i32) as ::core::ffi::c_uint);
        bs_write_ue_big(s, (*sh).i_pps_id as ::core::ffi::c_uint);
        bs_write(
            s,
            (*sh).sps.i_log2_max_frame_num,
            ((*sh).i_frame_num & (((1i32) << (*sh).sps.i_log2_max_frame_num) - 1i32))
                as crate::stdlib::uint32_t,
        );
        if !(*sh).sps.frame_mbs_only {
            bs_write1(s, (*sh).field_pic as crate::stdlib::uint32_t);
            if (*sh).field_pic {
                bs_write1(s, (*sh).bottom_field as crate::stdlib::uint32_t);
            }
        }
        if (*sh).i_idr_pic_id >= 0i32 {
            bs_write_ue_big(s, (*sh).i_idr_pic_id as ::core::ffi::c_uint);
        }
        if (*sh).sps.i_poc_type == 0i32 {
            bs_write(
                s,
                (*sh).sps.i_log2_max_poc_lsb,
                ((*sh).i_poc & (((1i32) << (*sh).sps.i_log2_max_poc_lsb) - 1i32))
                    as crate::stdlib::uint32_t,
            );
            if (*(*sh).pps).pic_order && !(*sh).field_pic {
                bs_write_se(s, (*sh).i_delta_poc_bottom);
            }
        }
        if (*(*sh).pps).redundant_pic_cnt {
            bs_write_ue_big(s, (*sh).i_redundant_pic_cnt as ::core::ffi::c_uint);
        }
        if (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            bs_write1(s, (*sh).direct_spatial_mv_pred as crate::stdlib::uint32_t);
        }
        if (*sh).i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            || (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
        {
            bs_write1(s, (*sh).num_ref_idx_override as crate::stdlib::uint32_t);
            if (*sh).num_ref_idx_override {
                bs_write_ue_big(
                    s,
                    ((*sh).i_num_ref_idx_l0_active - 1i32) as ::core::ffi::c_uint,
                );
                if (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    bs_write_ue_big(
                        s,
                        ((*sh).i_num_ref_idx_l1_active - 1i32) as ::core::ffi::c_uint,
                    );
                }
            }
        }
        if (*sh).i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            bs_write1(
                s,
                (*sh).ref_pic_list_reordering[0usize] as crate::stdlib::uint32_t,
            );
            if (*sh).ref_pic_list_reordering[0usize] {
                let mut i = 0i32;
                while i < (*sh).i_num_ref_idx_l0_active {
                    bs_write_ue_big(
                        s,
                        (*sh).ref_pic_list_order[0usize][i as usize].idc as ::core::ffi::c_uint,
                    );
                    bs_write_ue_big(
                        s,
                        (*sh).ref_pic_list_order[0usize][i as usize].arg as ::core::ffi::c_uint,
                    );
                    i += 1;
                }
                bs_write_ue_big(s, 3u32);
            }
        }
        if (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            bs_write1(
                s,
                (*sh).ref_pic_list_reordering[1usize] as crate::stdlib::uint32_t,
            );
            if (*sh).ref_pic_list_reordering[1usize] {
                let mut i_0 = 0i32;
                while i_0 < (*sh).i_num_ref_idx_l1_active {
                    bs_write_ue_big(
                        s,
                        (*sh).ref_pic_list_order[1usize][i_0 as usize].idc as ::core::ffi::c_uint,
                    );
                    bs_write_ue_big(
                        s,
                        (*sh).ref_pic_list_order[1usize][i_0 as usize].arg as ::core::ffi::c_uint,
                    );
                    i_0 += 1;
                }
                bs_write_ue_big(s, 3u32);
            }
        }
        (*sh).weighted_pred = false;
        if (*(*sh).pps).weighted_pred
            && (*sh).i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
        {
            let mut i_1 = 0i32;
            (*sh).weighted_pred = !(*sh).weight[0usize][0usize].weightfn.is_null()
                || !(*sh).weight[0usize][1usize].weightfn.is_null()
                || !(*sh).weight[0usize][2usize].weightfn.is_null();
            bs_write_ue_big(
                s,
                (*sh).weight[0usize][0usize].i_denom as ::core::ffi::c_uint,
            );
            if !(*sh).sps.i_chroma_format_idc.is_400() {
                bs_write_ue_big(
                    s,
                    (*sh).weight[0usize][1usize].i_denom as ::core::ffi::c_uint,
                );
            }
            while i_1 < (*sh).i_num_ref_idx_l0_active {
                let mut luma_weight_l0_flag =
                    !(*sh).weight[i_1 as usize][0usize].weightfn.is_null() as ::core::ffi::c_int;
                bs_write1(s, luma_weight_l0_flag as crate::stdlib::uint32_t);
                if luma_weight_l0_flag != 0 {
                    bs_write_se(s, (*sh).weight[i_1 as usize][0usize].i_scale);
                    bs_write_se(s, (*sh).weight[i_1 as usize][0usize].i_offset);
                }
                if !(*sh).sps.i_chroma_format_idc.is_400() {
                    let mut chroma_weight_l0_flag =
                        (!(*sh).weight[i_1 as usize][1usize].weightfn.is_null()
                            || !(*sh).weight[i_1 as usize][2usize].weightfn.is_null())
                            as ::core::ffi::c_int;
                    bs_write1(s, chroma_weight_l0_flag as crate::stdlib::uint32_t);
                    if chroma_weight_l0_flag != 0 {
                        let mut j = 1i32;
                        while j < 3i32 {
                            bs_write_se(s, (*sh).weight[i_1 as usize][j as usize].i_scale);
                            bs_write_se(s, (*sh).weight[i_1 as usize][j as usize].i_offset);
                            j += 1;
                        }
                    }
                }
                i_1 += 1;
            }
        } else {
            // TODO: unused_must_use
            let _ = (*(*sh).pps).weighted_bipred == 1i32
                && (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int;
        }
        if i_nal_ref_idc != 0i32 {
            if (*sh).i_idr_pic_id >= 0i32 {
                bs_write1(s, 0u32);
                bs_write1(s, 0u32);
            } else {
                bs_write1(
                    s,
                    ((*sh).i_mmco_command_count > 0i32) as crate::stdlib::uint32_t,
                );
                if (*sh).i_mmco_command_count > 0i32 {
                    let mut i_2 = 0i32;
                    while i_2 < (*sh).i_mmco_command_count {
                        bs_write_ue_big(s, 1u32);
                        bs_write_ue_big(
                            s,
                            ((*sh).mmco[i_2 as usize].i_difference_of_pic_nums - 1i32)
                                as ::core::ffi::c_uint,
                        );
                        i_2 += 1;
                    }
                    bs_write_ue_big(s, 0u32);
                }
            }
        }
        if (*(*sh).pps).cabac
            && (*sh).i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
        {
            bs_write_ue_big(s, (*sh).i_cabac_init_idc as ::core::ffi::c_uint);
        }
        bs_write_se(s, (*sh).i_qp_delta);
        if (*(*sh).pps).deblocking_filter_control {
            bs_write_ue_big(
                s,
                (*sh).i_disable_deblocking_filter_idc as ::core::ffi::c_uint,
            );
            if (*sh).i_disable_deblocking_filter_idc != 1i32 {
                bs_write_se(s, (*sh).i_alpha_c0_offset >> 1i32);
                bs_write_se(s, (*sh).i_beta_offset >> 1i32);
            }
        }
    }
}
unsafe extern "C" fn bitstream_check_buffer_internal(
    mut h: *mut x264_t,
    mut size: ::core::ffi::c_int,
    mut b_cabac: ::core::ffi::c_int,
    mut i_nal: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if b_cabac != 0
            && ((*h).cabac.p_end.offset_from((*h).cabac.p) as ::core::ffi::c_long)
                < size as ::core::ffi::c_long
            || ((*h).out.bs.p_end.offset_from((*h).out.bs.p) as ::core::ffi::c_long)
                < size as ::core::ffi::c_long
        {
            let mut i = 0i32;
            if size > crate::limits_h::INT_MAX - (*h).out.i_bitstream {
                return -(1i32);
            }
            let mut buf_size = (*h).out.i_bitstream + size;
            let mut buf = crate::src::common::base::x264_malloc(buf_size as crate::stdlib::int64_t)
                as *mut crate::stdlib::uint8_t;
            if buf.is_null() {
                return -(1i32);
            }
            let mut aligned_size = (*h).out.i_bitstream & !(15i32);
            (*h).mc.memcpy_aligned.expect("non-null function pointer")(
                buf as *mut ::core::ffi::c_void,
                (*h).out.p_bitstream as *const ::core::ffi::c_void,
                aligned_size as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                buf.offset(aligned_size as isize) as *mut ::core::ffi::c_void,
                (*h).out.p_bitstream.offset(aligned_size as isize) as *const ::core::ffi::c_void,
                ((*h).out.i_bitstream - aligned_size) as crate::__stddef_size_t_h::size_t,
            );
            let mut delta = buf.offset_from((*h).out.p_bitstream);
            (*h).out.bs.p_start = (*h).out.bs.p_start.offset(delta);
            (*h).out.bs.p = (*h).out.bs.p.offset(delta);
            (*h).out.bs.p_end = buf.offset(buf_size as isize);
            (*h).cabac.p_start = (*h).cabac.p_start.offset(delta);
            (*h).cabac.p = (*h).cabac.p.offset(delta);
            (*h).cabac.p_end = buf.offset(buf_size as isize);
            while i <= i_nal {
                let ref mut c2rust_fresh2 = (*(*h).out.nal.offset(i as isize)).p_payload;
                *c2rust_fresh2 = (*c2rust_fresh2).offset(delta);
                i += 1;
            }
            x264_free((*h).out.p_bitstream as *mut ::core::ffi::c_void);
            (*h).out.p_bitstream = buf;
            (*h).out.i_bitstream = buf_size;
        }
        return 0i32;
    }
}
unsafe extern "C" fn bitstream_check_buffer(mut h: *mut x264_t) -> ::core::ffi::c_int {
    unsafe {
        let mut max_row_size =
            ((2500i32) << (*h).sh.mbaff as ::core::ffi::c_int) * (*h).mb.i_mb_width;
        return bitstream_check_buffer_internal(
            h,
            max_row_size,
            (*h).param.cabac as ::core::ffi::c_int,
            (*h).out.i_nal,
        );
    }
}
unsafe extern "C" fn bitstream_check_buffer_filler(
    mut h: *mut x264_t,
    mut filler: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        filler += 32i32;
        return bitstream_check_buffer_internal(h, filler, 0i32, -(1i32));
    }
}
unsafe extern "C" fn validate_parameters(
    mut h: *mut x264_t,
    mut b_open: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut w_mod = 1i32;
        if (*h).param.pf_log.is_none() {
            crate::src::common::base::x264_log_internal(
                crate::x264_h::X264_LOG_ERROR_1,
                b"pf_log not set! did you forget to call x264_param_default?\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -(1i32);
        }
        (*h).param.interlaced = (*h).param.interlaced;
        if (*h).param.i_width <= 0i32
            || (*h).param.i_height <= 0i32
            || (*h).param.i_width > MAX_RESOLUTION
            || (*h).param.i_height > MAX_RESOLUTION
        {
            log::error!(
                "invalid width x height ({}x{})",
                (*h).param.i_width,
                (*h).param.i_height
            );
            return -(1i32);
        }
        let mut i_csp = (*h).param.i_csp & crate::x264_h::X264_CSP_MASK;
        if !(*h).sps.i_chroma_format_idc.is_400() && i_csp == crate::x264_h::X264_CSP_I400 {
            log::error!("not compiled with 4:0:0 support");
            return -(1i32);
        } else if !(*h).sps.i_chroma_format_idc.is_420()
            && i_csp >= crate::x264_h::X264_CSP_I420
            && i_csp < crate::x264_h::X264_CSP_I422
        {
            log::error!("not compiled with 4:2:0 support");
            return -(1i32);
        } else if !(*h).sps.i_chroma_format_idc.is_422()
            && i_csp >= crate::x264_h::X264_CSP_I422
            && i_csp < crate::x264_h::X264_CSP_I444
        {
            log::error!("not compiled with 4:2:2 support");
            return -(1i32);
        } else if !(*h).sps.i_chroma_format_idc.is_444()
            && i_csp >= crate::x264_h::X264_CSP_I444
            && i_csp <= crate::x264_h::X264_CSP_RGB
        {
            log::error!("not compiled with 4:4:4 support");
            return -(1i32);
        }
        if i_csp <= crate::x264_h::X264_CSP_NONE || i_csp >= crate::x264_h::X264_CSP_MAX {
            log::error!(
                "invalid CSP (only I400/I420/YV12/NV12/NV21/I422/YV16/NV16/YUYV/UYVY/I444/YV24/BGR/BGRA/RGB supported)"
            );
            return -(1i32);
        }
        let mut h_mod =
            (1i32) << ((*h).param.interlaced || (*h).param.fake_interlaced) as ::core::ffi::c_int;
        if i_csp == crate::x264_h::X264_CSP_I400 {
            (*h).param.analyse.i_chroma_qp_offset = 0i32;
            (*h).param.analyse.chroma_me = false;
            (*h).param.vui.i_colmatrix = 2i32;
        } else if i_csp < crate::x264_h::X264_CSP_I444 {
            w_mod = 2i32;
            if i_csp < crate::x264_h::X264_CSP_I422 {
                h_mod *= 2i32;
            }
        }
        if (*h).param.i_width % w_mod != 0 {
            log::error!(
                "width not divisible by {w_mod} ({}x{})",
                (*h).param.i_width,
                (*h).param.i_height
            );
            return -(1i32);
        }
        if (*h).param.i_height % h_mod != 0 {
            log::error!(
                "height not divisible by {h_mod} ({}x{})",
                (*h).param.i_width,
                (*h).param.i_height
            );
            return -(1i32);
        }
        if (*h).param.crop_rect.i_left < 0i32
            || (*h).param.crop_rect.i_left >= (*h).param.i_width
            || (*h).param.crop_rect.i_right < 0i32
            || (*h).param.crop_rect.i_right >= (*h).param.i_width
            || (*h).param.crop_rect.i_top < 0i32
            || (*h).param.crop_rect.i_top >= (*h).param.i_height
            || (*h).param.crop_rect.i_bottom < 0i32
            || (*h).param.crop_rect.i_bottom >= (*h).param.i_height
            || (*h).param.crop_rect.i_left + (*h).param.crop_rect.i_right >= (*h).param.i_width
            || (*h).param.crop_rect.i_top + (*h).param.crop_rect.i_bottom >= (*h).param.i_height
        {
            log::error!(
                "invalid crop-rect {},{},{},{}",
                (*h).param.crop_rect.i_left,
                (*h).param.crop_rect.i_top,
                (*h).param.crop_rect.i_right,
                (*h).param.crop_rect.i_bottom
            );
            return -(1i32);
        }
        if (*h).param.crop_rect.i_left % w_mod != 0
            || (*h).param.crop_rect.i_right % w_mod != 0
            || (*h).param.crop_rect.i_top % h_mod != 0
            || (*h).param.crop_rect.i_bottom % h_mod != 0
        {
            log::error!(
                "crop-rect {},{},{},{} not divisible by {w_mod}x{h_mod}",
                (*h).param.crop_rect.i_left,
                (*h).param.crop_rect.i_top,
                (*h).param.crop_rect.i_right,
                (*h).param.crop_rect.i_bottom
            );
            return -(1i32);
        }
        if (*h).param.vui.i_sar_width <= 0i32 || (*h).param.vui.i_sar_height <= 0i32 {
            (*h).param.vui.i_sar_width = 0i32;
            (*h).param.vui.i_sar_height = 0i32;
        }
        if (*h).param.i_threads == crate::x264_h::X264_THREADS_AUTO {
            (*h).param.i_threads = crate::src::common::cpu::x264_cpu_num_processors()
                * (if (*h).param.sliced_threads {
                    2i32
                } else {
                    3i32
                })
                / 2i32;
            let mut max_threads = if 1i32 > ((*h).param.i_height + 15i32) / 16i32 / 2i32 {
                1i32
            } else {
                ((*h).param.i_height + 15i32) / 16i32 / 2i32
            };
            (*h).param.i_threads = if (*h).param.i_threads < max_threads {
                (*h).param.i_threads
            } else {
                max_threads
            };
        }
        let mut max_sliced_threads = if 1i32 > ((*h).param.i_height + 15i32) / 16i32 / 4i32 {
            1i32
        } else {
            ((*h).param.i_height + 15i32) / 16i32 / 4i32
        };
        if (*h).param.i_threads > 1i32 {
            if (*h).param.sliced_threads {
                (*h).param.i_threads = if (*h).param.i_threads < max_sliced_threads {
                    (*h).param.i_threads
                } else {
                    max_sliced_threads
                };
            }
        }
        (*h).param.i_threads = x264_clip3(
            (*h).param.i_threads,
            1i32,
            crate::src::common::base::X264_THREAD_MAX,
        );
        if (*h).param.i_threads == 1i32 {
            (*h).param.sliced_threads = false;
            (*h).param.i_lookahead_threads = 1i32;
        }
        (*h).i_thread_frames = if (*h).param.sliced_threads {
            1i32
        } else {
            (*h).param.i_threads
        };
        if (*h).i_thread_frames > 1i32 {
            (*h).param.nalu_process = None;
        }
        if (*h).param.opencl {
            log::warn!("OpenCL: not compiled with OpenCL support, disabling");
            (*h).param.opencl = false;
            if !(*h).param.opencl_device_id.is_null() && (*h).param.i_opencl_device != 0 {
                log::warn!("OpenCL: device id and device skip count configured; dropping skip");
                (*h).param.i_opencl_device = 0i32;
            }
        }
        (*h).param.i_keyint_max = x264_clip3(
            (*h).param.i_keyint_max,
            1i32,
            crate::x264_h::X264_KEYINT_MAX_INFINITE,
        );
        if (*h).param.i_keyint_max == 1i32 {
            (*h).param.intra_refresh = false;
            (*h).param.analyse.i_weighted_pred = 0i32;
            (*h).param.i_frame_reference = 1i32;
            (*h).param.i_dpb_size = 1i32;
        }
        if (*h).param.i_frame_packing < -(1i32) || (*h).param.i_frame_packing > 7i32 {
            log::warn!("ignoring unknown frame packing value");
            (*h).param.i_frame_packing = -(1i32);
        }
        if (*h).param.i_frame_packing == 7i32
            && (((*h).param.i_width - (*h).param.crop_rect.i_left - (*h).param.crop_rect.i_right)
                % 3i32
                != 0
                || ((*h).param.i_height
                    - (*h).param.crop_rect.i_top
                    - (*h).param.crop_rect.i_bottom)
                    % 3i32
                    != 0)
        {
            log::error!(
                "cropped resolution {}x{} not compatible with tile format frame packing",
                (*h).param.i_width - (*h).param.crop_rect.i_left - (*h).param.crop_rect.i_right,
                (*h).param.i_height - (*h).param.crop_rect.i_top - (*h).param.crop_rect.i_bottom
            );
            return -(1i32);
        }
        if (*h).param.mastering_display.mastering_display {
            if (*h).param.mastering_display.i_display_min == 50000
                && (*h).param.mastering_display.i_display_max == 50000
            {
                log::error!("mastering display min and max brightness cannot both be 50000");
                return -(1i32);
            }
        }
        if b_open != 0 {
            let mut score = 0i32;
            score += ((*h).param.analyse.i_me_range == 0i32) as ::core::ffi::c_int;
            score += ((*h).param.rc.i_qp_step == 3i32) as ::core::ffi::c_int;
            score += ((*h).param.i_keyint_max == 12i32) as ::core::ffi::c_int;
            score += ((*h).param.rc.i_qp_min == 2i32) as ::core::ffi::c_int;
            score += ((*h).param.rc.i_qp_max == 31i32) as ::core::ffi::c_int;
            score +=
                ((*h).param.rc.f_qcompress as ::core::ffi::c_double == 0.5) as ::core::ffi::c_int;
            score +=
                (crate::stdlib::fabs((*h).param.rc.f_ip_factor as ::core::ffi::c_double - 1.25)
                    < 0.01) as ::core::ffi::c_int;
            score +=
                (crate::stdlib::fabs((*h).param.rc.f_pb_factor as ::core::ffi::c_double - 1.25)
                    < 0.01) as ::core::ffi::c_int;
            score += ((*h).param.analyse.inter == 0u32
                && (*h).param.analyse.i_subpel_refine == 8i32)
                as ::core::ffi::c_int;
            if score >= 5i32 {
                log::error!("broken ffmpeg default settings detected");
                log::error!("use an encoding preset (e.g. -vpre medium)");
                log::error!("preset usage: -vpre <speed> -vpre <profile>");
                log::error!("speed presets are listed in x264 --help");
                log::error!("profile is optional; x264 defaults to high");
                return -(1i32);
            }
        }
        if (*h).param.rc.i_rc_method < 0i32 || (*h).param.rc.i_rc_method > 2i32 {
            log::error!("no ratecontrol method specified");
            return -(1i32);
        }
        if (*h).param.interlaced {
            (*h).param.pic_struct = true;
        }
        if (*h).param.i_avcintra_class != 0 {
            let mut i = 0i32;
            if crate::internal::BIT_DEPTH != 10i32 {
                log::error!(
                    "{:2}-bit AVC-Intra is not widely compatible",
                    crate::internal::BIT_DEPTH
                );
                log::error!("10-bit x264 is required to encode AVC-Intra");
                return -(1i32);
            }
            let mut type_0 = if (*h).param.i_avcintra_class == 480i32 {
                4i32
            } else if (*h).param.i_avcintra_class == 300i32 {
                3i32
            } else if (*h).param.i_avcintra_class == 200i32 {
                2i32
            } else if (*h).param.i_avcintra_class == 100i32 {
                1i32
            } else if (*h).param.i_avcintra_class == 50i32 {
                0i32
            } else {
                -(1i32)
            };
            if type_0 < 0i32 {
                log::error!("Invalid AVC-Intra class");
                return -(1i32);
            } else if type_0 > 2i32
                && (*h).param.i_avcintra_flavor != crate::x264_h::X264_AVCINTRA_FLAVOR_SONY
            {
                log::error!(
                    "AVC-Intra {} only supported by Sony XAVC flavor",
                    (*h).param.i_avcintra_class
                );
                return -(1i32);
            }
            static mut avcintra_lut: [[[C2Rust_Unnamed_22; 7]; 2]; 5] = [
                [
                    [
                        C2Rust_Unnamed_22 {
                            fps_num: 60000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 912u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 50u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 1100u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 912u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 1100u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 24000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 912u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                    ],
                    [
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 1u8,
                            frame_size: 1820u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci50_1080i_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 1u8,
                            frame_size: 2196u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci50_1080i_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 60000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 1820u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 1820u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 50u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 2196u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 2196u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 24000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 1820u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic: &raw const crate::src::common::tables::x264_cqm_avci50_4ic
                                as *const crate::stdlib::uint8_t,
                            cqm_8iy: &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy
                                as *const crate::stdlib::uint8_t,
                        },
                    ],
                ],
                [
                    [
                        C2Rust_Unnamed_22 {
                            fps_num: 60000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 1848u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 50u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 2224u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 1848u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 2224u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 24000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 1848u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                    ],
                    [
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 1u8,
                            frame_size: 3692u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080i_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 1u8,
                            frame_size: 4444u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080i_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 60000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 3692u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 3692u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 50u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 4444u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 4444u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 24000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 3692u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                    ],
                ],
                [
                    [
                        C2Rust_Unnamed_22 {
                            fps_num: 60000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 3724u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 50u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 4472u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                    ],
                    [
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 1u8,
                            frame_size: 7444u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080i_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 1u8,
                            frame_size: 8940u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080i_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 60000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 7444u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 7444u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 50u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 8940u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 8940u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 24000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 7444u16,
                            cqm_4iy: &raw const crate::src::common::tables::x264_cqm_jvt4i
                                as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                    ],
                ],
                [
                    [
                        C2Rust_Unnamed_22 {
                            fps_num: 60000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 9844u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 50u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 9844u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 9844u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 9844u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 24000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 9844u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                    ],
                    [C2Rust_Unnamed_22 {
                        fps_num: 0,
                        fps_den: 0,
                        interlaced: 0,
                        frame_size: 0,
                        cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                    }; 7],
                ],
                [
                    [
                        C2Rust_Unnamed_22 {
                            fps_num: 60000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 15700u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 50u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 15700u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 30000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 15700u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 25u16,
                            fps_den: 1u16,
                            interlaced: 0u8,
                            frame_size: 15700u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 24000u16,
                            fps_den: 1001u16,
                            interlaced: 0u8,
                            frame_size: 15700u16,
                            cqm_4iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy
                                    as *const crate::stdlib::uint8_t,
                            cqm_4ic:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic
                                    as *const crate::stdlib::uint8_t,
                            cqm_8iy:
                                &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy
                                    as *const crate::stdlib::uint8_t,
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                        C2Rust_Unnamed_22 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                            cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        },
                    ],
                    [C2Rust_Unnamed_22 {
                        fps_num: 0,
                        fps_den: 0,
                        interlaced: 0,
                        frame_size: 0,
                        cqm_4iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        cqm_4ic: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                        cqm_8iy: ::core::ptr::null::<crate::stdlib::uint8_t>(),
                    }; 7],
                ],
            ];
            let mut res = -(1i32);
            if i_csp >= crate::x264_h::X264_CSP_I420
                && i_csp < crate::x264_h::X264_CSP_I422
                && type_0 == 0
            {
                if (*h).param.i_width == 1440i32 && (*h).param.i_height == 1080i32 {
                    res = 1i32;
                } else if (*h).param.i_width == 960i32 && (*h).param.i_height == 720i32 {
                    res = 0i32;
                }
            } else if i_csp >= crate::x264_h::X264_CSP_I422
                && i_csp < crate::x264_h::X264_CSP_I444
                && type_0 != 0
            {
                if type_0 < 3i32 {
                    if (*h).param.i_width == 1920i32 && (*h).param.i_height == 1080i32 {
                        res = 1i32;
                    } else if (*h).param.i_width == 2048i32 && (*h).param.i_height == 1080i32 {
                        res = 1i32;
                    } else if (*h).param.i_width == 1280i32 && (*h).param.i_height == 720i32 {
                        res = 0i32;
                    }
                } else if (*h).param.i_width == 3840i32 && (*h).param.i_height == 2160i32 {
                    res = 0i32;
                } else if (*h).param.i_width == 4096i32 && (*h).param.i_height == 2160i32 {
                    res = 0i32;
                }
            } else {
                log::error!(
                    "Invalid colorspace for AVC-Intra {}",
                    (*h).param.i_avcintra_class
                );
                return -(1i32);
            }
            if res < 0i32 {
                log::error!(
                    "Resolution {}x{} invalid for AVC-Intra {}",
                    (*h).param.i_width,
                    (*h).param.i_height,
                    (*h).param.i_avcintra_class
                );
                return -(1i32);
            }
            if (*h).param.nalu_process.is_some() {
                log::error!("nalu_process is not supported in AVC-Intra mode");
                return -(1i32);
            }
            if !(*h).param.repeat_headers {
                log::error!("Separate headers not supported in AVC-Intra mode");
                return -(1i32);
            }
            let mut fps_num = (*h).param.i_fps_num;
            let mut fps_den = (*h).param.i_fps_den;
            crate::src::common::base::x264_reduce_fraction(&raw mut fps_num, &raw mut fps_den);
            while i < 7i32 {
                if avcintra_lut[type_0 as usize][res as usize][i as usize].fps_num
                    as crate::stdlib::uint32_t
                    == fps_num
                    && avcintra_lut[type_0 as usize][res as usize][i as usize].fps_den
                        as crate::stdlib::uint32_t
                        == fps_den
                    && avcintra_lut[type_0 as usize][res as usize][i as usize].interlaced
                        as ::core::ffi::c_int
                        == (*h).param.interlaced as ::core::ffi::c_int
                {
                    break;
                }
                i += 1;
            }
            if i == 7i32 {
                log::error!(
                    "FPS {}/{}{} not compatible with AVC-Intra {}",
                    (*h).param.i_fps_num,
                    (*h).param.i_fps_den,
                    if (*h).param.interlaced { 'i' } else { 'p' },
                    (*h).param.i_avcintra_class
                );
                return -(1i32);
            }
            (*h).param.i_keyint_max = 1i32;
            (*h).param.intra_refresh = false;
            (*h).param.analyse.i_weighted_pred = 0i32;
            (*h).param.i_frame_reference = 1i32;
            (*h).param.i_dpb_size = 1i32;
            (*h).param.bluray_compat = false;
            (*h).param.vfr_input = false;
            (*h).param.aud = true;
            (*h).param.vui.i_chroma_loc = 0i32;
            (*h).param.i_nal_hrd = crate::x264_h::X264_NAL_HRD_NONE;
            (*h).param.deblocking_filter = false;
            (*h).param.stitchable = true;
            (*h).param.pic_struct = false;
            (*h).param.analyse.transform_8x8 = true;
            (*h).param.analyse.intra = crate::x264_h::X264_ANALYSE_I8x8;
            (*h).param.analyse.i_chroma_qp_offset = if type_0 > 2i32 {
                -(4i32)
            } else if res != 0 && type_0 != 0 {
                3i32
            } else {
                4i32
            };
            (*h).param.cabac = type_0 == 0;
            (*h).param.rc.i_vbv_buffer_size = avcintra_lut[type_0 as usize][res as usize]
                [i as usize]
                .frame_size as ::core::ffi::c_int;
            (*h).param.rc.i_bitrate = ((*h).param.rc.i_vbv_buffer_size as crate::stdlib::uint32_t)
                .wrapping_mul(fps_num)
                .wrapping_div(fps_den) as ::core::ffi::c_int;
            (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
            (*h).param.rc.i_rc_method = crate::x264_h::X264_RC_ABR;
            (*h).param.rc.f_vbv_buffer_init = 1.0f32;
            (*h).param.rc.filler = true;
            (*h).param.i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            crate::stdlib::memcpy(
                &raw mut (*h).param.cqm_4iy as *mut ::core::ffi::c_void,
                avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_4iy
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>(),
            );
            crate::stdlib::memcpy(
                &raw mut (*h).param.cqm_4ic as *mut ::core::ffi::c_void,
                avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_4ic
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>(),
            );
            crate::stdlib::memcpy(
                &raw mut (*h).param.cqm_8iy as *mut ::core::ffi::c_void,
                avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_8iy
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[crate::stdlib::uint8_t; 64]>(),
            );
            if (*h).param.i_avcintra_flavor == crate::x264_h::X264_AVCINTRA_FLAVOR_SONY {
                (*h).param.i_slice_count = 8i32;
                if (*h).param.sliced_threads {
                    (*h).param.i_threads = (*h).param.i_slice_count;
                }
            } else {
                (*h).param.i_slice_max_mbs = ((*h).param.i_width + 15i32) / 16i32
                    * (((*h).param.i_height + 15i32) / 16i32)
                    / 10i32;
                (*h).param.i_slice_max_size = 0i32;
                if (*h).param.sliced_threads {
                    if res != 0 {
                        (*h).param.i_threads = if (2i32) < (*h).param.i_threads {
                            2i32
                        } else {
                            (*h).param.i_threads
                        };
                    } else {
                        (*h).param.i_threads = if (5i32) < (*h).param.i_threads {
                            5i32
                        } else {
                            (*h).param.i_threads
                        };
                        if (*h).param.i_threads < 5i32 {
                            (*h).param.i_threads = 1i32;
                        }
                    }
                }
                (*h).param.rc.i_qp_min = if (*h).param.rc.i_qp_min > 6i32 * (8i32 - 8i32) + 1i32 {
                    (*h).param.rc.i_qp_min
                } else {
                    6i32 * (8i32 - 8i32) + 1i32
                };
            }
            if type_0 != 0 {
                (*h).param.vui.i_sar_height = 1i32;
                (*h).param.vui.i_sar_width = (*h).param.vui.i_sar_height;
            } else {
                (*h).param.vui.i_sar_width = 4i32;
                (*h).param.vui.i_sar_height = 3i32;
            }
        }
        (*h).param.rc.f_rf_constant = x264_clip3f(
            (*h).param.rc.f_rf_constant as ::core::ffi::c_double,
            -crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_double,
            51f64,
        ) as ::core::ffi::c_float;
        (*h).param.rc.f_rf_constant_max = x264_clip3f(
            (*h).param.rc.f_rf_constant_max as ::core::ffi::c_double,
            -crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_double,
            51f64,
        ) as ::core::ffi::c_float;
        (*h).param.rc.i_qp_constant = x264_clip3(
            (*h).param.rc.i_qp_constant,
            -(1i32),
            crate::src::common::common::QP_MAX,
        );
        (*h).param.analyse.i_subpel_refine =
            x264_clip3((*h).param.analyse.i_subpel_refine, 0i32, 11i32);
        (*h).param.rc.f_ip_factor = x264_clip3f(
            (*h).param.rc.f_ip_factor as ::core::ffi::c_double,
            0.01,
            10.0,
        ) as ::core::ffi::c_float;
        (*h).param.rc.f_pb_factor = x264_clip3f(
            (*h).param.rc.f_pb_factor as ::core::ffi::c_double,
            0.01,
            10.0,
        ) as ::core::ffi::c_float;
        if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF {
            (*h).param.rc.i_qp_constant = ((*h).param.rc.f_rf_constant
                + crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_float)
                as ::core::ffi::c_int;
            (*h).param.rc.i_bitrate = 0i32;
        }
        if b_open != 0
            && ((*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CQP
                || (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF)
            && (*h).param.rc.i_qp_constant == 0i32
        {
            (*h).mb.lossless = true;
            (*h).param.i_cqm_preset = crate::x264_h::X264_CQM_FLAT;
            (*h).param.psz_cqm_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*h).param.rc.i_rc_method = crate::x264_h::X264_RC_CQP;
            (*h).param.rc.f_ip_factor = 1f32;
            (*h).param.rc.f_pb_factor = 1f32;
            (*h).param.analyse.psnr = false;
            (*h).param.analyse.ssim = false;
            (*h).param.analyse.i_chroma_qp_offset = 0i32;
            (*h).param.analyse.i_trellis = 0i32;
            (*h).param.analyse.fast_pskip = false;
            (*h).param.analyse.i_noise_reduction = 0i32;
            (*h).param.analyse.psy = false;
            (*h).param.i_bframe = 0i32;
            if !(*h).param.cabac && (*h).param.analyse.i_subpel_refine < 6i32 {
                (*h).param.analyse.transform_8x8 = false;
            }
        }
        if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CQP {
            let mut qp_p = (*h).param.rc.i_qp_constant as ::core::ffi::c_float;
            let mut qp_i = qp_p - 6f32 * crate::stdlib::log2f((*h).param.rc.f_ip_factor);
            let mut qp_b = qp_p + 6f32 * crate::stdlib::log2f((*h).param.rc.f_pb_factor);
            if qp_p < 0f32 {
                log::error!("qp not specified");
                return -(1i32);
            }
            (*h).param.rc.i_qp_min = x264_clip3(
                (if qp_p < (if qp_i < qp_b { qp_i } else { qp_b }) {
                    qp_p
                } else if qp_i < qp_b {
                    qp_i
                } else {
                    qp_b
                }) as ::core::ffi::c_int,
                0i32,
                crate::src::common::common::QP_MAX,
            );
            (*h).param.rc.i_qp_max = x264_clip3(
                ((if qp_p > (if qp_i > qp_b { qp_i } else { qp_b }) {
                    qp_p
                } else {
                    if qp_i > qp_b { qp_i } else { qp_b }
                }) as ::core::ffi::c_double
                    + 0.999) as ::core::ffi::c_int,
                0i32,
                crate::src::common::common::QP_MAX,
            );
            (*h).param.rc.i_aq_mode = 0i32;
            (*h).param.rc.mb_tree = false;
            (*h).param.rc.i_bitrate = 0i32;
        }
        (*h).param.rc.i_qp_max = x264_clip3(
            (*h).param.rc.i_qp_max,
            0i32,
            crate::src::common::common::QP_MAX,
        );
        (*h).param.rc.i_qp_min = x264_clip3((*h).param.rc.i_qp_min, 0i32, (*h).param.rc.i_qp_max);
        (*h).param.rc.i_qp_step = x264_clip3(
            (*h).param.rc.i_qp_step,
            2i32,
            crate::src::common::common::QP_MAX,
        );
        (*h).param.rc.i_bitrate = x264_clip3((*h).param.rc.i_bitrate, 0i32, 2000000i32);
        if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR && (*h).param.rc.i_bitrate == 0 {
            log::error!("bitrate not specified");
            return -(1i32);
        }
        (*h).param.rc.i_vbv_buffer_size =
            x264_clip3((*h).param.rc.i_vbv_buffer_size, 0i32, 2000000i32);
        (*h).param.rc.i_vbv_max_bitrate =
            x264_clip3((*h).param.rc.i_vbv_max_bitrate, 0i32, 2000000i32);
        (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
            (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double,
            0f64,
            2000000f64,
        ) as ::core::ffi::c_float;
        if (*h).param.rc.i_vbv_buffer_size != 0 {
            if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CQP {
                log::warn!("VBV is incompatible with constant QP, ignored.");
                (*h).param.rc.i_vbv_max_bitrate = 0i32;
                (*h).param.rc.i_vbv_buffer_size = 0i32;
            } else if (*h).param.rc.i_vbv_max_bitrate == 0i32 {
                if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR {
                    log::warn!("VBV maxrate unspecified, assuming CBR");
                    (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
                } else {
                    log::warn!("VBV bufsize set but maxrate unspecified, ignored");
                    (*h).param.rc.i_vbv_buffer_size = 0i32;
                }
            } else if (*h).param.rc.i_vbv_max_bitrate < (*h).param.rc.i_bitrate
                && (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
            {
                log::warn!("max bitrate less than average bitrate, assuming CBR");
                (*h).param.rc.i_bitrate = (*h).param.rc.i_vbv_max_bitrate;
            }
        } else if (*h).param.rc.i_vbv_max_bitrate != 0 {
            log::warn!("VBV maxrate specified, but no bufsize, ignored");
            (*h).param.rc.i_vbv_max_bitrate = 0i32;
        }
        (*h).param.i_slice_max_size = if (*h).param.i_slice_max_size > 0i32 {
            (*h).param.i_slice_max_size
        } else {
            0i32
        };
        (*h).param.i_slice_max_mbs = if (*h).param.i_slice_max_mbs > 0i32 {
            (*h).param.i_slice_max_mbs
        } else {
            0i32
        };
        (*h).param.i_slice_min_mbs = if (*h).param.i_slice_min_mbs > 0i32 {
            (*h).param.i_slice_min_mbs
        } else {
            0i32
        };
        if (*h).param.i_slice_max_mbs != 0 {
            (*h).param.i_slice_min_mbs =
                if (*h).param.i_slice_min_mbs < (*h).param.i_slice_max_mbs / 2i32 {
                    (*h).param.i_slice_min_mbs
                } else {
                    (*h).param.i_slice_max_mbs / 2i32
                };
        } else if (*h).param.i_slice_max_size == 0 {
            (*h).param.i_slice_min_mbs = 0i32;
        }
        if (*h).param.interlaced && (*h).param.i_slice_min_mbs != 0 {
            log::warn!("interlace + slice-min-mbs is not implemented");
            (*h).param.i_slice_min_mbs = 0i32;
        }
        let mut mb_width = ((*h).param.i_width + 15i32) / 16i32;
        if (*h).param.i_slice_min_mbs > mb_width {
            log::warn!("slice-min-mbs > row mb size ({mb_width}) not implemented");
            (*h).param.i_slice_min_mbs = mb_width;
        }
        let mut max_slices = ((*h).param.i_height
            + (((16i32) << (*h).param.interlaced as ::core::ffi::c_int) - 1i32))
            / ((16i32) << (*h).param.interlaced as ::core::ffi::c_int);
        if (*h).param.sliced_threads {
            (*h).param.i_slice_count = x264_clip3((*h).param.i_threads, 0i32, max_slices);
        } else {
            (*h).param.i_slice_count = x264_clip3((*h).param.i_slice_count, 0i32, max_slices);
            if (*h).param.i_slice_max_mbs != 0 || (*h).param.i_slice_max_size != 0 {
                (*h).param.i_slice_count = 0i32;
            }
        }
        if (*h).param.i_slice_count_max > 0i32 {
            (*h).param.i_slice_count_max =
                if (*h).param.i_slice_count > (*h).param.i_slice_count_max {
                    (*h).param.i_slice_count
                } else {
                    (*h).param.i_slice_count_max
                };
        }
        if (*h).param.bluray_compat {
            (*h).param.i_bframe_pyramid = if (1i32) < (*h).param.i_bframe_pyramid {
                1i32
            } else {
                (*h).param.i_bframe_pyramid
            };
            (*h).param.i_bframe = if (*h).param.i_bframe < 3i32 {
                (*h).param.i_bframe
            } else {
                3i32
            };
            (*h).param.aud = true;
            (*h).param.i_nal_hrd = if (*h).param.i_nal_hrd > 1i32 {
                (*h).param.i_nal_hrd
            } else {
                1i32
            };
            (*h).param.i_slice_max_size = 0i32;
            (*h).param.i_slice_max_mbs = 0i32;
            (*h).param.intra_refresh = false;
            (*h).param.i_frame_reference = if (*h).param.i_frame_reference < 6i32 {
                (*h).param.i_frame_reference
            } else {
                6i32
            };
            (*h).param.i_dpb_size = if (*h).param.i_dpb_size < 6i32 {
                (*h).param.i_dpb_size
            } else {
                6i32
            };
            (*h).param.i_keyint_min = 1i32;
            (*h).param.analyse.i_weighted_pred = if (*h).param.analyse.i_weighted_pred < 1i32 {
                (*h).param.analyse.i_weighted_pred
            } else {
                1i32
            };
            if (*h).param.fake_interlaced {
                (*h).param.pic_struct = true;
            }
        }
        (*h).param.i_frame_reference = x264_clip3(
            (*h).param.i_frame_reference,
            1i32,
            crate::src::common::base::X264_REF_MAX,
        );
        (*h).param.i_dpb_size = x264_clip3(
            (*h).param.i_dpb_size,
            1i32,
            crate::src::common::base::X264_REF_MAX,
        );
        if (*h).param.i_scenecut_threshold < 0i32 {
            (*h).param.i_scenecut_threshold = 0i32;
        }
        (*h).param.analyse.i_direct_mv_pred = x264_clip3(
            (*h).param.analyse.i_direct_mv_pred,
            crate::x264_h::X264_DIRECT_PRED_NONE,
            crate::x264_h::X264_DIRECT_PRED_AUTO,
        );
        if (*h).param.analyse.i_subpel_refine == 0
            && (*h).param.analyse.i_direct_mv_pred > crate::x264_h::X264_DIRECT_PRED_SPATIAL
        {
            log::warn!("subme=0 + direct=temporal is not supported");
            (*h).param.analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_SPATIAL;
        }
        (*h).param.i_bframe = x264_clip3(
            (*h).param.i_bframe,
            0i32,
            if (16i32) < (*h).param.i_keyint_max - 1i32 {
                16i32
            } else {
                (*h).param.i_keyint_max - 1i32
            },
        );
        (*h).param.i_bframe_bias = x264_clip3((*h).param.i_bframe_bias, -(90i32), 100i32);
        if (*h).param.i_bframe <= 1i32 {
            (*h).param.i_bframe_pyramid = crate::x264_h::X264_B_PYRAMID_NONE;
        }
        (*h).param.i_bframe_pyramid = x264_clip3(
            (*h).param.i_bframe_pyramid,
            crate::x264_h::X264_B_PYRAMID_NONE,
            crate::x264_h::X264_B_PYRAMID_NORMAL,
        );
        (*h).param.i_bframe_adaptive = x264_clip3(
            (*h).param.i_bframe_adaptive,
            crate::x264_h::X264_B_ADAPT_NONE,
            crate::x264_h::X264_B_ADAPT_TRELLIS,
        );
        if (*h).param.i_bframe == 0 {
            (*h).param.i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_NONE;
            (*h).param.analyse.i_direct_mv_pred = 0i32;
            (*h).param.analyse.weighted_bipred = false;
            (*h).param.open_gop = false;
        }
        if (*h).param.intra_refresh
            && (*h).param.i_bframe_pyramid == crate::x264_h::X264_B_PYRAMID_NORMAL
        {
            log::warn!("b-pyramid normal + intra-refresh is not supported");
            (*h).param.i_bframe_pyramid = crate::x264_h::X264_B_PYRAMID_STRICT;
        }
        if (*h).param.intra_refresh
            && ((*h).param.i_frame_reference > 1i32 || (*h).param.i_dpb_size > 1i32)
        {
            log::warn!("ref > 1 + intra-refresh is not supported");
            (*h).param.i_frame_reference = 1i32;
            (*h).param.i_dpb_size = 1i32;
        }
        if (*h).param.intra_refresh && (*h).param.open_gop {
            log::warn!("intra-refresh is not compatible with open-gop");
            (*h).param.open_gop = false;
        }
        if (*h).param.i_fps_num == 0 || (*h).param.i_fps_den == 0 {
            (*h).param.i_fps_num = 25u32;
            (*h).param.i_fps_den = 1u32;
        }
        let mut fps = (*h).param.i_fps_num as ::core::ffi::c_float
            / (*h).param.i_fps_den as ::core::ffi::c_float;
        if (*h).param.i_keyint_min == crate::x264_h::X264_KEYINT_MIN_AUTO {
            (*h).param.i_keyint_min =
                if ((*h).param.i_keyint_max / 10i32) < fps as ::core::ffi::c_int {
                    (*h).param.i_keyint_max / 10i32
                } else {
                    fps as ::core::ffi::c_int
                };
        }
        (*h).param.i_keyint_min = x264_clip3(
            (*h).param.i_keyint_min,
            1i32,
            (*h).param.i_keyint_max / 2i32 + 1i32,
        );
        (*h).param.rc.i_lookahead = x264_clip3(
            (*h).param.rc.i_lookahead,
            0i32,
            crate::src::common::base::X264_LOOKAHEAD_MAX,
        );
        let mut maxrate = if (*h).param.rc.i_vbv_max_bitrate > (*h).param.rc.i_bitrate {
            (*h).param.rc.i_vbv_max_bitrate
        } else {
            (*h).param.rc.i_bitrate
        };
        let mut bufsize = if maxrate != 0 {
            (*h).param.rc.i_vbv_buffer_size as ::core::ffi::c_float
                / maxrate as ::core::ffi::c_float
        } else {
            0f32
        };
        (*h).param.rc.i_lookahead = (if ((*h).param.rc.i_lookahead as ::core::ffi::c_float)
            < (if (*h).param.i_keyint_max as ::core::ffi::c_float > bufsize * fps {
                (*h).param.i_keyint_max as ::core::ffi::c_float
            } else {
                bufsize * fps
            }) {
            (*h).param.rc.i_lookahead as ::core::ffi::c_float
        } else if (*h).param.i_keyint_max as ::core::ffi::c_float > bufsize * fps {
            (*h).param.i_keyint_max as ::core::ffi::c_float
        } else {
            bufsize * fps
        }) as ::core::ffi::c_int;
        if (*h).param.i_timebase_num == 0
            || (*h).param.i_timebase_den == 0
            || !((*h).param.vfr_input || (*h).param.pulldown)
        {
            (*h).param.i_timebase_num = (*h).param.i_fps_den;
            (*h).param.i_timebase_den = (*h).param.i_fps_num;
        }
        (*h).param.rc.f_qcompress =
            x264_clip3f((*h).param.rc.f_qcompress as ::core::ffi::c_double, 0.0, 1.0)
                as ::core::ffi::c_float;
        if (*h).param.i_keyint_max == 1i32 || (*h).param.rc.f_qcompress == 1f32 {
            (*h).param.rc.mb_tree = false;
        }
        if !(*h).param.intra_refresh
            && (*h).param.i_keyint_max != crate::x264_h::X264_KEYINT_MAX_INFINITE
            && (*h).param.rc.i_lookahead == 0
            && (*h).param.rc.mb_tree
        {
            log::warn!("lookaheadless mb-tree requires intra refresh or infinite keyint");
            (*h).param.rc.mb_tree = false;
        }
        if b_open != 0 && (*h).param.rc.stat_read {
            (*h).param.rc.i_lookahead = 0i32;
        }
        if (*h).param.i_sync_lookahead < 0i32 {
            (*h).param.i_sync_lookahead = (*h).param.i_bframe + 1i32;
        }
        (*h).param.i_sync_lookahead = if (*h).param.i_sync_lookahead < 250i32 {
            (*h).param.i_sync_lookahead
        } else {
            250i32
        };
        if (*h).param.rc.stat_read || (*h).i_thread_frames == 1i32 {
            (*h).param.i_sync_lookahead = 0i32;
        }
        (*h).param.i_deblocking_filter_alphac0 =
            x264_clip3((*h).param.i_deblocking_filter_alphac0, -(6i32), 6i32);
        (*h).param.i_deblocking_filter_beta =
            x264_clip3((*h).param.i_deblocking_filter_beta, -(6i32), 6i32);
        (*h).param.analyse.i_luma_deadzone[0usize] =
            x264_clip3((*h).param.analyse.i_luma_deadzone[0usize], 0i32, 32i32);
        (*h).param.analyse.i_luma_deadzone[1usize] =
            x264_clip3((*h).param.analyse.i_luma_deadzone[1usize], 0i32, 32i32);
        (*h).param.i_cabac_init_idc = x264_clip3((*h).param.i_cabac_init_idc, 0i32, 2i32);
        if (*h).param.i_cqm_preset < crate::x264_h::X264_CQM_FLAT
            || (*h).param.i_cqm_preset > crate::x264_h::X264_CQM_CUSTOM
        {
            (*h).param.i_cqm_preset = crate::x264_h::X264_CQM_FLAT;
        }
        if (*h).param.analyse.i_me_method < crate::x264_h::X264_ME_DIA
            || (*h).param.analyse.i_me_method > crate::x264_h::X264_ME_TESA
        {
            (*h).param.analyse.i_me_method = crate::x264_h::X264_ME_HEX;
        }
        (*h).param.analyse.i_me_range = x264_clip3((*h).param.analyse.i_me_range, 4i32, 1024i32);
        if (*h).param.analyse.i_me_range > 16i32
            && (*h).param.analyse.i_me_method <= crate::x264_h::X264_ME_HEX
        {
            (*h).param.analyse.i_me_range = 16i32;
        }
        if (*h).param.analyse.i_me_method == crate::x264_h::X264_ME_TESA
            && ((*h).mb.lossless || (*h).param.analyse.i_subpel_refine <= 1i32)
        {
            (*h).param.analyse.i_me_method = crate::x264_h::X264_ME_ESA;
        }
        (*h).param.analyse.mixed_references =
            (*h).param.analyse.mixed_references && (*h).param.i_frame_reference > 1i32;
        (*h).param.analyse.inter &= crate::x264_h::X264_ANALYSE_PSUB16x16
            | crate::x264_h::X264_ANALYSE_PSUB8x8
            | crate::x264_h::X264_ANALYSE_BSUB16x16
            | crate::x264_h::X264_ANALYSE_I4x4
            | crate::x264_h::X264_ANALYSE_I8x8;
        (*h).param.analyse.intra &=
            crate::x264_h::X264_ANALYSE_I4x4 | crate::x264_h::X264_ANALYSE_I8x8;
        if (*h).param.analyse.inter & crate::x264_h::X264_ANALYSE_PSUB16x16 == 0 {
            (*h).param.analyse.inter &= !crate::x264_h::X264_ANALYSE_PSUB8x8;
        }
        if !(*h).param.analyse.transform_8x8 {
            (*h).param.analyse.inter &= !crate::x264_h::X264_ANALYSE_I8x8;
            (*h).param.analyse.intra &= !crate::x264_h::X264_ANALYSE_I8x8;
        }
        (*h).param.analyse.i_trellis = x264_clip3((*h).param.analyse.i_trellis, 0i32, 2i32);
        (*h).param.rc.i_aq_mode = x264_clip3((*h).param.rc.i_aq_mode, 0i32, 3i32);
        (*h).param.rc.f_aq_strength = x264_clip3f(
            (*h).param.rc.f_aq_strength as ::core::ffi::c_double,
            0f64,
            3f64,
        ) as ::core::ffi::c_float;
        if (*h).param.rc.f_aq_strength == 0f32 {
            (*h).param.rc.i_aq_mode = 0i32;
        }
        if (*h).param.i_log_level < crate::x264_h::X264_LOG_INFO {
            (*h).param.analyse.psnr = false;
            (*h).param.analyse.ssim = false;
        }
        if b_open != 0 && ((*h).param.analyse.psnr || (*h).param.analyse.ssim) {
            let mut s = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if (*h).param.analyse.psy {
                s = (if (*h).param.analyse.psnr {
                    b"psnr\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"ssim\0".as_ptr() as *const ::core::ffi::c_char
                }) as *mut ::core::ffi::c_char;
                log::warn!(
                    "--{} used with psy on: results will be invalid!",
                    std::ffi::CStr::from_ptr(s).to_string_lossy()
                );
            } else if (*h).param.rc.i_aq_mode == 0 && (*h).param.analyse.ssim {
                log::warn!("--ssim used with AQ off: results will be invalid!");
                s = b"ssim\0".as_ptr() as *mut ::core::ffi::c_char;
            } else if (*h).param.rc.i_aq_mode != 0 && (*h).param.analyse.psnr {
                log::warn!("--psnr used with AQ on: results will be invalid!");
                s = b"psnr\0".as_ptr() as *mut ::core::ffi::c_char;
            }
            if !s.is_null() {
                let s_str = std::ffi::CStr::from_ptr(s).to_string_lossy();
                log::warn!(
                    "--tune {} should be used if attempting to benchmark {}!",
                    s_str,
                    s_str
                );
            }
        }
        if !(*h).param.analyse.psy {
            (*h).param.analyse.f_psy_rd = 0f32;
            (*h).param.analyse.f_psy_trellis = 0f32;
        }
        (*h).param.analyse.f_psy_rd = x264_clip3f(
            (*h).param.analyse.f_psy_rd as ::core::ffi::c_double,
            0f64,
            10f64,
        ) as ::core::ffi::c_float;
        (*h).param.analyse.f_psy_trellis = x264_clip3f(
            (*h).param.analyse.f_psy_trellis as ::core::ffi::c_double,
            0f64,
            10f64,
        ) as ::core::ffi::c_float;
        (*h).mb.i_psy_rd = if (*h).param.analyse.i_subpel_refine >= 6i32 {
            (((*h).param.analyse.f_psy_rd * ((1i32) << 8i32) as ::core::ffi::c_float)
                as ::core::ffi::c_double
                + 0.5) as ::core::ffi::c_int
        } else {
            0i32
        };
        (*h).mb.i_psy_trellis = if (*h).param.analyse.i_trellis != 0 {
            (((*h).param.analyse.f_psy_trellis / 4f32 * ((1i32) << 8i32) as ::core::ffi::c_float)
                as ::core::ffi::c_double
                + 0.5) as ::core::ffi::c_int
        } else {
            0i32
        };
        (*h).param.analyse.i_chroma_qp_offset =
            x264_clip3((*h).param.analyse.i_chroma_qp_offset, -(32i32), 32i32);
        if b_open != 0
            && i_csp >= crate::x264_h::X264_CSP_I444
            && i_csp < crate::x264_h::X264_CSP_BGR
            && (*h).param.analyse.psy
        {
            (*h).param.analyse.i_chroma_qp_offset += 6i32;
        }
        if b_open != 0 && (*h).mb.i_psy_rd != 0 && (*h).param.i_avcintra_class == 0 {
            (*h).param.analyse.i_chroma_qp_offset -=
                if ((*h).param.analyse.f_psy_rd as ::core::ffi::c_double) < 0.25f64 {
                    1i32
                } else {
                    2i32
                };
        }
        if b_open != 0 && (*h).mb.i_psy_trellis != 0 && (*h).param.i_avcintra_class == 0 {
            (*h).param.analyse.i_chroma_qp_offset -=
                if ((*h).param.analyse.f_psy_trellis as ::core::ffi::c_double) < 0.25f64 {
                    1i32
                } else {
                    2i32
                };
        }
        (*h).param.analyse.i_chroma_qp_offset =
            x264_clip3((*h).param.analyse.i_chroma_qp_offset, -(12i32), 12i32);
        if (*h).param.rc.i_aq_mode == 0 && (*h).param.rc.mb_tree {
            (*h).param.rc.i_aq_mode = 1i32;
            (*h).param.rc.f_aq_strength = 0f32;
        }
        (*h).param.analyse.i_noise_reduction =
            x264_clip3((*h).param.analyse.i_noise_reduction, 0i32, (1i32) << 16i32);
        if (*h).param.analyse.i_subpel_refine >= 10i32
            && ((*h).param.analyse.i_trellis != 2i32 || (*h).param.rc.i_aq_mode == 0)
        {
            (*h).param.analyse.i_subpel_refine = 9i32;
        }
        if b_open != 0 {
            let mut l = &raw const crate::src::common::tables::x264_levels
                as *const crate::x264_h::x264_level_t;
            if (*h).param.i_level_idc < 0i32 {
                let mut maxrate_bak = (*h).param.rc.i_vbv_max_bitrate;
                if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
                    && (*h).param.rc.i_vbv_buffer_size <= 0i32
                {
                    (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate * 2i32;
                }
                crate::src::encoder::set::x264_8_sps_init(
                    &mut (*h).sps,
                    (*h).param.i_sps_id,
                    &raw mut (*h).param,
                );
                loop {
                    (*h).param.i_level_idc = (*l).level_idc as ::core::ffi::c_int;
                    if !((*l.offset(1isize)).level_idc as ::core::ffi::c_int != 0
                        && crate::src::encoder::set::x264_8_validate_levels(h, 0i32) != 0
                        && {
                            let c2rust_fresh0 = l;
                            l = l.offset(1);
                            !c2rust_fresh0.is_null()
                        })
                    {
                        break;
                    }
                }
                (*h).param.rc.i_vbv_max_bitrate = maxrate_bak;
            } else {
                while (*l).level_idc as ::core::ffi::c_int != 0
                    && (*l).level_idc as ::core::ffi::c_int != (*h).param.i_level_idc
                {
                    l = l.offset(1);
                }
                if (*l).level_idc as ::core::ffi::c_int == 0i32 {
                    log::error!("invalid level_idc: {}", (*h).param.i_level_idc);
                    return -(1i32);
                }
            }
            if (*h).param.analyse.i_mv_range <= 0i32 {
                (*h).param.analyse.i_mv_range = (*l).mv_range as ::core::ffi::c_int
                    >> (*h).param.interlaced as ::core::ffi::c_int;
            } else {
                (*h).param.analyse.i_mv_range = x264_clip3(
                    (*h).param.analyse.i_mv_range,
                    32i32,
                    8192i32 >> (*h).param.interlaced as ::core::ffi::c_int,
                );
            }
        }
        (*h).param.analyse.i_weighted_pred = x264_clip3(
            (*h).param.analyse.i_weighted_pred,
            crate::x264_h::X264_WEIGHTP_NONE,
            crate::x264_h::X264_WEIGHTP_SMART,
        );
        if (*h).param.i_lookahead_threads == crate::x264_h::X264_THREADS_AUTO {
            if (*h).param.sliced_threads {
                (*h).param.i_lookahead_threads = (*h).param.i_threads;
            } else {
                let mut badapt = ((*h).param.i_bframe_adaptive
                    == crate::x264_h::X264_B_ADAPT_TRELLIS)
                    as ::core::ffi::c_int;
                let mut subme = (if ((*h).param.analyse.i_subpel_refine / 3i32) < 3i32 {
                    (*h).param.analyse.i_subpel_refine / 3i32
                } else {
                    3i32
                }) + ((*h).param.analyse.i_subpel_refine > 1i32)
                    as ::core::ffi::c_int;
                let mut bframes = if (((*h).param.i_bframe - 1i32) / 3i32) < 3i32 {
                    ((*h).param.i_bframe - 1i32) / 3i32
                } else {
                    3i32
                };
                static mut lookahead_thread_div: [[[crate::stdlib::uint8_t; 4]; 5]; 2] = [
                    [
                        [6u8, 6u8, 6u8, 6u8],
                        [3u8, 3u8, 3u8, 3u8],
                        [4u8, 4u8, 4u8, 4u8],
                        [6u8, 6u8, 6u8, 6u8],
                        [12u8, 12u8, 12u8, 12u8],
                    ],
                    [
                        [3u8, 2u8, 1u8, 1u8],
                        [2u8, 1u8, 1u8, 1u8],
                        [4u8, 3u8, 2u8, 1u8],
                        [6u8, 4u8, 3u8, 2u8],
                        [12u8, 9u8, 6u8, 4u8],
                    ],
                ];
                (*h).param.i_lookahead_threads = (*h).param.i_threads
                    / lookahead_thread_div[badapt as usize][subme as usize][bframes as usize]
                        as ::core::ffi::c_int;
                (*h).param.i_lookahead_threads =
                    if (*h).param.i_lookahead_threads < (*h).param.i_height / 128i32 {
                        (*h).param.i_lookahead_threads
                    } else {
                        (*h).param.i_height / 128i32
                    };
            }
        }
        (*h).param.i_lookahead_threads = x264_clip3(
            (*h).param.i_lookahead_threads,
            1i32,
            if max_sliced_threads < 16i32 {
                max_sliced_threads
            } else {
                16i32
            },
        );
        if (*h).param.interlaced {
            if (*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA {
                log::warn!("interlace + me=esa is not implemented");
                (*h).param.analyse.i_me_method = crate::x264_h::X264_ME_UMH;
            }
            if (*h).param.analyse.i_weighted_pred > 0i32 {
                log::warn!("interlace + weightp is not implemented");
                (*h).param.analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_NONE;
            }
        }
        if (*h).param.analyse.i_weighted_pred == 0
            && (*h).param.rc.mb_tree
            && (*h).param.analyse.psy
        {
            (*h).param.analyse.i_weighted_pred = crate::src::common::base::X264_WEIGHTP_FAKE;
        }
        if (*h).i_thread_frames > 1i32 {
            let mut r = (*h).param.analyse.i_mv_range_thread;
            if r <= 0i32 {
                let mut max_range = ((*h).param.i_height
                    + crate::src::common::base::X264_THREAD_HEIGHT)
                    / (*h).i_thread_frames
                    - crate::src::common::base::X264_THREAD_HEIGHT;
                r = max_range / 2i32;
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
            let mut r2 = (r & !(15i32)) + (-crate::src::common::base::X264_THREAD_HEIGHT & 15i32);
            if r2 < r {
                r2 += 16i32;
            }
            log::debug!("using mv_range_thread = {r2}");
            (*h).param.analyse.i_mv_range_thread = r2;
        }
        if (*h).param.rc.f_rate_tolerance < 0f32 {
            (*h).param.rc.f_rate_tolerance = 0f32;
        }
        if (*h).param.rc.f_qblur < 0f32 {
            (*h).param.rc.f_qblur = 0f32;
        }
        if (*h).param.rc.f_complexity_blur < 0f32 {
            (*h).param.rc.f_complexity_blur = 0f32;
        }
        (*h).param.i_sps_id &= 31i32;
        (*h).param.i_nal_hrd = x264_clip3(
            (*h).param.i_nal_hrd,
            crate::x264_h::X264_NAL_HRD_NONE,
            crate::x264_h::X264_NAL_HRD_CBR,
        );
        if (*h).param.i_nal_hrd != 0 && (*h).param.rc.i_vbv_buffer_size == 0 {
            log::warn!("NAL HRD parameters require VBV parameters");
            (*h).param.i_nal_hrd = crate::x264_h::X264_NAL_HRD_NONE;
        }
        if (*h).param.i_nal_hrd == crate::x264_h::X264_NAL_HRD_CBR
            && ((*h).param.rc.i_bitrate != (*h).param.rc.i_vbv_max_bitrate
                || (*h).param.rc.i_vbv_max_bitrate == 0)
        {
            log::warn!("CBR HRD requires constant bitrate");
            (*h).param.i_nal_hrd = crate::x264_h::X264_NAL_HRD_VBR;
        }
        if (*h).param.i_nal_hrd == crate::x264_h::X264_NAL_HRD_CBR {
            (*h).param.rc.filler = true;
        }
        return 0i32;
    }
}
pub const MAX_RESOLUTION: ::core::ffi::c_int = 16384i32;
unsafe extern "C" fn mbcmp_init(mut h: *mut x264_t) {
    unsafe {
        let mut satd =
            (!(*h).mb.lossless && (*h).param.analyse.i_subpel_refine > 1i32) as ::core::ffi::c_int;
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.mbcmp as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd as *mut crate::src::common::pixel::x264_pixel_cmp_t
            } else {
                &raw mut (*h).pixf.sad_aligned as *mut crate::src::common::pixel::x264_pixel_cmp_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_t; 8]>(),
        );
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.mbcmp_unaligned as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd as *mut crate::src::common::pixel::x264_pixel_cmp_t
            } else {
                &raw mut (*h).pixf.sad as *mut crate::src::common::pixel::x264_pixel_cmp_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_t; 8]>(),
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
        (*h).pixf.intra_mbcmp_x9_4x4 = if (*h).param.cpu_independent || (*h).mb.lossless {
            None
        } else if satd != 0 {
            (*h).pixf.intra_satd_x9_4x4
        } else {
            (*h).pixf.intra_sad_x9_4x4
        };
        (*h).pixf.intra_mbcmp_x9_8x8 = if (*h).param.cpu_independent || (*h).mb.lossless {
            None
        } else if satd != 0 {
            (*h).pixf.intra_sa8d_x9_8x8
        } else {
            (*h).pixf.intra_sad_x9_8x8
        };
        satd &=
            ((*h).param.analyse.i_me_method == crate::x264_h::X264_ME_TESA) as ::core::ffi::c_int;
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.fpelcmp as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd as *mut crate::src::common::pixel::x264_pixel_cmp_t
            } else {
                &raw mut (*h).pixf.sad as *mut crate::src::common::pixel::x264_pixel_cmp_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_t; 8]>(),
        );
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.fpelcmp_x3 as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd_x3 as *mut crate::src::common::pixel::x264_pixel_cmp_x3_t
            } else {
                &raw mut (*h).pixf.sad_x3 as *mut crate::src::common::pixel::x264_pixel_cmp_x3_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_x3_t; 7]>(),
        );
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.fpelcmp_x4 as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd_x4 as *mut crate::src::common::pixel::x264_pixel_cmp_x4_t
            } else {
                &raw mut (*h).pixf.sad_x4 as *mut crate::src::common::pixel::x264_pixel_cmp_x4_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_x4_t; 7]>(),
        );
    }
}
unsafe extern "C" fn chroma_dsp_init(mut h: *mut x264_t) {
    unsafe {
        crate::stdlib::memcpy(
            &raw mut (*h).luma2chroma_pixel as *mut ::core::ffi::c_void,
            &raw const *(&raw const x264_luma2chroma_pixel as *const [crate::stdlib::uint8_t; 7])
                .offset((*h).sps.i_chroma_format_idc as isize)
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 7]>(),
        );
        match (*h).sps.i_chroma_format_idc {
            ChromaFormat::Chroma400 => {
                (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_400;
            }
            ChromaFormat::Chroma420 => {
                crate::stdlib::memcpy(
                    &raw mut (*h).predict_chroma as *mut ::core::ffi::c_void,
                    &raw mut (*h).predict_8x8c as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[crate::src::common::predict::x264_predict_t; 7]>(),
                );
                (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_420;
                (*h).loopf.deblock_chroma[0usize] = (*h).loopf.deblock_h_chroma_420;
                (*h).loopf.deblock_chroma_intra[0usize] = (*h).loopf.deblock_h_chroma_420_intra;
                (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_chroma_420_mbaff;
                (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_chroma_420_intra_mbaff;
                (*h).pixf.intra_mbcmp_x3_chroma = (*h).pixf.intra_mbcmp_x3_8x8c;
                (*h).quantf.coeff_last[crate::src::common::macroblock::DCT_CHROMA_DC
                    as ::core::ffi::c_int as usize] = (*h).quantf.coeff_last4;
                (*h).quantf.coeff_level_run[crate::src::common::macroblock::DCT_CHROMA_DC
                    as ::core::ffi::c_int as usize] = (*h).quantf.coeff_level_run4;
            }
            ChromaFormat::Chroma422 => {
                crate::stdlib::memcpy(
                    &raw mut (*h).predict_chroma as *mut ::core::ffi::c_void,
                    &raw mut (*h).predict_8x16c as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[crate::src::common::predict::x264_predict_t; 7]>(),
                );
                (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_422;
                (*h).loopf.deblock_chroma[0usize] = (*h).loopf.deblock_h_chroma_422;
                (*h).loopf.deblock_chroma_intra[0usize] = (*h).loopf.deblock_h_chroma_422_intra;
                (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_chroma_422_mbaff;
                (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_chroma_422_intra_mbaff;
                (*h).pixf.intra_mbcmp_x3_chroma = (*h).pixf.intra_mbcmp_x3_8x16c;
                (*h).quantf.coeff_last[crate::src::common::macroblock::DCT_CHROMA_DC
                    as ::core::ffi::c_int as usize] = (*h).quantf.coeff_last8;
                (*h).quantf.coeff_level_run[crate::src::common::macroblock::DCT_CHROMA_DC
                    as ::core::ffi::c_int as usize] = (*h).quantf.coeff_level_run8;
            }
            ChromaFormat::Chroma444 => {
                (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_422;
                (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_luma_mbaff;
                (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_luma_intra_mbaff;
            }
        };
    }
}
unsafe extern "C" fn set_aspect_ratio(
    mut h: *mut x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
    mut initial: ::core::ffi::c_int,
) {
    unsafe {
        if (*param).vui.i_sar_width > 0i32 && (*param).vui.i_sar_height > 0i32 {
            let mut i_w = (*param).vui.i_sar_width as crate::stdlib::uint32_t;
            let mut i_h = (*param).vui.i_sar_height as crate::stdlib::uint32_t;
            let mut old_w = (*h).param.vui.i_sar_width as crate::stdlib::uint32_t;
            let mut old_h = (*h).param.vui.i_sar_height as crate::stdlib::uint32_t;
            crate::src::common::base::x264_reduce_fraction(&raw mut i_w, &raw mut i_h);
            while i_w > 65535u32 || i_h > 65535u32 {
                i_w = i_w.wrapping_div(2u32);
                i_h = i_h.wrapping_div(2u32);
            }
            crate::src::common::base::x264_reduce_fraction(&raw mut i_w, &raw mut i_h);
            if i_w != old_w || i_h != old_h || initial != 0 {
                (*h).param.vui.i_sar_width = 0i32;
                (*h).param.vui.i_sar_height = 0i32;
                if i_w == 0u32 || i_h == 0u32 {
                    log::warn!("cannot create valid sample aspect ratio");
                } else {
                    if initial != 0 {
                        log::info!("using SAR={i_w}/{i_h}");
                    } else {
                        log::debug!("using SAR={i_w}/{i_h}");
                    }
                    (*h).param.vui.i_sar_width = i_w as ::core::ffi::c_int;
                    (*h).param.vui.i_sar_height = i_h as ::core::ffi::c_int;
                }
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_encoder_open<'a>(
    mut param: *mut crate::x264_h::x264_param_t,
    mut api: *mut ::core::ffi::c_void,
) -> *mut x264_t<'a> {
    unsafe {
        const subsampling: [&'static str; 4] = ["4:0:0", "4:2:0", "4:2:2", "4:4:4"];
        let mut h = crate::src::common::base::x264_malloc(
            ::core::mem::size_of::<x264_t>() as crate::stdlib::int64_t
        ) as *mut x264_t;
        if h.is_null() {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        crate::stdlib::memset(
            h as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<x264_t>(),
        );
        crate::stdlib::memcpy(
            &raw mut (*h).param as *mut ::core::ffi::c_void,
            param as *const ::core::ffi::c_void,
            ::core::mem::size_of::<crate::x264_h::x264_param_t>(),
        );
        (*h).param.opaque = crate::__stddef_null_h::NULL;
        (*h).param.param_free = None;
        if !(*h).param.psz_cqm_file.is_null() {
            (*h).param.psz_cqm_file =
                x264_param_strdup(&raw mut (*h).param, (*h).param.psz_cqm_file);
            if (*h).param.psz_cqm_file.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
        }
        if !(*h).param.psz_dump_yuv.is_null() {
            (*h).param.psz_dump_yuv =
                x264_param_strdup(&raw mut (*h).param, (*h).param.psz_dump_yuv);
            if (*h).param.psz_dump_yuv.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
        }
        if !(*h).param.rc.psz_stat_out.is_null() {
            (*h).param.rc.psz_stat_out =
                x264_param_strdup(&raw mut (*h).param, (*h).param.rc.psz_stat_out);
            if (*h).param.rc.psz_stat_out.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
        }
        if !(*h).param.rc.psz_stat_in.is_null() {
            (*h).param.rc.psz_stat_in =
                x264_param_strdup(&raw mut (*h).param, (*h).param.rc.psz_stat_in);
            if (*h).param.rc.psz_stat_in.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
        }
        if !(*h).param.rc.psz_zones.is_null() {
            (*h).param.rc.psz_zones =
                x264_param_strdup(&raw mut (*h).param, (*h).param.rc.psz_zones);
            if (*h).param.rc.psz_zones.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
        }
        if !(*h).param.psz_clbin_file.is_null() {
            (*h).param.psz_clbin_file =
                x264_param_strdup(&raw mut (*h).param, (*h).param.psz_clbin_file);
            if (*h).param.psz_clbin_file.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
        }
        if (*param).param_free.is_some() {
            crate::src::common::base::x264_param_cleanup(param);
            (*param).param_free.expect("non-null function pointer")(
                param as *mut ::core::ffi::c_void,
            );
        }
        (*h).api = api;
        if validate_parameters(h, 1i32) < 0 {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        if !(*h).param.psz_cqm_file.is_null() {
            if crate::src::common::set::x264_8_cqm_parse_file(h, (*h).param.psz_cqm_file) < 0i32 {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
        }
        crate::src::common::base::x264_reduce_fraction(
            &raw mut (*h).param.i_fps_num,
            &raw mut (*h).param.i_fps_den,
        );
        crate::src::common::base::x264_reduce_fraction(
            &raw mut (*h).param.i_timebase_num,
            &raw mut (*h).param.i_timebase_den,
        );
        (*h).i_frame = -(1i32);
        (*h).i_frame_num = 0i32;
        if (*h).param.i_avcintra_class != 0 {
            (*h).i_idr_pic_id = if (*h).param.i_avcintra_class > 200i32 {
                4i32
            } else {
                5i32
            };
        } else {
            (*h).i_idr_pic_id = 0i32;
        }
        if u64::from((*h).param.i_timebase_den) * 2 > u32::MAX.into() {
            log::error!(
                "Effective timebase denominator {} exceeds H.264 maximum",
                (*h).param.i_timebase_den
            );
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        set_aspect_ratio(h, &raw mut (*h).param, 1i32);
        crate::src::encoder::set::x264_8_sps_init(
            &mut (*h).sps,
            (*h).param.i_sps_id,
            &raw mut (*h).param,
        );
        crate::src::encoder::set::x264_8_sps_init_scaling_list(&mut (*h).sps, &raw mut (*h).param);
        crate::src::encoder::set::x264_8_pps_init(
            &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t,
            (*h).param.i_sps_id,
            &raw mut (*h).param,
            &(*h).sps,
        );
        crate::src::encoder::set::x264_8_validate_levels(h, 1i32);
        (*h).chroma_qp_table = (&raw const i_chroma_qp_table as *const crate::stdlib::uint8_t)
            .offset(12isize)
            .offset(
                (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset as isize,
            );
        if crate::src::common::set::x264_8_cqm_init(h) < 0 {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        let mut i_slicetype_length = 0;
        (*h).mb.i_mb_width = (*h).sps.i_mb_width;
        (*h).mb.i_mb_height = (*h).sps.i_mb_height;
        (*h).mb.i_mb_count = (*h).mb.i_mb_width * (*h).mb.i_mb_height;
        (*h).mb.chroma_h_shift = ((*h).sps.i_chroma_format_idc.is_420()
            || (*h).sps.i_chroma_format_idc.is_422())
            as ::core::ffi::c_int;
        (*h).mb.chroma_v_shift = ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
        (*h).mb.adaptive_mbaff = (*h).param.interlaced && (*h).param.analyse.i_subpel_refine != 0;
        if (*h).param.i_bframe_adaptive == crate::x264_h::X264_B_ADAPT_TRELLIS
            && !(*h).param.rc.stat_read
        {
            (*h).frames.i_delay = (if (*h).param.i_bframe > 3i32 {
                (*h).param.i_bframe
            } else {
                3i32
            }) * 4i32;
        } else {
            (*h).frames.i_delay = (*h).param.i_bframe;
        }
        if (*h).param.rc.mb_tree || (*h).param.rc.i_vbv_buffer_size != 0 {
            (*h).frames.i_delay = if (*h).frames.i_delay > (*h).param.rc.i_lookahead {
                (*h).frames.i_delay
            } else {
                (*h).param.rc.i_lookahead
            };
        }
        i_slicetype_length = (*h).frames.i_delay;
        (*h).frames.i_delay += (*h).i_thread_frames - 1i32;
        (*h).frames.i_delay += (*h).param.i_sync_lookahead;
        (*h).frames.i_delay += (*h).param.vfr_input as ::core::ffi::c_int;
        (*h).frames.i_bframe_delay = if (*h).param.i_bframe != 0 {
            if (*h).param.i_bframe_pyramid != 0 {
                2i32
            } else {
                1i32
            }
        } else {
            0i32
        };
        (*h).frames.i_max_ref0 = (*h).param.i_frame_reference;
        (*h).frames.i_max_ref1 = if (*h).sps.vui.i_num_reorder_frames < (*h).param.i_frame_reference
        {
            (*h).sps.vui.i_num_reorder_frames
        } else {
            (*h).param.i_frame_reference
        };
        (*h).frames.i_max_dpb = (*h).sps.vui.i_max_dec_frame_buffering;
        (*h).frames.have_lowres = !(*h).param.rc.stat_read
            && ((*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
                || (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF
                || (*h).param.i_bframe_adaptive != 0
                || (*h).param.i_scenecut_threshold != 0
                || (*h).param.rc.mb_tree
                || (*h).param.analyse.i_weighted_pred != 0);
        (*h).frames.have_lowres |=
            (*h).param.rc.stat_read && (*h).param.rc.i_vbv_buffer_size > 0i32;
        (*h).frames.have_sub8x8_esa =
            (*h).param.analyse.inter & crate::x264_h::X264_ANALYSE_PSUB8x8 != 0;
        (*h).frames.i_last_keyframe = -(*h).param.i_keyint_max;
        (*h).frames.i_last_idr = (*h).frames.i_last_keyframe;
        (*h).frames.i_input = 0i32;
        (*h).frames.i_second_largest_pts = -1i64;
        (*h).frames.i_largest_pts = (*h).frames.i_second_largest_pts;
        (*h).frames.i_poc_last_open_gop = -(1i32);
        (*h).cost_table = crate::src::common::base::x264_malloc(::core::mem::size_of::<
            crate::src::common::common::C2Rust_Unnamed_11,
        >()
            as crate::stdlib::int64_t)
            as *mut crate::src::common::common::C2Rust_Unnamed_11;
        if (*h).cost_table.is_null() {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        crate::stdlib::memset(
            (*h).cost_table as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<crate::src::common::common::C2Rust_Unnamed_11>(),
        );
        (*h).frames.unused[0usize] = crate::src::common::base::x264_malloc(
            (((*h).frames.i_delay + 3i32) as usize).wrapping_mul(::core::mem::size_of::<
                *mut crate::src::common::frame::x264_frame_t,
            >()) as crate::stdlib::int64_t,
        ) as *mut *mut crate::src::common::frame::x264_frame_t;
        if (*h).frames.unused[0usize].is_null() {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        crate::stdlib::memset(
            (*h).frames.unused[0usize] as *mut ::core::ffi::c_void,
            0i32,
            (((*h).frames.i_delay + 3i32) as crate::__stddef_size_t_h::size_t).wrapping_mul(
                ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>(),
            ),
        );
        (*h).frames.unused[1usize] = crate::src::common::base::x264_malloc(
            (((*h).i_thread_frames + 16i32 + 4i32) as usize).wrapping_mul(::core::mem::size_of::<
                *mut crate::src::common::frame::x264_frame_t,
            >()) as crate::stdlib::int64_t,
        ) as *mut *mut crate::src::common::frame::x264_frame_t;
        if (*h).frames.unused[1usize].is_null() {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        crate::stdlib::memset(
            (*h).frames.unused[1usize] as *mut ::core::ffi::c_void,
            0i32,
            (((*h).i_thread_frames + 16i32 + 4i32) as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<
                    *mut crate::src::common::frame::x264_frame_t,
                >()),
        );
        (*h).frames.current = crate::src::common::base::x264_malloc(
            (((*h).param.i_sync_lookahead + (*h).param.i_bframe + (*h).i_thread_frames + 3i32)
                as usize)
                .wrapping_mul(::core::mem::size_of::<
                    *mut crate::src::common::frame::x264_frame_t,
                >()) as crate::stdlib::int64_t,
        ) as *mut *mut crate::src::common::frame::x264_frame_t;
        if (*h).frames.current.is_null() {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        crate::stdlib::memset(
            (*h).frames.current as *mut ::core::ffi::c_void,
            0i32,
            (((*h).param.i_sync_lookahead + (*h).param.i_bframe + (*h).i_thread_frames + 3i32)
                as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<
                    *mut crate::src::common::frame::x264_frame_t,
                >()),
        );
        if (*h).param.analyse.i_weighted_pred > 0i32 {
            (*h).frames.blank_unused = crate::src::common::base::x264_malloc(
                (((*h).i_thread_frames * 4i32) as usize).wrapping_mul(::core::mem::size_of::<
                    *mut crate::src::common::frame::x264_frame_t,
                >()) as crate::stdlib::int64_t,
            )
                as *mut *mut crate::src::common::frame::x264_frame_t;
            if (*h).frames.blank_unused.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
            crate::stdlib::memset(
                (*h).frames.blank_unused as *mut ::core::ffi::c_void,
                0i32,
                (((*h).i_thread_frames * 4i32) as crate::__stddef_size_t_h::size_t).wrapping_mul(
                    ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>(),
                ),
            );
        }
        let mut buf = [0; 1000];
        let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*h).i_ref[1usize] = 0i32;
        (*h).i_ref[0usize] = (*h).i_ref[1usize];
        (*h).i_disp_fields = 0i64;
        (*h).i_coded_fields = (*h).i_disp_fields;
        (*h).i_cpb_delay = (*h).i_coded_fields;
        (*h).i_prev_duration = ((*h).param.i_fps_den as crate::stdlib::uint64_t)
            .wrapping_mul((*h).sps.vui.i_time_scale as crate::stdlib::uint64_t)
            .wrapping_div(
                ((*h).param.i_fps_num as crate::stdlib::uint64_t)
                    .wrapping_mul((*h).sps.vui.i_num_units_in_tick as crate::stdlib::uint64_t),
            ) as crate::stdlib::int64_t;
        (*h).i_disp_fields_last_frame = -(1i32);
        crate::src::encoder::analyse::rdo_c::x264_8_rdo_init();
        crate::src::common::predict::x264_8_predict_16x16_init(
            (*h).param.cpu,
            &raw mut (*h).predict_16x16 as *mut crate::src::common::predict::x264_predict_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_init(
            (*h).param.cpu,
            &raw mut (*h).predict_8x8c as *mut crate::src::common::predict::x264_predict_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_init(
            (*h).param.cpu,
            &raw mut (*h).predict_8x16c as *mut crate::src::common::predict::x264_predict_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_init(
            (*h).param.cpu,
            &raw mut (*h).predict_8x8 as *mut crate::src::common::predict::x264_predict8x8_t,
            &raw mut (*h).predict_8x8_filter,
        );
        crate::src::common::predict::x264_8_predict_4x4_init(
            (*h).param.cpu,
            &raw mut (*h).predict_4x4 as *mut crate::src::common::predict::x264_predict_t,
        );
        crate::src::common::pixel::x264_8_pixel_init((*h).param.cpu, &raw mut (*h).pixf);
        crate::src::common::dct::x264_8_dct_init((*h).param.cpu, &raw mut (*h).dctf);
        crate::src::common::dct::x264_8_zigzag_init(
            (*h).param.cpu,
            &raw mut (*h).zigzagf_progressive,
            &raw mut (*h).zigzagf_interlaced,
        );
        crate::stdlib::memcpy(
            &raw mut (*h).zigzagf as *mut ::core::ffi::c_void,
            (if (*h).param.interlaced {
                &raw mut (*h).zigzagf_interlaced
            } else {
                &raw mut (*h).zigzagf_progressive
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<crate::src::common::dct::x264_zigzag_function_t>(),
        );
        crate::src::common::mc::x264_8_mc_init(
            (*h).param.cpu,
            &raw mut (*h).mc as *mut crate::src::common::mc::x264_mc_functions_t_6,
            (*h).param.cpu_independent as ::core::ffi::c_int,
        );
        crate::src::common::quant::x264_8_quant_init(h, (*h).param.cpu, &raw mut (*h).quantf);
        crate::src::common::deblock::x264_8_deblock_init(
            (*h).param.cpu,
            &raw mut (*h).loopf,
            (*h).param.interlaced as ::core::ffi::c_int,
        );
        crate::src::common::bitstream::x264_8_bitstream_init((*h).param.cpu, &raw mut (*h).bsf);
        if (*h).param.cabac {
            crate::src::common::cabac::x264_8_cabac_init(h);
        } else {
            crate::src::common::vlc::x264_8_cavlc_init(h);
        }
        mbcmp_init(h);
        chroma_dsp_init(h);
        p = (&raw mut buf as *mut ::core::ffi::c_char).offset(crate::stdlib::sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"using cpu capabilities:\0".as_ptr() as *const ::core::ffi::c_char,
        ) as isize);
        for (i, (name, flags)) in X264_CPU_NAMES.iter().enumerate() {
            if !(libc::strcmp(name.as_ptr(), c"SSE".as_ptr()) == 0
                && (*h).param.cpu & X264_CPU_SSE2 != 0)
            {
                continue;
            }
            if !(libc::strcmp(name.as_ptr(), c"SSE2".as_ptr()) == 0
                && (*h).param.cpu & (X264_CPU_SSE2_IS_FAST | X264_CPU_SSE2_IS_SLOW) != 0)
            {
                continue;
            }
            if !(libc::strcmp(name.as_ptr(), c"SSE3".as_ptr()) == 0
                && ((*h).param.cpu & X264_CPU_SSSE3 != 0
                    || (*h).param.cpu & X264_CPU_CACHELINE_64 == 0))
            {
                continue;
            }
            if !(libc::strcmp(name.as_ptr(), c"SSE4.1".as_ptr()) == 0
                && (*h).param.cpu & X264_CPU_SSE42 != 0)
            {
                continue;
            }
            if !(libc::strcmp(name.as_ptr(), c"LZCNT".as_ptr()) == 0
                && (*h).param.cpu & X264_CPU_BMI1 != 0)
            {
                continue;
            }
            if !(libc::strcmp(name.as_ptr(), c"BMI1".as_ptr()) == 0
                && (*h).param.cpu & X264_CPU_BMI2 != 0)
            {
                continue;
            }
            if !(libc::strcmp(name.as_ptr(), c"FMA4".as_ptr()) == 0
                && (*h).param.cpu & X264_CPU_FMA3 != 0)
            {
                continue;
            }
            if (*h).param.cpu & *flags == *flags && (i == 0 || *flags != X264_CPU_NAMES[i - 1].1) {
                p = p.offset(crate::stdlib::sprintf(p, c" %s".as_ptr(), name.as_ptr()) as isize);
            }
        }
        if (*h).param.cpu == 0 {
            p = p.offset(crate::stdlib::sprintf(
                p,
                b" none!\0".as_ptr() as *const ::core::ffi::c_char,
            ) as isize);
        }
        log::info!(
            "{}",
            std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                .to_string_lossy()
        );
        if crate::src::encoder::analyse::x264_8_analyse_init_costs(h) != 0 {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        let mut temp = 0;
        temp = 392i32;
        if (temp as ::core::ffi::c_uint).leading_zeros() as i32 != 23i32 {
            log::error!("CLZ test failed: x264 has been miscompiled!");
            log::error!("Are you attempting to run an SSE4a/LZCNT-targeted build on a CPU that");
            log::error!("doesn't support it?");
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        (*h).out.i_nal = 0i32;
        (*h).out.i_bitstream = x264_clip3f(
            ((*h).param.i_width * (*h).param.i_height * 4i32) as ::core::ffi::c_double
                * (if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR {
                    crate::stdlib::pow(0.95, (*h).param.rc.i_qp_min as ::core::ffi::c_double)
                } else {
                    crate::stdlib::pow(0.95, (*h).param.rc.i_qp_constant as ::core::ffi::c_double)
                        * (if 1f32 > (*h).param.rc.f_ip_factor {
                            1f32
                        } else {
                            (*h).param.rc.f_ip_factor
                        }) as ::core::ffi::c_double
                }),
            1000000f64,
            (crate::limits_h::INT_MAX / 3i32) as ::core::ffi::c_double,
        ) as ::core::ffi::c_int;
        (*h).nal_buffer_size = (*h).out.i_bitstream * 3i32 / 2i32 + 4i32 + 64i32;
        (*h).nal_buffer =
            crate::src::common::base::x264_malloc((*h).nal_buffer_size as crate::stdlib::int64_t)
                as *mut crate::stdlib::uint8_t;
        if (*h).nal_buffer.is_null() {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        (*h).reconfig_h = crate::src::common::base::x264_malloc(
            ::core::mem::size_of::<x264_t>() as crate::stdlib::int64_t
        ) as *mut x264_t;
        if (*h).reconfig_h.is_null() {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        if (*h).param.i_threads > 1i32
            && crate::src::common::threadpool::x264_8_threadpool_init(
                &raw mut (*h).threadpool,
                (*h).param.i_threads,
            ) != 0
        {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        if (*h).param.i_lookahead_threads > 1i32
            && crate::src::common::threadpool::x264_8_threadpool_init(
                &raw mut (*h).lookaheadpool,
                (*h).param.i_lookahead_threads,
            ) != 0
        {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        (*h).thread[0usize] = h;
        loop {
            let mut i_0 = 1i32;
            if !(i_0
                < (*h).param.i_threads + ((*h).param.i_sync_lookahead != 0) as ::core::ffi::c_int)
            {
                break;
            }
            (*h).thread[i_0 as usize] = crate::src::common::base::x264_malloc(
                ::core::mem::size_of::<x264_t>() as crate::stdlib::int64_t,
            ) as *mut x264_t;
            if (*h).thread[i_0 as usize].is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
            i_0 += 1;
        }
        if (*h).param.i_lookahead_threads > 1i32 {
            loop {
                let mut i_1 = 0i32;
                if !(i_1 < (*h).param.i_lookahead_threads) {
                    break;
                }
                (*h).lookahead_thread[i_1 as usize] = crate::src::common::base::x264_malloc(
                    ::core::mem::size_of::<x264_t>() as crate::stdlib::int64_t,
                ) as *mut x264_t;
                if (*h).lookahead_thread[i_1 as usize].is_null() {
                    x264_free(h as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<x264_t>();
                }
                *(*h).lookahead_thread[i_1 as usize] = *h;
                i_1 += 1;
            }
        }
        *(*h).reconfig_h = *h;
        loop {
            let mut i_2 = 0i32;
            if !(i_2 < (*h).param.i_threads) {
                break;
            }
            let mut init_nal_count = (*h).param.i_slice_count + 3i32;
            let mut allocate_threadlocal_data =
                (!(*h).param.sliced_threads || i_2 == 0) as ::core::ffi::c_int;
            if i_2 > 0i32 {
                *(*h).thread[i_2 as usize] = *h;
            }
            if crate::stdlib::pthread_mutex_init(
                &raw mut (**(&raw mut (*h).thread as *mut *mut x264_t).offset(i_2 as isize)).mutex,
                ::core::ptr::null::<crate::stdlib::pthread_mutexattr_t>(),
            ) != 0
            {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
            if crate::stdlib::pthread_cond_init(
                &raw mut (**(&raw mut (*h).thread as *mut *mut x264_t).offset(i_2 as isize)).cv,
                ::core::ptr::null::<crate::stdlib::pthread_condattr_t>(),
            ) != 0
            {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
            if allocate_threadlocal_data != 0 {
                (*(*h).thread[i_2 as usize]).fdec =
                    crate::src::common::frame::x264_8_frame_pop_unused(h, 1i32);
                if (*(*h).thread[i_2 as usize]).fdec.is_null() {
                    x264_free(h as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<x264_t>();
                }
            } else {
                (*(*h).thread[i_2 as usize]).fdec = (*(*h).thread[0usize]).fdec;
            }
            (*(*h).thread[i_2 as usize]).out.p_bitstream = crate::src::common::base::x264_malloc(
                (*h).out.i_bitstream as crate::stdlib::int64_t,
            )
                as *mut crate::stdlib::uint8_t;
            if (*(*h).thread[i_2 as usize]).out.p_bitstream.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
            (*(*h).thread[i_2 as usize]).out.nal = crate::src::common::base::x264_malloc(
                (init_nal_count as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::x264_h::x264_nal_t>())
                    as crate::stdlib::int64_t,
            ) as *mut crate::x264_h::x264_nal_t;
            if (*(*h).thread[i_2 as usize]).out.nal.is_null() {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
            (*(*h).thread[i_2 as usize]).out.i_nals_allocated = init_nal_count;
            if allocate_threadlocal_data != 0
                && crate::src::common::macroblock::x264_8_macroblock_cache_allocate(
                    (*h).thread[i_2 as usize],
                ) < 0i32
            {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
            i_2 += 1;
        }
        if crate::src::encoder::lookahead::x264_8_lookahead_init(h, i_slicetype_length) != 0 {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        loop {
            let mut i_3 = 0i32;
            if !(i_3 < (*h).param.i_threads) {
                break;
            }
            if crate::src::common::macroblock::x264_8_macroblock_thread_allocate(
                (*h).thread[i_3 as usize],
                0i32,
            ) < 0i32
            {
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            }
            i_3 += 1;
        }
        if crate::src::encoder::ratecontrol::x264_8_ratecontrol_new(h) < 0i32 {
            x264_free(h as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<x264_t>();
        }
        if (*h).param.i_nal_hrd != 0 {
            log::debug!(
                "HRD bitrate: {} bits/sec",
                (*h).sps.vui.hrd.i_bit_rate_unscaled
            );
            log::debug!("CPB size: {} bits", (*h).sps.vui.hrd.i_cpb_size_unscaled);
        }
        if !(*h).param.psz_dump_yuv.is_null() {
            let mut f = crate::stdlib::fopen(
                (*h).param.psz_dump_yuv,
                b"w\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if f.is_null() {
                log::error!(
                    "dump_yuv: can't write to {}",
                    std::ffi::CStr::from_ptr((*h).param.psz_dump_yuv).to_string_lossy()
                );
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            } else if x264_is_regular_file(f) == 0 {
                log::error!(
                    "dump_yuv: incompatible with non-regular file {}",
                    std::ffi::CStr::from_ptr((*h).param.psz_dump_yuv).to_string_lossy()
                );
                crate::stdlib::fclose(f);
                x264_free(h as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<x264_t>();
            } else {
                crate::stdlib::fclose(f);
            }
        }
        let mut profile = ::core::ptr::null::<::core::ffi::c_char>();
        let mut level = [0; 16];
        profile = if (*h).sps.i_profile_idc
            == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int
        {
            b"Constrained Baseline\0".as_ptr() as *const ::core::ffi::c_char
        } else if (*h).sps.i_profile_idc
            == crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int
        {
            b"Main\0".as_ptr() as *const ::core::ffi::c_char
        } else if (*h).sps.i_profile_idc
            == crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
        {
            b"High\0".as_ptr() as *const ::core::ffi::c_char
        } else if (*h).sps.i_profile_idc
            == crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int
        {
            if (*h).sps.constraint_set3 {
                b"High 10 Intra\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"High 10\0".as_ptr() as *const ::core::ffi::c_char
            }
        } else if (*h).sps.i_profile_idc
            == crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int
        {
            if (*h).sps.constraint_set3 {
                b"High 4:2:2 Intra\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"High 4:2:2\0".as_ptr() as *const ::core::ffi::c_char
            }
        } else if (*h).sps.constraint_set3 {
            b"High 4:4:4 Intra\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"High 4:4:4 Predictive\0".as_ptr() as *const ::core::ffi::c_char
        };
        level = [0; 16];
        if (*h).sps.i_level_idc == 9i32
            || (*h).sps.i_level_idc == 11i32
                && (*h).sps.constraint_set3
                && ((*h).sps.i_profile_idc
                    == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int
                    || (*h).sps.i_profile_idc
                        == crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int)
        {
            crate::stdlib::strcpy(
                &raw mut level as *mut ::core::ffi::c_char,
                b"1b\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            crate::stdlib::snprintf(
                &raw mut level as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                b"%d.%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*h).sps.i_level_idc / 10i32,
                (*h).sps.i_level_idc % 10i32,
            );
        }
        log::info!(
            "profile {}, level {}, {}, {}-bit",
            std::ffi::CStr::from_ptr(profile).to_string_lossy(),
            std::ffi::CStr::from_ptr(&raw const level as *const ::core::ffi::c_char)
                .to_string_lossy(),
            subsampling[(*h).sps.i_chroma_format_idc as usize],
            crate::internal::BIT_DEPTH,
        );
        return h;
    }
}
unsafe extern "C" fn encoder_try_reconfig(
    mut h: *mut x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
    mut rc_reconfig: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        *rc_reconfig = 0i32;
        set_aspect_ratio(h, param, 0i32);
        (*h).param.i_frame_reference = (*param).i_frame_reference;
        (*h).param.i_bframe_bias = (*param).i_bframe_bias;
        if (*h).param.i_scenecut_threshold != 0 {
            (*h).param.i_scenecut_threshold = (*param).i_scenecut_threshold;
        }
        (*h).param.deblocking_filter = (*param).deblocking_filter;
        (*h).param.i_deblocking_filter_alphac0 = (*param).i_deblocking_filter_alphac0;
        (*h).param.i_deblocking_filter_beta = (*param).i_deblocking_filter_beta;
        (*h).param.i_frame_packing = (*param).i_frame_packing;
        (*h).param.mastering_display = (*param).mastering_display;
        (*h).param.content_light_level = (*param).content_light_level;
        (*h).param.i_alternative_transfer = (*param).i_alternative_transfer;
        (*h).param.analyse.inter = (*param).analyse.inter;
        (*h).param.analyse.intra = (*param).analyse.intra;
        (*h).param.analyse.i_direct_mv_pred = (*param).analyse.i_direct_mv_pred;
        if (*h).param.analyse.i_me_method < crate::x264_h::X264_ME_ESA
            || (*param).analyse.i_me_range < (*h).param.analyse.i_me_range
        {
            (*h).param.analyse.i_me_range = (*param).analyse.i_me_range;
        }
        (*h).param.analyse.i_noise_reduction = (*param).analyse.i_noise_reduction;
        if (*h).param.analyse.i_subpel_refine != 0 {
            (*h).param.analyse.i_subpel_refine = (*param).analyse.i_subpel_refine;
        }
        (*h).param.analyse.i_trellis = (*param).analyse.i_trellis;
        (*h).param.analyse.chroma_me = (*param).analyse.chroma_me;
        (*h).param.analyse.dct_decimate = (*param).analyse.dct_decimate;
        (*h).param.analyse.fast_pskip = (*param).analyse.fast_pskip;
        (*h).param.analyse.mixed_references = (*param).analyse.mixed_references;
        (*h).param.analyse.f_psy_rd = (*param).analyse.f_psy_rd;
        (*h).param.analyse.f_psy_trellis = (*param).analyse.f_psy_trellis;
        (*h).param.crop_rect = (*param).crop_rect;
        if (*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA
            || (*param).analyse.i_me_method < crate::x264_h::X264_ME_ESA
        {
            (*h).param.analyse.i_me_method = (*param).analyse.i_me_method;
        }
        if (*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA
            && !(*h).frames.have_sub8x8_esa
        {
            (*h).param.analyse.inter &= !crate::x264_h::X264_ANALYSE_PSUB8x8;
        }
        if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).transform_8x8_mode {
            (*h).param.analyse.transform_8x8 = (*param).analyse.transform_8x8;
        }
        if (*h).frames.i_max_ref1 > 1i32 {
            (*h).param.i_bframe_pyramid = (*param).i_bframe_pyramid;
        }
        (*h).param.i_slice_max_size = (*param).i_slice_max_size;
        (*h).param.i_slice_max_mbs = (*param).i_slice_max_mbs;
        (*h).param.i_slice_min_mbs = (*param).i_slice_min_mbs;
        (*h).param.i_slice_count = (*param).i_slice_count;
        (*h).param.i_slice_count_max = (*param).i_slice_count_max;
        (*h).param.tff = (*param).tff;
        if (*h).param.rc.i_vbv_max_bitrate > 0i32
            && (*h).param.rc.i_vbv_buffer_size > 0i32
            && (*param).rc.i_vbv_max_bitrate > 0i32
            && (*param).rc.i_vbv_buffer_size > 0i32
        {
            *rc_reconfig |= ((*h).param.rc.i_vbv_max_bitrate != (*param).rc.i_vbv_max_bitrate)
                as ::core::ffi::c_int;
            *rc_reconfig |= ((*h).param.rc.i_vbv_buffer_size != (*param).rc.i_vbv_buffer_size)
                as ::core::ffi::c_int;
            *rc_reconfig |=
                ((*h).param.rc.i_bitrate != (*param).rc.i_bitrate) as ::core::ffi::c_int;
            (*h).param.rc.i_vbv_max_bitrate = (*param).rc.i_vbv_max_bitrate;
            (*h).param.rc.i_vbv_buffer_size = (*param).rc.i_vbv_buffer_size;
            (*h).param.rc.i_bitrate = (*param).rc.i_bitrate;
        }
        *rc_reconfig |=
            ((*h).param.rc.f_rf_constant != (*param).rc.f_rf_constant) as ::core::ffi::c_int;
        *rc_reconfig |= ((*h).param.rc.f_rf_constant_max != (*param).rc.f_rf_constant_max)
            as ::core::ffi::c_int;
        (*h).param.rc.f_rf_constant = (*param).rc.f_rf_constant;
        (*h).param.rc.f_rf_constant_max = (*param).rc.f_rf_constant_max;
        return validate_parameters(h, 0i32);
    }
}
pub unsafe extern "C" fn x264_8_encoder_reconfig_apply(
    mut h: *mut x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc_reconfig = 0;
        let mut ret = encoder_try_reconfig(h, param, &raw mut rc_reconfig);
        mbcmp_init(h);
        if ret == 0 {
            crate::src::encoder::set::x264_8_sps_init_reconfigurable(
                &mut (*h).sps,
                &raw mut (*h).param,
            );
        }
        if ret == 0 && rc_reconfig != 0 {
            crate::src::encoder::ratecontrol::x264_8_ratecontrol_init_reconfigurable(h, 0i32);
        }
        return ret;
    }
}
pub unsafe extern "C" fn x264_8_encoder_reconfig(
    mut h: *mut x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc_reconfig = 0;
        h = (*h).thread[(*(*h).thread[0usize]).i_thread_phase as usize];
        let mut param_save = (*(*h).reconfig_h).param;
        (*(*h).reconfig_h).param = (*h).param;
        let mut ret = encoder_try_reconfig((*h).reconfig_h, param, &raw mut rc_reconfig);
        if ret == 0 {
            (*h).reconfig = 1i32;
        } else {
            (*(*h).reconfig_h).param = param_save;
        }
        return ret;
    }
}
pub unsafe extern "C" fn x264_8_encoder_parameters(
    mut h: *mut x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        crate::stdlib::memcpy(
            param as *mut ::core::ffi::c_void,
            &raw mut (**(&raw mut (*h).thread as *mut *mut x264_t)
                .offset((*h).i_thread_phase as isize))
            .param as *const ::core::ffi::c_void,
            ::core::mem::size_of::<crate::x264_h::x264_param_t>(),
        );
        (*param).opaque = crate::__stddef_null_h::NULL;
    }
}
unsafe extern "C" fn nal_start(
    mut h: *mut x264_t,
    mut i_type: ::core::ffi::c_int,
    mut i_ref_idc: ::core::ffi::c_int,
) {
    unsafe {
        let mut nal = (*h).out.nal.offset((*h).out.i_nal as isize);
        (*nal).i_ref_idc = i_ref_idc;
        (*nal).i_type = i_type;
        (*nal).long_startcode = true;
        (*nal).i_payload = 0i32;
        (*nal).p_payload = (*h).out.p_bitstream.offset(
            ((bs_pos
                as unsafe extern "C" fn(
                    *mut crate::src::common::bitstream::bs_t,
                ) -> ::core::ffi::c_int)(&raw mut (*h).out.bs)
                / 8i32) as isize,
        );
        (*nal).i_padding = 0i32;
    }
}
unsafe extern "C" fn nal_check_buffer(mut h: *mut x264_t) -> ::core::ffi::c_int {
    unsafe {
        if (*h).out.i_nal >= (*h).out.i_nals_allocated {
            let mut new_out = crate::src::common::base::x264_malloc(
                (::core::mem::size_of::<crate::x264_h::x264_nal_t>())
                    .wrapping_mul(((*h).out.i_nals_allocated * 2i32) as usize)
                    as crate::stdlib::int64_t,
            ) as *mut crate::x264_h::x264_nal_t;
            if new_out.is_null() {
                return -(1i32);
            }
            crate::stdlib::memcpy(
                new_out as *mut ::core::ffi::c_void,
                (*h).out.nal as *const ::core::ffi::c_void,
                (::core::mem::size_of::<crate::x264_h::x264_nal_t>())
                    .wrapping_mul((*h).out.i_nals_allocated as crate::__stddef_size_t_h::size_t),
            );
            x264_free((*h).out.nal as *mut ::core::ffi::c_void);
            (*h).out.nal = new_out;
            (*h).out.i_nals_allocated *= 2i32;
        }
        return 0i32;
    }
}
unsafe extern "C" fn nal_end(mut h: *mut x264_t) -> ::core::ffi::c_int {
    unsafe {
        let mut nal = (*h).out.nal.offset((*h).out.i_nal as isize);
        let mut end = (*h).out.p_bitstream.offset(
            ((bs_pos
                as unsafe extern "C" fn(
                    *mut crate::src::common::bitstream::bs_t,
                ) -> ::core::ffi::c_int)(&raw mut (*h).out.bs)
                / 8i32) as isize,
        );
        (*nal).i_payload = end.offset_from((*nal).p_payload) as ::core::ffi::c_int;
        crate::stdlib::memset(end as *mut ::core::ffi::c_void, 0xffi32, 64usize);
        if (*h).param.nalu_process.is_some() {
            (*h).param.nalu_process.expect("non-null function pointer")(
                (*h).api as *mut x264_t,
                nal,
                (*(*h).fenc).opaque,
            );
        }
        (*h).out.i_nal += 1;
        return nal_check_buffer(h);
    }
}
unsafe extern "C" fn check_encapsulated_buffer(
    mut h: *mut x264_t,
    mut h0: *mut x264_t,
    mut start: ::core::ffi::c_int,
    mut previous_nal_size: crate::stdlib::int64_t,
    mut necessary_size: crate::stdlib::int64_t,
) -> ::core::ffi::c_int {
    unsafe {
        if ((*h0).nal_buffer_size as crate::stdlib::int64_t) < necessary_size {
            let mut i = 0i32;
            necessary_size *= 2i64;
            if necessary_size > crate::limits_h::INT_MAX as crate::stdlib::int64_t {
                return -(1i32);
            }
            let mut buf = crate::src::common::base::x264_malloc(necessary_size)
                as *mut crate::stdlib::uint8_t;
            if buf.is_null() {
                return -(1i32);
            }
            if previous_nal_size != 0 {
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (*h0).nal_buffer as *const ::core::ffi::c_void,
                    previous_nal_size as crate::__stddef_size_t_h::size_t,
                );
            }
            let mut delta = buf.offset_from((*h0).nal_buffer);
            while i < start {
                let ref mut c2rust_fresh1 = (*(*h).out.nal.offset(i as isize)).p_payload;
                *c2rust_fresh1 = (*c2rust_fresh1).offset(delta);
                i += 1;
            }
            x264_free((*h0).nal_buffer as *mut ::core::ffi::c_void);
            (*h0).nal_buffer = buf;
            (*h0).nal_buffer_size = necessary_size as ::core::ffi::c_int;
        }
        return 0i32;
    }
}
unsafe extern "C" fn encoder_encapsulate_nals(
    mut h: *mut x264_t,
    mut start: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nal_size = 0i64;
        let mut previous_nal_size = 0i64;
        let mut i_0 = 0i32;
        let mut h0 = (*h).thread[0usize];
        if (*h).param.nalu_process.is_some() {
            let mut i = start;
            while i < (*h).out.i_nal {
                nal_size += (*(*h).out.nal.offset(i as isize)).i_payload as crate::stdlib::int64_t;
                i += 1;
            }
            if nal_size > crate::limits_h::INT_MAX as crate::stdlib::int64_t {
                return -(1i32);
            }
            return nal_size as ::core::ffi::c_int;
        }
        while i_0 < start {
            previous_nal_size +=
                (*(*h).out.nal.offset(i_0 as isize)).i_payload as crate::stdlib::int64_t;
            i_0 += 1;
        }
        let mut i_1 = start;
        while i_1 < (*h).out.i_nal {
            nal_size += (*(*h).out.nal.offset(i_1 as isize)).i_payload as crate::stdlib::int64_t;
            i_1 += 1;
        }
        let mut necessary_size = previous_nal_size
            + nal_size * 3i64 / 2i64
            + ((*h).out.i_nal * 4i32) as crate::stdlib::int64_t
            + 4i64
            + 64i64;
        let mut i_2 = start;
        while i_2 < (*h).out.i_nal {
            necessary_size +=
                (*(*h).out.nal.offset(i_2 as isize)).i_padding as crate::stdlib::int64_t;
            i_2 += 1;
        }
        if check_encapsulated_buffer(h, h0, start, previous_nal_size, necessary_size) != 0 {
            return -(1i32);
        }
        let mut nal_buffer = (*h0).nal_buffer.offset(previous_nal_size as isize);
        let mut i_3 = start;
        while i_3 < (*h).out.i_nal {
            (*(*h).out.nal.offset(i_3 as isize)).long_startcode = i_3 == 0
                || (*(*h).out.nal.offset(i_3 as isize)).i_type
                    == crate::x264_h::NAL_SPS as ::core::ffi::c_int
                || (*(*h).out.nal.offset(i_3 as isize)).i_type
                    == crate::x264_h::NAL_PPS as ::core::ffi::c_int
                || (*h).param.i_avcintra_class != 0;
            crate::src::common::bitstream::x264_8_nal_encode(
                h,
                nal_buffer,
                (*h).out.nal.offset(i_3 as isize),
            );
            nal_buffer = nal_buffer.offset((*(*h).out.nal.offset(i_3 as isize)).i_payload as isize);
            i_3 += 1;
        }
        return nal_buffer.offset_from((*h0).nal_buffer.offset(previous_nal_size as isize))
            as ::core::ffi::c_int;
    }
}
pub unsafe extern "C" fn x264_8_encoder_headers(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        (*h).out.i_nal = 0i32;
        bs_init(
            &raw mut (*h).out.bs,
            (*h).out.p_bitstream as *mut ::core::ffi::c_void,
            (*h).out.i_bitstream,
        );
        nal_start(
            h,
            crate::x264_h::NAL_SPS as ::core::ffi::c_int,
            crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
        );
        crate::src::encoder::set::x264_8_sps_write(&raw mut (*h).out.bs, &(*h).sps);
        if nal_end(h) != 0 {
            return -(1i32);
        }
        nal_start(
            h,
            crate::x264_h::NAL_PPS as ::core::ffi::c_int,
            crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
        );
        crate::src::encoder::set::x264_8_pps_write(
            &raw mut (*h).out.bs,
            &(*h).sps,
            &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t,
        );
        if nal_end(h) != 0 {
            return -(1i32);
        }
        nal_start(
            h,
            crate::x264_h::NAL_SEI as ::core::ffi::c_int,
            crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        if crate::src::encoder::set::x264_8_sei_version_write(h, &raw mut (*h).out.bs) != 0 {
            return -(1i32);
        }
        if nal_end(h) != 0 {
            return -(1i32);
        }
        let mut frame_size = encoder_encapsulate_nals(h, 0i32);
        if frame_size < 0i32 {
            return -(1i32);
        }
        *pi_nal = (*h).out.i_nal;
        *pp_nal = (*h).out.nal.offset(0isize);
        (*h).out.i_nal = 0i32;
        return frame_size;
    }
}
#[inline]
unsafe extern "C" fn reference_check_reorder(mut h: *mut x264_t) {
    unsafe {
        let mut i = 0i32;
        let mut list = 0i32;
        while !(*h).frames.reference[i as usize].is_null() {
            if (*(*h).frames.reference[i as usize]).corrupt {
                (*h).ref_reorder[0usize] = true;
                return;
            }
            i += 1;
        }
        while list
            <= ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                as ::core::ffi::c_int
        {
            let mut i_0 = 0i32;
            while i_0 < (*h).i_ref[list as usize] - 1i32 {
                let mut framenum_diff = (*(*h).fref[list as usize][(i_0 + 1i32) as usize])
                    .i_frame_num
                    - (*(*h).fref[list as usize][i_0 as usize]).i_frame_num;
                let mut poc_diff = (*(*h).fref[list as usize][(i_0 + 1i32) as usize]).i_poc
                    - (*(*h).fref[list as usize][i_0 as usize]).i_poc;
                if if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                {
                    (framenum_diff > 0i32) as ::core::ffi::c_int
                } else if list == 1i32 {
                    (poc_diff < 0i32) as ::core::ffi::c_int
                } else {
                    (poc_diff > 0i32) as ::core::ffi::c_int
                } != 0
                {
                    (*h).ref_reorder[list as usize] = true;
                    return;
                }
                i_0 += 1;
            }
            list += 1;
        }
    }
}
unsafe extern "C" fn weighted_reference_duplicate(
    mut h: *mut x264_t,
    mut i_ref: ::core::ffi::c_int,
    mut w: *const crate::src::common::mc::x264_weight_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut j = 1i32;
        let mut i = (*h).i_ref[0usize];
        if i <= 1i32 {
            return -(1i32);
        }
        if (*h).param.analyse.i_weighted_pred != crate::x264_h::X264_WEIGHTP_SMART {
            return -(1i32);
        }
        if crate::internal::BIT_DEPTH > 8i32
            && w != &raw mut crate::src::common::tables::x264_zero
                as *const crate::src::common::mc::x264_weight_t
        {
            return -(1i32);
        }
        let mut newframe = crate::src::common::frame::x264_8_frame_pop_blank_unused(h);
        if newframe.is_null() {
            return -(1i32);
        }
        *newframe = *(*h).fref[0usize][i_ref as usize];
        (*newframe).i_reference_count = 1i32;
        (*newframe).orig = (*h).fref[0usize][i_ref as usize];
        (*newframe).duplicate = true;
        crate::stdlib::memcpy(
            &raw mut *(&raw mut (*(*h).fenc).weight
                as *mut [crate::src::common::mc::x264_weight_t; 3])
                .offset(j as isize) as *mut ::core::ffi::c_void,
            w as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::mc::x264_weight_t; 3]>(),
        );
        (*h).ref_reorder[0usize] = true;
        if (*h).i_ref[0usize] < crate::src::common::base::X264_REF_MAX {
            (*h).i_ref[0usize] += 1;
        }
        (*h).fref[0usize][(crate::src::common::base::X264_REF_MAX - 1i32) as usize] =
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        crate::src::common::frame::x264_8_frame_unshift(
            (&raw mut *(&raw mut (*h).fref
                as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                .offset(0isize) as *mut *mut crate::src::common::frame::x264_frame_t)
                .offset(j as isize),
            newframe,
        );
        return j;
    }
}
unsafe extern "C" fn weighted_pred_init(mut h: *mut x264_t) {
    unsafe {
        let mut i_ref = 0i32;
        let mut i_ref_0 = 0i32;
        let mut weightplane = [0i32, 0i32];
        let mut i_0 = 0i32;
        while i_ref < (*h).i_ref[0usize] {
            (*(*h).fenc).weighted[i_ref as usize] =
                (*(*h).fref[0usize][i_ref as usize]).filtered[0usize][0usize];
            i_ref += 1;
        }
        (*(*h).fenc).i_lines_weighted = 0i32;
        while i_ref_0 < (*h).i_ref[0usize] << (*h).sh.mbaff as ::core::ffi::c_int {
            let mut i = 0i32;
            while i < 3i32 {
                (*h).sh.weight[i_ref_0 as usize][i as usize].weightfn =
                    ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
                i += 1;
            }
            i_ref_0 += 1;
        }
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            || (*h).param.analyse.i_weighted_pred <= 0i32
        {
            return;
        }
        let mut i_padv =
            crate::src::common::frame::PADV << (*h).param.interlaced as ::core::ffi::c_int;
        let mut denom = -(1i32);
        while i_0 < 3i32 {
            let mut j = 0i32;
            while j < (*h).i_ref[0usize] {
                if !(*(*h).fenc).weight[j as usize][i_0 as usize]
                    .weightfn
                    .is_null()
                {
                    (*h).sh.weight[j as usize][i_0 as usize] =
                        (*(*h).fenc).weight[j as usize][i_0 as usize];
                    if (*h).sh.weight[j as usize][i_0 as usize].i_scale
                        == (1i32) << (*h).sh.weight[j as usize][i_0 as usize].i_denom
                        && (*h).sh.weight[j as usize][i_0 as usize].i_offset == 0i32
                    {
                        (*h).sh.weight[j as usize][i_0 as usize].weightfn =
                            ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
                    } else {
                        if weightplane[(i_0 != 0) as ::core::ffi::c_int as usize] == 0 {
                            weightplane[(i_0 != 0) as ::core::ffi::c_int as usize] = 1i32;
                            denom = (*h).sh.weight[j as usize][i_0 as usize].i_denom;
                            (*h).sh.weight[0usize][(i_0 != 0) as ::core::ffi::c_int as usize]
                                .i_denom = denom;
                            '_c2rust_label: {
                                if x264_clip3(denom, 0i32, 7i32) == denom {
                                } else {
                                    crate::stdlib::__assert_fail(
                                        b"x264_clip3( denom, 0, 7 ) == denom\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"encoder/encoder.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2240u32,
                                        b"void weighted_pred_init(x264_t *)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                        }
                        '_c2rust_label_0: {
                            if (*h).sh.weight[j as usize][i_0 as usize].i_denom == denom {
                            } else {
                                crate::stdlib::__assert_fail(
                                    b"h->sh.weight[j][i].i_denom == denom\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                                    2243u32,
                                    b"void weighted_pred_init(x264_t *)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        if i_0 == 0 {
                            let mut buffer_next = 0i32;
                            let c2rust_fresh3 = buffer_next;
                            buffer_next = buffer_next + 1;
                            (*(*h).fenc).weighted[j as usize] = (*h).mb.p_weight_buf
                                [c2rust_fresh3 as usize]
                                .offset(((*(*h).fenc).i_stride[0usize] * i_padv) as isize)
                                .offset(
                                    (if 32i32
                                        > 64i32
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    {
                                        32i32
                                    } else {
                                        64i32
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    }) as isize,
                                );
                            if (*h).param.i_threads == 1i32 {
                                let mut src = (*(*h).fref[0usize][j as usize]).filtered[0usize]
                                    [0usize]
                                    .offset(
                                        -(((*(*h).fref[0usize][j as usize]).i_stride[0usize]
                                            * i_padv)
                                            as isize),
                                    )
                                    .offset(
                                        -((if 32i32
                                            > 64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        {
                                            32i32
                                        } else {
                                            64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        }) as isize),
                                    );
                                let mut dst = (*(*h).fenc).weighted[j as usize]
                                    .offset(-(((*(*h).fenc).i_stride[0usize] * i_padv) as isize))
                                    .offset(
                                        -((if 32i32
                                            > 64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        {
                                            32i32
                                        } else {
                                            64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        }) as isize),
                                    );
                                let mut stride = (*(*h).fenc).i_stride[0usize];
                                let mut width = (*(*h).fenc).i_width[0usize]
                                    + ((if 32i32
                                        > 64i32
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    {
                                        32i32
                                    } else {
                                        64i32
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    }) + crate::src::common::frame::PADH);
                                let mut height = (*(*h).fenc).i_lines[0usize] + i_padv * 2i32;
                                crate::src::common::frame::x264_8_weight_scale_plane(
                                    h,
                                    dst,
                                    stride as crate::stdlib::intptr_t,
                                    src,
                                    stride as crate::stdlib::intptr_t,
                                    width,
                                    height,
                                    (&raw mut *(&raw mut (*h).sh.weight
                                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                                        .offset(j as isize)
                                        as *mut crate::src::common::mc::x264_weight_t)
                                        .offset(0isize),
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
        if weightplane[1usize] != 0 {
            let mut i_1 = 0i32;
            while i_1 < (*h).i_ref[0usize] {
                if !(*h).sh.weight[i_1 as usize][1usize].weightfn.is_null()
                    && (*h).sh.weight[i_1 as usize][2usize].weightfn.is_null()
                {
                    (*h).sh.weight[i_1 as usize][2usize].i_scale =
                        (1i32) << (*h).sh.weight[0usize][1usize].i_denom;
                    (*h).sh.weight[i_1 as usize][2usize].i_offset = 0i32;
                } else if !(*h).sh.weight[i_1 as usize][2usize].weightfn.is_null()
                    && (*h).sh.weight[i_1 as usize][1usize].weightfn.is_null()
                {
                    (*h).sh.weight[i_1 as usize][1usize].i_scale =
                        (1i32) << (*h).sh.weight[0usize][1usize].i_denom;
                    (*h).sh.weight[i_1 as usize][1usize].i_offset = 0i32;
                }
                i_1 += 1;
            }
        }
        if weightplane[0usize] == 0 {
            (*h).sh.weight[0usize][0usize].i_denom = 0i32;
        }
        if weightplane[1usize] == 0 {
            (*h).sh.weight[0usize][1usize].i_denom = 0i32;
        }
        (*h).sh.weight[0usize][2usize].i_denom = (*h).sh.weight[0usize][1usize].i_denom;
    }
}
#[inline]
unsafe extern "C" fn reference_distance(
    mut h: *mut x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*h).param.i_frame_packing == 5i32 {
            return crate::stdlib::abs(
                ((*(*h).fenc).i_frame & !(1i32)) - ((*frame).i_frame & !(1i32)),
            ) + ((*(*h).fenc).i_frame & 1i32 != (*frame).i_frame & 1i32)
                as ::core::ffi::c_int;
        } else {
            return crate::stdlib::abs((*(*h).fenc).i_frame - (*frame).i_frame);
        };
    }
}
#[inline]
unsafe extern "C" fn reference_build_list(mut h: *mut x264_t, mut i_poc: ::core::ffi::c_int) {
    unsafe {
        let mut b_ok = 0;
        let mut i = 0i32;
        let mut list = 0i32;
        (*h).i_ref[0usize] = 0i32;
        (*h).mb.pic.i_fref[0usize] = (*h).i_ref[0usize];
        (*h).i_ref[1usize] = 0i32;
        (*h).mb.pic.i_fref[1usize] = (*h).i_ref[1usize];
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            return;
        }
        while !(*h).frames.reference[i as usize].is_null() {
            if !((*(*h).frames.reference[i as usize]).corrupt) {
                if (*(*h).frames.reference[i as usize]).i_poc < i_poc {
                    let c2rust_fresh4 = (*h).i_ref[0usize];
                    (*h).i_ref[0usize] = (*h).i_ref[0usize] + 1;
                    (*h).fref[0usize][c2rust_fresh4 as usize] = (*h).frames.reference[i as usize];
                } else if (*(*h).frames.reference[i as usize]).i_poc > i_poc {
                    let c2rust_fresh5 = (*h).i_ref[1usize];
                    (*h).i_ref[1usize] = (*h).i_ref[1usize] + 1;
                    (*h).fref[1usize][c2rust_fresh5 as usize] = (*h).frames.reference[i as usize];
                }
            }
            i += 1;
        }
        if (*h).sh.i_mmco_remove_from_end != 0 {
            loop {
                let mut i_0 = 0i32;
                b_ok = 1i32;
                while i_0 < (*h).i_ref[0usize] - 1i32 {
                    if (*(*h).fref[0usize][i_0 as usize]).i_frame
                        < (*(*h).fref[0usize][(i_0 + 1i32) as usize]).i_frame
                    {
                        let mut t = (*h).fref[0usize][i_0 as usize];
                        (*h).fref[0usize][i_0 as usize] = (*h).fref[0usize][(i_0 + 1i32) as usize];
                        (*h).fref[0usize][(i_0 + 1i32) as usize] = t;
                        b_ok = 0i32;
                        break;
                    } else {
                        i_0 += 1;
                    }
                }
                if !(b_ok == 0) {
                    break;
                }
            }
            let mut i_1 = (*h).i_ref[0usize] - 1i32;
            while i_1 >= (*h).i_ref[0usize] - (*h).sh.i_mmco_remove_from_end {
                let mut diff = (*h).i_frame_num - (*(*h).fref[0usize][i_1 as usize]).i_frame_num;
                (*h).sh.mmco[(*h).sh.i_mmco_command_count as usize].i_poc =
                    (*(*h).fref[0usize][i_1 as usize]).i_poc;
                let c2rust_fresh6 = (*h).sh.i_mmco_command_count;
                (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_command_count + 1;
                (*h).sh.mmco[c2rust_fresh6 as usize].i_difference_of_pic_nums = diff;
                i_1 -= 1;
            }
        }
        while list < 2i32 {
            (*h).fref_nearest[list as usize] = (*h).fref[list as usize][0usize];
            loop {
                let mut i_2 = 0i32;
                b_ok = 1i32;
                while i_2 < (*h).i_ref[list as usize] - 1i32 {
                    if if list != 0 {
                        ((*(*h).fref[list as usize][(i_2 + 1i32) as usize]).i_poc
                            < (*(*h).fref_nearest[list as usize]).i_poc)
                            as ::core::ffi::c_int
                    } else {
                        ((*(*h).fref[list as usize][(i_2 + 1i32) as usize]).i_poc
                            > (*(*h).fref_nearest[list as usize]).i_poc)
                            as ::core::ffi::c_int
                    } != 0
                    {
                        (*h).fref_nearest[list as usize] =
                            (*h).fref[list as usize][(i_2 + 1i32) as usize];
                    }
                    if reference_distance(h, (*h).fref[list as usize][i_2 as usize])
                        > reference_distance(h, (*h).fref[list as usize][(i_2 + 1i32) as usize])
                    {
                        let mut t_0 = (*h).fref[list as usize][i_2 as usize];
                        (*h).fref[list as usize][i_2 as usize] =
                            (*h).fref[list as usize][(i_2 + 1i32) as usize];
                        (*h).fref[list as usize][(i_2 + 1i32) as usize] = t_0;
                        b_ok = 0i32;
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
        (*h).i_ref[1usize] = if (*h).i_ref[1usize] < (*h).frames.i_max_ref1 {
            (*h).i_ref[1usize]
        } else {
            (*h).frames.i_max_ref1
        };
        (*h).i_ref[0usize] = if (*h).i_ref[0usize] < (*h).frames.i_max_ref0 {
            (*h).i_ref[0usize]
        } else {
            (*h).frames.i_max_ref0
        };
        (*h).i_ref[0usize] = if (*h).i_ref[0usize] < (*h).param.i_frame_reference {
            (*h).i_ref[0usize]
        } else {
            (*h).param.i_frame_reference
        };
        if ((*(*h).fenc).i_type == crate::x264_h::X264_TYPE_B
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_BREF)
            && (*h).param.bluray_compat
        {
            (*h).i_ref[0usize] = if (*h).i_ref[0usize]
                < ((*(*h).fref[0usize][0usize]).i_type == 0x5i32
                    || (*(*h).fref[0usize][0usize]).i_type == 0x4i32)
                    as ::core::ffi::c_int
                    + 1i32
            {
                (*h).i_ref[0usize]
            } else {
                ((*(*h).fref[0usize][0usize]).i_type == 0x5i32
                    || (*(*h).fref[0usize][0usize]).i_type == 0x4i32)
                    as ::core::ffi::c_int
                    + 1i32
            };
        }
        if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_P {
            let mut idx = -(1i32);
            if (*h).param.analyse.i_weighted_pred >= crate::x264_h::X264_WEIGHTP_SIMPLE {
                let mut w = [crate::src::common::mc::x264_weight_t {
                    cachea: [0; 8],
                    cacheb: [0; 8],
                    i_denom: 0,
                    i_scale: 0,
                    i_offset: 0,
                    weightfn: ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>(),
                }; 3];
                w[2usize].weightfn = ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
                w[1usize].weightfn = w[2usize].weightfn;
                if (*h).param.rc.stat_read {
                    crate::src::encoder::ratecontrol::x264_8_ratecontrol_set_weights(h, (*h).fenc);
                }
                if (*(*h).fenc).weight[0usize][0usize].weightfn.is_null() {
                    (*(*h).fenc).weight[0usize][0usize].i_denom = 0i32;
                    w[0usize].i_scale = 1i32;
                    w[0usize].i_denom = 0i32;
                    w[0usize].i_offset = -1i32;
                    (*h).mc.weight_cache.expect("non-null function pointer")(
                        h,
                        (&raw mut w as *mut crate::src::common::mc::x264_weight_t).offset(0isize),
                    );
                    idx = weighted_reference_duplicate(
                        h,
                        0i32,
                        &raw mut w as *mut crate::src::common::mc::x264_weight_t,
                    );
                } else {
                    if (*(*h).fenc).weight[0usize][0usize].i_scale
                        == (1i32) << (*(*h).fenc).weight[0usize][0usize].i_denom
                    {
                        (*(*h).fenc).weight[0usize][0usize].i_scale = 1i32;
                        (*(*h).fenc).weight[0usize][0usize].i_denom = 0i32;
                        (*(*h).fenc).weight[0usize][0usize].i_offset =
                            (*(*h).fenc).weight[0usize][0usize].i_offset;
                        (*h).mc.weight_cache.expect("non-null function pointer")(
                            h,
                            (&raw mut *(&raw mut (*(*h).fenc).weight
                                as *mut [crate::src::common::mc::x264_weight_t; 3])
                                .offset(0isize)
                                as *mut crate::src::common::mc::x264_weight_t)
                                .offset(0isize),
                        );
                    }
                    weighted_reference_duplicate(
                        h,
                        0i32,
                        &raw mut crate::src::common::tables::x264_zero
                            as *const crate::src::common::mc::x264_weight_t,
                    );
                    if (*(*h).fenc).weight[0usize][0usize].i_offset > -(128i32) {
                        w[0usize] = (*(*h).fenc).weight[0usize][0usize];
                        w[0usize].i_offset -= 1;
                        (*h).mc.weight_cache.expect("non-null function pointer")(
                            h,
                            (&raw mut w as *mut crate::src::common::mc::x264_weight_t)
                                .offset(0isize),
                        );
                        idx = weighted_reference_duplicate(
                            h,
                            0i32,
                            &raw mut w as *mut crate::src::common::mc::x264_weight_t,
                        );
                    }
                }
            }
            (*h).mb.ref_blind_dupe = idx;
        }
        '_c2rust_label: {
            if (*h).i_ref[0usize] + (*h).i_ref[1usize] <= 16i32 {
            } else {
                crate::stdlib::__assert_fail(
                    b"h->i_ref[0] + h->i_ref[1] <= X264_REF_MAX\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2408u32,
                    b"void reference_build_list(x264_t *, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*h).mb.pic.i_fref[0usize] = (*h).i_ref[0usize];
        (*h).mb.pic.i_fref[1usize] = (*h).i_ref[1usize];
    }
}
unsafe extern "C" fn fdec_filter_row(
    mut h: *mut x264_t,
    mut mb_y: ::core::ffi::c_int,
    mut pass: ::core::ffi::c_int,
) {
    unsafe {
        let mut b_measure_quality = 1i32;
        let mut b_hpel = (*(*h).fdec).kept_as_ref;
        let mut b_deblock = ((*h).sh.i_disable_deblocking_filter_idc != 1i32) as ::core::ffi::c_int;
        let mut b_end = (mb_y == (*h).i_threadslice_end) as ::core::ffi::c_int;
        let mut min_y = mb_y - ((1i32) << (*h).sh.mbaff as ::core::ffi::c_int);
        let mut b_start = (min_y == (*h).i_threadslice_start) as ::core::ffi::c_int;
        let mut minpix_y = min_y * 16i32 - 4i32 * (b_start == 0) as ::core::ffi::c_int;
        let mut maxpix_y = mb_y * 16i32 - 4i32 * (b_end == 0) as ::core::ffi::c_int;
        b_deblock &= (b_hpel || (*h).param.full_recon || !(*h).param.psz_dump_yuv.is_null())
            as ::core::ffi::c_int;
        if (*h).param.sliced_threads {
            match pass {
                1 => {
                    b_deblock &= (!(*h).param.full_recon) as ::core::ffi::c_int;
                    b_hpel &= !(b_start != 0 && min_y > 0i32);
                    b_measure_quality = 0i32;
                }
                2 => {
                    b_deblock = 0i32;
                    b_measure_quality = 0i32;
                }
                0 | _ => {
                    b_deblock &= (*h).param.full_recon as ::core::ffi::c_int;
                    b_hpel = false;
                }
            }
        }
        if mb_y & (*h).sh.mbaff as ::core::ffi::c_int != 0 {
            return;
        }
        if min_y < (*h).i_threadslice_start {
            return;
        }
        if b_deblock != 0 {
            let mut y = min_y;
            while y < mb_y {
                crate::src::common::deblock::x264_8_frame_deblock_row(h, y);
                y += (1i32) << (*h).sh.mbaff as ::core::ffi::c_int;
            }
        }
        if (*h).param.interlaced && (!(*h).param.sliced_threads || pass == 1i32) {
            let mut p = 0i32;
            while p < (*(*h).fdec).i_plane {
                let mut i = minpix_y
                    >> ((*h).sps.i_chroma_format_idc.is_420() && p != 0) as ::core::ffi::c_int;
                while i < maxpix_y
                    >> ((*h).sps.i_chroma_format_idc.is_420() && p != 0) as ::core::ffi::c_int
                {
                    crate::stdlib::memcpy(
                        (*(*h).fdec).plane_fld[p as usize]
                            .offset((i * (*(*h).fdec).i_stride[p as usize]) as isize)
                            as *mut ::core::ffi::c_void,
                        (*(*h).fdec).plane[p as usize]
                            .offset((i * (*(*h).fdec).i_stride[p as usize]) as isize)
                            as *const ::core::ffi::c_void,
                        ((*h).mb.i_mb_width * 16i32 * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                    i += 1;
                }
                p += 1;
            }
        }
        if (*(*h).fdec).kept_as_ref && (!(*h).param.sliced_threads || pass == 1i32) {
            crate::src::common::frame::x264_8_frame_expand_border(h, (*h).fdec, min_y);
        }
        if b_hpel {
            let mut end = (mb_y == (*h).mb.i_mb_height) as ::core::ffi::c_int;
            if (*h).param.analyse.i_subpel_refine != 0 {
                crate::src::common::mc::x264_8_frame_filter(h, (*h).fdec, min_y, end);
                crate::src::common::frame::x264_8_frame_expand_border_filtered(
                    h,
                    (*h).fdec,
                    min_y,
                    end,
                );
            }
        }
        if (*h).sh.mbaff && pass == 0i32 {
            let mut i_0 = 0i32;
            while i_0 < 3i32 {
                let mut t = (*h).intra_border_backup[0usize][i_0 as usize];
                (*h).intra_border_backup[0usize][i_0 as usize] =
                    (*h).intra_border_backup[3usize][i_0 as usize];
                (*h).intra_border_backup[3usize][i_0 as usize] = t;
                let mut t_0 = (*h).intra_border_backup[1usize][i_0 as usize];
                (*h).intra_border_backup[1usize][i_0 as usize] =
                    (*h).intra_border_backup[4usize][i_0 as usize];
                (*h).intra_border_backup[4usize][i_0 as usize] = t_0;
                i_0 += 1;
            }
        }
        if (*h).i_thread_frames > 1i32 && (*(*h).fdec).kept_as_ref {
            crate::src::common::frame::x264_8_frame_cond_broadcast(
                (*h).fdec,
                mb_y * 16i32
                    + (if b_end != 0 {
                        10000i32
                    } else {
                        -(crate::src::common::base::X264_THREAD_HEIGHT
                            << (*h).sh.mbaff as ::core::ffi::c_int)
                    }),
            );
        }
        if b_measure_quality != 0 {
            maxpix_y = if maxpix_y < (*h).param.i_height {
                maxpix_y
            } else {
                (*h).param.i_height
            };
            if (*h).param.analyse.psnr {
                let mut p_0 = 0i32;
                while p_0
                    < (if (*h).sps.i_chroma_format_idc.is_444() {
                        3i32
                    } else {
                        1i32
                    })
                {
                    (*h).stat.frame.i_ssd[p_0 as usize] = ((*h).stat.frame.i_ssd[p_0 as usize]
                        as crate::stdlib::uint64_t)
                        .wrapping_add(crate::src::common::pixel::x264_8_pixel_ssd_wxh(
                            &raw mut (*h).pixf,
                            (*(*h).fdec).plane[p_0 as usize]
                                .offset((minpix_y * (*(*h).fdec).i_stride[p_0 as usize]) as isize),
                            (*(*h).fdec).i_stride[p_0 as usize] as crate::stdlib::intptr_t,
                            (*(*h).fenc).plane[p_0 as usize]
                                .offset((minpix_y * (*(*h).fenc).i_stride[p_0 as usize]) as isize),
                            (*(*h).fenc).i_stride[p_0 as usize] as crate::stdlib::intptr_t,
                            (*h).param.i_width,
                            maxpix_y - minpix_y,
                        ))
                        as crate::stdlib::int64_t;
                    p_0 += 1;
                }
                if !((*h).sps.i_chroma_format_idc.is_444()) {
                    let mut ssd_u = 0;
                    let mut ssd_v = 0;
                    let mut v_shift = ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
                    crate::src::common::pixel::x264_8_pixel_ssd_nv12(
                        &raw mut (*h).pixf,
                        (*(*h).fdec).plane[1usize].offset(
                            ((minpix_y >> v_shift) * (*(*h).fdec).i_stride[1usize]) as isize,
                        ),
                        (*(*h).fdec).i_stride[1usize] as crate::stdlib::intptr_t,
                        (*(*h).fenc).plane[1usize].offset(
                            ((minpix_y >> v_shift) * (*(*h).fenc).i_stride[1usize]) as isize,
                        ),
                        (*(*h).fenc).i_stride[1usize] as crate::stdlib::intptr_t,
                        (*h).param.i_width >> 1i32,
                        (maxpix_y - minpix_y) >> v_shift,
                        &raw mut ssd_u,
                        &raw mut ssd_v,
                    );
                    (*h).stat.frame.i_ssd[1usize] =
                        ((*h).stat.frame.i_ssd[1usize] as crate::stdlib::uint64_t)
                            .wrapping_add(ssd_u) as crate::stdlib::int64_t;
                    (*h).stat.frame.i_ssd[2usize] =
                        ((*h).stat.frame.i_ssd[2usize] as crate::stdlib::uint64_t)
                            .wrapping_add(ssd_v) as crate::stdlib::int64_t;
                }
            }
            if (*h).param.analyse.ssim {
                let mut ssim_cnt = 0;
                minpix_y += if b_start != 0 { 2i32 } else { -(6i32) };
                (*h).stat.frame.f_ssim += crate::src::common::pixel::x264_8_pixel_ssim_wxh(
                    &raw mut (*h).pixf,
                    (*(*h).fdec).plane[0usize]
                        .offset(2isize)
                        .offset((minpix_y * (*(*h).fdec).i_stride[0usize]) as isize),
                    (*(*h).fdec).i_stride[0usize] as crate::stdlib::intptr_t,
                    (*(*h).fenc).plane[0usize]
                        .offset(2isize)
                        .offset((minpix_y * (*(*h).fenc).i_stride[0usize]) as isize),
                    (*(*h).fenc).i_stride[0usize] as crate::stdlib::intptr_t,
                    (*h).param.i_width - 2i32,
                    maxpix_y - minpix_y,
                    (*h).scratch_buffer,
                    &raw mut ssim_cnt,
                ) as ::core::ffi::c_double;
                (*h).stat.frame.i_ssim_cnt += ssim_cnt;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn reference_update(mut h: *mut x264_t) -> ::core::ffi::c_int {
    unsafe {
        let mut i = 0i32;
        if !(*(*h).fdec).kept_as_ref {
            if (*h).i_thread_frames > 1i32 {
                crate::src::common::frame::x264_8_frame_push_unused(h, (*h).fdec);
                (*h).fdec = crate::src::common::frame::x264_8_frame_pop_unused(h, 1i32);
                if (*h).fdec.is_null() {
                    return -(1i32);
                }
            }
            return 0i32;
        }
        while i < (*h).sh.i_mmco_command_count {
            let mut j = 0i32;
            while !(*h).frames.reference[j as usize].is_null() {
                if (*(*h).frames.reference[j as usize]).i_poc == (*h).sh.mmco[i as usize].i_poc {
                    crate::src::common::frame::x264_8_frame_push_unused(
                        h,
                        crate::src::common::frame::x264_8_frame_shift(
                            (&raw mut (*h).frames.reference
                                as *mut *mut crate::src::common::frame::x264_frame_t)
                                .offset(j as isize),
                        ),
                    );
                }
                j += 1;
            }
            i += 1;
        }
        crate::src::common::frame::x264_8_frame_push(
            &raw mut (*h).frames.reference as *mut *mut crate::src::common::frame::x264_frame,
            (*h).fdec,
        );
        if !(*h).frames.reference[(*h).sps.i_num_ref_frames as usize].is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(
                h,
                crate::src::common::frame::x264_8_frame_shift(
                    &raw mut (*h).frames.reference
                        as *mut *mut crate::src::common::frame::x264_frame,
                ),
            );
        }
        (*h).fdec = crate::src::common::frame::x264_8_frame_pop_unused(h, 1i32);
        if (*h).fdec.is_null() {
            return -(1i32);
        }
        return 0i32;
    }
}
#[inline]
unsafe extern "C" fn reference_reset(mut h: *mut x264_t) {
    unsafe {
        while !(*h).frames.reference[0usize].is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(
                h,
                crate::src::common::frame::x264_8_frame_pop(
                    &raw mut (*h).frames.reference
                        as *mut *mut crate::src::common::frame::x264_frame,
                ),
            );
        }
        (*(*h).fenc).i_poc = 0i32;
        (*(*h).fdec).i_poc = (*(*h).fenc).i_poc;
    }
}
#[inline]
unsafe extern "C" fn reference_hierarchy_reset(mut h: *mut x264_t) {
    unsafe {
        let mut b_hasdelayframe = 0i32;
        let mut i = 0i32;
        let mut ref_0 = 0i32;
        while !(*(*h).frames.current.offset(i as isize)).is_null()
            && (**(*h).frames.current.offset(i as isize)).i_type == crate::x264_h::X264_TYPE_B
        {
            b_hasdelayframe |= ((**(*h).frames.current.offset(i as isize)).i_coded
                != (**(*h).frames.current.offset(i as isize)).i_frame
                    + (*h).sps.vui.i_num_reorder_frames)
                as ::core::ffi::c_int;
            i += 1;
        }
        if (*h).param.i_bframe_pyramid != crate::x264_h::X264_B_PYRAMID_STRICT
            && b_hasdelayframe == 0
            && (*h).frames.i_poc_last_open_gop == -(1i32)
        {
            return;
        }
        while !(*h).frames.reference[ref_0 as usize].is_null() {
            if (*h).param.i_bframe_pyramid == crate::x264_h::X264_B_PYRAMID_STRICT
                && (*(*h).frames.reference[ref_0 as usize]).i_type == crate::x264_h::X264_TYPE_BREF
                || (*(*h).frames.reference[ref_0 as usize]).i_poc < (*h).frames.i_poc_last_open_gop
                    && (*h).sh.i_type
                        != crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            {
                let mut diff =
                    (*h).i_frame_num - (*(*h).frames.reference[ref_0 as usize]).i_frame_num;
                (*h).sh.mmco[(*h).sh.i_mmco_command_count as usize].i_difference_of_pic_nums = diff;
                let c2rust_fresh7 = (*h).sh.i_mmco_command_count;
                (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_command_count + 1;
                (*h).sh.mmco[c2rust_fresh7 as usize].i_poc =
                    (*(*h).frames.reference[ref_0 as usize]).i_poc;
                crate::src::common::frame::x264_8_frame_push_unused(
                    h,
                    crate::src::common::frame::x264_8_frame_shift(
                        (&raw mut (*h).frames.reference
                            as *mut *mut crate::src::common::frame::x264_frame_t)
                            .offset(ref_0 as isize),
                    ),
                );
                (*h).ref_reorder[0usize] = true;
                ref_0 -= 1;
            }
            ref_0 += 1;
        }
        if (*h).param.i_bframe_pyramid != 0 {
            (*h).sh.i_mmco_remove_from_end = if ref_0 + 2i32 - (*h).frames.i_max_dpb > 0i32 {
                ref_0 + 2i32 - (*h).frames.i_max_dpb
            } else {
                0i32
            };
        }
    }
}
#[inline]
unsafe extern "C" fn slice_init(
    mut h: *mut x264_t,
    mut i_nal_type: ::core::ffi::c_int,
    mut i_global_qp: ::core::ffi::c_int,
) {
    unsafe {
        if i_nal_type == crate::x264_h::NAL_SLICE_IDR as ::core::ffi::c_int {
            slice_header_init(
                h,
                &raw mut (*h).sh,
                &(*h).sps,
                &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t,
                (*h).i_idr_pic_id,
                (*h).i_frame_num,
                i_global_qp,
            );
            if (*h).param.i_avcintra_class != 0 {
                match (*h).i_idr_pic_id {
                    5 => {
                        (*h).i_idr_pic_id = 3i32;
                    }
                    3 => {
                        (*h).i_idr_pic_id = 4i32;
                    }
                    4 | _ => {
                        (*h).i_idr_pic_id = 5i32;
                    }
                }
            } else {
                (*h).i_idr_pic_id ^= 1i32;
            }
        } else {
            slice_header_init(
                h,
                &raw mut (*h).sh,
                &(*h).sps,
                &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t,
                -(1i32),
                (*h).i_frame_num,
                i_global_qp,
            );
            (*h).sh.i_num_ref_idx_l0_active = if (*h).i_ref[0usize] <= 0i32 {
                1i32
            } else {
                (*h).i_ref[0usize]
            };
            (*h).sh.i_num_ref_idx_l1_active = if (*h).i_ref[1usize] <= 0i32 {
                1i32
            } else {
                (*h).i_ref[1usize]
            };
            if (*h).sh.i_num_ref_idx_l0_active
                != (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_num_ref_idx_l0_default_active
                || (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
                    && (*h).sh.i_num_ref_idx_l1_active
                        != (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                            .i_num_ref_idx_l1_default_active
            {
                (*h).sh.num_ref_idx_override = true;
            }
        }
        if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_BREF
            && (*h).param.bluray_compat
            && (*h).sh.i_mmco_command_count != 0
        {
            (*h).has_sh_backup = true;
            (*h).sh_backup = (*h).sh;
        }
        (*(*h).fdec).i_frame_num = (*h).sh.i_frame_num;
        if (*h).sps.i_poc_type == 0i32 {
            (*h).sh.i_poc = (*(*h).fdec).i_poc;
            if (*h).param.interlaced {
                (*h).sh.i_delta_poc_bottom = if (*h).param.tff { 1i32 } else { -(1i32) };
                (*h).sh.i_poc += ((*h).sh.i_delta_poc_bottom == -(1i32)) as ::core::ffi::c_int;
            } else {
                (*h).sh.i_delta_poc_bottom = 0i32;
            }
            (*(*h).fdec).i_delta_poc[0usize] =
                ((*h).sh.i_delta_poc_bottom == -(1i32)) as ::core::ffi::c_int;
            (*(*h).fdec).i_delta_poc[1usize] =
                ((*h).sh.i_delta_poc_bottom == 1i32) as ::core::ffi::c_int;
        }
        crate::src::common::macroblock::x264_8_macroblock_slice_init(h);
    }
}
#[inline(always)]
unsafe extern "C" fn bitstream_backup(
    mut h: *mut x264_t,
    mut bak: *mut x264_bs_bak_t,
    mut i_skip: ::core::ffi::c_int,
    mut full: ::core::ffi::c_int,
) {
    unsafe {
        if full != 0 {
            (*bak).stat = (*h).stat.frame;
            (*bak).last_qp = (*h).mb.i_last_qp;
            (*bak).last_dqp = (*h).mb.i_last_dqp;
            (*bak).field_decoding_flag = (*h).mb.field_decoding_flag;
        } else {
            (*bak).stat.i_mv_bits = (*h).stat.frame.i_mv_bits;
            (*bak).stat.i_tex_bits = (*h).stat.frame.i_tex_bits;
        }
        if (*h).param.cabac {
            if full != 0 {
                crate::stdlib::memcpy(
                    &raw mut (*bak).cabac as *mut ::core::ffi::c_void,
                    &raw mut (*h).cabac as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<crate::src::common::cabac::x264_cabac_t>(),
                );
            } else {
                crate::stdlib::memcpy(
                    &raw mut (*bak).cabac as *mut ::core::ffi::c_void,
                    &raw mut (*h).cabac as *const ::core::ffi::c_void,
                    64usize,
                );
            }
            (*bak).cabac_prevbyte = *(*h).cabac.p.offset(-1isize);
        } else {
            (*bak).bs = (*h).out.bs;
            (*bak).skip = i_skip;
        };
    }
}
#[inline(always)]
unsafe extern "C" fn bitstream_restore(
    mut h: *mut x264_t,
    mut bak: *mut x264_bs_bak_t,
    mut skip: *mut ::core::ffi::c_int,
    mut full: ::core::ffi::c_int,
) {
    unsafe {
        if full != 0 {
            (*h).stat.frame = (*bak).stat;
            (*h).mb.i_last_qp = (*bak).last_qp;
            (*h).mb.i_last_dqp = (*bak).last_dqp;
            (*h).mb.field_decoding_flag = (*bak).field_decoding_flag;
        } else {
            (*h).stat.frame.i_mv_bits = (*bak).stat.i_mv_bits;
            (*h).stat.frame.i_tex_bits = (*bak).stat.i_tex_bits;
        }
        if (*h).param.cabac {
            if full != 0 {
                crate::stdlib::memcpy(
                    &raw mut (*h).cabac as *mut ::core::ffi::c_void,
                    &raw mut (*bak).cabac as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<crate::src::common::cabac::x264_cabac_t>(),
                );
            } else {
                crate::stdlib::memcpy(
                    &raw mut (*h).cabac as *mut ::core::ffi::c_void,
                    &raw mut (*bak).cabac as *const ::core::ffi::c_void,
                    64usize,
                );
            }
            *(*h).cabac.p.offset(-1isize) = (*bak).cabac_prevbyte;
        } else {
            (*h).out.bs = (*bak).bs;
            *skip = (*bak).skip;
        };
    }
}
unsafe extern "C" fn slice_write(mut h: *mut x264_t) -> crate::stdlib::intptr_t {
    unsafe {
        let mut last_emu_check = ::core::ptr::null_mut::<crate::stdlib::uint8_t>();
        let mut i_skip = 0i32;
        let mut overhead_guess = crate::src::common::common::NALU_OVERHEAD
            - ((*h).param.annexb && (*h).out.i_nal != 0) as ::core::ffi::c_int
            + 1i32
            + (*h).param.cabac as ::core::ffi::c_int
            + 5i32;
        let mut slice_max_size = if (*h).param.i_slice_max_size > 0i32 {
            ((*h).param.i_slice_max_size - overhead_guess) * 8i32
        } else {
            0i32
        };
        let mut back_up_bitstream_cavlc = (!(*h).param.cabac
            && (*h).sps.i_profile_idc
                < crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut back_up_bitstream =
            (slice_max_size != 0 || back_up_bitstream_cavlc != 0) as ::core::ffi::c_int;
        let mut starting_bits = bs_pos(&raw mut (*h).out.bs);
        let mut b_deblock = ((*h).sh.i_disable_deblocking_filter_idc != 1i32) as ::core::ffi::c_int;
        let mut b_hpel = (*(*h).fdec).kept_as_ref;
        let mut orig_last_mb = (*h).sh.i_last_mb;
        let mut thread_last_mb = (*h).i_threadslice_end * (*h).mb.i_mb_width - 1i32;
        b_deblock &= (b_hpel || (*h).param.full_recon || !(*h).param.psz_dump_yuv.is_null())
            as ::core::ffi::c_int;
        bs_realign(&raw mut (*h).out.bs);
        nal_start(h, (*h).i_nal_type, (*h).i_nal_ref_idc);
        (*(*h).out.nal.offset((*h).out.i_nal as isize)).i_first_mb = (*h).sh.i_first_mb;
        crate::src::common::macroblock::x264_8_macroblock_thread_init(h);
        (*h).mb.i_mb_xy = (*h).sh.i_first_mb;
        (*h).sh.i_qp = crate::src::encoder::ratecontrol::x264_8_ratecontrol_mb_qp(h);
        (*h).sh.i_qp = if (*h).sh.i_qp < 51i32 + 6i32 * (8i32 - 8i32) {
            (*h).sh.i_qp
        } else {
            51i32 + 6i32 * (8i32 - 8i32)
        };
        (*h).sh.i_qp_delta = (*h).sh.i_qp
            - (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).i_pic_init_qp;
        slice_header_write(&raw mut (*h).out.bs, &raw mut (*h).sh, (*h).i_nal_ref_idc);
        if (*h).param.cabac {
            bs_align_1(&raw mut (*h).out.bs);
            crate::src::common::cabac::x264_8_cabac_context_init(
                h,
                &raw mut (*h).cabac,
                (*h).sh.i_type,
                x264_clip3(
                    (*h).sh.i_qp - crate::src::common::common::QP_BD_OFFSET,
                    0i32,
                    51i32,
                ),
                (*h).sh.i_cabac_init_idc,
            );
            crate::src::common::cabac::x264_8_cabac_encode_init(
                &raw mut (*h).cabac,
                (*h).out.bs.p,
                (*h).out.bs.p_end,
            );
            last_emu_check = (*h).cabac.p;
        } else {
            last_emu_check = (*h).out.bs.p;
        }
        (*h).mb.i_last_qp = (*h).sh.i_qp;
        (*h).mb.i_last_dqp = 0i32;
        (*h).mb.field_decoding_flag = 0i32;
        let mut i_mb_y = (*h).sh.i_first_mb / (*h).mb.i_mb_width;
        let mut i_mb_x = (*h).sh.i_first_mb % (*h).mb.i_mb_width;
        loop {
            let mut mb_xy = 0;
            let mut bs_bak = [x264_bs_bak_t {
                skip: 0,
                cabac_prevbyte: 0,
                bs: crate::src::common::bitstream::bs_s {
                    p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                    p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                    p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                    cur_bits: 0,
                    i_left: 0,
                    i_bits_encoded: 0,
                },
                cabac: crate::src::common::cabac::x264_cabac_t {
                    i_low: 0,
                    i_range: 0,
                    i_queue: 0,
                    i_bytes_outstanding: 0,
                    p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                    p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                    p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                    f8_bits_encoded: 0,
                    state: [0; 1024],
                    padding: [0; 12],
                },
                stat: crate::src::common::common::x264_frame_stat_t {
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
            mb_xy = i_mb_x + i_mb_y * (*h).mb.i_mb_width;
            let mut mb_spos = bs_pos(&raw mut (*h).out.bs) + x264_cabac_pos(&raw mut (*h).cabac);
            if i_mb_x == 0i32 {
                if bitstream_check_buffer(h) != 0 {
                    return -1isize;
                }
                if i_mb_y & (*h).sh.mbaff as ::core::ffi::c_int == 0
                    && (*h).param.rc.i_vbv_buffer_size != 0
                {
                    bitstream_backup(
                        h,
                        (&raw mut bs_bak as *mut x264_bs_bak_t).offset(BS_BAK_ROW_VBV as isize),
                        i_skip,
                        1i32,
                    );
                }
                if !(*h).mb.reencode_mb {
                    fdec_filter_row(h, i_mb_y, 0i32);
                }
            }
            if back_up_bitstream != 0 {
                if back_up_bitstream_cavlc != 0 {
                    bitstream_backup(
                        h,
                        (&raw mut bs_bak as *mut x264_bs_bak_t)
                            .offset(BS_BAK_CAVLC_OVERFLOW as isize),
                        i_skip,
                        0i32,
                    );
                }
                if slice_max_size != 0 && i_mb_y & (*h).sh.mbaff as ::core::ffi::c_int == 0 {
                    bitstream_backup(
                        h,
                        (&raw mut bs_bak as *mut x264_bs_bak_t)
                            .offset(BS_BAK_SLICE_MAX_SIZE as isize),
                        i_skip,
                        0i32,
                    );
                    if thread_last_mb + 1i32 - mb_xy == (*h).param.i_slice_min_mbs {
                        bitstream_backup(
                            h,
                            (&raw mut bs_bak as *mut x264_bs_bak_t)
                                .offset(BS_BAK_SLICE_MIN_MBS as isize),
                            i_skip,
                            0i32,
                        );
                    }
                }
            }
            if (*h).param.interlaced {
                if (*h).mb.adaptive_mbaff {
                    if i_mb_y & 1i32 == 0 {
                        (*h).mb.interlaced =
                            crate::src::common::pixel::x264_8_field_vsad(h, i_mb_x, i_mb_y);
                        crate::stdlib::memcpy(
                            &raw mut (*h).zigzagf as *mut ::core::ffi::c_void,
                            (if (*h).mb.interlaced {
                                &raw mut (*h).zigzagf_interlaced
                            } else {
                                &raw mut (*h).zigzagf_progressive
                            }) as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<crate::src::common::dct::x264_zigzag_function_t>(
                            ),
                        );
                        if !(*h).mb.interlaced && i_mb_y + 2i32 == (*h).mb.i_mb_height {
                            crate::src::common::frame::x264_8_expand_border_mbpair(
                                h, i_mb_x, i_mb_y,
                            );
                        }
                    }
                }
                *(*h).mb.field.offset(mb_xy as isize) =
                    (*h).mb.interlaced as crate::stdlib::uint8_t;
            }
            if (*h).sh.mbaff {
                crate::src::common::macroblock::x264_8_macroblock_cache_load_interlaced(
                    h, i_mb_x, i_mb_y,
                );
            } else {
                crate::src::common::macroblock::x264_8_macroblock_cache_load_progressive(
                    h, i_mb_x, i_mb_y,
                );
            }
            crate::src::encoder::analyse::x264_8_macroblock_analyse(h);
            loop {
                crate::src::encoder::macroblock::x264_8_macroblock_encode(h);
                if (*h).param.cabac {
                    if mb_xy > (*h).sh.i_first_mb && !((*h).sh.mbaff && i_mb_y & 1i32 != 0) {
                        crate::src::common::cabac::x264_8_cabac_encode_terminal_c(
                            &raw mut (*h).cabac,
                        );
                    }
                    if (*h).mb.i_type
                        == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                        || (*h).mb.i_type
                            == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
                    {
                        crate::src::encoder::cabac::x264_8_cabac_mb_skip(h, 1i32);
                    } else {
                        if (*h).sh.i_type
                            != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                        {
                            crate::src::encoder::cabac::x264_8_cabac_mb_skip(h, 0i32);
                        }
                        crate::src::encoder::cabac::x264_8_macroblock_write_cabac(
                            h,
                            &raw mut (*h).cabac,
                        );
                    }
                    break;
                } else if (*h).mb.i_type
                    == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
                {
                    i_skip += 1;
                    break;
                } else {
                    if (*h).sh.i_type
                        != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                    {
                        bs_write_ue_big(&raw mut (*h).out.bs, i_skip as ::core::ffi::c_uint);
                        i_skip = 0i32;
                    }
                    crate::src::encoder::cavlc::x264_8_macroblock_write_cavlc(h);
                    if !((*h).mb.overflow) {
                        break;
                    }
                    (*h).mb.i_qp += 1;
                    (*h).mb.i_chroma_qp =
                        *(*h).chroma_qp_table.offset((*h).mb.i_qp as isize) as ::core::ffi::c_int;
                    (*h).mb.i_skip_intra = 0i32;
                    (*h).mb.skip_mc = false;
                    (*h).mb.overflow = false;
                    bitstream_restore(
                        h,
                        (&raw mut bs_bak as *mut x264_bs_bak_t)
                            .offset(BS_BAK_CAVLC_OVERFLOW as isize),
                        &raw mut i_skip,
                        0i32,
                    );
                }
            }
            let mut total_bits = bs_pos(&raw mut (*h).out.bs) + x264_cabac_pos(&raw mut (*h).cabac);
            let mut mb_size = total_bits - mb_spos;
            if slice_max_size != 0 && (!(*h).sh.mbaff || i_mb_y & 1i32 != 0) {
                if !(*h).param.cabac {
                    total_bits += bs_size_ue_big(i_skip as ::core::ffi::c_uint);
                }
                let mut end = if (*h).param.cabac {
                    (*h).cabac.p
                } else {
                    (*h).out.bs.p
                };
                while last_emu_check < end.offset(-(2isize)) {
                    if *last_emu_check.offset(0isize) as ::core::ffi::c_int == 0i32
                        && *last_emu_check.offset(1isize) as ::core::ffi::c_int == 0i32
                        && *last_emu_check.offset(2isize) as ::core::ffi::c_int <= 3i32
                    {
                        slice_max_size -= 8i32;
                        last_emu_check = last_emu_check.offset(1);
                    }
                    last_emu_check = last_emu_check.offset(1);
                }
                if total_bits - starting_bits > slice_max_size && !(*h).mb.reencode_mb {
                    if crate::src::common::frame::x264_8_frame_new_slice(h, (*h).fdec) == 0 {
                        if mb_xy <= thread_last_mb
                            && thread_last_mb + 1i32 - mb_xy < (*h).param.i_slice_min_mbs
                        {
                            if thread_last_mb - (*h).param.i_slice_min_mbs
                                < (*h).sh.i_first_mb + (*h).param.i_slice_min_mbs
                            {
                                log::warn!(
                                    "slice-max-size violated (frame {}, cause: slice-min-mbs)",
                                    (*h).i_frame
                                );
                                slice_max_size = 0i32;
                            } else {
                                bitstream_restore(
                                    h,
                                    (&raw mut bs_bak as *mut x264_bs_bak_t)
                                        .offset(BS_BAK_SLICE_MIN_MBS as isize),
                                    &raw mut i_skip,
                                    0i32,
                                );
                                (*h).mb.reencode_mb = true;
                                (*h).sh.i_last_mb = thread_last_mb - (*h).param.i_slice_min_mbs;
                                break;
                            }
                        } else if mb_xy - (*h).sh.mbaff as ::core::ffi::c_int * (*h).mb.i_mb_stride
                            != (*h).sh.i_first_mb
                        {
                            bitstream_restore(
                                h,
                                (&raw mut bs_bak as *mut x264_bs_bak_t)
                                    .offset(BS_BAK_SLICE_MAX_SIZE as isize),
                                &raw mut i_skip,
                                0i32,
                            );
                            (*h).mb.reencode_mb = true;
                            if (*h).sh.mbaff {
                                if i_mb_x != 0 {
                                    (*h).sh.i_last_mb = mb_xy - 1i32
                                        + (*h).mb.i_mb_stride
                                            * (i_mb_y & 1i32 == 0) as ::core::ffi::c_int;
                                } else {
                                    (*h).sh.i_last_mb = (i_mb_y - 2i32
                                        + (i_mb_y & 1i32 == 0) as ::core::ffi::c_int)
                                        * (*h).mb.i_mb_stride
                                        + (*h).mb.i_mb_width
                                        - 1i32;
                                }
                            } else {
                                (*h).sh.i_last_mb = mb_xy - 1i32;
                            }
                            break;
                        } else {
                            (*h).sh.i_last_mb = mb_xy;
                        }
                    } else {
                        slice_max_size = 0i32;
                    }
                }
            }
            (*h).mb.reencode_mb = false;
            crate::src::common::macroblock::x264_8_macroblock_cache_save(h);
            if crate::src::encoder::ratecontrol::x264_8_ratecontrol_mb(h, mb_size) < 0i32 {
                bitstream_restore(
                    h,
                    (&raw mut bs_bak as *mut x264_bs_bak_t).offset(BS_BAK_ROW_VBV as isize),
                    &raw mut i_skip,
                    1i32,
                );
                (*h).mb.reencode_mb = true;
                i_mb_x = 0i32;
                i_mb_y = i_mb_y - (*h).sh.mbaff as ::core::ffi::c_int;
                (*h).mb.i_mb_prev_xy = i_mb_y * (*h).mb.i_mb_stride - 1i32;
                (*h).sh.i_last_mb = orig_last_mb;
            } else {
                (*h).stat.frame.i_mb_count[(*h).mb.i_type as usize] += 1;
                let mut b_intra = ((*h).mb.i_type
                    == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                let mut b_skip = ((*h).mb.i_type
                    == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                if (*h).param.i_log_level >= crate::x264_h::X264_LOG_INFO
                    || (*h).param.rc.stat_write
                {
                    if b_intra == 0
                        && b_skip == 0
                        && !((*h).mb.i_type
                            == crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int)
                    {
                        if (*h).mb.i_partition
                            != crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int
                        {
                            (*h).stat.frame.i_mb_partition[(*h).mb.i_partition as usize] += 4i32;
                        } else {
                            let mut i = 0i32;
                            while i < 4i32 {
                                (*h).stat.frame.i_mb_partition
                                    [(*h).mb.i_sub_partition[i as usize] as usize] += 1;
                                i += 1;
                            }
                        }
                        if (*h).param.i_frame_reference > 1i32 {
                            let mut i_list = 0i32;
                            while i_list
                                <= ((*h).sh.i_type
                                    == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                            {
                                let mut i_0 = 0i32;
                                while i_0 < 4i32 {
                                    let mut i_ref = (*h).mb.cache.ref_0[i_list as usize]
                                        [x264_scan8[(4i32 * i_0) as usize] as usize]
                                        as ::core::ffi::c_int;
                                    if i_ref >= 0i32 {
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
                if (*h).param.i_log_level >= crate::x264_h::X264_LOG_INFO {
                    if (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma != 0 {
                        if (*h).sps.i_chroma_format_idc.is_444() {
                            let mut i_1 = 0i32;
                            while i_1 < 4i32 {
                                if (*h).mb.i_cbp_luma & (1i32) << i_1 != 0 {
                                    let mut p = 0i32;
                                    while p < 3i32 {
                                        let mut s8 = i_1 * 4i32 + p * 16i32;
                                        let mut nnz8x8 = (*((&raw mut (*h).mb.cache.non_zero_count
                                            as *mut crate::stdlib::uint8_t)
                                            .offset(
                                                (*(&raw const x264_scan8
                                                    as *const crate::stdlib::uint8_t)
                                                    .offset(s8 as isize)
                                                    as ::core::ffi::c_int
                                                    + 0i32)
                                                    as isize,
                                            )
                                            as *mut crate::src::common::base::x264_union16_t))
                                            .i
                                            as ::core::ffi::c_int
                                            | (*((&raw mut (*h).mb.cache.non_zero_count
                                                as *mut crate::stdlib::uint8_t)
                                                .offset(
                                                    (*(&raw const x264_scan8
                                                        as *const crate::stdlib::uint8_t)
                                                        .offset(s8 as isize)
                                                        as ::core::ffi::c_int
                                                        + 8i32)
                                                        as isize,
                                                )
                                                as *mut crate::src::common::base::x264_union16_t))
                                                .i
                                                as ::core::ffi::c_int;
                                        (*h).stat.frame.i_mb_cbp[((b_intra == 0)
                                            as ::core::ffi::c_int
                                            + p * 2i32)
                                            as usize] += (nnz8x8 != 0) as ::core::ffi::c_int;
                                        p += 1;
                                    }
                                }
                                i_1 += 1;
                            }
                        } else {
                            let mut cbpsum = ((*h).mb.i_cbp_luma & 1i32)
                                + ((*h).mb.i_cbp_luma >> 1i32 & 1i32)
                                + ((*h).mb.i_cbp_luma >> 2i32 & 1i32)
                                + ((*h).mb.i_cbp_luma >> 3i32);
                            (*h).stat.frame.i_mb_cbp
                                [((b_intra == 0) as ::core::ffi::c_int + 0i32) as usize] += cbpsum;
                            (*h).stat.frame.i_mb_cbp
                                [((b_intra == 0) as ::core::ffi::c_int + 2i32) as usize] +=
                                ((*h).mb.i_cbp_chroma != 0) as ::core::ffi::c_int;
                            (*h).stat.frame.i_mb_cbp
                                [((b_intra == 0) as ::core::ffi::c_int + 4i32) as usize] +=
                                (*h).mb.i_cbp_chroma >> 1i32;
                        }
                    }
                    if (*h).mb.i_cbp_luma != 0 && b_intra == 0 {
                        (*h).stat.frame.i_mb_count_8x8dct[0usize] += 1;
                        (*h).stat.frame.i_mb_count_8x8dct[1usize] +=
                            (*h).mb.transform_8x8 as ::core::ffi::c_int;
                    }
                    if b_intra != 0
                        && (*h).mb.i_type
                            != crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
                    {
                        if (*h).mb.i_type
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        {
                            (*h).stat.frame.i_mb_pred_mode[0usize]
                                [(*h).mb.i_intra16x16_pred_mode as usize] += 1;
                        } else if (*h).mb.i_type
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        {
                            let mut i_2 = 0i32;
                            while i_2 < 16i32 {
                                (*h).stat.frame.i_mb_pred_mode[1usize][(*h)
                                    .mb
                                    .cache
                                    .intra4x4_pred_mode
                                    [x264_scan8[i_2 as usize] as usize]
                                    as usize] += 1;
                                i_2 += 4i32;
                            }
                        } else {
                            let mut i_3 = 0i32;
                            while i_3 < 16i32 {
                                (*h).stat.frame.i_mb_pred_mode[2usize][(*h)
                                    .mb
                                    .cache
                                    .intra4x4_pred_mode
                                    [x264_scan8[i_3 as usize] as usize]
                                    as usize] += 1;
                                i_3 += 1;
                            }
                        }
                        (*h).stat.frame.i_mb_pred_mode[3usize][x264_mb_chroma_pred_mode_fix
                            [(*h).mb.i_chroma_pred_mode as usize]
                            as usize] += 1;
                    }
                    (*h).stat.frame.i_mb_field[(if b_intra != 0 {
                        0i32
                    } else if b_skip != 0 {
                        2i32
                    } else {
                        1i32
                    }) as usize] += (*h).mb.interlaced as ::core::ffi::c_int;
                }
                if b_deblock != 0 {
                    crate::src::common::macroblock::x264_8_macroblock_deblock_strength(h);
                }
                if mb_xy == (*h).sh.i_last_mb {
                    break;
                }
                if (*h).sh.mbaff {
                    i_mb_x += i_mb_y & 1i32;
                    i_mb_y ^= (i_mb_x < (*h).mb.i_mb_width) as ::core::ffi::c_int;
                } else {
                    i_mb_x += 1;
                }
                if i_mb_x == (*h).mb.i_mb_width {
                    i_mb_y += 1;
                    i_mb_x = 0i32;
                }
            }
        }
        if (*h).sh.i_last_mb < (*h).sh.i_first_mb {
            return 0isize;
        }
        (*(*h).out.nal.offset((*h).out.i_nal as isize)).i_last_mb = (*h).sh.i_last_mb;
        if (*h).param.cabac {
            crate::src::common::cabac::x264_8_cabac_encode_flush(h, &raw mut (*h).cabac);
            (*h).out.bs.p = (*h).cabac.p;
        } else {
            if i_skip > 0i32 {
                bs_write_ue_big(&raw mut (*h).out.bs, i_skip as ::core::ffi::c_uint);
            }
            bs_rbsp_trailing(&raw mut (*h).out.bs);
            bs_flush(&raw mut (*h).out.bs);
        }
        if nal_end(h) != 0 {
            return -1isize;
        }
        if (*h).sh.i_last_mb == (*h).i_threadslice_end * (*h).mb.i_mb_width - 1i32 {
            (*h).stat.frame.i_misc_bits = bs_pos(&raw mut (*h).out.bs)
                + (*h).out.i_nal * crate::src::common::common::NALU_OVERHEAD * 8i32
                - (*h).stat.frame.i_tex_bits
                - (*h).stat.frame.i_mv_bits;
            fdec_filter_row(h, (*h).i_threadslice_end, 0i32);
            if (*h).param.sliced_threads {
                crate::src::common::frame::x264_8_threadslice_cond_broadcast(h, 1i32);
                let mut mb_y = (*h).i_threadslice_start;
                while mb_y <= (*h).i_threadslice_end {
                    fdec_filter_row(h, mb_y, 1i32);
                    mb_y += 1;
                }
                crate::src::common::frame::x264_8_threadslice_cond_broadcast(h, 2i32);
                if (*h).i_thread_idx > 0i32 {
                    crate::src::common::frame::x264_8_threadslice_cond_wait(
                        (*h).thread[((*h).i_thread_idx - 1i32) as usize],
                        2i32,
                    );
                    fdec_filter_row(
                        h,
                        (*h).i_threadslice_start + ((1i32) << (*h).sh.mbaff as ::core::ffi::c_int),
                        2i32,
                    );
                }
            }
            if (*(*h).fdec).mb_info_free.is_some()
                && (!(*h).param.sliced_threads || (*h).i_thread_idx == (*h).param.i_threads - 1i32)
            {
                (*(*h).fdec)
                    .mb_info_free
                    .expect("non-null function pointer")(
                    (*(*h).fdec).mb_info as *mut ::core::ffi::c_void,
                );
                (*(*h).fdec).mb_info = ::core::ptr::null_mut::<crate::stdlib::uint8_t>();
                (*(*h).fdec).mb_info_free = None;
            }
        }
        return 0isize;
    }
}
pub const BS_BAK_SLICE_MAX_SIZE: ::core::ffi::c_int = 0i32;
pub const BS_BAK_CAVLC_OVERFLOW: ::core::ffi::c_int = 1i32;
pub const BS_BAK_SLICE_MIN_MBS: ::core::ffi::c_int = 2i32;
pub const BS_BAK_ROW_VBV: ::core::ffi::c_int = 3i32;
unsafe extern "C" fn thread_sync_context<'a>(mut dst: *mut x264_t<'a>, mut src: *mut x264_t<'a>) {
    unsafe {
        if dst == src {
            return;
        }
        let mut f =
            &raw mut (*src).frames.reference as *mut *mut crate::src::common::frame::x264_frame_t;
        while !(*f).is_null() {
            (**f).i_reference_count += 1;
            f = f.offset(1);
        }
        let mut f_0 =
            &raw mut (*dst).frames.reference as *mut *mut crate::src::common::frame::x264_frame_t;
        while !(*f_0).is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(src, *f_0);
            f_0 = f_0.offset(1);
        }
        (*(*src).fdec).i_reference_count += 1;
        crate::src::common::frame::x264_8_frame_push_unused(src, (*dst).fdec);
        crate::stdlib::memcpy(
            &raw mut (*dst).i_frame as *mut ::core::ffi::c_void,
            &raw mut (*src).i_frame as *const ::core::ffi::c_void,
            (24912usize).wrapping_sub(2420usize),
        );
        (*dst).param = (*src).param;
        (*dst).stat = (*src).stat;
        (*dst).pixf = (*src).pixf;
        (*dst).reconfig = (*src).reconfig;
    }
}
unsafe extern "C" fn thread_sync_stat<'a>(mut dst: *mut x264_t<'a>, mut src: *mut x264_t<'a>) {
    unsafe {
        if dst != src {
            crate::stdlib::memcpy(
                &raw mut (*dst).stat as *mut ::core::ffi::c_void,
                &raw mut (*src).stat as *const ::core::ffi::c_void,
                (42848usize).wrapping_sub(40264usize),
            );
        }
    }
}
unsafe extern "C" fn slices_write(mut h: *mut x264_t) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut last_thread_mb = (*h).sh.i_last_mb;
        let mut round_bias = if (*h).param.i_avcintra_class != 0 {
            0i32
        } else {
            (*h).param.i_slice_count / 2i32
        };
        crate::stdlib::memset(
            &raw mut (*h).stat.frame as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<crate::src::common::common::x264_frame_stat_t>(),
        );
        (*h).mb.reencode_mb = false;
        loop {
            let mut i_slice_num = 0i32;
            if !((*h).sh.i_first_mb + (*h).sh.mbaff as ::core::ffi::c_int * (*h).mb.i_mb_stride
                <= last_thread_mb)
            {
                c2rust_current_block = 5634871135123216486;
                break;
            }
            (*h).sh.i_last_mb = last_thread_mb;
            if i_slice_num == 0
                || crate::src::common::frame::x264_8_frame_new_slice(h, (*h).fdec) == 0
            {
                if (*h).param.i_slice_max_mbs != 0 {
                    if (*h).sh.mbaff {
                        let mut last_mbaff = 2i32 * ((*h).sh.i_first_mb % (*h).mb.i_mb_width)
                            + (*h).mb.i_mb_width * ((*h).sh.i_first_mb / (*h).mb.i_mb_width)
                            + (*h).param.i_slice_max_mbs
                            - 1i32;
                        let mut last_x = last_mbaff % (2i32 * (*h).mb.i_mb_width) / 2i32;
                        let mut last_y = last_mbaff / (2i32 * (*h).mb.i_mb_width) * 2i32 + 1i32;
                        (*h).sh.i_last_mb = last_x + (*h).mb.i_mb_stride * last_y;
                    } else {
                        (*h).sh.i_last_mb = (*h).sh.i_first_mb + (*h).param.i_slice_max_mbs - 1i32;
                        if (*h).sh.i_last_mb < last_thread_mb
                            && last_thread_mb - (*h).sh.i_last_mb < (*h).param.i_slice_min_mbs
                        {
                            (*h).sh.i_last_mb = last_thread_mb - (*h).param.i_slice_min_mbs;
                        }
                    }
                    i_slice_num += 1;
                } else if (*h).param.i_slice_count != 0 && !(*h).param.sliced_threads {
                    let mut height =
                        (*h).mb.i_mb_height >> (*h).param.interlaced as ::core::ffi::c_int;
                    let mut width =
                        (*h).mb.i_mb_width << (*h).param.interlaced as ::core::ffi::c_int;
                    i_slice_num += 1;
                    (*h).sh.i_last_mb =
                        (height * i_slice_num + round_bias) / (*h).param.i_slice_count * width
                            - 1i32;
                }
            }
            (*h).sh.i_last_mb = if (*h).sh.i_last_mb < last_thread_mb {
                (*h).sh.i_last_mb
            } else {
                last_thread_mb
            };
            if slice_write(h) != 0 {
                c2rust_current_block = 12683324716538595482;
                break;
            }
            (*h).sh.i_first_mb = (*h).sh.i_last_mb + 1i32;
            if (*h).sh.mbaff && (*h).sh.i_first_mb % (*h).mb.i_mb_width != 0 {
                (*h).sh.i_first_mb -= (*h).mb.i_mb_stride;
            }
        }
        match c2rust_current_block {
            5634871135123216486 => return ::core::ptr::null_mut::<::core::ffi::c_void>(),
            _ => {
                if (*h).param.sliced_threads {
                    crate::src::common::frame::x264_8_threadslice_cond_broadcast(h, 2i32);
                }
                return -(1i32) as *mut ::core::ffi::c_void;
            }
        };
    }
}
unsafe extern "C" fn threaded_slices_write(mut h: *mut x264_t) -> ::core::ffi::c_int {
    unsafe {
        let mut i = 0i32;
        let mut i_0 = 0i32;
        let mut i_1 = 0i32;
        let mut i_2 = 0i32;
        let mut i_3 = 1i32;
        let mut round_bias = if (*h).param.i_avcintra_class != 0 {
            0i32
        } else {
            (*h).param.i_slice_count / 2i32
        };
        while i < (*h).param.i_threads {
            let mut t = (*h).thread[i as usize];
            if i != 0 {
                (*t).param = (*h).param;
                crate::stdlib::memcpy(
                    &raw mut (*t).i_frame as *mut ::core::ffi::c_void,
                    &raw mut (*h).i_frame as *const ::core::ffi::c_void,
                    (40256usize).wrapping_sub(2420usize),
                );
            }
            let mut height = (*h).mb.i_mb_height >> (*h).param.interlaced as ::core::ffi::c_int;
            (*t).i_threadslice_start = ((height * i + round_bias) / (*h).param.i_threads)
                << (*h).param.interlaced as ::core::ffi::c_int;
            (*t).i_threadslice_end = ((height * (i + 1i32) + round_bias) / (*h).param.i_threads)
                << (*h).param.interlaced as ::core::ffi::c_int;
            (*t).sh.i_first_mb = (*t).i_threadslice_start * (*h).mb.i_mb_width;
            (*t).sh.i_last_mb = (*t).i_threadslice_end * (*h).mb.i_mb_width - 1i32;
            i += 1;
        }
        crate::src::encoder::analyse::x264_8_analyse_weight_frame(
            h,
            (*h).mb.i_mb_height * 16i32 + 16i32,
        );
        crate::src::encoder::ratecontrol::x264_8_threads_distribute_ratecontrol(h);
        while i_0 < (*h).param.i_threads {
            (*(*h).thread[i_0 as usize]).i_thread_idx = i_0;
            (*(*h).thread[i_0 as usize]).thread_active = true;
            crate::src::common::frame::x264_8_threadslice_cond_broadcast(
                (*h).thread[i_0 as usize],
                0i32,
            );
            i_0 += 1;
        }
        while i_1 < (*h).param.i_threads {
            crate::src::common::threadpool::x264_8_threadpool_run(
                (*h).threadpool,
                ::core::mem::transmute::<
                    *mut ::core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
                    >,
                >(::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut x264_t) -> *mut ::core::ffi::c_void>,
                    *mut ::core::ffi::c_void,
                >(Some(
                    slices_write as unsafe extern "C" fn(*mut x264_t) -> *mut ::core::ffi::c_void,
                ))),
                (*h).thread[i_1 as usize] as *mut ::core::ffi::c_void,
            );
            i_1 += 1;
        }
        while i_2 < (*h).param.i_threads {
            crate::src::common::frame::x264_8_threadslice_cond_wait(
                (*h).thread[i_2 as usize],
                1i32,
            );
            i_2 += 1;
        }
        crate::src::encoder::ratecontrol::x264_8_threads_merge_ratecontrol(h);
        while i_3 < (*h).param.i_threads {
            let mut j = 0i32;
            let mut j_0 = 0usize;
            let mut j_1 = 0i32;
            let mut t_0 = (*h).thread[i_3 as usize];
            while j < (*t_0).out.i_nal {
                *(*h).out.nal.offset((*h).out.i_nal as isize) = *(*t_0).out.nal.offset(j as isize);
                (*h).out.i_nal += 1;
                nal_check_buffer(h);
                j += 1;
            }
            while j_0
                < (43536usize)
                    .wrapping_sub(42848usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>())
            {
                *(&raw mut (*h).stat.frame as *mut ::core::ffi::c_int).offset(j_0 as isize) +=
                    *(&raw mut (*t_0).stat.frame as *mut ::core::ffi::c_int).offset(j_0 as isize);
                j_0 = j_0.wrapping_add(1);
            }
            while j_1 < 3i32 {
                (*h).stat.frame.i_ssd[j_1 as usize] += (*t_0).stat.frame.i_ssd[j_1 as usize];
                j_1 += 1;
            }
            (*h).stat.frame.f_ssim += (*t_0).stat.frame.f_ssim;
            (*h).stat.frame.i_ssim_cnt += (*t_0).stat.frame.i_ssim_cnt;
            i_3 += 1;
        }
        return 0i32;
    }
}
pub unsafe extern "C" fn x264_8_encoder_intra_refresh(mut h: *mut x264_t) {
    unsafe {
        h = (*h).thread[(*h).i_thread_phase as usize];
        (*h).queued_intra_refresh = true;
    }
}
pub unsafe extern "C" fn x264_8_encoder_invalidate_reference(
    mut h: *mut x264_t,
    mut pts: crate::stdlib::int64_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*h).param.i_bframe != 0 {
            log::error!("x264_encoder_invalidate_reference is not supported with B-frames enabled");
            return -(1i32);
        }
        if (*h).param.intra_refresh {
            log::error!(
                "x264_encoder_invalidate_reference is not supported with intra refresh enabled"
            );
            return -(1i32);
        }
        h = (*h).thread[(*h).i_thread_phase as usize];
        if pts >= (*h).i_last_idr_pts {
            let mut i = 0i32;
            while !(*h).frames.reference[i as usize].is_null() {
                if pts <= (*(*h).frames.reference[i as usize]).i_pts {
                    (*(*h).frames.reference[i as usize]).corrupt = true;
                }
                i += 1;
            }
            if pts <= (*(*h).fdec).i_pts {
                (*(*h).fdec).corrupt = true;
            }
        }
        return 0i32;
    }
}
pub unsafe extern "C" fn x264_8_encoder_encode(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_in: *mut crate::x264_h::x264_picture_t,
    mut pic_out: *mut crate::x264_h::x264_picture_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut thread_current = ::core::ptr::null_mut::<x264_t>();
        let mut thread_oldest = ::core::ptr::null_mut::<x264_t>();
        let mut i_nal_type = 0;
        let mut i_nal_ref_idc = 0;
        let mut i_1 = 0i32;
        let mut overhead = crate::src::common::common::NALU_OVERHEAD;
        if (*h).i_thread_frames > 1i32 {
            let mut thread_prev = ::core::ptr::null_mut::<x264_t>();
            thread_prev = (*h).thread[(*h).i_thread_phase as usize];
            (*h).i_thread_phase = ((*h).i_thread_phase + 1i32) % (*h).i_thread_frames;
            thread_current = (*h).thread[(*h).i_thread_phase as usize];
            thread_oldest =
                (*h).thread[(((*h).i_thread_phase + 1i32) % (*h).i_thread_frames) as usize];
            thread_sync_context(thread_current, thread_prev);
            crate::src::encoder::ratecontrol::x264_8_thread_sync_ratecontrol(
                thread_current,
                thread_prev,
                thread_oldest,
            );
            h = thread_current;
        } else {
            thread_oldest = h;
            thread_current = thread_oldest;
        }
        (*h).i_cpb_delay_pir_offset = (*h).i_cpb_delay_pir_offset_next;
        *pi_nal = 0i32;
        *pp_nal = ::core::ptr::null_mut::<crate::x264_h::x264_nal_t>();
        if !pic_in.is_null() {
            if (*(*h).lookahead).b_exit_thread != 0 {
                log::error!("lookahead thread is already stopped");
                return -(1i32);
            }
            let mut fenc = crate::src::common::frame::x264_8_frame_pop_unused(h, 0i32);
            if fenc.is_null() {
                return -(1i32);
            }
            if crate::src::common::frame::x264_8_frame_copy_picture(
                h,
                fenc,
                pic_in as *mut crate::x264_h::x264_picture_t,
            ) < 0i32
            {
                return -(1i32);
            }
            if (*h).param.i_width != 16i32 * (*h).mb.i_mb_width
                || (*h).param.i_height != 16i32 * (*h).mb.i_mb_height
            {
                crate::src::common::frame::x264_8_frame_expand_border_mod16(h, fenc);
            }
            let c2rust_fresh8 = (*h).frames.i_input;
            (*h).frames.i_input = (*h).frames.i_input + 1;
            (*fenc).i_frame = c2rust_fresh8;
            if (*fenc).i_frame == 0i32 {
                (*h).frames.i_first_pts = (*fenc).i_pts;
            }
            if (*h).frames.i_bframe_delay != 0 && (*fenc).i_frame == (*h).frames.i_bframe_delay {
                (*h).frames.i_bframe_delay_time = (*fenc).i_pts - (*h).frames.i_first_pts;
            }
            if (*h).param.vfr_input && (*fenc).i_pts <= (*h).frames.i_largest_pts {
                log::warn!("non-strictly-monotonic PTS");
            }
            (*h).frames.i_second_largest_pts = (*h).frames.i_largest_pts;
            (*h).frames.i_largest_pts = (*fenc).i_pts;
            if (*fenc).i_pic_struct < crate::x264_h::PIC_STRUCT_AUTO as ::core::ffi::c_int
                || (*fenc).i_pic_struct > crate::x264_h::PIC_STRUCT_TRIPLE as ::core::ffi::c_int
            {
                (*fenc).i_pic_struct = crate::x264_h::PIC_STRUCT_AUTO as ::core::ffi::c_int;
            }
            if (*fenc).i_pic_struct == crate::x264_h::PIC_STRUCT_AUTO as ::core::ffi::c_int {
                let mut b_interlaced = if !(*fenc).param.is_null() {
                    (*(*fenc).param).interlaced
                } else {
                    (*h).param.interlaced
                };
                if b_interlaced {
                    let mut b_tff = if !(*fenc).param.is_null() {
                        (*(*fenc).param).tff
                    } else {
                        (*h).param.tff
                    };
                    (*fenc).i_pic_struct = if b_tff {
                        crate::x264_h::PIC_STRUCT_TOP_BOTTOM as ::core::ffi::c_int
                    } else {
                        crate::x264_h::PIC_STRUCT_BOTTOM_TOP as ::core::ffi::c_int
                    };
                } else {
                    (*fenc).i_pic_struct =
                        crate::x264_h::PIC_STRUCT_PROGRESSIVE as ::core::ffi::c_int;
                }
            }
            if (*h).param.rc.mb_tree && (*h).param.rc.stat_read {
                if crate::src::encoder::ratecontrol::x264_8_macroblock_tree_read(
                    h,
                    fenc,
                    (*pic_in).prop.quant_offsets,
                ) != 0
                {
                    return -(1i32);
                }
            } else {
                crate::src::encoder::ratecontrol::x264_8_adaptive_quant_frame(
                    h,
                    fenc,
                    (*pic_in).prop.quant_offsets,
                );
            }
            if (*pic_in).prop.quant_offsets_free.is_some() {
                (*pic_in)
                    .prop
                    .quant_offsets_free
                    .expect("non-null function pointer")(
                    (*pic_in).prop.quant_offsets as *mut ::core::ffi::c_void,
                );
            }
            if (*h).frames.have_lowres {
                crate::src::common::mc::x264_8_frame_init_lowres(h, fenc);
            }
            crate::src::encoder::lookahead::x264_8_lookahead_put_frame(h, fenc);
            if (*h).frames.i_input <= (*h).frames.i_delay + 1i32 - (*h).i_thread_frames {
                (*pic_out).i_type = crate::x264_h::X264_TYPE_AUTO;
                return 0i32;
            }
        } else {
            crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ifbuf.mutex);
            ::core::ptr::write_volatile(
                &mut (*(*h).lookahead).b_exit_thread as *mut crate::stdlib::uint8_t,
                1u8,
            );
            crate::stdlib::pthread_cond_broadcast(&raw mut (*(*h).lookahead).ifbuf.cv_fill);
            crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
        }
        (*h).i_frame += 1;
        if (*(*h).frames.current.offset(0isize)).is_null() {
            crate::src::encoder::lookahead::x264_8_lookahead_get_frames(h);
        }
        if (*(*h).frames.current.offset(0isize)).is_null()
            && crate::src::encoder::lookahead::x264_8_lookahead_is_empty(h) != 0
        {
            return encoder_frame_end(thread_oldest, thread_current, pp_nal, pi_nal, pic_out);
        }
        (*h).fenc = crate::src::common::frame::x264_8_frame_shift((*h).frames.current);
        if (*h).param.sliced_threads {
            if threadpool_wait_all(h) < 0i32 {
                return -(1i32);
            }
        }
        if (*h).i_frame == 0i32 {
            (*h).i_reordered_pts_delay = (*(*h).fenc).i_reordered_pts;
        }
        if (*h).reconfig != 0 {
            x264_8_encoder_reconfig_apply(h, &raw mut (*(*h).reconfig_h).param);
            (*h).reconfig = 0i32;
        }
        if !(*(*h).fenc).param.is_null() {
            x264_8_encoder_reconfig_apply(h, (*(*h).fenc).param);
            if (*(*(*h).fenc).param).param_free.is_some() {
                crate::src::common::base::x264_param_cleanup((*(*h).fenc).param);
                (*(*(*h).fenc).param)
                    .param_free
                    .expect("non-null function pointer")(
                    (*(*h).fenc).param as *mut ::core::ffi::c_void,
                );
                (*(*h).fenc).param = ::core::ptr::null_mut::<crate::x264_h::x264_param_t>();
            }
        }
        crate::src::encoder::ratecontrol::x264_8_ratecontrol_zone_init(h);
        if reference_update(h) != 0 {
            return -(1i32);
        }
        (*(*h).fdec).i_lines_completed = -(1i32);
        if !((*(*h).fenc).i_type == crate::x264_h::X264_TYPE_I
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_KEYFRAME)
        {
            let mut valid_refs_left = 0i32;
            let mut i = 0i32;
            while !(*h).frames.reference[i as usize].is_null() {
                if !(*(*h).frames.reference[i as usize]).corrupt {
                    valid_refs_left += 1;
                }
                i += 1;
            }
            if valid_refs_left == 0 {
                (*(*h).fenc).keyframe = true;
                (*(*h).fenc).i_type = crate::x264_h::X264_TYPE_IDR;
            }
        }
        if (*(*h).fenc).keyframe {
            (*h).frames.i_last_keyframe = (*(*h).fenc).i_frame;
            if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR {
                (*h).i_frame_num = 0i32;
                (*h).frames.i_last_idr = (*(*h).fenc).i_frame;
            }
        }
        (*h).sh.i_mmco_remove_from_end = 0i32;
        (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_remove_from_end;
        (*h).ref_reorder[1usize] = false;
        (*h).ref_reorder[0usize] = (*h).ref_reorder[1usize];
        (*(*h).fenc).i_poc = 2i32
            * ((*(*h).fenc).i_frame
                - (if (*h).frames.i_last_idr > 0i32 {
                    (*h).frames.i_last_idr
                } else {
                    0i32
                }));
        (*(*h).fdec).i_poc = (*(*h).fenc).i_poc;
        if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR {
            i_nal_type = crate::x264_h::NAL_SLICE_IDR as ::core::ffi::c_int;
            i_nal_ref_idc = crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int;
            (*h).sh.i_type = crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int;
            reference_reset(h);
            (*h).frames.i_poc_last_open_gop = -(1i32);
        } else if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_I {
            i_nal_type = crate::x264_h::NAL_SLICE as ::core::ffi::c_int;
            i_nal_ref_idc = crate::x264_h::NAL_PRIORITY_HIGH as ::core::ffi::c_int;
            (*h).sh.i_type = crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int;
            reference_hierarchy_reset(h);
            if (*h).param.open_gop {
                (*h).frames.i_poc_last_open_gop = if (*(*h).fenc).keyframe {
                    (*(*h).fenc).i_poc
                } else {
                    -(1i32)
                };
            }
        } else if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_P {
            i_nal_type = crate::x264_h::NAL_SLICE as ::core::ffi::c_int;
            i_nal_ref_idc = crate::x264_h::NAL_PRIORITY_HIGH as ::core::ffi::c_int;
            (*h).sh.i_type = crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int;
            reference_hierarchy_reset(h);
            (*h).frames.i_poc_last_open_gop = -(1i32);
        } else if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_BREF {
            i_nal_type = crate::x264_h::NAL_SLICE as ::core::ffi::c_int;
            i_nal_ref_idc = if (*h).param.i_bframe_pyramid == crate::x264_h::X264_B_PYRAMID_STRICT {
                crate::x264_h::NAL_PRIORITY_LOW as ::core::ffi::c_int
            } else {
                crate::x264_h::NAL_PRIORITY_HIGH as ::core::ffi::c_int
            };
            (*h).sh.i_type = crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int;
            reference_hierarchy_reset(h);
        } else {
            i_nal_type = crate::x264_h::NAL_SLICE as ::core::ffi::c_int;
            i_nal_ref_idc = crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int;
            (*h).sh.i_type = crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int;
        }
        (*(*h).fdec).i_type = (*(*h).fenc).i_type;
        (*(*h).fdec).i_frame = (*(*h).fenc).i_frame;
        (*(*h).fdec).kept_as_ref = i_nal_ref_idc
            != crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int
            && (*h).param.i_keyint_max > 1i32;
        (*(*h).fenc).kept_as_ref = (*(*h).fdec).kept_as_ref;
        (*(*h).fdec).mb_info = (*(*h).fenc).mb_info;
        (*(*h).fdec).mb_info_free = (*(*h).fenc).mb_info_free;
        (*(*h).fenc).mb_info = ::core::ptr::null_mut::<crate::stdlib::uint8_t>();
        (*(*h).fenc).mb_info_free = None;
        (*(*h).fdec).i_pts = (*(*h).fenc).i_pts;
        if (*h).frames.i_bframe_delay != 0 {
            let mut prev_reordered_pts = &raw mut (*thread_current).frames.i_prev_reordered_pts
                as *mut crate::stdlib::int64_t;
            (*(*h).fdec).i_dts = if (*h).i_frame > (*h).frames.i_bframe_delay {
                *prev_reordered_pts.offset(
                    (((*h).i_frame - (*h).frames.i_bframe_delay) % (*h).frames.i_bframe_delay)
                        as isize,
                )
            } else {
                (*(*h).fenc).i_reordered_pts - (*h).frames.i_bframe_delay_time
            };
            *prev_reordered_pts.offset(((*h).i_frame % (*h).frames.i_bframe_delay) as isize) =
                (*(*h).fenc).i_reordered_pts;
        } else {
            (*(*h).fdec).i_dts = (*(*h).fenc).i_reordered_pts;
        }
        if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR {
            (*h).i_last_idr_pts = (*(*h).fdec).i_pts;
        }
        reference_build_list(h, (*(*h).fdec).i_poc);
        if (*h).param.sliced_threads {
            let mut i_0 = 0i32;
            while i_0 < (*h).param.i_threads {
                bs_init(
                    &raw mut (**(&raw mut (*h).thread as *mut *mut x264_t).offset(i_0 as isize))
                        .out
                        .bs,
                    (*(*h).thread[i_0 as usize]).out.p_bitstream as *mut ::core::ffi::c_void,
                    (*(*h).thread[i_0 as usize]).out.i_bitstream,
                );
                (*(*h).thread[i_0 as usize]).out.i_nal = 0i32;
                i_0 += 1;
            }
        } else {
            bs_init(
                &raw mut (*h).out.bs,
                (*h).out.p_bitstream as *mut ::core::ffi::c_void,
                (*h).out.i_bitstream,
            );
            (*h).out.i_nal = 0i32;
        }
        if (*h).param.aud {
            let mut pic_type = 0;
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
                pic_type = 0i32;
            } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            {
                pic_type = 1i32;
            } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            {
                pic_type = 2i32;
            } else {
                pic_type = 7i32;
            }
            nal_start(
                h,
                crate::x264_h::NAL_AUD as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            bs_write(
                &raw mut (*h).out.bs,
                3i32,
                pic_type as crate::stdlib::uint32_t,
            );
            bs_rbsp_trailing(&raw mut (*h).out.bs);
            bs_flush(&raw mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1i32);
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                + crate::src::common::common::NALU_OVERHEAD;
        }
        (*h).i_nal_type = i_nal_type;
        (*h).i_nal_ref_idc = i_nal_ref_idc;
        if (*h).param.intra_refresh {
            if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_I
                || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR
                || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_KEYFRAME
            {
                (*(*h).fdec).i_frames_since_pir = 0i32;
                (*h).queued_intra_refresh = false;
                (*(*h).fdec).f_pir_position = (*h).mb.i_mb_width as ::core::ffi::c_float;
            } else if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_P {
                let mut pocdiff = ((*(*h).fdec).i_poc - (*(*h).fref[0usize][0usize]).i_poc) / 2i32;
                let mut increment = if ((*h).mb.i_mb_width as ::core::ffi::c_float - 1f32)
                    / (*h).param.i_keyint_max as ::core::ffi::c_float
                    > 1f32
                {
                    ((*h).mb.i_mb_width as ::core::ffi::c_float - 1f32)
                        / (*h).param.i_keyint_max as ::core::ffi::c_float
                } else {
                    1f32
                };
                (*(*h).fdec).f_pir_position = (*(*h).fref[0usize][0usize]).f_pir_position;
                (*(*h).fdec).i_frames_since_pir =
                    (*(*h).fref[0usize][0usize]).i_frames_since_pir + pocdiff;
                if (*(*h).fdec).i_frames_since_pir >= (*h).param.i_keyint_max
                    || (*h).queued_intra_refresh
                        && (*(*h).fdec).f_pir_position as ::core::ffi::c_double + 0.5
                            >= (*h).mb.i_mb_width as ::core::ffi::c_double
                {
                    (*(*h).fdec).f_pir_position = 0f32;
                    (*(*h).fdec).i_frames_since_pir = 0i32;
                    (*h).queued_intra_refresh = false;
                    (*(*h).fenc).keyframe = true;
                }
                (*(*h).fdec).i_pir_start_col = ((*(*h).fdec).f_pir_position
                    as ::core::ffi::c_double
                    + 0.5) as ::core::ffi::c_int;
                (*(*h).fdec).f_pir_position += increment * pocdiff as ::core::ffi::c_float;
                (*(*h).fdec).i_pir_end_col = ((*(*h).fdec).f_pir_position as ::core::ffi::c_double
                    + 0.5) as ::core::ffi::c_int;
                if (*(*h).fdec).i_pir_end_col >= (*h).mb.i_mb_width - 1i32 {
                    (*(*h).fdec).f_pir_position = (*h).mb.i_mb_width as ::core::ffi::c_float;
                    (*(*h).fdec).i_pir_end_col = (*h).mb.i_mb_width - 1i32;
                }
            }
        }
        if (*(*h).fenc).keyframe {
            if (*h).param.repeat_headers {
                nal_start(
                    h,
                    crate::x264_h::NAL_SPS as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sps_write(&raw mut (*h).out.bs, &(*h).sps);
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                if (*h).param.i_avcintra_class != 0 {
                    (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_padding = 256i32
                        - bs_pos(&raw mut (*h).out.bs) / 8i32
                        - 2i32 * crate::src::common::common::NALU_OVERHEAD;
                }
                overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                    + (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_padding
                    + crate::src::common::common::NALU_OVERHEAD;
                nal_start(
                    h,
                    crate::x264_h::NAL_PPS as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_pps_write(
                    &raw mut (*h).out.bs,
                    &(*h).sps,
                    &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t,
                );
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                if (*h).param.i_avcintra_class != 0 {
                    let mut total_len = 256i32;
                    if (*h).param.i_avcintra_flavor == crate::x264_h::X264_AVCINTRA_FLAVOR_SONY {
                        total_len += if (*h).param.i_height >= 1080i32 {
                            18i32 * 512i32
                        } else {
                            10i32 * 512i32
                        };
                    }
                    (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_padding = total_len
                        - (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                        - crate::src::common::common::NALU_OVERHEAD;
                }
                overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                    + (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_padding
                    + crate::src::common::common::NALU_OVERHEAD;
            }
            if (*h).i_thread_frames == 1i32 && (*h).sps.vui.nal_hrd_parameters_present {
                crate::src::encoder::ratecontrol::x264_8_hrd_fullness(h);
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_buffering_period_write(
                    h,
                    &raw mut (*h).out.bs,
                );
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.annexb
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1i32 != 0)
                            as ::core::ffi::c_int);
            }
        }
        while i_1 < (*(*h).fenc).extra_sei.num_payloads {
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_write(
                &raw mut (*h).out.bs,
                (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload,
                (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload_size,
                (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload_type,
            );
            if nal_end(h) != 0 {
                return -(1i32);
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.annexb
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1i32 != 0) as ::core::ffi::c_int);
            if (*(*h).fenc).extra_sei.sei_free.is_some() {
                (*(*h).fenc)
                    .extra_sei
                    .sei_free
                    .expect("non-null function pointer")(
                    (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload
                        as *mut ::core::ffi::c_void,
                );
                let ref mut c2rust_fresh9 =
                    (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload;
                *c2rust_fresh9 = ::core::ptr::null_mut::<crate::stdlib::uint8_t>();
            }
            i_1 += 1;
        }
        if (*(*h).fenc).extra_sei.sei_free.is_some() {
            (*(*h).fenc)
                .extra_sei
                .sei_free
                .expect("non-null function pointer")(
                (*(*h).fenc).extra_sei.payloads as *mut ::core::ffi::c_void,
            );
            (*(*h).fenc).extra_sei.payloads =
                ::core::ptr::null_mut::<crate::x264_h::x264_sei_payload_t>();
            (*(*h).fenc).extra_sei.sei_free = None;
        }
        if (*(*h).fenc).keyframe {
            if (*h).param.repeat_headers
                && (*(*h).fenc).i_frame == 0i32
                && (*h).param.i_avcintra_class == 0
            {
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                if crate::src::encoder::set::x264_8_sei_version_write(h, &raw mut (*h).out.bs) != 0
                {
                    return -(1i32);
                }
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.annexb
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1i32 != 0)
                            as ::core::ffi::c_int);
            }
            if (*(*h).fenc).i_type != crate::x264_h::X264_TYPE_IDR {
                let mut time_to_recovery = if (*h).param.open_gop {
                    0i32
                } else {
                    (if (*h).mb.i_mb_width - 1i32 < (*h).param.i_keyint_max {
                        (*h).mb.i_mb_width - 1i32
                    } else {
                        (*h).param.i_keyint_max
                    }) + (*h).param.i_bframe
                        - 1i32
                };
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_recovery_point_write(
                    h,
                    &raw mut (*h).out.bs,
                    time_to_recovery,
                );
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.annexb
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1i32 != 0)
                            as ::core::ffi::c_int);
            }
            if (*h).param.mastering_display.mastering_display {
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_mastering_display_write(
                    h,
                    &raw mut (*h).out.bs,
                );
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.annexb
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1i32 != 0)
                            as ::core::ffi::c_int);
            }
            if (*h).param.content_light_level.cll {
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_content_light_level_write(
                    h,
                    &raw mut (*h).out.bs,
                );
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.annexb
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1i32 != 0)
                            as ::core::ffi::c_int);
            }
            if (*h).param.i_alternative_transfer != 2i32 {
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_alternative_transfer_write(
                    h,
                    &raw mut (*h).out.bs,
                );
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.annexb
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1i32 != 0)
                            as ::core::ffi::c_int);
            }
        }
        if (*h).param.i_frame_packing >= 0i32
            && ((*(*h).fenc).keyframe || (*h).param.i_frame_packing == 5i32)
        {
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_frame_packing_write(h, &raw mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1i32);
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.annexb
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1i32 != 0) as ::core::ffi::c_int);
        }
        if (*h).sps.vui.pic_struct_present || (*h).sps.vui.nal_hrd_parameters_present {
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_pic_timing_write(h, &raw mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1i32);
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.annexb
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1i32 != 0) as ::core::ffi::c_int);
        }
        if !((*(*h).fenc).i_type == crate::x264_h::X264_TYPE_B
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_BREF)
            && (*h).has_sh_backup
        {
            (*h).has_sh_backup = false;
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_dec_ref_pic_marking_write(h, &raw mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1i32);
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.annexb
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1i32 != 0) as ::core::ffi::c_int);
        }
        if (*(*h).fenc).keyframe && (*h).param.intra_refresh {
            (*h).i_cpb_delay_pir_offset_next = (*(*h).fenc).i_cpb_delay;
        }
        if (*h).param.i_avcintra_class != 0
            && (*h).param.i_avcintra_flavor != crate::x264_h::X264_AVCINTRA_FLAVOR_SONY
        {
            let mut unpadded_len = 0;
            let mut total_len_0 = 0;
            nal_start(
                h,
                crate::x264_h::NAL_FILLER as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_filler_write(h, &raw mut (*h).out.bs, 0i32);
            if nal_end(h) != 0 {
                return -(1i32);
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                + crate::src::common::common::NALU_OVERHEAD;
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            if crate::src::encoder::set::x264_8_sei_avcintra_umid_write(h, &raw mut (*h).out.bs)
                < 0i32
            {
                return -(1i32);
            }
            if nal_end(h) != 0 {
                return -(1i32);
            }
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.annexb
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1i32 != 0) as ::core::ffi::c_int);
            if (*h).param.i_height == 1080i32 {
                unpadded_len = 5780i32;
                total_len_0 = 17i32 * 512i32;
            } else {
                unpadded_len = 2900i32;
                total_len_0 = 9i32 * 512i32;
            }
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            if crate::src::encoder::set::x264_8_sei_avcintra_vanc_write(
                h,
                &raw mut (*h).out.bs,
                unpadded_len,
            ) < 0i32
            {
                return -(1i32);
            }
            if nal_end(h) != 0 {
                return -(1i32);
            }
            (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_padding = total_len_0
                - (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                - (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.annexb
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1i32 != 0) as ::core::ffi::c_int);
            overhead += (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_payload
                + (*(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize)).i_padding
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.annexb
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1i32 != 0) as ::core::ffi::c_int);
        }
        crate::src::encoder::ratecontrol::x264_8_ratecontrol_start(
            h,
            (*(*h).fenc).i_qpplus1,
            overhead * 8i32,
        );
        let mut i_global_qp = crate::src::encoder::ratecontrol::x264_8_ratecontrol_qp(h);
        (*(*h).fdec).i_qpplus1 = i_global_qp + 1i32;
        (*pic_out).i_qpplus1 = (*(*h).fdec).i_qpplus1;
        if (*h).param.rc.stat_read
            && (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
        {
            crate::src::encoder::ratecontrol::x264_8_reference_build_list_optimal(h);
            reference_check_reorder(h);
        }
        if (*h).i_ref[0usize] != 0 {
            (*(*h).fdec).i_poc_l0ref0 = (*(*h).fref[0usize][0usize]).i_poc;
        }
        slice_init(h, i_nal_type, i_global_qp);
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            crate::src::common::macroblock::x264_8_macroblock_bipred_init(h);
        }
        weighted_pred_init(h);
        if i_nal_ref_idc != crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int {
            (*h).i_frame_num += 1;
        }
        (*h).i_threadslice_start = 0i32;
        (*h).i_threadslice_end = (*h).mb.i_mb_height;
        if (*h).i_thread_frames > 1i32 {
            crate::src::common::threadpool::x264_8_threadpool_run(
                (*h).threadpool,
                ::core::mem::transmute::<
                    *mut ::core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
                    >,
                >(::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut x264_t) -> *mut ::core::ffi::c_void>,
                    *mut ::core::ffi::c_void,
                >(Some(
                    slices_write as unsafe extern "C" fn(*mut x264_t) -> *mut ::core::ffi::c_void,
                ))),
                h as *mut ::core::ffi::c_void,
            );
            (*h).thread_active = true;
        } else if (*h).param.sliced_threads {
            if threaded_slices_write(h) != 0 {
                return -(1i32);
            }
        } else if slices_write(h) as crate::stdlib::intptr_t != 0 {
            return -(1i32);
        }
        return encoder_frame_end(thread_oldest, thread_current, pp_nal, pi_nal, pic_out);
    }
}
unsafe extern "C" fn encoder_frame_end<'a>(
    mut h: *mut x264_t<'a>,
    mut thread_current: *mut x264_t<'a>,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_out: *mut crate::x264_h::x264_picture_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut psz_message = [0; 80];
        let mut i = 0i32;
        let mut filler = 0i32;
        let mut i_0 = 0i32;
        let mut i_1 = 0i32;
        let mut i_2 = 0i32;
        let mut i_3 = 0i32;
        let mut i_6 = 0i32;
        let mut i_9 = 0i32;
        if !(*h).param.sliced_threads && (*h).thread_active {
            (*h).thread_active = false;
            if crate::src::common::threadpool::x264_8_threadpool_wait(
                (*h).threadpool,
                h as *mut ::core::ffi::c_void,
            ) as crate::stdlib::intptr_t
                != 0
            {
                return -(1i32);
            }
        }
        if (*h).out.i_nal == 0 {
            (*pic_out).i_type = crate::x264_h::X264_TYPE_AUTO;
            return 0i32;
        }
        if (*h).i_thread_frames > 1i32
            && (*(*h).fenc).keyframe
            && (*h).sps.vui.nal_hrd_parameters_present
        {
            let mut idx = 0i32;
            crate::src::encoder::ratecontrol::x264_8_hrd_fullness(h);
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_buffering_period_write(h, &raw mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1i32);
            }
            while (*(*h).out.nal.offset(idx as isize)).i_type
                == crate::x264_h::NAL_AUD as ::core::ffi::c_int
                || (*(*h).out.nal.offset(idx as isize)).i_type
                    == crate::x264_h::NAL_SPS as ::core::ffi::c_int
                || (*(*h).out.nal.offset(idx as isize)).i_type
                    == crate::x264_h::NAL_PPS as ::core::ffi::c_int
            {
                idx += 1;
            }
            let mut nal_tmp = *(*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize);
            crate::stdlib::memmove(
                (*h).out.nal.offset((idx + 1i32) as isize) as *mut ::core::ffi::c_void,
                (*h).out.nal.offset(idx as isize) as *const ::core::ffi::c_void,
                (((*h).out.i_nal - idx - 1i32) as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::x264_h::x264_nal_t>()),
            );
            *(*h).out.nal.offset(idx as isize) = nal_tmp;
        }
        let mut frame_size = encoder_encapsulate_nals(h, 0i32);
        if frame_size < 0i32 {
            return -(1i32);
        }
        (*pic_out).i_type = (*(*h).fenc).i_type;
        (*pic_out).keyframe = (*(*h).fenc).keyframe;
        (*pic_out).i_pic_struct = (*(*h).fenc).i_pic_struct;
        (*pic_out).i_pts = (*(*h).fdec).i_pts;
        (*pic_out).i_dts = (*(*h).fdec).i_dts;
        if (*pic_out).i_pts < (*pic_out).i_dts {
            log::warn!("invalid DTS: PTS is less than DTS");
        }
        (*pic_out).opaque = (*(*h).fenc).opaque;
        (*pic_out).img.i_csp = (*(*h).fdec).i_csp;
        (*pic_out).img.i_plane = (*(*h).fdec).i_plane;
        while i < (*pic_out).img.i_plane {
            (*pic_out).img.i_stride[i as usize] =
                (*(*h).fdec).i_stride[i as usize] * crate::src::common::common::SIZEOF_PIXEL;
            (*pic_out).img.plane[i as usize] = (*(*h).fdec).plane[i as usize];
            i += 1;
        }
        crate::src::common::frame::x264_8_frame_push_unused(thread_current, (*h).fenc);
        if crate::src::encoder::ratecontrol::x264_8_ratecontrol_end(
            h,
            frame_size * 8i32,
            &raw mut filler,
        ) < 0i32
        {
            return -(1i32);
        }
        (*pic_out).hrd_timing = (*(*h).fenc).hrd_timing;
        (*pic_out).prop.f_crf_avg = (*(*h).fdec).f_crf_avg as ::core::ffi::c_double;
        if (*h).param.i_avcintra_class != 0 {
            if check_encapsulated_buffer(
                h,
                (*h).thread[0usize],
                (*h).out.i_nal,
                frame_size as crate::stdlib::int64_t,
                frame_size as crate::stdlib::int64_t + filler as crate::stdlib::int64_t,
            ) < 0i32
            {
                return -(1i32);
            }
            let mut nal = (*h).out.nal.offset(((*h).out.i_nal - 1i32) as isize);
            crate::stdlib::memset(
                (*nal).p_payload.offset((*nal).i_payload as isize) as *mut ::core::ffi::c_void,
                0i32,
                filler as crate::__stddef_size_t_h::size_t,
            );
            (*nal).i_payload += filler;
            (*nal).i_padding = filler;
            frame_size += filler;
            if !(*h).param.annexb {
                let mut nal_data = (*nal).p_payload;
                let mut chunk_size = (*nal).i_payload - 4i32;
                *nal_data.offset(0isize) = (chunk_size >> 24i32) as crate::stdlib::uint8_t;
                *nal_data.offset(1isize) = (chunk_size >> 16i32) as crate::stdlib::uint8_t;
                *nal_data.offset(2isize) = (chunk_size >> 8i32) as crate::stdlib::uint8_t;
                *nal_data.offset(3isize) = (chunk_size >> 0i32) as crate::stdlib::uint8_t;
            }
        } else {
            while filler > 0i32 {
                let mut f = 0;
                let mut overhead = crate::src::common::common::FILLER_OVERHEAD
                    - (*h).param.annexb as ::core::ffi::c_int;
                if (*h).param.i_slice_max_size != 0 && filler > (*h).param.i_slice_max_size {
                    let mut next_size = filler - (*h).param.i_slice_max_size;
                    let mut overflow = if overhead - next_size > 0i32 {
                        overhead - next_size
                    } else {
                        0i32
                    };
                    f = (*h).param.i_slice_max_size - overhead - overflow;
                } else {
                    f = if 0i32 > filler - overhead {
                        0i32
                    } else {
                        filler - overhead
                    };
                }
                if bitstream_check_buffer_filler(h, f) != 0 {
                    return -(1i32);
                }
                nal_start(
                    h,
                    crate::x264_h::NAL_FILLER as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_filler_write(h, &raw mut (*h).out.bs, f);
                if nal_end(h) != 0 {
                    return -(1i32);
                }
                let mut total_size = encoder_encapsulate_nals(h, (*h).out.i_nal - 1i32);
                if total_size < 0i32 {
                    return -(1i32);
                }
                frame_size += total_size;
                filler -= total_size;
            }
        }
        *pi_nal = (*h).out.i_nal;
        *pp_nal = (*h).out.nal;
        (*h).out.i_nal = 0i32;
        crate::src::encoder::macroblock::x264_8_noise_reduction_update(h);
        thread_sync_stat(h, (*h).thread[0usize]);
        (*h).stat.i_frame_count[(*h).sh.i_type as usize] += 1;
        (*h).stat.i_frame_size[(*h).sh.i_type as usize] += frame_size as crate::stdlib::int64_t;
        (*h).stat.f_frame_qp[(*h).sh.i_type as usize] +=
            (*(*h).fdec).f_qp_avg_aq as ::core::ffi::c_double;
        while i_0 < crate::src::common::macroblock::X264_MBTYPE_MAX as ::core::ffi::c_int {
            (*h).stat.i_mb_count[(*h).sh.i_type as usize][i_0 as usize] +=
                (*h).stat.frame.i_mb_count[i_0 as usize] as crate::stdlib::int64_t;
            i_0 += 1;
        }
        while i_1 < 2i32 {
            (*h).stat.i_mb_count_8x8dct[i_1 as usize] +=
                (*h).stat.frame.i_mb_count_8x8dct[i_1 as usize] as crate::stdlib::int64_t;
            i_1 += 1;
        }
        while i_2 < 6i32 {
            (*h).stat.i_mb_cbp[i_2 as usize] +=
                (*h).stat.frame.i_mb_cbp[i_2 as usize] as crate::stdlib::int64_t;
            i_2 += 1;
        }
        while i_3 < 4i32 {
            let mut j = 0i32;
            while j < 13i32 {
                (*h).stat.i_mb_pred_mode[i_3 as usize][j as usize] += (*h).stat.frame.i_mb_pred_mode
                    [i_3 as usize][j as usize]
                    as crate::stdlib::int64_t;
                j += 1;
            }
            i_3 += 1;
        }
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            let mut i_4 = 0i32;
            let mut i_list = 0i32;
            while i_4 < crate::src::common::macroblock::X264_PARTTYPE_MAX as ::core::ffi::c_int {
                (*h).stat.i_mb_partition[(*h).sh.i_type as usize][i_4 as usize] +=
                    (*h).stat.frame.i_mb_partition[i_4 as usize] as crate::stdlib::int64_t;
                i_4 += 1;
            }
            while i_list < 2i32 {
                let mut i_5 = 0i32;
                while i_5 < crate::src::common::base::X264_REF_MAX * 2i32 {
                    (*h).stat.i_mb_count_ref[(*h).sh.i_type as usize][i_list as usize]
                        [i_5 as usize] += (*h).stat.frame.i_mb_count_ref[i_list as usize]
                        [i_5 as usize]
                        as crate::stdlib::int64_t;
                    i_5 += 1;
                }
                i_list += 1;
            }
        }
        while i_6 < 3i32 {
            (*h).stat.i_mb_field[i_6 as usize] +=
                (*h).stat.frame.i_mb_field[i_6 as usize] as crate::stdlib::int64_t;
            i_6 += 1;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            && (*h).param.analyse.i_weighted_pred >= crate::x264_h::X264_WEIGHTP_SIMPLE
        {
            (*h).stat.i_wpred[0usize] +=
                !(*h).sh.weight[0usize][0usize].weightfn.is_null() as ::core::ffi::c_int;
            (*h).stat.i_wpred[1usize] += (!(*h).sh.weight[0usize][1usize].weightfn.is_null()
                || !(*h).sh.weight[0usize][2usize].weightfn.is_null())
                as ::core::ffi::c_int;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            (*h).stat.i_direct_frames[(*h).sh.direct_spatial_mv_pred as usize] += 1;
            if (*h).mb.direct_auto_write {
                let mut i_8 = 0i32;
                if (*h).stat.i_direct_score[0usize] + (*h).stat.i_direct_score[1usize]
                    > (*h).mb.i_mb_count
                {
                    let mut i_7 = 0i32;
                    while i_7 < 2i32 {
                        (*h).stat.i_direct_score[i_7 as usize] =
                            (*h).stat.i_direct_score[i_7 as usize] * 9i32 / 10i32;
                        i_7 += 1;
                    }
                }
                while i_8 < 2i32 {
                    (*h).stat.i_direct_score[i_8 as usize] +=
                        (*h).stat.frame.i_direct_score[i_8 as usize];
                    i_8 += 1;
                }
            }
        } else {
            (*h).stat.i_consecutive_bframes[(*(*h).fenc).i_bframes as usize] += 1;
        }
        psz_message[0usize] = '\0' as ::core::ffi::c_char;
        let mut dur = (*(*h).fenc).f_duration as ::core::ffi::c_double;
        (*h).stat.f_frame_duration[(*h).sh.i_type as usize] += dur;
        if (*h).param.analyse.psnr {
            let mut ssd = [
                (*h).stat.frame.i_ssd[0usize],
                (*h).stat.frame.i_ssd[1usize],
                (*h).stat.frame.i_ssd[2usize],
            ];
            let mut luma_size = (*h).param.i_width * (*h).param.i_height;
            let mut chroma_size = if !(*h).sps.i_chroma_format_idc.is_400() {
                luma_size
                    >> (((*h).sps.i_chroma_format_idc.is_420()
                        || (*h).sps.i_chroma_format_idc.is_422())
                        as ::core::ffi::c_int
                        + ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int)
            } else {
                0i32
            };
            (*pic_out).prop.f_psnr[0usize] = calc_psnr(
                ssd[0usize] as ::core::ffi::c_double,
                luma_size as ::core::ffi::c_double,
            );
            (*pic_out).prop.f_psnr[1usize] = calc_psnr(
                ssd[1usize] as ::core::ffi::c_double,
                chroma_size as ::core::ffi::c_double,
            );
            (*pic_out).prop.f_psnr[2usize] = calc_psnr(
                ssd[2usize] as ::core::ffi::c_double,
                chroma_size as ::core::ffi::c_double,
            );
            (*pic_out).prop.f_psnr_avg = calc_psnr(
                (ssd[0usize] + ssd[1usize] + ssd[2usize]) as ::core::ffi::c_double,
                (luma_size + chroma_size * 2i32) as ::core::ffi::c_double,
            );
            (*h).stat.f_ssd_global[(*h).sh.i_type as usize] +=
                dur * (ssd[0usize] + ssd[1usize] + ssd[2usize]) as ::core::ffi::c_double;
            (*h).stat.f_psnr_average[(*h).sh.i_type as usize] += dur * (*pic_out).prop.f_psnr_avg;
            (*h).stat.f_psnr_mean_y[(*h).sh.i_type as usize] +=
                dur * (*pic_out).prop.f_psnr[0usize];
            (*h).stat.f_psnr_mean_u[(*h).sh.i_type as usize] +=
                dur * (*pic_out).prop.f_psnr[1usize];
            (*h).stat.f_psnr_mean_v[(*h).sh.i_type as usize] +=
                dur * (*pic_out).prop.f_psnr[2usize];
            crate::stdlib::snprintf(
                &raw mut psz_message as *mut ::core::ffi::c_char,
                80usize,
                b" PSNR Y:%5.2f U:%5.2f V:%5.2f\0".as_ptr() as *const ::core::ffi::c_char,
                (*pic_out).prop.f_psnr[0usize],
                (*pic_out).prop.f_psnr[1usize],
                (*pic_out).prop.f_psnr[2usize],
            );
        }
        if (*h).param.analyse.ssim {
            (*pic_out).prop.f_ssim =
                (*h).stat.frame.f_ssim / (*h).stat.frame.i_ssim_cnt as ::core::ffi::c_double;
            (*h).stat.f_ssim_mean_y[(*h).sh.i_type as usize] += (*pic_out).prop.f_ssim * dur;
            let mut msg_len =
                crate::stdlib::strlen(&raw mut psz_message as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_int;
            crate::stdlib::snprintf(
                (&raw mut psz_message as *mut ::core::ffi::c_char).offset(msg_len as isize),
                (80i32 - msg_len) as crate::__stddef_size_t_h::size_t,
                b" SSIM Y:%.5f\0".as_ptr() as *const ::core::ffi::c_char,
                (*pic_out).prop.f_ssim,
            );
        }
        psz_message[79usize] = '\0' as ::core::ffi::c_char;
        log::debug!(
            "frame={:4} QP={:.2} NAL={} Slice:{} Poc:{:<3} I:{:<4} P:{:<4} SKIP:{:<4} size={} bytes{}",
            (*h).i_frame,
            (*(*h).fdec).f_qp_avg_aq as ::core::ffi::c_double,
            (*h).i_nal_ref_idc,
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
                'I'
            } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            {
                'P'
            } else {
                'B'
            },
            (*(*h).fdec).i_poc,
            (*h).stat.frame.i_mb_count_i,
            (*h).stat.frame.i_mb_count_p,
            (*h).stat.frame.i_mb_count_skip,
            frame_size,
            std::ffi::CStr::from_ptr(&raw const psz_message as *const ::core::ffi::c_char)
                .to_string_lossy(),
        );
        thread_sync_stat((*h).thread[0usize], h);
        thread_sync_stat(thread_current, h);
        while i_9 < (*h).i_ref[0usize] {
            if !(*h).fref[0usize][i_9 as usize].is_null()
                && (*(*h).fref[0usize][i_9 as usize]).duplicate
            {
                crate::src::common::frame::x264_8_frame_push_blank_unused(
                    h,
                    (*h).fref[0usize][i_9 as usize],
                );
                (*h).fref[0usize][i_9 as usize] =
                    ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
            }
            i_9 += 1;
        }
        if !(*h).param.psz_dump_yuv.is_null() {
            frame_dump(h);
        }
        return frame_size;
    }
}
unsafe extern "C" fn print_intra(
    mut i_mb_count: *mut crate::stdlib::int64_t,
    mut i_count: ::core::ffi::c_double,
    mut b_print_pcm: ::core::ffi::c_int,
    mut intra: *mut ::core::ffi::c_char,
) {
    unsafe {
        intra = intra.offset(crate::stdlib::sprintf(
            intra,
            b"I16..4%s: %4.1f%% %4.1f%% %4.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
            if b_print_pcm != 0 {
                b"..PCM\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            *i_mb_count
                .offset(crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / i_count,
            *i_mb_count.offset(crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / i_count,
            *i_mb_count.offset(crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / i_count,
        ) as isize);
        if b_print_pcm != 0 {
            crate::stdlib::sprintf(
                intra,
                b" %4.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                *i_mb_count
                    .offset(crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / i_count,
            );
        }
    }
}
pub unsafe extern "C" fn x264_8_encoder_close(mut h: *mut x264_t) {
    unsafe {
        let mut i_mb_count_size = [[0i64, 0, 0, 0, 0, 0, 0], [0; 7]];
        let mut buf = [0; 200];
        let mut i_0 = 0i32;
        let mut i_type = 0i32;
        let mut i_10 = 0i32;
        let mut i_yuv_size = ((*h).param.i_width * (*h).param.i_height
            + 2i32
                * (if !(*h).sps.i_chroma_format_idc.is_400() {
                    ((*h).param.i_width * (*h).param.i_height)
                        >> (((*h).sps.i_chroma_format_idc.is_420()
                            || (*h).sps.i_chroma_format_idc.is_422())
                            as ::core::ffi::c_int
                            + ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int)
                } else {
                    0i32
                })) as crate::stdlib::int64_t;
        let mut b_print_pcm = ((*h).stat.i_mb_count
            [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
            [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]
            != 0
            || (*h).stat.i_mb_count
                [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]
                != 0
            || (*h).stat.i_mb_count
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int;
        crate::src::encoder::lookahead::x264_8_lookahead_delete(h);
        if (*h).param.sliced_threads {
            threadpool_wait_all(h);
        }
        if (*h).param.i_threads > 1i32 {
            crate::src::common::threadpool::x264_8_threadpool_delete((*h).threadpool);
        }
        if (*h).param.i_lookahead_threads > 1i32 {
            crate::src::common::threadpool::x264_8_threadpool_delete((*h).lookaheadpool);
        }
        if (*h).i_thread_frames > 1i32 {
            let mut i = 0i32;
            while i < (*h).i_thread_frames {
                if (*(*h).thread[i as usize]).thread_active {
                    '_c2rust_label: {
                        if (*(*(*h).thread[i as usize]).fenc).i_reference_count == 1i32 {
                        } else {
                            crate::stdlib::__assert_fail(
                                b"h->thread[i]->fenc->i_reference_count == 1\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                                4223u32,
                                b"void x264_8_encoder_close(x264_t *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    crate::src::common::frame::x264_8_frame_delete((*(*h).thread[i as usize]).fenc);
                }
                i += 1;
            }
            let mut thread_prev = (*h).thread[(*h).i_thread_phase as usize];
            crate::src::encoder::ratecontrol::x264_8_thread_sync_ratecontrol(h, thread_prev, h);
            crate::src::encoder::ratecontrol::x264_8_thread_sync_ratecontrol(
                thread_prev,
                thread_prev,
                h,
            );
            (*h).i_frame = (*thread_prev).i_frame + 1i32 - (*h).i_thread_frames;
        }
        (*h).i_frame += 1;
        while i_0 < 3i32 {
            static mut slice_order: [crate::stdlib::uint8_t; 3] = [
                crate::src::common::base::SLICE_TYPE_I as crate::stdlib::uint8_t,
                crate::src::common::base::SLICE_TYPE_P as crate::stdlib::uint8_t,
                crate::src::common::base::SLICE_TYPE_B as crate::stdlib::uint8_t,
            ];
            let mut i_slice = slice_order[i_0 as usize] as ::core::ffi::c_int;
            if (*h).stat.i_frame_count[i_slice as usize] > 0i32 {
                let mut i_count = (*h).stat.i_frame_count[i_slice as usize];
                let mut dur = (*h).stat.f_frame_duration[i_slice as usize];
                if (*h).param.analyse.psnr {
                    log::info!(
                        "frame {}:{i_count:<5} Avg QP:{:5.2}  size:{:6.0}  PSNR Mean Y:{:5.2} U:{:5.2} V:{:5.2} Avg:{:5.2} Global:{:5.2}",
                        slice_type_to_char[i_slice as usize],
                        (*h).stat.f_frame_qp[i_slice as usize] / i_count as ::core::ffi::c_double,
                        (*h).stat.i_frame_size[i_slice as usize] as ::core::ffi::c_double
                            / i_count as ::core::ffi::c_double,
                        (*h).stat.f_psnr_mean_y[i_slice as usize] / dur,
                        (*h).stat.f_psnr_mean_u[i_slice as usize] / dur,
                        (*h).stat.f_psnr_mean_v[i_slice as usize] / dur,
                        (*h).stat.f_psnr_average[i_slice as usize] / dur,
                        calc_psnr(
                            (*h).stat.f_ssd_global[i_slice as usize],
                            dur * i_yuv_size as ::core::ffi::c_double
                        ),
                    );
                } else {
                    log::info!(
                        "frame {}:{i_count:<5} Avg QP:{:5.2}  size:{:6.0}",
                        slice_type_to_char[i_slice as usize],
                        (*h).stat.f_frame_qp[i_slice as usize] / i_count as ::core::ffi::c_double,
                        (*h).stat.i_frame_size[i_slice as usize] as ::core::ffi::c_double
                            / i_count as ::core::ffi::c_double,
                    );
                }
            }
            i_0 += 1;
        }
        if (*h).param.i_bframe != 0
            && (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                != 0
        {
            let mut den = 0i32;
            let mut i_1 = 0i32;
            let mut i_2 = 0i32;
            let mut p = &raw mut buf as *mut ::core::ffi::c_char;
            while i_1 <= (*h).param.i_bframe {
                den += (i_1 + 1i32) * (*h).stat.i_consecutive_bframes[i_1 as usize];
                i_1 += 1;
            }
            while i_2 <= (*h).param.i_bframe {
                p = p.offset(crate::stdlib::sprintf(
                    p,
                    b" %4.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                    100.0f64
                        * (i_2 + 1i32) as ::core::ffi::c_double
                        * (*h).stat.i_consecutive_bframes[i_2 as usize] as ::core::ffi::c_double
                        / den as ::core::ffi::c_double,
                ) as isize);
                i_2 += 1;
            }
            log::info!(
                "consecutive B-frames:{}",
                std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                    .to_string_lossy()
            );
        }
        while i_type < 2i32 {
            let mut i_3 = 0i32;
            while i_3 < crate::src::common::macroblock::X264_PARTTYPE_MAX as ::core::ffi::c_int {
                if !(i_3 == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int) {
                    i_mb_count_size[i_type as usize]
                        [x264_mb_partition_pixel_table[i_3 as usize] as usize] +=
                        (*h).stat.i_mb_partition[i_type as usize][i_3 as usize];
                }
                i_3 += 1;
            }
            i_type += 1;
        }
        if (*h).stat.i_frame_count
            [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
            > 0i32
        {
            let mut i_mb_count = &raw mut *(&raw mut (*h).stat.i_mb_count
                as *mut [crate::stdlib::int64_t; 19])
                .offset(crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int64_t;
            let mut i_count_0 = (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                as ::core::ffi::c_double
                * (*h).mb.i_mb_count as ::core::ffi::c_double
                / 100.0;
            print_intra(
                i_mb_count,
                i_count_0,
                b_print_pcm,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            log::info!(
                "mb I  {}",
                std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                    .to_string_lossy()
            );
        }
        if (*h).stat.i_frame_count
            [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
            > 0i32
        {
            let mut i_mb_count_0 = &raw mut *(&raw mut (*h).stat.i_mb_count
                as *mut [crate::stdlib::int64_t; 19])
                .offset(crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int64_t;
            let mut i_count_1 = (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                as ::core::ffi::c_double
                * (*h).mb.i_mb_count as ::core::ffi::c_double
                / 100.0;
            let mut i_mb_size = &raw mut *(&raw mut i_mb_count_size
                as *mut [crate::stdlib::int64_t; 7])
                .offset(crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int64_t;
            print_intra(
                i_mb_count_0,
                i_count_1,
                b_print_pcm,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            log::info!(
                "mb P  {}  P16..4: {:4.1}% {:4.1}% {:4.1}% {:4.1}% {:4.1}%    skip:{:4.1}%",
                std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                    .to_string_lossy(),
                *i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_1 * 4f64),
                (*i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as isize)
                    + *i_mb_size.offset(
                        crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as isize
                    )) as ::core::ffi::c_double
                    / (i_count_1 * 4f64),
                *i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_1 * 4f64),
                (*i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as isize)
                    + *i_mb_size.offset(
                        crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as isize
                    )) as ::core::ffi::c_double
                    / (i_count_1 * 4f64),
                *i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_1 * 4f64),
                *i_mb_count_0
                    .offset(crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / i_count_1,
            );
        }
        if (*h).stat.i_frame_count
            [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
            > 0i32
        {
            let mut list_count = [0i64, 0, 0];
            let mut i_4 = 0i32;
            let mut i_mb_count_1 = &raw mut *(&raw mut (*h).stat.i_mb_count
                as *mut [crate::stdlib::int64_t; 19])
                .offset(crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int64_t;
            let mut i_count_2 = (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                as ::core::ffi::c_double
                * (*h).mb.i_mb_count as ::core::ffi::c_double
                / 100.0;
            let mut i_mb_size_0 = &raw mut *(&raw mut i_mb_count_size
                as *mut [crate::stdlib::int64_t; 7])
                .offset(crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int64_t;
            print_intra(
                i_mb_count_1,
                i_count_2,
                b_print_pcm,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            while i_4 < crate::src::common::macroblock::X264_PARTTYPE_MAX as ::core::ffi::c_int {
                let mut j = 0i32;
                while j < 2i32 {
                    let mut l0 = x264_mb_type_list_table[i_4 as usize][0usize][j as usize]
                        as ::core::ffi::c_int;
                    let mut l1 = x264_mb_type_list_table[i_4 as usize][1usize][j as usize]
                        as ::core::ffi::c_int;
                    if l0 != 0 || l1 != 0 {
                        list_count[(l1 + l0 * l1) as usize] += (*h).stat.i_mb_count
                            [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                            [i_4 as usize]
                            * 2i64;
                    }
                    j += 1;
                }
                i_4 += 1;
            }
            list_count[0usize] += (*h).stat.i_mb_partition
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::D_L0_8x8 as ::core::ffi::c_int as usize];
            list_count[1usize] += (*h).stat.i_mb_partition
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::D_L1_8x8 as ::core::ffi::c_int as usize];
            list_count[2usize] += (*h).stat.i_mb_partition
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::D_BI_8x8 as ::core::ffi::c_int as usize];
            *i_mb_count_1
                .offset(crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int as isize) +=
                ((*h).stat.i_mb_partition
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int as usize]
                    + 2i64)
                    / 4i64;
            let mut i_mb_list_count = (list_count[0usize] + list_count[1usize] + list_count[2usize])
                as ::core::ffi::c_double
                / 100.0;
            crate::stdlib::sprintf(
                (&raw mut buf as *mut ::core::ffi::c_char).offset(crate::stdlib::strlen(
                    &raw mut buf as *mut ::core::ffi::c_char,
                ) as isize),
                b"  B16..8: %4.1f%% %4.1f%% %4.1f%%  direct:%4.1f%%  skip:%4.1f%%\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *i_mb_size_0
                    .offset(crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_2 * 4f64),
                (*i_mb_size_0
                    .offset(crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as isize)
                    + *i_mb_size_0.offset(
                        crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as isize,
                    )) as ::core::ffi::c_double
                    / (i_count_2 * 4f64),
                *i_mb_size_0
                    .offset(crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_2 * 4f64),
                *i_mb_count_1
                    .offset(crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / i_count_2,
                *i_mb_count_1
                    .offset(crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / i_count_2,
            );
            if i_mb_list_count != 0f64 {
                crate::stdlib::sprintf(
                    (&raw mut buf as *mut ::core::ffi::c_char).offset(crate::stdlib::strlen(
                        &raw mut buf as *mut ::core::ffi::c_char,
                    )
                        as isize),
                    b"  L0:%4.1f%% L1:%4.1f%% BI:%4.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                    list_count[0usize] as ::core::ffi::c_double / i_mb_list_count,
                    list_count[1usize] as ::core::ffi::c_double / i_mb_list_count,
                    list_count[2usize] as ::core::ffi::c_double / i_mb_list_count,
                );
            }
            log::info!(
                "mb B  {}",
                std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                    .to_string_lossy()
            );
        }
        crate::src::encoder::ratecontrol::x264_8_ratecontrol_summary(h);
        if (*h).stat.i_frame_count
            [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
            > 0i32
        {
            let mut fixed_pred_modes = [[0i64; 9]; 4];
            let mut sum_pred_modes = [0i64; 4];
            let mut i_5 = 0i32;
            let mut i_7 = 0i32;
            let mut i_list = 0i32;
            let mut i_i8x8 = (*h).stat.i_mb_count
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int as usize]
                + (*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int as usize]
                + (*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int as usize];
            let mut i_intra = i_i8x8
                + ((*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int as usize])
                + ((*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int as usize]);
            let mut i_all_intra = i_intra
                + ((*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]);
            let mut i_skip = (*h).stat.i_mb_count
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int as usize]
                + (*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int as usize]
                + (*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int as usize]
                + ((*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int as usize]);
            let i_count_3 = (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize];
            let mut i_mb_count_2 =
                i_count_3 as crate::stdlib::int64_t * (*h).mb.i_mb_count as crate::stdlib::int64_t;
            let mut i_inter = i_mb_count_2 - i_skip - i_all_intra;
            let duration = (*h).stat.f_frame_duration
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                + (*h).stat.f_frame_duration
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                + (*h).stat.f_frame_duration
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize];
            let mut f_bitrate = (((*h).stat.i_frame_size
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_size
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_size
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize])
                as ::core::ffi::c_double
                / duration
                / 125f64) as ::core::ffi::c_float;
            if (*h).param.interlaced {
                let mut fieldstats = &raw mut buf as *mut ::core::ffi::c_char;
                *fieldstats.offset(0isize) = 0i8;
                if i_inter != 0 {
                    fieldstats = fieldstats.offset(crate::stdlib::sprintf(
                        fieldstats,
                        b" inter:%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_mb_field[1usize] as ::core::ffi::c_double * 100.0f64
                            / i_inter as ::core::ffi::c_double,
                    ) as isize);
                }
                if i_skip != 0 {
                    fieldstats = fieldstats.offset(crate::stdlib::sprintf(
                        fieldstats,
                        b" skip:%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_mb_field[2usize] as ::core::ffi::c_double * 100.0f64
                            / i_skip as ::core::ffi::c_double,
                    ) as isize);
                }
                log::info!(
                    "field mbs: intra: {:.1}%{}",
                    (*h).stat.i_mb_field[0usize] as ::core::ffi::c_double * 100.0f64
                        / i_all_intra as ::core::ffi::c_double,
                    std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                        .to_string_lossy()
                );
            }
            if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).transform_8x8_mode
            {
                buf[0usize] = 0i8;
                if (*h).stat.i_mb_count_8x8dct[0usize] != 0 {
                    crate::stdlib::sprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b" inter:%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        100.0f64 * (*h).stat.i_mb_count_8x8dct[1usize] as ::core::ffi::c_double
                            / (*h).stat.i_mb_count_8x8dct[0usize] as ::core::ffi::c_double,
                    );
                }
                log::info!(
                    "8x8 transform intra:{:.1}%{}",
                    100.0f64 * i_i8x8 as ::core::ffi::c_double
                        / (if i_intra > 1i64 { i_intra } else { 1i64 }) as ::core::ffi::c_double,
                    std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                        .to_string_lossy()
                );
            }
            if ((*h).param.analyse.i_direct_mv_pred == crate::x264_h::X264_DIRECT_PRED_AUTO
                || (*h).stat.i_direct_frames[0usize] != 0 && (*h).stat.i_direct_frames[1usize] != 0)
                && (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                    != 0
            {
                log::info!(
                    "direct mvs  spatial:{:.1}% temporal:{:.1}%",
                    (*h).stat.i_direct_frames[1usize] as ::core::ffi::c_double * 100.0f64
                        / (*h).stat.i_frame_count
                            [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double,
                    (*h).stat.i_direct_frames[0usize] as ::core::ffi::c_double * 100.0f64
                        / (*h).stat.i_frame_count
                            [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double,
                );
            }
            buf[0usize] = 0i8;
            if !(*h).sps.i_chroma_format_idc.is_400() {
                let mut csize = if (*h).sps.i_chroma_format_idc.is_444() {
                    4i32
                } else {
                    1i32
                };
                if i_mb_count_2 != i_all_intra {
                    crate::stdlib::sprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b" inter: %.1f%% %.1f%% %.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_mb_cbp[1usize] as ::core::ffi::c_double * 100.0f64
                            / ((i_mb_count_2 - i_all_intra) * 4i64) as ::core::ffi::c_double,
                        (*h).stat.i_mb_cbp[3usize] as ::core::ffi::c_double * 100.0f64
                            / ((i_mb_count_2 - i_all_intra) * csize as crate::stdlib::int64_t)
                                as ::core::ffi::c_double,
                        (*h).stat.i_mb_cbp[5usize] as ::core::ffi::c_double * 100.0f64
                            / ((i_mb_count_2 - i_all_intra) * csize as crate::stdlib::int64_t)
                                as ::core::ffi::c_double,
                    );
                }
                log::info!(
                    "coded y,{},{} intra: {:.1}% {:.1}% {:.1}%{}",
                    if (*h).sps.i_chroma_format_idc.is_444() {
                        "u"
                    } else {
                        "uvDC"
                    },
                    if (*h).sps.i_chroma_format_idc.is_444() {
                        "v"
                    } else {
                        "uvAC"
                    },
                    (*h).stat.i_mb_cbp[0usize] as ::core::ffi::c_double * 100.0f64
                        / (i_all_intra * 4i64) as ::core::ffi::c_double,
                    (*h).stat.i_mb_cbp[2usize] as ::core::ffi::c_double * 100.0f64
                        / (i_all_intra * csize as crate::stdlib::int64_t) as ::core::ffi::c_double,
                    (*h).stat.i_mb_cbp[4usize] as ::core::ffi::c_double * 100.0f64
                        / (i_all_intra * csize as crate::stdlib::int64_t) as ::core::ffi::c_double,
                    std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                        .to_string_lossy()
                );
            } else {
                if i_mb_count_2 != i_all_intra {
                    crate::stdlib::sprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b" inter: %.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_mb_cbp[1usize] as ::core::ffi::c_double * 100.0f64
                            / ((i_mb_count_2 - i_all_intra) * 4i64) as ::core::ffi::c_double,
                    );
                }
                log::info!(
                    "coded y intra: {:.1}%{}",
                    (*h).stat.i_mb_cbp[0usize] as ::core::ffi::c_double * 100.0f64
                        / (i_all_intra * 4i64) as ::core::ffi::c_double,
                    std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                        .to_string_lossy()
                );
            }
            while i_5 <= crate::src::common::predict::I_PRED_16x16_DC_128 as ::core::ffi::c_int {
                fixed_pred_modes[0usize][x264_mb_pred_mode16x16_fix[i_5 as usize] as usize] +=
                    (*h).stat.i_mb_pred_mode[0usize][i_5 as usize];
                sum_pred_modes[0usize] += (*h).stat.i_mb_pred_mode[0usize][i_5 as usize];
                i_5 += 1;
            }
            if sum_pred_modes[0usize] != 0 {
                log::info!(
                    "i16 v,h,dc,p: {:2.0}% {:2.0}% {:2.0}% {:2.0}%",
                    fixed_pred_modes[0usize][0usize] as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[0usize] as ::core::ffi::c_double,
                    fixed_pred_modes[0usize][1usize] as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[0usize] as ::core::ffi::c_double,
                    fixed_pred_modes[0usize][2usize] as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[0usize] as ::core::ffi::c_double,
                    fixed_pred_modes[0usize][3usize] as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[0usize] as ::core::ffi::c_double,
                );
            }
            for i in 1..=2 {
                let mut j_0 = 0i32;
                while j_0 <= crate::src::common::predict::I_PRED_8x8_DC_128 as ::core::ffi::c_int {
                    fixed_pred_modes[i]
                        [x264_mb_pred_mode4x4_fix[(j_0 + 1i32) as usize] as usize] +=
                        (*h).stat.i_mb_pred_mode[i][j_0 as usize];
                    sum_pred_modes[i] += (*h).stat.i_mb_pred_mode[i][j_0 as usize];
                    j_0 += 1;
                }
                if sum_pred_modes[i as usize] != 0 {
                    log::info!(
                        "i{} v,h,dc,ddl,ddr,vr,hd,vl,hu: {:2.0}% {:2.0}% {:2.0}% {:2.0}% {:2.0}% {:2.0}% {:2.0}% {:2.0}% {:2.0}%",
                        (3 - i) * 4,
                        fixed_pred_modes[i][0] as f64 * 100.0 / sum_pred_modes[i] as f64,
                        fixed_pred_modes[i][1] as f64 * 100.0 / sum_pred_modes[i] as f64,
                        fixed_pred_modes[i][2] as f64 * 100.0 / sum_pred_modes[i] as f64,
                        fixed_pred_modes[i][3] as f64 * 100.0 / sum_pred_modes[i] as f64,
                        fixed_pred_modes[i][4] as f64 * 100.0 / sum_pred_modes[i] as f64,
                        fixed_pred_modes[i][5] as f64 * 100.0 / sum_pred_modes[i] as f64,
                        fixed_pred_modes[i][6] as f64 * 100.0 / sum_pred_modes[i] as f64,
                        fixed_pred_modes[i][7] as f64 * 100.0 / sum_pred_modes[i] as f64,
                        fixed_pred_modes[i][8] as f64 * 100.0 / sum_pred_modes[i] as f64,
                    );
                }
            }
            while i_7 <= crate::src::common::predict::I_PRED_CHROMA_DC_128 as ::core::ffi::c_int {
                fixed_pred_modes[3usize][x264_mb_chroma_pred_mode_fix[i_7 as usize] as usize] +=
                    (*h).stat.i_mb_pred_mode[3usize][i_7 as usize];
                sum_pred_modes[3usize] += (*h).stat.i_mb_pred_mode[3usize][i_7 as usize];
                i_7 += 1;
            }
            if sum_pred_modes[3usize] != 0 && !((*h).sps.i_chroma_format_idc.is_444()) {
                log::info!(
                    "i8c dc,h,v,p: {:2.0}% {:2.0}% {:2.0}% {:2.0}%",
                    fixed_pred_modes[3usize][0usize] as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[3usize] as ::core::ffi::c_double,
                    fixed_pred_modes[3usize][1usize] as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[3usize] as ::core::ffi::c_double,
                    fixed_pred_modes[3usize][2usize] as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[3usize] as ::core::ffi::c_double,
                    fixed_pred_modes[3usize][3usize] as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[3usize] as ::core::ffi::c_double,
                );
            }
            if (*h).param.analyse.i_weighted_pred >= crate::x264_h::X264_WEIGHTP_SIMPLE
                && (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    > 0i32
            {
                buf[0usize] = 0i8;
                if !(*h).sps.i_chroma_format_idc.is_400() {
                    crate::stdlib::sprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b" UV:%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_wpred[1usize] as ::core::ffi::c_double * 100.0f64
                            / (*h).stat.i_frame_count[crate::src::common::base::SLICE_TYPE_P
                                as ::core::ffi::c_int
                                as usize] as ::core::ffi::c_double,
                    );
                }
                log::info!(
                    "Weighted P-Frames: Y:{:.1}%{}",
                    (*h).stat.i_wpred[0usize] as ::core::ffi::c_double * 100.0f64
                        / (*h).stat.i_frame_count
                            [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double,
                    std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                        .to_string_lossy()
                );
            }
            while i_list < 2i32 {
                let mut i_slice_0 = 0i32;
                while i_slice_0 < 2i32 {
                    let mut i_den = 0i64;
                    let mut i_max = 0i32;
                    let mut i_8 = 0i32;
                    let mut p_0 = &raw mut buf as *mut ::core::ffi::c_char;
                    while i_8 < crate::src::common::base::X264_REF_MAX * 2i32 {
                        if (*h).stat.i_mb_count_ref[i_slice_0 as usize][i_list as usize]
                            [i_8 as usize]
                            != 0
                        {
                            i_den += (*h).stat.i_mb_count_ref[i_slice_0 as usize][i_list as usize]
                                [i_8 as usize];
                            i_max = i_8;
                        }
                        i_8 += 1;
                    }
                    if !(i_max == 0i32) {
                        let mut i_9 = 0i32;
                        while i_9 <= i_max {
                            p_0 = p_0.offset(crate::stdlib::sprintf(
                                p_0,
                                b" %4.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                                100.0f64
                                    * (*h).stat.i_mb_count_ref[i_slice_0 as usize][i_list as usize]
                                        [i_9 as usize]
                                        as ::core::ffi::c_double
                                    / i_den as ::core::ffi::c_double,
                            ) as isize);
                            i_9 += 1;
                        }
                        log::info!(
                            "ref {} L{}:{}",
                            ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"PB\0")
                                [i_slice_0 as usize] as u8 as char,
                            i_list,
                            std::ffi::CStr::from_ptr(&raw const buf as *const ::core::ffi::c_char)
                                .to_string_lossy()
                        );
                    }
                    i_slice_0 += 1;
                }
                i_list += 1;
            }
            if (*h).param.analyse.ssim {
                let mut ssim = (((*h).stat.f_ssim_mean_y
                    [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    + (*h).stat.f_ssim_mean_y
                        [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    + (*h).stat.f_ssim_mean_y
                        [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize])
                    / duration) as ::core::ffi::c_float;
                log::info!(
                    "SSIM Mean Y:{:.7} ({:6.3}db)",
                    ssim as ::core::ffi::c_double,
                    calc_ssim_db(ssim as ::core::ffi::c_double),
                );
            }
            if (*h).param.analyse.psnr {
                log::info!(
                    "PSNR Mean Y:{:6.3} U:{:6.3} V:{:6.3} Avg:{:6.3} Global:{:6.3} kb/s:{:.2}",
                    ((*h).stat.f_psnr_mean_y
                        [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                        + (*h).stat.f_psnr_mean_y[crate::src::common::base::SLICE_TYPE_P
                            as ::core::ffi::c_int
                            as usize]
                        + (*h).stat.f_psnr_mean_y[crate::src::common::base::SLICE_TYPE_B
                            as ::core::ffi::c_int
                            as usize])
                        / duration,
                    ((*h).stat.f_psnr_mean_u
                        [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                        + (*h).stat.f_psnr_mean_u[crate::src::common::base::SLICE_TYPE_P
                            as ::core::ffi::c_int
                            as usize]
                        + (*h).stat.f_psnr_mean_u[crate::src::common::base::SLICE_TYPE_B
                            as ::core::ffi::c_int
                            as usize])
                        / duration,
                    ((*h).stat.f_psnr_mean_v
                        [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                        + (*h).stat.f_psnr_mean_v[crate::src::common::base::SLICE_TYPE_P
                            as ::core::ffi::c_int
                            as usize]
                        + (*h).stat.f_psnr_mean_v[crate::src::common::base::SLICE_TYPE_B
                            as ::core::ffi::c_int
                            as usize])
                        / duration,
                    ((*h).stat.f_psnr_average
                        [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                        + (*h).stat.f_psnr_average[crate::src::common::base::SLICE_TYPE_P
                            as ::core::ffi::c_int
                            as usize]
                        + (*h).stat.f_psnr_average[crate::src::common::base::SLICE_TYPE_B
                            as ::core::ffi::c_int
                            as usize])
                        / duration,
                    calc_psnr(
                        (*h).stat.f_ssd_global
                            [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                            + (*h).stat.f_ssd_global[crate::src::common::base::SLICE_TYPE_P
                                as ::core::ffi::c_int
                                as usize]
                            + (*h).stat.f_ssd_global[crate::src::common::base::SLICE_TYPE_B
                                as ::core::ffi::c_int
                                as usize],
                        duration * i_yuv_size as ::core::ffi::c_double,
                    ),
                    f_bitrate as ::core::ffi::c_double,
                );
            } else {
                log::info!("kb/s:{f_bitrate:.2}");
            }
        }
        crate::src::encoder::ratecontrol::x264_8_ratecontrol_delete(h);
        crate::src::common::base::x264_param_cleanup(&raw mut (*h).param);
        crate::src::common::set::x264_8_cqm_delete(h);
        x264_free((*h).nal_buffer as *mut ::core::ffi::c_void);
        x264_free((*h).reconfig_h as *mut ::core::ffi::c_void);
        crate::src::encoder::analyse::x264_8_analyse_free_costs(h);
        x264_free((*h).cost_table as *mut ::core::ffi::c_void);
        if (*h).i_thread_frames > 1i32 {
            h = (*h).thread[(*h).i_thread_phase as usize];
        }
        crate::src::common::frame::x264_8_frame_delete_list((*h).frames.unused[0usize]);
        crate::src::common::frame::x264_8_frame_delete_list((*h).frames.unused[1usize]);
        crate::src::common::frame::x264_8_frame_delete_list((*h).frames.current);
        crate::src::common::frame::x264_8_frame_delete_list((*h).frames.blank_unused);
        h = (*h).thread[0usize];
        while i_10 < (*h).i_thread_frames {
            if (*(*h).thread[i_10 as usize]).thread_active {
                let mut j_1 = 0i32;
                while j_1 < (*(*h).thread[i_10 as usize]).i_ref[0usize] {
                    if !(*(*h).thread[i_10 as usize]).fref[0usize][j_1 as usize].is_null()
                        && (*(*(*h).thread[i_10 as usize]).fref[0usize][j_1 as usize]).duplicate
                    {
                        crate::src::common::frame::x264_8_frame_delete(
                            (*(*h).thread[i_10 as usize]).fref[0usize][j_1 as usize],
                        );
                    }
                    j_1 += 1;
                }
            }
            i_10 += 1;
        }
        if (*h).param.i_lookahead_threads > 1i32 {
            let mut i_11 = 0i32;
            while i_11 < (*h).param.i_lookahead_threads {
                x264_free((*h).lookahead_thread[i_11 as usize] as *mut ::core::ffi::c_void);
                i_11 += 1;
            }
        }
        let mut i_12 = (*h).param.i_threads - 1i32;
        while i_12 >= 0i32 {
            if !(*h).param.sliced_threads || i_12 == 0i32 {
                let mut frame =
                    ::core::ptr::null_mut::<*mut crate::src::common::frame::x264_frame_t>();
                frame = &raw mut (**(&raw mut (*h).thread as *mut *mut x264_t)
                    .offset(i_12 as isize))
                .frames
                .reference
                    as *mut *mut crate::src::common::frame::x264_frame_t;
                while !(*frame).is_null() {
                    '_c2rust_label_0: {
                        if (**frame).i_reference_count > 0i32 {
                        } else {
                            crate::stdlib::__assert_fail(
                                b"(*frame)->i_reference_count > 0\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                                4552u32,
                                b"void x264_8_encoder_close(x264_t *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    (**frame).i_reference_count -= 1;
                    if (**frame).i_reference_count == 0i32 {
                        crate::src::common::frame::x264_8_frame_delete(*frame);
                    }
                    frame = frame.offset(1);
                }
                frame = &raw mut (**(&raw mut (*h).thread as *mut *mut x264_t)
                    .offset(i_12 as isize))
                .fdec;
                if !(*frame).is_null() {
                    '_c2rust_label_1: {
                        if (**frame).i_reference_count > 0i32 {
                        } else {
                            crate::stdlib::__assert_fail(
                                b"(*frame)->i_reference_count > 0\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                                4560u32,
                                b"void x264_8_encoder_close(x264_t *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    (**frame).i_reference_count -= 1;
                    if (**frame).i_reference_count == 0i32 {
                        crate::src::common::frame::x264_8_frame_delete(*frame);
                    }
                }
                crate::src::common::macroblock::x264_8_macroblock_cache_free(
                    (*h).thread[i_12 as usize],
                );
            }
            crate::src::common::macroblock::x264_8_macroblock_thread_free(
                (*h).thread[i_12 as usize],
                0i32,
            );
            x264_free((*(*h).thread[i_12 as usize]).out.p_bitstream as *mut ::core::ffi::c_void);
            x264_free((*(*h).thread[i_12 as usize]).out.nal as *mut ::core::ffi::c_void);
            crate::stdlib::pthread_mutex_destroy(
                &raw mut (**(&raw mut (*h).thread as *mut *mut x264_t).offset(i_12 as isize)).mutex,
            );
            crate::stdlib::pthread_cond_destroy(
                &raw mut (**(&raw mut (*h).thread as *mut *mut x264_t).offset(i_12 as isize)).cv,
            );
            x264_free((*h).thread[i_12 as usize] as *mut ::core::ffi::c_void);
            i_12 -= 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_encoder_delayed_frames(mut h: *mut x264_t) -> ::core::ffi::c_int {
    unsafe {
        let mut delayed_frames = 0i32;
        let mut i_0 = 0i32;
        if (*h).i_thread_frames > 1i32 {
            let mut i = 0i32;
            while i < (*h).i_thread_frames {
                delayed_frames += (*(*h).thread[i as usize]).thread_active as ::core::ffi::c_int;
                i += 1;
            }
            h = (*h).thread[(*h).i_thread_phase as usize];
        }
        while !(*(*h).frames.current.offset(i_0 as isize)).is_null() {
            delayed_frames += 1;
            i_0 += 1;
        }
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ifbuf.mutex);
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).next.mutex);
        delayed_frames += (*(*h).lookahead).ifbuf.i_size
            + (*(*h).lookahead).next.i_size
            + (*(*h).lookahead).ofbuf.i_size;
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        return delayed_frames;
    }
}
pub unsafe extern "C" fn x264_8_encoder_maximum_delayed_frames(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        return (*h).frames.i_delay;
    }
}
