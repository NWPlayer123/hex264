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
                (*s).cur_bits = endian_fix32((*((*s).p as *mut crate::src::common::base::x264_union32_t)).i)
                    as crate::stdlib::uintptr_t;
                (*s).cur_bits >>= (4i32 - offset) * 8i32;
            } else {
                (*s).cur_bits = 0usize;
            };
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_pos(mut s: *mut crate::src::common::bitstream::bs_t) -> ::core::ffi::c_int {
        unsafe {
            (8i64 * (*s).p.offset_from((*s).p_start) as ::core::ffi::c_long
                + (crate::osdep_h::WORD_SIZE * 8i32) as ::core::ffi::c_long
                - (*s).i_left as ::core::ffi::c_long) as ::core::ffi::c_int
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_flush(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            (*((*s).p as *mut crate::src::common::base::x264_union32_t)).i =
                endian_fix32(((*s).cur_bits << ((*s).i_left & 31i32)) as crate::stdlib::uint32_t);
            (*s).p = (*s).p.offset((crate::osdep_h::WORD_SIZE - ((*s).i_left >> 3i32)) as isize);
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
    pub unsafe extern "C" fn bs_align_0(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            bs_write(s, (*s).i_left & 7i32, 0u32);
            bs_flush(s);
        }
    }
    pub static mut x264_ue_size_tab: [crate::stdlib::uint8_t; 256] = [
        1u8, 1u8, 3u8, 3u8, 5u8, 5u8, 5u8, 5u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 7u8, 9u8, 9u8, 9u8, 9u8,
        9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8,
        11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8,
        11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
        13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
        13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
        13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
        13u8, 13u8, 13u8, 13u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
        15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
    ];
    #[inline]
    pub unsafe extern "C" fn bs_write_ue(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut val: ::core::ffi::c_int,
    ) {
        unsafe {
            bs_write(
                s,
                x264_ue_size_tab[(val + 1i32) as usize] as ::core::ffi::c_int,
                (val + 1i32) as crate::stdlib::uint32_t,
            );
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
    pub unsafe extern "C" fn bs_write_te(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut x: ::core::ffi::c_int,
        mut val: ::core::ffi::c_int,
    ) {
        unsafe {
            if x == 1i32 {
                bs_write1(s, (1i32 ^ val) as crate::stdlib::uint32_t);
            } else {
                bs_write_ue(s, val);
            };
        }
    }
    use crate::src::encoder::cavlc::osdep_h::{endian_fix, endian_fix32};
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
        [1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8],
    ];
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
            m
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_predict_non_zero_code(
        mut h: *mut crate::src::common::common::x264_t,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            let za = (*h).mb.cache.non_zero_count
                [(x264_scan8[idx as usize] as ::core::ffi::c_int - 1i32) as usize]
                as ::core::ffi::c_int;
            let zb = (*h).mb.cache.non_zero_count
                [(x264_scan8[idx as usize] as ::core::ffi::c_int - 8i32) as usize]
                as ::core::ffi::c_int;
            let mut i_ret = za + zb;
            if i_ret < 0x80i32 {
                i_ret = (i_ret + 1i32) >> 1i32;
            }
            i_ret & 0x7Fi32
        }
    }
    pub static mut x264_transform_allowed: [crate::stdlib::uint8_t; 19] =
        [0u8, 0u8, 0u8, 0u8, 1u8, 2u8, 0u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8];
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_transform_8x8_allowed(
        mut h: *mut crate::src::common::common::x264_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            if !(*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).transform_8x8_mode {
                return 0i32;
            }
            if (*h).mb.ty != MacroblockType::P_8x8 {
                return x264_transform_allowed[(*h).mb.ty as usize] as ::core::ffi::c_int;
            }
            ((*(&raw mut (*h).mb.i_sub_partition as *mut crate::src::common::base::x264_union32_t)).i
                == (Partition::D_L0_8x8 as i32 * 0x1010101i32) as crate::stdlib::uint32_t)
                as ::core::ffi::c_int
        }
    }
    use crate::src::{
        common::macroblock::{MacroblockType, Partition},
        encoder::cavlc::{base_h::x264_scan8, predict_h::x264_mb_pred_mode4x4_fix},
    };
}
pub mod osdep_h {
    #[inline(always)]
    pub extern "C" fn endian_fix32(mut x: crate::stdlib::uint32_t) -> crate::stdlib::uint32_t {
        (x << 24i32)
            .wrapping_add(x << 8i32 & 0xFF0000u32)
            .wrapping_add(x >> 8i32 & 0xFF00u32)
            .wrapping_add(x >> 24i32)
    }
    #[inline(always)]
    pub extern "C" fn endian_fix64(mut x: crate::stdlib::uint64_t) -> crate::stdlib::uint64_t {
        (endian_fix32((x >> 32i32) as crate::stdlib::uint32_t) as crate::stdlib::uint64_t)
            .wrapping_add((endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uint64_t) << 32i32)
    }
    #[inline(always)]
    pub extern "C" fn endian_fix(mut x: crate::stdlib::uintptr_t) -> crate::stdlib::uintptr_t {
        if crate::osdep_h::WORD_SIZE == 8i32 {
            endian_fix64(x as crate::stdlib::uint64_t) as crate::stdlib::uintptr_t
        } else {
            endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uintptr_t
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_ctz_4bit(mut x: crate::stdlib::uint32_t) -> ::core::ffi::c_int {
        unsafe {
            pub static mut lut: [crate::stdlib::uint8_t; 16] =
                [4u8, 0u8, 1u8, 0u8, 2u8, 0u8, 1u8, 0u8, 3u8, 0u8, 1u8, 0u8, 2u8, 0u8, 1u8, 0u8];
            lut[x as usize] as ::core::ffi::c_int
        }
    }
}
use crate::src::{
    common::macroblock::{MacroblockType, Partition},
    encoder::cavlc::{
        base_h::x264_scan8,
        bitstream_h::{
            bs_align_0, bs_init, bs_pos, bs_write, bs_write_se, bs_write_te, bs_write_ue, bs_write1,
        },
        macroblock_h::{
            x264_mb_partition_listX_table, x264_mb_predict_intra4x4_mode, x264_mb_predict_non_zero_code,
            x264_mb_transform_8x8_allowed, x264_mb_type_list_table,
        },
        osdep_h::x264_ctz_4bit,
        predict_h::{x264_mb_chroma_pred_mode_fix, x264_mb_pred_mode4x4_fix, x264_mb_pred_mode16x16_fix},
    },
};
static mut cbp_to_golomb: [[[u8; 48]; 2]; 2] = [
    [
        [
            0, 1, 2, 5, 3, 6, 14, 10, 4, 15, 7, 11, 8, 12, 13, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            1, 10, 11, 6, 12, 7, 14, 2, 13, 15, 8, 3, 9, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    ],
    [
        [
            0, 2, 3, 7, 4, 8, 17, 13, 5, 18, 9, 14, 10, 15, 16, 11, 1, 32, 33, 36, 34, 37, 44, 40, 35, 45,
            38, 41, 39, 42, 43, 19, 6, 24, 25, 20, 26, 21, 46, 28, 27, 47, 22, 29, 23, 30, 31, 12,
        ],
        [
            3, 29, 30, 17, 31, 18, 37, 8, 32, 38, 19, 9, 20, 10, 11, 2, 16, 33, 34, 21, 35, 22, 39, 4, 36,
            40, 23, 5, 24, 6, 7, 1, 41, 42, 43, 25, 44, 26, 46, 12, 45, 47, 27, 13, 28, 14, 15, 0,
        ],
    ],
];
const fn mb_type_b_to_golomb(partition: Partition, mb_type: MacroblockType) -> u8 {
    use MacroblockType::*;

    use crate::src::common::macroblock::Partition::*;
    match (partition, mb_type) {
        (D_16x16, B_L0_L0) => 1,
        (D_16x16, B_L1_L1) => 2,
        (D_16x16, B_BI_BI) => 3,
        (D_16x8, B_L0_L0) => 4,
        (D_8x16, B_L0_L0) => 5,
        (D_16x8, B_L1_L1) => 6,
        (D_8x16, B_L1_L1) => 7,
        (D_16x8, B_L0_L1) => 8,
        (D_8x16, B_L0_L1) => 9,
        (D_16x8, B_L1_L0) => 10,
        (D_8x16, B_L1_L0) => 11,
        (D_16x8, B_L0_BI) => 12,
        (D_8x16, B_L0_BI) => 13,
        (D_16x8, B_L1_BI) => 14,
        (D_8x16, B_L1_BI) => 15,
        (D_16x8, B_BI_L0) => 16,
        (D_8x16, B_BI_L0) => 17,
        (D_16x8, B_BI_L1) => 18,
        (D_8x16, B_BI_L1) => 19,
        (D_16x8, B_BI_BI) => 20,
        (D_8x16, B_BI_BI) => 21,
        _ => unreachable!(),
    }
}
static mut subpartition_p_to_golomb: [crate::stdlib::uint8_t; 4] = [3u8, 1u8, 2u8, 0u8];
static mut subpartition_b_to_golomb: [crate::stdlib::uint8_t; 13] =
    [10u8, 4u8, 5u8, 1u8, 11u8, 6u8, 7u8, 2u8, 12u8, 8u8, 9u8, 3u8, 0u8];
#[inline]
unsafe extern "C" fn cavlc_block_residual_escape(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_suffix_length: ::core::ffi::c_int,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut s = &raw mut (*h).out.bs;
        static mut next_suffix: [crate::stdlib::uint16_t; 7] =
            [0u16, 3u16, 6u16, 12u16, 24u16, 48u16, 0xFFFFu16];
        let mut mask = level >> 31i32;
        let mut abs_level = (level ^ mask) - mask;
        let mut i_level_code = abs_level * 2i32 - mask - 2i32;
        if i_level_code >> i_suffix_length < 15i32 {
            bs_write(
                s,
                (i_level_code >> i_suffix_length) + 1i32 + i_suffix_length,
                (((1i32) << i_suffix_length) + (i_level_code & (((1i32) << i_suffix_length) - 1i32)))
                    as crate::stdlib::uint32_t,
            );
        } else {
            let mut i_level_prefix = 15i32;
            i_level_code -= (15i32) << i_suffix_length;
            if i_suffix_length == 0i32 {
                i_level_code -= 15i32;
            }
            if i_level_code >= (1i32) << 12i32 {
                if (*h).sps.i_profile_idc >= crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int {
                    while i_level_code >= (1i32) << (i_level_prefix - 3i32) {
                        i_level_code -= (1i32) << (i_level_prefix - 3i32);
                        i_level_prefix += 1;
                    }
                } else {
                    (*h).mb.overflow = true;
                }
            }
            bs_write(s, i_level_prefix + 1i32, 1u32);
            bs_write(
                s,
                i_level_prefix - 3i32,
                (i_level_code & (((1i32) << (i_level_prefix - 3i32)) - 1i32)) as crate::stdlib::uint32_t,
            );
        }
        if i_suffix_length == 0i32 {
            i_suffix_length += 1;
        }
        if abs_level > next_suffix[i_suffix_length as usize] as ::core::ffi::c_int {
            i_suffix_length += 1;
        }
        i_suffix_length
    }
}
unsafe extern "C" fn cavlc_block_residual_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut crate::src::common::common::dctcoef,
    mut nC: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut runlevel =
            crate::src::common::bitstream::x264_run_level_t { last: 0, mask: 0, level: [0; 18] };
        let mut s = &raw mut (*h).out.bs;
        static mut ctz_index: [crate::stdlib::uint8_t; 8] = [3u8, 0u8, 1u8, 0u8, 2u8, 0u8, 1u8, 0u8];
        static mut count_cat: [crate::stdlib::uint8_t; 14] =
            [16u8, 15u8, 16u8, 0u8, 15u8, 64u8, 16u8, 15u8, 16u8, 64u8, 16u8, 15u8, 16u8, 64u8];
        let mut i_total = (*h).quantf.coeff_level_run[ctx_block_cat as usize]
            .expect("non-null function pointer")(l, &raw mut runlevel);
        let mut i_total_zero = runlevel.last + 1i32 - i_total;
        runlevel.level[(i_total + 0i32) as usize] = 2i16;
        runlevel.level[(i_total + 1i32) as usize] = 2i16;
        let mut i_trailing = ((runlevel.level[0usize] as ::core::ffi::c_int + 1i32)
            | (1i32 - runlevel.level[0usize] as ::core::ffi::c_int))
            >> 31i32
            & 1i32
            | ((runlevel.level[1usize] as ::core::ffi::c_int + 1i32)
                | (1i32 - runlevel.level[1usize] as ::core::ffi::c_int))
                >> 31i32
                & 2i32
            | ((runlevel.level[2usize] as ::core::ffi::c_int + 1i32)
                | (1i32 - runlevel.level[2usize] as ::core::ffi::c_int))
                >> 31i32
                & 4i32;
        i_trailing = ctz_index[i_trailing as usize] as ::core::ffi::c_int;
        let mut i_sign = (runlevel.level[2usize] as ::core::ffi::c_int >> 31i32 & 1i32
            | runlevel.level[1usize] as ::core::ffi::c_int >> 31i32 & 2i32
            | runlevel.level[0usize] as ::core::ffi::c_int >> 31i32 & 4i32)
            as ::core::ffi::c_uint;
        i_sign >>= 3i32 - i_trailing;
        bs_write(
            s,
            crate::src::common::tables::x264_coeff_token[nC as usize][(i_total - 1i32) as usize]
                [i_trailing as usize]
                .i_size as ::core::ffi::c_int,
            crate::src::common::tables::x264_coeff_token[nC as usize][(i_total - 1i32) as usize]
                [i_trailing as usize]
                .i_bits as crate::stdlib::uint32_t,
        );
        let mut i_suffix_length = (i_total > 10i32 && i_trailing < 3i32) as ::core::ffi::c_int;
        bs_write(s, i_trailing, i_sign);
        if i_trailing < i_total {
            let mut val = runlevel.level[i_trailing as usize] as ::core::ffi::c_int;
            let mut val_original = runlevel.level[i_trailing as usize] as ::core::ffi::c_int
                + crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2i32;
            val -= (val >> 31i32 | 1i32) & -((i_trailing < 3i32) as ::core::ffi::c_int);
            val += crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2i32;
            if (val_original as ::core::ffi::c_uint)
                < crate::src::common::bitstream::LEVEL_TABLE_SIZE as ::core::ffi::c_uint
            {
                bs_write(
                    s,
                    crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize][val as usize].i_size
                        as ::core::ffi::c_int,
                    crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize][val as usize].i_bits
                        as crate::stdlib::uint32_t,
                );
                i_suffix_length = crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize]
                    [val_original as usize]
                    .i_next as ::core::ffi::c_int;
            } else {
                i_suffix_length = cavlc_block_residual_escape(
                    h,
                    i_suffix_length,
                    val - crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2i32,
                );
            }
            let mut i = i_trailing + 1i32;
            while i < i_total {
                val = runlevel.level[i as usize] as ::core::ffi::c_int
                    + crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2i32;
                if (val as ::core::ffi::c_uint)
                    < crate::src::common::bitstream::LEVEL_TABLE_SIZE as ::core::ffi::c_uint
                {
                    bs_write(
                        s,
                        crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize][val as usize]
                            .i_size as ::core::ffi::c_int,
                        crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize][val as usize]
                            .i_bits as crate::stdlib::uint32_t,
                    );
                    i_suffix_length = crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize]
                        [val as usize]
                        .i_next as ::core::ffi::c_int;
                } else {
                    i_suffix_length = cavlc_block_residual_escape(
                        h,
                        i_suffix_length,
                        val - crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2i32,
                    );
                }
                i += 1;
            }
        }
        if ctx_block_cat == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int {
            if i_total < 8i32 >> ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int {
                let mut total_zeros = if (*h).sps.i_chroma_format_idc.is_420() {
                    crate::src::common::tables::x264_total_zeros_2x2_dc[(i_total - 1i32) as usize]
                        [i_total_zero as usize]
                } else {
                    crate::src::common::tables::x264_total_zeros_2x4_dc[(i_total - 1i32) as usize]
                        [i_total_zero as usize]
                };
                bs_write(
                    s,
                    total_zeros.i_size as ::core::ffi::c_int,
                    total_zeros.i_bits as crate::stdlib::uint32_t,
                );
            }
        } else if (i_total as crate::stdlib::uint8_t as ::core::ffi::c_int)
            < count_cat[ctx_block_cat as usize] as ::core::ffi::c_int
        {
            bs_write(
                s,
                crate::src::common::tables::x264_total_zeros[(i_total - 1i32) as usize][i_total_zero as usize]
                    .i_size as ::core::ffi::c_int,
                crate::src::common::tables::x264_total_zeros[(i_total - 1i32) as usize][i_total_zero as usize]
                    .i_bits as crate::stdlib::uint32_t,
            );
        }
        let mut zero_run_code =
            crate::src::common::vlc::x264_8_run_before[runlevel.mask as usize] as ::core::ffi::c_int;
        bs_write(s, zero_run_code & 0x1Fi32, (zero_run_code >> 5i32) as crate::stdlib::uint32_t);
        i_total
    }
}
static mut ct_index: [crate::stdlib::uint8_t; 17] =
    [0u8, 0u8, 1u8, 1u8, 2u8, 2u8, 2u8, 2u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8];
unsafe extern "C" fn cavlc_qp_delta(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut s = &raw mut (*h).out.bs;
        let mut i_dqp = (*h).mb.i_qp - (*h).mb.i_last_qp;
        if (*h).mb.ty == MacroblockType::I_16x16
            && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
            && (*h).mb.cache.non_zero_count[x264_scan8[crate::src::common::base::LUMA_DC as usize] as usize]
                == 0
            && (*h).mb.cache.non_zero_count
                [x264_scan8[(crate::src::common::base::CHROMA_DC + 0i32) as usize] as usize]
                == 0
            && (*h).mb.cache.non_zero_count
                [x264_scan8[(crate::src::common::base::CHROMA_DC + 1i32) as usize] as usize]
                == 0
            && (*h).mb.i_qp > (*h).mb.i_last_qp
        {
            (*h).mb.i_qp = (*h).mb.i_last_qp;
            i_dqp = 0i32;
        }
        if i_dqp != 0 {
            if i_dqp < -(crate::src::common::common::QP_MAX_SPEC + 1i32) / 2i32 {
                i_dqp += crate::src::common::common::QP_MAX_SPEC + 1i32;
            } else if i_dqp > crate::src::common::common::QP_MAX_SPEC / 2i32 {
                i_dqp -= crate::src::common::common::QP_MAX_SPEC + 1i32;
            }
        }
        bs_write_se(s, i_dqp);
    }
}
unsafe extern "C" fn cavlc_mvd(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) {
    unsafe {
        let mut mvp = [0; 2];
        let mut s = &raw mut (*h).out.bs;
        crate::src::common::mvpred::x264_8_mb_predict_mv(
            h,
            i_list,
            idx,
            width,
            &raw mut mvp as *mut crate::stdlib::int16_t,
        );
        bs_write_se(
            s,
            (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize][0usize]
                as ::core::ffi::c_int
                - mvp[0usize],
        );
        bs_write_se(
            s,
            (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize][1usize]
                as ::core::ffi::c_int
                - mvp[1usize],
        );
    }
}
#[inline]
unsafe extern "C" fn cavlc_8x8_mvd(
    mut h: *mut crate::src::common::common::x264_t,
    mut i: ::core::ffi::c_int,
) {
    unsafe {
        match (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int {
            3 => {
                cavlc_mvd(h, 0i32, 4i32 * i, 2i32);
            }
            1 => {
                cavlc_mvd(h, 0i32, 4i32 * i + 0i32, 2i32);
                cavlc_mvd(h, 0i32, 4i32 * i + 2i32, 2i32);
            }
            2 => {
                cavlc_mvd(h, 0i32, 4i32 * i + 0i32, 1i32);
                cavlc_mvd(h, 0i32, 4i32 * i + 1i32, 1i32);
            }
            0 => {
                cavlc_mvd(h, 0i32, 4i32 * i + 0i32, 1i32);
                cavlc_mvd(h, 0i32, 4i32 * i + 1i32, 1i32);
                cavlc_mvd(h, 0i32, 4i32 * i + 2i32, 1i32);
                cavlc_mvd(h, 0i32, 4i32 * i + 3i32, 1i32);
            }
            _ => {}
        };
    }
}
#[inline(always)]
unsafe extern "C" fn cavlc_macroblock_luma_residual(
    mut h: *mut crate::src::common::common::x264_t,
    mut plane_count: ::core::ffi::c_int,
) {
    unsafe {
        let mut p_0 = 0i32;
        if (*h).mb.transform_8x8 {
            let mut p = 0i32;
            while p < plane_count {
                let mut i8 = 0i32;
                while i8 < 4i32 {
                    if (*h).mb.cache.non_zero_count[x264_scan8[(p * 16i32 + i8 * 4i32) as usize] as usize]
                        != 0
                    {
                        (*h).zigzagf.interleave_8x8_cavlc.expect("non-null function pointer")(
                            &raw mut *(&raw mut (*h).dct.luma4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset((p * 16i32 + i8 * 4i32) as isize)
                                as *mut crate::src::common::common::dctcoef,
                            &raw mut *(&raw mut (*h).dct.luma8x8
                                as *mut [crate::src::common::common::dctcoef; 64])
                                .offset((p * 4i32 + i8) as isize)
                                as *mut crate::src::common::common::dctcoef,
                            (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset((p * 16i32 + i8 * 4i32) as isize)
                                    as isize,
                            ),
                        );
                    }
                    i8 += 1;
                }
                p += 1;
            }
        }
        while p_0 < plane_count {
            let mut i8_0 = 0i32;
            let mut msk = (*h).mb.i_cbp_luma;
            while msk != 0 && {
                let mut skip = 0;
                skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                i8_0 += skip;
                msk >>= skip + 1i32;
                1i32 != 0
            } {
                let mut i4 = 0i32;
                while i4 < 4i32 {
                    let mut nC = if crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int
                        == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                    {
                        5i32 - ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int
                    } else {
                        ct_index[x264_mb_predict_non_zero_code(
                            h,
                            if crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int
                                == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                            {
                                (i4 + i8_0 * 4i32 + p_0 * 16i32 - crate::src::common::base::LUMA_DC) * 16i32
                            } else {
                                i4 + i8_0 * 4i32 + p_0 * 16i32
                            },
                        ) as usize] as ::core::ffi::c_int
                    };
                    let mut nnz = (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset((i4 + i8_0 * 4i32 + p_0 * 16i32) as isize)
                                as isize,
                        );
                    if *nnz == 0 {
                        bs_write(
                            &raw mut (*h).out.bs,
                            crate::src::common::tables::x264_coeff0_token[nC as usize].i_size
                                as ::core::ffi::c_int,
                            crate::src::common::tables::x264_coeff0_token[nC as usize].i_bits
                                as crate::stdlib::uint32_t,
                        );
                    } else {
                        *nnz = cavlc_block_residual_internal(
                            h,
                            crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.luma4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset((i4 + i8_0 * 4i32 + p_0 * 16i32) as isize)
                                as *mut crate::src::common::common::dctcoef,
                            nC,
                        ) as crate::stdlib::uint8_t;
                    }
                    i4 += 1;
                }
                i8_0 += 1;
            }
            p_0 += 1;
        }
    }
}
unsafe extern "C" fn cavlc_mb_header_i(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_mb_type: MacroblockType,
    mut i_mb_i_offset: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut s = &raw mut (*h).out.bs;
        if i_mb_type == MacroblockType::I_16x16 {
            bs_write_ue(
                s,
                i_mb_i_offset
                    + 1i32
                    + x264_mb_pred_mode16x16_fix[(*h).mb.i_intra16x16_pred_mode as usize]
                        as ::core::ffi::c_int
                    + (*h).mb.i_cbp_chroma * 4i32
                    + (if (*h).mb.i_cbp_luma == 0i32 { 0i32 } else { 12i32 }),
            );
        } else {
            let mut i = 0i32;
            let mut di = if i_mb_type == MacroblockType::I_8x8 { 4i32 } else { 1i32 };
            bs_write_ue(s, i_mb_i_offset + 0i32);
            if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).transform_8x8_mode {
                bs_write1(s, (*h).mb.transform_8x8 as crate::stdlib::uint32_t);
            }
            while i < 16i32 {
                let mut i_pred = x264_mb_predict_intra4x4_mode(h, i);
                let mut i_mode = x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                    [x264_scan8[i as usize] as usize]
                    as ::core::ffi::c_int
                    + 1i32) as usize] as ::core::ffi::c_int;
                if i_pred == i_mode {
                    bs_write1(s, 1u32);
                } else {
                    bs_write(
                        s,
                        4i32,
                        (i_mode - (i_mode > i_pred) as ::core::ffi::c_int) as crate::stdlib::uint32_t,
                    );
                }
                i += di;
            }
        }
        if chroma != 0 {
            bs_write_ue(
                s,
                x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as ::core::ffi::c_int,
            );
        }
    }
}
#[inline(always)]
unsafe extern "C" fn cavlc_mb_header_p(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_mb_type: MacroblockType,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut s = &raw mut (*h).out.bs;
        if i_mb_type == MacroblockType::P_L0 {
            if (*h).mb.i_partition == Partition::D_16x16 {
                bs_write1(s, 1u32);
                if (*h).mb.pic.i_fref[0usize] > 1i32 {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0usize] - 1i32,
                        (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int,
                    );
                }
                cavlc_mvd(h, 0i32, 0i32, 4i32);
            } else if (*h).mb.i_partition == Partition::D_16x8 {
                bs_write_ue(s, 1i32);
                if (*h).mb.pic.i_fref[0usize] > 1i32 {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0usize] - 1i32,
                        (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int,
                    );
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0usize] - 1i32,
                        (*h).mb.cache.ref_0[0usize][x264_scan8[8usize] as usize] as ::core::ffi::c_int,
                    );
                }
                cavlc_mvd(h, 0i32, 0i32, 4i32);
                cavlc_mvd(h, 0i32, 8i32, 4i32);
            } else if (*h).mb.i_partition == Partition::D_8x16 {
                bs_write_ue(s, 2i32);
                if (*h).mb.pic.i_fref[0usize] > 1i32 {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0usize] - 1i32,
                        (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int,
                    );
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0usize] - 1i32,
                        (*h).mb.cache.ref_0[0usize][x264_scan8[4usize] as usize] as ::core::ffi::c_int,
                    );
                }
                cavlc_mvd(h, 0i32, 0i32, 2i32);
                cavlc_mvd(h, 0i32, 4i32, 2i32);
            }
        } else if i_mb_type == MacroblockType::P_8x8 {
            let mut b_sub_ref = 0;
            let mut i_0 = 0i32;
            if (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int
                | (*h).mb.cache.ref_0[0usize][x264_scan8[4usize] as usize] as ::core::ffi::c_int
                | (*h).mb.cache.ref_0[0usize][x264_scan8[8usize] as usize] as ::core::ffi::c_int
                | (*h).mb.cache.ref_0[0usize][x264_scan8[12usize] as usize] as ::core::ffi::c_int
                == 0i32
            {
                bs_write_ue(s, 4i32);
                b_sub_ref = 0i32;
            } else {
                bs_write_ue(s, 3i32);
                b_sub_ref = 1i32;
            }
            if (*h).param.analyse.inter & crate::x264_h::X264_ANALYSE_PSUB8x8 != 0 {
                let mut i = 0i32;
                while i < 4i32 {
                    bs_write_ue(
                        s,
                        subpartition_p_to_golomb[(*h).mb.i_sub_partition[i as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                    i += 1;
                }
            } else {
                bs_write(s, 4i32, 0xFu32);
            }
            if b_sub_ref != 0 {
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0usize] - 1i32,
                    (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0usize] - 1i32,
                    (*h).mb.cache.ref_0[0usize][x264_scan8[4usize] as usize] as ::core::ffi::c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0usize] - 1i32,
                    (*h).mb.cache.ref_0[0usize][x264_scan8[8usize] as usize] as ::core::ffi::c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0usize] - 1i32,
                    (*h).mb.cache.ref_0[0usize][x264_scan8[12usize] as usize] as ::core::ffi::c_int,
                );
            }
            while i_0 < 4i32 {
                cavlc_8x8_mvd(h, i_0);
                i_0 += 1;
            }
        } else {
            cavlc_mb_header_i(h, i_mb_type, 5i32, chroma);
        };
    }
}
#[inline(always)]
unsafe extern "C" fn cavlc_mb_header_b(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_mb_type: MacroblockType,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut s = &raw mut (*h).out.bs;
        if i_mb_type == MacroblockType::B_8x8 {
            let mut i = 0i32;
            let mut i_2 = 0i32;
            let mut i_3 = 0i32;
            bs_write_ue(s, 22i32);
            while i < 4i32 {
                bs_write_ue(
                    s,
                    subpartition_b_to_golomb[(*h).mb.i_sub_partition[i as usize] as usize]
                        as ::core::ffi::c_int,
                );
                i += 1;
            }
            if (*h).mb.pic.i_fref[0usize] > 1i32 {
                let mut i_0 = 0i32;
                while i_0 < 4i32 {
                    if x264_mb_partition_listX_table[0usize][(*h).mb.i_sub_partition[i_0 as usize] as usize]
                        != 0
                    {
                        bs_write_te(
                            s,
                            (*h).mb.pic.i_fref[0usize] - 1i32,
                            (*h).mb.cache.ref_0[0usize][x264_scan8[(i_0 * 4i32) as usize] as usize]
                                as ::core::ffi::c_int,
                        );
                    }
                    i_0 += 1;
                }
            }
            if (*h).mb.pic.i_fref[1usize] > 1i32 {
                let mut i_1 = 0i32;
                while i_1 < 4i32 {
                    if x264_mb_partition_listX_table[1usize][(*h).mb.i_sub_partition[i_1 as usize] as usize]
                        != 0
                    {
                        bs_write_te(
                            s,
                            (*h).mb.pic.i_fref[1usize] - 1i32,
                            (*h).mb.cache.ref_0[1usize][x264_scan8[(i_1 * 4i32) as usize] as usize]
                                as ::core::ffi::c_int,
                        );
                    }
                    i_1 += 1;
                }
            }
            while i_2 < 4i32 {
                if x264_mb_partition_listX_table[0usize][(*h).mb.i_sub_partition[i_2 as usize] as usize] != 0
                {
                    cavlc_mvd(h, 0i32, 4i32 * i_2, 2i32);
                }
                i_2 += 1;
            }
            while i_3 < 4i32 {
                if x264_mb_partition_listX_table[1usize][(*h).mb.i_sub_partition[i_3 as usize] as usize] != 0
                {
                    cavlc_mvd(h, 1i32, 4i32 * i_3, 2i32);
                }
                i_3 += 1;
            }
        } else if i_mb_type >= MacroblockType::B_L0_L0 && i_mb_type <= MacroblockType::B_BI_BI {
            let mut b_list =
                &raw const *(&raw const x264_mb_type_list_table as *const [[crate::stdlib::uint8_t; 2]; 2])
                    .offset(i_mb_type as isize) as *const [crate::stdlib::uint8_t; 2];
            let i_ref0_max = (*h).mb.pic.i_fref[0usize] - 1i32;
            let i_ref1_max = (*h).mb.pic.i_fref[1usize] - 1i32;
            bs_write_ue(s, mb_type_b_to_golomb((*h).mb.i_partition, i_mb_type) as ::core::ffi::c_int);
            if (*h).mb.i_partition == Partition::D_16x16 {
                if i_ref0_max != 0 && (*b_list.offset(0isize))[0usize] as ::core::ffi::c_int != 0 {
                    bs_write_te(
                        s,
                        i_ref0_max,
                        (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int,
                    );
                }
                if i_ref1_max != 0 && (*b_list.offset(1isize))[0usize] as ::core::ffi::c_int != 0 {
                    bs_write_te(
                        s,
                        i_ref1_max,
                        (*h).mb.cache.ref_0[1usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int,
                    );
                }
                if (*b_list.offset(0isize))[0usize] != 0 {
                    cavlc_mvd(h, 0i32, 0i32, 4i32);
                }
                if (*b_list.offset(1isize))[0usize] != 0 {
                    cavlc_mvd(h, 1i32, 0i32, 4i32);
                }
            } else {
                if i_ref0_max != 0 && (*b_list.offset(0isize))[0usize] as ::core::ffi::c_int != 0 {
                    bs_write_te(
                        s,
                        i_ref0_max,
                        (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int,
                    );
                }
                if i_ref0_max != 0 && (*b_list.offset(0isize))[1usize] as ::core::ffi::c_int != 0 {
                    bs_write_te(
                        s,
                        i_ref0_max,
                        (*h).mb.cache.ref_0[0usize][x264_scan8[12usize] as usize] as ::core::ffi::c_int,
                    );
                }
                if i_ref1_max != 0 && (*b_list.offset(1isize))[0usize] as ::core::ffi::c_int != 0 {
                    bs_write_te(
                        s,
                        i_ref1_max,
                        (*h).mb.cache.ref_0[1usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int,
                    );
                }
                if i_ref1_max != 0 && (*b_list.offset(1isize))[1usize] as ::core::ffi::c_int != 0 {
                    bs_write_te(
                        s,
                        i_ref1_max,
                        (*h).mb.cache.ref_0[1usize][x264_scan8[12usize] as usize] as ::core::ffi::c_int,
                    );
                }
                if (*h).mb.i_partition == Partition::D_16x8 {
                    if (*b_list.offset(0isize))[0usize] != 0 {
                        cavlc_mvd(h, 0i32, 0i32, 4i32);
                    }
                    if (*b_list.offset(0isize))[1usize] != 0 {
                        cavlc_mvd(h, 0i32, 8i32, 4i32);
                    }
                    if (*b_list.offset(1isize))[0usize] != 0 {
                        cavlc_mvd(h, 1i32, 0i32, 4i32);
                    }
                    if (*b_list.offset(1isize))[1usize] != 0 {
                        cavlc_mvd(h, 1i32, 8i32, 4i32);
                    }
                } else {
                    if (*b_list.offset(0isize))[0usize] != 0 {
                        cavlc_mvd(h, 0i32, 0i32, 2i32);
                    }
                    if (*b_list.offset(0isize))[1usize] != 0 {
                        cavlc_mvd(h, 0i32, 4i32, 2i32);
                    }
                    if (*b_list.offset(1isize))[0usize] != 0 {
                        cavlc_mvd(h, 1i32, 0i32, 2i32);
                    }
                    if (*b_list.offset(1isize))[1usize] != 0 {
                        cavlc_mvd(h, 1i32, 4i32, 2i32);
                    }
                }
            }
        } else if i_mb_type == MacroblockType::B_DIRECT {
            bs_write1(s, 1u32);
        } else {
            cavlc_mb_header_i(h, i_mb_type, 23i32, chroma);
        };
    }
}
pub unsafe extern "C" fn x264_8_macroblock_write_cavlc(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut i_mb_pos_tex = 0;
        let mut s = &raw mut (*h).out.bs;
        let i_mb_type = (*h).mb.ty;
        let mut plane_count = if (*h).sps.i_chroma_format_idc.is_444() { 3i32 } else { 1i32 };
        let mut chroma = ((*h).sps.i_chroma_format_idc.is_420() || (*h).sps.i_chroma_format_idc.is_422())
            as ::core::ffi::c_int;
        let i_mb_pos_start = bs_pos(s);
        if (*h).sh.mbaff
            && ((*h).mb.i_mb_y & 1i32 == 0
                || matches!(
                    *(*h).mb.types.add(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride - (*h).sh.i_first_mb) as usize),
                    MacroblockType::P_SKIP | MacroblockType::B_SKIP
                ))
        {
            bs_write1(s, (*h).mb.interlaced as crate::stdlib::uint32_t);
            (*h).mb.field_decoding_flag = (*h).mb.interlaced as ::core::ffi::c_int;
        }
        if i_mb_type == MacroblockType::I_PCM {
            let mut p = 0i32;
            static mut i_offsets: [crate::stdlib::uint8_t; 3] = [5u8, 23u8, 0u8];
            let mut p_start = (*s).p_start;
            bs_write_ue(s, i_offsets[(*h).sh.i_type as usize] as ::core::ffi::c_int + 25i32);
            i_mb_pos_tex = bs_pos(s);
            (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
            bs_align_0(s);
            while p < plane_count {
                let mut i = 0i32;
                while i < 256i32 {
                    bs_write(
                        s,
                        crate::internal::BIT_DEPTH,
                        *(*h).mb.pic.p_fenc[p as usize].offset(i as isize) as crate::stdlib::uint32_t,
                    );
                    i += 1;
                }
                p += 1;
            }
            if chroma != 0 {
                let mut ch = 1i32;
                while ch < 3i32 {
                    let mut i_0 = 0i32;
                    while i_0 < 16i32 >> ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int {
                        let mut j = 0i32;
                        while j < 8i32 {
                            bs_write(
                                s,
                                crate::internal::BIT_DEPTH,
                                *(*h).mb.pic.p_fenc[ch as usize]
                                    .offset((i_0 * crate::src::common::common::FENC_STRIDE + j) as isize)
                                    as crate::stdlib::uint32_t,
                            );
                            j += 1;
                        }
                        i_0 += 1;
                    }
                    ch += 1;
                }
            }
            bs_init(
                s,
                (*s).p as *mut ::core::ffi::c_void,
                (*s).p_end.offset_from((*s).p) as ::core::ffi::c_int,
            );
            (*s).p_start = p_start;
            (*h).stat.frame.i_tex_bits += bs_pos(s) - i_mb_pos_tex;
            return;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            cavlc_mb_header_p(h, i_mb_type, chroma);
        } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            cavlc_mb_header_b(h, i_mb_type, chroma);
        } else {
            cavlc_mb_header_i(h, i_mb_type, 0i32, chroma);
        }
        i_mb_pos_tex = bs_pos(s);
        (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
        if i_mb_type != MacroblockType::I_16x16 {
            bs_write_ue(
                s,
                cbp_to_golomb[chroma as usize][(i_mb_type == MacroblockType::I_4x4
                    || i_mb_type == MacroblockType::I_8x8
                    || i_mb_type == MacroblockType::I_16x16
                    || i_mb_type == MacroblockType::I_PCM)
                    as ::core::ffi::c_int as usize]
                    [((*h).mb.i_cbp_chroma << 4i32 | (*h).mb.i_cbp_luma) as usize]
                    as ::core::ffi::c_int,
            );
        }
        if x264_mb_transform_8x8_allowed(h) != 0 && (*h).mb.i_cbp_luma != 0 {
            bs_write1(s, (*h).mb.transform_8x8 as crate::stdlib::uint32_t);
        }
        if i_mb_type == MacroblockType::I_16x16 {
            let mut p_0 = 0i32;
            cavlc_qp_delta(h);
            while p_0 < plane_count {
                let mut nC = if crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                    == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                {
                    5i32 - ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int
                } else {
                    ct_index[x264_mb_predict_non_zero_code(
                        h,
                        if crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                            == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                        {
                            (48i32 + p_0 - crate::src::common::base::LUMA_DC) * 16i32
                        } else {
                            48i32 + p_0
                        },
                    ) as usize] as ::core::ffi::c_int
                };
                let mut nnz = (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset((48i32 + p_0) as isize)
                        as isize,
                );
                if *nnz == 0 {
                    bs_write(
                        &raw mut (*h).out.bs,
                        crate::src::common::tables::x264_coeff0_token[nC as usize].i_size
                            as ::core::ffi::c_int,
                        crate::src::common::tables::x264_coeff0_token[nC as usize].i_bits
                            as crate::stdlib::uint32_t,
                    );
                } else {
                    *nnz = cavlc_block_residual_internal(
                        h,
                        crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int,
                        &raw mut *(&raw mut (*h).dct.luma16x16_dc
                            as *mut [crate::src::common::common::dctcoef; 16])
                            .offset(p_0 as isize)
                            as *mut crate::src::common::common::dctcoef,
                        nC,
                    ) as crate::stdlib::uint8_t;
                }
                if (*h).mb.i_cbp_luma != 0 {
                    let mut i_1 = p_0 * 16i32;
                    while i_1 < p_0 * 16i32 + 16i32 {
                        let mut nC_0 = if crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int
                            == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                        {
                            5i32 - ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int
                        } else {
                            ct_index[x264_mb_predict_non_zero_code(
                                h,
                                if crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int
                                    == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                                {
                                    (i_1 - crate::src::common::base::LUMA_DC) * 16i32
                                } else {
                                    i_1
                                },
                            ) as usize] as ::core::ffi::c_int
                        };
                        let mut nnz_0 =
                            (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(i_1 as isize)
                                    as isize,
                            );
                        if *nnz_0 == 0 {
                            bs_write(
                                &raw mut (*h).out.bs,
                                crate::src::common::tables::x264_coeff0_token[nC_0 as usize].i_size
                                    as ::core::ffi::c_int,
                                crate::src::common::tables::x264_coeff0_token[nC_0 as usize].i_bits
                                    as crate::stdlib::uint32_t,
                            );
                        } else {
                            *nnz_0 = cavlc_block_residual_internal(
                                h,
                                crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int,
                                (&raw mut *(&raw mut (*h).dct.luma4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(i_1 as isize)
                                    as *mut crate::src::common::common::dctcoef)
                                    .offset(1isize),
                                nC_0,
                            ) as crate::stdlib::uint8_t;
                        }
                        i_1 += 1;
                    }
                }
                p_0 += 1;
            }
        } else if (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma != 0 {
            cavlc_qp_delta(h);
            cavlc_macroblock_luma_residual(h, plane_count);
        }
        if (*h).mb.i_cbp_chroma != 0 {
            let mut nC_1 = if crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
            {
                5i32 - ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int
            } else {
                ct_index[x264_mb_predict_non_zero_code(
                    h,
                    if crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                        == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                    {
                        (49i32 + 0i32 - crate::src::common::base::LUMA_DC) * 16i32
                    } else {
                        49i32 + 0i32
                    },
                ) as usize] as ::core::ffi::c_int
            };
            let mut nnz_1 = (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset((49i32 + 0i32) as isize)
                    as isize,
            );
            if *nnz_1 == 0 {
                bs_write(
                    &raw mut (*h).out.bs,
                    crate::src::common::tables::x264_coeff0_token[nC_1 as usize].i_size as ::core::ffi::c_int,
                    crate::src::common::tables::x264_coeff0_token[nC_1 as usize].i_bits
                        as crate::stdlib::uint32_t,
                );
            } else {
                *nnz_1 = cavlc_block_residual_internal(
                    h,
                    crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                    &raw mut *(&raw mut (*h).dct.chroma_dc as *mut [crate::src::common::common::dctcoef; 8])
                        .offset(0isize) as *mut crate::src::common::common::dctcoef,
                    nC_1,
                ) as crate::stdlib::uint8_t;
            }
            let mut nC_2 = if crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
            {
                5i32 - ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int
            } else {
                ct_index[x264_mb_predict_non_zero_code(
                    h,
                    if crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                        == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                    {
                        (49i32 + 1i32 - crate::src::common::base::LUMA_DC) * 16i32
                    } else {
                        49i32 + 1i32
                    },
                ) as usize] as ::core::ffi::c_int
            };
            let mut nnz_2 = (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset((49i32 + 1i32) as isize)
                    as isize,
            );
            if *nnz_2 == 0 {
                bs_write(
                    &raw mut (*h).out.bs,
                    crate::src::common::tables::x264_coeff0_token[nC_2 as usize].i_size as ::core::ffi::c_int,
                    crate::src::common::tables::x264_coeff0_token[nC_2 as usize].i_bits
                        as crate::stdlib::uint32_t,
                );
            } else {
                *nnz_2 = cavlc_block_residual_internal(
                    h,
                    crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                    &raw mut *(&raw mut (*h).dct.chroma_dc as *mut [crate::src::common::common::dctcoef; 8])
                        .offset(1isize) as *mut crate::src::common::common::dctcoef,
                    nC_2,
                ) as crate::stdlib::uint8_t;
            }
            if (*h).mb.i_cbp_chroma == 2i32 {
                let mut i_2 = 16i32;
                let mut step = (8i32) << ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
                while i_2 < 3i32 * 16i32 {
                    let mut j_0 = i_2;
                    while j_0 < i_2 + 4i32 {
                        let mut nC_3 = if crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int
                            == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                        {
                            5i32 - ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int
                        } else {
                            ct_index[x264_mb_predict_non_zero_code(
                                h,
                                if crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int
                                    == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                                {
                                    (j_0 - crate::src::common::base::LUMA_DC) * 16i32
                                } else {
                                    j_0
                                },
                            ) as usize] as ::core::ffi::c_int
                        };
                        let mut nnz_3 =
                            (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(j_0 as isize)
                                    as isize,
                            );
                        if *nnz_3 == 0 {
                            bs_write(
                                &raw mut (*h).out.bs,
                                crate::src::common::tables::x264_coeff0_token[nC_3 as usize].i_size
                                    as ::core::ffi::c_int,
                                crate::src::common::tables::x264_coeff0_token[nC_3 as usize].i_bits
                                    as crate::stdlib::uint32_t,
                            );
                        } else {
                            *nnz_3 = cavlc_block_residual_internal(
                                h,
                                crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int,
                                (&raw mut *(&raw mut (*h).dct.luma4x4
                                    as *mut [crate::src::common::common::dctcoef; 16])
                                    .offset(j_0 as isize)
                                    as *mut crate::src::common::common::dctcoef)
                                    .offset(1isize),
                                nC_3,
                            ) as crate::stdlib::uint8_t;
                        }
                        j_0 += 1;
                    }
                    i_2 += step;
                }
            }
        }
        (*h).stat.frame.i_tex_bits += bs_pos(s) - i_mb_pos_tex;
    }
}
