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
    pub unsafe extern "C" fn bs_align_0(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            bs_write(
                s,
                (*s).i_left & 7 as ::core::ffi::c_int,
                0 as crate::stdlib::uint32_t,
            );
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
    pub unsafe extern "C" fn bs_write_ue(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut val: ::core::ffi::c_int,
    ) {
        unsafe {
            bs_write(
                s,
                x264_ue_size_tab[(val + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int,
                (val + 1 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
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
    pub unsafe extern "C" fn bs_write_te(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut x: ::core::ffi::c_int,
        mut val: ::core::ffi::c_int,
    ) {
        unsafe {
            if x == 1 as ::core::ffi::c_int {
                bs_write1(
                    s,
                    (1 as ::core::ffi::c_int ^ val) as crate::stdlib::uint32_t,
                );
            } else {
                bs_write_ue(s, val);
            };
        }
    }
    use crate::src::encoder::cavlc::osdep_h::endian_fix;
    use crate::src::encoder::cavlc::osdep_h::endian_fix32;
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
    #[inline(always)]
    pub unsafe extern "C" fn x264_mb_predict_non_zero_code(
        mut h: *mut crate::src::common::common::x264_t,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            let za: ::core::ffi::c_int = (*h).mb.cache.non_zero_count[(x264_scan8[idx as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int;
            let zb: ::core::ffi::c_int = (*h).mb.cache.non_zero_count[(x264_scan8[idx as usize]
                as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int;
            let mut i_ret: ::core::ffi::c_int = za + zb;
            if i_ret < 0x80 as ::core::ffi::c_int {
                i_ret = i_ret + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
            }
            return i_ret & 0x7f as ::core::ffi::c_int;
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
    use crate::src::encoder::cavlc::base_h::x264_scan8;
    use crate::src::encoder::cavlc::predict_h::x264_mb_pred_mode4x4_fix;
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
use crate::src::encoder::cavlc::base_h::x264_scan8;
use crate::src::encoder::cavlc::bitstream_h::bs_align_0;
use crate::src::encoder::cavlc::bitstream_h::bs_init;
use crate::src::encoder::cavlc::bitstream_h::bs_pos;
use crate::src::encoder::cavlc::bitstream_h::bs_write;
use crate::src::encoder::cavlc::bitstream_h::bs_write1;
use crate::src::encoder::cavlc::bitstream_h::bs_write_se;
use crate::src::encoder::cavlc::bitstream_h::bs_write_te;
use crate::src::encoder::cavlc::bitstream_h::bs_write_ue;
use crate::src::encoder::cavlc::macroblock_h::x264_mb_partition_listX_table;
use crate::src::encoder::cavlc::macroblock_h::x264_mb_predict_intra4x4_mode;
use crate::src::encoder::cavlc::macroblock_h::x264_mb_predict_non_zero_code;
use crate::src::encoder::cavlc::macroblock_h::x264_mb_transform_8x8_allowed;
use crate::src::encoder::cavlc::macroblock_h::x264_mb_type_list_table;
use crate::src::encoder::cavlc::osdep_h::x264_ctz_4bit;
use crate::src::encoder::cavlc::predict_h::x264_mb_chroma_pred_mode_fix;
use crate::src::encoder::cavlc::predict_h::x264_mb_pred_mode16x16_fix;
use crate::src::encoder::cavlc::predict_h::x264_mb_pred_mode4x4_fix;
static mut cbp_to_golomb: [[[crate::stdlib::uint8_t; 48]; 2]; 2] = [
    [
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
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
        ],
        [
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
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
        ],
    ],
    [
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            17 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            32 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            33 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            36 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            34 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            37 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            44 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            40 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            35 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            45 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            38 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            41 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            39 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            42 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            43 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            19 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            24 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            26 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            21 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            46 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            28 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            27 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            47 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            22 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            29 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            30 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            31 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        [
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            29 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            30 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            17 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            31 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            37 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            32 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            38 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            19 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            33 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            34 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            21 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            35 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            22 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            39 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            36 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            40 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            24 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            41 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            42 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            43 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            44 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            26 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            46 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            45 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            47 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            27 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            28 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    ],
];
static mut mb_type_b_to_golomb: [[crate::stdlib::uint8_t; 9]; 3] = [
    [
        4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        17 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        19 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        21 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        -(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        -(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        -(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        -(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        -(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        -(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
];
static mut subpartition_p_to_golomb: [crate::stdlib::uint8_t; 4] = [
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
static mut subpartition_b_to_golomb: [crate::stdlib::uint8_t; 13] = [
    10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
#[inline]
unsafe extern "C" fn cavlc_block_residual_escape(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_suffix_length: ::core::ffi::c_int,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut s: *mut crate::src::common::bitstream::bs_t = &raw mut (*h).out.bs;
        static mut next_suffix: [crate::stdlib::uint16_t; 7] = [
            0 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            12 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            24 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            48 as ::core::ffi::c_int as crate::stdlib::uint16_t,
            0xffff as ::core::ffi::c_int as crate::stdlib::uint16_t,
        ];
        let mut i_level_prefix: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
        let mut mask: ::core::ffi::c_int = level >> 31 as ::core::ffi::c_int;
        let mut abs_level: ::core::ffi::c_int = (level ^ mask) - mask;
        let mut i_level_code: ::core::ffi::c_int =
            abs_level * 2 as ::core::ffi::c_int - mask - 2 as ::core::ffi::c_int;
        if i_level_code >> i_suffix_length < 15 as ::core::ffi::c_int {
            bs_write(
                s,
                (i_level_code >> i_suffix_length) + 1 as ::core::ffi::c_int + i_suffix_length,
                (((1 as ::core::ffi::c_int) << i_suffix_length)
                    + (i_level_code
                        & ((1 as ::core::ffi::c_int) << i_suffix_length) - 1 as ::core::ffi::c_int))
                    as crate::stdlib::uint32_t,
            );
        } else {
            i_level_code -= (15 as ::core::ffi::c_int) << i_suffix_length;
            if i_suffix_length == 0 as ::core::ffi::c_int {
                i_level_code -= 15 as ::core::ffi::c_int;
            }
            if i_level_code >= (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int {
                if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_profile_idc
                    >= crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
                {
                    while i_level_code
                        >= (1 as ::core::ffi::c_int) << i_level_prefix - 3 as ::core::ffi::c_int
                    {
                        i_level_code -=
                            (1 as ::core::ffi::c_int) << i_level_prefix - 3 as ::core::ffi::c_int;
                        i_level_prefix += 1;
                    }
                } else {
                    (*h).mb.b_overflow = 1 as ::core::ffi::c_int;
                }
            }
            bs_write(
                s,
                i_level_prefix + 1 as ::core::ffi::c_int,
                1 as crate::stdlib::uint32_t,
            );
            bs_write(
                s,
                i_level_prefix - 3 as ::core::ffi::c_int,
                (i_level_code
                    & ((1 as ::core::ffi::c_int) << i_level_prefix - 3 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
            );
        }
        if i_suffix_length == 0 as ::core::ffi::c_int {
            i_suffix_length += 1;
        }
        if abs_level > next_suffix[i_suffix_length as usize] as ::core::ffi::c_int {
            i_suffix_length += 1;
        }
        return i_suffix_length;
    }
}
unsafe extern "C" fn cavlc_block_residual_internal(
    mut h: *mut crate::src::common::common::x264_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut crate::src::common::common::dctcoef,
    mut nC: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut s: *mut crate::src::common::bitstream::bs_t = &raw mut (*h).out.bs;
        static mut ctz_index: [crate::stdlib::uint8_t; 8] = [
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ];
        static mut count_cat: [crate::stdlib::uint8_t; 14] = [
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            64 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            64 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            64 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ];
        let mut runlevel: crate::src::common::bitstream::x264_run_level_t =
            crate::src::common::bitstream::x264_run_level_t {
                last: 0,
                mask: 0,
                level: [0; 18],
            };
        let mut i_total: ::core::ffi::c_int = (*h).quantf.coeff_level_run[ctx_block_cat as usize]
            .expect("non-null function pointer")(
            l, &raw mut runlevel
        );
        (&raw mut crate::src::common::vlc::x264_8_run_before as *mut crate::stdlib::uint32_t)
            .offset(runlevel.mask as isize) as *mut crate::stdlib::uint32_t;
        let mut i_total_zero: ::core::ffi::c_int = (runlevel.last + 1 as crate::stdlib::int32_t
            - i_total as crate::stdlib::int32_t)
            as ::core::ffi::c_int;
        runlevel.level[(i_total + 0 as ::core::ffi::c_int) as usize] =
            2 as crate::src::common::common::dctcoef;
        runlevel.level[(i_total + 1 as ::core::ffi::c_int) as usize] =
            2 as crate::src::common::common::dctcoef;
        let mut i_trailing: ::core::ffi::c_int = (runlevel.level[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int
                - runlevel.level[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            >> 31 as ::core::ffi::c_int
            & 1 as ::core::ffi::c_int
            | (runlevel.level[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int
                    - runlevel.level[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                >> 31 as ::core::ffi::c_int
                & 2 as ::core::ffi::c_int
            | (runlevel.level[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int
                    - runlevel.level[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                >> 31 as ::core::ffi::c_int
                & 4 as ::core::ffi::c_int;
        i_trailing = ctz_index[i_trailing as usize] as ::core::ffi::c_int;
        let mut i_sign: ::core::ffi::c_uint =
            (runlevel.level[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                >> 31 as ::core::ffi::c_int
                & 1 as ::core::ffi::c_int
                | runlevel.level[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >> 31 as ::core::ffi::c_int
                    & 2 as ::core::ffi::c_int
                | runlevel.level[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >> 31 as ::core::ffi::c_int
                    & 4 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        i_sign >>= 3 as ::core::ffi::c_int - i_trailing;
        bs_write(
            s,
            crate::src::common::tables::x264_coeff_token[nC as usize]
                [(i_total - 1 as ::core::ffi::c_int) as usize][i_trailing as usize]
                .i_size as ::core::ffi::c_int,
            crate::src::common::tables::x264_coeff_token[nC as usize]
                [(i_total - 1 as ::core::ffi::c_int) as usize][i_trailing as usize]
                .i_bits as crate::stdlib::uint32_t,
        );
        let mut i_suffix_length: ::core::ffi::c_int = (i_total > 10 as ::core::ffi::c_int
            && i_trailing < 3 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        bs_write(s, i_trailing, i_sign as crate::stdlib::uint32_t);
        if i_trailing < i_total {
            let mut val: ::core::ffi::c_int =
                runlevel.level[i_trailing as usize] as ::core::ffi::c_int;
            let mut val_original: ::core::ffi::c_int = runlevel.level[i_trailing as usize]
                as ::core::ffi::c_int
                + crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2 as ::core::ffi::c_int;
            val -= (val >> 31 as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
                & -((i_trailing < 3 as ::core::ffi::c_int) as ::core::ffi::c_int);
            val += crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2 as ::core::ffi::c_int;
            if (val_original as ::core::ffi::c_uint)
                < crate::src::common::bitstream::LEVEL_TABLE_SIZE as ::core::ffi::c_uint
            {
                bs_write(
                    s,
                    crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize]
                        [val as usize]
                        .i_size as ::core::ffi::c_int,
                    crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize]
                        [val as usize]
                        .i_bits as crate::stdlib::uint32_t,
                );
                i_suffix_length = crate::src::common::vlc::x264_8_level_token
                    [i_suffix_length as usize][val_original as usize]
                    .i_next as ::core::ffi::c_int;
            } else {
                i_suffix_length = cavlc_block_residual_escape(
                    h,
                    i_suffix_length,
                    val - crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2 as ::core::ffi::c_int,
                );
            }
            let mut i: ::core::ffi::c_int = i_trailing + 1 as ::core::ffi::c_int;
            while i < i_total {
                val = runlevel.level[i as usize] as ::core::ffi::c_int
                    + crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2 as ::core::ffi::c_int;
                if (val as ::core::ffi::c_uint)
                    < crate::src::common::bitstream::LEVEL_TABLE_SIZE as ::core::ffi::c_uint
                {
                    bs_write(
                        s,
                        crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize]
                            [val as usize]
                            .i_size as ::core::ffi::c_int,
                        crate::src::common::vlc::x264_8_level_token[i_suffix_length as usize]
                            [val as usize]
                            .i_bits as crate::stdlib::uint32_t,
                    );
                    i_suffix_length = crate::src::common::vlc::x264_8_level_token
                        [i_suffix_length as usize][val as usize]
                        .i_next as ::core::ffi::c_int;
                } else {
                    i_suffix_length = cavlc_block_residual_escape(
                        h,
                        i_suffix_length,
                        val - crate::src::common::bitstream::LEVEL_TABLE_SIZE
                            / 2 as ::core::ffi::c_int,
                    );
                }
                i += 1;
            }
        }
        if ctx_block_cat == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int {
            if i_total
                < 8 as ::core::ffi::c_int
                    >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
            {
                let mut total_zeros: crate::src::common::tables::vlc_t =
                    if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                    {
                        crate::src::common::tables::x264_total_zeros_2x2_dc
                            [(i_total - 1 as ::core::ffi::c_int) as usize]
                            [i_total_zero as usize]
                    } else {
                        crate::src::common::tables::x264_total_zeros_2x4_dc
                            [(i_total - 1 as ::core::ffi::c_int) as usize]
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
                crate::src::common::tables::x264_total_zeros
                    [(i_total - 1 as ::core::ffi::c_int) as usize][i_total_zero as usize]
                    .i_size as ::core::ffi::c_int,
                crate::src::common::tables::x264_total_zeros
                    [(i_total - 1 as ::core::ffi::c_int) as usize][i_total_zero as usize]
                    .i_bits as crate::stdlib::uint32_t,
            );
        }
        let mut zero_run_code: ::core::ffi::c_int = crate::src::common::vlc::x264_8_run_before
            [runlevel.mask as usize]
            as ::core::ffi::c_int;
        bs_write(
            s,
            zero_run_code & 0x1f as ::core::ffi::c_int,
            (zero_run_code >> 5 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        );
        return i_total;
    }
}
static mut ct_index: [crate::stdlib::uint8_t; 17] = [
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
unsafe extern "C" fn cavlc_qp_delta(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut s: *mut crate::src::common::bitstream::bs_t = &raw mut (*h).out.bs;
        let mut i_dqp: ::core::ffi::c_int = (*h).mb.i_qp - (*h).mb.i_last_qp;
        if (*h).mb.i_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
            && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
            && (*h).mb.cache.non_zero_count
                [x264_scan8[crate::src::common::base::LUMA_DC as usize] as usize]
                == 0
            && (*h).mb.cache.non_zero_count[x264_scan8
                [(crate::src::common::base::CHROMA_DC + 0 as ::core::ffi::c_int) as usize]
                as usize]
                == 0
            && (*h).mb.cache.non_zero_count[x264_scan8
                [(crate::src::common::base::CHROMA_DC + 1 as ::core::ffi::c_int) as usize]
                as usize]
                == 0
            && (*h).mb.i_qp > (*h).mb.i_last_qp
        {
            (*h).mb.i_qp = (*h).mb.i_last_qp;
            i_dqp = 0 as ::core::ffi::c_int;
        }
        if i_dqp != 0 {
            if i_dqp
                < -(crate::src::common::common::QP_MAX_SPEC + 1 as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int
            {
                i_dqp += crate::src::common::common::QP_MAX_SPEC + 1 as ::core::ffi::c_int;
            } else if i_dqp > crate::src::common::common::QP_MAX_SPEC / 2 as ::core::ffi::c_int {
                i_dqp -= crate::src::common::common::QP_MAX_SPEC + 1 as ::core::ffi::c_int;
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
        let mut s: *mut crate::src::common::bitstream::bs_t = &raw mut (*h).out.bs;
        let mut mvp: [crate::stdlib::int16_t; 2] = [0; 2];
        crate::src::common::mvpred::x264_8_mb_predict_mv(
            h as *mut crate::src::common::common::x264_t,
            i_list,
            idx,
            width,
            &raw mut mvp as *mut crate::stdlib::int16_t,
        );
        bs_write_se(
            s,
            (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize]
                [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        );
        bs_write_se(
            s,
            (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize]
                [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
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
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i,
                    2 as ::core::ffi::c_int,
                );
            }
            1 => {
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
            }
            2 => {
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
            0 => {
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
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
        if (*h).mb.b_transform_8x8 != 0 {
            let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p < plane_count {
                let mut i8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i8 < 4 as ::core::ffi::c_int {
                    if (*h).mb.cache.non_zero_count[x264_scan8
                        [(p * 16 as ::core::ffi::c_int + i8 * 4 as ::core::ffi::c_int) as usize]
                        as usize]
                        != 0
                    {
                        (*h).zigzagf
                            .interleave_8x8_cavlc
                            .expect("non-null function pointer")(
                            &raw mut *(&raw mut (*h).dct.luma4x4
                                as *mut [crate::src::common::common::dctcoef; 16])
                                .offset(
                                    (p * 16 as ::core::ffi::c_int + i8 * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                as *mut crate::src::common::common::dctcoef,
                            &raw mut *(&raw mut (*h).dct.luma8x8
                                as *mut [crate::src::common::common::dctcoef; 64])
                                .offset((p * 4 as ::core::ffi::c_int + i8) as isize)
                                as *mut crate::src::common::common::dctcoef,
                            (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                                .offset(
                                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset(
                                            (p * 16 as ::core::ffi::c_int
                                                + i8 * 4 as ::core::ffi::c_int)
                                                as isize,
                                        ) as isize,
                                ) as *mut crate::stdlib::uint8_t,
                        );
                    }
                    i8 += 1;
                }
                p += 1;
            }
        }
        let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p_0 < plane_count {
            let mut i8_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut msk: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
            let mut skip: ::core::ffi::c_int = 0;
            while msk != 0 && {
                skip = x264_ctz_4bit(msk as crate::stdlib::uint32_t);
                i8_0 += skip;
                msk >>= skip + 1 as ::core::ffi::c_int;
                1 as ::core::ffi::c_int != 0
            } {
                let mut i4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i4 < 4 as ::core::ffi::c_int {
                    let mut nC: ::core::ffi::c_int = if crate::src::common::macroblock::DCT_LUMA_4x4
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                    {
                        5 as ::core::ffi::c_int
                            - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                    } else {
                        ct_index[x264_mb_predict_non_zero_code(
                            h,
                            if crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int
                                == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                            {
                                (i4 + i8_0 * 4 as ::core::ffi::c_int
                                    + p_0 * 16 as ::core::ffi::c_int
                                    - crate::src::common::base::LUMA_DC)
                                    * 16 as ::core::ffi::c_int
                            } else {
                                i4 + i8_0 * 4 as ::core::ffi::c_int + p_0 * 16 as ::core::ffi::c_int
                            },
                        ) as usize] as ::core::ffi::c_int
                    };
                    let mut nnz: *mut crate::stdlib::uint8_t =
                        (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(
                                    (i4 + i8_0 * 4 as ::core::ffi::c_int
                                        + p_0 * 16 as ::core::ffi::c_int)
                                        as isize,
                                ) as isize,
                            ) as *mut crate::stdlib::uint8_t;
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
                                .offset(
                                    (i4 + i8_0 * 4 as ::core::ffi::c_int
                                        + p_0 * 16 as ::core::ffi::c_int)
                                        as isize,
                                )
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
    mut i_mb_type: ::core::ffi::c_int,
    mut i_mb_i_offset: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut s: *mut crate::src::common::bitstream::bs_t = &raw mut (*h).out.bs;
        if i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int {
            bs_write_ue(
                s,
                i_mb_i_offset
                    + 1 as ::core::ffi::c_int
                    + x264_mb_pred_mode16x16_fix[(*h).mb.i_intra16x16_pred_mode as usize]
                        as ::core::ffi::c_int
                    + (*h).mb.i_cbp_chroma * 4 as ::core::ffi::c_int
                    + (if (*h).mb.i_cbp_luma == 0 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int
                    } else {
                        12 as ::core::ffi::c_int
                    }),
            );
        } else {
            let mut di: ::core::ffi::c_int =
                if i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
            bs_write_ue(s, i_mb_i_offset + 0 as ::core::ffi::c_int);
            if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                .b_transform_8x8_mode
                != 0
            {
                bs_write1(s, (*h).mb.b_transform_8x8 as crate::stdlib::uint32_t);
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let mut i_pred: ::core::ffi::c_int = x264_mb_predict_intra4x4_mode(h, i);
                let mut i_mode: ::core::ffi::c_int =
                    x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                        [x264_scan8[i as usize] as usize]
                        as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                if i_pred == i_mode {
                    bs_write1(s, 1 as crate::stdlib::uint32_t);
                } else {
                    bs_write(
                        s,
                        4 as ::core::ffi::c_int,
                        (i_mode - (i_mode > i_pred) as ::core::ffi::c_int)
                            as crate::stdlib::uint32_t,
                    );
                }
                i += di;
            }
        }
        if chroma != 0 {
            bs_write_ue(
                s,
                x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize]
                    as ::core::ffi::c_int,
            );
        }
    }
}
#[inline(always)]
unsafe extern "C" fn cavlc_mb_header_p(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut s: *mut crate::src::common::bitstream::bs_t = &raw mut (*h).out.bs;
        if i_mb_type == crate::src::common::macroblock::P_L0 as ::core::ffi::c_int {
            if (*h).mb.i_partition == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            {
                bs_write1(s, 1 as crate::stdlib::uint32_t);
                if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize]
                            - 1 as ::core::ffi::c_int,
                        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
            } else if (*h).mb.i_partition
                == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
            {
                bs_write_ue(s, 1 as ::core::ffi::c_int);
                if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize]
                            - 1 as ::core::ffi::c_int,
                        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize]
                            - 1 as ::core::ffi::c_int,
                        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                            [x264_scan8[8 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
            } else if (*h).mb.i_partition
                == crate::src::common::macroblock::D_8x16 as ::core::ffi::c_int
            {
                bs_write_ue(s, 2 as ::core::ffi::c_int);
                if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize]
                            - 1 as ::core::ffi::c_int,
                        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize]
                            - 1 as ::core::ffi::c_int,
                        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                            [x264_scan8[4 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
                cavlc_mvd(
                    h,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
            }
        } else if i_mb_type == crate::src::common::macroblock::P_8x8 as ::core::ffi::c_int {
            let mut b_sub_ref: ::core::ffi::c_int = 0;
            if (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int
                | (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[4 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int
                | (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[8 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int
                | (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                bs_write_ue(s, 4 as ::core::ffi::c_int);
                b_sub_ref = 0 as ::core::ffi::c_int;
            } else {
                bs_write_ue(s, 3 as ::core::ffi::c_int);
                b_sub_ref = 1 as ::core::ffi::c_int;
            }
            if (*h).param.analyse.inter & crate::x264_h::X264_ANALYSE_PSUB8x8 != 0 {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < 4 as ::core::ffi::c_int {
                    bs_write_ue(
                        s,
                        subpartition_p_to_golomb[(*h).mb.i_sub_partition[i as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                    i += 1;
                }
            } else {
                bs_write(s, 4 as ::core::ffi::c_int, 0xf as crate::stdlib::uint32_t);
            }
            if b_sub_ref != 0 {
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int,
                    (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                        [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                        as ::core::ffi::c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int,
                    (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                        [x264_scan8[4 as ::core::ffi::c_int as usize] as usize]
                        as ::core::ffi::c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int,
                    (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                        [x264_scan8[8 as ::core::ffi::c_int as usize] as usize]
                        as ::core::ffi::c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int,
                    (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                        [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                        as ::core::ffi::c_int,
                );
            }
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
                cavlc_8x8_mvd(h, i_0);
                i_0 += 1;
            }
        } else {
            cavlc_mb_header_i(h, i_mb_type, 5 as ::core::ffi::c_int, chroma);
        };
    }
}
#[inline(always)]
unsafe extern "C" fn cavlc_mb_header_b(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut s: *mut crate::src::common::bitstream::bs_t = &raw mut (*h).out.bs;
        if i_mb_type == crate::src::common::macroblock::B_8x8 as ::core::ffi::c_int {
            bs_write_ue(s, 22 as ::core::ffi::c_int);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                bs_write_ue(
                    s,
                    subpartition_b_to_golomb[(*h).mb.i_sub_partition[i as usize] as usize]
                        as ::core::ffi::c_int,
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
                        bs_write_te(
                            s,
                            (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize]
                                - 1 as ::core::ffi::c_int,
                            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                                [x264_scan8[(i_0 * 4 as ::core::ffi::c_int) as usize] as usize]
                                as ::core::ffi::c_int,
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
                        bs_write_te(
                            s,
                            (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize]
                                - 1 as ::core::ffi::c_int,
                            (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                                [x264_scan8[(i_1 * 4 as ::core::ffi::c_int) as usize] as usize]
                                as ::core::ffi::c_int,
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
                    cavlc_mvd(
                        h,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int * i_2,
                        2 as ::core::ffi::c_int,
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
                    cavlc_mvd(
                        h,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int * i_3,
                        2 as ::core::ffi::c_int,
                    );
                }
                i_3 += 1;
            }
        } else if i_mb_type >= crate::src::common::macroblock::B_L0_L0 as ::core::ffi::c_int
            && i_mb_type <= crate::src::common::macroblock::B_BI_BI as ::core::ffi::c_int
        {
            let mut b_list: *const [crate::stdlib::uint8_t; 2] =
                &raw const *(&raw const x264_mb_type_list_table
                    as *const [[crate::stdlib::uint8_t; 2]; 2])
                    .offset(i_mb_type as isize)
                    as *const [crate::stdlib::uint8_t; 2];
            let i_ref0_max: ::core::ffi::c_int =
                (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int;
            let i_ref1_max: ::core::ffi::c_int =
                (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int;
            bs_write_ue(
                s,
                mb_type_b_to_golomb[((*h).mb.i_partition
                    - crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int)
                    as usize][(i_mb_type
                    - crate::src::common::macroblock::B_L0_L0 as ::core::ffi::c_int)
                    as usize] as ::core::ffi::c_int,
            );
            if (*h).mb.i_partition == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            {
                if i_ref0_max != 0
                    && (*b_list.offset(0 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        != 0
                {
                    bs_write_te(
                        s,
                        i_ref0_max,
                        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                if i_ref1_max != 0
                    && (*b_list.offset(1 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        != 0
                {
                    bs_write_te(
                        s,
                        i_ref1_max,
                        (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                if (*b_list.offset(0 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize]
                    != 0
                {
                    cavlc_mvd(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                }
                if (*b_list.offset(1 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize]
                    != 0
                {
                    cavlc_mvd(
                        h,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                }
            } else {
                if i_ref0_max != 0
                    && (*b_list.offset(0 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        != 0
                {
                    bs_write_te(
                        s,
                        i_ref0_max,
                        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                if i_ref0_max != 0
                    && (*b_list.offset(0 as ::core::ffi::c_int as isize))
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        != 0
                {
                    bs_write_te(
                        s,
                        i_ref0_max,
                        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                            [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                if i_ref1_max != 0
                    && (*b_list.offset(1 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        != 0
                {
                    bs_write_te(
                        s,
                        i_ref1_max,
                        (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                if i_ref1_max != 0
                    && (*b_list.offset(1 as ::core::ffi::c_int as isize))
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        != 0
                {
                    bs_write_te(
                        s,
                        i_ref1_max,
                        (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                            [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                    );
                }
                if (*h).mb.i_partition
                    == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
                {
                    if (*b_list.offset(0 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        != 0
                    {
                        cavlc_mvd(
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    }
                    if (*b_list.offset(0 as ::core::ffi::c_int as isize))
                        [1 as ::core::ffi::c_int as usize]
                        != 0
                    {
                        cavlc_mvd(
                            h,
                            0 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    }
                    if (*b_list.offset(1 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        != 0
                    {
                        cavlc_mvd(
                            h,
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    }
                    if (*b_list.offset(1 as ::core::ffi::c_int as isize))
                        [1 as ::core::ffi::c_int as usize]
                        != 0
                    {
                        cavlc_mvd(
                            h,
                            1 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    if (*b_list.offset(0 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        != 0
                    {
                        cavlc_mvd(
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                    }
                    if (*b_list.offset(0 as ::core::ffi::c_int as isize))
                        [1 as ::core::ffi::c_int as usize]
                        != 0
                    {
                        cavlc_mvd(
                            h,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                    }
                    if (*b_list.offset(1 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        != 0
                    {
                        cavlc_mvd(
                            h,
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                    }
                    if (*b_list.offset(1 as ::core::ffi::c_int as isize))
                        [1 as ::core::ffi::c_int as usize]
                        != 0
                    {
                        cavlc_mvd(
                            h,
                            1 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                    }
                }
            }
        } else if i_mb_type == crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int {
            bs_write1(s, 1 as crate::stdlib::uint32_t);
        } else {
            cavlc_mb_header_i(h, i_mb_type, 23 as ::core::ffi::c_int, chroma);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_write_cavlc(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut s: *mut crate::src::common::bitstream::bs_t = &raw mut (*h).out.bs;
        let i_mb_type: ::core::ffi::c_int = (*h).mb.i_type;
        let mut plane_count: ::core::ffi::c_int = if crate::src::common::base::CHROMA_444
            as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        let mut chroma: ::core::ffi::c_int = (crate::src::common::base::CHROMA_444
            as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
            || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let i_mb_pos_start: ::core::ffi::c_int = bs_pos(s) as ::core::ffi::c_int;
        let mut i_mb_pos_tex: ::core::ffi::c_int = 0;
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
            bs_write1(s, (*h).mb.b_interlaced as crate::stdlib::uint32_t);
            (*h).mb.field_decoding_flag = (*h).mb.b_interlaced;
        }
        if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            static mut i_offsets: [crate::stdlib::uint8_t; 3] = [
                5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            ];
            let mut p_start: *mut crate::stdlib::uint8_t = (*s).p_start;
            bs_write_ue(
                s,
                i_offsets[(*h).sh.i_type as usize] as ::core::ffi::c_int + 25 as ::core::ffi::c_int,
            );
            i_mb_pos_tex = bs_pos(s);
            (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
            bs_align_0(s);
            let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p < plane_count {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < 256 as ::core::ffi::c_int {
                    bs_write(
                        s,
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
                                s,
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
            bs_init(
                s,
                (*s).p as *mut ::core::ffi::c_void,
                (*s).p_end.offset_from((*s).p) as ::core::ffi::c_long as ::core::ffi::c_int,
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
            cavlc_mb_header_i(h, i_mb_type, 0 as ::core::ffi::c_int, chroma);
        }
        i_mb_pos_tex = bs_pos(s);
        (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
        if i_mb_type != crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int {
            bs_write_ue(
                s,
                cbp_to_golomb[chroma as usize][(i_mb_type
                    == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                    || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                    || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                    || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    as ::core::ffi::c_int as usize][((*h).mb.i_cbp_chroma
                    << 4 as ::core::ffi::c_int
                    | (*h).mb.i_cbp_luma)
                    as usize] as ::core::ffi::c_int,
            );
        }
        if x264_mb_transform_8x8_allowed(h) != 0 && (*h).mb.i_cbp_luma != 0 {
            bs_write1(s, (*h).mb.b_transform_8x8 as crate::stdlib::uint32_t);
        }
        if i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int {
            cavlc_qp_delta(h);
            let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_0 < plane_count {
                let mut nC: ::core::ffi::c_int = if crate::src::common::macroblock::DCT_LUMA_DC
                    as ::core::ffi::c_int
                    == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                {
                    5 as ::core::ffi::c_int
                        - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                } else {
                    ct_index[x264_mb_predict_non_zero_code(
                        h,
                        if crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                            == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                        {
                            (48 as ::core::ffi::c_int + p_0 - crate::src::common::base::LUMA_DC)
                                * 16 as ::core::ffi::c_int
                        } else {
                            48 as ::core::ffi::c_int + p_0
                        },
                    ) as usize] as ::core::ffi::c_int
                };
                let mut nnz: *mut crate::stdlib::uint8_t =
                    (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset((48 as ::core::ffi::c_int + p_0) as isize)
                            as isize,
                    ) as *mut crate::stdlib::uint8_t;
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
                    let mut i_1: ::core::ffi::c_int = p_0 * 16 as ::core::ffi::c_int;
                    while i_1 < p_0 * 16 as ::core::ffi::c_int + 16 as ::core::ffi::c_int {
                        let mut nC_0: ::core::ffi::c_int =
                            if crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int
                                == crate::src::common::macroblock::DCT_CHROMA_DC
                                    as ::core::ffi::c_int
                            {
                                5 as ::core::ffi::c_int
                                    - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                        == crate::src::common::base::CHROMA_420
                                            as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                            } else {
                                ct_index[x264_mb_predict_non_zero_code(
                                    h,
                                    if crate::src::common::macroblock::DCT_LUMA_AC
                                        as ::core::ffi::c_int
                                        == crate::src::common::macroblock::DCT_LUMA_DC
                                            as ::core::ffi::c_int
                                    {
                                        (i_1 - crate::src::common::base::LUMA_DC)
                                            * 16 as ::core::ffi::c_int
                                    } else {
                                        i_1
                                    },
                                ) as usize] as ::core::ffi::c_int
                            };
                        let mut nnz_0: *mut crate::stdlib::uint8_t =
                            (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                                .offset(
                                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset(i_1 as isize)
                                        as isize,
                                ) as *mut crate::stdlib::uint8_t;
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
                                    .offset(1 as ::core::ffi::c_int as isize),
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
            let mut nC_1: ::core::ffi::c_int = if crate::src::common::macroblock::DCT_CHROMA_DC
                as ::core::ffi::c_int
                == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
            {
                5 as ::core::ffi::c_int
                    - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
            } else {
                ct_index[x264_mb_predict_non_zero_code(
                    h,
                    if crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                        == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                    {
                        (49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                            - crate::src::common::base::LUMA_DC)
                            * 16 as ::core::ffi::c_int
                    } else {
                        49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                    },
                ) as usize] as ::core::ffi::c_int
            };
            let mut nnz_1: *mut crate::stdlib::uint8_t =
                (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                        as isize,
                ) as *mut crate::stdlib::uint8_t;
            if *nnz_1 == 0 {
                bs_write(
                    &raw mut (*h).out.bs,
                    crate::src::common::tables::x264_coeff0_token[nC_1 as usize].i_size
                        as ::core::ffi::c_int,
                    crate::src::common::tables::x264_coeff0_token[nC_1 as usize].i_bits
                        as crate::stdlib::uint32_t,
                );
            } else {
                *nnz_1 = cavlc_block_residual_internal(
                    h,
                    crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                    &raw mut *(&raw mut (*h).dct.chroma_dc
                        as *mut [crate::src::common::common::dctcoef; 8])
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::common::dctcoef,
                    nC_1,
                ) as crate::stdlib::uint8_t;
            }
            let mut nC_2: ::core::ffi::c_int = if crate::src::common::macroblock::DCT_CHROMA_DC
                as ::core::ffi::c_int
                == crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
            {
                5 as ::core::ffi::c_int
                    - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
            } else {
                ct_index[x264_mb_predict_non_zero_code(
                    h,
                    if crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int
                        == crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int
                    {
                        (49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            - crate::src::common::base::LUMA_DC)
                            * 16 as ::core::ffi::c_int
                    } else {
                        49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    },
                ) as usize] as ::core::ffi::c_int
            };
            let mut nnz_2: *mut crate::stdlib::uint8_t =
                (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as isize,
                ) as *mut crate::stdlib::uint8_t;
            if *nnz_2 == 0 {
                bs_write(
                    &raw mut (*h).out.bs,
                    crate::src::common::tables::x264_coeff0_token[nC_2 as usize].i_size
                        as ::core::ffi::c_int,
                    crate::src::common::tables::x264_coeff0_token[nC_2 as usize].i_bits
                        as crate::stdlib::uint32_t,
                );
            } else {
                *nnz_2 = cavlc_block_residual_internal(
                    h,
                    crate::src::common::macroblock::DCT_CHROMA_DC as ::core::ffi::c_int,
                    &raw mut *(&raw mut (*h).dct.chroma_dc
                        as *mut [crate::src::common::common::dctcoef; 8])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::common::dctcoef,
                    nC_2,
                ) as crate::stdlib::uint8_t;
            }
            if (*h).mb.i_cbp_chroma == 2 as ::core::ffi::c_int {
                let mut step: ::core::ffi::c_int = (8 as ::core::ffi::c_int)
                    << (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                let mut i_2: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
                while i_2 < 3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int {
                    let mut j_0: ::core::ffi::c_int = i_2;
                    while j_0 < i_2 + 4 as ::core::ffi::c_int {
                        let mut nC_3: ::core::ffi::c_int =
                            if crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int
                                == crate::src::common::macroblock::DCT_CHROMA_DC
                                    as ::core::ffi::c_int
                            {
                                5 as ::core::ffi::c_int
                                    - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                        == crate::src::common::base::CHROMA_420
                                            as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                            } else {
                                ct_index[x264_mb_predict_non_zero_code(
                                    h,
                                    if crate::src::common::macroblock::DCT_CHROMA_AC
                                        as ::core::ffi::c_int
                                        == crate::src::common::macroblock::DCT_LUMA_DC
                                            as ::core::ffi::c_int
                                    {
                                        (j_0 - crate::src::common::base::LUMA_DC)
                                            * 16 as ::core::ffi::c_int
                                    } else {
                                        j_0
                                    },
                                ) as usize] as ::core::ffi::c_int
                            };
                        let mut nnz_3: *mut crate::stdlib::uint8_t =
                            (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                                .offset(
                                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                        .offset(j_0 as isize)
                                        as isize,
                                ) as *mut crate::stdlib::uint8_t;
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
                                    .offset(1 as ::core::ffi::c_int as isize),
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
