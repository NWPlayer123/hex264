pub mod bitstream_h {
    #[inline]
    pub unsafe extern "C" fn bs_init(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut p_data: *mut ::core::ffi::c_void,
        mut i_data: ::core::ffi::c_int,
    ) {
        unsafe {
            let mut offset: ::core::ffi::c_int = (p_data as crate::stdlib::intptr_t
                & 3 as ::core::ffi::c_int as crate::stdlib::intptr_t)
                as ::core::ffi::c_int;
            (*s).p_start = (p_data as *mut crate::stdlib::uint8_t).offset(-(offset as isize));
            (*s).p = (*s).p_start;
            (*s).p_end = (p_data as *mut crate::stdlib::uint8_t).offset(i_data as isize);
            (*s).i_left = (crate::osdep_h::WORD_SIZE - offset) * 8 as ::core::ffi::c_int;
            if offset != 0 {
                (*s).cur_bits =
                    endian_fix32((*((*s).p as *mut crate::src::common::base::x264_union32_t)).i)
                        as crate::stdlib::uintptr_t;
                (*s).cur_bits >>= (4 as ::core::ffi::c_int - offset) * 8 as ::core::ffi::c_int;
            } else {
                (*s).cur_bits = 0 as crate::stdlib::uintptr_t;
            };
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_flush(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            (*((*s).p as *mut crate::src::common::base::x264_union32_t)).i = endian_fix32(
                ((*s).cur_bits << ((*s).i_left & 31 as ::core::ffi::c_int))
                    as crate::stdlib::uint32_t,
            );
            (*s).p = (*s).p.offset(
                (crate::osdep_h::WORD_SIZE - ((*s).i_left >> 3 as ::core::ffi::c_int)) as isize,
            );
            (*s).i_left = crate::osdep_h::WORD_SIZE * 8 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_write(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut i_count: ::core::ffi::c_int,
        mut i_bits: crate::stdlib::uint32_t,
    ) {
        unsafe {
            if crate::osdep_h::WORD_SIZE == 8 as ::core::ffi::c_int {
                (*s).cur_bits = (*s).cur_bits << i_count | i_bits as crate::stdlib::uintptr_t;
                (*s).i_left -= i_count;
                if (*s).i_left <= 32 as ::core::ffi::c_int {
                    (*((*s).p as *mut crate::src::common::base::x264_union32_t)).i =
                        endian_fix((*s).cur_bits << (*s).i_left) as crate::stdlib::uint32_t;
                    (*s).i_left += 32 as ::core::ffi::c_int;
                    (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
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
                (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
                (*s).cur_bits = i_bits as crate::stdlib::uintptr_t;
                (*s).i_left = 32 as ::core::ffi::c_int - i_count;
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
                * 8 as ::core::ffi::c_long
                + (*cb).i_queue as ::core::ffi::c_long) as ::core::ffi::c_int;
        }
    }
}
pub mod predict_h {
    pub static mut x264_mb_chroma_pred_mode_fix: [crate::stdlib::uint8_t; 7] = [
        crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_H as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_V as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_P as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
    ];
    pub static mut x264_mb_pred_mode16x16_fix: [crate::stdlib::uint8_t; 7] = [
        crate::src::common::predict::I_PRED_16x16_V as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_H as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_P as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_16x16_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
    ];
    pub static mut x264_mb_pred_mode4x4_fix: [crate::stdlib::int8_t; 13] = [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_V as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_H as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DDL as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DDR as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_VR as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_HD as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_VL as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_HU as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int as crate::stdlib::int8_t,
        crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int as crate::stdlib::int8_t,
    ];
}
pub mod base_h {
    pub static mut x264_scan8: [crate::stdlib::uint8_t; 51] = [
        (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (0 as ::core::ffi::c_int + 10 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
    ];
    #[inline(always)]
    pub unsafe extern "C" fn x264_cabac_mvd_sum(
        mut mvdleft: *mut crate::stdlib::uint8_t,
        mut mvdtop: *mut crate::stdlib::uint8_t,
    ) -> crate::stdlib::uint16_t {
        unsafe {
            let mut amvd0: ::core::ffi::c_int = *mvdleft.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                + *mvdtop.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            let mut amvd1: ::core::ffi::c_int = *mvdleft.offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                + *mvdtop.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            amvd0 = (amvd0 > 2 as ::core::ffi::c_int) as ::core::ffi::c_int
                + (amvd0 > 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
            amvd1 = (amvd1 > 2 as ::core::ffi::c_int) as ::core::ffi::c_int
                + (amvd1 > 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
            return (amvd0 + (amvd1 << 8 as ::core::ffi::c_int)) as crate::stdlib::uint16_t;
        }
    }
}
pub mod macroblock_h {
    pub static mut x264_mb_type_list_table: [[[crate::stdlib::uint8_t; 2]; 2]; 19] = [
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ],
        ],
    ];
    pub static mut x264_mb_partition_listX_table: [[crate::stdlib::uint8_t; 17]; 2] = [
        [
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    ];
    pub static mut block_idx_x: [crate::stdlib::uint8_t; 16] = [
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ];
    pub static mut block_idx_y: [crate::stdlib::uint8_t; 16] = [
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ];
    pub static mut ctx_cat_plane: [[crate::stdlib::uint8_t; 3]; 6] = [
        [
            crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAU_DC as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAV_DC as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
        ],
        [
            crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAU_AC as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAV_AC as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
        ],
        [
            crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAU_4x4 as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAV_4x4 as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
        ],
        [0 as ::core::ffi::c_int as crate::stdlib::uint8_t, 0, 0],
        [0 as ::core::ffi::c_int as crate::stdlib::uint8_t, 0, 0],
        [
            crate::src::common::macroblock::DCT_LUMA_8x8 as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAU_8x8 as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
            crate::src::common::macroblock::DCT_CHROMAV_8x8 as ::core::ffi::c_int
                as crate::stdlib::uint8_t,
        ],
    ];
    #[inline(always)]
    pub extern "C" fn pack8to16(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
        return a.wrapping_add(b << 8 as ::core::ffi::c_int);
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_predict_intra4x4_mode(
        mut h: *mut crate::src::common::common::x264_t,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            let ma: ::core::ffi::c_int = (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[idx as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int;
            let mb: ::core::ffi::c_int = (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[idx as usize]
                as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int;
            let m: ::core::ffi::c_int = if (x264_mb_pred_mode4x4_fix
                [(ma + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int)
                < x264_mb_pred_mode4x4_fix[(mb + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
            {
                x264_mb_pred_mode4x4_fix[(ma + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
            } else {
                x264_mb_pred_mode4x4_fix[(mb + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
            };
            if m < 0 as ::core::ffi::c_int {
                return crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int;
            }
            return m;
        }
    }
    pub static mut x264_transform_allowed: [crate::stdlib::uint8_t; 19] = [
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ];
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_transform_8x8_allowed(
        mut h: *mut crate::src::common::common::x264_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                .b_transform_8x8_mode
                == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*h).mb.i_type != crate::src::common::macroblock::P_8x8 as ::core::ffi::c_int {
                return x264_transform_allowed[(*h).mb.i_type as usize] as ::core::ffi::c_int;
            }
            return ((*(&raw mut (*h).mb.i_sub_partition as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i
                == (crate::src::common::macroblock::D_L0_8x8 as ::core::ffi::c_int
                    * 0x1010101 as ::core::ffi::c_int)
                    as crate::stdlib::uint32_t) as ::core::ffi::c_int;
        }
    }
    use crate::src::encoder::cabac::base_h::x264_scan8;
    use crate::src::encoder::cabac::predict_h::x264_mb_pred_mode4x4_fix;
}
pub mod osdep_h {
    #[inline(always)]
    pub extern "C" fn endian_fix32(mut x: crate::stdlib::uint32_t) -> crate::stdlib::uint32_t {
        return (x << 24 as ::core::ffi::c_int)
            .wrapping_add(x << 8 as ::core::ffi::c_int & 0xff0000 as crate::stdlib::uint32_t)
            .wrapping_add(x >> 8 as ::core::ffi::c_int & 0xff00 as crate::stdlib::uint32_t)
            .wrapping_add(x >> 24 as ::core::ffi::c_int);
    }
    #[inline(always)]
    pub extern "C" fn endian_fix64(mut x: crate::stdlib::uint64_t) -> crate::stdlib::uint64_t {
        return (endian_fix32((x >> 32 as ::core::ffi::c_int) as crate::stdlib::uint32_t)
            as crate::stdlib::uint64_t)
            .wrapping_add(
                (endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uint64_t)
                    << 32 as ::core::ffi::c_int,
            );
    }
    #[inline(always)]
    pub extern "C" fn endian_fix(mut x: crate::stdlib::uintptr_t) -> crate::stdlib::uintptr_t {
        return if crate::osdep_h::WORD_SIZE == 8 as ::core::ffi::c_int {
            endian_fix64(x as crate::stdlib::uint64_t) as crate::stdlib::uintptr_t
        } else {
            endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uintptr_t
        };
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_ctz_4bit(mut x: crate::stdlib::uint32_t) -> ::core::ffi::c_int {
        unsafe {
            pub static mut lut: [crate::stdlib::uint8_t; 16] = [
                4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
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
            let mut d: *mut crate::stdlib::uint8_t = dst as *mut crate::stdlib::uint8_t;
            let mut v2: crate::stdlib::uint16_t = (if s >= 2 as ::core::ffi::c_int {
                v
            } else {
                v.wrapping_mul(0x101 as crate::stdlib::uint32_t)
            }) as crate::stdlib::uint16_t;
            let mut v4: crate::stdlib::uint32_t = if s >= 4 as ::core::ffi::c_int {
                v
            } else if s >= 2 as ::core::ffi::c_int {
                v.wrapping_mul(0x10001 as crate::stdlib::uint32_t)
            } else {
                v.wrapping_mul(0x1010101 as crate::stdlib::uint32_t)
            };
            let mut v8: crate::stdlib::uint64_t = (v4 as crate::stdlib::uint64_t)
                .wrapping_add((v4 as crate::stdlib::uint64_t) << 32 as ::core::ffi::c_int);
            s *= 8 as ::core::ffi::c_int;
            if w == 2 as ::core::ffi::c_int {
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                if h == 1 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                if h == 2 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
            } else if w == 4 as ::core::ffi::c_int {
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                if h == 1 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                if h == 2 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
            } else if w == 8 as ::core::ffi::c_int {
                if crate::osdep_h::WORD_SIZE == 8 as ::core::ffi::c_int {
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    if h == 1 as ::core::ffi::c_int {
                        return;
                    }
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    if h == 2 as ::core::ffi::c_int {
                        return;
                    }
                    (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                } else {
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    if h == 1 as ::core::ffi::c_int {
                        return;
                    }
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    if h == 2 as ::core::ffi::c_int {
                        return;
                    }
                    (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                }
            } else if w == 16 as ::core::ffi::c_int {
                '_c2rust_label: {
                    if h != 1 as ::core::ffi::c_int {
                    } else {
                        crate::stdlib::__assert_fail(
                            b"h != 1\0".as_ptr() as *const ::core::ffi::c_char,
                            b"./common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
                            82 as ::core::ffi::c_uint,
                            b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                if crate::osdep_h::WORD_SIZE == 8 as ::core::ffi::c_int {
                    loop {
                        (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        h -= 2 as ::core::ffi::c_int;
                        d = d.offset((s * 2 as ::core::ffi::c_int) as isize);
                        if !(h != 0) {
                            break;
                        }
                    }
                } else {
                    loop {
                        (*(d.offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = v4;
                        (*(d.offset(4 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = v4;
                        (*(d.offset(8 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = v4;
                        (*(d.offset(12 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = v4;
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
                        118 as ::core::ffi::c_uint,
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
            let mut mvd_cache: *mut ::core::ffi::c_void =
                (&raw mut *(&raw mut (*h).mb.cache.mvd as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(i_list as isize) as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        (crate::src::common::base::X264_SCAN8_0 + x + 8 as ::core::ffi::c_int * y)
                            as isize,
                    ) as *mut [crate::stdlib::uint8_t; 2]
                    as *mut ::core::ffi::c_void;
            if 0 == 0 || 0 == 0 {
                crate::src::common::rectangle::x264_8_cache_mvd_func_table[(width
                    + (height << 1 as ::core::ffi::c_int)
                    - 3 as ::core::ffi::c_int)
                    as usize]
                    .expect("non-null function pointer")(
                    mvd_cache, mvd as crate::stdlib::uint32_t
                );
            } else {
                x264_macroblock_cache_rect(
                    mvd_cache,
                    width * 2 as ::core::ffi::c_int,
                    height,
                    2 as ::core::ffi::c_int,
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
use crate::src::encoder::cabac::predict_h::x264_mb_pred_mode16x16_fix;
use crate::src::encoder::cabac::predict_h::x264_mb_pred_mode4x4_fix;
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
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                ctx0,
                0 as ::core::ffi::c_int,
            );
        } else if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                ctx0,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_flush(
                h as *mut crate::src::common::common::x264_t,
                cb as *mut crate::src::common::cabac::x264_cabac_t,
            );
        } else {
            let mut i_pred: ::core::ffi::c_int = x264_mb_pred_mode16x16_fix
                [(*h).mb.i_intra16x16_pred_mode as usize]
                as ::core::ffi::c_int;
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                ctx0,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_terminal_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                ctx1,
                ((*h).mb.i_cbp_luma != 0) as ::core::ffi::c_int,
            );
            if (*h).mb.i_cbp_chroma == 0 as ::core::ffi::c_int {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    ctx2,
                    0 as ::core::ffi::c_int,
                );
            } else {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    ctx2,
                    1 as ::core::ffi::c_int,
                );
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    ctx3,
                    (*h).mb.i_cbp_chroma >> 1 as ::core::ffi::c_int,
                );
            }
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                ctx4,
                i_pred >> 1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                ctx5,
                i_pred & 1 as ::core::ffi::c_int,
            );
        };
    }
}
unsafe extern "C" fn cabac_field_decoding_flag(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ctx += (*h).mb.field_decoding_flag & ((*h).mb.i_mb_x != 0) as ::core::ffi::c_int;
        ctx += ((*h).mb.i_mb_top_mbpair_xy >= 0 as ::core::ffi::c_int
            && *(*h)
                .mb
                .slice_table
                .offset((*h).mb.i_mb_top_mbpair_xy as isize)
                == (*h).sh.i_first_mb as crate::stdlib::int32_t
            && *(*h).mb.field.offset((*h).mb.i_mb_top_mbpair_xy as isize) as ::core::ffi::c_int
                != 0) as ::core::ffi::c_int;
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            70 as ::core::ffi::c_int + ctx,
            (*h).mb.b_interlaced,
        );
        (*h).mb.field_decoding_flag = (*h).mb.b_interlaced;
    }
}
unsafe extern "C" fn cabac_intra4x4_pred_mode(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_pred: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
) {
    unsafe {
        if i_pred == i_mode {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                68 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        } else {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                68 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if i_mode > i_pred {
                i_mode -= 1;
            }
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                69 as ::core::ffi::c_int,
                i_mode & 0x1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                69 as ::core::ffi::c_int,
                i_mode >> 1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                69 as ::core::ffi::c_int,
                i_mode >> 2 as ::core::ffi::c_int,
            );
        };
    }
}
unsafe extern "C" fn cabac_intra_chroma_pred_mode(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut i_mode: ::core::ffi::c_int =
            x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as ::core::ffi::c_int;
        let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*h).mb.i_neighbour
            & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && *(*h)
                .mb
                .chroma_pred_mode
                .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            ctx += 1;
        }
        if (*h).mb.i_neighbour
            & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && *(*h)
                .mb
                .chroma_pred_mode
                .offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            ctx += 1;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            64 as ::core::ffi::c_int + ctx,
            (i_mode > 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        if i_mode > 0 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                64 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                (i_mode > 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
            if i_mode > 1 as ::core::ffi::c_int {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    64 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                    (i_mode > 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
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
        let mut cbp: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
        let mut cbp_l: ::core::ffi::c_int = (*h).mb.cache.i_cbp_left;
        let mut cbp_t: ::core::ffi::c_int = (*h).mb.cache.i_cbp_top;
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            76 as ::core::ffi::c_int
                - (cbp_l >> 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
                - (cbp_t >> 1 as ::core::ffi::c_int & 2 as ::core::ffi::c_int),
            cbp >> 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
        );
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            76 as ::core::ffi::c_int
                - (cbp >> 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
                - (cbp_t >> 2 as ::core::ffi::c_int & 2 as ::core::ffi::c_int),
            cbp >> 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
        );
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            76 as ::core::ffi::c_int
                - (cbp_l >> 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
                - (cbp << 1 as ::core::ffi::c_int & 2 as ::core::ffi::c_int),
            cbp >> 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
        );
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            76 as ::core::ffi::c_int
                - (cbp >> 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
                - (cbp >> 0 as ::core::ffi::c_int & 2 as ::core::ffi::c_int),
            cbp >> 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn cabac_cbp_chroma(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut cbp_a: ::core::ffi::c_int = (*h).mb.cache.i_cbp_left & 0x30 as ::core::ffi::c_int;
        let mut cbp_b: ::core::ffi::c_int = (*h).mb.cache.i_cbp_top & 0x30 as ::core::ffi::c_int;
        let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if cbp_a != 0 && (*h).mb.cache.i_cbp_left != -(1 as ::core::ffi::c_int) {
            ctx += 1;
        }
        if cbp_b != 0 && (*h).mb.cache.i_cbp_top != -(1 as ::core::ffi::c_int) {
            ctx += 2 as ::core::ffi::c_int;
        }
        if (*h).mb.i_cbp_chroma == 0 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                77 as ::core::ffi::c_int + ctx,
                0 as ::core::ffi::c_int,
            );
        } else {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                77 as ::core::ffi::c_int + ctx,
                1 as ::core::ffi::c_int,
            );
            ctx = 4 as ::core::ffi::c_int;
            if cbp_a == 0x20 as ::core::ffi::c_int {
                ctx += 1;
            }
            if cbp_b == 0x20 as ::core::ffi::c_int {
                ctx += 2 as ::core::ffi::c_int;
            }
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                77 as ::core::ffi::c_int + ctx,
                (*h).mb.i_cbp_chroma >> 1 as ::core::ffi::c_int,
            );
        };
    }
}
unsafe extern "C" fn cabac_qp_delta(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        let mut i_dqp: ::core::ffi::c_int = (*h).mb.i_qp - (*h).mb.i_last_qp;
        if (*h).mb.i_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
            && *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) == 0
            && (*h).mb.i_qp > (*h).mb.i_last_qp
        {
            (*h).mb.i_qp = (*h).mb.i_last_qp;
            i_dqp = 0 as ::core::ffi::c_int;
        }
        let mut ctx: ::core::ffi::c_int = ((*h).mb.i_last_dqp != 0
            && (*(*h).mb.type_0.offset((*h).mb.i_mb_prev_xy as isize) as ::core::ffi::c_int
                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || *(*h).mb.cbp.offset((*h).mb.i_mb_prev_xy as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int
                    != 0)) as ::core::ffi::c_int;
        if i_dqp != 0 as ::core::ffi::c_int {
            i_dqp *= 2 as ::core::ffi::c_int;
            let mut val: ::core::ffi::c_int = 1 as ::core::ffi::c_int - i_dqp;
            if val < 0 as ::core::ffi::c_int {
                val = i_dqp;
            }
            val -= 1;
            if val >= crate::src::common::common::QP_MAX_SPEC
                && val != crate::src::common::common::QP_MAX_SPEC + 1 as ::core::ffi::c_int
            {
                val = 2 as ::core::ffi::c_int * crate::src::common::common::QP_MAX_SPEC
                    + 1 as ::core::ffi::c_int
                    - val;
            }
            loop {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    60 as ::core::ffi::c_int + ctx,
                    1 as ::core::ffi::c_int,
                );
                ctx = 2 as ::core::ffi::c_int + (ctx >> 1 as ::core::ffi::c_int);
                val -= 1;
                if !(val != 0) {
                    break;
                }
            }
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            60 as ::core::ffi::c_int + ctx,
            0 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_mb_skip(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_skip: ::core::ffi::c_int,
) {
    unsafe {
        let mut ctx: ::core::ffi::c_int = (*h).mb.cache.i_neighbour_skip + 11 as ::core::ffi::c_int;
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            ctx += 13 as ::core::ffi::c_int;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            &raw mut (*h).cabac as *mut _ as *mut crate::src::common::cabac::x264_cabac_t,
            ctx,
            b_skip,
        );
    }
}
#[inline]
unsafe extern "C" fn cabac_subpartition_p(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_sub: ::core::ffi::c_int,
) {
    unsafe {
        if i_sub == crate::src::common::macroblock::D_L0_8x8 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                21 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            return;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            21 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if i_sub == crate::src::common::macroblock::D_L0_8x4 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                22 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                22 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                23 as ::core::ffi::c_int,
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
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                36 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            return;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            36 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        if i_sub == crate::src::common::macroblock::D_BI_8x8 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                37 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                38 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                39 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                39 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            return;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            37 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            39 as ::core::ffi::c_int,
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
        let mut ctx: ::core::ffi::c_int =
            399 as ::core::ffi::c_int + (*h).mb.cache.i_neighbour_transform_size;
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            ctx,
            (*h).mb.b_transform_8x8,
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
        let i8: ::core::ffi::c_int = x264_scan8[idx as usize] as ::core::ffi::c_int;
        let i_refa: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
            [(i8 - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        let i_refb: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
            [(i8 - 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if i_refa > 0 as ::core::ffi::c_int
            && (bframe == 0 || (*h).mb.cache.skip[(i8 - 1 as ::core::ffi::c_int) as usize] == 0)
        {
            ctx += 1;
        }
        if i_refb > 0 as ::core::ffi::c_int
            && (bframe == 0 || (*h).mb.cache.skip[(i8 - 8 as ::core::ffi::c_int) as usize] == 0)
        {
            ctx += 2 as ::core::ffi::c_int;
        }
        let mut i_ref: ::core::ffi::c_int =
            (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as ::core::ffi::c_int;
        while i_ref > 0 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                54 as ::core::ffi::c_int + ctx,
                1 as ::core::ffi::c_int,
            );
            ctx = (ctx >> 2 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int;
            i_ref -= 1;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            54 as ::core::ffi::c_int + ctx,
            0 as ::core::ffi::c_int,
        );
    }
}
#[inline(never)]
unsafe extern "C" fn cabac_ref_p(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut idx: ::core::ffi::c_int,
) {
    unsafe {
        cabac_ref_internal(h, cb, 0 as ::core::ffi::c_int, idx, 0 as ::core::ffi::c_int);
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
        cabac_ref_internal(h, cb, i_list, idx, 1 as ::core::ffi::c_int);
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
        let mut ctxbase: ::core::ffi::c_int = if l != 0 {
            47 as ::core::ffi::c_int
        } else {
            40 as ::core::ffi::c_int
        };
        if mvd == 0 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                ctxbase + ctx,
                0 as ::core::ffi::c_int,
            );
            return 0 as ::core::ffi::c_int;
        }
        let mut i_abs: ::core::ffi::c_int = crate::stdlib::abs(mvd);
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            ctxbase + ctx,
            1 as ::core::ffi::c_int,
        );
        static mut ctxes: [crate::stdlib::uint8_t; 8] = [
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ];
        if i_abs < 9 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i < i_abs {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    ctxbase + ctxes[(i - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                i += 1;
            }
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                ctxbase + ctxes[(i_abs - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else {
            let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i_0 < 9 as ::core::ffi::c_int {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    ctxbase + ctxes[(i_0 - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                i_0 += 1;
            }
            crate::src::common::cabac::x264_8_cabac_encode_ue_bypass(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                3 as ::core::ffi::c_int,
                i_abs - 9 as ::core::ffi::c_int,
            );
        }
        crate::src::common::cabac::x264_8_cabac_encode_bypass_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            mvd >> 31 as ::core::ffi::c_int,
        );
        return if i_abs < 66 as ::core::ffi::c_int {
            i_abs
        } else {
            66 as ::core::ffi::c_int
        };
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
        let mut mvp: [crate::stdlib::int16_t; 2] = [0; 2];
        crate::src::common::mvpred::x264_8_mb_predict_mv(
            h as *mut crate::src::common::common::x264_t,
            i_list,
            idx,
            width,
            &raw mut mvp as *mut crate::stdlib::int16_t,
        );
        let mut mdx: ::core::ffi::c_int = (*h).mb.cache.mv[i_list as usize]
            [x264_scan8[idx as usize] as usize][0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let mut mdy: ::core::ffi::c_int = (*h).mb.cache.mv[i_list as usize]
            [x264_scan8[idx as usize] as usize][1 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let mut amvd: crate::stdlib::uint16_t = x264_cabac_mvd_sum(
            &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                as *mut [[crate::stdlib::uint8_t; 2]; 40])
                .offset(i_list as isize) as *mut [crate::stdlib::uint8_t; 2])
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(idx as isize)
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as isize,
                ) as *mut crate::stdlib::uint8_t,
            &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                as *mut [[crate::stdlib::uint8_t; 2]; 40])
                .offset(i_list as isize) as *mut [crate::stdlib::uint8_t; 2])
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(idx as isize)
                        as ::core::ffi::c_int
                        - 8 as ::core::ffi::c_int) as isize,
                ) as *mut crate::stdlib::uint8_t,
        );
        mdx = cabac_mvd_cpn(
            h,
            cb,
            i_list,
            idx,
            0 as ::core::ffi::c_int,
            mdx,
            amvd as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
        );
        mdy = cabac_mvd_cpn(
            h,
            cb,
            i_list,
            idx,
            1 as ::core::ffi::c_int,
            mdy,
            amvd as ::core::ffi::c_int >> 8 as ::core::ffi::c_int,
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
                let mut mvd: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i,
                    2 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i) as usize] as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd,
                );
            }
            1 => {
                let mut mvd_0: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_0,
                );
                let mut mvd_1: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_1,
                );
            }
            2 => {
                let mut mvd_2: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_2,
                );
                let mut mvd_3: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_3,
                );
            }
            0 => {
                let mut mvd_4: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_4,
                );
                let mut mvd_5: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_5,
                );
                let mut mvd_6: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_6,
                );
                let mut mvd_7: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_7,
                );
            }
            _ => {
                '_c2rust_label: {
                    crate::stdlib::__assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"encoder/cabac.c\0".as_ptr() as *const ::core::ffi::c_char,
                        377 as ::core::ffi::c_uint,
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
            let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
                && (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                    != crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            {
                ctx += 1;
            }
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
                && (*h).mb.i_mb_type_top
                    != crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            {
                ctx += 1;
            }
            cabac_mb_type_intra(
                h,
                cb,
                i_mb_type,
                3 as ::core::ffi::c_int + ctx,
                3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int,
                3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int,
            );
        } else if slice_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                14 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            cabac_mb_type_intra(
                h,
                cb,
                i_mb_type,
                17 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                17 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                17 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                17 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                17 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                17 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            );
        } else if slice_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            cabac_mb_type_intra(
                h,
                cb,
                i_mb_type,
                32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            );
        }
        if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            return;
        }
        if i_mb_type != crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int {
            if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                .b_transform_8x8_mode
                != 0
            {
                cabac_transform_size(h, cb);
            }
            let mut di: ::core::ffi::c_int = if (*h).mb.b_transform_8x8 != 0 {
                4 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let i_pred: ::core::ffi::c_int =
                    x264_mb_predict_intra4x4_mode(h, i) as ::core::ffi::c_int;
                let i_mode: ::core::ffi::c_int =
                    x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                        [x264_scan8[i as usize] as usize]
                        as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
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
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                14 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if (*h).mb.i_partition == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    15 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    16 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                    cabac_ref_p(h, cb, 0 as ::core::ffi::c_int);
                }
                let mut mvd: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd,
                );
            } else if (*h).mb.i_partition
                == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
            {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    15 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    17 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                    cabac_ref_p(h, cb, 0 as ::core::ffi::c_int);
                    cabac_ref_p(h, cb, 8 as ::core::ffi::c_int);
                }
                let mut mvd_0: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_0,
                );
                let mut mvd_1: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    block_idx_y[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_1,
                );
            } else {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    15 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    17 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                    cabac_ref_p(h, cb, 0 as ::core::ffi::c_int);
                    cabac_ref_p(h, cb, 4 as ::core::ffi::c_int);
                }
                let mut mvd_2: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_2,
                );
                let mut mvd_3: crate::stdlib::uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    block_idx_y[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd_3,
                );
            }
        } else if i_mb_type == crate::src::common::macroblock::P_8x8 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                14 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                15 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                16 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                cabac_subpartition_p(
                    cb,
                    (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int,
                );
                i += 1;
            }
            if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                cabac_ref_p(h, cb, 0 as ::core::ffi::c_int);
                cabac_ref_p(h, cb, 4 as ::core::ffi::c_int);
                cabac_ref_p(h, cb, 8 as ::core::ffi::c_int);
                cabac_ref_p(h, cb, 12 as ::core::ffi::c_int);
            }
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
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
        let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*h).mb.i_neighbour
            & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                != crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
            && (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                != crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int
        {
            ctx += 1;
        }
        if (*h).mb.i_neighbour
            & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && (*h).mb.i_mb_type_top != crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
            && (*h).mb.i_mb_type_top
                != crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int
        {
            ctx += 1;
        }
        if i_mb_type == crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + ctx,
                0 as ::core::ffi::c_int,
            );
            return;
        }
        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
            cb as *mut crate::src::common::cabac::x264_cabac_t,
            27 as ::core::ffi::c_int + ctx,
            1 as ::core::ffi::c_int,
        );
        if i_mb_type == crate::src::common::macroblock::B_8x8 as ::core::ffi::c_int {
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                cabac_subpartition_b(
                    cb,
                    (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int,
                );
                i += 1;
            }
            if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < 4 as ::core::ffi::c_int {
                    if x264_mb_partition_listX_table[0 as ::core::ffi::c_int as usize]
                        [(*h).mb.i_sub_partition[i_0 as usize] as usize]
                        != 0
                    {
                        cabac_ref_b(
                            h,
                            cb,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int * i_0,
                        );
                    }
                    i_0 += 1;
                }
            }
            if (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_1 < 4 as ::core::ffi::c_int {
                    if x264_mb_partition_listX_table[1 as ::core::ffi::c_int as usize]
                        [(*h).mb.i_sub_partition[i_1 as usize] as usize]
                        != 0
                    {
                        cabac_ref_b(
                            h,
                            cb,
                            1 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int * i_1,
                        );
                    }
                    i_1 += 1;
                }
            }
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < 4 as ::core::ffi::c_int {
                if x264_mb_partition_listX_table[0 as ::core::ffi::c_int as usize]
                    [(*h).mb.i_sub_partition[i_2 as usize] as usize]
                    != 0
                {
                    let mut mvd: crate::stdlib::uint16_t = cabac_mvd(
                        h,
                        cb,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int * i_2,
                        2 as ::core::ffi::c_int,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[(4 as ::core::ffi::c_int * i_2) as usize] as ::core::ffi::c_int,
                        block_idx_y[(4 as ::core::ffi::c_int * i_2) as usize] as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        mvd,
                    );
                }
                i_2 += 1;
            }
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_3 < 4 as ::core::ffi::c_int {
                if x264_mb_partition_listX_table[1 as ::core::ffi::c_int as usize]
                    [(*h).mb.i_sub_partition[i_3 as usize] as usize]
                    != 0
                {
                    let mut mvd_0: crate::stdlib::uint16_t = cabac_mvd(
                        h,
                        cb,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int * i_3,
                        2 as ::core::ffi::c_int,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[(4 as ::core::ffi::c_int * i_3) as usize] as ::core::ffi::c_int,
                        block_idx_y[(4 as ::core::ffi::c_int * i_3) as usize] as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        mvd_0,
                    );
                }
                i_3 += 1;
            }
        } else if i_mb_type >= crate::src::common::macroblock::B_L0_L0 as ::core::ffi::c_int
            && i_mb_type <= crate::src::common::macroblock::B_BI_BI as ::core::ffi::c_int
        {
            static mut i_mb_bits: [crate::stdlib::uint8_t; 27] = [
                0x31 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x29 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x35 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x2d as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x43 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x63 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x3d as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x2f as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x39 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x53 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x73 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x4b as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x6b as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x5b as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x7b as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x47 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x67 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0x21 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ];
            let idx: ::core::ffi::c_int = (i_mb_type
                - crate::src::common::macroblock::B_L0_L0 as ::core::ffi::c_int
                    as ::core::ffi::c_int)
                * 3 as ::core::ffi::c_int
                + ((*h).mb.i_partition
                    - crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
                        as ::core::ffi::c_int);
            let mut bits: ::core::ffi::c_int = i_mb_bits[idx as usize] as ::core::ffi::c_int;
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                bits & 1 as ::core::ffi::c_int,
            );
            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int
                    - (bits & 1 as ::core::ffi::c_int),
                bits >> 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
            );
            bits >>= 2 as ::core::ffi::c_int;
            if bits != 1 as ::core::ffi::c_int {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                    bits & 1 as ::core::ffi::c_int,
                );
                bits >>= 1 as ::core::ffi::c_int;
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                    bits & 1 as ::core::ffi::c_int,
                );
                bits >>= 1 as ::core::ffi::c_int;
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                    bits & 1 as ::core::ffi::c_int,
                );
                bits >>= 1 as ::core::ffi::c_int;
                if bits != 1 as ::core::ffi::c_int {
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                        cb as *mut crate::src::common::cabac::x264_cabac_t,
                        27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                        bits & 1 as ::core::ffi::c_int,
                    );
                }
            }
            let mut b_list: *const [crate::stdlib::uint8_t; 2] =
                &raw const *(&raw const x264_mb_type_list_table
                    as *const [[crate::stdlib::uint8_t; 2]; 2])
                    .offset(i_mb_type as isize)
                    as *const [crate::stdlib::uint8_t; 2];
            if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                if (*b_list.offset(0 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize]
                    != 0
                {
                    cabac_ref_b(h, cb, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
                }
                if (*b_list.offset(0 as ::core::ffi::c_int as isize))
                    [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != 0
                    && (*h).mb.i_partition
                        != crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                {
                    cabac_ref_b(
                        h,
                        cb,
                        0 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int
                            >> ((*h).mb.i_partition
                                == crate::src::common::macroblock::D_8x16 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                    );
                }
            }
            if (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                if (*b_list.offset(1 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize]
                    != 0
                {
                    cabac_ref_b(h, cb, 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
                }
                if (*b_list.offset(1 as ::core::ffi::c_int as isize))
                    [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != 0
                    && (*h).mb.i_partition
                        != crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                {
                    cabac_ref_b(
                        h,
                        cb,
                        1 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int
                            >> ((*h).mb.i_partition
                                == crate::src::common::macroblock::D_8x16 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                    );
                }
            }
            let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_list < 2 as ::core::ffi::c_int {
                if (*h).mb.i_partition
                    == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                {
                    if (*b_list.offset(i_list as isize))[0 as ::core::ffi::c_int as usize] != 0 {
                        let mut mvd_1: crate::stdlib::uint16_t = cabac_mvd(
                            h,
                            cb,
                            i_list,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            i_list,
                            mvd_1,
                        );
                    }
                } else if (*h).mb.i_partition
                    == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
                {
                    if (*b_list.offset(i_list as isize))[0 as ::core::ffi::c_int as usize] != 0 {
                        let mut mvd_2: crate::stdlib::uint16_t = cabac_mvd(
                            h,
                            cb,
                            i_list,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            i_list,
                            mvd_2,
                        );
                    }
                    if (*b_list.offset(i_list as isize))[1 as ::core::ffi::c_int as usize] != 0 {
                        let mut mvd_3: crate::stdlib::uint16_t = cabac_mvd(
                            h,
                            cb,
                            i_list,
                            8 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            block_idx_y[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            i_list,
                            mvd_3,
                        );
                    }
                } else {
                    if (*b_list.offset(i_list as isize))[0 as ::core::ffi::c_int as usize] != 0 {
                        let mut mvd_4: crate::stdlib::uint16_t = cabac_mvd(
                            h,
                            cb,
                            i_list,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            i_list,
                            mvd_4,
                        );
                    }
                    if (*b_list.offset(i_list as isize))[1 as ::core::ffi::c_int as usize] != 0 {
                        let mut mvd_5: crate::stdlib::uint16_t = cabac_mvd(
                            h,
                            cb,
                            i_list,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                        x264_macroblock_cache_mvd(
                            h,
                            block_idx_x[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            block_idx_y[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
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
            85 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            89 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            93 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            97 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            101 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            1012 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            460 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            464 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            468 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            1016 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            472 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            476 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            480 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            1020 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        ];
        if b_dc != 0 {
            i_idx -= crate::src::common::base::LUMA_DC;
            if i_cat == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int {
                let mut i_nza: ::core::ffi::c_int =
                    if (*h).mb.cache.i_cbp_left != -(1 as ::core::ffi::c_int) {
                        (*h).mb.cache.i_cbp_left >> 8 as ::core::ffi::c_int + i_idx
                            & 1 as ::core::ffi::c_int
                    } else {
                        b_intra
                    };
                let mut i_nzb: ::core::ffi::c_int =
                    if (*h).mb.cache.i_cbp_top != -(1 as ::core::ffi::c_int) {
                        (*h).mb.cache.i_cbp_top >> 8 as ::core::ffi::c_int + i_idx
                            & 1 as ::core::ffi::c_int
                    } else {
                        b_intra
                    };
                return base_ctx[i_cat as usize] as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * i_nzb
                    + i_nza;
            } else {
                let mut i_nza_0: ::core::ffi::c_int = (*h).mb.cache.i_cbp_left
                    >> 8 as ::core::ffi::c_int + i_idx
                    & 1 as ::core::ffi::c_int;
                let mut i_nzb_0: ::core::ffi::c_int = (*h).mb.cache.i_cbp_top
                    >> 8 as ::core::ffi::c_int + i_idx
                    & 1 as ::core::ffi::c_int;
                return base_ctx[i_cat as usize] as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * i_nzb_0
                    + i_nza_0;
            }
        } else {
            let mut i_nza_1: ::core::ffi::c_int =
                (*h).mb.cache.non_zero_count[(x264_scan8[i_idx as usize] as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] as ::core::ffi::c_int;
            let mut i_nzb_1: ::core::ffi::c_int =
                (*h).mb.cache.non_zero_count[(x264_scan8[i_idx as usize] as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int)
                    as usize] as ::core::ffi::c_int;
            if 0 != 0 && b_intra == 0 {
                return base_ctx[i_cat as usize] as ::core::ffi::c_int
                    + (2 as ::core::ffi::c_int * i_nzb_1 + i_nza_1 & 0x7f as ::core::ffi::c_int);
            } else {
                i_nza_1 &= 0x7f as ::core::ffi::c_int + (b_intra << 7 as ::core::ffi::c_int);
                i_nzb_1 &= 0x7f as ::core::ffi::c_int + (b_intra << 7 as ::core::ffi::c_int);
                return base_ctx[i_cat as usize] as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * (i_nzb_1 != 0) as ::core::ffi::c_int
                    + (i_nza_1 != 0) as ::core::ffi::c_int;
            }
        };
    }
}
static mut coeff_abs_level1_ctx: [crate::stdlib::uint8_t; 8] = [
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
static mut coeff_abs_levelgt1_ctx: [crate::stdlib::uint8_t; 8] = [
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
static mut coeff_abs_levelgt1_ctx_chroma_dc: [crate::stdlib::uint8_t; 8] = [
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
static mut coeff_abs_level_transition: [[crate::stdlib::uint8_t; 8]; 2] = [
    [
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
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
        let mut ctx_sig: ::core::ffi::c_int =
            crate::src::common::tables::x264_significant_coeff_flag_offset
                [(*h).mb.b_interlaced as usize][ctx_block_cat as usize]
                as ::core::ffi::c_int;
        let mut ctx_last: ::core::ffi::c_int =
            crate::src::common::tables::x264_last_coeff_flag_offset[(*h).mb.b_interlaced as usize]
                [ctx_block_cat as usize] as ::core::ffi::c_int;
        let mut ctx_level: ::core::ffi::c_int =
            crate::src::common::tables::x264_coeff_abs_level_m1_offset[ctx_block_cat as usize]
                as ::core::ffi::c_int;
        let mut coeff_idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut node_ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut last: ::core::ffi::c_int =
            (*h).quantf.coeff_last[ctx_block_cat as usize].expect("non-null function pointer")(l);
        let mut levelgt1_ctx: *const crate::stdlib::uint8_t = if chroma422dc != 0 {
            &raw const coeff_abs_levelgt1_ctx_chroma_dc as *const crate::stdlib::uint8_t
        } else {
            &raw const coeff_abs_levelgt1_ctx as *const crate::stdlib::uint8_t
        };
        let mut coeffs: [crate::src::common::common::dctcoef; 64] = [0; 64];
        if chroma422dc != 0 {
            let mut count_m1: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                if *l.offset(i as isize) != 0 {
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i as isize);
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                        cb as *mut crate::src::common::cabac::x264_cabac_t,
                        ctx_sig
                            + crate::src::common::tables::x264_coeff_flag_offset_chroma_422_dc
                                [i as usize] as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    if i == last {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctx_last
                                + crate::src::common::tables::x264_coeff_flag_offset_chroma_422_dc
                                    [i as usize]
                                    as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                        break;
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctx_last
                                + crate::src::common::tables::x264_coeff_flag_offset_chroma_422_dc
                                    [i as usize]
                                    as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                        cb as *mut crate::src::common::cabac::x264_cabac_t,
                        ctx_sig
                            + crate::src::common::tables::x264_coeff_flag_offset_chroma_422_dc
                                [i as usize] as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
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
            let mut count_m1_0: ::core::ffi::c_int = crate::src::common::tables::x264_count_cat_m1
                [ctx_block_cat as usize]
                as ::core::ffi::c_int;
            if count_m1_0 == 63 as ::core::ffi::c_int {
                let mut sig_offset: *const crate::stdlib::uint8_t =
                    &raw const *(&raw const crate::src::common::tables::x264_significant_coeff_flag_offset_8x8
                        as *const [crate::stdlib::uint8_t; 64])
                        .offset((*h).mb.b_interlaced as isize)
                        as *const crate::stdlib::uint8_t;
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                loop {
                    if *l.offset(i_0 as isize) != 0 {
                        coeff_idx += 1;
                        coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctx_sig + *sig_offset.offset(i_0 as isize) as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                        if i_0 == last {
                            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                cb as *mut crate::src::common::cabac::x264_cabac_t,
                                ctx_last
                                    + crate::src::common::tables::x264_last_coeff_flag_offset_8x8
                                        [i_0 as usize]
                                        as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            );
                            break;
                        } else {
                            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                cb as *mut crate::src::common::cabac::x264_cabac_t,
                                ctx_last
                                    + crate::src::common::tables::x264_last_coeff_flag_offset_8x8
                                        [i_0 as usize]
                                        as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                        }
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctx_sig + *sig_offset.offset(i_0 as isize) as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
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
                let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                loop {
                    if *l.offset(i_1 as isize) != 0 {
                        coeff_idx += 1;
                        coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctx_sig + i_1,
                            1 as ::core::ffi::c_int,
                        );
                        if i_1 == last {
                            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                cb as *mut crate::src::common::cabac::x264_cabac_t,
                                ctx_last + i_1,
                                1 as ::core::ffi::c_int,
                            );
                            break;
                        } else {
                            crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                cb as *mut crate::src::common::cabac::x264_cabac_t,
                                ctx_last + i_1,
                                0 as ::core::ffi::c_int,
                            );
                        }
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctx_sig + i_1,
                            0 as ::core::ffi::c_int,
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
            let mut coeff: ::core::ffi::c_int = coeffs[coeff_idx as usize] as ::core::ffi::c_int;
            let mut abs_coeff: ::core::ffi::c_int = crate::stdlib::abs(coeff);
            let mut coeff_sign: ::core::ffi::c_int = coeff >> 31 as ::core::ffi::c_int;
            let mut ctx: ::core::ffi::c_int =
                coeff_abs_level1_ctx[node_ctx as usize] as ::core::ffi::c_int + ctx_level;
            if abs_coeff > 1 as ::core::ffi::c_int {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    ctx,
                    1 as ::core::ffi::c_int,
                );
                ctx = *levelgt1_ctx.offset(node_ctx as isize) as ::core::ffi::c_int + ctx_level;
                let mut i_2: ::core::ffi::c_int = (if abs_coeff < 15 as ::core::ffi::c_int {
                    abs_coeff
                } else {
                    15 as ::core::ffi::c_int
                }) - 2 as ::core::ffi::c_int;
                while i_2 > 0 as ::core::ffi::c_int {
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                        cb as *mut crate::src::common::cabac::x264_cabac_t,
                        ctx,
                        1 as ::core::ffi::c_int,
                    );
                    i_2 -= 1;
                }
                if abs_coeff < 15 as ::core::ffi::c_int {
                    crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                        cb as *mut crate::src::common::cabac::x264_cabac_t,
                        ctx,
                        0 as ::core::ffi::c_int,
                    );
                } else {
                    crate::src::common::cabac::x264_8_cabac_encode_ue_bypass(
                        cb as *mut crate::src::common::cabac::x264_cabac_t,
                        0 as ::core::ffi::c_int,
                        abs_coeff - 15 as ::core::ffi::c_int,
                    );
                }
                node_ctx = coeff_abs_level_transition[1 as ::core::ffi::c_int as usize]
                    [node_ctx as usize] as ::core::ffi::c_int;
            } else {
                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                    ctx,
                    0 as ::core::ffi::c_int,
                );
                node_ctx = coeff_abs_level_transition[0 as ::core::ffi::c_int as usize]
                    [node_ctx as usize] as ::core::ffi::c_int;
            }
            crate::src::common::cabac::x264_8_cabac_encode_bypass_c(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
                coeff_sign,
            );
            coeff_idx -= 1;
            if !(coeff_idx >= 0 as ::core::ffi::c_int) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_block_residual_c(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        cabac_block_residual_internal(h, cb, ctx_block_cat, l, 0 as ::core::ffi::c_int);
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
            1 as ::core::ffi::c_int,
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
        let i_mb_type: ::core::ffi::c_int = (*h).mb.i_type;
        let i_mb_pos_start: ::core::ffi::c_int = x264_cabac_pos(cb) as ::core::ffi::c_int;
        if (*h).sh.b_mbaff != 0
            && ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int == 0
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
        let mut i_mb_pos_tex: ::core::ffi::c_int = x264_cabac_pos(cb);
        (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
        if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            let mut s: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
                p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
                cur_bits: 0,
                i_left: 0,
                i_bits_encoded: 0,
            };
            bs_init(
                &raw mut s,
                (*cb).p as *mut ::core::ffi::c_void,
                (*cb).p_end.offset_from((*cb).p) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
            let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p < plane_count {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < 256 as ::core::ffi::c_int {
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
                let mut ch: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while ch < 3 as ::core::ffi::c_int {
                    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_0
                        < 16 as ::core::ffi::c_int
                            >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                    {
                        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while j < 8 as ::core::ffi::c_int {
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
            crate::src::common::cabac::x264_8_cabac_encode_init_core(
                cb as *mut crate::src::common::cabac::x264_cabac_t,
            );
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
            let b_intra: ::core::ffi::c_int = (i_mb_type
                == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            cabac_qp_delta(h, cb);
            if i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int {
                let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while p_0 < plane_count {
                    let mut ctxidxinc: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                        h,
                        ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_DC
                            as ::core::ffi::c_int as usize][p_0 as usize]
                            as ::core::ffi::c_int,
                        48 as ::core::ffi::c_int + p_0,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    if (*h).mb.cache.non_zero_count
                        [x264_scan8[(48 as ::core::ffi::c_int + p_0) as usize] as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc,
                            1 as ::core::ffi::c_int,
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
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    if (*h).mb.i_cbp_luma != 0 {
                        let mut i_1: ::core::ffi::c_int = p_0 * 16 as ::core::ffi::c_int;
                        while i_1 < p_0 * 16 as ::core::ffi::c_int + 16 as ::core::ffi::c_int {
                            let mut ctxidxinc_0: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                                h,
                                ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_AC
                                    as ::core::ffi::c_int
                                    as usize][p_0 as usize]
                                    as ::core::ffi::c_int,
                                i_1,
                                1 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                            if (*h).mb.cache.non_zero_count[x264_scan8[i_1 as usize] as usize] != 0
                            {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                                    ctxidxinc_0,
                                    1 as ::core::ffi::c_int,
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
                                        .offset(1 as ::core::ffi::c_int as isize),
                                );
                            } else {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                                    ctxidxinc_0,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                            i_1 += 1;
                        }
                    }
                    p_0 += 1;
                }
            } else if (*h).mb.b_transform_8x8 != 0 {
                if plane_count == 3 as ::core::ffi::c_int {
                    let mut nnzbak: [[crate::stdlib::uint8_t; 8]; 3] = [[0; 8]; 3];
                    if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                            == 0
                        && *(*h)
                            .mb
                            .cbp
                            .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                            as ::core::ffi::c_int
                            & 0x1000 as ::core::ffi::c_int
                            == 0
                    {
                        nnzbak[0 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[0 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[1 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[2 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                    }
                    if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                            == 0
                        && *(*h)
                            .mb
                            .cbp
                            .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                            as ::core::ffi::c_int
                            & 0x1000 as ::core::ffi::c_int
                            == 0
                    {
                        nnzbak[0 as ::core::ffi::c_int as usize]
                            [2 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[0 as ::core::ffi::c_int as usize]
                            [3 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[1 as ::core::ffi::c_int as usize]
                            [2 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[1 as ::core::ffi::c_int as usize]
                            [3 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[2 as ::core::ffi::c_int as usize]
                            [2 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                        nnzbak[2 as ::core::ffi::c_int as usize]
                            [3 as ::core::ffi::c_int as usize] = (*h).mb.cache.non_zero_count
                            [(x264_scan8[(16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = 0 as crate::stdlib::uint8_t;
                    }
                    if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_top_xy as isize)
                            == 0
                        && *(*h).mb.cbp.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                            & 0x1000 as ::core::ffi::c_int
                            == 0
                    {
                        (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*((&raw mut (*h).mb.cache.non_zero_count
                            as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0 as ::core::ffi::c_uint as crate::stdlib::uint32_t;
                        (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*((&raw mut (*h).mb.cache.non_zero_count
                            as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0 as ::core::ffi::c_uint as crate::stdlib::uint32_t;
                        (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = (*((&raw mut (*h).mb.cache.non_zero_count
                            as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = 0 as ::core::ffi::c_uint as crate::stdlib::uint32_t;
                    }
                    let mut p_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while p_1 < 3 as ::core::ffi::c_int {
                        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut msk: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
                        let mut skip: ::core::ffi::c_int = 0;
                        while msk != 0 && {
                            skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                            i_2 += skip;
                            msk >>= skip + 1 as ::core::ffi::c_int;
                            1 as ::core::ffi::c_int != 0
                        } {
                            let mut ctxidxinc_1: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                                h,
                                ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_8x8
                                    as ::core::ffi::c_int
                                    as usize][p_1 as usize]
                                    as ::core::ffi::c_int,
                                i_2 * 4 as ::core::ffi::c_int + p_1 * 16 as ::core::ffi::c_int,
                                b_intra,
                                0 as ::core::ffi::c_int,
                            );
                            if (*h).mb.cache.non_zero_count[x264_scan8[(i_2
                                * 4 as ::core::ffi::c_int
                                + p_1 * 16 as ::core::ffi::c_int)
                                as usize]
                                as usize]
                                != 0
                            {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                                    ctxidxinc_1,
                                    1 as ::core::ffi::c_int,
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
                                        .offset((i_2 + p_1 * 4 as ::core::ffi::c_int) as isize)
                                        as *mut crate::src::common::common::dctcoef,
                                );
                            } else {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                                    ctxidxinc_1,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                            i_2 += 1;
                        }
                        p_1 += 1;
                    }
                    if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                            == 0
                        && *(*h)
                            .mb
                            .cbp
                            .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                            as ::core::ffi::c_int
                            & 0x1000 as ::core::ffi::c_int
                            == 0
                    {
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[0 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[0 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[1 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[2 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize];
                    }
                    if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                            == 0
                        && *(*h)
                            .mb
                            .cbp
                            .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                            as ::core::ffi::c_int
                            & 0x1000 as ::core::ffi::c_int
                            == 0
                    {
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[0 as ::core::ffi::c_int as usize]
                            [2 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[0 as ::core::ffi::c_int as usize]
                            [3 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[1 as ::core::ffi::c_int as usize]
                            [2 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[1 as ::core::ffi::c_int as usize]
                            [3 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[2 as ::core::ffi::c_int as usize]
                            [2 as ::core::ffi::c_int as usize];
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] = nnzbak[2 as ::core::ffi::c_int as usize]
                            [3 as ::core::ffi::c_int as usize];
                    }
                    if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                        && *(*h)
                            .mb
                            .mb_transform_size
                            .offset((*h).mb.i_mb_top_xy as isize)
                            == 0
                        && *(*h).mb.cbp.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                            & 0x1000 as ::core::ffi::c_int
                            == 0
                    {
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i =
                            (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t
                                as *mut crate::src::common::base::x264_union32_t))
                                .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i =
                            (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t
                                as *mut crate::src::common::base::x264_union32_t))
                                .i;
                        (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    - 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i =
                            (*((&raw mut *(&raw mut nnzbak as *mut [crate::stdlib::uint8_t; 8])
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t
                                as *mut crate::src::common::base::x264_union32_t))
                                .i;
                    }
                } else {
                    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut msk_0: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
                    let mut skip_0: ::core::ffi::c_int = 0;
                    while msk_0 != 0 && {
                        skip_0 = x264_ctz_4bit(msk_0 as crate::stdlib::uint32_t);
                        i_3 += skip_0;
                        msk_0 >>= skip_0 + 1 as ::core::ffi::c_int;
                        1 as ::core::ffi::c_int != 0
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
                let mut p_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while p_2 < plane_count {
                    let mut i8x8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut msk_1: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
                    let mut skip_1: ::core::ffi::c_int = 0;
                    while msk_1 != 0 && {
                        skip_1 = x264_ctz_4bit(msk_1 as crate::stdlib::uint32_t);
                        i8x8 += skip_1;
                        msk_1 >>= skip_1 + 1 as ::core::ffi::c_int;
                        1 as ::core::ffi::c_int != 0
                    } {
                        let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_4 < 4 as ::core::ffi::c_int {
                            let mut ctxidxinc_2: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                                h,
                                ctx_cat_plane[crate::src::common::macroblock::DCT_LUMA_4x4
                                    as ::core::ffi::c_int
                                    as usize][p_2 as usize]
                                    as ::core::ffi::c_int,
                                i_4 + i8x8 * 4 as ::core::ffi::c_int
                                    + p_2 * 16 as ::core::ffi::c_int,
                                b_intra,
                                0 as ::core::ffi::c_int,
                            );
                            if (*h).mb.cache.non_zero_count[x264_scan8[(i_4
                                + i8x8 * 4 as ::core::ffi::c_int
                                + p_2 * 16 as ::core::ffi::c_int)
                                as usize]
                                as usize]
                                != 0
                            {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                                    ctxidxinc_2,
                                    1 as ::core::ffi::c_int,
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
                                        .offset(
                                            (i_4 + i8x8 * 4 as ::core::ffi::c_int
                                                + p_2 * 16 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        as *mut crate::src::common::common::dctcoef,
                                );
                            } else {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                                    ctxidxinc_2,
                                    0 as ::core::ffi::c_int,
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
                    let mut ctxidxinc_3: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                        h,
                        crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                        49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                        b_intra,
                        1 as ::core::ffi::c_int,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8
                        [(49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                        as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc_3,
                            1 as ::core::ffi::c_int,
                        );
                        cabac_block_residual_422_dc(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.chroma_dc
                                as *mut [crate::src::common::common::dctcoef; 8])
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc_3,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    let mut ctxidxinc_4: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                        h,
                        crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                        49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                        b_intra,
                        1 as ::core::ffi::c_int,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8
                        [(49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc_4,
                            1 as ::core::ffi::c_int,
                        );
                        cabac_block_residual_422_dc(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.chroma_dc
                                as *mut [crate::src::common::common::dctcoef; 8])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc_4,
                            0 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    let mut ctxidxinc_5: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                        h,
                        crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                        49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                        b_intra,
                        1 as ::core::ffi::c_int,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8
                        [(49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                        as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc_5,
                            1 as ::core::ffi::c_int,
                        );
                        cabac_block_residual(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.chroma_dc
                                as *mut [crate::src::common::common::dctcoef; 8])
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc_5,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    let mut ctxidxinc_6: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                        h,
                        crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                        49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                        b_intra,
                        1 as ::core::ffi::c_int,
                    );
                    if (*h).mb.cache.non_zero_count[x264_scan8
                        [(49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        as usize]
                        != 0
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc_6,
                            1 as ::core::ffi::c_int,
                        );
                        cabac_block_residual(
                            h,
                            cb,
                            crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                            &raw mut *(&raw mut (*h).dct.chroma_dc
                                as *mut [crate::src::common::common::dctcoef; 8])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::src::common::common::dctcoef,
                        );
                    } else {
                        crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                            cb as *mut crate::src::common::cabac::x264_cabac_t,
                            ctxidxinc_6,
                            0 as ::core::ffi::c_int,
                        );
                    }
                }
                if (*h).mb.i_cbp_chroma == 2 as ::core::ffi::c_int {
                    let mut step: ::core::ffi::c_int = (8 as ::core::ffi::c_int)
                        << (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                    let mut i_5: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
                    while i_5 < 3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int {
                        let mut j_0: ::core::ffi::c_int = i_5;
                        while j_0 < i_5 + 4 as ::core::ffi::c_int {
                            let mut ctxidxinc_7: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                                h,
                                crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int,
                                j_0,
                                b_intra,
                                0 as ::core::ffi::c_int,
                            );
                            if (*h).mb.cache.non_zero_count[x264_scan8[j_0 as usize] as usize] != 0
                            {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                                    ctxidxinc_7,
                                    1 as ::core::ffi::c_int,
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
                                        .offset(1 as ::core::ffi::c_int as isize),
                                );
                            } else {
                                crate::src::common::cabac::x264_8_cabac_encode_decision_c(
                                    cb as *mut crate::src::common::cabac::x264_cabac_t,
                                    ctxidxinc_7,
                                    0 as ::core::ffi::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_write_cabac(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            macroblock_write_cabac_internal(
                h,
                cb,
                3 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            macroblock_write_cabac_internal(
                h,
                cb,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        } else {
            macroblock_write_cabac_internal(
                h,
                cb,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        };
    }
}
