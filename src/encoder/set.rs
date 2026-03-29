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
    pub unsafe extern "C" fn bs_pos(
        mut s: *mut crate::src::common::bitstream::bs_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            return (8 as ::core::ffi::c_long
                * (*s).p.offset_from((*s).p_start) as ::core::ffi::c_long
                + (crate::osdep_h::WORD_SIZE * 8 as ::core::ffi::c_int) as ::core::ffi::c_long
                - (*s).i_left as ::core::ffi::c_long) as ::core::ffi::c_int;
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
    pub unsafe extern "C" fn bs_realign(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            let mut offset: ::core::ffi::c_int = ((*s).p as crate::stdlib::intptr_t
                & 3 as ::core::ffi::c_int as crate::stdlib::intptr_t)
                as ::core::ffi::c_int;
            if offset != 0 {
                (*s).p = (*s).p.offset(-(offset as isize));
                (*s).i_left = (crate::osdep_h::WORD_SIZE - offset) * 8 as ::core::ffi::c_int;
                (*s).cur_bits =
                    endian_fix32((*((*s).p as *mut crate::src::common::base::x264_union32_t)).i)
                        as crate::stdlib::uintptr_t;
                (*s).cur_bits >>= (4 as ::core::ffi::c_int - offset) * 8 as ::core::ffi::c_int;
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
    #[inline]
    pub unsafe extern "C" fn bs_write32(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut i_bits: crate::stdlib::uint32_t,
    ) {
        unsafe {
            bs_write(
                s,
                16 as ::core::ffi::c_int,
                i_bits >> 16 as ::core::ffi::c_int,
            );
            bs_write(s, 16 as ::core::ffi::c_int, i_bits);
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_write1(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut i_bit: crate::stdlib::uint32_t,
    ) {
        unsafe {
            (*s).cur_bits <<= 1 as ::core::ffi::c_int;
            (*s).cur_bits |= i_bit as crate::stdlib::uintptr_t;
            (*s).i_left -= 1;
            if (*s).i_left
                == crate::osdep_h::WORD_SIZE * 8 as ::core::ffi::c_int - 32 as ::core::ffi::c_int
            {
                (*((*s).p as *mut crate::src::common::base::x264_union32_t)).i =
                    endian_fix32((*s).cur_bits as crate::stdlib::uint32_t);
                (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
                (*s).i_left = crate::osdep_h::WORD_SIZE * 8 as ::core::ffi::c_int;
            }
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_align_10(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            if (*s).i_left & 7 as ::core::ffi::c_int != 0 {
                bs_write(
                    s,
                    (*s).i_left & 7 as ::core::ffi::c_int,
                    ((1 as ::core::ffi::c_int)
                        << ((*s).i_left & 7 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
            }
            bs_flush(s);
        }
    }
    pub static mut x264_ue_size_tab: [crate::stdlib::uint8_t; 256] = [
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ];
    #[inline]
    pub unsafe extern "C" fn bs_write_ue_big(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut val: ::core::ffi::c_uint,
    ) {
        unsafe {
            let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            val = val.wrapping_add(1);
            let mut tmp: ::core::ffi::c_int = val as ::core::ffi::c_int;
            if tmp >= 0x10000 as ::core::ffi::c_int {
                size = 32 as ::core::ffi::c_int;
                tmp >>= 16 as ::core::ffi::c_int;
            }
            if tmp >= 0x100 as ::core::ffi::c_int {
                size += 16 as ::core::ffi::c_int;
                tmp >>= 8 as ::core::ffi::c_int;
            }
            size += x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
            bs_write(
                s,
                size >> 1 as ::core::ffi::c_int,
                0 as crate::stdlib::uint32_t,
            );
            bs_write(
                s,
                (size >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int,
                val as crate::stdlib::uint32_t,
            );
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_write_se(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut val: ::core::ffi::c_int,
    ) {
        unsafe {
            let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut tmp: ::core::ffi::c_int =
                1 as ::core::ffi::c_int - val * 2 as ::core::ffi::c_int;
            if tmp < 0 as ::core::ffi::c_int {
                tmp = val * 2 as ::core::ffi::c_int;
            }
            val = tmp;
            if tmp >= 0x100 as ::core::ffi::c_int {
                size = 16 as ::core::ffi::c_int;
                tmp >>= 8 as ::core::ffi::c_int;
            }
            size += x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
            bs_write(s, size, val as crate::stdlib::uint32_t);
        }
    }
    #[inline]
    pub unsafe extern "C" fn bs_rbsp_trailing(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            bs_write1(s, 1 as crate::stdlib::uint32_t);
            bs_write(
                s,
                (*s).i_left & 7 as ::core::ffi::c_int,
                0 as crate::stdlib::uint32_t,
            );
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn bs_size_se(mut val: ::core::ffi::c_int) -> ::core::ffi::c_int {
        unsafe {
            let mut tmp: ::core::ffi::c_int =
                1 as ::core::ffi::c_int - val * 2 as ::core::ffi::c_int;
            if tmp < 0 as ::core::ffi::c_int {
                tmp = val * 2 as ::core::ffi::c_int;
            }
            if tmp < 256 as ::core::ffi::c_int {
                return x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
            } else {
                return x264_ue_size_tab[(tmp >> 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int;
            };
        }
    }
    use crate::src::encoder::set::osdep_h::endian_fix;
    use crate::src::encoder::set::osdep_h::endian_fix32;
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
}
pub mod macroblock_h {
    pub static mut x264_zigzag_scan4: [[crate::stdlib::uint8_t; 16]; 2] = [
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    ];
    pub static mut x264_zigzag_scan8: [[crate::stdlib::uint8_t; 64]; 2] = [
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            24 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            17 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            32 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            40 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            33 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            26 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            19 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            27 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            34 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            41 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            48 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            56 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            49 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            42 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            35 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            28 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            21 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            22 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            29 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            36 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            43 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            50 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            57 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            58 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            51 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            44 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            37 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            30 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            31 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            38 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            45 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            52 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            59 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            60 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            53 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            46 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            39 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            47 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            54 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            61 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            62 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            55 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            63 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            17 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            24 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            19 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            32 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            26 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            21 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            22 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            27 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            33 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            40 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            34 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            28 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            29 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            30 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            31 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            35 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            41 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            48 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            42 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            36 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            37 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            38 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            39 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            43 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            49 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            50 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            44 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            45 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            46 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            47 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            51 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            56 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            57 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            52 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            53 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            54 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            55 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            58 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            59 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            60 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            61 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            62 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            63 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    ];
}
use crate::src::encoder::set::bitstream_h::bs_align_10;
use crate::src::encoder::set::bitstream_h::bs_flush;
use crate::src::encoder::set::bitstream_h::bs_init;
use crate::src::encoder::set::bitstream_h::bs_pos;
use crate::src::encoder::set::bitstream_h::bs_rbsp_trailing;
use crate::src::encoder::set::bitstream_h::bs_realign;
use crate::src::encoder::set::bitstream_h::bs_size_se;
use crate::src::encoder::set::bitstream_h::bs_write;
use crate::src::encoder::set::bitstream_h::bs_write1;
use crate::src::encoder::set::bitstream_h::bs_write32;
use crate::src::encoder::set::bitstream_h::bs_write_se;
use crate::src::encoder::set::bitstream_h::bs_write_ue_big;
use crate::src::encoder::set::macroblock_h::x264_zigzag_scan4;
use crate::src::encoder::set::macroblock_h::x264_zigzag_scan8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_465 {
    pub w: crate::stdlib::uint8_t,
    pub h: crate::stdlib::uint8_t,
    pub sar: crate::stdlib::uint8_t,
}
static mut num_clock_ts: [crate::stdlib::uint8_t; 10] = [
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
static mut avcintra_uuid: [crate::stdlib::uint8_t; 16] = [
    0xf7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x49 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x3e as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0xb3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0xd4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x47 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x96 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x86 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x86 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0xc9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x70 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x7b as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x64 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x37 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0x2a as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
unsafe extern "C" fn transpose(mut buf: *mut crate::stdlib::uint8_t, mut w: ::core::ffi::c_int) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < w {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < i {
                let mut t: crate::stdlib::uint8_t = *buf.offset((w * i + j) as isize);
                *buf.offset((w * i + j) as isize) = *buf.offset((w * j + i) as isize);
                *buf.offset((w * j + i) as isize) = t;
                j += 1;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn scaling_list_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut sps: *mut crate::src::common::set::x264_sps_t,
    mut idx: ::core::ffi::c_int,
) {
    unsafe {
        let len: ::core::ffi::c_int = if idx < 4 as ::core::ffi::c_int {
            16 as ::core::ffi::c_int
        } else {
            64 as ::core::ffi::c_int
        };
        let mut zigzag: *const crate::stdlib::uint8_t = if idx < 4 as ::core::ffi::c_int {
            &raw const *(&raw const x264_zigzag_scan4 as *const [crate::stdlib::uint8_t; 16])
                .offset(0 as ::core::ffi::c_int as isize)
                as *const crate::stdlib::uint8_t
        } else {
            &raw const *(&raw const x264_zigzag_scan8 as *const [crate::stdlib::uint8_t; 64])
                .offset(0 as ::core::ffi::c_int as isize)
                as *const crate::stdlib::uint8_t
        };
        let mut list: *const crate::stdlib::uint8_t = (*sps).scaling_list[idx as usize];
        let mut def_list: *const crate::stdlib::uint8_t =
            if idx == crate::src::common::set::CQM_4IC as ::core::ffi::c_int {
                (*sps).scaling_list[crate::src::common::set::CQM_4IY as ::core::ffi::c_int as usize]
            } else if idx == crate::src::common::set::CQM_4PC as ::core::ffi::c_int {
                (*sps).scaling_list[crate::src::common::set::CQM_4PY as ::core::ffi::c_int as usize]
            } else if idx
                == crate::src::common::set::CQM_8IC as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            {
                (*sps).scaling_list[(crate::src::common::set::CQM_8IY as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as usize]
            } else if idx
                == crate::src::common::set::CQM_8PC as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            {
                (*sps).scaling_list[(crate::src::common::set::CQM_8PY as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as usize]
            } else {
                crate::src::common::tables::x264_cqm_jvt[idx as usize]
            };
        if crate::stdlib::memcmp(
            list as *const ::core::ffi::c_void,
            def_list as *const ::core::ffi::c_void,
            len as crate::__stddef_size_t_h::size_t,
        ) == 0
        {
            bs_write1(s, 0 as crate::stdlib::uint32_t);
        } else if crate::stdlib::memcmp(
            list as *const ::core::ffi::c_void,
            crate::src::common::tables::x264_cqm_jvt[idx as usize] as *const ::core::ffi::c_void,
            len as crate::__stddef_size_t_h::size_t,
        ) == 0
        {
            bs_write1(s, 1 as crate::stdlib::uint32_t);
            bs_write_se(s, -(8 as ::core::ffi::c_int));
        } else {
            let mut run: ::core::ffi::c_int = 0;
            bs_write1(s, 1 as crate::stdlib::uint32_t);
            run = len;
            while run > 1 as ::core::ffi::c_int {
                if *list.offset(*zigzag.offset((run - 1 as ::core::ffi::c_int) as isize) as isize)
                    as ::core::ffi::c_int
                    != *list
                        .offset(*zigzag.offset((run - 2 as ::core::ffi::c_int) as isize) as isize)
                        as ::core::ffi::c_int
                {
                    break;
                }
                run -= 1;
            }
            if run < len
                && len - run
                    < bs_size_se(
                        -(*list.offset(*zigzag.offset(run as isize) as isize) as ::core::ffi::c_int)
                            as crate::stdlib::int8_t as ::core::ffi::c_int,
                    )
            {
                run = len;
            }
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < run {
                bs_write_se(
                    s,
                    (*list.offset(*zigzag.offset(j as isize) as isize) as ::core::ffi::c_int
                        - (if j > 0 as ::core::ffi::c_int {
                            *list.offset(
                                *zigzag.offset((j - 1 as ::core::ffi::c_int) as isize) as isize
                            ) as ::core::ffi::c_int
                        } else {
                            8 as ::core::ffi::c_int
                        })) as crate::stdlib::int8_t as ::core::ffi::c_int,
                );
                j += 1;
            }
            if run < len {
                bs_write_se(
                    s,
                    -(*list.offset(*zigzag.offset(run as isize) as isize) as ::core::ffi::c_int)
                        as crate::stdlib::int8_t as ::core::ffi::c_int,
                );
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut payload: *mut crate::stdlib::uint8_t,
    mut payload_size: ::core::ffi::c_int,
    mut payload_type: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        bs_realign(s);
        i = 0 as ::core::ffi::c_int;
        while i <= payload_type - 255 as ::core::ffi::c_int {
            bs_write(s, 8 as ::core::ffi::c_int, 255 as crate::stdlib::uint32_t);
            i += 255 as ::core::ffi::c_int;
        }
        bs_write(
            s,
            8 as ::core::ffi::c_int,
            (payload_type - i) as crate::stdlib::uint32_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i <= payload_size - 255 as ::core::ffi::c_int {
            bs_write(s, 8 as ::core::ffi::c_int, 255 as crate::stdlib::uint32_t);
            i += 255 as ::core::ffi::c_int;
        }
        bs_write(
            s,
            8 as ::core::ffi::c_int,
            (payload_size - i) as crate::stdlib::uint32_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i < payload_size {
            bs_write(
                s,
                8 as ::core::ffi::c_int,
                *payload.offset(i as isize) as crate::stdlib::uint32_t,
            );
            i += 1;
        }
        bs_rbsp_trailing(s);
        bs_flush(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init(
    mut sps: *mut crate::src::common::set::x264_sps_t,
    mut i_id: ::core::ffi::c_int,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        let mut csp: ::core::ffi::c_int = (*param).i_csp & crate::x264_h::X264_CSP_MASK;
        (*sps).i_id = i_id;
        (*sps).i_mb_width =
            ((*param).i_width + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int;
        (*sps).i_mb_height =
            ((*param).i_height + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int;
        (*sps).b_frame_mbs_only =
            !((*param).b_interlaced != 0 || (*param).b_fake_interlaced != 0) as ::core::ffi::c_int;
        if (*sps).b_frame_mbs_only == 0 {
            (*sps).i_mb_height =
                (*sps).i_mb_height + 1 as ::core::ffi::c_int & !(1 as ::core::ffi::c_int);
        }
        (*sps).i_chroma_format_idc = if csp >= crate::x264_h::X264_CSP_I444 {
            crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        } else if csp >= crate::x264_h::X264_CSP_I422 {
            crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
        } else if csp >= crate::x264_h::X264_CSP_I420 {
            crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
        } else {
            crate::src::common::base::CHROMA_400 as ::core::ffi::c_int
        };
        (*sps).b_qpprime_y_zero_transform_bypass = ((*param).rc.i_rc_method
            == crate::x264_h::X264_RC_CQP
            && (*param).rc.i_qp_constant == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if (*sps).b_qpprime_y_zero_transform_bypass != 0
            || (*sps).i_chroma_format_idc
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            (*sps).i_profile_idc =
                crate::src::common::base::PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int;
        } else if (*sps).i_chroma_format_idc
            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
        {
            (*sps).i_profile_idc = crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int;
        } else if crate::internal::BIT_DEPTH > 8 as ::core::ffi::c_int {
            (*sps).i_profile_idc = crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int;
        } else if (*param).analyse.b_transform_8x8 != 0
            || (*param).i_cqm_preset != crate::x264_h::X264_CQM_FLAT
            || (*sps).i_chroma_format_idc
                == crate::src::common::base::CHROMA_400 as ::core::ffi::c_int
        {
            (*sps).i_profile_idc = crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int;
        } else if (*param).b_cabac != 0
            || (*param).i_bframe > 0 as ::core::ffi::c_int
            || (*param).b_interlaced != 0
            || (*param).b_fake_interlaced != 0
            || (*param).analyse.i_weighted_pred > 0 as ::core::ffi::c_int
        {
            (*sps).i_profile_idc = crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int;
        } else {
            (*sps).i_profile_idc = crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int;
        }
        (*sps).b_constraint_set0 = ((*sps).i_profile_idc
            == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*sps).b_constraint_set1 = ((*sps).i_profile_idc
            <= crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*sps).b_constraint_set2 = 0 as ::core::ffi::c_int;
        (*sps).b_constraint_set3 = 0 as ::core::ffi::c_int;
        (*sps).i_level_idc = (*param).i_level_idc;
        if (*param).i_level_idc == 9 as ::core::ffi::c_int
            && ((*sps).i_profile_idc
                == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int
                || (*sps).i_profile_idc
                    == crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int)
        {
            (*sps).b_constraint_set3 = 1 as ::core::ffi::c_int;
            (*sps).i_level_idc = 11 as ::core::ffi::c_int;
        }
        if (*param).i_keyint_max == 1 as ::core::ffi::c_int
            && (*sps).i_profile_idc >= crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
        {
            (*sps).b_constraint_set3 = 1 as ::core::ffi::c_int;
        }
        (*sps).vui.i_num_reorder_frames = if (*param).i_bframe_pyramid != 0 {
            2 as ::core::ffi::c_int
        } else if (*param).i_bframe != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        (*sps).i_num_ref_frames = if (16 as ::core::ffi::c_int)
            < (if (*param).i_frame_reference
                > (if 1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
                    > (if (if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) > (*param).i_dpb_size
                    {
                        if (*param).i_bframe_pyramid != 0 {
                            4 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }
                    } else {
                        (*param).i_dpb_size
                    })
                {
                    1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
                } else {
                    if (if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) > (*param).i_dpb_size
                    {
                        if (*param).i_bframe_pyramid != 0 {
                            4 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }
                    } else {
                        (*param).i_dpb_size
                    }
                })
            {
                (*param).i_frame_reference
            } else {
                if 1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
                    > (if (if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) > (*param).i_dpb_size
                    {
                        if (*param).i_bframe_pyramid != 0 {
                            4 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }
                    } else {
                        (*param).i_dpb_size
                    })
                {
                    1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
                } else {
                    if (if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) > (*param).i_dpb_size
                    {
                        if (*param).i_bframe_pyramid != 0 {
                            4 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }
                    } else {
                        (*param).i_dpb_size
                    }
                }
            }) {
            16 as ::core::ffi::c_int
        } else if (*param).i_frame_reference
            > (if 1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
                > (if (if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) > (*param).i_dpb_size
                {
                    if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }
                } else {
                    (*param).i_dpb_size
                })
            {
                1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
            } else {
                if (if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) > (*param).i_dpb_size
                {
                    if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }
                } else {
                    (*param).i_dpb_size
                }
            })
        {
            (*param).i_frame_reference
        } else if 1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
            > (if (if (*param).i_bframe_pyramid != 0 {
                4 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) > (*param).i_dpb_size
            {
                if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }
            } else {
                (*param).i_dpb_size
            })
        {
            1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
        } else if (if (*param).i_bframe_pyramid != 0 {
            4 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) > (*param).i_dpb_size
        {
            if (*param).i_bframe_pyramid != 0 {
                4 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }
        } else {
            (*param).i_dpb_size
        };
        (*sps).vui.i_max_dec_frame_buffering = (*sps).i_num_ref_frames;
        (*sps).i_num_ref_frames -= ((*param).i_bframe_pyramid
            == crate::x264_h::X264_B_PYRAMID_STRICT)
            as ::core::ffi::c_int;
        if (*param).i_keyint_max == 1 as ::core::ffi::c_int {
            (*sps).i_num_ref_frames = 0 as ::core::ffi::c_int;
            (*sps).vui.i_max_dec_frame_buffering = 0 as ::core::ffi::c_int;
        }
        let mut max_frame_num: ::core::ffi::c_int = (*sps).vui.i_max_dec_frame_buffering
            * (((*param).i_bframe_pyramid != 0) as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int;
        if (*param).b_intra_refresh != 0 {
            let mut time_to_recovery: ::core::ffi::c_int =
                (if ((*sps).i_mb_width - 1 as ::core::ffi::c_int) < (*param).i_keyint_max {
                    (*sps).i_mb_width - 1 as ::core::ffi::c_int
                } else {
                    (*param).i_keyint_max
                }) + (*param).i_bframe
                    - 1 as ::core::ffi::c_int;
            max_frame_num = if max_frame_num > time_to_recovery + 1 as ::core::ffi::c_int {
                max_frame_num
            } else {
                time_to_recovery + 1 as ::core::ffi::c_int
            };
        }
        (*sps).i_log2_max_frame_num = 4 as ::core::ffi::c_int;
        while (1 as ::core::ffi::c_int) << (*sps).i_log2_max_frame_num <= max_frame_num {
            (*sps).i_log2_max_frame_num += 1;
        }
        (*sps).i_poc_type = if (*param).i_bframe != 0
            || (*param).b_interlaced != 0
            || (*param).i_avcintra_class != 0
        {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
        if (*sps).i_poc_type == 0 as ::core::ffi::c_int {
            let mut max_delta_poc: ::core::ffi::c_int = ((*param).i_bframe
                + 2 as ::core::ffi::c_int)
                * (((*param).i_bframe_pyramid != 0) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)
                * 2 as ::core::ffi::c_int;
            (*sps).i_log2_max_poc_lsb = 4 as ::core::ffi::c_int;
            while (1 as ::core::ffi::c_int) << (*sps).i_log2_max_poc_lsb
                <= max_delta_poc * 2 as ::core::ffi::c_int
            {
                (*sps).i_log2_max_poc_lsb += 1;
            }
        }
        (*sps).b_vui = 1 as ::core::ffi::c_int;
        (*sps).b_gaps_in_frame_num_value_allowed = 0 as ::core::ffi::c_int;
        (*sps).b_mb_adaptive_frame_field = (*param).b_interlaced;
        (*sps).b_direct8x8_inference = 1 as ::core::ffi::c_int;
        x264_8_sps_init_reconfigurable(sps, param);
        (*sps).vui.b_overscan_info_present = ((*param).vui.i_overscan > 0 as ::core::ffi::c_int
            && (*param).vui.i_overscan <= 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if (*sps).vui.b_overscan_info_present != 0 {
            (*sps).vui.b_overscan_info = if (*param).vui.i_overscan == 2 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        (*sps).vui.b_signal_type_present = 0 as ::core::ffi::c_int;
        (*sps).vui.i_vidformat = if (*param).vui.i_vidformat >= 0 as ::core::ffi::c_int
            && (*param).vui.i_vidformat <= 5 as ::core::ffi::c_int
        {
            (*param).vui.i_vidformat
        } else {
            5 as ::core::ffi::c_int
        };
        (*sps).vui.b_fullrange = if (*param).vui.b_fullrange >= 0 as ::core::ffi::c_int
            && (*param).vui.b_fullrange <= 1 as ::core::ffi::c_int
        {
            (*param).vui.b_fullrange
        } else if csp >= crate::x264_h::X264_CSP_BGR {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        (*sps).vui.b_color_description_present = 0 as ::core::ffi::c_int;
        (*sps).vui.i_colorprim = if (*param).vui.i_colorprim >= 0 as ::core::ffi::c_int
            && (*param).vui.i_colorprim <= 12 as ::core::ffi::c_int
        {
            (*param).vui.i_colorprim
        } else {
            2 as ::core::ffi::c_int
        };
        (*sps).vui.i_transfer = if (*param).vui.i_transfer >= 0 as ::core::ffi::c_int
            && (*param).vui.i_transfer <= 18 as ::core::ffi::c_int
        {
            (*param).vui.i_transfer
        } else {
            2 as ::core::ffi::c_int
        };
        (*sps).vui.i_colmatrix = if (*param).vui.i_colmatrix >= 0 as ::core::ffi::c_int
            && (*param).vui.i_colmatrix <= 14 as ::core::ffi::c_int
        {
            (*param).vui.i_colmatrix
        } else if csp >= crate::x264_h::X264_CSP_BGR {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
        if (*sps).vui.i_colorprim != 2 as ::core::ffi::c_int
            || (*sps).vui.i_transfer != 2 as ::core::ffi::c_int
            || (*sps).vui.i_colmatrix != 2 as ::core::ffi::c_int
        {
            (*sps).vui.b_color_description_present = 1 as ::core::ffi::c_int;
        }
        if (*sps).vui.i_vidformat != 5 as ::core::ffi::c_int
            || (*sps).vui.b_fullrange != 0
            || (*sps).vui.b_color_description_present != 0
        {
            (*sps).vui.b_signal_type_present = 1 as ::core::ffi::c_int;
        }
        (*sps).vui.b_chroma_loc_info_present = ((*param).vui.i_chroma_loc > 0 as ::core::ffi::c_int
            && (*param).vui.i_chroma_loc <= 5 as ::core::ffi::c_int
            && (*sps).i_chroma_format_idc
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if (*sps).vui.b_chroma_loc_info_present != 0 {
            (*sps).vui.i_chroma_loc_top = (*param).vui.i_chroma_loc;
            (*sps).vui.i_chroma_loc_bottom = (*param).vui.i_chroma_loc;
        }
        (*sps).vui.b_timing_info_present = ((*param).i_timebase_num > 0 as crate::stdlib::uint32_t
            && (*param).i_timebase_den > 0 as crate::stdlib::uint32_t)
            as ::core::ffi::c_int;
        if (*sps).vui.b_timing_info_present != 0 {
            (*sps).vui.i_num_units_in_tick = (*param).i_timebase_num;
            (*sps).vui.i_time_scale = (*param)
                .i_timebase_den
                .wrapping_mul(2 as crate::stdlib::uint32_t);
            (*sps).vui.b_fixed_frame_rate = ((*param).b_vfr_input == 0) as ::core::ffi::c_int;
        }
        (*sps).vui.b_vcl_hrd_parameters_present = 0 as ::core::ffi::c_int;
        (*sps).vui.b_nal_hrd_parameters_present = ((*param).i_nal_hrd != 0) as ::core::ffi::c_int;
        (*sps).vui.b_pic_struct_present = (*param).b_pic_struct;
        (*sps).vui.b_bitstream_restriction = !((*sps).b_constraint_set3 != 0
            && (*sps).i_profile_idc >= crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if (*sps).vui.b_bitstream_restriction != 0 {
            (*sps).vui.b_motion_vectors_over_pic_boundaries = 1 as ::core::ffi::c_int;
            (*sps).vui.i_max_bytes_per_pic_denom = 0 as ::core::ffi::c_int;
            (*sps).vui.i_max_bits_per_mb_denom = 0 as ::core::ffi::c_int;
            (*sps).vui.i_log2_max_mv_length_vertical = crate::stdlib::log2f(
                (if 1 as ::core::ffi::c_int
                    > (*param).analyse.i_mv_range * 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (*param).analyse.i_mv_range * 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                }) as ::core::ffi::c_float,
            ) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int;
            (*sps).vui.i_log2_max_mv_length_horizontal = (*sps).vui.i_log2_max_mv_length_vertical;
        }
        (*sps).b_avcintra_hd = ((*param).i_avcintra_class != 0
            && (*param).i_avcintra_class <= 200 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*sps).b_avcintra_4k =
            ((*param).i_avcintra_class > 200 as ::core::ffi::c_int) as ::core::ffi::c_int;
        (*sps).i_cqm_preset = (*param).i_cqm_preset;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init_reconfigurable(
    mut sps: *mut crate::src::common::set::x264_sps_t,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        (*sps).crop.i_left = (*param).crop_rect.i_left;
        (*sps).crop.i_top = (*param).crop_rect.i_top;
        (*sps).crop.i_right = (*param).crop_rect.i_right
            + (*sps).i_mb_width * 16 as ::core::ffi::c_int
            - (*param).i_width;
        (*sps).crop.i_bottom = (*param).crop_rect.i_bottom
            + (*sps).i_mb_height * 16 as ::core::ffi::c_int
            - (*param).i_height;
        (*sps).b_crop = ((*sps).crop.i_left != 0
            || (*sps).crop.i_top != 0
            || (*sps).crop.i_right != 0
            || (*sps).crop.i_bottom != 0) as ::core::ffi::c_int;
        (*sps).vui.b_aspect_ratio_info_present = 0 as ::core::ffi::c_int;
        if (*param).vui.i_sar_width > 0 as ::core::ffi::c_int
            && (*param).vui.i_sar_height > 0 as ::core::ffi::c_int
        {
            (*sps).vui.b_aspect_ratio_info_present = 1 as ::core::ffi::c_int;
            (*sps).vui.i_sar_width = (*param).vui.i_sar_width;
            (*sps).vui.i_sar_height = (*param).vui.i_sar_height;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init_scaling_list(
    mut sps: *mut crate::src::common::set::x264_sps_t,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        match (*sps).i_cqm_preset {
            crate::x264_h::X264_CQM_FLAT => {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < 8 as ::core::ffi::c_int {
                    (*sps).scaling_list[i as usize] =
                        &raw const crate::src::common::tables::x264_cqm_flat16
                            as *const crate::stdlib::uint8_t;
                    i += 1;
                }
            }
            crate::x264_h::X264_CQM_JVT_1 => {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < 8 as ::core::ffi::c_int {
                    (*sps).scaling_list[i_0 as usize] =
                        crate::src::common::tables::x264_cqm_jvt[i_0 as usize];
                    i_0 += 1;
                }
            }
            crate::x264_h::X264_CQM_CUSTOM_1 => {
                transpose(
                    &raw mut (*param).cqm_4iy as *mut crate::stdlib::uint8_t,
                    4 as ::core::ffi::c_int,
                );
                transpose(
                    &raw mut (*param).cqm_4py as *mut crate::stdlib::uint8_t,
                    4 as ::core::ffi::c_int,
                );
                transpose(
                    &raw mut (*param).cqm_4ic as *mut crate::stdlib::uint8_t,
                    4 as ::core::ffi::c_int,
                );
                transpose(
                    &raw mut (*param).cqm_4pc as *mut crate::stdlib::uint8_t,
                    4 as ::core::ffi::c_int,
                );
                transpose(
                    &raw mut (*param).cqm_8iy as *mut crate::stdlib::uint8_t,
                    8 as ::core::ffi::c_int,
                );
                transpose(
                    &raw mut (*param).cqm_8py as *mut crate::stdlib::uint8_t,
                    8 as ::core::ffi::c_int,
                );
                transpose(
                    &raw mut (*param).cqm_8ic as *mut crate::stdlib::uint8_t,
                    8 as ::core::ffi::c_int,
                );
                transpose(
                    &raw mut (*param).cqm_8pc as *mut crate::stdlib::uint8_t,
                    8 as ::core::ffi::c_int,
                );
                (*sps).scaling_list
                    [crate::src::common::set::CQM_4IY as ::core::ffi::c_int as usize] =
                    &raw mut (*param).cqm_4iy as *mut crate::stdlib::uint8_t;
                (*sps).scaling_list
                    [crate::src::common::set::CQM_4PY as ::core::ffi::c_int as usize] =
                    &raw mut (*param).cqm_4py as *mut crate::stdlib::uint8_t;
                (*sps).scaling_list
                    [crate::src::common::set::CQM_4IC as ::core::ffi::c_int as usize] =
                    &raw mut (*param).cqm_4ic as *mut crate::stdlib::uint8_t;
                (*sps).scaling_list
                    [crate::src::common::set::CQM_4PC as ::core::ffi::c_int as usize] =
                    &raw mut (*param).cqm_4pc as *mut crate::stdlib::uint8_t;
                (*sps).scaling_list[(crate::src::common::set::CQM_8IY as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as usize] =
                    &raw mut (*param).cqm_8iy as *mut crate::stdlib::uint8_t;
                (*sps).scaling_list[(crate::src::common::set::CQM_8PY as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as usize] =
                    &raw mut (*param).cqm_8py as *mut crate::stdlib::uint8_t;
                (*sps).scaling_list[(crate::src::common::set::CQM_8IC as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as usize] =
                    &raw mut (*param).cqm_8ic as *mut crate::stdlib::uint8_t;
                (*sps).scaling_list[(crate::src::common::set::CQM_8PC as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as usize] =
                    &raw mut (*param).cqm_8pc as *mut crate::stdlib::uint8_t;
                let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_1 < 8 as ::core::ffi::c_int {
                    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j
                        < (if i_1 < 4 as ::core::ffi::c_int {
                            16 as ::core::ffi::c_int
                        } else {
                            64 as ::core::ffi::c_int
                        })
                    {
                        if *(*sps).scaling_list[i_1 as usize].offset(j as isize)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*sps).scaling_list[i_1 as usize] =
                                crate::src::common::tables::x264_cqm_jvt[i_1 as usize];
                        }
                        j += 1;
                    }
                    i_1 += 1;
                }
            }
            _ => {}
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut sps: *mut crate::src::common::set::x264_sps_t,
) {
    unsafe {
        bs_realign(s);
        bs_write(
            s,
            8 as ::core::ffi::c_int,
            (*sps).i_profile_idc as crate::stdlib::uint32_t,
        );
        bs_write1(s, (*sps).b_constraint_set0 as crate::stdlib::uint32_t);
        bs_write1(s, (*sps).b_constraint_set1 as crate::stdlib::uint32_t);
        bs_write1(s, (*sps).b_constraint_set2 as crate::stdlib::uint32_t);
        bs_write1(s, (*sps).b_constraint_set3 as crate::stdlib::uint32_t);
        bs_write(s, 4 as ::core::ffi::c_int, 0 as crate::stdlib::uint32_t);
        bs_write(
            s,
            8 as ::core::ffi::c_int,
            (*sps).i_level_idc as crate::stdlib::uint32_t,
        );
        bs_write_ue_big(s, (*sps).i_id as ::core::ffi::c_uint);
        if (*sps).i_profile_idc >= crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int {
            bs_write_ue_big(s, (*sps).i_chroma_format_idc as ::core::ffi::c_uint);
            if (*sps).i_chroma_format_idc
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                bs_write1(s, 0 as crate::stdlib::uint32_t);
            }
            bs_write_ue_big(
                s,
                (crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
            bs_write_ue_big(
                s,
                (crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
            bs_write1(
                s,
                (*sps).b_qpprime_y_zero_transform_bypass as crate::stdlib::uint32_t,
            );
            bs_write1(s, (*sps).b_avcintra_hd as crate::stdlib::uint32_t);
            if (*sps).b_avcintra_hd != 0 {
                scaling_list_write(
                    s,
                    sps,
                    crate::src::common::set::CQM_4IY as ::core::ffi::c_int,
                );
                scaling_list_write(
                    s,
                    sps,
                    crate::src::common::set::CQM_4IC as ::core::ffi::c_int,
                );
                scaling_list_write(
                    s,
                    sps,
                    crate::src::common::set::CQM_4IC as ::core::ffi::c_int,
                );
                bs_write1(s, 0 as crate::stdlib::uint32_t);
                bs_write1(s, 0 as crate::stdlib::uint32_t);
                bs_write1(s, 0 as crate::stdlib::uint32_t);
                scaling_list_write(
                    s,
                    sps,
                    crate::src::common::set::CQM_8IY as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int,
                );
                bs_write1(s, 0 as crate::stdlib::uint32_t);
                if (*sps).i_chroma_format_idc
                    == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                {
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_8IC as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int,
                    );
                    bs_write1(s, 0 as crate::stdlib::uint32_t);
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_8IC as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int,
                    );
                    bs_write1(s, 0 as crate::stdlib::uint32_t);
                }
            }
        }
        bs_write_ue_big(
            s,
            ((*sps).i_log2_max_frame_num - 4 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        bs_write_ue_big(s, (*sps).i_poc_type as ::core::ffi::c_uint);
        if (*sps).i_poc_type == 0 as ::core::ffi::c_int {
            bs_write_ue_big(
                s,
                ((*sps).i_log2_max_poc_lsb - 4 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        }
        bs_write_ue_big(s, (*sps).i_num_ref_frames as ::core::ffi::c_uint);
        bs_write1(
            s,
            (*sps).b_gaps_in_frame_num_value_allowed as crate::stdlib::uint32_t,
        );
        bs_write_ue_big(
            s,
            ((*sps).i_mb_width - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        bs_write_ue_big(
            s,
            (((*sps).i_mb_height >> ((*sps).b_frame_mbs_only == 0) as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        bs_write1(s, (*sps).b_frame_mbs_only as crate::stdlib::uint32_t);
        if (*sps).b_frame_mbs_only == 0 {
            bs_write1(
                s,
                (*sps).b_mb_adaptive_frame_field as crate::stdlib::uint32_t,
            );
        }
        bs_write1(s, (*sps).b_direct8x8_inference as crate::stdlib::uint32_t);
        bs_write1(s, (*sps).b_crop as crate::stdlib::uint32_t);
        if (*sps).b_crop != 0 {
            let mut h_shift: ::core::ffi::c_int = ((*sps).i_chroma_format_idc
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                || (*sps).i_chroma_format_idc
                    == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            let mut v_shift: ::core::ffi::c_int = ((*sps).i_chroma_format_idc
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int
                + ((*sps).b_frame_mbs_only == 0) as ::core::ffi::c_int;
            bs_write_ue_big(s, ((*sps).crop.i_left >> h_shift) as ::core::ffi::c_uint);
            bs_write_ue_big(s, ((*sps).crop.i_right >> h_shift) as ::core::ffi::c_uint);
            bs_write_ue_big(s, ((*sps).crop.i_top >> v_shift) as ::core::ffi::c_uint);
            bs_write_ue_big(s, ((*sps).crop.i_bottom >> v_shift) as ::core::ffi::c_uint);
        }
        bs_write1(s, (*sps).b_vui as crate::stdlib::uint32_t);
        if (*sps).b_vui != 0 {
            bs_write1(
                s,
                (*sps).vui.b_aspect_ratio_info_present as crate::stdlib::uint32_t,
            );
            if (*sps).vui.b_aspect_ratio_info_present != 0 {
                let mut i: ::core::ffi::c_int = 0;
                static mut sar: [C2Rust_Unnamed_465; 17] = [
                    C2Rust_Unnamed_465 {
                        w: 1 as crate::stdlib::uint8_t,
                        h: 1 as crate::stdlib::uint8_t,
                        sar: 1 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 12 as crate::stdlib::uint8_t,
                        h: 11 as crate::stdlib::uint8_t,
                        sar: 2 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 10 as crate::stdlib::uint8_t,
                        h: 11 as crate::stdlib::uint8_t,
                        sar: 3 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 16 as crate::stdlib::uint8_t,
                        h: 11 as crate::stdlib::uint8_t,
                        sar: 4 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 40 as crate::stdlib::uint8_t,
                        h: 33 as crate::stdlib::uint8_t,
                        sar: 5 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 24 as crate::stdlib::uint8_t,
                        h: 11 as crate::stdlib::uint8_t,
                        sar: 6 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 20 as crate::stdlib::uint8_t,
                        h: 11 as crate::stdlib::uint8_t,
                        sar: 7 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 32 as crate::stdlib::uint8_t,
                        h: 11 as crate::stdlib::uint8_t,
                        sar: 8 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 80 as crate::stdlib::uint8_t,
                        h: 33 as crate::stdlib::uint8_t,
                        sar: 9 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 18 as crate::stdlib::uint8_t,
                        h: 11 as crate::stdlib::uint8_t,
                        sar: 10 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 15 as crate::stdlib::uint8_t,
                        h: 11 as crate::stdlib::uint8_t,
                        sar: 11 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 64 as crate::stdlib::uint8_t,
                        h: 33 as crate::stdlib::uint8_t,
                        sar: 12 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 160 as crate::stdlib::uint8_t,
                        h: 99 as crate::stdlib::uint8_t,
                        sar: 13 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 4 as crate::stdlib::uint8_t,
                        h: 3 as crate::stdlib::uint8_t,
                        sar: 14 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 3 as crate::stdlib::uint8_t,
                        h: 2 as crate::stdlib::uint8_t,
                        sar: 15 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 2 as crate::stdlib::uint8_t,
                        h: 1 as crate::stdlib::uint8_t,
                        sar: 16 as crate::stdlib::uint8_t,
                    },
                    C2Rust_Unnamed_465 {
                        w: 0 as crate::stdlib::uint8_t,
                        h: 0 as crate::stdlib::uint8_t,
                        sar: 255 as crate::stdlib::uint8_t,
                    },
                ];
                i = 0 as ::core::ffi::c_int;
                while sar[i as usize].sar as ::core::ffi::c_int != 255 as ::core::ffi::c_int {
                    if sar[i as usize].w as ::core::ffi::c_int == (*sps).vui.i_sar_width
                        && sar[i as usize].h as ::core::ffi::c_int == (*sps).vui.i_sar_height
                    {
                        break;
                    }
                    i += 1;
                }
                bs_write(
                    s,
                    8 as ::core::ffi::c_int,
                    sar[i as usize].sar as crate::stdlib::uint32_t,
                );
                if sar[i as usize].sar as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
                    bs_write(
                        s,
                        16 as ::core::ffi::c_int,
                        (*sps).vui.i_sar_width as crate::stdlib::uint32_t,
                    );
                    bs_write(
                        s,
                        16 as ::core::ffi::c_int,
                        (*sps).vui.i_sar_height as crate::stdlib::uint32_t,
                    );
                }
            }
            bs_write1(
                s,
                (*sps).vui.b_overscan_info_present as crate::stdlib::uint32_t,
            );
            if (*sps).vui.b_overscan_info_present != 0 {
                bs_write1(s, (*sps).vui.b_overscan_info as crate::stdlib::uint32_t);
            }
            bs_write1(
                s,
                (*sps).vui.b_signal_type_present as crate::stdlib::uint32_t,
            );
            if (*sps).vui.b_signal_type_present != 0 {
                bs_write(
                    s,
                    3 as ::core::ffi::c_int,
                    (*sps).vui.i_vidformat as crate::stdlib::uint32_t,
                );
                bs_write1(s, (*sps).vui.b_fullrange as crate::stdlib::uint32_t);
                bs_write1(
                    s,
                    (*sps).vui.b_color_description_present as crate::stdlib::uint32_t,
                );
                if (*sps).vui.b_color_description_present != 0 {
                    bs_write(
                        s,
                        8 as ::core::ffi::c_int,
                        (*sps).vui.i_colorprim as crate::stdlib::uint32_t,
                    );
                    bs_write(
                        s,
                        8 as ::core::ffi::c_int,
                        (*sps).vui.i_transfer as crate::stdlib::uint32_t,
                    );
                    bs_write(
                        s,
                        8 as ::core::ffi::c_int,
                        (*sps).vui.i_colmatrix as crate::stdlib::uint32_t,
                    );
                }
            }
            bs_write1(
                s,
                (*sps).vui.b_chroma_loc_info_present as crate::stdlib::uint32_t,
            );
            if (*sps).vui.b_chroma_loc_info_present != 0 {
                bs_write_ue_big(s, (*sps).vui.i_chroma_loc_top as ::core::ffi::c_uint);
                bs_write_ue_big(s, (*sps).vui.i_chroma_loc_bottom as ::core::ffi::c_uint);
            }
            bs_write1(
                s,
                (*sps).vui.b_timing_info_present as crate::stdlib::uint32_t,
            );
            if (*sps).vui.b_timing_info_present != 0 {
                bs_write32(s, (*sps).vui.i_num_units_in_tick);
                bs_write32(s, (*sps).vui.i_time_scale);
                bs_write1(s, (*sps).vui.b_fixed_frame_rate as crate::stdlib::uint32_t);
            }
            bs_write1(
                s,
                (*sps).vui.b_nal_hrd_parameters_present as crate::stdlib::uint32_t,
            );
            if (*sps).vui.b_nal_hrd_parameters_present != 0 {
                bs_write_ue_big(
                    s,
                    ((*sps).vui.hrd.i_cpb_cnt - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                );
                bs_write(
                    s,
                    4 as ::core::ffi::c_int,
                    (*sps).vui.hrd.i_bit_rate_scale as crate::stdlib::uint32_t,
                );
                bs_write(
                    s,
                    4 as ::core::ffi::c_int,
                    (*sps).vui.hrd.i_cpb_size_scale as crate::stdlib::uint32_t,
                );
                bs_write_ue_big(
                    s,
                    ((*sps).vui.hrd.i_bit_rate_value - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
                bs_write_ue_big(
                    s,
                    ((*sps).vui.hrd.i_cpb_size_value - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
                bs_write1(s, (*sps).vui.hrd.b_cbr_hrd as crate::stdlib::uint32_t);
                bs_write(
                    s,
                    5 as ::core::ffi::c_int,
                    ((*sps).vui.hrd.i_initial_cpb_removal_delay_length - 1 as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                bs_write(
                    s,
                    5 as ::core::ffi::c_int,
                    ((*sps).vui.hrd.i_cpb_removal_delay_length - 1 as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                bs_write(
                    s,
                    5 as ::core::ffi::c_int,
                    ((*sps).vui.hrd.i_dpb_output_delay_length - 1 as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                bs_write(
                    s,
                    5 as ::core::ffi::c_int,
                    (*sps).vui.hrd.i_time_offset_length as crate::stdlib::uint32_t,
                );
            }
            bs_write1(
                s,
                (*sps).vui.b_vcl_hrd_parameters_present as crate::stdlib::uint32_t,
            );
            if (*sps).vui.b_nal_hrd_parameters_present != 0
                || (*sps).vui.b_vcl_hrd_parameters_present != 0
            {
                bs_write1(s, 0 as crate::stdlib::uint32_t);
            }
            bs_write1(
                s,
                (*sps).vui.b_pic_struct_present as crate::stdlib::uint32_t,
            );
            bs_write1(
                s,
                (*sps).vui.b_bitstream_restriction as crate::stdlib::uint32_t,
            );
            if (*sps).vui.b_bitstream_restriction != 0 {
                bs_write1(
                    s,
                    (*sps).vui.b_motion_vectors_over_pic_boundaries as crate::stdlib::uint32_t,
                );
                bs_write_ue_big(
                    s,
                    (*sps).vui.i_max_bytes_per_pic_denom as ::core::ffi::c_uint,
                );
                bs_write_ue_big(s, (*sps).vui.i_max_bits_per_mb_denom as ::core::ffi::c_uint);
                bs_write_ue_big(
                    s,
                    (*sps).vui.i_log2_max_mv_length_horizontal as ::core::ffi::c_uint,
                );
                bs_write_ue_big(
                    s,
                    (*sps).vui.i_log2_max_mv_length_vertical as ::core::ffi::c_uint,
                );
                bs_write_ue_big(s, (*sps).vui.i_num_reorder_frames as ::core::ffi::c_uint);
                bs_write_ue_big(
                    s,
                    (*sps).vui.i_max_dec_frame_buffering as ::core::ffi::c_uint,
                );
            }
        }
        bs_rbsp_trailing(s);
        bs_flush(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pps_init(
    mut pps: *mut crate::src::common::set::x264_pps_t,
    mut i_id: ::core::ffi::c_int,
    mut param: *mut crate::x264_h::x264_param_t,
    mut sps: *mut crate::src::common::set::x264_sps_t,
) {
    unsafe {
        (*pps).i_id = i_id;
        (*pps).i_sps_id = (*sps).i_id;
        (*pps).b_cabac = (*param).b_cabac;
        (*pps).b_pic_order =
            ((*param).i_avcintra_class == 0 && (*param).b_interlaced != 0) as ::core::ffi::c_int;
        (*pps).i_num_slice_groups = 1 as ::core::ffi::c_int;
        (*pps).i_num_ref_idx_l0_default_active = (*param).i_frame_reference;
        (*pps).i_num_ref_idx_l1_default_active = 1 as ::core::ffi::c_int;
        (*pps).b_weighted_pred =
            ((*param).analyse.i_weighted_pred > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        (*pps).b_weighted_bipred = if (*param).analyse.b_weighted_bipred != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        (*pps).i_pic_init_qp = if (*param).rc.i_rc_method == crate::x264_h::X264_RC_ABR
            || (*param).b_stitchable != 0
        {
            26 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET
        } else if (*param).rc.i_qp_constant
            < 51 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
        {
            (*param).rc.i_qp_constant
        } else {
            51 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
        };
        (*pps).i_pic_init_qs = 26 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET;
        (*pps).i_chroma_qp_index_offset = (*param).analyse.i_chroma_qp_offset;
        (*pps).b_deblocking_filter_control = 1 as ::core::ffi::c_int;
        (*pps).b_constrained_intra_pred = (*param).b_constrained_intra;
        (*pps).b_redundant_pic_cnt = 0 as ::core::ffi::c_int;
        (*pps).b_transform_8x8_mode = if (*param).analyse.b_transform_8x8 != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pps_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut sps: *mut crate::src::common::set::x264_sps_t,
    mut pps: *mut crate::src::common::set::x264_pps_t,
) {
    unsafe {
        bs_realign(s);
        bs_write_ue_big(s, (*pps).i_id as ::core::ffi::c_uint);
        bs_write_ue_big(s, (*pps).i_sps_id as ::core::ffi::c_uint);
        bs_write1(s, (*pps).b_cabac as crate::stdlib::uint32_t);
        bs_write1(s, (*pps).b_pic_order as crate::stdlib::uint32_t);
        bs_write_ue_big(
            s,
            ((*pps).i_num_slice_groups - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        bs_write_ue_big(
            s,
            ((*pps).i_num_ref_idx_l0_default_active - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
        bs_write_ue_big(
            s,
            ((*pps).i_num_ref_idx_l1_default_active - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
        bs_write1(s, (*pps).b_weighted_pred as crate::stdlib::uint32_t);
        bs_write(
            s,
            2 as ::core::ffi::c_int,
            (*pps).b_weighted_bipred as crate::stdlib::uint32_t,
        );
        bs_write_se(
            s,
            (*pps).i_pic_init_qp
                - 26 as ::core::ffi::c_int
                - crate::src::common::common::QP_BD_OFFSET,
        );
        bs_write_se(
            s,
            (*pps).i_pic_init_qs
                - 26 as ::core::ffi::c_int
                - crate::src::common::common::QP_BD_OFFSET,
        );
        bs_write_se(s, (*pps).i_chroma_qp_index_offset);
        bs_write1(
            s,
            (*pps).b_deblocking_filter_control as crate::stdlib::uint32_t,
        );
        bs_write1(
            s,
            (*pps).b_constrained_intra_pred as crate::stdlib::uint32_t,
        );
        bs_write1(s, (*pps).b_redundant_pic_cnt as crate::stdlib::uint32_t);
        let mut b_scaling_list: ::core::ffi::c_int = ((*sps).b_avcintra_hd == 0
            && (*sps).i_cqm_preset != crate::x264_h::X264_CQM_FLAT)
            as ::core::ffi::c_int;
        if (*pps).b_transform_8x8_mode != 0 || b_scaling_list != 0 {
            bs_write1(s, (*pps).b_transform_8x8_mode as crate::stdlib::uint32_t);
            bs_write1(s, b_scaling_list as crate::stdlib::uint32_t);
            if b_scaling_list != 0 {
                scaling_list_write(
                    s,
                    sps,
                    crate::src::common::set::CQM_4IY as ::core::ffi::c_int,
                );
                scaling_list_write(
                    s,
                    sps,
                    crate::src::common::set::CQM_4IC as ::core::ffi::c_int,
                );
                if (*sps).b_avcintra_4k != 0 {
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_4IC as ::core::ffi::c_int,
                    );
                    bs_write1(s, 0 as crate::stdlib::uint32_t);
                    bs_write1(s, 0 as crate::stdlib::uint32_t);
                    bs_write1(s, 0 as crate::stdlib::uint32_t);
                } else {
                    bs_write1(s, 0 as crate::stdlib::uint32_t);
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_4PY as ::core::ffi::c_int,
                    );
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_4PC as ::core::ffi::c_int,
                    );
                    bs_write1(s, 0 as crate::stdlib::uint32_t);
                }
                if (*pps).b_transform_8x8_mode != 0 {
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_8IY as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int,
                    );
                    if (*sps).b_avcintra_4k != 0 {
                        bs_write1(s, 0 as crate::stdlib::uint32_t);
                    } else {
                        scaling_list_write(
                            s,
                            sps,
                            crate::src::common::set::CQM_8PY as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int,
                        );
                    }
                    if (*sps).i_chroma_format_idc
                        == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    {
                        scaling_list_write(
                            s,
                            sps,
                            crate::src::common::set::CQM_8IC as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int,
                        );
                        scaling_list_write(
                            s,
                            sps,
                            crate::src::common::set::CQM_8PC as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int,
                        );
                        bs_write1(s, 0 as crate::stdlib::uint32_t);
                        bs_write1(s, 0 as crate::stdlib::uint32_t);
                    }
                }
            }
            bs_write_se(s, (*pps).i_chroma_qp_index_offset);
        }
        bs_rbsp_trailing(s);
        bs_flush(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_recovery_point_write(
    mut _h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut recovery_frame_cnt: ::core::ffi::c_int,
) {
    unsafe {
        let mut q: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf: [crate::stdlib::uint8_t; 100] = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = 0 as crate::stdlib::uint32_t;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            100 as ::core::ffi::c_int,
        );
        bs_realign(&raw mut q);
        bs_write_ue_big(&raw mut q, recovery_frame_cnt as ::core::ffi::c_uint);
        bs_write1(&raw mut q, 1 as crate::stdlib::uint32_t);
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        bs_write(
            &raw mut q,
            2 as ::core::ffi::c_int,
            0 as crate::stdlib::uint32_t,
        );
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8 as ::core::ffi::c_int,
            crate::src::common::base::SEI_RECOVERY_POINT as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_version_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) -> ::core::ffi::c_int {
    unsafe {
        static mut uuid: [crate::stdlib::uint8_t; 16] = [
            0xdc as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0x45 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xe9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xbd as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xe6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xd9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0x48 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xb7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0x96 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0x2c as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xd8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0x20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xd9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0x23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xee as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0xef as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ];
        let mut opts: *mut ::core::ffi::c_char = crate::src::common::base::x264_param2string(
            &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
            0 as ::core::ffi::c_int,
        );
        let mut payload: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut length: ::core::ffi::c_int = 0;
        if opts.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        payload = crate::src::common::base::x264_malloc(
            (200 as crate::__stddef_size_t_h::size_t).wrapping_add(crate::stdlib::strlen(opts))
                as crate::stdlib::int64_t,
        ) as *mut ::core::ffi::c_char;
        if payload.is_null() {
            crate::src::common::base::x264_free(opts as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        } else {
            crate::stdlib::memcpy(
                payload as *mut ::core::ffi::c_void,
                &raw const uuid as *const crate::stdlib::uint8_t as *const ::core::ffi::c_void,
                16 as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::sprintf(
                payload.offset(16 as ::core::ffi::c_int as isize),
                b"x264 - core %d%s - H.264/MPEG-4 AVC codec - Copy%s 2003-2025 - http://www.videolan.org/x264.html - options: %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                crate::x264_h::X264_BUILD,
                crate::x264_config_h::X264_VERSION.as_ptr(),
                if crate::config_h::HAVE_GPL != 0 {
                    b"left\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"right\0".as_ptr() as *const ::core::ffi::c_char
                },
                opts,
            );
            length = crate::stdlib::strlen(payload)
                .wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                as ::core::ffi::c_int;
            x264_8_sei_write(
                s,
                payload as *mut crate::stdlib::uint8_t,
                length,
                crate::src::common::base::SEI_USER_DATA_UNREGISTERED as ::core::ffi::c_int,
            );
            crate::src::common::base::x264_free(opts as *mut ::core::ffi::c_void);
            crate::src::common::base::x264_free(payload as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_buffering_period_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut sps: *mut crate::src::common::set::x264_sps_t =
            &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t;
        let mut q: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf: [crate::stdlib::uint8_t; 100] = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = 0 as crate::stdlib::uint32_t;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            100 as ::core::ffi::c_int,
        );
        bs_realign(&raw mut q);
        bs_write_ue_big(&raw mut q, (*sps).i_id as ::core::ffi::c_uint);
        if (*sps).vui.b_nal_hrd_parameters_present != 0 {
            bs_write(
                &raw mut q,
                (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
                (*h).initial_cpb_removal_delay as crate::stdlib::uint32_t,
            );
            bs_write(
                &raw mut q,
                (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
                (*h).initial_cpb_removal_delay_offset as crate::stdlib::uint32_t,
            );
        }
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8 as ::core::ffi::c_int,
            crate::src::common::base::SEI_BUFFERING_PERIOD as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_pic_timing_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut sps: *mut crate::src::common::set::x264_sps_t =
            &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t;
        let mut q: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf: [crate::stdlib::uint8_t; 100] = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = 0 as crate::stdlib::uint32_t;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            100 as ::core::ffi::c_int,
        );
        bs_realign(&raw mut q);
        if (*sps).vui.b_nal_hrd_parameters_present != 0
            || (*sps).vui.b_vcl_hrd_parameters_present != 0
        {
            bs_write(
                &raw mut q,
                (*sps).vui.hrd.i_cpb_removal_delay_length,
                ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset) as crate::stdlib::uint32_t,
            );
            bs_write(
                &raw mut q,
                (*sps).vui.hrd.i_dpb_output_delay_length,
                (*(*h).fenc).i_dpb_output_delay as crate::stdlib::uint32_t,
            );
        }
        if (*sps).vui.b_pic_struct_present != 0 {
            bs_write(
                &raw mut q,
                4 as ::core::ffi::c_int,
                ((*(*h).fenc).i_pic_struct - 1 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < num_clock_ts[(*(*h).fenc).i_pic_struct as usize] as ::core::ffi::c_int {
                bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
                i += 1;
            }
        }
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8 as ::core::ffi::c_int,
            crate::src::common::base::SEI_PIC_TIMING as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_frame_packing_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut quincunx_sampling_flag: ::core::ffi::c_int =
            ((*h).param.i_frame_packing == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut q: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf: [crate::stdlib::uint8_t; 100] = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = 0 as crate::stdlib::uint32_t;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            100 as ::core::ffi::c_int,
        );
        bs_realign(&raw mut q);
        bs_write_ue_big(&raw mut q, 0 as ::core::ffi::c_uint);
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        bs_write(
            &raw mut q,
            7 as ::core::ffi::c_int,
            (*h).param.i_frame_packing as crate::stdlib::uint32_t,
        );
        bs_write1(
            &raw mut q,
            quincunx_sampling_flag as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            6 as ::core::ffi::c_int,
            ((*h).param.i_frame_packing != 6 as ::core::ffi::c_int) as ::core::ffi::c_int
                as crate::stdlib::uint32_t,
        );
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        bs_write1(
            &raw mut q,
            ((*h).param.i_frame_packing == 5 as ::core::ffi::c_int
                && (*(*h).fenc).i_frame & 1 as ::core::ffi::c_int == 0)
                as ::core::ffi::c_int as crate::stdlib::uint32_t,
        );
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        if quincunx_sampling_flag == 0 as ::core::ffi::c_int
            && (*h).param.i_frame_packing != 5 as ::core::ffi::c_int
        {
            bs_write(
                &raw mut q,
                4 as ::core::ffi::c_int,
                0 as crate::stdlib::uint32_t,
            );
            bs_write(
                &raw mut q,
                4 as ::core::ffi::c_int,
                0 as crate::stdlib::uint32_t,
            );
            bs_write(
                &raw mut q,
                4 as ::core::ffi::c_int,
                0 as crate::stdlib::uint32_t,
            );
            bs_write(
                &raw mut q,
                4 as ::core::ffi::c_int,
                0 as crate::stdlib::uint32_t,
            );
        }
        bs_write(
            &raw mut q,
            8 as ::core::ffi::c_int,
            0 as crate::stdlib::uint32_t,
        );
        bs_write_ue_big(
            &raw mut q,
            ((*h).param.i_frame_packing != 5 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8 as ::core::ffi::c_int,
            crate::src::common::base::SEI_FRAME_PACKING as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_mastering_display_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf: [crate::stdlib::uint8_t; 100] = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = 0 as crate::stdlib::uint32_t;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            100 as ::core::ffi::c_int,
        );
        bs_realign(&raw mut q);
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.mastering_display.i_green_x as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.mastering_display.i_green_y as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.mastering_display.i_blue_x as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.mastering_display.i_blue_y as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.mastering_display.i_red_x as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.mastering_display.i_red_y as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.mastering_display.i_white_x as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.mastering_display.i_white_y as crate::stdlib::uint32_t,
        );
        bs_write32(
            &raw mut q,
            (*h).param.mastering_display.i_display_max as crate::stdlib::uint32_t,
        );
        bs_write32(
            &raw mut q,
            (*h).param.mastering_display.i_display_min as crate::stdlib::uint32_t,
        );
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8 as ::core::ffi::c_int,
            crate::src::common::base::SEI_MASTERING_DISPLAY as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_content_light_level_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf: [crate::stdlib::uint8_t; 100] = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = 0 as crate::stdlib::uint32_t;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            100 as ::core::ffi::c_int,
        );
        bs_realign(&raw mut q);
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.content_light_level.i_max_cll as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16 as ::core::ffi::c_int,
            (*h).param.content_light_level.i_max_fall as crate::stdlib::uint32_t,
        );
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8 as ::core::ffi::c_int,
            crate::src::common::base::SEI_CONTENT_LIGHT_LEVEL as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_alternative_transfer_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf: [crate::stdlib::uint8_t; 100] = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = 0 as crate::stdlib::uint32_t;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            100 as ::core::ffi::c_int,
        );
        bs_realign(&raw mut q);
        bs_write(
            &raw mut q,
            8 as ::core::ffi::c_int,
            (*h).param.i_alternative_transfer as crate::stdlib::uint32_t,
        );
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8 as ::core::ffi::c_int,
            crate::src::common::base::SEI_ALTERNATIVE_TRANSFER as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_filler_write(
    mut _h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut filler: ::core::ffi::c_int,
) {
    unsafe {
        bs_realign(s);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < filler {
            bs_write(s, 8 as ::core::ffi::c_int, 0xff as crate::stdlib::uint32_t);
            i += 1;
        }
        bs_rbsp_trailing(s);
        bs_flush(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_dec_ref_pic_marking_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut sh: *mut crate::src::common::common::x264_slice_header_t = &raw mut (*h).sh_backup;
        let mut q: crate::src::common::bitstream::bs_t = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf: [crate::stdlib::uint8_t; 100] = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = 0 as crate::stdlib::uint32_t;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            100 as ::core::ffi::c_int,
        );
        bs_realign(&raw mut q);
        bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        bs_write_ue_big(&raw mut q, (*sh).i_frame_num as ::core::ffi::c_uint);
        if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).b_frame_mbs_only == 0
        {
            bs_write1(&raw mut q, 0 as crate::stdlib::uint32_t);
        }
        bs_write1(
            &raw mut q,
            ((*sh).i_mmco_command_count > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as crate::stdlib::uint32_t,
        );
        if (*sh).i_mmco_command_count > 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*sh).i_mmco_command_count {
                bs_write_ue_big(&raw mut q, 1 as ::core::ffi::c_uint);
                bs_write_ue_big(
                    &raw mut q,
                    ((*sh).mmco[i as usize].i_difference_of_pic_nums - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
                i += 1;
            }
            bs_write_ue_big(&raw mut q, 0 as ::core::ffi::c_uint);
        }
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8 as ::core::ffi::c_int,
            crate::src::common::base::SEI_DEC_REF_PIC_MARKING as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_avcintra_umid_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut _s: *mut crate::src::common::bitstream::bs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut data: [crate::stdlib::uint8_t; 512] = [0; 512];
        let mut msg: *const ::core::ffi::c_char = b"UMID\0".as_ptr() as *const ::core::ffi::c_char;
        let len: ::core::ffi::c_int = 497 as ::core::ffi::c_int;
        crate::stdlib::memset(
            &raw mut data as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            len as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut data as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            &raw const avcintra_uuid as *const crate::stdlib::uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>()
                as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            (&raw mut data as *mut crate::stdlib::uint8_t).offset(16 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            msg as *const ::core::ffi::c_void,
            crate::stdlib::strlen(msg),
        );
        data[20 as ::core::ffi::c_int as usize] = 0x13 as crate::stdlib::uint8_t;
        data[26 as ::core::ffi::c_int as usize] = 0 as crate::stdlib::uint8_t;
        data[25 as ::core::ffi::c_int as usize] = data[26 as ::core::ffi::c_int as usize];
        data[23 as ::core::ffi::c_int as usize] = data[25 as ::core::ffi::c_int as usize];
        data[22 as ::core::ffi::c_int as usize] = data[23 as ::core::ffi::c_int as usize];
        data[28 as ::core::ffi::c_int as usize] = 0x14 as crate::stdlib::uint8_t;
        data[34 as ::core::ffi::c_int as usize] = 0 as crate::stdlib::uint8_t;
        data[33 as ::core::ffi::c_int as usize] = data[34 as ::core::ffi::c_int as usize];
        data[31 as ::core::ffi::c_int as usize] = data[33 as ::core::ffi::c_int as usize];
        data[30 as ::core::ffi::c_int as usize] = data[31 as ::core::ffi::c_int as usize];
        data[36 as ::core::ffi::c_int as usize] = 0x60 as crate::stdlib::uint8_t;
        data[41 as ::core::ffi::c_int as usize] = 0x22 as crate::stdlib::uint8_t;
        data[60 as ::core::ffi::c_int as usize] = 0x62 as crate::stdlib::uint8_t;
        data[66 as ::core::ffi::c_int as usize] = 0 as crate::stdlib::uint8_t;
        data[65 as ::core::ffi::c_int as usize] = data[66 as ::core::ffi::c_int as usize];
        data[63 as ::core::ffi::c_int as usize] = data[65 as ::core::ffi::c_int as usize];
        data[62 as ::core::ffi::c_int as usize] = data[63 as ::core::ffi::c_int as usize];
        data[68 as ::core::ffi::c_int as usize] = 0x63 as crate::stdlib::uint8_t;
        data[74 as ::core::ffi::c_int as usize] = 0 as crate::stdlib::uint8_t;
        data[73 as ::core::ffi::c_int as usize] = data[74 as ::core::ffi::c_int as usize];
        data[71 as ::core::ffi::c_int as usize] = data[73 as ::core::ffi::c_int as usize];
        data[70 as ::core::ffi::c_int as usize] = data[71 as ::core::ffi::c_int as usize];
        x264_8_sei_write(
            &raw mut (*h).out.bs,
            &raw mut data as *mut crate::stdlib::uint8_t,
            len,
            crate::src::common::base::SEI_USER_DATA_UNREGISTERED as ::core::ffi::c_int,
        );
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_avcintra_vanc_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut _s: *mut crate::src::common::bitstream::bs_t,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut data: [crate::stdlib::uint8_t; 6000] = [0; 6000];
        let mut msg: *const ::core::ffi::c_char = b"VANC\0".as_ptr() as *const ::core::ffi::c_char;
        if len < 0 as ::core::ffi::c_int
            || len as ::core::ffi::c_uint as usize
                > ::core::mem::size_of::<[crate::stdlib::uint8_t; 6000]>() as usize
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"AVC-Intra SEI is too large (%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                len,
            );
            return -(1 as ::core::ffi::c_int);
        }
        crate::stdlib::memset(
            &raw mut data as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            len as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut data as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            &raw const avcintra_uuid as *const crate::stdlib::uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>()
                as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            (&raw mut data as *mut crate::stdlib::uint8_t).offset(16 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            msg as *const ::core::ffi::c_void,
            crate::stdlib::strlen(msg),
        );
        x264_8_sei_write(
            &raw mut (*h).out.bs,
            &raw mut data as *mut crate::stdlib::uint8_t,
            len,
            crate::src::common::base::SEI_USER_DATA_UNREGISTERED as ::core::ffi::c_int,
        );
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_validate_levels(
    mut h: *mut crate::src::common::common::x264_t,
    mut verbose: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mbs: ::core::ffi::c_int =
            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_mb_width
                * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_mb_height;
        let mut dpb: ::core::ffi::c_int = mbs
            * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .i_max_dec_frame_buffering;
        let mut cbp_factor: ::core::ffi::c_int = if (*(&raw mut (*h).sps
            as *mut crate::src::common::set::x264_sps_t))
            .i_profile_idc
            >= crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int
        {
            16 as ::core::ffi::c_int
        } else if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_profile_idc
            == crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int
        {
            12 as ::core::ffi::c_int
        } else if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_profile_idc
            == crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
        {
            5 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
        let mut l: *const crate::x264_h::x264_level_t =
            &raw const crate::src::common::tables::x264_levels
                as *const crate::x264_h::x264_level_t;
        while (*l).level_idc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && (*l).level_idc as ::core::ffi::c_int != (*h).param.i_level_idc
        {
            l = l.offset(1);
        }
        if (*l).frame_size < mbs as crate::stdlib::int32_t
            || ((*l).frame_size * 8 as crate::stdlib::int32_t)
                < (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_mb_width
                    as crate::stdlib::int32_t
                    * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_mb_width
                        as crate::stdlib::int32_t
            || ((*l).frame_size * 8 as crate::stdlib::int32_t)
                < (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_mb_height
                    as crate::stdlib::int32_t
                    * (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_mb_height
                        as crate::stdlib::int32_t
        {
            if verbose != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"frame MB size (%dx%d) > level limit (%d)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_mb_width,
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_mb_height,
                    (*l).frame_size,
                );
            }
            ret = 1 as ::core::ffi::c_int;
        }
        if dpb as crate::stdlib::int32_t > (*l).dpb {
            if verbose != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"DPB size (%d frames, %d mbs) > level limit (%d frames, %d mbs)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_max_dec_frame_buffering,
                    dpb,
                    (*l).dpb / mbs as crate::stdlib::int32_t,
                    (*l).dpb,
                );
            }
            ret = 1 as ::core::ffi::c_int;
        }
        if (*h).param.rc.i_vbv_max_bitrate as crate::stdlib::int32_t
            > (*l).bitrate * cbp_factor as crate::stdlib::int32_t / 4 as crate::stdlib::int32_t
        {
            if verbose != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"VBV bitrate (%ld) > level limit (%d)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.rc.i_vbv_max_bitrate as crate::stdlib::int64_t,
                    (*l).bitrate * cbp_factor as crate::stdlib::int32_t
                        / 4 as crate::stdlib::int32_t,
                );
            }
            ret = 1 as ::core::ffi::c_int;
        }
        if (*h).param.rc.i_vbv_buffer_size as crate::stdlib::int32_t
            > (*l).cpb * cbp_factor as crate::stdlib::int32_t / 4 as crate::stdlib::int32_t
        {
            if verbose != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"VBV buffer (%ld) > level limit (%d)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.rc.i_vbv_buffer_size as crate::stdlib::int64_t,
                    (*l).cpb * cbp_factor as crate::stdlib::int32_t / 4 as crate::stdlib::int32_t,
                );
            }
            ret = 1 as ::core::ffi::c_int;
        }
        if (*h).param.analyse.i_mv_range > (*l).mv_range as ::core::ffi::c_int {
            if verbose != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"MV range (%ld) > level limit (%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*h).param.analyse.i_mv_range as crate::stdlib::int64_t,
                    (*l).mv_range as ::core::ffi::c_int,
                );
            }
            ret = 1 as ::core::ffi::c_int;
        }
        if (*h).param.b_interlaced > ((*l).frame_only == 0) as ::core::ffi::c_int {
            if verbose != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"interlaced (%ld) > level limit (%d)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.b_interlaced as crate::stdlib::int64_t,
                    ((*l).frame_only == 0) as ::core::ffi::c_int,
                );
            }
            ret = 1 as ::core::ffi::c_int;
        }
        if (*h).param.b_fake_interlaced > ((*l).frame_only == 0) as ::core::ffi::c_int {
            if verbose != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"fake interlaced (%ld) > level limit (%d)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.b_fake_interlaced as crate::stdlib::int64_t,
                    ((*l).frame_only == 0) as ::core::ffi::c_int,
                );
            }
            ret = 1 as ::core::ffi::c_int;
        }
        if (*h).param.i_fps_den > 0 as crate::stdlib::uint32_t {
            if mbs as crate::stdlib::int64_t * (*h).param.i_fps_num as crate::stdlib::int64_t
                / (*h).param.i_fps_den as crate::stdlib::int64_t
                > (*l).mbps as crate::stdlib::int64_t
            {
                if verbose != 0 {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_WARNING_1,
                        b"MB rate (%ld) > level limit (%d)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        mbs as crate::stdlib::int64_t
                            * (*h).param.i_fps_num as crate::stdlib::int64_t
                            / (*h).param.i_fps_den as crate::stdlib::int64_t,
                        (*l).mbps,
                    );
                }
                ret = 1 as ::core::ffi::c_int;
            }
        }
        return ret;
    }
}
