use ::c2rust_bitfields;

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

    pub unsafe extern "C" fn bs_align_1(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            bs_write(
                s,
                (*s).i_left & 7 as ::core::ffi::c_int,
                (((1 as ::core::ffi::c_int) << ((*s).i_left & 7 as ::core::ffi::c_int))
                    - 1 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
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

    pub unsafe extern "C" fn bs_size_ue_big(mut val: ::core::ffi::c_uint) -> ::core::ffi::c_int {
        unsafe {
            if val < 255 as ::core::ffi::c_uint {
                return x264_ue_size_tab[val.wrapping_add(1 as ::core::ffi::c_uint) as usize]
                    as ::core::ffi::c_int;
            } else {
                return x264_ue_size_tab[(val.wrapping_add(1 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int;
            };
        }
    }
    use crate::src::common::base::x264_union32_t;

    use crate::osdep_h::WORD_SIZE;
    use crate::src::encoder::encoder::osdep_h::endian_fix;
    use crate::src::encoder::encoder::osdep_h::endian_fix32;
    use crate::stdlib::intptr_t;
    use crate::stdlib::uintptr_t;

    use crate::stdlib::uint32_t;
    use crate::stdlib::uint8_t;
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

pub mod pixel_h {

    pub static mut x264_luma2chroma_pixel: [[crate::stdlib::uint8_t; 7]; 4] = [
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_2x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_2x2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        [
            crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_2x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_2x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        [
            crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    ];

    use crate::stdlib::uint8_t;
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

    use crate::stdlib::int8_t;
    use crate::stdlib::uint8_t;
}

pub mod base_h {

    pub static mut slice_type_to_char: [::core::ffi::c_char; 3] = [
        'P' as i32 as ::core::ffi::c_char,
        'B' as i32 as ::core::ffi::c_char,
        'I' as i32 as ::core::ffi::c_char,
    ];

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

    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            return if v < i_min {
                i_min
            } else if v > i_max {
                i_max
            } else {
                v
            };
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn x264_clip3f(
        mut v: ::core::ffi::c_double,
        mut f_min: ::core::ffi::c_double,
        mut f_max: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double {
        unsafe {
            return if v < f_min {
                f_min
            } else if v > f_max {
                f_max
            } else {
                v
            };
        }
    }

    use crate::stdlib::uint8_t;
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

    pub static mut x264_mb_partition_pixel_table: [crate::stdlib::uint8_t; 17] = [
        crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ];

    pub static mut i_chroma_qp_table: [crate::stdlib::uint8_t; 94] = [
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        (0 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (1 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (2 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (3 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (8 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (9 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (10 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (11 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (12 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (13 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (14 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (15 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (16 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (17 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (18 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (19 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (20 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (21 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (22 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (23 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (24 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (25 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (26 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (27 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (28 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (29 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (29 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (30 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (31 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (32 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (32 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (33 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (34 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (34 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (35 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (35 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (36 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (36 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (37 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (37 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (37 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (38 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (38 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (38 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
        (39 as ::core::ffi::c_int + crate::src::common::common::QP_BD_OFFSET)
            as crate::stdlib::uint8_t,
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
    use crate::src::common::common::QP_BD_OFFSET;

    use crate::src::common::pixel::PIXEL_16x16;
    use crate::src::common::pixel::PIXEL_16x8;
    use crate::src::common::pixel::PIXEL_4x4;
    use crate::src::common::pixel::PIXEL_4x8;
    use crate::src::common::pixel::PIXEL_8x16;
    use crate::src::common::pixel::PIXEL_8x4;
    use crate::src::common::pixel::PIXEL_8x8;
    use crate::stdlib::uint8_t;
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

    #[inline(always)]

    pub unsafe extern "C" fn endian_fix32(
        mut x: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
        unsafe {
            return (x << 24 as ::core::ffi::c_int)
                .wrapping_add(x << 8 as ::core::ffi::c_int & 0xff0000 as crate::stdlib::uint32_t)
                .wrapping_add(x >> 8 as ::core::ffi::c_int & 0xff00 as crate::stdlib::uint32_t)
                .wrapping_add(x >> 24 as ::core::ffi::c_int);
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn endian_fix64(
        mut x: crate::stdlib::uint64_t,
    ) -> crate::stdlib::uint64_t {
        unsafe {
            return (endian_fix32((x >> 32 as ::core::ffi::c_int) as crate::stdlib::uint32_t)
                as crate::stdlib::uint64_t)
                .wrapping_add(
                    (endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uint64_t)
                        << 32 as ::core::ffi::c_int,
                );
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn endian_fix(
        mut x: crate::stdlib::uintptr_t,
    ) -> crate::stdlib::uintptr_t {
        unsafe {
            return if crate::osdep_h::WORD_SIZE == 8 as ::core::ffi::c_int {
                endian_fix64(x as crate::stdlib::uint64_t) as crate::stdlib::uintptr_t
            } else {
                endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uintptr_t
            };
        }
    }
    use crate::stdlib::__mode_t;
    use crate::stdlib::fileno;
    use crate::stdlib::fstat;
    use crate::stdlib::stat;
    use crate::stdlib::timespec;
    use crate::stdlib::uint32_t;
    use crate::stdlib::uint64_t;
    use crate::stdlib::uintptr_t;
    use crate::stdlib::__S_IFMT;
}

pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::common::base::chroma_format_e;
pub use crate::src::common::base::profile_e;
pub use crate::src::common::base::slice_type_e;
pub use crate::src::common::base::x264_free;
pub use crate::src::common::base::x264_log_internal;
pub use crate::src::common::base::x264_malloc;
pub use crate::src::common::base::x264_param_strdup;
pub use crate::src::common::base::x264_reduce_fraction;
pub use crate::src::common::base::x264_union16_t;
pub use crate::src::common::base::x264_union32_t;
pub use crate::src::common::base::CHROMA_400;
pub use crate::src::common::base::CHROMA_420;
pub use crate::src::common::base::CHROMA_422;
pub use crate::src::common::base::CHROMA_444;
pub use crate::src::common::base::PROFILE_BASELINE;
pub use crate::src::common::base::PROFILE_HIGH;
pub use crate::src::common::base::PROFILE_HIGH10;
pub use crate::src::common::base::PROFILE_HIGH422;
pub use crate::src::common::base::PROFILE_HIGH444_PREDICTIVE;
pub use crate::src::common::base::PROFILE_MAIN;
pub use crate::src::common::base::SLICE_TYPE_B;
pub use crate::src::common::base::SLICE_TYPE_I;
pub use crate::src::common::base::SLICE_TYPE_P;
pub use crate::src::common::base::X264_LOOKAHEAD_MAX;
pub use crate::src::common::base::X264_REF_MAX;
pub use crate::src::common::base::X264_THREAD_HEIGHT;
pub use crate::src::common::base::X264_THREAD_MAX;
pub use crate::src::common::base::X264_WEIGHTP_FAKE;
use crate::src::encoder::analyse::x264_8_analyse_free_costs;
use crate::src::encoder::analyse::x264_8_analyse_init_costs;
use crate::src::encoder::analyse::x264_8_analyse_weight_frame;
use crate::src::encoder::analyse::x264_8_macroblock_analyse;
pub use crate::src::encoder::encoder::base_h::slice_type_to_char;
pub use crate::src::encoder::encoder::base_h::x264_clip3;
pub use crate::src::encoder::encoder::base_h::x264_clip3f;
pub use crate::src::encoder::encoder::base_h::x264_scan8;
use crate::src::encoder::lookahead::x264_8_lookahead_delete;
use crate::src::encoder::lookahead::x264_8_lookahead_get_frames;
use crate::src::encoder::lookahead::x264_8_lookahead_init;
use crate::src::encoder::lookahead::x264_8_lookahead_is_empty;
use crate::src::encoder::lookahead::x264_8_lookahead_put_frame;
pub use crate::stdlib::C2Rust_Unnamed_7;
use crate::stdlib::__assert_fail;
use crate::stdlib::__assert_single_arg;
pub use crate::stdlib::__atomic_wide_counter;
pub use crate::stdlib::__S_IFMT;

pub use crate::internal::__va_list_tag;
pub use crate::internal::BIT_DEPTH;
pub use crate::internal::__INT_MAX__;
pub use crate::limits_h::INT_MAX;
pub use crate::src::common::bitstream::bs_s;
pub use crate::src::common::bitstream::bs_t;
pub use crate::src::common::bitstream::x264_8_bitstream_init;
pub use crate::src::common::bitstream::x264_bitstream_function_t;
pub use crate::src::common::bitstream::x264_run_level_t;
pub use crate::src::common::cabac::x264_8_cabac_context_init;
pub use crate::src::common::cabac::x264_8_cabac_encode_flush;
pub use crate::src::common::cabac::x264_8_cabac_encode_init;
pub use crate::src::common::cabac::x264_8_cabac_encode_terminal_c;
pub use crate::src::common::cabac::x264_8_cabac_init;
pub use crate::src::common::cabac::x264_cabac_t;
pub use crate::src::common::common::dctcoef;
pub use crate::src::common::common::pixel;
pub use crate::src::common::common::udctcoef;
pub use crate::src::common::common::x264_8_log;
pub use crate::src::common::common::x264_frame_stat_t;
pub use crate::src::common::common::x264_left_table_t;
pub use crate::src::common::common::x264_lookahead_t;
pub use crate::src::common::common::x264_ratecontrol_t;
pub use crate::src::common::common::x264_slice_header_t;
pub use crate::src::common::common::x264_t;
pub use crate::src::common::common::C2Rust_Unnamed_10;
pub use crate::src::common::common::C2Rust_Unnamed_11;
pub use crate::src::common::common::C2Rust_Unnamed_12;
pub use crate::src::common::common::C2Rust_Unnamed_13;
pub use crate::src::common::common::C2Rust_Unnamed_14;
pub use crate::src::common::common::C2Rust_Unnamed_15;
pub use crate::src::common::common::C2Rust_Unnamed_16;
pub use crate::src::common::common::C2Rust_Unnamed_17;
pub use crate::src::common::common::C2Rust_Unnamed_8;
pub use crate::src::common::common::C2Rust_Unnamed_9;
pub use crate::src::common::common::FILLER_OVERHEAD;
pub use crate::src::common::common::NALU_OVERHEAD;
pub use crate::src::common::common::PIXEL_MAX;
pub use crate::src::common::common::QP_BD_OFFSET;
pub use crate::src::common::common::QP_MAX;
pub use crate::src::common::common::QP_MAX_SPEC;
pub use crate::src::common::common::SIZEOF_PIXEL;
pub use crate::src::common::cpu::x264_cpu_name_t;
pub use crate::src::common::cpu::x264_cpu_names;
pub use crate::src::common::cpu::x264_cpu_num_processors;
pub use crate::src::common::dct::x264_8_dct_init;
pub use crate::src::common::dct::x264_8_zigzag_init;
pub use crate::src::common::dct::x264_dct_function_t;
pub use crate::src::common::dct::x264_zigzag_function_t;
pub use crate::src::common::deblock::x264_8_deblock_init;
pub use crate::src::common::deblock::x264_8_frame_deblock_row;
pub use crate::src::common::frame::x264_8_expand_border_mbpair;
pub use crate::src::common::frame::x264_8_frame_cond_broadcast;
pub use crate::src::common::frame::x264_8_frame_copy_picture;
pub use crate::src::common::frame::x264_8_frame_delete;
pub use crate::src::common::frame::x264_8_frame_delete_list;
pub use crate::src::common::frame::x264_8_frame_expand_border;
pub use crate::src::common::frame::x264_8_frame_expand_border_filtered;
pub use crate::src::common::frame::x264_8_frame_expand_border_mod16;
pub use crate::src::common::frame::x264_8_frame_new_slice;
pub use crate::src::common::frame::x264_8_frame_pop;
pub use crate::src::common::frame::x264_8_frame_pop_blank_unused;
pub use crate::src::common::frame::x264_8_frame_pop_unused;
pub use crate::src::common::frame::x264_8_frame_push;
pub use crate::src::common::frame::x264_8_frame_push_blank_unused;
pub use crate::src::common::frame::x264_8_frame_push_unused;
pub use crate::src::common::frame::x264_8_frame_shift;
pub use crate::src::common::frame::x264_8_frame_unshift;
pub use crate::src::common::frame::x264_8_threadslice_cond_broadcast;
pub use crate::src::common::frame::x264_8_threadslice_cond_wait;
pub use crate::src::common::frame::x264_8_weight_scale_plane;
pub use crate::src::common::frame::x264_deblock_function_t;
pub use crate::src::common::frame::x264_deblock_inter_t;
pub use crate::src::common::frame::x264_deblock_intra_t;
pub use crate::src::common::frame::x264_frame;
pub use crate::src::common::frame::x264_frame_t;
pub use crate::src::common::frame::x264_sync_frame_list_t;
pub use crate::src::common::frame::PADH;
pub use crate::src::common::frame::PADV;
pub use crate::src::common::mc::x264_8_frame_filter;
pub use crate::src::common::mc::x264_8_frame_init_lowres;
pub use crate::src::common::vlc::x264_8_cavlc_init;
use crate::src::encoder::analyse::rdo_c::x264_8_rdo_init;
use crate::src::encoder::cabac::x264_8_cabac_mb_skip;
use crate::src::encoder::cabac::x264_8_macroblock_write_cabac;
use crate::src::encoder::cavlc::x264_8_macroblock_write_cavlc;
pub use crate::src::encoder::encoder::bitstream_h::bs_align_1;
pub use crate::src::encoder::encoder::bitstream_h::bs_flush;
pub use crate::src::encoder::encoder::bitstream_h::bs_init;
pub use crate::src::encoder::encoder::bitstream_h::bs_pos;
pub use crate::src::encoder::encoder::bitstream_h::bs_rbsp_trailing;
pub use crate::src::encoder::encoder::bitstream_h::bs_realign;
pub use crate::src::encoder::encoder::bitstream_h::bs_size_ue_big;
pub use crate::src::encoder::encoder::bitstream_h::bs_write;
pub use crate::src::encoder::encoder::bitstream_h::bs_write1;
pub use crate::src::encoder::encoder::bitstream_h::bs_write_se;
pub use crate::src::encoder::encoder::bitstream_h::bs_write_ue_big;
pub use crate::src::encoder::encoder::bitstream_h::x264_ue_size_tab;
pub use crate::src::encoder::encoder::cabac_h::x264_cabac_pos;
use crate::src::encoder::macroblock::x264_8_macroblock_encode;
use crate::src::encoder::macroblock::x264_8_noise_reduction_update;
use crate::src::encoder::set::x264_8_filler_write;
use crate::src::encoder::set::x264_8_pps_init;
use crate::src::encoder::set::x264_8_pps_write;
use crate::src::encoder::set::x264_8_sei_alternative_transfer_write;
use crate::src::encoder::set::x264_8_sei_avcintra_umid_write;
use crate::src::encoder::set::x264_8_sei_avcintra_vanc_write;
use crate::src::encoder::set::x264_8_sei_buffering_period_write;
use crate::src::encoder::set::x264_8_sei_content_light_level_write;
use crate::src::encoder::set::x264_8_sei_dec_ref_pic_marking_write;
use crate::src::encoder::set::x264_8_sei_frame_packing_write;
use crate::src::encoder::set::x264_8_sei_mastering_display_write;
use crate::src::encoder::set::x264_8_sei_pic_timing_write;
use crate::src::encoder::set::x264_8_sei_recovery_point_write;
use crate::src::encoder::set::x264_8_sei_version_write;
use crate::src::encoder::set::x264_8_sei_write;
use crate::src::encoder::set::x264_8_sps_init;
use crate::src::encoder::set::x264_8_sps_init_reconfigurable;
use crate::src::encoder::set::x264_8_sps_init_scaling_list;
use crate::src::encoder::set::x264_8_sps_write;
use crate::src::encoder::set::x264_8_validate_levels;

pub use crate::osdep_h::WORD_SIZE;
pub use crate::src::common::macroblock::cabac_ctx_block_cat_e;
pub use crate::src::common::macroblock::mb_class_e;
pub use crate::src::common::macroblock::mb_partition_e;
pub use crate::src::common::macroblock::x264_8_macroblock_bipred_init;
pub use crate::src::common::macroblock::x264_8_macroblock_cache_allocate;
pub use crate::src::common::macroblock::x264_8_macroblock_cache_free;
pub use crate::src::common::macroblock::x264_8_macroblock_cache_load_interlaced;
pub use crate::src::common::macroblock::x264_8_macroblock_cache_save;
pub use crate::src::common::macroblock::x264_8_macroblock_deblock_strength;
pub use crate::src::common::macroblock::x264_8_macroblock_slice_init;
pub use crate::src::common::macroblock::x264_8_macroblock_thread_allocate;
pub use crate::src::common::macroblock::x264_8_macroblock_thread_free;
pub use crate::src::common::macroblock::x264_8_macroblock_thread_init;
pub use crate::src::common::macroblock::B_8x8;
pub use crate::src::common::macroblock::DCT_CHROMAU_4x4;
pub use crate::src::common::macroblock::DCT_CHROMAU_8x8;
pub use crate::src::common::macroblock::DCT_CHROMAV_4x4;
pub use crate::src::common::macroblock::DCT_CHROMAV_8x8;
pub use crate::src::common::macroblock::DCT_LUMA_4x4;
pub use crate::src::common::macroblock::DCT_LUMA_8x8;
pub use crate::src::common::macroblock::D_16x16;
pub use crate::src::common::macroblock::D_16x8;
pub use crate::src::common::macroblock::D_8x16;
pub use crate::src::common::macroblock::D_8x8;
pub use crate::src::common::macroblock::D_BI_4x4;
pub use crate::src::common::macroblock::D_BI_4x8;
pub use crate::src::common::macroblock::D_BI_8x4;
pub use crate::src::common::macroblock::D_BI_8x8;
pub use crate::src::common::macroblock::D_DIRECT_8x8;
pub use crate::src::common::macroblock::D_L0_4x4;
pub use crate::src::common::macroblock::D_L0_4x8;
pub use crate::src::common::macroblock::D_L0_8x4;
pub use crate::src::common::macroblock::D_L0_8x8;
pub use crate::src::common::macroblock::D_L1_4x4;
pub use crate::src::common::macroblock::D_L1_4x8;
pub use crate::src::common::macroblock::D_L1_8x4;
pub use crate::src::common::macroblock::D_L1_8x8;
pub use crate::src::common::macroblock::I_16x16;
pub use crate::src::common::macroblock::I_4x4;
pub use crate::src::common::macroblock::I_8x8;
pub use crate::src::common::macroblock::P_8x8;
pub use crate::src::common::macroblock::B_BI_BI;
pub use crate::src::common::macroblock::B_BI_L0;
pub use crate::src::common::macroblock::B_BI_L1;
pub use crate::src::common::macroblock::B_DIRECT;
pub use crate::src::common::macroblock::B_L0_BI;
pub use crate::src::common::macroblock::B_L0_L0;
pub use crate::src::common::macroblock::B_L0_L1;
pub use crate::src::common::macroblock::B_L1_BI;
pub use crate::src::common::macroblock::B_L1_L0;
pub use crate::src::common::macroblock::B_L1_L1;
pub use crate::src::common::macroblock::B_SKIP;
pub use crate::src::common::macroblock::DCT_CHROMAU_AC;
pub use crate::src::common::macroblock::DCT_CHROMAU_DC;
pub use crate::src::common::macroblock::DCT_CHROMAV_AC;
pub use crate::src::common::macroblock::DCT_CHROMAV_DC;
pub use crate::src::common::macroblock::DCT_CHROMA_AC;
pub use crate::src::common::macroblock::DCT_CHROMA_DC;
pub use crate::src::common::macroblock::DCT_LUMA_AC;
pub use crate::src::common::macroblock::DCT_LUMA_DC;
pub use crate::src::common::macroblock::I_PCM;
pub use crate::src::common::macroblock::P_L0;
pub use crate::src::common::macroblock::P_SKIP;
pub use crate::src::common::macroblock::X264_MBTYPE_MAX;
pub use crate::src::common::macroblock::X264_PARTTYPE_MAX;
pub use crate::src::common::mc::weight_fn_t;
pub use crate::src::common::mc::x264_8_mc_init;
pub use crate::src::common::mc::x264_mc_functions_t_16;
pub use crate::src::common::mc::x264_weight_t;
pub use crate::src::common::pixel::x264_8_field_vsad;
pub use crate::src::common::pixel::x264_8_pixel_init;
pub use crate::src::common::pixel::x264_8_pixel_ssd_nv12;
pub use crate::src::common::pixel::x264_8_pixel_ssd_wxh;
pub use crate::src::common::pixel::x264_8_pixel_ssim_wxh;
pub use crate::src::common::pixel::x264_pixel_cmp_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x3_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x4_t;
pub use crate::src::common::pixel::x264_pixel_function_t;
pub use crate::src::common::pixel::PIXEL_16x16;
pub use crate::src::common::pixel::PIXEL_16x8;
pub use crate::src::common::pixel::PIXEL_2x2;
pub use crate::src::common::pixel::PIXEL_2x4;
pub use crate::src::common::pixel::PIXEL_2x8;
pub use crate::src::common::pixel::PIXEL_4x16;
pub use crate::src::common::pixel::PIXEL_4x2;
pub use crate::src::common::pixel::PIXEL_4x4;
pub use crate::src::common::pixel::PIXEL_4x8;
pub use crate::src::common::pixel::PIXEL_8x16;
pub use crate::src::common::pixel::PIXEL_8x4;
pub use crate::src::common::pixel::PIXEL_8x8;
pub use crate::src::common::predict::intra16x16_pred_e;
pub use crate::src::common::predict::intra4x4_pred_e;
pub use crate::src::common::predict::intra8x8_pred_e;
pub use crate::src::common::predict::intra_chroma_pred_e;
pub use crate::src::common::predict::x264_8_predict_16x16_init;
pub use crate::src::common::predict::x264_8_predict_4x4_init;
pub use crate::src::common::predict::x264_8_predict_8x16c_init;
pub use crate::src::common::predict::x264_8_predict_8x8_init;
pub use crate::src::common::predict::x264_8_predict_8x8c_init;
pub use crate::src::common::predict::x264_predict8x8_t;
pub use crate::src::common::predict::x264_predict_8x8_filter_t;
pub use crate::src::common::predict::x264_predict_t;
pub use crate::src::common::predict::I_PRED_16x16_DC;
pub use crate::src::common::predict::I_PRED_16x16_DC_128;
pub use crate::src::common::predict::I_PRED_16x16_DC_LEFT;
pub use crate::src::common::predict::I_PRED_16x16_DC_TOP;
pub use crate::src::common::predict::I_PRED_16x16_H;
pub use crate::src::common::predict::I_PRED_16x16_P;
pub use crate::src::common::predict::I_PRED_16x16_V;
pub use crate::src::common::predict::I_PRED_4x4_DC;
pub use crate::src::common::predict::I_PRED_4x4_DC_128;
pub use crate::src::common::predict::I_PRED_4x4_DC_LEFT;
pub use crate::src::common::predict::I_PRED_4x4_DC_TOP;
pub use crate::src::common::predict::I_PRED_4x4_DDL;
pub use crate::src::common::predict::I_PRED_4x4_DDR;
pub use crate::src::common::predict::I_PRED_4x4_H;
pub use crate::src::common::predict::I_PRED_4x4_HD;
pub use crate::src::common::predict::I_PRED_4x4_HU;
pub use crate::src::common::predict::I_PRED_4x4_V;
pub use crate::src::common::predict::I_PRED_4x4_VL;
pub use crate::src::common::predict::I_PRED_4x4_VR;
pub use crate::src::common::predict::I_PRED_8x8_DC;
pub use crate::src::common::predict::I_PRED_8x8_DC_128;
pub use crate::src::common::predict::I_PRED_8x8_DC_LEFT;
pub use crate::src::common::predict::I_PRED_8x8_DC_TOP;
pub use crate::src::common::predict::I_PRED_8x8_DDL;
pub use crate::src::common::predict::I_PRED_8x8_DDR;
pub use crate::src::common::predict::I_PRED_8x8_H;
pub use crate::src::common::predict::I_PRED_8x8_HD;
pub use crate::src::common::predict::I_PRED_8x8_HU;
pub use crate::src::common::predict::I_PRED_8x8_V;
pub use crate::src::common::predict::I_PRED_8x8_VL;
pub use crate::src::common::predict::I_PRED_8x8_VR;
pub use crate::src::common::predict::I_PRED_CHROMA_DC;
pub use crate::src::common::predict::I_PRED_CHROMA_DC_128;
pub use crate::src::common::predict::I_PRED_CHROMA_DC_LEFT;
pub use crate::src::common::predict::I_PRED_CHROMA_DC_TOP;
pub use crate::src::common::predict::I_PRED_CHROMA_H;
pub use crate::src::common::predict::I_PRED_CHROMA_P;
pub use crate::src::common::predict::I_PRED_CHROMA_V;
pub use crate::src::common::quant::x264_8_quant_init;
pub use crate::src::common::quant::x264_quant_function_t;
pub use crate::src::common::set::x264_8_cqm_delete;
pub use crate::src::common::set::x264_8_cqm_init;
pub use crate::src::common::set::x264_8_cqm_parse_file;
pub use crate::src::common::set::x264_pps_t;
pub use crate::src::common::set::x264_sps_t;
pub use crate::src::common::set::C2Rust_Unnamed_24;
pub use crate::src::common::set::C2Rust_Unnamed_25;
pub use crate::src::common::set::C2Rust_Unnamed_26;
pub use crate::src::encoder::encoder::macroblock_h::i_chroma_qp_table;
pub use crate::src::encoder::encoder::macroblock_h::x264_mb_partition_pixel_table;
pub use crate::src::encoder::encoder::macroblock_h::x264_mb_type_list_table;
pub use crate::src::encoder::encoder::osdep_h::endian_fix;
pub use crate::src::encoder::encoder::osdep_h::endian_fix32;
pub use crate::src::encoder::encoder::osdep_h::endian_fix64;
pub use crate::src::encoder::encoder::osdep_h::x264_is_regular_file;
pub use crate::src::encoder::encoder::pixel_h::x264_luma2chroma_pixel;
pub use crate::src::encoder::encoder::predict_h::x264_mb_chroma_pred_mode_fix;
pub use crate::src::encoder::encoder::predict_h::x264_mb_pred_mode16x16_fix;
pub use crate::src::encoder::encoder::predict_h::x264_mb_pred_mode4x4_fix;
use crate::src::encoder::ratecontrol::x264_8_adaptive_quant_frame;
use crate::src::encoder::ratecontrol::x264_8_hrd_fullness;
use crate::src::encoder::ratecontrol::x264_8_macroblock_tree_read;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_delete;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_end;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_init_reconfigurable;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_mb;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_mb_qp;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_new;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_qp;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_set_weights;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_start;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_summary;
use crate::src::encoder::ratecontrol::x264_8_ratecontrol_zone_init;
use crate::src::encoder::ratecontrol::x264_8_reference_build_list_optimal;
use crate::src::encoder::ratecontrol::x264_8_thread_sync_ratecontrol;
use crate::src::encoder::ratecontrol::x264_8_threads_distribute_ratecontrol;
use crate::src::encoder::ratecontrol::x264_8_threads_merge_ratecontrol;
use crate::stdlib::fabs;
use crate::stdlib::fstat;
use crate::stdlib::log10;
use crate::stdlib::log2f;
use crate::stdlib::pow;
use crate::stdlib::pthread_cond_broadcast;
use crate::stdlib::pthread_cond_destroy;
use crate::stdlib::pthread_cond_init;
pub use crate::stdlib::pthread_cond_t;
pub use crate::stdlib::pthread_condattr_t;
use crate::stdlib::pthread_mutex_destroy;
use crate::stdlib::pthread_mutex_init;
use crate::stdlib::pthread_mutex_lock;
pub use crate::stdlib::pthread_mutex_t;
use crate::stdlib::pthread_mutex_unlock;
pub use crate::stdlib::pthread_mutexattr_t;
pub use crate::stdlib::pthread_t;
pub use crate::stdlib::C2Rust_Unnamed_6;

use crate::stdlib::abs;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fileno;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fseeko;
pub use crate::stdlib::fwrite;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::int8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::snprintf;
pub use crate::stdlib::sprintf;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::uintptr_t;
pub use crate::stdlib::SEEK_SET;
pub use crate::stdlib::UINT16_MAX;
pub use crate::stdlib::UINT32_MAX;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__pthread_mutex_s;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
pub use crate::stdlib::stat;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
pub use crate::stdlib::timespec;
pub use crate::stdlib::_IO_FILE;

pub use crate::src::common::base::x264_param_cleanup;
use crate::src::common::tables::x264_cqm_avci100_1080_4ic;
use crate::src::common::tables::x264_cqm_avci100_1080i_8iy;
use crate::src::common::tables::x264_cqm_avci100_1080p_8iy;
use crate::src::common::tables::x264_cqm_avci100_720p_4ic;
use crate::src::common::tables::x264_cqm_avci100_720p_8iy;
use crate::src::common::tables::x264_cqm_avci300_2160p_4ic;
use crate::src::common::tables::x264_cqm_avci300_2160p_4iy;
use crate::src::common::tables::x264_cqm_avci300_2160p_8iy;
use crate::src::common::tables::x264_cqm_avci50_1080i_8iy;
use crate::src::common::tables::x264_cqm_avci50_4ic;
use crate::src::common::tables::x264_cqm_avci50_p_8iy;
use crate::src::common::tables::x264_cqm_jvt4i;
pub use crate::src::common::tables::x264_levels;
use crate::src::common::tables::x264_zero;
use crate::src::common::threadpool::x264_8_threadpool_delete;
use crate::src::common::threadpool::x264_8_threadpool_init;
use crate::src::common::threadpool::x264_8_threadpool_run;
use crate::src::common::threadpool::x264_8_threadpool_wait;
use crate::src::common::threadpool::x264_threadpool_t;
pub use crate::stdlib::__blkcnt_t;
pub use crate::stdlib::__blksize_t;
pub use crate::stdlib::__dev_t;
pub use crate::stdlib::__gid_t;
pub use crate::stdlib::__ino_t;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__int8_t;
pub use crate::stdlib::__mode_t;
pub use crate::stdlib::__nlink_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__pthread_cond_s;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__syscall_slong_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::FILE;
pub use crate::x264_h::nal_priority_e;
pub use crate::x264_h::nal_unit_type_e;
pub use crate::x264_h::pic_struct_e;
pub use crate::x264_h::x264_hrd_t;
pub use crate::x264_h::x264_image_properties_t;
pub use crate::x264_h::x264_image_t;
pub use crate::x264_h::x264_level_t;
pub use crate::x264_h::x264_nal_t;
pub use crate::x264_h::x264_param_t;
pub use crate::x264_h::x264_picture_t_3;
pub use crate::x264_h::x264_sei_payload_t;
pub use crate::x264_h::x264_sei_t;
pub use crate::x264_h::x264_zone_t;
pub use crate::x264_h::C2Rust_Unnamed_0;
pub use crate::x264_h::C2Rust_Unnamed_1;
pub use crate::x264_h::C2Rust_Unnamed_2;
pub use crate::x264_h::C2Rust_Unnamed_3;
pub use crate::x264_h::C2Rust_Unnamed_4;
pub use crate::x264_h::C2Rust_Unnamed_5;
pub use crate::x264_h::X264_ANALYSE_BSUB16x16;
pub use crate::x264_h::X264_ANALYSE_I4x4;
pub use crate::x264_h::X264_ANALYSE_I8x8;
pub use crate::x264_h::X264_ANALYSE_PSUB16x16;
pub use crate::x264_h::X264_ANALYSE_PSUB8x8;
pub use crate::x264_h::NAL_AUD;
pub use crate::x264_h::NAL_FILLER;
pub use crate::x264_h::NAL_PPS;
pub use crate::x264_h::NAL_PRIORITY_DISPOSABLE;
pub use crate::x264_h::NAL_PRIORITY_HIGH;
pub use crate::x264_h::NAL_PRIORITY_HIGHEST;
pub use crate::x264_h::NAL_PRIORITY_LOW;
pub use crate::x264_h::NAL_SEI;
pub use crate::x264_h::NAL_SLICE;
pub use crate::x264_h::NAL_SLICE_DPA;
pub use crate::x264_h::NAL_SLICE_DPB;
pub use crate::x264_h::NAL_SLICE_DPC;
pub use crate::x264_h::NAL_SLICE_IDR;
pub use crate::x264_h::NAL_SPS;
pub use crate::x264_h::NAL_UNKNOWN;
pub use crate::x264_h::PIC_STRUCT_AUTO;
pub use crate::x264_h::PIC_STRUCT_BOTTOM_TOP;
pub use crate::x264_h::PIC_STRUCT_BOTTOM_TOP_BOTTOM;
pub use crate::x264_h::PIC_STRUCT_DOUBLE;
pub use crate::x264_h::PIC_STRUCT_PROGRESSIVE;
pub use crate::x264_h::PIC_STRUCT_TOP_BOTTOM;
pub use crate::x264_h::PIC_STRUCT_TOP_BOTTOM_TOP;
pub use crate::x264_h::PIC_STRUCT_TRIPLE;
pub use crate::x264_h::X264_AVCINTRA_FLAVOR_SONY;
pub use crate::x264_h::X264_B_ADAPT_NONE;
pub use crate::x264_h::X264_B_ADAPT_TRELLIS;
pub use crate::x264_h::X264_B_PYRAMID_NONE;
pub use crate::x264_h::X264_B_PYRAMID_NORMAL;
pub use crate::x264_h::X264_B_PYRAMID_STRICT;
pub use crate::x264_h::X264_CPU_BMI1;
pub use crate::x264_h::X264_CPU_BMI2;
pub use crate::x264_h::X264_CPU_CACHELINE_64;
pub use crate::x264_h::X264_CPU_FMA3;
pub use crate::x264_h::X264_CPU_SSE2;
pub use crate::x264_h::X264_CPU_SSE2_IS_FAST;
pub use crate::x264_h::X264_CPU_SSE2_IS_SLOW;
pub use crate::x264_h::X264_CPU_SSE42;
pub use crate::x264_h::X264_CPU_SSSE3;
pub use crate::x264_h::X264_CQM_CUSTOM;
pub use crate::x264_h::X264_CQM_FLAT;
pub use crate::x264_h::X264_CSP_BGR;
pub use crate::x264_h::X264_CSP_I400;
pub use crate::x264_h::X264_CSP_I420;
pub use crate::x264_h::X264_CSP_I422;
pub use crate::x264_h::X264_CSP_I444;
pub use crate::x264_h::X264_CSP_MASK;
pub use crate::x264_h::X264_CSP_MAX;
pub use crate::x264_h::X264_CSP_NONE;
pub use crate::x264_h::X264_CSP_RGB;
pub use crate::x264_h::X264_DIRECT_PRED_AUTO;
pub use crate::x264_h::X264_DIRECT_PRED_NONE;
pub use crate::x264_h::X264_DIRECT_PRED_SPATIAL;
pub use crate::x264_h::X264_KEYINT_MAX_INFINITE;
pub use crate::x264_h::X264_KEYINT_MIN_AUTO;
pub use crate::x264_h::X264_LOG_DEBUG_1;
pub use crate::x264_h::X264_LOG_ERROR_1;
pub use crate::x264_h::X264_LOG_INFO;
pub use crate::x264_h::X264_LOG_WARNING_1;
pub use crate::x264_h::X264_ME_DIA;
pub use crate::x264_h::X264_ME_ESA;
pub use crate::x264_h::X264_ME_HEX;
pub use crate::x264_h::X264_ME_TESA;
pub use crate::x264_h::X264_ME_UMH;
pub use crate::x264_h::X264_NAL_HRD_CBR;
pub use crate::x264_h::X264_NAL_HRD_NONE;
pub use crate::x264_h::X264_NAL_HRD_VBR;
pub use crate::x264_h::X264_RC_ABR;
pub use crate::x264_h::X264_RC_CQP;
pub use crate::x264_h::X264_RC_CRF;
pub use crate::x264_h::X264_THREADS_AUTO;
pub use crate::x264_h::X264_TYPE_AUTO;
pub use crate::x264_h::X264_TYPE_B;
pub use crate::x264_h::X264_TYPE_BREF;
pub use crate::x264_h::X264_TYPE_I;
pub use crate::x264_h::X264_TYPE_IDR;
pub use crate::x264_h::X264_TYPE_KEYFRAME;
pub use crate::x264_h::X264_TYPE_P;
pub use crate::x264_h::X264_WEIGHTP_NONE;
pub use crate::x264_h::X264_WEIGHTP_SIMPLE;
pub use crate::x264_h::X264_WEIGHTP_SMART;
extern "C" {

    pub fn x264_8_nal_encode(
        h: *mut crate::src::common::common::x264_t,
        dst: *mut crate::stdlib::uint8_t,
        nal: *mut crate::x264_h::x264_nal_t,
    );

    pub fn x264_8_macroblock_cache_load_progressive(
        h: *mut crate::src::common::common::x264_t,
        i_mb_x: ::core::ffi::c_int,
        i_mb_y: ::core::ffi::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_359 {
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
        let mut mse: ::core::ffi::c_double = sqe
            / ((crate::src::common::common::PIXEL_MAX * crate::src::common::common::PIXEL_MAX)
                as ::core::ffi::c_double
                * size);
        if mse <= 0.0000000001f64 {
            return 100 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        return -10.0f64 * crate::stdlib::log10(mse);
    }
}

unsafe extern "C" fn calc_ssim_db(mut ssim: ::core::ffi::c_double) -> ::core::ffi::c_double {
    unsafe {
        let mut inv_ssim: ::core::ffi::c_double =
            1 as ::core::ffi::c_int as ::core::ffi::c_double - ssim;
        if inv_ssim <= 0.0000000001f64 {
            return 100 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        return -10.0f64 * crate::stdlib::log10(inv_ssim);
    }
}

unsafe extern "C" fn threadpool_wait_all(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).param.i_threads {
            if (*(*h).thread[i as usize]).b_thread_active != 0 {
                (*(*h).thread[i as usize]).b_thread_active = 0 as ::core::ffi::c_int;
                if (crate::src::common::threadpool::x264_8_threadpool_wait(
                    (*h).threadpool,
                    (*h).thread[i as usize] as *mut ::core::ffi::c_void,
                ) as crate::stdlib::intptr_t)
                    < 0 as crate::stdlib::intptr_t
                {
                    return -(1 as ::core::ffi::c_int);
                }
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn frame_dump(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut f: *mut crate::stdlib::FILE = crate::stdlib::fopen(
            (*h).param.psz_dump_yuv,
            b"r+b\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut crate::stdlib::FILE;
        if f.is_null() {
            return;
        }
        if (*h).param.b_sliced_threads != 0 {
            threadpool_wait_all(h);
        }
        let mut frame_size: ::core::ffi::c_int = (*h).param.i_height
            * (*h).param.i_width
            * ::core::mem::size_of::<crate::src::common::common::pixel>() as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    (*h).param.i_height
                        * (*h).param.i_width
                        * ::core::mem::size_of::<crate::src::common::common::pixel>()
                            as ::core::ffi::c_int
                        >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                            || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            + (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                });
        if crate::stdlib::fseeko(
            f,
            (*(*h).fdec).i_frame as crate::stdlib::__off64_t
                * frame_size as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_SET,
        ) == 0
        {
            let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p
                < (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                })
            {
                let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while y < (*h).param.i_height {
                    crate::stdlib::fwrite(
                        (*(&raw mut (*(*h).fdec).plane
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(p as isize))
                        .offset(
                            (y * *(&raw mut (*(*h).fdec).i_stride as *mut ::core::ffi::c_int)
                                .offset(p as isize)) as isize,
                        ) as *mut crate::src::common::common::pixel
                            as *const ::core::ffi::c_void,
                        crate::src::common::common::SIZEOF_PIXEL
                            as crate::__stddef_size_t_h::size_t,
                        (*h).param.i_width as crate::__stddef_size_t_h::size_t,
                        f,
                    );
                    y += 1;
                }
                p += 1;
            }
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
            {
                let mut cw: ::core::ffi::c_int = (*h).param.i_width >> 1 as ::core::ffi::c_int;
                let mut ch: ::core::ffi::c_int = (*h).param.i_height
                    >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                let mut planeu: *mut crate::src::common::common::pixel =
                    crate::src::common::base::x264_malloc(
                        (2 as ::core::ffi::c_int
                            * (cw * ch * crate::src::common::common::SIZEOF_PIXEL
                                + 32 as ::core::ffi::c_int))
                            as crate::stdlib::int64_t,
                    ) as *mut crate::src::common::common::pixel;
                if !planeu.is_null() {
                    let mut planev: *mut crate::src::common::common::pixel =
                        planeu.offset((cw * ch) as isize).offset(
                            (32 as ::core::ffi::c_int / crate::src::common::common::SIZEOF_PIXEL)
                                as isize,
                        );
                    (*h).mc
                        .plane_copy_deinterleave
                        .expect("non-null function pointer")(
                        planeu,
                        cw as crate::stdlib::intptr_t,
                        planev,
                        cw as crate::stdlib::intptr_t,
                        (*(*h).fdec).plane[1 as ::core::ffi::c_int as usize],
                        (*(*h).fdec).i_stride[1 as ::core::ffi::c_int as usize]
                            as crate::stdlib::intptr_t,
                        cw,
                        ch,
                    );
                    crate::stdlib::fwrite(
                        planeu as *const ::core::ffi::c_void,
                        1 as crate::__stddef_size_t_h::size_t,
                        (cw * ch * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                        f,
                    );
                    crate::stdlib::fwrite(
                        planev as *const ::core::ffi::c_void,
                        1 as crate::__stddef_size_t_h::size_t,
                        (cw * ch * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                        f,
                    );
                    crate::src::common::base::x264_free(planeu as *mut ::core::ffi::c_void);
                }
            }
        }
        crate::stdlib::fclose(f);
    }
}

unsafe extern "C" fn slice_header_init(
    mut h: *mut crate::src::common::common::x264_t,
    mut sh: *mut crate::src::common::common::x264_slice_header_t,
    mut sps: *mut crate::src::common::set::x264_sps_t,
    mut pps: *mut crate::src::common::set::x264_pps_t,
    mut i_idr_pic_id: ::core::ffi::c_int,
    mut i_frame: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let mut param: *mut crate::x264_h::x264_param_t = &raw mut (*h).param;
        (*sh).sps = sps;
        (*sh).pps = pps;
        (*sh).i_first_mb = 0 as ::core::ffi::c_int;
        (*sh).i_last_mb = (*h).mb.i_mb_count - 1 as ::core::ffi::c_int;
        (*sh).i_pps_id = (*pps).i_id;
        (*sh).i_frame_num = i_frame;
        (*sh).b_mbaff = (*h).param.b_interlaced;
        (*sh).b_field_pic = 0 as ::core::ffi::c_int;
        (*sh).b_bottom_field = 0 as ::core::ffi::c_int;
        (*sh).i_idr_pic_id = i_idr_pic_id;
        (*sh).i_poc = 0 as ::core::ffi::c_int;
        (*sh).i_delta_poc_bottom = 0 as ::core::ffi::c_int;
        (*sh).i_delta_poc[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        (*sh).i_delta_poc[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        (*sh).i_redundant_pic_cnt = 0 as ::core::ffi::c_int;
        (*h).mb.b_direct_auto_write = ((*h).param.analyse.i_direct_mv_pred
            == crate::x264_h::X264_DIRECT_PRED_AUTO
            && (*h).param.i_bframe != 0
            && ((*h).param.rc.b_stat_write != 0 || (*h).param.rc.b_stat_read == 0))
            as ::core::ffi::c_int;
        if (*h).mb.b_direct_auto_read == 0
            && (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
        {
            if (*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .i_poc_l0ref0
                == (*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_poc
            {
                if (*h).mb.b_direct_auto_write != 0 {
                    (*sh).b_direct_spatial_mv_pred = ((*h).stat.i_direct_score
                        [1 as ::core::ffi::c_int as usize]
                        > (*h).stat.i_direct_score[0 as ::core::ffi::c_int as usize])
                        as ::core::ffi::c_int;
                } else {
                    (*sh).b_direct_spatial_mv_pred = ((*param).analyse.i_direct_mv_pred
                        == crate::x264_h::X264_DIRECT_PRED_SPATIAL)
                        as ::core::ffi::c_int;
                }
            } else {
                (*h).mb.b_direct_auto_write = 0 as ::core::ffi::c_int;
                (*sh).b_direct_spatial_mv_pred = 1 as ::core::ffi::c_int;
            }
        }
        (*sh).b_num_ref_idx_override = 0 as ::core::ffi::c_int;
        (*sh).i_num_ref_idx_l0_active = 1 as ::core::ffi::c_int;
        (*sh).i_num_ref_idx_l1_active = 1 as ::core::ffi::c_int;
        (*sh).b_ref_pic_list_reordering[0 as ::core::ffi::c_int as usize] =
            (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize];
        (*sh).b_ref_pic_list_reordering[1 as ::core::ffi::c_int as usize] =
            (*h).b_ref_reorder[1 as ::core::ffi::c_int as usize];
        let mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while list < 2 as ::core::ffi::c_int {
            if (*sh).b_ref_pic_list_reordering[list as usize] != 0 {
                let mut pred_frame_num: ::core::ffi::c_int = i_frame;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*h).i_ref[list as usize] {
                    let mut diff: ::core::ffi::c_int =
                        (*(*h).fref[list as usize][i as usize]).i_frame_num - pred_frame_num;
                    (*sh).ref_pic_list_order[list as usize][i as usize].idc =
                        (diff > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                    (*sh).ref_pic_list_order[list as usize][i as usize].arg =
                        crate::stdlib::abs(diff) - 1 as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << (*sps).i_log2_max_frame_num)
                                - 1 as ::core::ffi::c_int;
                    pred_frame_num = (*(*h).fref[list as usize][i as usize]).i_frame_num;
                    i += 1;
                }
            }
            list += 1;
        }
        (*sh).i_cabac_init_idc = (*param).i_cabac_init_idc;
        (*sh).i_qp = if i_qp
            < 51 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
        {
            i_qp
        } else {
            51 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
        };
        (*sh).i_qp_delta = (*sh).i_qp - (*pps).i_pic_init_qp;
        (*sh).b_sp_for_swidth = 0 as ::core::ffi::c_int;
        (*sh).i_qs_delta = 0 as ::core::ffi::c_int;
        let mut deblock_thresh: ::core::ffi::c_int = i_qp
            + 2 as ::core::ffi::c_int
                * (if (*param).i_deblocking_filter_alphac0 < (*param).i_deblocking_filter_beta {
                    (*param).i_deblocking_filter_alphac0
                } else {
                    (*param).i_deblocking_filter_beta
                });
        if (*param).b_deblocking_filter != 0
            && ((*h).mb.b_variable_qp != 0 || (15 as ::core::ffi::c_int) < deblock_thresh)
        {
            (*sh).i_disable_deblocking_filter_idc = if (*param).b_sliced_threads != 0 {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        } else {
            (*sh).i_disable_deblocking_filter_idc = 1 as ::core::ffi::c_int;
        }
        (*sh).i_alpha_c0_offset = (*param).i_deblocking_filter_alphac0 * 2 as ::core::ffi::c_int;
        (*sh).i_beta_offset = (*param).i_deblocking_filter_beta * 2 as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn slice_header_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut sh: *mut crate::src::common::common::x264_slice_header_t,
    mut i_nal_ref_idc: ::core::ffi::c_int,
) {
    unsafe {
        if (*sh).b_mbaff != 0 {
            let mut first_x: ::core::ffi::c_int = (*sh).i_first_mb % (*(*sh).sps).i_mb_width;
            let mut first_y: ::core::ffi::c_int = (*sh).i_first_mb / (*(*sh).sps).i_mb_width;
            '_c2rust_label: {
                if first_y & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                } else {
                    crate::stdlib::__assert_fail(
                        b"(first_y&1) == 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                        219 as ::core::ffi::c_uint,
                        b"void slice_header_write(bs_t *, x264_slice_header_t *, int)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            bs_write_ue_big(
                s,
                (2 as ::core::ffi::c_int * first_x
                    + (*(*sh).sps).i_mb_width * (first_y & !(1 as ::core::ffi::c_int))
                    + (first_y & 1 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
        } else {
            bs_write_ue_big(s, (*sh).i_first_mb as ::core::ffi::c_uint);
        }
        bs_write_ue_big(
            s,
            ((*sh).i_type + 5 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        bs_write_ue_big(s, (*sh).i_pps_id as ::core::ffi::c_uint);
        bs_write(
            s,
            (*(*sh).sps).i_log2_max_frame_num,
            ((*sh).i_frame_num
                & ((1 as ::core::ffi::c_int) << (*(*sh).sps).i_log2_max_frame_num)
                    - 1 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        );
        if (*(*sh).sps).b_frame_mbs_only == 0 {
            bs_write1(s, (*sh).b_field_pic as crate::stdlib::uint32_t);
            if (*sh).b_field_pic != 0 {
                bs_write1(s, (*sh).b_bottom_field as crate::stdlib::uint32_t);
            }
        }
        if (*sh).i_idr_pic_id >= 0 as ::core::ffi::c_int {
            bs_write_ue_big(s, (*sh).i_idr_pic_id as ::core::ffi::c_uint);
        }
        if (*(*sh).sps).i_poc_type == 0 as ::core::ffi::c_int {
            bs_write(
                s,
                (*(*sh).sps).i_log2_max_poc_lsb,
                ((*sh).i_poc
                    & ((1 as ::core::ffi::c_int) << (*(*sh).sps).i_log2_max_poc_lsb)
                        - 1 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
            );
            if (*(*sh).pps).b_pic_order != 0 && (*sh).b_field_pic == 0 {
                bs_write_se(s, (*sh).i_delta_poc_bottom);
            }
        }
        if (*(*sh).pps).b_redundant_pic_cnt != 0 {
            bs_write_ue_big(s, (*sh).i_redundant_pic_cnt as ::core::ffi::c_uint);
        }
        if (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            bs_write1(s, (*sh).b_direct_spatial_mv_pred as crate::stdlib::uint32_t);
        }
        if (*sh).i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            || (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
        {
            bs_write1(s, (*sh).b_num_ref_idx_override as crate::stdlib::uint32_t);
            if (*sh).b_num_ref_idx_override != 0 {
                bs_write_ue_big(
                    s,
                    ((*sh).i_num_ref_idx_l0_active - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
                if (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    bs_write_ue_big(
                        s,
                        ((*sh).i_num_ref_idx_l1_active - 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint,
                    );
                }
            }
        }
        if (*sh).i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            bs_write1(
                s,
                (*sh).b_ref_pic_list_reordering[0 as ::core::ffi::c_int as usize]
                    as crate::stdlib::uint32_t,
            );
            if (*sh).b_ref_pic_list_reordering[0 as ::core::ffi::c_int as usize] != 0 {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*sh).i_num_ref_idx_l0_active {
                    bs_write_ue_big(
                        s,
                        (*sh).ref_pic_list_order[0 as ::core::ffi::c_int as usize][i as usize].idc
                            as ::core::ffi::c_uint,
                    );
                    bs_write_ue_big(
                        s,
                        (*sh).ref_pic_list_order[0 as ::core::ffi::c_int as usize][i as usize].arg
                            as ::core::ffi::c_uint,
                    );
                    i += 1;
                }
                bs_write_ue_big(s, 3 as ::core::ffi::c_uint);
            }
        }
        if (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            bs_write1(
                s,
                (*sh).b_ref_pic_list_reordering[1 as ::core::ffi::c_int as usize]
                    as crate::stdlib::uint32_t,
            );
            if (*sh).b_ref_pic_list_reordering[1 as ::core::ffi::c_int as usize] != 0 {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < (*sh).i_num_ref_idx_l1_active {
                    bs_write_ue_big(
                        s,
                        (*sh).ref_pic_list_order[1 as ::core::ffi::c_int as usize][i_0 as usize].idc
                            as ::core::ffi::c_uint,
                    );
                    bs_write_ue_big(
                        s,
                        (*sh).ref_pic_list_order[1 as ::core::ffi::c_int as usize][i_0 as usize].arg
                            as ::core::ffi::c_uint,
                    );
                    i_0 += 1;
                }
                bs_write_ue_big(s, 3 as ::core::ffi::c_uint);
            }
        }
        (*sh).b_weighted_pred = 0 as ::core::ffi::c_int;
        if (*(*sh).pps).b_weighted_pred != 0
            && (*sh).i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
        {
            (*sh).b_weighted_pred = (!(*sh).weight[0 as ::core::ffi::c_int as usize]
                [0 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
                || !(*sh).weight[0 as ::core::ffi::c_int as usize]
                    [1 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
                || !(*sh).weight[0 as ::core::ffi::c_int as usize]
                    [2 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()) as ::core::ffi::c_int;
            bs_write_ue_big(
                s,
                (*sh).weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                    .i_denom as ::core::ffi::c_uint,
            );
            if (*(*sh).sps).i_chroma_format_idc != 0 {
                bs_write_ue_big(
                    s,
                    (*sh).weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                        .i_denom as ::core::ffi::c_uint,
                );
            }
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < (*sh).i_num_ref_idx_l0_active {
                let mut luma_weight_l0_flag: ::core::ffi::c_int =
                    !(*sh).weight[i_1 as usize][0 as ::core::ffi::c_int as usize]
                        .weightfn
                        .is_null() as ::core::ffi::c_int;
                bs_write1(s, luma_weight_l0_flag as crate::stdlib::uint32_t);
                if luma_weight_l0_flag != 0 {
                    bs_write_se(
                        s,
                        (*sh).weight[i_1 as usize][0 as ::core::ffi::c_int as usize].i_scale
                            as ::core::ffi::c_int,
                    );
                    bs_write_se(
                        s,
                        (*sh).weight[i_1 as usize][0 as ::core::ffi::c_int as usize].i_offset
                            as ::core::ffi::c_int,
                    );
                }
                if (*(*sh).sps).i_chroma_format_idc != 0 {
                    let mut chroma_weight_l0_flag: ::core::ffi::c_int =
                        (!(*sh).weight[i_1 as usize][1 as ::core::ffi::c_int as usize]
                            .weightfn
                            .is_null()
                            || !(*sh).weight[i_1 as usize][2 as ::core::ffi::c_int as usize]
                                .weightfn
                                .is_null()) as ::core::ffi::c_int;
                    bs_write1(s, chroma_weight_l0_flag as crate::stdlib::uint32_t);
                    if chroma_weight_l0_flag != 0 {
                        let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        while j < 3 as ::core::ffi::c_int {
                            bs_write_se(
                                s,
                                (*sh).weight[i_1 as usize][j as usize].i_scale
                                    as ::core::ffi::c_int,
                            );
                            bs_write_se(
                                s,
                                (*sh).weight[i_1 as usize][j as usize].i_offset
                                    as ::core::ffi::c_int,
                            );
                            j += 1;
                        }
                    }
                }
                i_1 += 1;
            }
        } else {
            (*(*sh).pps).b_weighted_bipred == 1 as ::core::ffi::c_int
                && (*sh).i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int;
        }
        if i_nal_ref_idc != 0 as ::core::ffi::c_int {
            if (*sh).i_idr_pic_id >= 0 as ::core::ffi::c_int {
                bs_write1(s, 0 as crate::stdlib::uint32_t);
                bs_write1(s, 0 as crate::stdlib::uint32_t);
            } else {
                bs_write1(
                    s,
                    ((*sh).i_mmco_command_count > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as crate::stdlib::uint32_t,
                );
                if (*sh).i_mmco_command_count > 0 as ::core::ffi::c_int {
                    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_2 < (*sh).i_mmco_command_count {
                        bs_write_ue_big(s, 1 as ::core::ffi::c_uint);
                        bs_write_ue_big(
                            s,
                            ((*sh).mmco[i_2 as usize].i_difference_of_pic_nums
                                - 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                        );
                        i_2 += 1;
                    }
                    bs_write_ue_big(s, 0 as ::core::ffi::c_uint);
                }
            }
        }
        if (*(*sh).pps).b_cabac != 0
            && (*sh).i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
        {
            bs_write_ue_big(s, (*sh).i_cabac_init_idc as ::core::ffi::c_uint);
        }
        bs_write_se(s, (*sh).i_qp_delta);
        if (*(*sh).pps).b_deblocking_filter_control != 0 {
            bs_write_ue_big(
                s,
                (*sh).i_disable_deblocking_filter_idc as ::core::ffi::c_uint,
            );
            if (*sh).i_disable_deblocking_filter_idc != 1 as ::core::ffi::c_int {
                bs_write_se(s, (*sh).i_alpha_c0_offset >> 1 as ::core::ffi::c_int);
                bs_write_se(s, (*sh).i_beta_offset >> 1 as ::core::ffi::c_int);
            }
        }
    }
}

unsafe extern "C" fn bitstream_check_buffer_internal(
    mut h: *mut crate::src::common::common::x264_t,
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
            if size > crate::limits_h::INT_MAX - (*h).out.i_bitstream {
                return -(1 as ::core::ffi::c_int);
            }
            let mut buf_size: ::core::ffi::c_int = (*h).out.i_bitstream + size;
            let mut buf: *mut crate::stdlib::uint8_t =
                crate::src::common::base::x264_malloc(buf_size as crate::stdlib::int64_t)
                    as *mut crate::stdlib::uint8_t;
            if buf.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
            let mut aligned_size: ::core::ffi::c_int =
                (*h).out.i_bitstream & !(15 as ::core::ffi::c_int);
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
            let mut delta: crate::stdlib::intptr_t =
                buf.offset_from((*h).out.p_bitstream) as crate::stdlib::intptr_t;
            (*h).out.bs.p_start = (*h).out.bs.p_start.offset(delta as isize);
            (*h).out.bs.p = (*h).out.bs.p.offset(delta as isize);
            (*h).out.bs.p_end = buf.offset(buf_size as isize);
            (*h).cabac.p_start = (*h).cabac.p_start.offset(delta as isize);
            (*h).cabac.p = (*h).cabac.p.offset(delta as isize);
            (*h).cabac.p_end = buf.offset(buf_size as isize);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i <= i_nal {
                let ref mut c2rust_fresh2 = (*(*h).out.nal.offset(i as isize)).p_payload;
                *c2rust_fresh2 = (*c2rust_fresh2).offset(delta as isize);
                i += 1;
            }
            crate::src::common::base::x264_free((*h).out.p_bitstream as *mut ::core::ffi::c_void);
            (*h).out.p_bitstream = buf;
            (*h).out.i_bitstream = buf_size;
        }
        return 0 as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn bitstream_check_buffer(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut max_row_size: ::core::ffi::c_int =
            ((2500 as ::core::ffi::c_int) << (*h).sh.b_mbaff) * (*h).mb.i_mb_width;
        return bitstream_check_buffer_internal(
            h,
            max_row_size,
            (*h).param.b_cabac,
            (*h).out.i_nal,
        );
    }
}

unsafe extern "C" fn bitstream_check_buffer_filler(
    mut h: *mut crate::src::common::common::x264_t,
    mut filler: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        filler += 32 as ::core::ffi::c_int;
        return bitstream_check_buffer_internal(
            h,
            filler,
            0 as ::core::ffi::c_int,
            -(1 as ::core::ffi::c_int),
        );
    }
}

unsafe extern "C" fn validate_parameters(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_open: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if (*h).param.pf_log.is_none() {
            crate::src::common::base::x264_log_internal(
                crate::x264_h::X264_LOG_ERROR_1,
                b"pf_log not set! did you forget to call x264_param_default?\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*h).param.b_interlaced = ((*h).param.b_interlaced != 0) as ::core::ffi::c_int;
        if (*h).param.i_width <= 0 as ::core::ffi::c_int
            || (*h).param.i_height <= 0 as ::core::ffi::c_int
            || (*h).param.i_width > MAX_RESOLUTION
            || (*h).param.i_height > MAX_RESOLUTION
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"invalid width x height (%dx%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*h).param.i_width,
                (*h).param.i_height,
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut i_csp: ::core::ffi::c_int = (*h).param.i_csp & crate::x264_h::X264_CSP_MASK;
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            != crate::src::common::base::CHROMA_400 as ::core::ffi::c_int
            && i_csp == crate::x264_h::X264_CSP_I400
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"not compiled with 4:0:0 support\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            != crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
            && i_csp >= crate::x264_h::X264_CSP_I420
            && i_csp < crate::x264_h::X264_CSP_I422
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"not compiled with 4:2:0 support\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            != crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
            && i_csp >= crate::x264_h::X264_CSP_I422
            && i_csp < crate::x264_h::X264_CSP_I444
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"not compiled with 4:2:2 support\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            != crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            && i_csp >= crate::x264_h::X264_CSP_I444
            && i_csp <= crate::x264_h::X264_CSP_RGB
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"not compiled with 4:4:4 support\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if i_csp <= crate::x264_h::X264_CSP_NONE || i_csp >= crate::x264_h::X264_CSP_MAX {
            crate::src::common::common::x264_8_log(

                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"invalid CSP (only I400/I420/YV12/NV12/NV21/I422/YV16/NV16/YUYV/UYVY/I444/YV24/BGR/BGRA/RGB supported)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut w_mod: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut h_mod: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
            << ((*h).param.b_interlaced != 0 || (*h).param.b_fake_interlaced != 0)
                as ::core::ffi::c_int;
        if i_csp == crate::x264_h::X264_CSP_I400 {
            (*h).param.analyse.i_chroma_qp_offset = 0 as ::core::ffi::c_int;
            (*h).param.analyse.b_chroma_me = 0 as ::core::ffi::c_int;
            (*h).param.vui.i_colmatrix = 2 as ::core::ffi::c_int;
        } else if i_csp < crate::x264_h::X264_CSP_I444 {
            w_mod = 2 as ::core::ffi::c_int;
            if i_csp < crate::x264_h::X264_CSP_I422 {
                h_mod *= 2 as ::core::ffi::c_int;
            }
        }
        if (*h).param.i_width % w_mod != 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"width not divisible by %d (%dx%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                w_mod,
                (*h).param.i_width,
                (*h).param.i_height,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.i_height % h_mod != 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"height not divisible by %d (%dx%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                h_mod,
                (*h).param.i_width,
                (*h).param.i_height,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.crop_rect.i_left < 0 as ::core::ffi::c_int
            || (*h).param.crop_rect.i_left >= (*h).param.i_width
            || (*h).param.crop_rect.i_right < 0 as ::core::ffi::c_int
            || (*h).param.crop_rect.i_right >= (*h).param.i_width
            || (*h).param.crop_rect.i_top < 0 as ::core::ffi::c_int
            || (*h).param.crop_rect.i_top >= (*h).param.i_height
            || (*h).param.crop_rect.i_bottom < 0 as ::core::ffi::c_int
            || (*h).param.crop_rect.i_bottom >= (*h).param.i_height
            || (*h).param.crop_rect.i_left + (*h).param.crop_rect.i_right >= (*h).param.i_width
            || (*h).param.crop_rect.i_top + (*h).param.crop_rect.i_bottom >= (*h).param.i_height
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"invalid crop-rect %d,%d,%d,%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*h).param.crop_rect.i_left,
                (*h).param.crop_rect.i_top,
                (*h).param.crop_rect.i_right,
                (*h).param.crop_rect.i_bottom,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.crop_rect.i_left % w_mod != 0
            || (*h).param.crop_rect.i_right % w_mod != 0
            || (*h).param.crop_rect.i_top % h_mod != 0
            || (*h).param.crop_rect.i_bottom % h_mod != 0
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"crop-rect %d,%d,%d,%d not divisible by %dx%d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*h).param.crop_rect.i_left,
                (*h).param.crop_rect.i_top,
                (*h).param.crop_rect.i_right,
                (*h).param.crop_rect.i_bottom,
                w_mod,
                h_mod,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.vui.i_sar_width <= 0 as ::core::ffi::c_int
            || (*h).param.vui.i_sar_height <= 0 as ::core::ffi::c_int
        {
            (*h).param.vui.i_sar_width = 0 as ::core::ffi::c_int;
            (*h).param.vui.i_sar_height = 0 as ::core::ffi::c_int;
        }
        if (*h).param.i_threads == crate::x264_h::X264_THREADS_AUTO {
            (*h).param.i_threads = crate::src::common::cpu::x264_cpu_num_processors()
                * (if (*h).param.b_sliced_threads != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    3 as ::core::ffi::c_int
                })
                / 2 as ::core::ffi::c_int;
            let mut max_threads: ::core::ffi::c_int = if 1 as ::core::ffi::c_int
                > ((*h).param.i_height + 15 as ::core::ffi::c_int)
                    / 16 as ::core::ffi::c_int
                    / 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                ((*h).param.i_height + 15 as ::core::ffi::c_int)
                    / 16 as ::core::ffi::c_int
                    / 2 as ::core::ffi::c_int
            };
            (*h).param.i_threads = if (*h).param.i_threads < max_threads {
                (*h).param.i_threads
            } else {
                max_threads
            };
        }
        let mut max_sliced_threads: ::core::ffi::c_int = if 1 as ::core::ffi::c_int
            > ((*h).param.i_height + 15 as ::core::ffi::c_int)
                / 16 as ::core::ffi::c_int
                / 4 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            ((*h).param.i_height + 15 as ::core::ffi::c_int)
                / 16 as ::core::ffi::c_int
                / 4 as ::core::ffi::c_int
        };
        if (*h).param.i_threads > 1 as ::core::ffi::c_int {
            if (*h).param.b_sliced_threads != 0 {
                (*h).param.i_threads = if (*h).param.i_threads < max_sliced_threads {
                    (*h).param.i_threads
                } else {
                    max_sliced_threads
                };
            }
        }
        (*h).param.i_threads = x264_clip3(
            (*h).param.i_threads,
            1 as ::core::ffi::c_int,
            crate::src::common::base::X264_THREAD_MAX,
        );
        if (*h).param.i_threads == 1 as ::core::ffi::c_int {
            (*h).param.b_sliced_threads = 0 as ::core::ffi::c_int;
            (*h).param.i_lookahead_threads = 1 as ::core::ffi::c_int;
        }
        (*h).i_thread_frames = if (*h).param.b_sliced_threads != 0 {
            1 as ::core::ffi::c_int
        } else {
            (*h).param.i_threads
        };
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            (*h).param.nalu_process = None;
        }
        if (*h).param.b_opencl != 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"OpenCL: not compiled with OpenCL support, disabling\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.b_opencl = 0 as ::core::ffi::c_int;
            if !(*h).param.opencl_device_id.is_null() && (*h).param.i_opencl_device != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"OpenCL: device id and device skip count configured; dropping skip\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                (*h).param.i_opencl_device = 0 as ::core::ffi::c_int;
            }
        }
        (*h).param.i_keyint_max = x264_clip3(
            (*h).param.i_keyint_max,
            1 as ::core::ffi::c_int,
            crate::x264_h::X264_KEYINT_MAX_INFINITE,
        );
        if (*h).param.i_keyint_max == 1 as ::core::ffi::c_int {
            (*h).param.b_intra_refresh = 0 as ::core::ffi::c_int;
            (*h).param.analyse.i_weighted_pred = 0 as ::core::ffi::c_int;
            (*h).param.i_frame_reference = 1 as ::core::ffi::c_int;
            (*h).param.i_dpb_size = 1 as ::core::ffi::c_int;
        }
        if (*h).param.i_frame_packing < -(1 as ::core::ffi::c_int)
            || (*h).param.i_frame_packing > 7 as ::core::ffi::c_int
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"ignoring unknown frame packing value\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*h).param.i_frame_packing = -(1 as ::core::ffi::c_int);
        }
        if (*h).param.i_frame_packing == 7 as ::core::ffi::c_int
            && (((*h).param.i_width - (*h).param.crop_rect.i_left - (*h).param.crop_rect.i_right)
                % 3 as ::core::ffi::c_int
                != 0
                || ((*h).param.i_height
                    - (*h).param.crop_rect.i_top
                    - (*h).param.crop_rect.i_bottom)
                    % 3 as ::core::ffi::c_int
                    != 0)
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"cropped resolution %dx%d not compatible with tile format frame packing\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*h).param.i_width - (*h).param.crop_rect.i_left - (*h).param.crop_rect.i_right,
                (*h).param.i_height - (*h).param.crop_rect.i_top - (*h).param.crop_rect.i_bottom,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.mastering_display.b_mastering_display != 0 {
            if (*h).param.mastering_display.i_green_x > crate::stdlib::UINT16_MAX
                || (*h).param.mastering_display.i_green_x < 0 as ::core::ffi::c_int
                || (*h).param.mastering_display.i_green_y > crate::stdlib::UINT16_MAX
                || (*h).param.mastering_display.i_green_y < 0 as ::core::ffi::c_int
                || (*h).param.mastering_display.i_blue_x > crate::stdlib::UINT16_MAX
                || (*h).param.mastering_display.i_blue_x < 0 as ::core::ffi::c_int
                || (*h).param.mastering_display.i_blue_y > crate::stdlib::UINT16_MAX
                || (*h).param.mastering_display.i_blue_y < 0 as ::core::ffi::c_int
                || (*h).param.mastering_display.i_red_x > crate::stdlib::UINT16_MAX
                || (*h).param.mastering_display.i_red_x < 0 as ::core::ffi::c_int
                || (*h).param.mastering_display.i_red_y > crate::stdlib::UINT16_MAX
                || (*h).param.mastering_display.i_red_y < 0 as ::core::ffi::c_int
                || (*h).param.mastering_display.i_white_x > crate::stdlib::UINT16_MAX
                || (*h).param.mastering_display.i_white_x < 0 as ::core::ffi::c_int
                || (*h).param.mastering_display.i_white_y > crate::stdlib::UINT16_MAX
                || (*h).param.mastering_display.i_white_y < 0 as ::core::ffi::c_int
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"mastering display xy coordinates out of range [0,%u]\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    crate::stdlib::UINT16_MAX,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if (*h).param.mastering_display.i_display_max
                > crate::stdlib::UINT32_MAX as crate::stdlib::int64_t
                || (*h).param.mastering_display.i_display_max < 0 as crate::stdlib::int64_t
                || (*h).param.mastering_display.i_display_min
                    > crate::stdlib::UINT32_MAX as crate::stdlib::int64_t
                || (*h).param.mastering_display.i_display_min < 0 as crate::stdlib::int64_t
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"mastering display brightness out of range [0,%u]\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    crate::stdlib::UINT32_MAX,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if (*h).param.mastering_display.i_display_min == 50000 as crate::stdlib::int64_t
                && (*h).param.mastering_display.i_display_max == 50000 as crate::stdlib::int64_t
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"mastering display min and max brightness cannot both be 50000\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
        if (*h).param.content_light_level.b_cll != 0
            && ((*h).param.content_light_level.i_max_cll > crate::stdlib::UINT16_MAX
                || (*h).param.content_light_level.i_max_cll < 0 as ::core::ffi::c_int
                || (*h).param.content_light_level.i_max_fall > crate::stdlib::UINT16_MAX
                || (*h).param.content_light_level.i_max_fall < 0 as ::core::ffi::c_int)
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"content light levels out of range [0,%u]\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                crate::stdlib::UINT16_MAX,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if b_open != 0 {
            let mut score: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            score +=
                ((*h).param.analyse.i_me_range == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            score += ((*h).param.rc.i_qp_step == 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
            score += ((*h).param.i_keyint_max == 12 as ::core::ffi::c_int) as ::core::ffi::c_int;
            score += ((*h).param.rc.i_qp_min == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
            score += ((*h).param.rc.i_qp_max == 31 as ::core::ffi::c_int) as ::core::ffi::c_int;
            score += ((*h).param.rc.f_qcompress as ::core::ffi::c_double == 0.5f64)
                as ::core::ffi::c_int;
            score +=
                (crate::stdlib::fabs((*h).param.rc.f_ip_factor as ::core::ffi::c_double - 1.25f64)
                    < 0.01f64) as ::core::ffi::c_int;
            score +=
                (crate::stdlib::fabs((*h).param.rc.f_pb_factor as ::core::ffi::c_double - 1.25f64)
                    < 0.01f64) as ::core::ffi::c_int;
            score += ((*h).param.analyse.inter == 0 as ::core::ffi::c_uint
                && (*h).param.analyse.i_subpel_refine == 8 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if score >= 5 as ::core::ffi::c_int {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"broken ffmpeg default settings detected\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"use an encoding preset (e.g. -vpre medium)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"preset usage: -vpre <speed> -vpre <profile>\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"speed presets are listed in x264 --help\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"profile is optional; x264 defaults to high\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
        if (*h).param.rc.i_rc_method < 0 as ::core::ffi::c_int
            || (*h).param.rc.i_rc_method > 2 as ::core::ffi::c_int
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"no ratecontrol method specified\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.b_interlaced != 0 {
            (*h).param.b_pic_struct = 1 as ::core::ffi::c_int;
        }
        if (*h).param.i_avcintra_class != 0 {
            if crate::internal::BIT_DEPTH != 10 as ::core::ffi::c_int {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"%2d-bit AVC-Intra is not widely compatible\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    crate::internal::BIT_DEPTH,
                );
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"10-bit x264 is required to encode AVC-Intra\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            let mut type_0: ::core::ffi::c_int =
                if (*h).param.i_avcintra_class == 480 as ::core::ffi::c_int {
                    4 as ::core::ffi::c_int
                } else if (*h).param.i_avcintra_class == 300 as ::core::ffi::c_int {
                    3 as ::core::ffi::c_int
                } else if (*h).param.i_avcintra_class == 200 as ::core::ffi::c_int {
                    2 as ::core::ffi::c_int
                } else if (*h).param.i_avcintra_class == 100 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else if (*h).param.i_avcintra_class == 50 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int
                } else {
                    -(1 as ::core::ffi::c_int)
                };
            if type_0 < 0 as ::core::ffi::c_int {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"Invalid AVC-Intra class\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            } else if type_0 > 2 as ::core::ffi::c_int
                && (*h).param.i_avcintra_flavor != crate::x264_h::X264_AVCINTRA_FLAVOR_SONY
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"AVC-Intra %d only supported by Sony XAVC flavor\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.i_avcintra_class,
                );
                return -(1 as ::core::ffi::c_int);
            }
            static mut avcintra_lut: [[[C2Rust_Unnamed_359; 7]; 2]; 5] = unsafe {
                [
                    [
                        [
                            C2Rust_Unnamed_359 {
    fps_num:  60000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  912 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  50 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  1100 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  912 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  1100 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  24000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  912 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                        ],
                        [
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  1 as crate::stdlib::uint8_t,
    frame_size:  1820 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_1080i_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  1 as crate::stdlib::uint8_t,
    frame_size:  2196 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_1080i_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  60000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  1820 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  1820 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  50 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  2196 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  2196 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  24000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  1820 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci50_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci50_p_8iy as *const crate::stdlib::uint8_t,
},
                        ],
                    ],
                    [
                        [
                            C2Rust_Unnamed_359 {
    fps_num:  60000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  1848 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  50 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  2224 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  1848 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  2224 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  24000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  1848 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                        ],
                        [
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  1 as crate::stdlib::uint8_t,
    frame_size:  3692 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080i_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  1 as crate::stdlib::uint8_t,
    frame_size:  4444 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080i_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  60000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  3692 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  3692 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  50 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  4444 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  4444 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  24000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  3692 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                        ],
                    ],
                    [
                        [
                            C2Rust_Unnamed_359 {
    fps_num:  60000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  3724 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  50 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  4472 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_720p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                        ],
                        [
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  1 as crate::stdlib::uint8_t,
    frame_size:  7444 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080i_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  1 as crate::stdlib::uint8_t,
    frame_size:  8940 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080i_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  60000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  7444 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  7444 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  50 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  8940 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  8940 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  24000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  7444 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci100_1080_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci100_1080p_8iy as *const crate::stdlib::uint8_t,
},
                        ],
                    ],
                    [
                        [
                            C2Rust_Unnamed_359 {
    fps_num:  60000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  9844 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  50 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  9844 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  9844 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  9844 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  24000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  9844 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                        ],
                        [C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
}; 7],
                    ],
                    [
                        [
                            C2Rust_Unnamed_359 {
    fps_num:  60000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  15700 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  50 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  15700 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  30000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  15700 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  25 as crate::stdlib::uint16_t,
    fps_den:  1 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  15700 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  24000 as crate::stdlib::uint16_t,
    fps_den:  1001 as crate::stdlib::uint16_t,
    interlaced:  0 as crate::stdlib::uint8_t,
    frame_size:  15700 as crate::stdlib::uint16_t,
    cqm_4iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4iy as *const crate::stdlib::uint8_t,
    cqm_4ic:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_4ic as *const crate::stdlib::uint8_t,
    cqm_8iy:  &raw const crate::src::common::tables::x264_cqm_avci300_2160p_8iy as *const crate::stdlib::uint8_t,
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                            C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
},
                        ],
                        [C2Rust_Unnamed_359 {
    fps_num:  0,
    fps_den:  0,
    interlaced:  0,
    frame_size:  0,
    cqm_4iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_4ic:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
    cqm_8iy:  ::core::ptr::null::<crate::stdlib::uint8_t>(),
}; 7],
                    ],
                ]
            };
            let mut res: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            if i_csp >= crate::x264_h::X264_CSP_I420
                && i_csp < crate::x264_h::X264_CSP_I422
                && type_0 == 0
            {
                if (*h).param.i_width == 1440 as ::core::ffi::c_int
                    && (*h).param.i_height == 1080 as ::core::ffi::c_int
                {
                    res = 1 as ::core::ffi::c_int;
                } else if (*h).param.i_width == 960 as ::core::ffi::c_int
                    && (*h).param.i_height == 720 as ::core::ffi::c_int
                {
                    res = 0 as ::core::ffi::c_int;
                }
            } else if i_csp >= crate::x264_h::X264_CSP_I422
                && i_csp < crate::x264_h::X264_CSP_I444
                && type_0 != 0
            {
                if type_0 < 3 as ::core::ffi::c_int {
                    if (*h).param.i_width == 1920 as ::core::ffi::c_int
                        && (*h).param.i_height == 1080 as ::core::ffi::c_int
                    {
                        res = 1 as ::core::ffi::c_int;
                    } else if (*h).param.i_width == 2048 as ::core::ffi::c_int
                        && (*h).param.i_height == 1080 as ::core::ffi::c_int
                    {
                        res = 1 as ::core::ffi::c_int;
                    } else if (*h).param.i_width == 1280 as ::core::ffi::c_int
                        && (*h).param.i_height == 720 as ::core::ffi::c_int
                    {
                        res = 0 as ::core::ffi::c_int;
                    }
                } else if (*h).param.i_width == 3840 as ::core::ffi::c_int
                    && (*h).param.i_height == 2160 as ::core::ffi::c_int
                {
                    res = 0 as ::core::ffi::c_int;
                } else if (*h).param.i_width == 4096 as ::core::ffi::c_int
                    && (*h).param.i_height == 2160 as ::core::ffi::c_int
                {
                    res = 0 as ::core::ffi::c_int;
                }
            } else {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"Invalid colorspace for AVC-Intra %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.i_avcintra_class,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if res < 0 as ::core::ffi::c_int {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"Resolution %dx%d invalid for AVC-Intra %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.i_width,
                    (*h).param.i_height,
                    (*h).param.i_avcintra_class,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if (*h).param.nalu_process.is_some() {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"nalu_process is not supported in AVC-Intra mode\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if (*h).param.b_repeat_headers == 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"Separate headers not supported in AVC-Intra mode\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            let mut i: ::core::ffi::c_int = 0;
            let mut fps_num: crate::stdlib::uint32_t = (*h).param.i_fps_num;
            let mut fps_den: crate::stdlib::uint32_t = (*h).param.i_fps_den;
            crate::src::common::base::x264_reduce_fraction(&raw mut fps_num, &raw mut fps_den);
            i = 0 as ::core::ffi::c_int;
            while i < 7 as ::core::ffi::c_int {
                if avcintra_lut[type_0 as usize][res as usize][i as usize].fps_num
                    as crate::stdlib::uint32_t
                    == fps_num
                    && avcintra_lut[type_0 as usize][res as usize][i as usize].fps_den
                        as crate::stdlib::uint32_t
                        == fps_den
                    && avcintra_lut[type_0 as usize][res as usize][i as usize].interlaced
                        as ::core::ffi::c_int
                        == (*h).param.b_interlaced
                {
                    break;
                }
                i += 1;
            }
            if i == 7 as ::core::ffi::c_int {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"FPS %d/%d%c not compatible with AVC-Intra %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).param.i_fps_num,
                    (*h).param.i_fps_den,
                    if (*h).param.b_interlaced != 0 {
                        'i' as i32
                    } else {
                        'p' as i32
                    },
                    (*h).param.i_avcintra_class,
                );
                return -(1 as ::core::ffi::c_int);
            }
            (*h).param.i_keyint_max = 1 as ::core::ffi::c_int;
            (*h).param.b_intra_refresh = 0 as ::core::ffi::c_int;
            (*h).param.analyse.i_weighted_pred = 0 as ::core::ffi::c_int;
            (*h).param.i_frame_reference = 1 as ::core::ffi::c_int;
            (*h).param.i_dpb_size = 1 as ::core::ffi::c_int;
            (*h).param.b_bluray_compat = 0 as ::core::ffi::c_int;
            (*h).param.b_vfr_input = 0 as ::core::ffi::c_int;
            (*h).param.b_aud = 1 as ::core::ffi::c_int;
            (*h).param.vui.i_chroma_loc = 0 as ::core::ffi::c_int;
            (*h).param.i_nal_hrd = crate::x264_h::X264_NAL_HRD_NONE;
            (*h).param.b_deblocking_filter = 0 as ::core::ffi::c_int;
            (*h).param.b_stitchable = 1 as ::core::ffi::c_int;
            (*h).param.b_pic_struct = 0 as ::core::ffi::c_int;
            (*h).param.analyse.b_transform_8x8 = 1 as ::core::ffi::c_int;
            (*h).param.analyse.intra = crate::x264_h::X264_ANALYSE_I8x8;
            (*h).param.analyse.i_chroma_qp_offset = if type_0 > 2 as ::core::ffi::c_int {
                -(4 as ::core::ffi::c_int)
            } else if res != 0 && type_0 != 0 {
                3 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            };
            (*h).param.b_cabac = (type_0 == 0) as ::core::ffi::c_int;
            (*h).param.rc.i_vbv_buffer_size = avcintra_lut[type_0 as usize][res as usize]
                [i as usize]
                .frame_size as ::core::ffi::c_int;
            (*h).param.rc.i_bitrate = ((*h).param.rc.i_vbv_buffer_size as crate::stdlib::uint32_t)
                .wrapping_mul(fps_num)
                .wrapping_div(fps_den) as ::core::ffi::c_int;
            (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
            (*h).param.rc.i_rc_method = crate::x264_h::X264_RC_ABR;
            (*h).param.rc.f_vbv_buffer_init = 1.0f32;
            (*h).param.rc.b_filler = 1 as ::core::ffi::c_int;
            (*h).param.i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            crate::stdlib::memcpy(
                &raw mut (*h).param.cqm_4iy as *mut crate::stdlib::uint8_t
                    as *mut ::core::ffi::c_void,
                avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_4iy
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>()
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*h).param.cqm_4ic as *mut crate::stdlib::uint8_t
                    as *mut ::core::ffi::c_void,
                avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_4ic
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>()
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*h).param.cqm_8iy as *mut crate::stdlib::uint8_t
                    as *mut ::core::ffi::c_void,
                avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_8iy
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[crate::stdlib::uint8_t; 64]>()
                    as crate::__stddef_size_t_h::size_t,
            );
            if (*h).param.i_avcintra_flavor == crate::x264_h::X264_AVCINTRA_FLAVOR_SONY {
                (*h).param.i_slice_count = 8 as ::core::ffi::c_int;
                if (*h).param.b_sliced_threads != 0 {
                    (*h).param.i_threads = (*h).param.i_slice_count;
                }
            } else {
                (*h).param.i_slice_max_mbs = ((*h).param.i_width + 15 as ::core::ffi::c_int)
                    / 16 as ::core::ffi::c_int
                    * (((*h).param.i_height + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int)
                    / 10 as ::core::ffi::c_int;
                (*h).param.i_slice_max_size = 0 as ::core::ffi::c_int;
                if (*h).param.b_sliced_threads != 0 {
                    if res != 0 {
                        (*h).param.i_threads = if (2 as ::core::ffi::c_int) < (*h).param.i_threads {
                            2 as ::core::ffi::c_int
                        } else {
                            (*h).param.i_threads
                        };
                    } else {
                        (*h).param.i_threads = if (5 as ::core::ffi::c_int) < (*h).param.i_threads {
                            5 as ::core::ffi::c_int
                        } else {
                            (*h).param.i_threads
                        };
                        if (*h).param.i_threads < 5 as ::core::ffi::c_int {
                            (*h).param.i_threads = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                (*h).param.rc.i_qp_min = if (*h).param.rc.i_qp_min
                    > 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int
                {
                    (*h).param.rc.i_qp_min
                } else {
                    6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int
                };
            }
            if type_0 != 0 {
                (*h).param.vui.i_sar_height = 1 as ::core::ffi::c_int;
                (*h).param.vui.i_sar_width = (*h).param.vui.i_sar_height;
            } else {
                (*h).param.vui.i_sar_width = 4 as ::core::ffi::c_int;
                (*h).param.vui.i_sar_height = 3 as ::core::ffi::c_int;
            }
        }
        (*h).param.rc.f_rf_constant = x264_clip3f(
            (*h).param.rc.f_rf_constant as ::core::ffi::c_double,
            -crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_double,
            51 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        (*h).param.rc.f_rf_constant_max = x264_clip3f(
            (*h).param.rc.f_rf_constant_max as ::core::ffi::c_double,
            -crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_double,
            51 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        (*h).param.rc.i_qp_constant = x264_clip3(
            (*h).param.rc.i_qp_constant,
            -(1 as ::core::ffi::c_int),
            crate::src::common::common::QP_MAX,
        );
        (*h).param.analyse.i_subpel_refine = x264_clip3(
            (*h).param.analyse.i_subpel_refine,
            0 as ::core::ffi::c_int,
            11 as ::core::ffi::c_int,
        );
        (*h).param.rc.f_ip_factor = x264_clip3f(
            (*h).param.rc.f_ip_factor as ::core::ffi::c_double,
            0.01f64,
            10.0f64,
        ) as ::core::ffi::c_float;
        (*h).param.rc.f_pb_factor = x264_clip3f(
            (*h).param.rc.f_pb_factor as ::core::ffi::c_double,
            0.01f64,
            10.0f64,
        ) as ::core::ffi::c_float;
        if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF {
            (*h).param.rc.i_qp_constant = ((*h).param.rc.f_rf_constant
                + crate::src::common::common::QP_BD_OFFSET as ::core::ffi::c_float)
                as ::core::ffi::c_int;
            (*h).param.rc.i_bitrate = 0 as ::core::ffi::c_int;
        }
        if b_open != 0
            && ((*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CQP
                || (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF)
            && (*h).param.rc.i_qp_constant == 0 as ::core::ffi::c_int
        {
            (*h).mb.b_lossless = 1 as ::core::ffi::c_int;
            (*h).param.i_cqm_preset = crate::x264_h::X264_CQM_FLAT;
            (*h).param.psz_cqm_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*h).param.rc.i_rc_method = crate::x264_h::X264_RC_CQP;
            (*h).param.rc.f_ip_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
            (*h).param.rc.f_pb_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
            (*h).param.analyse.b_psnr = 0 as ::core::ffi::c_int;
            (*h).param.analyse.b_ssim = 0 as ::core::ffi::c_int;
            (*h).param.analyse.i_chroma_qp_offset = 0 as ::core::ffi::c_int;
            (*h).param.analyse.i_trellis = 0 as ::core::ffi::c_int;
            (*h).param.analyse.b_fast_pskip = 0 as ::core::ffi::c_int;
            (*h).param.analyse.i_noise_reduction = 0 as ::core::ffi::c_int;
            (*h).param.analyse.b_psy = 0 as ::core::ffi::c_int;
            (*h).param.i_bframe = 0 as ::core::ffi::c_int;
            if (*h).param.b_cabac == 0
                && (*h).param.analyse.i_subpel_refine < 6 as ::core::ffi::c_int
            {
                (*h).param.analyse.b_transform_8x8 = 0 as ::core::ffi::c_int;
            }
        }
        if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CQP {
            let mut qp_p: ::core::ffi::c_float =
                (*h).param.rc.i_qp_constant as ::core::ffi::c_float;
            let mut qp_i: ::core::ffi::c_float = qp_p
                - 6 as ::core::ffi::c_int as ::core::ffi::c_float
                    * crate::stdlib::log2f((*h).param.rc.f_ip_factor);
            let mut qp_b: ::core::ffi::c_float = qp_p
                + 6 as ::core::ffi::c_int as ::core::ffi::c_float
                    * crate::stdlib::log2f((*h).param.rc.f_pb_factor);
            if qp_p < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"qp not specified\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            (*h).param.rc.i_qp_min = x264_clip3(
                (if qp_p < (if qp_i < qp_b { qp_i } else { qp_b }) {
                    qp_p
                } else if qp_i < qp_b {
                    qp_i
                } else {
                    qp_b
                }) as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                crate::src::common::common::QP_MAX,
            );
            (*h).param.rc.i_qp_max = x264_clip3(
                ((if qp_p > (if qp_i > qp_b { qp_i } else { qp_b }) {
                    qp_p
                } else {
                    (if qp_i > qp_b { qp_i } else { qp_b })
                }) as ::core::ffi::c_double
                    + 0.999f64) as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                crate::src::common::common::QP_MAX,
            );
            (*h).param.rc.i_aq_mode = 0 as ::core::ffi::c_int;
            (*h).param.rc.b_mb_tree = 0 as ::core::ffi::c_int;
            (*h).param.rc.i_bitrate = 0 as ::core::ffi::c_int;
        }
        (*h).param.rc.i_qp_max = x264_clip3(
            (*h).param.rc.i_qp_max,
            0 as ::core::ffi::c_int,
            crate::src::common::common::QP_MAX,
        );
        (*h).param.rc.i_qp_min = x264_clip3(
            (*h).param.rc.i_qp_min,
            0 as ::core::ffi::c_int,
            (*h).param.rc.i_qp_max,
        );
        (*h).param.rc.i_qp_step = x264_clip3(
            (*h).param.rc.i_qp_step,
            2 as ::core::ffi::c_int,
            crate::src::common::common::QP_MAX,
        );
        (*h).param.rc.i_bitrate = x264_clip3(
            (*h).param.rc.i_bitrate,
            0 as ::core::ffi::c_int,
            2000000 as ::core::ffi::c_int,
        );
        if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR && (*h).param.rc.i_bitrate == 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"bitrate not specified\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*h).param.rc.i_vbv_buffer_size = x264_clip3(
            (*h).param.rc.i_vbv_buffer_size,
            0 as ::core::ffi::c_int,
            2000000 as ::core::ffi::c_int,
        );
        (*h).param.rc.i_vbv_max_bitrate = x264_clip3(
            (*h).param.rc.i_vbv_max_bitrate,
            0 as ::core::ffi::c_int,
            2000000 as ::core::ffi::c_int,
        );
        (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
            (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            2000000 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        if (*h).param.rc.i_vbv_buffer_size != 0 {
            if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CQP {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"VBV is incompatible with constant QP, ignored.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*h).param.rc.i_vbv_max_bitrate = 0 as ::core::ffi::c_int;
                (*h).param.rc.i_vbv_buffer_size = 0 as ::core::ffi::c_int;
            } else if (*h).param.rc.i_vbv_max_bitrate == 0 as ::core::ffi::c_int {
                if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_WARNING_1,
                        b"VBV maxrate unspecified, assuming CBR\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
                } else {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_WARNING_1,
                        b"VBV bufsize set but maxrate unspecified, ignored\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*h).param.rc.i_vbv_buffer_size = 0 as ::core::ffi::c_int;
                }
            } else if (*h).param.rc.i_vbv_max_bitrate < (*h).param.rc.i_bitrate
                && (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"max bitrate less than average bitrate, assuming CBR\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*h).param.rc.i_bitrate = (*h).param.rc.i_vbv_max_bitrate;
            }
        } else if (*h).param.rc.i_vbv_max_bitrate != 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"VBV maxrate specified, but no bufsize, ignored\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.rc.i_vbv_max_bitrate = 0 as ::core::ffi::c_int;
        }
        (*h).param.i_slice_max_size = if (*h).param.i_slice_max_size > 0 as ::core::ffi::c_int {
            (*h).param.i_slice_max_size
        } else {
            0 as ::core::ffi::c_int
        };
        (*h).param.i_slice_max_mbs = if (*h).param.i_slice_max_mbs > 0 as ::core::ffi::c_int {
            (*h).param.i_slice_max_mbs
        } else {
            0 as ::core::ffi::c_int
        };
        (*h).param.i_slice_min_mbs = if (*h).param.i_slice_min_mbs > 0 as ::core::ffi::c_int {
            (*h).param.i_slice_min_mbs
        } else {
            0 as ::core::ffi::c_int
        };
        if (*h).param.i_slice_max_mbs != 0 {
            (*h).param.i_slice_min_mbs = if (*h).param.i_slice_min_mbs
                < (*h).param.i_slice_max_mbs / 2 as ::core::ffi::c_int
            {
                (*h).param.i_slice_min_mbs
            } else {
                (*h).param.i_slice_max_mbs / 2 as ::core::ffi::c_int
            };
        } else if (*h).param.i_slice_max_size == 0 {
            (*h).param.i_slice_min_mbs = 0 as ::core::ffi::c_int;
        }
        if (*h).param.b_interlaced != 0 && (*h).param.i_slice_min_mbs != 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"interlace + slice-min-mbs is not implemented\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.i_slice_min_mbs = 0 as ::core::ffi::c_int;
        }
        let mut mb_width: ::core::ffi::c_int =
            ((*h).param.i_width + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int;
        if (*h).param.i_slice_min_mbs > mb_width {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"slice-min-mbs > row mb size (%d) not implemented\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mb_width,
            );
            (*h).param.i_slice_min_mbs = mb_width;
        }
        let mut max_slices: ::core::ffi::c_int = ((*h).param.i_height
            + (((16 as ::core::ffi::c_int) << (*h).param.b_interlaced) - 1 as ::core::ffi::c_int))
            / ((16 as ::core::ffi::c_int) << (*h).param.b_interlaced);
        if (*h).param.b_sliced_threads != 0 {
            (*h).param.i_slice_count =
                x264_clip3((*h).param.i_threads, 0 as ::core::ffi::c_int, max_slices);
        } else {
            (*h).param.i_slice_count = x264_clip3(
                (*h).param.i_slice_count,
                0 as ::core::ffi::c_int,
                max_slices,
            );
            if (*h).param.i_slice_max_mbs != 0 || (*h).param.i_slice_max_size != 0 {
                (*h).param.i_slice_count = 0 as ::core::ffi::c_int;
            }
        }
        if (*h).param.i_slice_count_max > 0 as ::core::ffi::c_int {
            (*h).param.i_slice_count_max =
                if (*h).param.i_slice_count > (*h).param.i_slice_count_max {
                    (*h).param.i_slice_count
                } else {
                    (*h).param.i_slice_count_max
                };
        }
        if (*h).param.b_bluray_compat != 0 {
            (*h).param.i_bframe_pyramid = if (1 as ::core::ffi::c_int) < (*h).param.i_bframe_pyramid
            {
                1 as ::core::ffi::c_int
            } else {
                (*h).param.i_bframe_pyramid
            };
            (*h).param.i_bframe = if (*h).param.i_bframe < 3 as ::core::ffi::c_int {
                (*h).param.i_bframe
            } else {
                3 as ::core::ffi::c_int
            };
            (*h).param.b_aud = 1 as ::core::ffi::c_int;
            (*h).param.i_nal_hrd = if (*h).param.i_nal_hrd > 1 as ::core::ffi::c_int {
                (*h).param.i_nal_hrd
            } else {
                1 as ::core::ffi::c_int
            };
            (*h).param.i_slice_max_size = 0 as ::core::ffi::c_int;
            (*h).param.i_slice_max_mbs = 0 as ::core::ffi::c_int;
            (*h).param.b_intra_refresh = 0 as ::core::ffi::c_int;
            (*h).param.i_frame_reference = if (*h).param.i_frame_reference < 6 as ::core::ffi::c_int
            {
                (*h).param.i_frame_reference
            } else {
                6 as ::core::ffi::c_int
            };
            (*h).param.i_dpb_size = if (*h).param.i_dpb_size < 6 as ::core::ffi::c_int {
                (*h).param.i_dpb_size
            } else {
                6 as ::core::ffi::c_int
            };
            (*h).param.i_keyint_min = 1 as ::core::ffi::c_int;
            (*h).param.analyse.i_weighted_pred =
                if (*h).param.analyse.i_weighted_pred < 1 as ::core::ffi::c_int {
                    (*h).param.analyse.i_weighted_pred
                } else {
                    1 as ::core::ffi::c_int
                };
            if (*h).param.b_fake_interlaced != 0 {
                (*h).param.b_pic_struct = 1 as ::core::ffi::c_int;
            }
        }
        (*h).param.i_frame_reference = x264_clip3(
            (*h).param.i_frame_reference,
            1 as ::core::ffi::c_int,
            crate::src::common::base::X264_REF_MAX,
        );
        (*h).param.i_dpb_size = x264_clip3(
            (*h).param.i_dpb_size,
            1 as ::core::ffi::c_int,
            crate::src::common::base::X264_REF_MAX,
        );
        if (*h).param.i_scenecut_threshold < 0 as ::core::ffi::c_int {
            (*h).param.i_scenecut_threshold = 0 as ::core::ffi::c_int;
        }
        (*h).param.analyse.i_direct_mv_pred = x264_clip3(
            (*h).param.analyse.i_direct_mv_pred,
            crate::x264_h::X264_DIRECT_PRED_NONE,
            crate::x264_h::X264_DIRECT_PRED_AUTO,
        );
        if (*h).param.analyse.i_subpel_refine == 0
            && (*h).param.analyse.i_direct_mv_pred > crate::x264_h::X264_DIRECT_PRED_SPATIAL
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"subme=0 + direct=temporal is not supported\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_SPATIAL;
        }
        (*h).param.i_bframe = x264_clip3(
            (*h).param.i_bframe,
            0 as ::core::ffi::c_int,
            if (16 as ::core::ffi::c_int) < (*h).param.i_keyint_max - 1 as ::core::ffi::c_int {
                16 as ::core::ffi::c_int
            } else {
                (*h).param.i_keyint_max - 1 as ::core::ffi::c_int
            },
        );
        (*h).param.i_bframe_bias = x264_clip3(
            (*h).param.i_bframe_bias,
            -(90 as ::core::ffi::c_int),
            100 as ::core::ffi::c_int,
        );
        if (*h).param.i_bframe <= 1 as ::core::ffi::c_int {
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
            (*h).param.analyse.i_direct_mv_pred = 0 as ::core::ffi::c_int;
            (*h).param.analyse.b_weighted_bipred = 0 as ::core::ffi::c_int;
            (*h).param.b_open_gop = 0 as ::core::ffi::c_int;
        }
        if (*h).param.b_intra_refresh != 0
            && (*h).param.i_bframe_pyramid == crate::x264_h::X264_B_PYRAMID_NORMAL
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"b-pyramid normal + intra-refresh is not supported\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.i_bframe_pyramid = crate::x264_h::X264_B_PYRAMID_STRICT;
        }
        if (*h).param.b_intra_refresh != 0
            && ((*h).param.i_frame_reference > 1 as ::core::ffi::c_int
                || (*h).param.i_dpb_size > 1 as ::core::ffi::c_int)
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"ref > 1 + intra-refresh is not supported\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.i_frame_reference = 1 as ::core::ffi::c_int;
            (*h).param.i_dpb_size = 1 as ::core::ffi::c_int;
        }
        if (*h).param.b_intra_refresh != 0 && (*h).param.b_open_gop != 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"intra-refresh is not compatible with open-gop\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.b_open_gop = 0 as ::core::ffi::c_int;
        }
        if (*h).param.i_fps_num == 0 || (*h).param.i_fps_den == 0 {
            (*h).param.i_fps_num = 25 as crate::stdlib::uint32_t;
            (*h).param.i_fps_den = 1 as crate::stdlib::uint32_t;
        }
        let mut fps: ::core::ffi::c_float = (*h).param.i_fps_num as ::core::ffi::c_float
            / (*h).param.i_fps_den as ::core::ffi::c_float;
        if (*h).param.i_keyint_min == crate::x264_h::X264_KEYINT_MIN_AUTO {
            (*h).param.i_keyint_min = if ((*h).param.i_keyint_max / 10 as ::core::ffi::c_int)
                < fps as ::core::ffi::c_int
            {
                (*h).param.i_keyint_max / 10 as ::core::ffi::c_int
            } else {
                fps as ::core::ffi::c_int
            };
        }
        (*h).param.i_keyint_min = x264_clip3(
            (*h).param.i_keyint_min,
            1 as ::core::ffi::c_int,
            (*h).param.i_keyint_max / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        );
        (*h).param.rc.i_lookahead = x264_clip3(
            (*h).param.rc.i_lookahead,
            0 as ::core::ffi::c_int,
            crate::src::common::base::X264_LOOKAHEAD_MAX,
        );
        let mut maxrate: ::core::ffi::c_int =
            if (*h).param.rc.i_vbv_max_bitrate > (*h).param.rc.i_bitrate {
                (*h).param.rc.i_vbv_max_bitrate
            } else {
                (*h).param.rc.i_bitrate
            };
        let mut bufsize: ::core::ffi::c_float = if maxrate != 0 {
            (*h).param.rc.i_vbv_buffer_size as ::core::ffi::c_float
                / maxrate as ::core::ffi::c_float
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_float
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
            || !((*h).param.b_vfr_input != 0 || (*h).param.b_pulldown != 0)
        {
            (*h).param.i_timebase_num = (*h).param.i_fps_den;
            (*h).param.i_timebase_den = (*h).param.i_fps_num;
        }
        (*h).param.rc.f_qcompress = x264_clip3f(
            (*h).param.rc.f_qcompress as ::core::ffi::c_double,
            0.0f64,
            1.0f64,
        ) as ::core::ffi::c_float;
        if (*h).param.i_keyint_max == 1 as ::core::ffi::c_int
            || (*h).param.rc.f_qcompress == 1 as ::core::ffi::c_int as ::core::ffi::c_float
        {
            (*h).param.rc.b_mb_tree = 0 as ::core::ffi::c_int;
        }
        if (*h).param.b_intra_refresh == 0
            && (*h).param.i_keyint_max != crate::x264_h::X264_KEYINT_MAX_INFINITE
            && (*h).param.rc.i_lookahead == 0
            && (*h).param.rc.b_mb_tree != 0
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"lookaheadless mb-tree requires intra refresh or infinite keyint\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.rc.b_mb_tree = 0 as ::core::ffi::c_int;
        }
        if b_open != 0 && (*h).param.rc.b_stat_read != 0 {
            (*h).param.rc.i_lookahead = 0 as ::core::ffi::c_int;
        }
        if (*h).param.i_sync_lookahead < 0 as ::core::ffi::c_int {
            (*h).param.i_sync_lookahead = (*h).param.i_bframe + 1 as ::core::ffi::c_int;
        }
        (*h).param.i_sync_lookahead = if (*h).param.i_sync_lookahead < 250 as ::core::ffi::c_int {
            (*h).param.i_sync_lookahead
        } else {
            250 as ::core::ffi::c_int
        };
        if (*h).param.rc.b_stat_read != 0 || (*h).i_thread_frames == 1 as ::core::ffi::c_int {
            (*h).param.i_sync_lookahead = 0 as ::core::ffi::c_int;
        }
        (*h).param.i_deblocking_filter_alphac0 = x264_clip3(
            (*h).param.i_deblocking_filter_alphac0,
            -(6 as ::core::ffi::c_int),
            6 as ::core::ffi::c_int,
        );
        (*h).param.i_deblocking_filter_beta = x264_clip3(
            (*h).param.i_deblocking_filter_beta,
            -(6 as ::core::ffi::c_int),
            6 as ::core::ffi::c_int,
        );
        (*h).param.analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize] = x264_clip3(
            (*h).param.analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize],
            0 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
        );
        (*h).param.analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize] = x264_clip3(
            (*h).param.analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize],
            0 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
        );
        (*h).param.i_cabac_init_idc = x264_clip3(
            (*h).param.i_cabac_init_idc,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
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
        (*h).param.analyse.i_me_range = x264_clip3(
            (*h).param.analyse.i_me_range,
            4 as ::core::ffi::c_int,
            1024 as ::core::ffi::c_int,
        );
        if (*h).param.analyse.i_me_range > 16 as ::core::ffi::c_int
            && (*h).param.analyse.i_me_method <= crate::x264_h::X264_ME_HEX
        {
            (*h).param.analyse.i_me_range = 16 as ::core::ffi::c_int;
        }
        if (*h).param.analyse.i_me_method == crate::x264_h::X264_ME_TESA
            && ((*h).mb.b_lossless != 0
                || (*h).param.analyse.i_subpel_refine <= 1 as ::core::ffi::c_int)
        {
            (*h).param.analyse.i_me_method = crate::x264_h::X264_ME_ESA;
        }
        (*h).param.analyse.b_mixed_references = ((*h).param.analyse.b_mixed_references != 0
            && (*h).param.i_frame_reference > 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
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
        if (*h).param.analyse.b_transform_8x8 == 0 {
            (*h).param.analyse.inter &= !crate::x264_h::X264_ANALYSE_I8x8;
            (*h).param.analyse.intra &= !crate::x264_h::X264_ANALYSE_I8x8;
        }
        (*h).param.analyse.i_trellis = x264_clip3(
            (*h).param.analyse.i_trellis,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
        (*h).param.rc.i_aq_mode = x264_clip3(
            (*h).param.rc.i_aq_mode,
            0 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
        );
        (*h).param.rc.f_aq_strength = x264_clip3f(
            (*h).param.rc.f_aq_strength as ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            3 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        if (*h).param.rc.f_aq_strength == 0 as ::core::ffi::c_int as ::core::ffi::c_float {
            (*h).param.rc.i_aq_mode = 0 as ::core::ffi::c_int;
        }
        if (*h).param.i_log_level < crate::x264_h::X264_LOG_INFO {
            (*h).param.analyse.b_psnr = 0 as ::core::ffi::c_int;
            (*h).param.analyse.b_ssim = 0 as ::core::ffi::c_int;
        }
        if b_open != 0 && ((*h).param.analyse.b_psnr != 0 || (*h).param.analyse.b_ssim != 0) {
            let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if (*h).param.analyse.b_psy != 0 {
                s = (if (*h).param.analyse.b_psnr != 0 {
                    b"psnr\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"ssim\0".as_ptr() as *const ::core::ffi::c_char
                }) as *mut ::core::ffi::c_char;
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"--%s used with psy on: results will be invalid!\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    s,
                );
            } else if (*h).param.rc.i_aq_mode == 0 && (*h).param.analyse.b_ssim != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"--ssim used with AQ off: results will be invalid!\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                s = b"ssim\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            } else if (*h).param.rc.i_aq_mode != 0 && (*h).param.analyse.b_psnr != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"--psnr used with AQ on: results will be invalid!\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                s = b"psnr\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            if !s.is_null() {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"--tune %s should be used if attempting to benchmark %s!\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    s,
                    s,
                );
            }
        }
        if (*h).param.analyse.b_psy == 0 {
            (*h).param.analyse.f_psy_rd = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
            (*h).param.analyse.f_psy_trellis = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        }
        (*h).param.analyse.f_psy_rd = x264_clip3f(
            (*h).param.analyse.f_psy_rd as ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            10 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        (*h).param.analyse.f_psy_trellis = x264_clip3f(
            (*h).param.analyse.f_psy_trellis as ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            10 as ::core::ffi::c_int as ::core::ffi::c_double,
        ) as ::core::ffi::c_float;
        (*h).mb.i_psy_rd = if (*h).param.analyse.i_subpel_refine >= 6 as ::core::ffi::c_int {
            (((*h).param.analyse.f_psy_rd
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_float)
                as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        (*h).mb.i_psy_trellis = if (*h).param.analyse.i_trellis != 0 {
            (((*h).param.analyse.f_psy_trellis / 4 as ::core::ffi::c_int as ::core::ffi::c_float
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_float)
                as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        (*h).param.analyse.i_chroma_qp_offset = x264_clip3(
            (*h).param.analyse.i_chroma_qp_offset,
            -(32 as ::core::ffi::c_int),
            32 as ::core::ffi::c_int,
        );
        if b_open != 0
            && i_csp >= crate::x264_h::X264_CSP_I444
            && i_csp < crate::x264_h::X264_CSP_BGR
            && (*h).param.analyse.b_psy != 0
        {
            (*h).param.analyse.i_chroma_qp_offset += 6 as ::core::ffi::c_int;
        }
        if b_open != 0 && (*h).mb.i_psy_rd != 0 && (*h).param.i_avcintra_class == 0 {
            (*h).param.analyse.i_chroma_qp_offset -=
                if ((*h).param.analyse.f_psy_rd as ::core::ffi::c_double) < 0.25f64 {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                };
        }
        if b_open != 0 && (*h).mb.i_psy_trellis != 0 && (*h).param.i_avcintra_class == 0 {
            (*h).param.analyse.i_chroma_qp_offset -=
                if ((*h).param.analyse.f_psy_trellis as ::core::ffi::c_double) < 0.25f64 {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                };
        }
        (*h).param.analyse.i_chroma_qp_offset = x264_clip3(
            (*h).param.analyse.i_chroma_qp_offset,
            -(12 as ::core::ffi::c_int),
            12 as ::core::ffi::c_int,
        );
        if (*h).param.rc.i_aq_mode == 0 && (*h).param.rc.b_mb_tree != 0 {
            (*h).param.rc.i_aq_mode = 1 as ::core::ffi::c_int;
            (*h).param.rc.f_aq_strength = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        }
        (*h).param.analyse.i_noise_reduction = x264_clip3(
            (*h).param.analyse.i_noise_reduction,
            0 as ::core::ffi::c_int,
            (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
        );
        if (*h).param.analyse.i_subpel_refine >= 10 as ::core::ffi::c_int
            && ((*h).param.analyse.i_trellis != 2 as ::core::ffi::c_int
                || (*h).param.rc.i_aq_mode == 0)
        {
            (*h).param.analyse.i_subpel_refine = 9 as ::core::ffi::c_int;
        }
        if b_open != 0 {
            let mut l: *const crate::x264_h::x264_level_t =
                &raw const crate::src::common::tables::x264_levels
                    as *const crate::x264_h::x264_level_t;
            if (*h).param.i_level_idc < 0 as ::core::ffi::c_int {
                let mut maxrate_bak: ::core::ffi::c_int = (*h).param.rc.i_vbv_max_bitrate;
                if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
                    && (*h).param.rc.i_vbv_buffer_size <= 0 as ::core::ffi::c_int
                {
                    (*h).param.rc.i_vbv_max_bitrate =
                        (*h).param.rc.i_bitrate * 2 as ::core::ffi::c_int;
                }
                crate::src::encoder::set::x264_8_sps_init(
                    &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t
                        as *mut crate::src::common::set::x264_sps_t,
                    (*h).param.i_sps_id,
                    &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
                );
                loop {
                    (*h).param.i_level_idc = (*l).level_idc as ::core::ffi::c_int;
                    if !((*l.offset(1 as ::core::ffi::c_int as isize)).level_idc
                        as ::core::ffi::c_int
                        != 0
                        && crate::src::encoder::set::x264_8_validate_levels(
                            h as *mut crate::src::common::common::x264_t,
                            0 as ::core::ffi::c_int,
                        ) != 0
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
                if (*l).level_idc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_ERROR_1,
                        b"invalid level_idc: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).param.i_level_idc,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
            }
            if (*h).param.analyse.i_mv_range <= 0 as ::core::ffi::c_int {
                (*h).param.analyse.i_mv_range =
                    (*l).mv_range as ::core::ffi::c_int >> (*h).param.b_interlaced;
            } else {
                (*h).param.analyse.i_mv_range = x264_clip3(
                    (*h).param.analyse.i_mv_range,
                    32 as ::core::ffi::c_int,
                    8192 as ::core::ffi::c_int >> (*h).param.b_interlaced,
                );
            }
        }
        (*h).param.analyse.i_weighted_pred = x264_clip3(
            (*h).param.analyse.i_weighted_pred,
            crate::x264_h::X264_WEIGHTP_NONE,
            crate::x264_h::X264_WEIGHTP_SMART,
        );
        if (*h).param.i_lookahead_threads == crate::x264_h::X264_THREADS_AUTO {
            if (*h).param.b_sliced_threads != 0 {
                (*h).param.i_lookahead_threads = (*h).param.i_threads;
            } else {
                let mut badapt: ::core::ffi::c_int = ((*h).param.i_bframe_adaptive
                    == crate::x264_h::X264_B_ADAPT_TRELLIS)
                    as ::core::ffi::c_int;
                let mut subme: ::core::ffi::c_int =
                    (if ((*h).param.analyse.i_subpel_refine / 3 as ::core::ffi::c_int)
                        < 3 as ::core::ffi::c_int
                    {
                        (*h).param.analyse.i_subpel_refine / 3 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    }) + ((*h).param.analyse.i_subpel_refine > 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                let mut bframes: ::core::ffi::c_int =
                    if (((*h).param.i_bframe - 1 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int)
                        < 3 as ::core::ffi::c_int
                    {
                        ((*h).param.i_bframe - 1 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int
                    } else {
                        3 as ::core::ffi::c_int
                    };
                static mut lookahead_thread_div: [[[crate::stdlib::uint8_t; 4]; 5]; 2] = [
                    [
                        [
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                        [
                            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                        [
                            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                        [
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                        [
                            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                    ],
                    [
                        [
                            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                        [
                            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                        [
                            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                        [
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                        [
                            12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        ],
                    ],
                ];
                (*h).param.i_lookahead_threads = (*h).param.i_threads
                    / lookahead_thread_div[badapt as usize][subme as usize][bframes as usize]
                        as ::core::ffi::c_int;
                (*h).param.i_lookahead_threads = if (*h).param.i_lookahead_threads
                    < (*h).param.i_height / 128 as ::core::ffi::c_int
                {
                    (*h).param.i_lookahead_threads
                } else {
                    (*h).param.i_height / 128 as ::core::ffi::c_int
                };
            }
        }
        (*h).param.i_lookahead_threads = x264_clip3(
            (*h).param.i_lookahead_threads,
            1 as ::core::ffi::c_int,
            if max_sliced_threads < 16 as ::core::ffi::c_int {
                max_sliced_threads
            } else {
                16 as ::core::ffi::c_int
            },
        );
        if (*h).param.b_interlaced != 0 {
            if (*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"interlace + me=esa is not implemented\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*h).param.analyse.i_me_method = crate::x264_h::X264_ME_UMH;
            }
            if (*h).param.analyse.i_weighted_pred > 0 as ::core::ffi::c_int {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"interlace + weightp is not implemented\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*h).param.analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_NONE;
            }
        }
        if (*h).param.analyse.i_weighted_pred == 0
            && (*h).param.rc.b_mb_tree != 0
            && (*h).param.analyse.b_psy != 0
        {
            (*h).param.analyse.i_weighted_pred = crate::src::common::base::X264_WEIGHTP_FAKE;
        }
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            let mut r: ::core::ffi::c_int = (*h).param.analyse.i_mv_range_thread;
            let mut r2: ::core::ffi::c_int = 0;
            if r <= 0 as ::core::ffi::c_int {
                let mut max_range: ::core::ffi::c_int = ((*h).param.i_height
                    + crate::src::common::base::X264_THREAD_HEIGHT)
                    / (*h).i_thread_frames
                    - crate::src::common::base::X264_THREAD_HEIGHT;
                r = max_range / 2 as ::core::ffi::c_int;
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
            r2 = (r & !(15 as ::core::ffi::c_int))
                + (-crate::src::common::base::X264_THREAD_HEIGHT & 15 as ::core::ffi::c_int);
            if r2 < r {
                r2 += 16 as ::core::ffi::c_int;
            }
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_DEBUG_1,
                b"using mv_range_thread = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                r2,
            );
            (*h).param.analyse.i_mv_range_thread = r2;
        }
        if (*h).param.rc.f_rate_tolerance < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
            (*h).param.rc.f_rate_tolerance = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        }
        if (*h).param.rc.f_qblur < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
            (*h).param.rc.f_qblur = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        }
        if (*h).param.rc.f_complexity_blur < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
            (*h).param.rc.f_complexity_blur = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        }
        (*h).param.i_sps_id &= 31 as ::core::ffi::c_int;
        (*h).param.i_nal_hrd = x264_clip3(
            (*h).param.i_nal_hrd,
            crate::x264_h::X264_NAL_HRD_NONE,
            crate::x264_h::X264_NAL_HRD_CBR,
        );
        if (*h).param.i_nal_hrd != 0 && (*h).param.rc.i_vbv_buffer_size == 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"NAL HRD parameters require VBV parameters\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*h).param.i_nal_hrd = crate::x264_h::X264_NAL_HRD_NONE;
        }
        if (*h).param.i_nal_hrd == crate::x264_h::X264_NAL_HRD_CBR
            && ((*h).param.rc.i_bitrate != (*h).param.rc.i_vbv_max_bitrate
                || (*h).param.rc.i_vbv_max_bitrate == 0)
        {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"CBR HRD requires constant bitrate\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*h).param.i_nal_hrd = crate::x264_h::X264_NAL_HRD_VBR;
        }
        if (*h).param.i_nal_hrd == crate::x264_h::X264_NAL_HRD_CBR {
            (*h).param.rc.b_filler = 1 as ::core::ffi::c_int;
        }
        (*h).param.b_cabac = ((*h).param.b_cabac != 0) as ::core::ffi::c_int;
        (*h).param.b_constrained_intra =
            ((*h).param.b_constrained_intra != 0) as ::core::ffi::c_int;
        (*h).param.b_deblocking_filter =
            ((*h).param.b_deblocking_filter != 0) as ::core::ffi::c_int;
        (*h).param.b_deterministic = ((*h).param.b_deterministic != 0) as ::core::ffi::c_int;
        (*h).param.b_sliced_threads = ((*h).param.b_sliced_threads != 0) as ::core::ffi::c_int;
        (*h).param.b_interlaced = ((*h).param.b_interlaced != 0) as ::core::ffi::c_int;
        (*h).param.b_intra_refresh = ((*h).param.b_intra_refresh != 0) as ::core::ffi::c_int;
        (*h).param.b_aud = ((*h).param.b_aud != 0) as ::core::ffi::c_int;
        (*h).param.b_repeat_headers = ((*h).param.b_repeat_headers != 0) as ::core::ffi::c_int;
        (*h).param.b_annexb = ((*h).param.b_annexb != 0) as ::core::ffi::c_int;
        (*h).param.b_vfr_input = ((*h).param.b_vfr_input != 0) as ::core::ffi::c_int;
        (*h).param.b_pulldown = ((*h).param.b_pulldown != 0) as ::core::ffi::c_int;
        (*h).param.b_tff = ((*h).param.b_tff != 0) as ::core::ffi::c_int;
        (*h).param.b_pic_struct = ((*h).param.b_pic_struct != 0) as ::core::ffi::c_int;
        (*h).param.b_fake_interlaced = ((*h).param.b_fake_interlaced != 0) as ::core::ffi::c_int;
        (*h).param.b_open_gop = ((*h).param.b_open_gop != 0) as ::core::ffi::c_int;
        (*h).param.b_bluray_compat = ((*h).param.b_bluray_compat != 0) as ::core::ffi::c_int;
        (*h).param.b_stitchable = ((*h).param.b_stitchable != 0) as ::core::ffi::c_int;
        (*h).param.b_full_recon = ((*h).param.b_full_recon != 0) as ::core::ffi::c_int;
        (*h).param.b_opencl = ((*h).param.b_opencl != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_transform_8x8 =
            ((*h).param.analyse.b_transform_8x8 != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_weighted_bipred =
            ((*h).param.analyse.b_weighted_bipred != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_chroma_me =
            ((*h).param.analyse.b_chroma_me != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_mixed_references =
            ((*h).param.analyse.b_mixed_references != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_fast_pskip =
            ((*h).param.analyse.b_fast_pskip != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_dct_decimate =
            ((*h).param.analyse.b_dct_decimate != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_psy = ((*h).param.analyse.b_psy != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_psnr = ((*h).param.analyse.b_psnr != 0) as ::core::ffi::c_int;
        (*h).param.analyse.b_ssim = ((*h).param.analyse.b_ssim != 0) as ::core::ffi::c_int;
        (*h).param.rc.b_stat_write = ((*h).param.rc.b_stat_write != 0) as ::core::ffi::c_int;
        (*h).param.rc.b_stat_read = ((*h).param.rc.b_stat_read != 0) as ::core::ffi::c_int;
        (*h).param.rc.b_mb_tree = ((*h).param.rc.b_mb_tree != 0) as ::core::ffi::c_int;
        (*h).param.rc.b_filler = ((*h).param.rc.b_filler != 0) as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
}

pub const MAX_RESOLUTION: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;

unsafe extern "C" fn mbcmp_init(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut satd: ::core::ffi::c_int = ((*h).mb.b_lossless == 0
            && (*h).param.analyse.i_subpel_refine > 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.mbcmp as *mut crate::src::common::pixel::x264_pixel_cmp_t
                as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd as *mut crate::src::common::pixel::x264_pixel_cmp_t
            } else {
                &raw mut (*h).pixf.sad_aligned as *mut crate::src::common::pixel::x264_pixel_cmp_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_t; 8]>()
                as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.mbcmp_unaligned as *mut crate::src::common::pixel::x264_pixel_cmp_t
                as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd as *mut crate::src::common::pixel::x264_pixel_cmp_t
            } else {
                &raw mut (*h).pixf.sad as *mut crate::src::common::pixel::x264_pixel_cmp_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_t; 8]>()
                as crate::__stddef_size_t_h::size_t,
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
        (*h).pixf.intra_mbcmp_x9_4x4 =
            if (*h).param.b_cpu_independent != 0 || (*h).mb.b_lossless != 0 {
                None
            } else if satd != 0 {
                (*h).pixf.intra_satd_x9_4x4
            } else {
                (*h).pixf.intra_sad_x9_4x4
            };
        (*h).pixf.intra_mbcmp_x9_8x8 =
            if (*h).param.b_cpu_independent != 0 || (*h).mb.b_lossless != 0 {
                None
            } else if satd != 0 {
                (*h).pixf.intra_sa8d_x9_8x8
            } else {
                (*h).pixf.intra_sad_x9_8x8
            };
        satd &=
            ((*h).param.analyse.i_me_method == crate::x264_h::X264_ME_TESA) as ::core::ffi::c_int;
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.fpelcmp as *mut crate::src::common::pixel::x264_pixel_cmp_t
                as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd as *mut crate::src::common::pixel::x264_pixel_cmp_t
            } else {
                &raw mut (*h).pixf.sad as *mut crate::src::common::pixel::x264_pixel_cmp_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_t; 8]>()
                as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.fpelcmp_x3 as *mut crate::src::common::pixel::x264_pixel_cmp_x3_t
                as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd_x3 as *mut crate::src::common::pixel::x264_pixel_cmp_x3_t
            } else {
                &raw mut (*h).pixf.sad_x3 as *mut crate::src::common::pixel::x264_pixel_cmp_x3_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_x3_t; 7]>()
                as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut (*h).pixf.fpelcmp_x4 as *mut crate::src::common::pixel::x264_pixel_cmp_x4_t
                as *mut ::core::ffi::c_void,
            (if satd != 0 {
                &raw mut (*h).pixf.satd_x4 as *mut crate::src::common::pixel::x264_pixel_cmp_x4_t
            } else {
                &raw mut (*h).pixf.sad_x4 as *mut crate::src::common::pixel::x264_pixel_cmp_x4_t
            }) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::pixel::x264_pixel_cmp_x4_t; 7]>()
                as crate::__stddef_size_t_h::size_t,
        );
    }
}

unsafe extern "C" fn chroma_dsp_init(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        crate::stdlib::memcpy(
            &raw mut (*h).luma2chroma_pixel as *mut crate::stdlib::uint8_t
                as *mut ::core::ffi::c_void,
            &raw const *(&raw const x264_luma2chroma_pixel as *const [crate::stdlib::uint8_t; 7])
                .offset(crate::src::common::base::CHROMA_444 as ::core::ffi::c_int as isize)
                as *const crate::stdlib::uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 7]>()
                as crate::__stddef_size_t_h::size_t,
        );
        match crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
            0 => {
                (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_400;
            }
            1 => {
                crate::stdlib::memcpy(
                    &raw mut (*h).predict_chroma as *mut crate::src::common::predict::x264_predict_t
                        as *mut ::core::ffi::c_void,
                    &raw mut (*h).predict_8x8c as *mut crate::src::common::predict::x264_predict_t
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[crate::src::common::predict::x264_predict_t; 7]>()
                        as crate::__stddef_size_t_h::size_t,
                );
                (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_420;
                (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize] =
                    (*h).loopf.deblock_h_chroma_420;
                (*h).loopf.deblock_chroma_intra[0 as ::core::ffi::c_int as usize] =
                    (*h).loopf.deblock_h_chroma_420_intra;
                (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_chroma_420_mbaff;
                (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_chroma_420_intra_mbaff;
                (*h).pixf.intra_mbcmp_x3_chroma = (*h).pixf.intra_mbcmp_x3_8x8c;
                (*h).quantf.coeff_last[crate::src::common::macroblock::DCT_CHROMA_DC
                    as ::core::ffi::c_int as usize] = (*h).quantf.coeff_last4;
                (*h).quantf.coeff_level_run[crate::src::common::macroblock::DCT_CHROMA_DC
                    as ::core::ffi::c_int as usize] = (*h).quantf.coeff_level_run4;
            }
            2 => {
                crate::stdlib::memcpy(
                    &raw mut (*h).predict_chroma as *mut crate::src::common::predict::x264_predict_t
                        as *mut ::core::ffi::c_void,
                    &raw mut (*h).predict_8x16c as *mut crate::src::common::predict::x264_predict_t
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[crate::src::common::predict::x264_predict_t; 7]>()
                        as crate::__stddef_size_t_h::size_t,
                );
                (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_422;
                (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize] =
                    (*h).loopf.deblock_h_chroma_422;
                (*h).loopf.deblock_chroma_intra[0 as ::core::ffi::c_int as usize] =
                    (*h).loopf.deblock_h_chroma_422_intra;
                (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_chroma_422_mbaff;
                (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_chroma_422_intra_mbaff;
                (*h).pixf.intra_mbcmp_x3_chroma = (*h).pixf.intra_mbcmp_x3_8x16c;
                (*h).quantf.coeff_last[crate::src::common::macroblock::DCT_CHROMA_DC
                    as ::core::ffi::c_int as usize] = (*h).quantf.coeff_last8;
                (*h).quantf.coeff_level_run[crate::src::common::macroblock::DCT_CHROMA_DC
                    as ::core::ffi::c_int as usize] = (*h).quantf.coeff_level_run8;
            }
            3 => {
                (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_422;
                (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_luma_mbaff;
                (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_luma_intra_mbaff;
            }
            _ => {}
        };
    }
}

unsafe extern "C" fn set_aspect_ratio(
    mut h: *mut crate::src::common::common::x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
    mut initial: ::core::ffi::c_int,
) {
    unsafe {
        if (*param).vui.i_sar_width > 0 as ::core::ffi::c_int
            && (*param).vui.i_sar_height > 0 as ::core::ffi::c_int
        {
            let mut i_w: crate::stdlib::uint32_t =
                (*param).vui.i_sar_width as crate::stdlib::uint32_t;
            let mut i_h: crate::stdlib::uint32_t =
                (*param).vui.i_sar_height as crate::stdlib::uint32_t;
            let mut old_w: crate::stdlib::uint32_t =
                (*h).param.vui.i_sar_width as crate::stdlib::uint32_t;
            let mut old_h: crate::stdlib::uint32_t =
                (*h).param.vui.i_sar_height as crate::stdlib::uint32_t;
            crate::src::common::base::x264_reduce_fraction(&raw mut i_w, &raw mut i_h);
            while i_w > 65535 as crate::stdlib::uint32_t || i_h > 65535 as crate::stdlib::uint32_t {
                i_w = i_w.wrapping_div(2 as crate::stdlib::uint32_t);
                i_h = i_h.wrapping_div(2 as crate::stdlib::uint32_t);
            }
            crate::src::common::base::x264_reduce_fraction(&raw mut i_w, &raw mut i_h);
            if i_w != old_w || i_h != old_h || initial != 0 {
                (*h).param.vui.i_sar_width = 0 as ::core::ffi::c_int;
                (*h).param.vui.i_sar_height = 0 as ::core::ffi::c_int;
                if i_w == 0 as crate::stdlib::uint32_t || i_h == 0 as crate::stdlib::uint32_t {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_WARNING_1,
                        b"cannot create valid sample aspect ratio\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        if initial != 0 {
                            crate::x264_h::X264_LOG_INFO
                        } else {
                            crate::x264_h::X264_LOG_DEBUG_1
                        },
                        b"using SAR=%d/%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        i_w,
                        i_h,
                    );
                    (*h).param.vui.i_sar_width = i_w as ::core::ffi::c_int;
                    (*h).param.vui.i_sar_height = i_h as ::core::ffi::c_int;
                }
            }
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_open(
    mut param: *mut crate::x264_h::x264_param_t,
    mut api: *mut ::core::ffi::c_void,
) -> *mut crate::src::common::common::x264_t {
    unsafe {
        let mut temp: ::core::ffi::c_int = 0;
        let mut profile: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut level: [::core::ffi::c_char; 16] = [0; 16];
        static mut subsampling: [*const ::core::ffi::c_char; 4] = [
            b"4:0:0\0".as_ptr() as *const ::core::ffi::c_char,
            b"4:2:0\0".as_ptr() as *const ::core::ffi::c_char,
            b"4:2:2\0".as_ptr() as *const ::core::ffi::c_char,
            b"4:4:4\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut c2rust_current_block: u64;
        let mut h: *mut crate::src::common::common::x264_t =
            ::core::ptr::null_mut::<crate::src::common::common::x264_t>();
        let mut buf: [::core::ffi::c_char; 1000] = [0; 1000];
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i_slicetype_length: ::core::ffi::c_int = 0;
        h = crate::src::common::base::x264_malloc(::core::mem::size_of::<
            crate::src::common::common::x264_t,
        >() as crate::stdlib::int64_t) as *mut crate::src::common::common::x264_t;
        if !h.is_null() {
            crate::stdlib::memset(
                h as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<crate::src::common::common::x264_t>()
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                &raw mut (*h).param as *mut ::core::ffi::c_void,
                param as *const ::core::ffi::c_void,
                ::core::mem::size_of::<crate::x264_h::x264_param_t>()
                    as crate::__stddef_size_t_h::size_t,
            );
            (*h).param.opaque = crate::__stddef_null_h::NULL;
            (*h).param.param_free = None;
            if !(*h).param.psz_cqm_file.is_null() {
                (*h).param.psz_cqm_file = crate::src::common::base::x264_param_strdup(
                    &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
                    (*h).param.psz_cqm_file,
                );
                if (*h).param.psz_cqm_file.is_null() {
                    c2rust_current_block = 4713171888074558396;
                } else {
                    c2rust_current_block = 5399440093318478209;
                }
            } else {
                c2rust_current_block = 5399440093318478209;
            }
            match c2rust_current_block {
                4713171888074558396 => {}
                _ => {
                    if !(*h).param.psz_dump_yuv.is_null() {
                        (*h).param.psz_dump_yuv = crate::src::common::base::x264_param_strdup(
                            &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
                            (*h).param.psz_dump_yuv,
                        );
                        if (*h).param.psz_dump_yuv.is_null() {
                            c2rust_current_block = 4713171888074558396;
                        } else {
                            c2rust_current_block = 8831408221741692167;
                        }
                    } else {
                        c2rust_current_block = 8831408221741692167;
                    }
                    match c2rust_current_block {
                        4713171888074558396 => {}
                        _ => {
                            if !(*h).param.rc.psz_stat_out.is_null() {
                                (*h).param.rc.psz_stat_out =
                                    crate::src::common::base::x264_param_strdup(
                                        &raw mut (*h).param as *mut _
                                            as *mut crate::x264_h::x264_param_t,
                                        (*h).param.rc.psz_stat_out,
                                    );
                                if (*h).param.rc.psz_stat_out.is_null() {
                                    c2rust_current_block = 4713171888074558396;
                                } else {
                                    c2rust_current_block = 4808432441040389987;
                                }
                            } else {
                                c2rust_current_block = 4808432441040389987;
                            }
                            match c2rust_current_block {
                                4713171888074558396 => {}
                                _ => {
                                    if !(*h).param.rc.psz_stat_in.is_null() {
                                        (*h).param.rc.psz_stat_in =
                                            crate::src::common::base::x264_param_strdup(
                                                &raw mut (*h).param as *mut _
                                                    as *mut crate::x264_h::x264_param_t,
                                                (*h).param.rc.psz_stat_in,
                                            );
                                        if (*h).param.rc.psz_stat_in.is_null() {
                                            c2rust_current_block = 4713171888074558396;
                                        } else {
                                            c2rust_current_block = 10043043949733653460;
                                        }
                                    } else {
                                        c2rust_current_block = 10043043949733653460;
                                    }
                                    match c2rust_current_block {
                                        4713171888074558396 => {}
                                        _ => {
                                            if !(*h).param.rc.psz_zones.is_null() {
                                                (*h).param.rc.psz_zones =
                                                    crate::src::common::base::x264_param_strdup(
                                                        &raw mut (*h).param as *mut _
                                                            as *mut crate::x264_h::x264_param_t,
                                                        (*h).param.rc.psz_zones,
                                                    );
                                                if (*h).param.rc.psz_zones.is_null() {
                                                    c2rust_current_block = 4713171888074558396;
                                                } else {
                                                    c2rust_current_block = 14648156034262866959;
                                                }
                                            } else {
                                                c2rust_current_block = 14648156034262866959;
                                            }
                                            match c2rust_current_block {
                                                4713171888074558396 => {}
                                                _ => {
                                                    if !(*h).param.psz_clbin_file.is_null() {
                                                        (*h).param.psz_clbin_file =
                                                            crate::src::common::base::x264_param_strdup(

                                                                &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
                                                                (*h).param.psz_clbin_file,
                                                            );
                                                        if (*h).param.psz_clbin_file.is_null() {
                                                            c2rust_current_block =
                                                                4713171888074558396;
                                                        } else {
                                                            c2rust_current_block =
                                                                652864300344834934;
                                                        }
                                                    } else {
                                                        c2rust_current_block = 652864300344834934;
                                                    }
                                                    match c2rust_current_block {
                                                        4713171888074558396 => {}
                                                        _ => {
                                                            if (*param).param_free.is_some() {
                                                                crate::src::common::base::x264_param_cleanup(param as *mut crate::x264_h::x264_param_t);
                                                                (*param).param_free.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    param
                                                                        as *mut ::core::ffi::c_void,
                                                                );
                                                            }
                                                            (*h).api = api;
                                                            if !(validate_parameters(
                                                                h,
                                                                1 as ::core::ffi::c_int,
                                                            ) < 0 as ::core::ffi::c_int)
                                                            {
                                                                if !(*h)
                                                                    .param
                                                                    .psz_cqm_file
                                                                    .is_null()
                                                                {
                                                                    if crate::src::common::set::x264_8_cqm_parse_file(

                                                                        h as *mut crate::src::common::common::x264_t,
                                                                        (*h).param.psz_cqm_file,
                                                                    ) < 0 as ::core::ffi::c_int
                                                                    {
                                                                        c2rust_current_block =
                                                                            4713171888074558396;
                                                                    } else {
                                                                        c2rust_current_block =
                                                                            3934796541983872331;
                                                                    }
                                                                } else {
                                                                    c2rust_current_block =
                                                                        3934796541983872331;
                                                                }
                                                                match c2rust_current_block {
                                                                    4713171888074558396 => {}
                                                                    _ => {
                                                                        crate::src::common::base::x264_reduce_fraction(
                                                                            &raw mut (*h)
                                                                                .param
                                                                                .i_fps_num,
                                                                            &raw mut (*h)
                                                                                .param
                                                                                .i_fps_den,
                                                                        );
                                                                        crate::src::common::base::x264_reduce_fraction(
                                                                            &raw mut (*h)
                                                                                .param
                                                                                .i_timebase_num,
                                                                            &raw mut (*h)
                                                                                .param
                                                                                .i_timebase_den,
                                                                        );
                                                                        (*h).i_frame = -(1
                                                                            as ::core::ffi::c_int);
                                                                        (*h).i_frame_num =
                                                                            0 as ::core::ffi::c_int;
                                                                        if (*h)
                                                                            .param
                                                                            .i_avcintra_class
                                                                            != 0
                                                                        {
                                                                            (*h).i_idr_pic_id = if (*h).param.i_avcintra_class
                                                                                > 200 as ::core::ffi::c_int
                                                                            {
                                                                                4 as ::core::ffi::c_int
                                                                            } else {
                                                                                5 as ::core::ffi::c_int
                                                                            };
                                                                        } else {
                                                                            (*h).i_idr_pic_id = 0 as ::core::ffi::c_int;
                                                                        }
                                                                        if ((*h)
                                                                            .param
                                                                            .i_timebase_den
                                                                            as crate::stdlib::uint64_t)
                                                                            .wrapping_mul(
                                                                                2 as crate::stdlib::uint64_t,
                                                                            )
                                                                            > crate::stdlib::UINT32_MAX as crate::stdlib::uint64_t
                                                                        {
                                                                            crate::src::common::common::x264_8_log(

                                                                                h as *mut crate::src::common::common::x264_t,
                                                                                crate::x264_h::X264_LOG_ERROR_1,
                                                                                b"Effective timebase denominator %u exceeds H.264 maximum\n\0"
                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                (*h).param.i_timebase_den,
                                                                            );
                                                                        } else {
                                                                            set_aspect_ratio(
                                                                                h,
                                                                                &raw mut (*h).param,
                                                                                1 as ::core::ffi::c_int,
                                                                            );
                                                                            crate::src::encoder::set::x264_8_sps_init(

                                                                                &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t as *mut crate::src::common::set::x264_sps_t,
                                                                                (*h).param.i_sps_id,

                                                                                &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
                                                                            );
                                                                            crate::src::encoder::set::x264_8_sps_init_scaling_list(

                                                                                &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t as *mut crate::src::common::set::x264_sps_t,

                                                                                &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
                                                                            );
                                                                            crate::src::encoder::set::x264_8_pps_init(

                                                                                &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t as *mut crate::src::common::set::x264_pps_t,
                                                                                (*h).param.i_sps_id,

                                                                                &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,

                                                                                &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t as *mut crate::src::common::set::x264_sps_t,
                                                                            );
                                                                            crate::src::encoder::set::x264_8_validate_levels(h as *mut crate::src::common::common::x264_t, 1 as ::core::ffi::c_int);
                                                                            (*h).chroma_qp_table = (&raw const i_chroma_qp_table
                                                                                as *const crate::stdlib::uint8_t)
                                                                                .offset(12 as ::core::ffi::c_int as isize)
                                                                                .offset(
                                                                                    (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                                                                                        .i_chroma_qp_index_offset as isize,
                                                                                );
                                                                            if !(crate::src::common::set::x264_8_cqm_init(h as *mut crate::src::common::common::x264_t) < 0 as ::core::ffi::c_int) {
                                                                                (*h).mb.i_mb_width = (*(&raw mut (*h).sps
                                                                                    as *mut crate::src::common::set::x264_sps_t))
                                                                                    .i_mb_width;
                                                                                (*h).mb.i_mb_height = (*(&raw mut (*h).sps
                                                                                    as *mut crate::src::common::set::x264_sps_t))
                                                                                    .i_mb_height;
                                                                                (*h).mb.i_mb_count = (*h).mb.i_mb_width
                                                                                    * (*h).mb.i_mb_height;
                                                                                (*h).mb.chroma_h_shift = (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                                                                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                                                                    || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                                                                        == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int) as ::core::ffi::c_int;
                                                                                (*h).mb.chroma_v_shift = (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                                                                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int) as ::core::ffi::c_int;
                                                                                (*h).mb.b_adaptive_mbaff = ((*h).param.b_interlaced != 0
                                                                                    && (*h).param.analyse.i_subpel_refine != 0)
                                                                                    as ::core::ffi::c_int;
                                                                                if (*h).param.i_bframe_adaptive == crate::x264_h::X264_B_ADAPT_TRELLIS
                                                                                    && (*h).param.rc.b_stat_read == 0
                                                                                {
                                                                                    (*h).frames.i_delay = (if (*h).param.i_bframe
                                                                                        > 3 as ::core::ffi::c_int
                                                                                    {
                                                                                        (*h).param.i_bframe
                                                                                    } else {
                                                                                        3 as ::core::ffi::c_int
                                                                                    }) * 4 as ::core::ffi::c_int;
                                                                                } else {
                                                                                    (*h).frames.i_delay = (*h).param.i_bframe;
                                                                                }
                                                                                if (*h).param.rc.b_mb_tree != 0
                                                                                    || (*h).param.rc.i_vbv_buffer_size != 0
                                                                                {
                                                                                    (*h).frames.i_delay = if (*h).frames.i_delay
                                                                                        > (*h).param.rc.i_lookahead
                                                                                    {
                                                                                        (*h).frames.i_delay
                                                                                    } else {
                                                                                        (*h).param.rc.i_lookahead
                                                                                    };
                                                                                }
                                                                                i_slicetype_length = (*h).frames.i_delay;
                                                                                (*h).frames.i_delay
                                                                                    += (*h).i_thread_frames - 1 as ::core::ffi::c_int;
                                                                                (*h).frames.i_delay += (*h).param.i_sync_lookahead;
                                                                                (*h).frames.i_delay += (*h).param.b_vfr_input;
                                                                                (*h).frames.i_bframe_delay = if (*h).param.i_bframe != 0 {
                                                                                    if (*h).param.i_bframe_pyramid != 0 {
                                                                                        2 as ::core::ffi::c_int
                                                                                    } else {
                                                                                        1 as ::core::ffi::c_int
                                                                                    }
                                                                                } else {
                                                                                    0 as ::core::ffi::c_int
                                                                                };
                                                                                (*h).frames.i_max_ref0 = (*h).param.i_frame_reference;
                                                                                (*h).frames.i_max_ref1 = if (*(&raw mut (*h).sps
                                                                                    as *mut crate::src::common::set::x264_sps_t))
                                                                                    .vui
                                                                                    .i_num_reorder_frames < (*h).param.i_frame_reference
                                                                                {
                                                                                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                        .vui
                                                                                        .i_num_reorder_frames
                                                                                } else {
                                                                                    (*h).param.i_frame_reference
                                                                                };
                                                                                (*h).frames.i_max_dpb = (*(&raw mut (*h).sps
                                                                                    as *mut crate::src::common::set::x264_sps_t))
                                                                                    .vui
                                                                                    .i_max_dec_frame_buffering;
                                                                                (*h).frames.b_have_lowres = ((*h).param.rc.b_stat_read == 0
                                                                                    && ((*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR
                                                                                        || (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_CRF
                                                                                        || (*h).param.i_bframe_adaptive != 0
                                                                                        || (*h).param.i_scenecut_threshold != 0
                                                                                        || (*h).param.rc.b_mb_tree != 0
                                                                                        || (*h).param.analyse.i_weighted_pred != 0))
                                                                                    as ::core::ffi::c_int;
                                                                                (*h).frames.b_have_lowres
                                                                                    |= ((*h).param.rc.b_stat_read != 0
                                                                                        && (*h).param.rc.i_vbv_buffer_size
                                                                                            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                                                                                (*h).frames.b_have_sub8x8_esa = ((*h).param.analyse.inter
                                                                                    & crate::x264_h::X264_ANALYSE_PSUB8x8 != 0) as ::core::ffi::c_int;
                                                                                (*h).frames.i_last_keyframe = -(*h).param.i_keyint_max;
                                                                                (*h).frames.i_last_idr = (*h).frames.i_last_keyframe;
                                                                                (*h).frames.i_input = 0 as ::core::ffi::c_int;
                                                                                (*h).frames.i_second_largest_pts = -(1
                                                                                    as ::core::ffi::c_int) as crate::stdlib::int64_t;
                                                                                (*h).frames.i_largest_pts = (*h)
                                                                                    .frames
                                                                                    .i_second_largest_pts;
                                                                                (*h).frames.i_poc_last_open_gop = -(1
                                                                                    as ::core::ffi::c_int);
                                                                                (*h).cost_table = crate::src::common::base::x264_malloc(
                                                                                    ::core::mem::size_of::<crate::src::common::common::C2Rust_Unnamed_16>() as crate::stdlib::int64_t,
                                                                                ) as *mut crate::src::common::common::C2Rust_Unnamed_16;
                                                                                if !(*h).cost_table.is_null() {
                                                                                    crate::stdlib::memset(
                                                                                        (*h).cost_table as *mut ::core::ffi::c_void,
                                                                                        0 as ::core::ffi::c_int,
                                                                                        ::core::mem::size_of::<crate::src::common::common::C2Rust_Unnamed_16>() as crate::__stddef_size_t_h::size_t,
                                                                                    );
                                                                                    (*h).frames.unused[0 as ::core::ffi::c_int as usize] = crate::src::common::base::x264_malloc(
                                                                                        (((*h).frames.i_delay + 3 as ::core::ffi::c_int) as usize)
                                                                                            .wrapping_mul(
                                                                                                ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>() as usize,
                                                                                            ) as crate::stdlib::int64_t,
                                                                                    ) as *mut *mut crate::src::common::frame::x264_frame_t;
                                                                                    if !(*h)
                                                                                        .frames
                                                                                        .unused[0 as ::core::ffi::c_int as usize]
                                                                                        .is_null()
                                                                                    {
                                                                                        crate::stdlib::memset(
                                                                                            (*h).frames.unused[0 as ::core::ffi::c_int as usize]
                                                                                                as *mut ::core::ffi::c_void,
                                                                                            0 as ::core::ffi::c_int,
                                                                                            (((*h).frames.i_delay + 3 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t)
                                                                                                .wrapping_mul(
                                                                                                    ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>() as crate::__stddef_size_t_h::size_t,
                                                                                                ),
                                                                                        );
                                                                                        (*h).frames.unused[1 as ::core::ffi::c_int as usize] = crate::src::common::base::x264_malloc(
                                                                                            (((*h).i_thread_frames + 16 as ::core::ffi::c_int
                                                                                                + 4 as ::core::ffi::c_int) as usize)
                                                                                                .wrapping_mul(
                                                                                                    ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>() as usize,
                                                                                                ) as crate::stdlib::int64_t,
                                                                                        ) as *mut *mut crate::src::common::frame::x264_frame_t;
                                                                                        if !(*h)
                                                                                            .frames
                                                                                            .unused[1 as ::core::ffi::c_int as usize]
                                                                                            .is_null()
                                                                                        {
                                                                                            crate::stdlib::memset(
                                                                                                (*h).frames.unused[1 as ::core::ffi::c_int as usize]
                                                                                                    as *mut ::core::ffi::c_void,
                                                                                                0 as ::core::ffi::c_int,
                                                                                                (((*h).i_thread_frames + 16 as ::core::ffi::c_int
                                                                                                    + 4 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t)
                                                                                                    .wrapping_mul(
                                                                                                        ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>() as crate::__stddef_size_t_h::size_t,
                                                                                                    ),
                                                                                            );
                                                                                            (*h).frames.current = crate::src::common::base::x264_malloc(
                                                                                                (((*h).param.i_sync_lookahead + (*h).param.i_bframe
                                                                                                    + (*h).i_thread_frames + 3 as ::core::ffi::c_int) as usize)
                                                                                                    .wrapping_mul(
                                                                                                        ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>() as usize,
                                                                                                    ) as crate::stdlib::int64_t,
                                                                                            ) as *mut *mut crate::src::common::frame::x264_frame_t;
                                                                                            if !(*h).frames.current.is_null() {
                                                                                                crate::stdlib::memset(
                                                                                                    (*h).frames.current as *mut ::core::ffi::c_void,
                                                                                                    0 as ::core::ffi::c_int,
                                                                                                    (((*h).param.i_sync_lookahead + (*h).param.i_bframe
                                                                                                        + (*h).i_thread_frames + 3 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t)
                                                                                                        .wrapping_mul(
                                                                                                            ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>() as crate::__stddef_size_t_h::size_t,
                                                                                                        ),
                                                                                                );
                                                                                                if (*h).param.analyse.i_weighted_pred
                                                                                                    > 0 as ::core::ffi::c_int
                                                                                                {
                                                                                                    (*h).frames.blank_unused = crate::src::common::base::x264_malloc(
                                                                                                        (((*h).i_thread_frames * 4 as ::core::ffi::c_int) as usize)
                                                                                                            .wrapping_mul(
                                                                                                                ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>() as usize,
                                                                                                            ) as crate::stdlib::int64_t,
                                                                                                    ) as *mut *mut crate::src::common::frame::x264_frame_t;
                                                                                                    if (*h).frames.blank_unused.is_null() {
                                                                                                        c2rust_current_block = 4713171888074558396;
                                                                                                    } else {
                                                                                                        crate::stdlib::memset(
                                                                                                            (*h).frames.blank_unused as *mut ::core::ffi::c_void,
                                                                                                            0 as ::core::ffi::c_int,
                                                                                                            (((*h).i_thread_frames * 4 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t)
                                                                                                                .wrapping_mul(
                                                                                                                    ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>() as crate::__stddef_size_t_h::size_t,
                                                                                                                ),
                                                                                                        );
                                                                                                        c2rust_current_block = 6406431739208918833;
                                                                                                    }
                                                                                                } else {
                                                                                                    c2rust_current_block = 6406431739208918833;
                                                                                                }
                                                                                                match c2rust_current_block {
                                                                                                    4713171888074558396 => {}
                                                                                                    _ => {
                                                                                                        (*h).i_ref[1 as ::core::ffi::c_int as usize] = 0
                                                                                                            as ::core::ffi::c_int;
                                                                                                        (*h).i_ref[0 as ::core::ffi::c_int as usize] = (*h)
                                                                                                            .i_ref[1 as ::core::ffi::c_int as usize];
                                                                                                        (*h).i_disp_fields = 0 as crate::stdlib::int64_t;
                                                                                                        (*h).i_coded_fields = (*h).i_disp_fields;
                                                                                                        (*h).i_cpb_delay = (*h).i_coded_fields;
                                                                                                        (*h).i_prev_duration = ((*h).param.i_fps_den as crate::stdlib::uint64_t)
                                                                                                            .wrapping_mul(
                                                                                                                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).vui.i_time_scale
                                                                                                                    as crate::stdlib::uint64_t,
                                                                                                            )
                                                                                                            .wrapping_div(
                                                                                                                ((*h).param.i_fps_num as crate::stdlib::uint64_t)
                                                                                                                    .wrapping_mul(
                                                                                                                        (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                            .vui
                                                                                                                            .i_num_units_in_tick as crate::stdlib::uint64_t,
                                                                                                                    ),
                                                                                                            ) as crate::stdlib::int64_t;
                                                                                                        (*h).i_disp_fields_last_frame = -(1 as ::core::ffi::c_int);
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
                                                                                                        crate::src::common::pixel::x264_8_pixel_init((*h).param.cpu,  &raw mut (*h).pixf as *mut _ as
    *mut crate::src::common::pixel::x264_pixel_function_t);
                                                                                                        crate::src::common::dct::x264_8_dct_init((*h).param.cpu,  &raw mut (*h).dctf as *mut _ as
    *mut crate::src::common::dct::x264_dct_function_t);
                                                                                                        crate::src::common::dct::x264_8_zigzag_init(
                                                                                                            (*h).param.cpu,

                                                                                                            &raw mut (*h).zigzagf_progressive as *mut _ as
    *mut crate::src::common::dct::x264_zigzag_function_t,

                                                                                                            &raw mut (*h).zigzagf_interlaced as *mut _ as
    *mut crate::src::common::dct::x264_zigzag_function_t,
                                                                                                        );
                                                                                                        crate::stdlib::memcpy(
                                                                                                            &raw mut (*h).zigzagf as *mut ::core::ffi::c_void,
                                                                                                            (if (*h).param.b_interlaced != 0 {
                                                                                                                &raw mut (*h).zigzagf_interlaced
                                                                                                            } else {
                                                                                                                &raw mut (*h).zigzagf_progressive
                                                                                                            }) as *const ::core::ffi::c_void,
                                                                                                            ::core::mem::size_of::<crate::src::common::dct::x264_zigzag_function_t>() as crate::__stddef_size_t_h::size_t,
                                                                                                        );
                                                                                                        crate::src::common::mc::x264_8_mc_init(
                                                                                                            (*h).param.cpu,

                                                                                                            &raw mut (*h).mc as *mut _ as
    *mut crate::src::common::mc::x264_mc_functions_t_6,
                                                                                                            (*h).param.b_cpu_independent,
                                                                                                        );
                                                                                                        crate::src::common::quant::x264_8_quant_init(h as *mut crate::src::common::common::x264_t, (*h).param.cpu,  &raw mut (*h).quantf as *mut _ as
    *mut crate::src::common::quant::x264_quant_function_t);
                                                                                                        crate::src::common::deblock::x264_8_deblock_init(
                                                                                                            (*h).param.cpu,

                                                                                                            &raw mut (*h).loopf as *mut _ as
    *mut crate::src::common::frame::x264_deblock_function_t,
                                                                                                            (*h).param.b_interlaced,
                                                                                                        );
                                                                                                        crate::src::common::bitstream::x264_8_bitstream_init((*h).param.cpu,  &raw mut (*h).bsf as *mut _ as
    *mut crate::src::common::bitstream::x264_bitstream_function_t);
                                                                                                        if (*h).param.b_cabac != 0 {
                                                                                                            crate::src::common::cabac::x264_8_cabac_init(h as *mut crate::src::common::common::x264_t);
                                                                                                        } else {
                                                                                                            crate::src::common::vlc::x264_8_cavlc_init(h as *mut crate::src::common::common::x264_t);
                                                                                                        }
                                                                                                        mbcmp_init(h);
                                                                                                        chroma_dsp_init(h);
                                                                                                        p = (&raw mut buf as *mut ::core::ffi::c_char)
                                                                                                            .offset(
                                                                                                                crate::stdlib::sprintf(
                                                                                                                    &raw mut buf as *mut ::core::ffi::c_char,
                                                                                                                    b"using cpu capabilities:\0".as_ptr()
                                                                                                                        as *const ::core::ffi::c_char,
                                                                                                                ) as isize,
                                                                                                            );
                                                                                                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                        while (*(&raw const crate::src::common::cpu::x264_cpu_names
                                                                                                            as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                            .offset(i as isize))
                                                                                                            .flags != 0
                                                                                                        {
                                                                                                            if !(crate::stdlib::strcmp(
                                                                                                                (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                    .offset(i as isize))
                                                                                                                    .name,
                                                                                                                b"SSE\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                            ) == 0
                                                                                                                && (*h).param.cpu
                                                                                                                    & (1 as crate::stdlib::uint32_t) << 3 as ::core::ffi::c_int != 0)
                                                                                                            {
                                                                                                                if !(crate::stdlib::strcmp(
                                                                                                                    (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                        .offset(i as isize))
                                                                                                                        .name,
                                                                                                                    b"SSE2\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                ) == 0
                                                                                                                    && (*h).param.cpu
                                                                                                                        & (crate::x264_h::X264_CPU_SSE2_IS_FAST as crate::stdlib::uint32_t
                                                                                                                            | crate::x264_h::X264_CPU_SSE2_IS_SLOW as crate::stdlib::uint32_t) != 0)
                                                                                                                {
                                                                                                                    if !(crate::stdlib::strcmp(
                                                                                                                        (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                            .offset(i as isize))
                                                                                                                            .name,
                                                                                                                        b"SSE3\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                    ) == 0
                                                                                                                        && ((*h).param.cpu & crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t != 0
                                                                                                                            || (*h).param.cpu & crate::x264_h::X264_CPU_CACHELINE_64 as crate::stdlib::uint32_t == 0))
                                                                                                                    {
                                                                                                                        if !(crate::stdlib::strcmp(
                                                                                                                            (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                .offset(i as isize))
                                                                                                                                .name,
                                                                                                                            b"SSE4.1\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                        ) == 0 && (*h).param.cpu & crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t != 0)
                                                                                                                        {
                                                                                                                            if !(crate::stdlib::strcmp(
                                                                                                                                (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                    .offset(i as isize))
                                                                                                                                    .name,
                                                                                                                                b"LZCNT\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                            ) == 0 && (*h).param.cpu & crate::x264_h::X264_CPU_BMI1 as crate::stdlib::uint32_t != 0)
                                                                                                                            {
                                                                                                                                if !(crate::stdlib::strcmp(
                                                                                                                                    (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                        .offset(i as isize))
                                                                                                                                        .name,
                                                                                                                                    b"BMI1\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                                ) == 0 && (*h).param.cpu & crate::x264_h::X264_CPU_BMI2 as crate::stdlib::uint32_t != 0)
                                                                                                                                {
                                                                                                                                    if !(crate::stdlib::strcmp(
                                                                                                                                        (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                            .offset(i as isize))
                                                                                                                                            .name,
                                                                                                                                        b"FMA4\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                                    ) == 0 && (*h).param.cpu & crate::x264_h::X264_CPU_FMA3 as crate::stdlib::uint32_t != 0)
                                                                                                                                    {
                                                                                                                                        if (*h).param.cpu
                                                                                                                                            & (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                                .offset(i as isize))
                                                                                                                                                .flags
                                                                                                                                            == (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                                .offset(i as isize))
                                                                                                                                                .flags
                                                                                                                                            && (i == 0
                                                                                                                                                || (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                                    .offset(i as isize))
                                                                                                                                                    .flags
                                                                                                                                                    != (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                                        .offset((i - 1 as ::core::ffi::c_int) as isize))
                                                                                                                                                        .flags)
                                                                                                                                        {
                                                                                                                                            p = p
                                                                                                                                                .offset(
                                                                                                                                                    crate::stdlib::sprintf(
                                                                                                                                                        p,
                                                                                                                                                        b" %s\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                                                        (*(&raw const crate::src::common::cpu::x264_cpu_names as *const crate::src::common::cpu::x264_cpu_name_t)
                                                                                                                                                            .offset(i as isize))
                                                                                                                                                            .name,
                                                                                                                                                    ) as isize,
                                                                                                                                                );
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            i += 1;
                                                                                                        }
                                                                                                        if (*h).param.cpu == 0 {
                                                                                                            p = p
                                                                                                                .offset(
                                                                                                                    crate::stdlib::sprintf(
                                                                                                                        p,
                                                                                                                        b" none!\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                    ) as isize,
                                                                                                                );
                                                                                                        }
                                                                                                        crate::src::common::common::x264_8_log(

                                                                                                            h as *mut crate::src::common::common::x264_t,
                                                                                                            crate::x264_h::X264_LOG_INFO,
                                                                                                            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                            &raw mut buf as *mut ::core::ffi::c_char,
                                                                                                        );
                                                                                                        if !(crate::src::encoder::analyse::x264_8_analyse_init_costs(h as *mut crate::src::common::common::x264_t) != 0) {
                                                                                                            temp = 392 as ::core::ffi::c_int;
                                                                                                            if (temp as ::core::ffi::c_uint).leading_zeros() as i32
                                                                                                                != 23 as ::core::ffi::c_int
                                                                                                            {
                                                                                                                crate::src::common::common::x264_8_log(

                                                                                                                    h as *mut crate::src::common::common::x264_t,
                                                                                                                    crate::x264_h::X264_LOG_ERROR_1,
                                                                                                                    b"CLZ test failed: x264 has been miscompiled!\n\0".as_ptr()
                                                                                                                        as *const ::core::ffi::c_char,
                                                                                                                );
                                                                                                                crate::src::common::common::x264_8_log(

                                                                                                                    h as *mut crate::src::common::common::x264_t,
                                                                                                                    crate::x264_h::X264_LOG_ERROR_1,
                                                                                                                    b"Are you attempting to run an SSE4a/LZCNT-targeted build on a CPU that\n\0"
                                                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                                                );
                                                                                                                crate::src::common::common::x264_8_log(

                                                                                                                    h as *mut crate::src::common::common::x264_t,
                                                                                                                    crate::x264_h::X264_LOG_ERROR_1,
                                                                                                                    b"doesn't support it?\n\0".as_ptr()
                                                                                                                        as *const ::core::ffi::c_char,
                                                                                                                );
                                                                                                            } else {
                                                                                                                (*h).out.i_nal = 0 as ::core::ffi::c_int;
                                                                                                                (*h).out.i_bitstream = x264_clip3f(
                                                                                                                    ((*h).param.i_width * (*h).param.i_height
                                                                                                                        * 4 as ::core::ffi::c_int) as ::core::ffi::c_double
                                                                                                                        * (if (*h).param.rc.i_rc_method == crate::x264_h::X264_RC_ABR {
                                                                                                                            crate::stdlib::pow(
                                                                                                                                0.95f64,
                                                                                                                                (*h).param.rc.i_qp_min as ::core::ffi::c_double,
                                                                                                                            )
                                                                                                                        } else {
                                                                                                                            crate::stdlib::pow(
                                                                                                                                0.95f64,
                                                                                                                                (*h).param.rc.i_qp_constant as ::core::ffi::c_double,
                                                                                                                            )
                                                                                                                                * (if 1 as ::core::ffi::c_int as ::core::ffi::c_float
                                                                                                                                    > (*h).param.rc.f_ip_factor
                                                                                                                                {
                                                                                                                                    1 as ::core::ffi::c_int as ::core::ffi::c_float
                                                                                                                                } else {
                                                                                                                                    (*h).param.rc.f_ip_factor
                                                                                                                                }) as ::core::ffi::c_double
                                                                                                                        }),
                                                                                                                    1000000 as ::core::ffi::c_int as ::core::ffi::c_double,
                                                                                                                    (crate::limits_h::INT_MAX / 3 as ::core::ffi::c_int) as ::core::ffi::c_double,
                                                                                                                ) as ::core::ffi::c_int;
                                                                                                                (*h).nal_buffer_size = (*h).out.i_bitstream
                                                                                                                    * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                                                                                                    + 4 as ::core::ffi::c_int + 64 as ::core::ffi::c_int;
                                                                                                                (*h).nal_buffer = crate::src::common::base::x264_malloc(
                                                                                                                    (*h).nal_buffer_size as crate::stdlib::int64_t,
                                                                                                                ) as *mut crate::stdlib::uint8_t;
                                                                                                                if !(*h).nal_buffer.is_null() {
                                                                                                                    (*h).reconfig_h = crate::src::common::base::x264_malloc(
                                                                                                                        ::core::mem::size_of::<crate::src::common::common::x264_t>() as crate::stdlib::int64_t,
                                                                                                                    ) as *mut crate::src::common::common::x264_t;
                                                                                                                    if !(*h).reconfig_h.is_null() {
                                                                                                                        if !((*h).param.i_threads > 1 as ::core::ffi::c_int
                                                                                                                            && crate::src::common::threadpool::x264_8_threadpool_init(
                                                                                                                                &raw mut (*h).threadpool,
                                                                                                                                (*h).param.i_threads,
                                                                                                                            ) != 0)
                                                                                                                        {
                                                                                                                            if !((*h).param.i_lookahead_threads
                                                                                                                                > 1 as ::core::ffi::c_int
                                                                                                                                && crate::src::common::threadpool::x264_8_threadpool_init(
                                                                                                                                    &raw mut (*h).lookaheadpool,
                                                                                                                                    (*h).param.i_lookahead_threads,
                                                                                                                                ) != 0)
                                                                                                                            {
                                                                                                                                (*h).thread[0 as ::core::ffi::c_int as usize] = h;
                                                                                                                                let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                                                                                                                                loop {
                                                                                                                                    if !(i_0
                                                                                                                                        < (*h).param.i_threads
                                                                                                                                            + ((*h).param.i_sync_lookahead != 0) as ::core::ffi::c_int)
                                                                                                                                    {
                                                                                                                                        c2rust_current_block = 5710330377809666066;
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    (*h).thread[i_0 as usize] = crate::src::common::base::x264_malloc(
                                                                                                                                        ::core::mem::size_of::<crate::src::common::common::x264_t>() as crate::stdlib::int64_t,
                                                                                                                                    ) as *mut crate::src::common::common::x264_t;
                                                                                                                                    if (*h).thread[i_0 as usize].is_null() {
                                                                                                                                        c2rust_current_block = 4713171888074558396;
                                                                                                                                        break;
                                                                                                                                    }
                                                                                                                                    i_0 += 1;
                                                                                                                                }
                                                                                                                                match c2rust_current_block {
                                                                                                                                    4713171888074558396 => {}
                                                                                                                                    _ => {
                                                                                                                                        if (*h).param.i_lookahead_threads > 1 as ::core::ffi::c_int
                                                                                                                                        {
                                                                                                                                            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                                                            loop {
                                                                                                                                                if !(i_1 < (*h).param.i_lookahead_threads) {
                                                                                                                                                    c2rust_current_block = 17855111796567036151;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                (*h).lookahead_thread[i_1 as usize] = crate::src::common::base::x264_malloc(
                                                                                                                                                    ::core::mem::size_of::<crate::src::common::common::x264_t>() as crate::stdlib::int64_t,
                                                                                                                                                ) as *mut crate::src::common::common::x264_t;
                                                                                                                                                if (*h).lookahead_thread[i_1 as usize].is_null() {
                                                                                                                                                    c2rust_current_block = 4713171888074558396;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                *(*h).lookahead_thread[i_1 as usize] = *h;
                                                                                                                                                i_1 += 1;
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            c2rust_current_block = 17855111796567036151;
                                                                                                                                        }
                                                                                                                                        match c2rust_current_block {
                                                                                                                                            4713171888074558396 => {}
                                                                                                                                            _ => {
                                                                                                                                                *(*h).reconfig_h = *h;
                                                                                                                                                let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                                                                loop {
                                                                                                                                                    if !(i_2 < (*h).param.i_threads) {
                                                                                                                                                        c2rust_current_block = 5482373152242628851;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    let mut init_nal_count: ::core::ffi::c_int = (*h)
                                                                                                                                                        .param
                                                                                                                                                        .i_slice_count + 3 as ::core::ffi::c_int;
                                                                                                                                                    let mut allocate_threadlocal_data: ::core::ffi::c_int = ((*h)
                                                                                                                                                        .param
                                                                                                                                                        .b_sliced_threads == 0 || i_2 == 0) as ::core::ffi::c_int;
                                                                                                                                                    if i_2 > 0 as ::core::ffi::c_int {
                                                                                                                                                        *(*h).thread[i_2 as usize] = *h;
                                                                                                                                                    }
                                                                                                                                                    if crate::stdlib::pthread_mutex_init(
                                                                                                                                                        &raw mut (**(&raw mut (*h).thread as *mut *mut crate::src::common::common::x264_t)
                                                                                                                                                            .offset(i_2 as isize))
                                                                                                                                                            .mutex,
                                                                                                                                                        ::core::ptr::null::<crate::stdlib::pthread_mutexattr_t>(),
                                                                                                                                                    ) != 0
                                                                                                                                                    {
                                                                                                                                                        c2rust_current_block = 4713171888074558396;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    if crate::stdlib::pthread_cond_init(
                                                                                                                                                        &raw mut (**(&raw mut (*h).thread as *mut *mut crate::src::common::common::x264_t)
                                                                                                                                                            .offset(i_2 as isize))
                                                                                                                                                            .cv,
                                                                                                                                                        ::core::ptr::null::<crate::stdlib::pthread_condattr_t>(),
                                                                                                                                                    ) != 0
                                                                                                                                                    {
                                                                                                                                                        c2rust_current_block = 4713171888074558396;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    if allocate_threadlocal_data != 0 {
                                                                                                                                                        (*(*h).thread[i_2 as usize]).fdec =  crate::src::common::frame::x264_8_frame_pop_unused(

                                                                                                                                                            h as *mut crate::src::common::common::x264_t,
                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                        ) as
    *mut crate::src::common::frame::x264_frame;
                                                                                                                                                        if (*(*h).thread[i_2 as usize]).fdec.is_null() {
                                                                                                                                                            c2rust_current_block = 4713171888074558396;
                                                                                                                                                            break;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        (*(*h).thread[i_2 as usize]).fdec = (*(*h)
                                                                                                                                                            .thread[0 as ::core::ffi::c_int as usize])
                                                                                                                                                            .fdec;
                                                                                                                                                    }
                                                                                                                                                    (*(*h).thread[i_2 as usize]).out.p_bitstream = crate::src::common::base::x264_malloc(
                                                                                                                                                        (*h).out.i_bitstream as crate::stdlib::int64_t,
                                                                                                                                                    ) as *mut crate::stdlib::uint8_t;
                                                                                                                                                    if (*(*h).thread[i_2 as usize]).out.p_bitstream.is_null() {
                                                                                                                                                        c2rust_current_block = 4713171888074558396;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    (*(*h).thread[i_2 as usize]).out.nal = crate::src::common::base::x264_malloc(
                                                                                                                                                        (init_nal_count as usize)
                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<crate::x264_h::x264_nal_t>() as usize)
                                                                                                                                                            as crate::stdlib::int64_t,
                                                                                                                                                    ) as *mut crate::x264_h::x264_nal_t;
                                                                                                                                                    if (*(*h).thread[i_2 as usize]).out.nal.is_null() {
                                                                                                                                                        c2rust_current_block = 4713171888074558396;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    (*(*h).thread[i_2 as usize]).out.i_nals_allocated = init_nal_count;
                                                                                                                                                    if allocate_threadlocal_data != 0
                                                                                                                                                        && crate::src::common::macroblock::x264_8_macroblock_cache_allocate(

                                                                                                                                                            (*h).thread[i_2 as usize] as *mut crate::src::common::common::x264_t,
                                                                                                                                                        ) < 0 as ::core::ffi::c_int
                                                                                                                                                    {
                                                                                                                                                        c2rust_current_block = 4713171888074558396;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                    i_2 += 1;
                                                                                                                                                }
                                                                                                                                                match c2rust_current_block {
                                                                                                                                                    4713171888074558396 => {}
                                                                                                                                                    _ => {
                                                                                                                                                        if !(crate::src::encoder::lookahead::x264_8_lookahead_init(h as *mut crate::src::common::common::x264_t, i_slicetype_length) != 0) {
                                                                                                                                                            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                                                                            loop {
                                                                                                                                                                if !(i_3 < (*h).param.i_threads) {
                                                                                                                                                                    c2rust_current_block = 4127803603908737533;
                                                                                                                                                                    break;
                                                                                                                                                                }
                                                                                                                                                                if crate::src::common::macroblock::x264_8_macroblock_thread_allocate(

                                                                                                                                                                    (*h).thread[i_3 as usize] as *mut crate::src::common::common::x264_t,
                                                                                                                                                                    0 as ::core::ffi::c_int,
                                                                                                                                                                ) < 0 as ::core::ffi::c_int
                                                                                                                                                                {
                                                                                                                                                                    c2rust_current_block = 4713171888074558396;
                                                                                                                                                                    break;
                                                                                                                                                                }
                                                                                                                                                                i_3 += 1;
                                                                                                                                                            }
                                                                                                                                                            match c2rust_current_block {
                                                                                                                                                                4713171888074558396 => {}
                                                                                                                                                                _ => {
                                                                                                                                                                    if !(crate::src::encoder::ratecontrol::x264_8_ratecontrol_new(h as *mut crate::src::common::common::x264_t) < 0 as ::core::ffi::c_int) {
                                                                                                                                                                        if (*h).param.i_nal_hrd != 0 {
                                                                                                                                                                            crate::src::common::common::x264_8_log(

                                                                                                                                                                                h as *mut crate::src::common::common::x264_t,
                                                                                                                                                                                crate::x264_h::X264_LOG_DEBUG_1,
                                                                                                                                                                                b"HRD bitrate: %i bits/sec\n\0".as_ptr()
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                    .vui
                                                                                                                                                                                    .hrd
                                                                                                                                                                                    .i_bit_rate_unscaled,
                                                                                                                                                                            );
                                                                                                                                                                            crate::src::common::common::x264_8_log(

                                                                                                                                                                                h as *mut crate::src::common::common::x264_t,
                                                                                                                                                                                crate::x264_h::X264_LOG_DEBUG_1,
                                                                                                                                                                                b"CPB size: %i bits\n\0".as_ptr()
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                    .vui
                                                                                                                                                                                    .hrd
                                                                                                                                                                                    .i_cpb_size_unscaled,
                                                                                                                                                                            );
                                                                                                                                                                        }
                                                                                                                                                                        if !(*h).param.psz_dump_yuv.is_null() {
                                                                                                                                                                            let mut f: *mut crate::stdlib::FILE = crate::stdlib::fopen(
                                                                                                                                                                                (*h).param.psz_dump_yuv,
                                                                                                                                                                                b"w\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                                                                            ) as *mut crate::stdlib::FILE;
                                                                                                                                                                            if f.is_null() {
                                                                                                                                                                                crate::src::common::common::x264_8_log(

                                                                                                                                                                                    h as *mut crate::src::common::common::x264_t,
                                                                                                                                                                                    crate::x264_h::X264_LOG_ERROR_1,
                                                                                                                                                                                    b"dump_yuv: can't write to %s\n\0".as_ptr()
                                                                                                                                                                                        as *const ::core::ffi::c_char,
                                                                                                                                                                                    (*h).param.psz_dump_yuv,
                                                                                                                                                                                );
                                                                                                                                                                                c2rust_current_block = 4713171888074558396;
                                                                                                                                                                            } else if x264_is_regular_file(f) == 0 {
                                                                                                                                                                                crate::src::common::common::x264_8_log(

                                                                                                                                                                                    h as *mut crate::src::common::common::x264_t,
                                                                                                                                                                                    crate::x264_h::X264_LOG_ERROR_1,
                                                                                                                                                                                    b"dump_yuv: incompatible with non-regular file %s\n\0"
                                                                                                                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                                                                                                                    (*h).param.psz_dump_yuv,
                                                                                                                                                                                );
                                                                                                                                                                                crate::stdlib::fclose(f);
                                                                                                                                                                                c2rust_current_block = 4713171888074558396;
                                                                                                                                                                            } else {
                                                                                                                                                                                crate::stdlib::fclose(f);
                                                                                                                                                                                c2rust_current_block = 12387625063048049585;
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            c2rust_current_block = 12387625063048049585;
                                                                                                                                                                        }
                                                                                                                                                                        match c2rust_current_block {
                                                                                                                                                                            4713171888074558396 => {}
                                                                                                                                                                            _ => {
                                                                                                                                                                                profile = if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                    .i_profile_idc == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int
                                                                                                                                                                                {
                                                                                                                                                                                    b"Constrained Baseline\0".as_ptr()
                                                                                                                                                                                        as *const ::core::ffi::c_char
                                                                                                                                                                                } else if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                    .i_profile_idc == crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int
                                                                                                                                                                                {
                                                                                                                                                                                    b"Main\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                                                                                                } else if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                    .i_profile_idc == crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
                                                                                                                                                                                {
                                                                                                                                                                                    b"High\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                                                                                                } else if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                    .i_profile_idc == crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int
                                                                                                                                                                                {
                                                                                                                                                                                    if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                        .b_constraint_set3 != 0
                                                                                                                                                                                    {
                                                                                                                                                                                        b"High 10 Intra\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                                                                                                    } else {
                                                                                                                                                                                        b"High 10\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                                                                                                    }
                                                                                                                                                                                } else if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                    .i_profile_idc == crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int
                                                                                                                                                                                {
                                                                                                                                                                                    if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                        .b_constraint_set3 != 0
                                                                                                                                                                                    {
                                                                                                                                                                                        b"High 4:2:2 Intra\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                                                                                                    } else {
                                                                                                                                                                                        b"High 4:2:2\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                                                                                                    }
                                                                                                                                                                                } else if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                    .b_constraint_set3 != 0
                                                                                                                                                                                {
                                                                                                                                                                                    b"High 4:4:4 Intra\0".as_ptr() as *const ::core::ffi::c_char
                                                                                                                                                                                } else {
                                                                                                                                                                                    b"High 4:4:4 Predictive\0".as_ptr()
                                                                                                                                                                                        as *const ::core::ffi::c_char
                                                                                                                                                                                };
                                                                                                                                                                                level = [0; 16];
                                                                                                                                                                                if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_level_idc
                                                                                                                                                                                    == 9 as ::core::ffi::c_int
                                                                                                                                                                                    || (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_level_idc
                                                                                                                                                                                        == 11 as ::core::ffi::c_int
                                                                                                                                                                                        && (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                                                                                                                                                                            .b_constraint_set3 != 0
                                                                                                                                                                                        && ((*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_profile_idc
                                                                                                                                                                                            == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int
                                                                                                                                                                                            || (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_profile_idc
                                                                                                                                                                                                == crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int)
                                                                                                                                                                                {
                                                                                                                                                                                    crate::stdlib::strcpy(
                                                                                                                                                                                        &raw mut level as *mut ::core::ffi::c_char,
                                                                                                                                                                                        b"1b\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                                                                                    );
                                                                                                                                                                                } else {
                                                                                                                                                                                    crate::stdlib::snprintf(
                                                                                                                                                                                        &raw mut level as *mut ::core::ffi::c_char,
                                                                                                                                                                                        ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                                                                                                                                                                            as crate::__stddef_size_t_h::size_t,
                                                                                                                                                                                        b"%d.%d\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                                                                                                                        (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_level_idc
                                                                                                                                                                                            / 10 as ::core::ffi::c_int,
                                                                                                                                                                                        (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_level_idc
                                                                                                                                                                                            % 10 as ::core::ffi::c_int,
                                                                                                                                                                                    );
                                                                                                                                                                                }
                                                                                                                                                                                crate::src::common::common::x264_8_log(

                                                                                                                                                                                    h as *mut crate::src::common::common::x264_t,
                                                                                                                                                                                    crate::x264_h::X264_LOG_INFO,
                                                                                                                                                                                    b"profile %s, level %s, %s, %d-bit\n\0".as_ptr()
                                                                                                                                                                                        as *const ::core::ffi::c_char,
                                                                                                                                                                                    profile,
                                                                                                                                                                                    &raw mut level as *mut ::core::ffi::c_char,
                                                                                                                                                                                    subsampling[crate::src::common::base::CHROMA_444 as ::core::ffi::c_int as usize],
                                                                                                                                                                                    crate::internal::BIT_DEPTH,
                                                                                                                                                                                );
                                                                                                                                                                                return h;
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        crate::src::common::base::x264_free(h as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::src::common::common::x264_t>();
    }
}

unsafe extern "C" fn encoder_try_reconfig(
    mut h: *mut crate::src::common::common::x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
    mut rc_reconfig: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        *rc_reconfig = 0 as ::core::ffi::c_int;
        set_aspect_ratio(h, param, 0 as ::core::ffi::c_int);
        (*h).param.i_frame_reference = (*param).i_frame_reference;
        (*h).param.i_bframe_bias = (*param).i_bframe_bias;
        if (*h).param.i_scenecut_threshold != 0 {
            (*h).param.i_scenecut_threshold = (*param).i_scenecut_threshold;
        }
        (*h).param.b_deblocking_filter = (*param).b_deblocking_filter;
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
        (*h).param.analyse.b_chroma_me = (*param).analyse.b_chroma_me;
        (*h).param.analyse.b_dct_decimate = (*param).analyse.b_dct_decimate;
        (*h).param.analyse.b_fast_pskip = (*param).analyse.b_fast_pskip;
        (*h).param.analyse.b_mixed_references = (*param).analyse.b_mixed_references;
        (*h).param.analyse.f_psy_rd = (*param).analyse.f_psy_rd;
        (*h).param.analyse.f_psy_trellis = (*param).analyse.f_psy_trellis;
        (*h).param.crop_rect = (*param).crop_rect;
        if (*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA
            || (*param).analyse.i_me_method < crate::x264_h::X264_ME_ESA
        {
            (*h).param.analyse.i_me_method = (*param).analyse.i_me_method;
        }
        if (*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA
            && (*h).frames.b_have_sub8x8_esa == 0
        {
            (*h).param.analyse.inter &= !crate::x264_h::X264_ANALYSE_PSUB8x8;
        }
        if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).b_transform_8x8_mode
            != 0
        {
            (*h).param.analyse.b_transform_8x8 = (*param).analyse.b_transform_8x8;
        }
        if (*h).frames.i_max_ref1 > 1 as ::core::ffi::c_int {
            (*h).param.i_bframe_pyramid = (*param).i_bframe_pyramid;
        }
        (*h).param.i_slice_max_size = (*param).i_slice_max_size;
        (*h).param.i_slice_max_mbs = (*param).i_slice_max_mbs;
        (*h).param.i_slice_min_mbs = (*param).i_slice_min_mbs;
        (*h).param.i_slice_count = (*param).i_slice_count;
        (*h).param.i_slice_count_max = (*param).i_slice_count_max;
        (*h).param.b_tff = (*param).b_tff;
        if (*h).param.rc.i_vbv_max_bitrate > 0 as ::core::ffi::c_int
            && (*h).param.rc.i_vbv_buffer_size > 0 as ::core::ffi::c_int
            && (*param).rc.i_vbv_max_bitrate > 0 as ::core::ffi::c_int
            && (*param).rc.i_vbv_buffer_size > 0 as ::core::ffi::c_int
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
        return validate_parameters(h, 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_reconfig_apply(
    mut h: *mut crate::src::common::common::x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc_reconfig: ::core::ffi::c_int = 0;
        let mut ret: ::core::ffi::c_int = encoder_try_reconfig(h, param, &raw mut rc_reconfig);
        mbcmp_init(h);
        if ret == 0 {
            crate::src::encoder::set::x264_8_sps_init_reconfigurable(
                &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t
                    as *mut crate::src::common::set::x264_sps_t,
                &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
            );
        }
        if ret == 0 && rc_reconfig != 0 {
            crate::src::encoder::ratecontrol::x264_8_ratecontrol_init_reconfigurable(
                h as *mut crate::src::common::common::x264_t,
                0 as ::core::ffi::c_int,
            );
        }
        return ret;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_reconfig(
    mut h: *mut crate::src::common::common::x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
) -> ::core::ffi::c_int {
    unsafe {
        h = (*h).thread[(*(*h).thread[0 as ::core::ffi::c_int as usize]).i_thread_phase as usize];
        let mut param_save: crate::x264_h::x264_param_t = (*(*h).reconfig_h).param;
        (*(*h).reconfig_h).param = (*h).param;
        let mut rc_reconfig: ::core::ffi::c_int = 0;
        let mut ret: ::core::ffi::c_int =
            encoder_try_reconfig((*h).reconfig_h, param, &raw mut rc_reconfig);
        if ret == 0 {
            (*h).reconfig = 1 as ::core::ffi::c_int;
        } else {
            (*(*h).reconfig_h).param = param_save;
        }
        return ret;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_parameters(
    mut h: *mut crate::src::common::common::x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        crate::stdlib::memcpy(
            param as *mut ::core::ffi::c_void,
            &raw mut (**(&raw mut (*h).thread as *mut *mut crate::src::common::common::x264_t)
                .offset((*h).i_thread_phase as isize))
            .param as *const ::core::ffi::c_void,
            ::core::mem::size_of::<crate::x264_h::x264_param_t>()
                as crate::__stddef_size_t_h::size_t,
        );
        (*param).opaque = crate::__stddef_null_h::NULL;
    }
}

unsafe extern "C" fn nal_start(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_type: ::core::ffi::c_int,
    mut i_ref_idc: ::core::ffi::c_int,
) {
    unsafe {
        let mut nal: *mut crate::x264_h::x264_nal_t =
            (*h).out.nal.offset((*h).out.i_nal as isize) as *mut crate::x264_h::x264_nal_t;
        (*nal).i_ref_idc = i_ref_idc;
        (*nal).i_type = i_type;
        (*nal).b_long_startcode = 1 as ::core::ffi::c_int;
        (*nal).i_payload = 0 as ::core::ffi::c_int;
        (*nal).p_payload = (*h).out.p_bitstream.offset(
            ((bs_pos
                as unsafe extern "C" fn(
                    *mut crate::src::common::bitstream::bs_t,
                ) -> ::core::ffi::c_int)(&raw mut (*h).out.bs)
                / 8 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t;
        (*nal).i_padding = 0 as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn nal_check_buffer(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*h).out.i_nal >= (*h).out.i_nals_allocated {
            let mut new_out: *mut crate::x264_h::x264_nal_t = crate::src::common::base::x264_malloc(
                (::core::mem::size_of::<crate::x264_h::x264_nal_t>() as usize)
                    .wrapping_mul(((*h).out.i_nals_allocated * 2 as ::core::ffi::c_int) as usize)
                    as crate::stdlib::int64_t,
            )
                as *mut crate::x264_h::x264_nal_t;
            if new_out.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
            crate::stdlib::memcpy(
                new_out as *mut ::core::ffi::c_void,
                (*h).out.nal as *const ::core::ffi::c_void,
                (::core::mem::size_of::<crate::x264_h::x264_nal_t>()
                    as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul((*h).out.i_nals_allocated as crate::__stddef_size_t_h::size_t),
            );
            crate::src::common::base::x264_free((*h).out.nal as *mut ::core::ffi::c_void);
            (*h).out.nal = new_out;
            (*h).out.i_nals_allocated *= 2 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn nal_end(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nal: *mut crate::x264_h::x264_nal_t =
            (*h).out.nal.offset((*h).out.i_nal as isize) as *mut crate::x264_h::x264_nal_t;
        let mut end: *mut crate::stdlib::uint8_t = (*h).out.p_bitstream.offset(
            ((bs_pos
                as unsafe extern "C" fn(
                    *mut crate::src::common::bitstream::bs_t,
                ) -> ::core::ffi::c_int)(&raw mut (*h).out.bs)
                / 8 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t;
        (*nal).i_payload =
            end.offset_from((*nal).p_payload) as ::core::ffi::c_long as ::core::ffi::c_int;
        crate::stdlib::memset(
            end as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            64 as crate::__stddef_size_t_h::size_t,
        );
        if (*h).param.nalu_process.is_some() {
            (*h).param.nalu_process.expect("non-null function pointer")(
                (*h).api as *mut crate::src::common::common::x264_t,
                nal,
                (*(*h).fenc).opaque,
            );
        }
        (*h).out.i_nal += 1;
        return nal_check_buffer(h);
    }
}

unsafe extern "C" fn check_encapsulated_buffer(
    mut h: *mut crate::src::common::common::x264_t,
    mut h0: *mut crate::src::common::common::x264_t,
    mut start: ::core::ffi::c_int,
    mut previous_nal_size: crate::stdlib::int64_t,
    mut necessary_size: crate::stdlib::int64_t,
) -> ::core::ffi::c_int {
    unsafe {
        if ((*h0).nal_buffer_size as crate::stdlib::int64_t) < necessary_size {
            necessary_size *= 2 as crate::stdlib::int64_t;
            if necessary_size > crate::limits_h::INT_MAX as crate::stdlib::int64_t {
                return -(1 as ::core::ffi::c_int);
            }
            let mut buf: *mut crate::stdlib::uint8_t =
                crate::src::common::base::x264_malloc(necessary_size)
                    as *mut crate::stdlib::uint8_t;
            if buf.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
            if previous_nal_size != 0 {
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (*h0).nal_buffer as *const ::core::ffi::c_void,
                    previous_nal_size as crate::__stddef_size_t_h::size_t,
                );
            }
            let mut delta: crate::stdlib::intptr_t =
                buf.offset_from((*h0).nal_buffer) as crate::stdlib::intptr_t;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < start {
                let ref mut c2rust_fresh1 = (*(*h).out.nal.offset(i as isize)).p_payload;
                *c2rust_fresh1 = (*c2rust_fresh1).offset(delta as isize);
                i += 1;
            }
            crate::src::common::base::x264_free((*h0).nal_buffer as *mut ::core::ffi::c_void);
            (*h0).nal_buffer = buf;
            (*h0).nal_buffer_size = necessary_size as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn encoder_encapsulate_nals(
    mut h: *mut crate::src::common::common::x264_t,
    mut start: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut h0: *mut crate::src::common::common::x264_t =
            (*h).thread[0 as ::core::ffi::c_int as usize];
        let mut nal_size: crate::stdlib::int64_t = 0 as crate::stdlib::int64_t;
        let mut previous_nal_size: crate::stdlib::int64_t = 0 as crate::stdlib::int64_t;
        if (*h).param.nalu_process.is_some() {
            let mut i: ::core::ffi::c_int = start;
            while i < (*h).out.i_nal {
                nal_size += (*(*h).out.nal.offset(i as isize)).i_payload as crate::stdlib::int64_t;
                i += 1;
            }
            if nal_size > crate::limits_h::INT_MAX as crate::stdlib::int64_t {
                return -(1 as ::core::ffi::c_int);
            }
            return nal_size as ::core::ffi::c_int;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < start {
            previous_nal_size +=
                (*(*h).out.nal.offset(i_0 as isize)).i_payload as crate::stdlib::int64_t;
            i_0 += 1;
        }
        let mut i_1: ::core::ffi::c_int = start;
        while i_1 < (*h).out.i_nal {
            nal_size += (*(*h).out.nal.offset(i_1 as isize)).i_payload as crate::stdlib::int64_t;
            i_1 += 1;
        }
        let mut necessary_size: crate::stdlib::int64_t = previous_nal_size
            + nal_size * 3 as crate::stdlib::int64_t / 2 as crate::stdlib::int64_t
            + ((*h).out.i_nal * 4 as ::core::ffi::c_int) as crate::stdlib::int64_t
            + 4 as crate::stdlib::int64_t
            + 64 as crate::stdlib::int64_t;
        let mut i_2: ::core::ffi::c_int = start;
        while i_2 < (*h).out.i_nal {
            necessary_size +=
                (*(*h).out.nal.offset(i_2 as isize)).i_padding as crate::stdlib::int64_t;
            i_2 += 1;
        }
        if check_encapsulated_buffer(h, h0, start, previous_nal_size, necessary_size) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        let mut nal_buffer: *mut crate::stdlib::uint8_t =
            (*h0).nal_buffer.offset(previous_nal_size as isize);
        let mut i_3: ::core::ffi::c_int = start;
        while i_3 < (*h).out.i_nal {
            (*(*h).out.nal.offset(i_3 as isize)).b_long_startcode = (i_3 == 0
                || (*(*h).out.nal.offset(i_3 as isize)).i_type
                    == crate::x264_h::NAL_SPS as ::core::ffi::c_int
                || (*(*h).out.nal.offset(i_3 as isize)).i_type
                    == crate::x264_h::NAL_PPS as ::core::ffi::c_int
                || (*h).param.i_avcintra_class != 0)
                as ::core::ffi::c_int;
            x264_8_nal_encode(
                h,
                nal_buffer,
                (*h).out.nal.offset(i_3 as isize) as *mut crate::x264_h::x264_nal_t,
            );
            nal_buffer = nal_buffer.offset((*(*h).out.nal.offset(i_3 as isize)).i_payload as isize);
            i_3 += 1;
        }
        return nal_buffer.offset_from((*h0).nal_buffer.offset(previous_nal_size as isize))
            as ::core::ffi::c_long as ::core::ffi::c_int;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_headers(
    mut h: *mut crate::src::common::common::x264_t,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut frame_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        (*h).out.i_nal = 0 as ::core::ffi::c_int;
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
        crate::src::encoder::set::x264_8_sps_write(
            &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
            &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t
                as *mut crate::src::common::set::x264_sps_t,
        );
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        nal_start(
            h,
            crate::x264_h::NAL_PPS as ::core::ffi::c_int,
            crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
        );
        crate::src::encoder::set::x264_8_pps_write(
            &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
            &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t
                as *mut crate::src::common::set::x264_sps_t,
            &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t
                as *mut crate::src::common::set::x264_pps_t,
        );
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        nal_start(
            h,
            crate::x264_h::NAL_SEI as ::core::ffi::c_int,
            crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        if crate::src::encoder::set::x264_8_sei_version_write(
            h as *mut crate::src::common::common::x264_t,
            &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
        ) != 0
        {
            return -(1 as ::core::ffi::c_int);
        }
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        frame_size = encoder_encapsulate_nals(h, 0 as ::core::ffi::c_int);
        if frame_size < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        *pi_nal = (*h).out.i_nal;
        *pp_nal =
            (*h).out.nal.offset(0 as ::core::ffi::c_int as isize) as *mut crate::x264_h::x264_nal_t;
        (*h).out.i_nal = 0 as ::core::ffi::c_int;
        return frame_size;
    }
}
#[inline]

unsafe extern "C" fn reference_check_reorder(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*h).frames.reference[i as usize].is_null() {
            if (*(*h).frames.reference[i as usize]).b_corrupt != 0 {
                (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
                return;
            }
            i += 1;
        }
        let mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while list
            <= ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                as ::core::ffi::c_int
        {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < (*h).i_ref[list as usize] - 1 as ::core::ffi::c_int {
                let mut framenum_diff: ::core::ffi::c_int = (*(*h).fref[list as usize]
                    [(i_0 + 1 as ::core::ffi::c_int) as usize])
                    .i_frame_num
                    - (*(*h).fref[list as usize][i_0 as usize]).i_frame_num;
                let mut poc_diff: ::core::ffi::c_int =
                    (*(*h).fref[list as usize][(i_0 + 1 as ::core::ffi::c_int) as usize]).i_poc
                        - (*(*h).fref[list as usize][i_0 as usize]).i_poc;
                if if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                {
                    (framenum_diff > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                } else if list == 1 as ::core::ffi::c_int {
                    (poc_diff < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                } else {
                    (poc_diff > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                } != 0
                {
                    (*h).b_ref_reorder[list as usize] = 1 as ::core::ffi::c_int;
                    return;
                }
                i_0 += 1;
            }
            list += 1;
        }
    }
}

unsafe extern "C" fn weighted_reference_duplicate(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_ref: ::core::ffi::c_int,
    mut w: *const crate::src::common::mc::x264_weight_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = (*h).i_ref[0 as ::core::ffi::c_int as usize];
        let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut newframe: *mut crate::src::common::frame::x264_frame_t =
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        if i <= 1 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.analyse.i_weighted_pred != crate::x264_h::X264_WEIGHTP_SMART {
            return -(1 as ::core::ffi::c_int);
        }
        if crate::internal::BIT_DEPTH > 8 as ::core::ffi::c_int
            && w != &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                as *const crate::src::common::mc::x264_weight_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        newframe = crate::src::common::frame::x264_8_frame_pop_blank_unused(
            h as *mut crate::src::common::common::x264_t,
        ) as *mut crate::src::common::frame::x264_frame;
        if newframe.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        *newframe = *(*h).fref[0 as ::core::ffi::c_int as usize][i_ref as usize];
        (*newframe).i_reference_count = 1 as ::core::ffi::c_int;
        (*newframe).orig = (*h).fref[0 as ::core::ffi::c_int as usize][i_ref as usize]
            as *mut crate::src::common::frame::x264_frame;
        (*newframe).b_duplicate = 1 as ::core::ffi::c_int;
        crate::stdlib::memcpy(
            &raw mut *(&raw mut (*(*h).fenc).weight
                as *mut [crate::src::common::mc::x264_weight_t; 3])
                .offset(j as isize) as *mut crate::src::common::mc::x264_weight_t
                as *mut ::core::ffi::c_void,
            w as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::src::common::mc::x264_weight_t; 3]>()
                as crate::__stddef_size_t_h::size_t,
        );
        (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if (*h).i_ref[0 as ::core::ffi::c_int as usize] < crate::src::common::base::X264_REF_MAX {
            (*h).i_ref[0 as ::core::ffi::c_int as usize] += 1;
        }
        (*h).fref[0 as ::core::ffi::c_int as usize]
            [(crate::src::common::base::X264_REF_MAX - 1 as ::core::ffi::c_int) as usize] =
            ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        crate::src::common::frame::x264_8_frame_unshift(
            (&raw mut *(&raw mut (*h).fref
                as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut crate::src::common::frame::x264_frame_t)
                .offset(j as isize)
                as *mut *mut crate::src::common::frame::x264_frame_t
                as *mut *mut crate::src::common::frame::x264_frame,
            newframe as *mut crate::src::common::frame::x264_frame,
        );
        return j;
    }
}

unsafe extern "C" fn weighted_pred_init(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut i_ref: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_ref < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
            (*(*h).fenc).weighted[i_ref as usize] = (*(*h).fref[0 as ::core::ffi::c_int as usize]
                [i_ref as usize])
                .filtered[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize];
            i_ref += 1;
        }
        (*(*h).fenc).i_lines_weighted = 0 as ::core::ffi::c_int;
        let mut i_ref_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_ref_0 < (*h).i_ref[0 as ::core::ffi::c_int as usize] << (*h).sh.b_mbaff {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 3 as ::core::ffi::c_int {
                (*h).sh.weight[i_ref_0 as usize][i as usize].weightfn =
                    ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
                i += 1;
            }
            i_ref_0 += 1;
        }
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            || (*h).param.analyse.i_weighted_pred <= 0 as ::core::ffi::c_int
        {
            return;
        }
        let mut i_padv: ::core::ffi::c_int =
            crate::src::common::frame::PADV << (*h).param.b_interlaced;
        let mut denom: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut weightplane: [::core::ffi::c_int; 2] =
            [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int];
        let mut buffer_next: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 3 as ::core::ffi::c_int {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
                if !(*(*h).fenc).weight[j as usize][i_0 as usize]
                    .weightfn
                    .is_null()
                {
                    (*h).sh.weight[j as usize][i_0 as usize] =
                        (*(*h).fenc).weight[j as usize][i_0 as usize];
                    if (*h).sh.weight[j as usize][i_0 as usize].i_scale
                        == (1 as crate::stdlib::int32_t)
                            << (*h).sh.weight[j as usize][i_0 as usize].i_denom
                        && (*h).sh.weight[j as usize][i_0 as usize].i_offset
                            == 0 as crate::stdlib::int32_t
                    {
                        (*h).sh.weight[j as usize][i_0 as usize].weightfn =
                            ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
                    } else {
                        if weightplane[(i_0 != 0) as ::core::ffi::c_int as usize] == 0 {
                            weightplane[(i_0 != 0) as ::core::ffi::c_int as usize] =
                                1 as ::core::ffi::c_int;
                            denom = (*h).sh.weight[j as usize][i_0 as usize].i_denom
                                as ::core::ffi::c_int;
                            (*h).sh.weight[0 as ::core::ffi::c_int as usize]
                                [(i_0 != 0) as ::core::ffi::c_int as usize]
                                .i_denom = denom as crate::stdlib::int32_t;
                            '_c2rust_label: {
                                if x264_clip3(
                                    denom,
                                    0 as ::core::ffi::c_int,
                                    7 as ::core::ffi::c_int,
                                ) == denom
                                {
                                } else {
                                    crate::stdlib::__assert_fail(
                                        b"x264_clip3( denom, 0, 7 ) == denom\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"encoder/encoder.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2240 as ::core::ffi::c_uint,
                                        b"void weighted_pred_init(x264_t *)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                        }
                        '_c2rust_label_0: {
                            if (*h).sh.weight[j as usize][i_0 as usize].i_denom
                                == denom as crate::stdlib::int32_t
                            {
                            } else {
                                crate::stdlib::__assert_fail(
                                    b"h->sh.weight[j][i].i_denom == denom\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                                    2243 as ::core::ffi::c_uint,
                                    b"void weighted_pred_init(x264_t *)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        };
                        if i_0 == 0 {
                            let c2rust_fresh3 = buffer_next;
                            buffer_next = buffer_next + 1;
                            (*(*h).fenc).weighted[j as usize] = (*h).mb.p_weight_buf
                                [c2rust_fresh3 as usize]
                                .offset(
                                    ((*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize]
                                        * i_padv) as isize,
                                )
                                .offset(
                                    (if 32 as ::core::ffi::c_int
                                        > 64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    {
                                        32 as ::core::ffi::c_int
                                    } else {
                                        64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    }) as isize,
                                );
                            if (*h).param.i_threads == 1 as ::core::ffi::c_int {
                                let mut src: *mut crate::src::common::common::pixel = (*(*h).fref
                                    [0 as ::core::ffi::c_int as usize]
                                    [j as usize])
                                    .filtered
                                    [0 as ::core::ffi::c_int as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    .offset(
                                        -(((*(*h).fref[0 as ::core::ffi::c_int as usize]
                                            [j as usize])
                                            .i_stride
                                            [0 as ::core::ffi::c_int as usize]
                                            * i_padv)
                                            as isize),
                                    )
                                    .offset(
                                        -((if 32 as ::core::ffi::c_int
                                            > 64 as ::core::ffi::c_int
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        {
                                            32 as ::core::ffi::c_int
                                        } else {
                                            64 as ::core::ffi::c_int
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        }) as isize),
                                    );
                                let mut dst: *mut crate::src::common::common::pixel = (*(*h).fenc)
                                    .weighted[j as usize]
                                    .offset(
                                        -(((*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize]
                                            * i_padv)
                                            as isize),
                                    )
                                    .offset(
                                        -((if 32 as ::core::ffi::c_int
                                            > 64 as ::core::ffi::c_int
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        {
                                            32 as ::core::ffi::c_int
                                        } else {
                                            64 as ::core::ffi::c_int
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        }) as isize),
                                    );
                                let mut stride: ::core::ffi::c_int =
                                    (*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize];
                                let mut width: ::core::ffi::c_int = (*(*h).fenc).i_width
                                    [0 as ::core::ffi::c_int as usize]
                                    + ((if 32 as ::core::ffi::c_int
                                        > 64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    {
                                        32 as ::core::ffi::c_int
                                    } else {
                                        64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    }) + crate::src::common::frame::PADH);
                                let mut height: ::core::ffi::c_int = (*(*h).fenc).i_lines
                                    [0 as ::core::ffi::c_int as usize]
                                    + i_padv * 2 as ::core::ffi::c_int;
                                crate::src::common::frame::x264_8_weight_scale_plane(
                                    h as *mut crate::src::common::common::x264_t,
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
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::src::common::mc::x264_weight_t
                                        as *mut crate::src::common::mc::x264_weight_t,
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
        if weightplane[1 as ::core::ffi::c_int as usize] != 0 {
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
                if !(*h).sh.weight[i_1 as usize][1 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
                    && (*h).sh.weight[i_1 as usize][2 as ::core::ffi::c_int as usize]
                        .weightfn
                        .is_null()
                {
                    (*h).sh.weight[i_1 as usize][2 as ::core::ffi::c_int as usize].i_scale =
                        ((1 as ::core::ffi::c_int)
                            << (*h).sh.weight[0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                .i_denom) as crate::stdlib::int32_t;
                    (*h).sh.weight[i_1 as usize][2 as ::core::ffi::c_int as usize].i_offset =
                        0 as ::core::ffi::c_int as crate::stdlib::int32_t;
                } else if !(*h).sh.weight[i_1 as usize][2 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
                    && (*h).sh.weight[i_1 as usize][1 as ::core::ffi::c_int as usize]
                        .weightfn
                        .is_null()
                {
                    (*h).sh.weight[i_1 as usize][1 as ::core::ffi::c_int as usize].i_scale =
                        ((1 as ::core::ffi::c_int)
                            << (*h).sh.weight[0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                .i_denom) as crate::stdlib::int32_t;
                    (*h).sh.weight[i_1 as usize][1 as ::core::ffi::c_int as usize].i_offset =
                        0 as ::core::ffi::c_int as crate::stdlib::int32_t;
                }
                i_1 += 1;
            }
        }
        if weightplane[0 as ::core::ffi::c_int as usize] == 0 {
            (*h).sh.weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                .i_denom = 0 as ::core::ffi::c_int as crate::stdlib::int32_t;
        }
        if weightplane[1 as ::core::ffi::c_int as usize] == 0 {
            (*h).sh.weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                .i_denom = 0 as ::core::ffi::c_int as crate::stdlib::int32_t;
        }
        (*h).sh.weight[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize]
            .i_denom = (*h).sh.weight[0 as ::core::ffi::c_int as usize]
            [1 as ::core::ffi::c_int as usize]
            .i_denom;
    }
}
#[inline]

unsafe extern "C" fn reference_distance(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*h).param.i_frame_packing == 5 as ::core::ffi::c_int {
            return crate::stdlib::abs(
                ((*(*h).fenc).i_frame & !(1 as ::core::ffi::c_int))
                    - ((*frame).i_frame & !(1 as ::core::ffi::c_int)),
            ) + ((*(*h).fenc).i_frame & 1 as ::core::ffi::c_int
                != (*frame).i_frame & 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        } else {
            return crate::stdlib::abs((*(*h).fenc).i_frame - (*frame).i_frame);
        };
    }
}
#[inline]

unsafe extern "C" fn reference_build_list(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_poc: ::core::ffi::c_int,
) {
    unsafe {
        let mut b_ok: ::core::ffi::c_int = 0;
        (*h).i_ref[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] =
            (*h).i_ref[0 as ::core::ffi::c_int as usize];
        (*h).i_ref[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] =
            (*h).i_ref[1 as ::core::ffi::c_int as usize];
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            return;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*h).frames.reference[i as usize].is_null() {
            if !((*(*h).frames.reference[i as usize]).b_corrupt != 0) {
                if (*(*h).frames.reference[i as usize]).i_poc < i_poc {
                    let c2rust_fresh4 = (*h).i_ref[0 as ::core::ffi::c_int as usize];
                    (*h).i_ref[0 as ::core::ffi::c_int as usize] =
                        (*h).i_ref[0 as ::core::ffi::c_int as usize] + 1;
                    (*h).fref[0 as ::core::ffi::c_int as usize][c2rust_fresh4 as usize] =
                        (*h).frames.reference[i as usize];
                } else if (*(*h).frames.reference[i as usize]).i_poc > i_poc {
                    let c2rust_fresh5 = (*h).i_ref[1 as ::core::ffi::c_int as usize];
                    (*h).i_ref[1 as ::core::ffi::c_int as usize] =
                        (*h).i_ref[1 as ::core::ffi::c_int as usize] + 1;
                    (*h).fref[1 as ::core::ffi::c_int as usize][c2rust_fresh5 as usize] =
                        (*h).frames.reference[i as usize];
                }
            }
            i += 1;
        }
        if (*h).sh.i_mmco_remove_from_end != 0 {
            loop {
                b_ok = 1 as ::core::ffi::c_int;
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < (*h).i_ref[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int {
                    if (*(*h).fref[0 as ::core::ffi::c_int as usize][i_0 as usize]).i_frame
                        < (*(*h).fref[0 as ::core::ffi::c_int as usize]
                            [(i_0 + 1 as ::core::ffi::c_int) as usize])
                            .i_frame
                    {
                        let mut t: *mut crate::src::common::frame::x264_frame_t =
                            (*h).fref[0 as ::core::ffi::c_int as usize][i_0 as usize];
                        (*h).fref[0 as ::core::ffi::c_int as usize][i_0 as usize] = (*h).fref
                            [0 as ::core::ffi::c_int as usize]
                            [(i_0 + 1 as ::core::ffi::c_int) as usize];
                        (*h).fref[0 as ::core::ffi::c_int as usize]
                            [(i_0 + 1 as ::core::ffi::c_int) as usize] = t;
                        b_ok = 0 as ::core::ffi::c_int;
                        break;
                    } else {
                        i_0 += 1;
                    }
                }
                if !(b_ok == 0) {
                    break;
                }
            }
            let mut i_1: ::core::ffi::c_int =
                (*h).i_ref[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int;
            while i_1
                >= (*h).i_ref[0 as ::core::ffi::c_int as usize] - (*h).sh.i_mmco_remove_from_end
            {
                let mut diff: ::core::ffi::c_int = (*h).i_frame_num
                    - (*(*h).fref[0 as ::core::ffi::c_int as usize][i_1 as usize]).i_frame_num;
                (*h).sh.mmco[(*h).sh.i_mmco_command_count as usize].i_poc =
                    (*(*h).fref[0 as ::core::ffi::c_int as usize][i_1 as usize]).i_poc;
                let c2rust_fresh6 = (*h).sh.i_mmco_command_count;
                (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_command_count + 1;
                (*h).sh.mmco[c2rust_fresh6 as usize].i_difference_of_pic_nums = diff;
                i_1 -= 1;
            }
        }
        let mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while list < 2 as ::core::ffi::c_int {
            (*h).fref_nearest[list as usize] =
                (*h).fref[list as usize][0 as ::core::ffi::c_int as usize];
            loop {
                b_ok = 1 as ::core::ffi::c_int;
                let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_2 < (*h).i_ref[list as usize] - 1 as ::core::ffi::c_int {
                    if if list != 0 {
                        ((*(*h).fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize])
                            .i_poc
                            < (*(*h).fref_nearest[list as usize]).i_poc)
                            as ::core::ffi::c_int
                    } else {
                        ((*(*h).fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize])
                            .i_poc
                            > (*(*h).fref_nearest[list as usize]).i_poc)
                            as ::core::ffi::c_int
                    } != 0
                    {
                        (*h).fref_nearest[list as usize] =
                            (*h).fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize];
                    }
                    if reference_distance(h, (*h).fref[list as usize][i_2 as usize])
                        > reference_distance(
                            h,
                            (*h).fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize],
                        )
                    {
                        let mut t_0: *mut crate::src::common::frame::x264_frame_t =
                            (*h).fref[list as usize][i_2 as usize];
                        (*h).fref[list as usize][i_2 as usize] =
                            (*h).fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize];
                        (*h).fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize] = t_0;
                        b_ok = 0 as ::core::ffi::c_int;
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
        (*h).i_ref[1 as ::core::ffi::c_int as usize] =
            if (*h).i_ref[1 as ::core::ffi::c_int as usize] < (*h).frames.i_max_ref1 {
                (*h).i_ref[1 as ::core::ffi::c_int as usize]
            } else {
                (*h).frames.i_max_ref1
            };
        (*h).i_ref[0 as ::core::ffi::c_int as usize] =
            if (*h).i_ref[0 as ::core::ffi::c_int as usize] < (*h).frames.i_max_ref0 {
                (*h).i_ref[0 as ::core::ffi::c_int as usize]
            } else {
                (*h).frames.i_max_ref0
            };
        (*h).i_ref[0 as ::core::ffi::c_int as usize] =
            if (*h).i_ref[0 as ::core::ffi::c_int as usize] < (*h).param.i_frame_reference {
                (*h).i_ref[0 as ::core::ffi::c_int as usize]
            } else {
                (*h).param.i_frame_reference
            };
        if ((*(*h).fenc).i_type == crate::x264_h::X264_TYPE_B
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_BREF)
            && (*h).param.b_bluray_compat != 0
        {
            (*h).i_ref[0 as ::core::ffi::c_int as usize] = if (*h).i_ref
                [0 as ::core::ffi::c_int as usize]
                < ((*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_type
                    == 0x5 as ::core::ffi::c_int
                    || (*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .i_type
                        == 0x4 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
            {
                (*h).i_ref[0 as ::core::ffi::c_int as usize]
            } else {
                ((*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_type
                    == 0x5 as ::core::ffi::c_int
                    || (*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .i_type
                        == 0x4 as ::core::ffi::c_int) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
            };
        }
        if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_P {
            let mut idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            if (*h).param.analyse.i_weighted_pred >= crate::x264_h::X264_WEIGHTP_SIMPLE {
                let mut w: [crate::src::common::mc::x264_weight_t; 3] =
                    [crate::src::common::mc::x264_weight_t {
                        cachea: [0; 8],
                        cacheb: [0; 8],
                        i_denom: 0,
                        i_scale: 0,
                        i_offset: 0,
                        weightfn: ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>(),
                    }; 3];
                w[2 as ::core::ffi::c_int as usize].weightfn =
                    ::core::ptr::null_mut::<crate::src::common::mc::weight_fn_t>();
                w[1 as ::core::ffi::c_int as usize].weightfn =
                    w[2 as ::core::ffi::c_int as usize].weightfn;
                if (*h).param.rc.b_stat_read != 0 {
                    crate::src::encoder::ratecontrol::x264_8_ratecontrol_set_weights(
                        h as *mut crate::src::common::common::x264_t,
                        (*h).fenc as *mut crate::src::common::frame::x264_frame,
                    );
                }
                if (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
                {
                    (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize]
                        .i_denom = 0 as ::core::ffi::c_int as crate::stdlib::int32_t;
                    w[0 as ::core::ffi::c_int as usize].i_scale =
                        1 as ::core::ffi::c_int as crate::stdlib::int32_t;
                    w[0 as ::core::ffi::c_int as usize].i_denom =
                        0 as ::core::ffi::c_int as crate::stdlib::int32_t;
                    w[0 as ::core::ffi::c_int as usize].i_offset =
                        -(1 as ::core::ffi::c_int) as crate::stdlib::int32_t;
                    (*h).mc.weight_cache.expect("non-null function pointer")(
                        h,
                        (&raw mut w as *mut crate::src::common::mc::x264_weight_t)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::mc::x264_weight_t,
                    );
                    idx = weighted_reference_duplicate(
                        h,
                        0 as ::core::ffi::c_int,
                        &raw mut w as *mut crate::src::common::mc::x264_weight_t,
                    );
                } else {
                    if (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize]
                        .i_scale
                        == (1 as crate::stdlib::int32_t)
                            << (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                                [0 as ::core::ffi::c_int as usize]
                                .i_denom
                    {
                        (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize]
                            .i_scale = 1 as ::core::ffi::c_int as crate::stdlib::int32_t;
                        (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize]
                            .i_denom = 0 as ::core::ffi::c_int as crate::stdlib::int32_t;
                        (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize]
                            .i_offset = (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize]
                            .i_offset;
                        (*h).mc.weight_cache.expect("non-null function pointer")(
                            h,
                            (&raw mut *(&raw mut (*(*h).fenc).weight
                                as *mut [crate::src::common::mc::x264_weight_t; 3])
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::src::common::mc::x264_weight_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::src::common::mc::x264_weight_t,
                        );
                    }
                    weighted_reference_duplicate(
                        h,
                        0 as ::core::ffi::c_int,
                        &raw mut crate::src::common::tables::x264_zero
                            as *mut crate::stdlib::uint8_t
                            as *const crate::src::common::mc::x264_weight_t,
                    );
                    if (*(*h).fenc).weight[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize]
                        .i_offset
                        > -(128 as crate::stdlib::int32_t)
                    {
                        w[0 as ::core::ffi::c_int as usize] = (*(*h).fenc).weight
                            [0 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize];
                        w[0 as ::core::ffi::c_int as usize].i_offset -= 1;
                        (*h).mc.weight_cache.expect("non-null function pointer")(
                            h,
                            (&raw mut w as *mut crate::src::common::mc::x264_weight_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::src::common::mc::x264_weight_t,
                        );
                        idx = weighted_reference_duplicate(
                            h,
                            0 as ::core::ffi::c_int,
                            &raw mut w as *mut crate::src::common::mc::x264_weight_t,
                        );
                    }
                }
            }
            (*h).mb.ref_blind_dupe = idx;
        }
        '_c2rust_label: {
            if (*h).i_ref[0 as ::core::ffi::c_int as usize]
                + (*h).i_ref[1 as ::core::ffi::c_int as usize]
                <= 16 as ::core::ffi::c_int
            {
            } else {
                crate::stdlib::__assert_fail(
                    b"h->i_ref[0] + h->i_ref[1] <= X264_REF_MAX\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2408 as ::core::ffi::c_uint,
                    b"void reference_build_list(x264_t *, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] =
            (*h).i_ref[0 as ::core::ffi::c_int as usize];
        (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] =
            (*h).i_ref[1 as ::core::ffi::c_int as usize];
    }
}

unsafe extern "C" fn fdec_filter_row(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_y: ::core::ffi::c_int,
    mut pass: ::core::ffi::c_int,
) {
    unsafe {
        let mut b_hpel: ::core::ffi::c_int = (*(*h).fdec).b_kept_as_ref;
        let mut b_deblock: ::core::ffi::c_int = ((*h).sh.i_disable_deblocking_filter_idc
            != 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut b_end: ::core::ffi::c_int = (mb_y == (*h).i_threadslice_end) as ::core::ffi::c_int;
        let mut b_measure_quality: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut min_y: ::core::ffi::c_int = mb_y - ((1 as ::core::ffi::c_int) << (*h).sh.b_mbaff);
        let mut b_start: ::core::ffi::c_int =
            (min_y == (*h).i_threadslice_start) as ::core::ffi::c_int;
        let mut minpix_y: ::core::ffi::c_int = min_y * 16 as ::core::ffi::c_int
            - 4 as ::core::ffi::c_int * (b_start == 0) as ::core::ffi::c_int;
        let mut maxpix_y: ::core::ffi::c_int = mb_y * 16 as ::core::ffi::c_int
            - 4 as ::core::ffi::c_int * (b_end == 0) as ::core::ffi::c_int;
        b_deblock &= (b_hpel != 0
            || (*h).param.b_full_recon != 0
            || !(*h).param.psz_dump_yuv.is_null()) as ::core::ffi::c_int;
        if (*h).param.b_sliced_threads != 0 {
            match pass {
                1 => {
                    b_deblock &= ((*h).param.b_full_recon == 0) as ::core::ffi::c_int;
                    b_hpel &=
                        !(b_start != 0 && min_y > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                    b_measure_quality = 0 as ::core::ffi::c_int;
                }
                2 => {
                    b_deblock = 0 as ::core::ffi::c_int;
                    b_measure_quality = 0 as ::core::ffi::c_int;
                }
                0 | _ => {
                    b_deblock &= (*h).param.b_full_recon;
                    b_hpel = 0 as ::core::ffi::c_int;
                }
            }
        }
        if mb_y & (*h).sh.b_mbaff != 0 {
            return;
        }
        if min_y < (*h).i_threadslice_start {
            return;
        }
        if b_deblock != 0 {
            let mut y: ::core::ffi::c_int = min_y;
            while y < mb_y {
                crate::src::common::deblock::x264_8_frame_deblock_row(
                    h as *mut crate::src::common::common::x264_t,
                    y,
                );
                y += (1 as ::core::ffi::c_int) << (*h).sh.b_mbaff;
            }
        }
        if (*h).param.b_interlaced != 0
            && ((*h).param.b_sliced_threads == 0 || pass == 1 as ::core::ffi::c_int)
        {
            let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p < (*(*h).fdec).i_plane {
                let mut i: ::core::ffi::c_int = minpix_y
                    >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && p != 0) as ::core::ffi::c_int;
                while i < maxpix_y
                    >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && p != 0) as ::core::ffi::c_int
                {
                    crate::stdlib::memcpy(
                        (*(*h).fdec).plane_fld[p as usize]
                            .offset((i * (*(*h).fdec).i_stride[p as usize]) as isize)
                            as *mut ::core::ffi::c_void,
                        (*(*h).fdec).plane[p as usize]
                            .offset((i * (*(*h).fdec).i_stride[p as usize]) as isize)
                            as *const ::core::ffi::c_void,
                        ((*h).mb.i_mb_width
                            * 16 as ::core::ffi::c_int
                            * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                    i += 1;
                }
                p += 1;
            }
        }
        if (*(*h).fdec).b_kept_as_ref != 0
            && ((*h).param.b_sliced_threads == 0 || pass == 1 as ::core::ffi::c_int)
        {
            crate::src::common::frame::x264_8_frame_expand_border(
                h as *mut crate::src::common::common::x264_t,
                (*h).fdec as *mut crate::src::common::frame::x264_frame,
                min_y,
            );
        }
        if b_hpel != 0 {
            let mut end: ::core::ffi::c_int = (mb_y == (*h).mb.i_mb_height) as ::core::ffi::c_int;
            if (*h).param.analyse.i_subpel_refine != 0 {
                crate::src::common::mc::x264_8_frame_filter(
                    h as *mut crate::src::common::common::x264_t,
                    (*h).fdec as *mut crate::src::common::frame::x264_frame,
                    min_y,
                    end,
                );
                crate::src::common::frame::x264_8_frame_expand_border_filtered(
                    h as *mut crate::src::common::common::x264_t,
                    (*h).fdec as *mut crate::src::common::frame::x264_frame,
                    min_y,
                    end,
                );
            }
        }
        if (*h).sh.b_mbaff != 0 && pass == 0 as ::core::ffi::c_int {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 3 as ::core::ffi::c_int {
                let mut t: *mut crate::src::common::common::pixel =
                    (*h).intra_border_backup[0 as ::core::ffi::c_int as usize][i_0 as usize];
                (*h).intra_border_backup[0 as ::core::ffi::c_int as usize][i_0 as usize] =
                    (*h).intra_border_backup[3 as ::core::ffi::c_int as usize][i_0 as usize];
                (*h).intra_border_backup[3 as ::core::ffi::c_int as usize][i_0 as usize] = t;
                let mut t_0: *mut crate::src::common::common::pixel =
                    (*h).intra_border_backup[1 as ::core::ffi::c_int as usize][i_0 as usize];
                (*h).intra_border_backup[1 as ::core::ffi::c_int as usize][i_0 as usize] =
                    (*h).intra_border_backup[4 as ::core::ffi::c_int as usize][i_0 as usize];
                (*h).intra_border_backup[4 as ::core::ffi::c_int as usize][i_0 as usize] = t_0;
                i_0 += 1;
            }
        }
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int && (*(*h).fdec).b_kept_as_ref != 0 {
            crate::src::common::frame::x264_8_frame_cond_broadcast(
                (*h).fdec as *mut crate::src::common::frame::x264_frame,
                mb_y * 16 as ::core::ffi::c_int
                    + (if b_end != 0 {
                        10000 as ::core::ffi::c_int
                    } else {
                        -(crate::src::common::base::X264_THREAD_HEIGHT << (*h).sh.b_mbaff)
                    }),
            );
        }
        if b_measure_quality != 0 {
            maxpix_y = if maxpix_y < (*h).param.i_height {
                maxpix_y
            } else {
                (*h).param.i_height
            };
            if (*h).param.analyse.b_psnr != 0 {
                let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while p_0
                    < (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                {
                    (*h).stat.frame.i_ssd[p_0 as usize] = ((*h).stat.frame.i_ssd[p_0 as usize]
                        as crate::stdlib::uint64_t)
                        .wrapping_add(crate::src::common::pixel::x264_8_pixel_ssd_wxh(
                            &raw mut (*h).pixf as *mut _
                                as *mut crate::src::common::pixel::x264_pixel_function_t,
                            (*(*h).fdec).plane[p_0 as usize]
                                .offset((minpix_y * (*(*h).fdec).i_stride[p_0 as usize]) as isize),
                            (*(*h).fdec).i_stride[p_0 as usize] as crate::stdlib::intptr_t,
                            (*(*h).fenc).plane[p_0 as usize]
                                .offset((minpix_y * (*(*h).fenc).i_stride[p_0 as usize]) as isize),
                            (*(*h).fenc).i_stride[p_0 as usize] as crate::stdlib::intptr_t,
                            (*h).param.i_width,
                            maxpix_y - minpix_y,
                        ))
                        as crate::stdlib::int64_t
                        as crate::stdlib::int64_t;
                    p_0 += 1;
                }
                if !(crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int)
                {
                    let mut ssd_u: crate::stdlib::uint64_t = 0;
                    let mut ssd_v: crate::stdlib::uint64_t = 0;
                    let mut v_shift: ::core::ffi::c_int = (crate::src::common::base::CHROMA_444
                        as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    crate::src::common::pixel::x264_8_pixel_ssd_nv12(
                        &raw mut (*h).pixf as *mut _
                            as *mut crate::src::common::pixel::x264_pixel_function_t,
                        (*(*h).fdec).plane[1 as ::core::ffi::c_int as usize].offset(
                            ((minpix_y >> v_shift)
                                * (*(*h).fdec).i_stride[1 as ::core::ffi::c_int as usize])
                                as isize,
                        ),
                        (*(*h).fdec).i_stride[1 as ::core::ffi::c_int as usize]
                            as crate::stdlib::intptr_t,
                        (*(*h).fenc).plane[1 as ::core::ffi::c_int as usize].offset(
                            ((minpix_y >> v_shift)
                                * (*(*h).fenc).i_stride[1 as ::core::ffi::c_int as usize])
                                as isize,
                        ),
                        (*(*h).fenc).i_stride[1 as ::core::ffi::c_int as usize]
                            as crate::stdlib::intptr_t,
                        (*h).param.i_width >> 1 as ::core::ffi::c_int,
                        maxpix_y - minpix_y >> v_shift,
                        &raw mut ssd_u,
                        &raw mut ssd_v,
                    );
                    (*h).stat.frame.i_ssd[1 as ::core::ffi::c_int as usize] =
                        ((*h).stat.frame.i_ssd[1 as ::core::ffi::c_int as usize]
                            as crate::stdlib::uint64_t)
                            .wrapping_add(ssd_u) as crate::stdlib::int64_t
                            as crate::stdlib::int64_t;
                    (*h).stat.frame.i_ssd[2 as ::core::ffi::c_int as usize] =
                        ((*h).stat.frame.i_ssd[2 as ::core::ffi::c_int as usize]
                            as crate::stdlib::uint64_t)
                            .wrapping_add(ssd_v) as crate::stdlib::int64_t
                            as crate::stdlib::int64_t;
                }
            }
            if (*h).param.analyse.b_ssim != 0 {
                let mut ssim_cnt: ::core::ffi::c_int = 0;
                minpix_y += if b_start != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    -(6 as ::core::ffi::c_int)
                };
                (*h).stat.frame.f_ssim += crate::src::common::pixel::x264_8_pixel_ssim_wxh(
                    &raw mut (*h).pixf as *mut _
                        as *mut crate::src::common::pixel::x264_pixel_function_t,
                    (*(*h).fdec).plane[0 as ::core::ffi::c_int as usize]
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(
                            (minpix_y * (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize])
                                as isize,
                        ),
                    (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize]
                        as crate::stdlib::intptr_t,
                    (*(*h).fenc).plane[0 as ::core::ffi::c_int as usize]
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(
                            (minpix_y * (*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize])
                                as isize,
                        ),
                    (*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize]
                        as crate::stdlib::intptr_t,
                    (*h).param.i_width - 2 as ::core::ffi::c_int,
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

unsafe extern "C" fn reference_update(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*(*h).fdec).b_kept_as_ref == 0 {
            if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
                crate::src::common::frame::x264_8_frame_push_unused(
                    h as *mut crate::src::common::common::x264_t,
                    (*h).fdec as *mut crate::src::common::frame::x264_frame,
                );
                (*h).fdec = crate::src::common::frame::x264_8_frame_pop_unused(
                    h as *mut crate::src::common::common::x264_t,
                    1 as ::core::ffi::c_int,
                ) as *mut crate::src::common::frame::x264_frame;
                if (*h).fdec.is_null() {
                    return -(1 as ::core::ffi::c_int);
                }
            }
            return 0 as ::core::ffi::c_int;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).sh.i_mmco_command_count {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !(*h).frames.reference[j as usize].is_null() {
                if (*(*h).frames.reference[j as usize]).i_poc == (*h).sh.mmco[i as usize].i_poc {
                    crate::src::common::frame::x264_8_frame_push_unused(
                        h as *mut crate::src::common::common::x264_t,
                        crate::src::common::frame::x264_8_frame_shift(
                            (&raw mut (*h).frames.reference
                                as *mut *mut crate::src::common::frame::x264_frame_t)
                                .offset(j as isize)
                                as *mut *mut crate::src::common::frame::x264_frame_t
                                as *mut *mut crate::src::common::frame::x264_frame,
                        ) as *mut crate::src::common::frame::x264_frame
                            as *mut crate::src::common::frame::x264_frame,
                    );
                }
                j += 1;
            }
            i += 1;
        }
        crate::src::common::frame::x264_8_frame_push(
            &raw mut (*h).frames.reference as *mut *mut crate::src::common::frame::x264_frame_t
                as *mut *mut crate::src::common::frame::x264_frame,
            (*h).fdec as *mut crate::src::common::frame::x264_frame,
        );
        if !(*h).frames.reference[(*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
            .i_num_ref_frames as usize]
            .is_null()
        {
            crate::src::common::frame::x264_8_frame_push_unused(
                h as *mut crate::src::common::common::x264_t,
                crate::src::common::frame::x264_8_frame_shift(
                    &raw mut (*h).frames.reference
                        as *mut *mut crate::src::common::frame::x264_frame_t
                        as *mut *mut crate::src::common::frame::x264_frame,
                ) as *mut crate::src::common::frame::x264_frame
                    as *mut crate::src::common::frame::x264_frame,
            );
        }
        (*h).fdec = crate::src::common::frame::x264_8_frame_pop_unused(
            h as *mut crate::src::common::common::x264_t,
            1 as ::core::ffi::c_int,
        ) as *mut crate::src::common::frame::x264_frame;
        if (*h).fdec.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]

unsafe extern "C" fn reference_reset(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        while !(*h).frames.reference[0 as ::core::ffi::c_int as usize].is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(
                h as *mut crate::src::common::common::x264_t,
                crate::src::common::frame::x264_8_frame_pop(
                    &raw mut (*h).frames.reference
                        as *mut *mut crate::src::common::frame::x264_frame_t
                        as *mut *mut crate::src::common::frame::x264_frame,
                ) as *mut crate::src::common::frame::x264_frame
                    as *mut crate::src::common::frame::x264_frame,
            );
        }
        (*(*h).fenc).i_poc = 0 as ::core::ffi::c_int;
        (*(*h).fdec).i_poc = (*(*h).fenc).i_poc;
    }
}
#[inline]

unsafe extern "C" fn reference_hierarchy_reset(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut ref_0: ::core::ffi::c_int = 0;
        let mut b_hasdelayframe: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*(*h).frames.current.offset(i as isize)).is_null()
            && (**(*h).frames.current.offset(i as isize)).i_type == crate::x264_h::X264_TYPE_B
        {
            b_hasdelayframe |= ((**(*h).frames.current.offset(i as isize)).i_coded
                != (**(*h).frames.current.offset(i as isize)).i_frame
                    + (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                        .vui
                        .i_num_reorder_frames) as ::core::ffi::c_int;
            i += 1;
        }
        if (*h).param.i_bframe_pyramid != crate::x264_h::X264_B_PYRAMID_STRICT
            && b_hasdelayframe == 0
            && (*h).frames.i_poc_last_open_gop == -(1 as ::core::ffi::c_int)
        {
            return;
        }
        ref_0 = 0 as ::core::ffi::c_int;
        while !(*h).frames.reference[ref_0 as usize].is_null() {
            if (*h).param.i_bframe_pyramid == crate::x264_h::X264_B_PYRAMID_STRICT
                && (*(*h).frames.reference[ref_0 as usize]).i_type == crate::x264_h::X264_TYPE_BREF
                || (*(*h).frames.reference[ref_0 as usize]).i_poc < (*h).frames.i_poc_last_open_gop
                    && (*h).sh.i_type
                        != crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            {
                let mut diff: ::core::ffi::c_int =
                    (*h).i_frame_num - (*(*h).frames.reference[ref_0 as usize]).i_frame_num;
                (*h).sh.mmco[(*h).sh.i_mmco_command_count as usize].i_difference_of_pic_nums = diff;
                let c2rust_fresh7 = (*h).sh.i_mmco_command_count;
                (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_command_count + 1;
                (*h).sh.mmco[c2rust_fresh7 as usize].i_poc =
                    (*(*h).frames.reference[ref_0 as usize]).i_poc;
                crate::src::common::frame::x264_8_frame_push_unused(
                    h as *mut crate::src::common::common::x264_t,
                    crate::src::common::frame::x264_8_frame_shift(
                        (&raw mut (*h).frames.reference
                            as *mut *mut crate::src::common::frame::x264_frame_t)
                            .offset(ref_0 as isize)
                            as *mut *mut crate::src::common::frame::x264_frame_t
                            as *mut *mut crate::src::common::frame::x264_frame,
                    ) as *mut crate::src::common::frame::x264_frame
                        as *mut crate::src::common::frame::x264_frame,
                );
                (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
                ref_0 -= 1;
            }
            ref_0 += 1;
        }
        if (*h).param.i_bframe_pyramid != 0 {
            (*h).sh.i_mmco_remove_from_end = if ref_0 + 2 as ::core::ffi::c_int
                - (*h).frames.i_max_dpb
                > 0 as ::core::ffi::c_int
            {
                ref_0 + 2 as ::core::ffi::c_int - (*h).frames.i_max_dpb
            } else {
                0 as ::core::ffi::c_int
            };
        }
    }
}
#[inline]

unsafe extern "C" fn slice_init(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_nal_type: ::core::ffi::c_int,
    mut i_global_qp: ::core::ffi::c_int,
) {
    unsafe {
        if i_nal_type == crate::x264_h::NAL_SLICE_IDR as ::core::ffi::c_int {
            slice_header_init(
                h,
                &raw mut (*h).sh,
                &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t,
                &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t,
                (*h).i_idr_pic_id,
                (*h).i_frame_num,
                i_global_qp,
            );
            if (*h).param.i_avcintra_class != 0 {
                match (*h).i_idr_pic_id {
                    5 => {
                        (*h).i_idr_pic_id = 3 as ::core::ffi::c_int;
                    }
                    3 => {
                        (*h).i_idr_pic_id = 4 as ::core::ffi::c_int;
                    }
                    4 | _ => {
                        (*h).i_idr_pic_id = 5 as ::core::ffi::c_int;
                    }
                }
            } else {
                (*h).i_idr_pic_id ^= 1 as ::core::ffi::c_int;
            }
        } else {
            slice_header_init(
                h,
                &raw mut (*h).sh,
                &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t,
                &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t,
                -(1 as ::core::ffi::c_int),
                (*h).i_frame_num,
                i_global_qp,
            );
            (*h).sh.i_num_ref_idx_l0_active =
                if (*h).i_ref[0 as ::core::ffi::c_int as usize] <= 0 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    (*h).i_ref[0 as ::core::ffi::c_int as usize]
                };
            (*h).sh.i_num_ref_idx_l1_active =
                if (*h).i_ref[1 as ::core::ffi::c_int as usize] <= 0 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    (*h).i_ref[1 as ::core::ffi::c_int as usize]
                };
            if (*h).sh.i_num_ref_idx_l0_active
                != (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_num_ref_idx_l0_default_active
                || (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
                    && (*h).sh.i_num_ref_idx_l1_active
                        != (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                            .i_num_ref_idx_l1_default_active
            {
                (*h).sh.b_num_ref_idx_override = 1 as ::core::ffi::c_int;
            }
        }
        if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_BREF
            && (*h).param.b_bluray_compat != 0
            && (*h).sh.i_mmco_command_count != 0
        {
            (*h).b_sh_backup = 1 as ::core::ffi::c_int;
            (*h).sh_backup = (*h).sh;
        }
        (*(*h).fdec).i_frame_num = (*h).sh.i_frame_num;
        if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_poc_type
            == 0 as ::core::ffi::c_int
        {
            (*h).sh.i_poc = (*(*h).fdec).i_poc;
            if (*h).param.b_interlaced != 0 {
                (*h).sh.i_delta_poc_bottom = if (*h).param.b_tff != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    -(1 as ::core::ffi::c_int)
                };
                (*h).sh.i_poc += ((*h).sh.i_delta_poc_bottom == -(1 as ::core::ffi::c_int))
                    as ::core::ffi::c_int;
            } else {
                (*h).sh.i_delta_poc_bottom = 0 as ::core::ffi::c_int;
            }
            (*(*h).fdec).i_delta_poc[0 as ::core::ffi::c_int as usize] =
                ((*h).sh.i_delta_poc_bottom == -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int;
            (*(*h).fdec).i_delta_poc[1 as ::core::ffi::c_int as usize] =
                ((*h).sh.i_delta_poc_bottom == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        crate::src::common::macroblock::x264_8_macroblock_slice_init(
            h as *mut crate::src::common::common::x264_t,
        );
    }
}
#[inline(always)]

unsafe extern "C" fn bitstream_backup(
    mut h: *mut crate::src::common::common::x264_t,
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
        if (*h).param.b_cabac != 0 {
            if full != 0 {
                crate::stdlib::memcpy(
                    &raw mut (*bak).cabac as *mut ::core::ffi::c_void,
                    &raw mut (*h).cabac as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<crate::src::common::cabac::x264_cabac_t>()
                        as crate::__stddef_size_t_h::size_t,
                );
            } else {
                crate::stdlib::memcpy(
                    &raw mut (*bak).cabac as *mut ::core::ffi::c_void,
                    &raw mut (*h).cabac as *const ::core::ffi::c_void,
                    64 as crate::__stddef_size_t_h::size_t,
                );
            }
            (*bak).cabac_prevbyte = *(*h).cabac.p.offset(-(1 as ::core::ffi::c_int) as isize);
        } else {
            (*bak).bs = (*h).out.bs;
            (*bak).skip = i_skip;
        };
    }
}
#[inline(always)]

unsafe extern "C" fn bitstream_restore(
    mut h: *mut crate::src::common::common::x264_t,
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
        if (*h).param.b_cabac != 0 {
            if full != 0 {
                crate::stdlib::memcpy(
                    &raw mut (*h).cabac as *mut ::core::ffi::c_void,
                    &raw mut (*bak).cabac as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<crate::src::common::cabac::x264_cabac_t>()
                        as crate::__stddef_size_t_h::size_t,
                );
            } else {
                crate::stdlib::memcpy(
                    &raw mut (*h).cabac as *mut ::core::ffi::c_void,
                    &raw mut (*bak).cabac as *const ::core::ffi::c_void,
                    64 as crate::__stddef_size_t_h::size_t,
                );
            }
            *(*h).cabac.p.offset(-(1 as ::core::ffi::c_int) as isize) = (*bak).cabac_prevbyte;
        } else {
            (*h).out.bs = (*bak).bs;
            *skip = (*bak).skip;
        };
    }
}

unsafe extern "C" fn slice_write(
    mut h: *mut crate::src::common::common::x264_t,
) -> crate::stdlib::intptr_t {
    unsafe {
        let mut i_skip: ::core::ffi::c_int = 0;
        let mut mb_xy: ::core::ffi::c_int = 0;
        let mut i_mb_x: ::core::ffi::c_int = 0;
        let mut i_mb_y: ::core::ffi::c_int = 0;
        let mut overhead_guess: ::core::ffi::c_int = crate::src::common::common::NALU_OVERHEAD
            - ((*h).param.b_annexb != 0 && (*h).out.i_nal != 0) as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + (*h).param.b_cabac
            + 5 as ::core::ffi::c_int;
        let mut slice_max_size: ::core::ffi::c_int =
            if (*h).param.i_slice_max_size > 0 as ::core::ffi::c_int {
                ((*h).param.i_slice_max_size - overhead_guess) * 8 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        let mut back_up_bitstream_cavlc: ::core::ffi::c_int = ((*h).param.b_cabac == 0
            && (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).i_profile_idc
                < crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut back_up_bitstream: ::core::ffi::c_int =
            (slice_max_size != 0 || back_up_bitstream_cavlc != 0) as ::core::ffi::c_int;
        let mut starting_bits: ::core::ffi::c_int = bs_pos(&raw mut (*h).out.bs);
        let mut b_deblock: ::core::ffi::c_int = ((*h).sh.i_disable_deblocking_filter_idc
            != 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut b_hpel: ::core::ffi::c_int = (*(*h).fdec).b_kept_as_ref;
        let mut orig_last_mb: ::core::ffi::c_int = (*h).sh.i_last_mb;
        let mut thread_last_mb: ::core::ffi::c_int =
            (*h).i_threadslice_end * (*h).mb.i_mb_width - 1 as ::core::ffi::c_int;
        let mut last_emu_check: *mut crate::stdlib::uint8_t =
            ::core::ptr::null_mut::<crate::stdlib::uint8_t>();
        let mut bs_bak: [x264_bs_bak_t; 4] = [x264_bs_bak_t {
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
        b_deblock &= (b_hpel != 0
            || (*h).param.b_full_recon != 0
            || !(*h).param.psz_dump_yuv.is_null()) as ::core::ffi::c_int;
        bs_realign(&raw mut (*h).out.bs);
        nal_start(h, (*h).i_nal_type, (*h).i_nal_ref_idc);
        (*(*h).out.nal.offset((*h).out.i_nal as isize)).i_first_mb = (*h).sh.i_first_mb;
        crate::src::common::macroblock::x264_8_macroblock_thread_init(
            h as *mut crate::src::common::common::x264_t,
        );
        (*h).mb.i_mb_xy = (*h).sh.i_first_mb;
        (*h).sh.i_qp = crate::src::encoder::ratecontrol::x264_8_ratecontrol_mb_qp(
            h as *mut crate::src::common::common::x264_t,
        );
        (*h).sh.i_qp = if (*h).sh.i_qp
            < 51 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
        {
            (*h).sh.i_qp
        } else {
            51 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
        };
        (*h).sh.i_qp_delta = (*h).sh.i_qp
            - (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).i_pic_init_qp;
        slice_header_write(&raw mut (*h).out.bs, &raw mut (*h).sh, (*h).i_nal_ref_idc);
        if (*h).param.b_cabac != 0 {
            bs_align_1(&raw mut (*h).out.bs);
            crate::src::common::cabac::x264_8_cabac_context_init(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).cabac as *mut _ as *mut crate::src::common::cabac::x264_cabac_t,
                (*h).sh.i_type,
                x264_clip3(
                    (*h).sh.i_qp - crate::src::common::common::QP_BD_OFFSET,
                    0 as ::core::ffi::c_int,
                    51 as ::core::ffi::c_int,
                ),
                (*h).sh.i_cabac_init_idc,
            );
            crate::src::common::cabac::x264_8_cabac_encode_init(
                &raw mut (*h).cabac as *mut _ as *mut crate::src::common::cabac::x264_cabac_t,
                (*h).out.bs.p,
                (*h).out.bs.p_end,
            );
            last_emu_check = (*h).cabac.p;
        } else {
            last_emu_check = (*h).out.bs.p;
        }
        (*h).mb.i_last_qp = (*h).sh.i_qp;
        (*h).mb.i_last_dqp = 0 as ::core::ffi::c_int;
        (*h).mb.field_decoding_flag = 0 as ::core::ffi::c_int;
        i_mb_y = (*h).sh.i_first_mb / (*h).mb.i_mb_width;
        i_mb_x = (*h).sh.i_first_mb % (*h).mb.i_mb_width;
        i_skip = 0 as ::core::ffi::c_int;
        loop {
            mb_xy = i_mb_x + i_mb_y * (*h).mb.i_mb_width;
            let mut mb_spos: ::core::ffi::c_int =
                bs_pos(&raw mut (*h).out.bs) + x264_cabac_pos(&raw mut (*h).cabac);
            if i_mb_x == 0 as ::core::ffi::c_int {
                if bitstream_check_buffer(h) != 0 {
                    return -(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t;
                }
                if i_mb_y & (*h).sh.b_mbaff == 0 && (*h).param.rc.i_vbv_buffer_size != 0 {
                    bitstream_backup(
                        h,
                        (&raw mut bs_bak as *mut x264_bs_bak_t).offset(BS_BAK_ROW_VBV as isize)
                            as *mut x264_bs_bak_t,
                        i_skip,
                        1 as ::core::ffi::c_int,
                    );
                }
                if (*h).mb.b_reencode_mb == 0 {
                    fdec_filter_row(h, i_mb_y, 0 as ::core::ffi::c_int);
                }
            }
            if back_up_bitstream != 0 {
                if back_up_bitstream_cavlc != 0 {
                    bitstream_backup(
                        h,
                        (&raw mut bs_bak as *mut x264_bs_bak_t)
                            .offset(BS_BAK_CAVLC_OVERFLOW as isize)
                            as *mut x264_bs_bak_t,
                        i_skip,
                        0 as ::core::ffi::c_int,
                    );
                }
                if slice_max_size != 0 && i_mb_y & (*h).sh.b_mbaff == 0 {
                    bitstream_backup(
                        h,
                        (&raw mut bs_bak as *mut x264_bs_bak_t)
                            .offset(BS_BAK_SLICE_MAX_SIZE as isize)
                            as *mut x264_bs_bak_t,
                        i_skip,
                        0 as ::core::ffi::c_int,
                    );
                    if thread_last_mb + 1 as ::core::ffi::c_int - mb_xy
                        == (*h).param.i_slice_min_mbs
                    {
                        bitstream_backup(
                            h,
                            (&raw mut bs_bak as *mut x264_bs_bak_t)
                                .offset(BS_BAK_SLICE_MIN_MBS as isize)
                                as *mut x264_bs_bak_t,
                            i_skip,
                            0 as ::core::ffi::c_int,
                        );
                    }
                }
            }
            if (*h).param.b_interlaced != 0 {
                if (*h).mb.b_adaptive_mbaff != 0 {
                    if i_mb_y & 1 as ::core::ffi::c_int == 0 {
                        (*h).mb.b_interlaced = crate::src::common::pixel::x264_8_field_vsad(
                            h as *mut crate::src::common::common::x264_t,
                            i_mb_x,
                            i_mb_y,
                        );
                        crate::stdlib::memcpy(
                            &raw mut (*h).zigzagf as *mut ::core::ffi::c_void,
                            (if (*h).mb.b_interlaced != 0 {
                                &raw mut (*h).zigzagf_interlaced
                            } else {
                                &raw mut (*h).zigzagf_progressive
                            }) as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<crate::src::common::dct::x264_zigzag_function_t>(
                            ) as crate::__stddef_size_t_h::size_t,
                        );
                        if (*h).mb.b_interlaced == 0
                            && i_mb_y + 2 as ::core::ffi::c_int == (*h).mb.i_mb_height
                        {
                            crate::src::common::frame::x264_8_expand_border_mbpair(
                                h as *mut crate::src::common::common::x264_t,
                                i_mb_x,
                                i_mb_y,
                            );
                        }
                    }
                }
                *(*h).mb.field.offset(mb_xy as isize) =
                    (*h).mb.b_interlaced as crate::stdlib::uint8_t;
            }
            if (*h).sh.b_mbaff != 0 {
                crate::src::common::macroblock::x264_8_macroblock_cache_load_interlaced(
                    h as *mut crate::src::common::common::x264_t,
                    i_mb_x,
                    i_mb_y,
                );
            } else {
                x264_8_macroblock_cache_load_progressive(h, i_mb_x, i_mb_y);
            }
            crate::src::encoder::analyse::x264_8_macroblock_analyse(
                h as *mut crate::src::common::common::x264_t,
            );
            loop {
                crate::src::encoder::macroblock::x264_8_macroblock_encode(
                    h as *mut crate::src::common::common::x264_t,
                );
                if (*h).param.b_cabac != 0 {
                    if mb_xy > (*h).sh.i_first_mb
                        && !((*h).sh.b_mbaff != 0 && i_mb_y & 1 as ::core::ffi::c_int != 0)
                    {
                        crate::src::common::cabac::x264_8_cabac_encode_terminal_c(
                            &raw mut (*h).cabac as *mut _
                                as *mut crate::src::common::cabac::x264_cabac_t,
                        );
                    }
                    if (*h).mb.i_type
                        == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                        || (*h).mb.i_type
                            == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
                    {
                        crate::src::encoder::cabac::x264_8_cabac_mb_skip(
                            h as *mut crate::src::common::common::x264_t,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        if (*h).sh.i_type
                            != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                        {
                            crate::src::encoder::cabac::x264_8_cabac_mb_skip(
                                h as *mut crate::src::common::common::x264_t,
                                0 as ::core::ffi::c_int,
                            );
                        }
                        crate::src::encoder::cabac::x264_8_macroblock_write_cabac(
                            h as *mut crate::src::common::common::x264_t,
                            &raw mut (*h).cabac as *mut _
                                as *mut crate::src::common::cabac::x264_cabac_t,
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
                        i_skip = 0 as ::core::ffi::c_int;
                    }
                    crate::src::encoder::cavlc::x264_8_macroblock_write_cavlc(
                        h as *mut crate::src::common::common::x264_t,
                    );
                    if !((*h).mb.b_overflow != 0) {
                        break;
                    }
                    (*h).mb.i_qp += 1;
                    (*h).mb.i_chroma_qp =
                        *(*h).chroma_qp_table.offset((*h).mb.i_qp as isize) as ::core::ffi::c_int;
                    (*h).mb.i_skip_intra = 0 as ::core::ffi::c_int;
                    (*h).mb.b_skip_mc = 0 as ::core::ffi::c_int;
                    (*h).mb.b_overflow = 0 as ::core::ffi::c_int;
                    bitstream_restore(
                        h,
                        (&raw mut bs_bak as *mut x264_bs_bak_t)
                            .offset(BS_BAK_CAVLC_OVERFLOW as isize)
                            as *mut x264_bs_bak_t,
                        &raw mut i_skip,
                        0 as ::core::ffi::c_int,
                    );
                }
            }
            let mut total_bits: ::core::ffi::c_int =
                bs_pos(&raw mut (*h).out.bs) + x264_cabac_pos(&raw mut (*h).cabac);
            let mut mb_size: ::core::ffi::c_int = total_bits - mb_spos;
            if slice_max_size != 0
                && ((*h).sh.b_mbaff == 0 || i_mb_y & 1 as ::core::ffi::c_int != 0)
            {
                if (*h).param.b_cabac == 0 {
                    total_bits += bs_size_ue_big(i_skip as ::core::ffi::c_uint);
                }
                let mut end: *mut crate::stdlib::uint8_t = if (*h).param.b_cabac != 0 {
                    (*h).cabac.p
                } else {
                    (*h).out.bs.p
                };
                while last_emu_check < end.offset(-(2 as ::core::ffi::c_int as isize)) {
                    if *last_emu_check.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && *last_emu_check.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        && *last_emu_check.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            <= 3 as ::core::ffi::c_int
                    {
                        slice_max_size -= 8 as ::core::ffi::c_int;
                        last_emu_check = last_emu_check.offset(1);
                    }
                    last_emu_check = last_emu_check.offset(1);
                }
                if total_bits - starting_bits > slice_max_size && (*h).mb.b_reencode_mb == 0 {
                    if crate::src::common::frame::x264_8_frame_new_slice(
                        h as *mut crate::src::common::common::x264_t,
                        (*h).fdec as *mut crate::src::common::frame::x264_frame,
                    ) == 0
                    {
                        if mb_xy <= thread_last_mb
                            && thread_last_mb + 1 as ::core::ffi::c_int - mb_xy
                                < (*h).param.i_slice_min_mbs
                        {
                            if thread_last_mb - (*h).param.i_slice_min_mbs
                                < (*h).sh.i_first_mb + (*h).param.i_slice_min_mbs
                            {
                                crate::src::common::common::x264_8_log(
                                    h as *mut crate::src::common::common::x264_t,
                                    crate::x264_h::X264_LOG_WARNING_1,
                                    b"slice-max-size violated (frame %d, cause: slice-min-mbs)\n\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*h).i_frame,
                                );
                                slice_max_size = 0 as ::core::ffi::c_int;
                            } else {
                                bitstream_restore(
                                    h,
                                    (&raw mut bs_bak as *mut x264_bs_bak_t)
                                        .offset(BS_BAK_SLICE_MIN_MBS as isize)
                                        as *mut x264_bs_bak_t,
                                    &raw mut i_skip,
                                    0 as ::core::ffi::c_int,
                                );
                                (*h).mb.b_reencode_mb = 1 as ::core::ffi::c_int;
                                (*h).sh.i_last_mb = thread_last_mb - (*h).param.i_slice_min_mbs;
                                break;
                            }
                        } else if mb_xy - (*h).sh.b_mbaff * (*h).mb.i_mb_stride
                            != (*h).sh.i_first_mb
                        {
                            bitstream_restore(
                                h,
                                (&raw mut bs_bak as *mut x264_bs_bak_t)
                                    .offset(BS_BAK_SLICE_MAX_SIZE as isize)
                                    as *mut x264_bs_bak_t,
                                &raw mut i_skip,
                                0 as ::core::ffi::c_int,
                            );
                            (*h).mb.b_reencode_mb = 1 as ::core::ffi::c_int;
                            if (*h).sh.b_mbaff != 0 {
                                if i_mb_x != 0 {
                                    (*h).sh.i_last_mb = mb_xy - 1 as ::core::ffi::c_int
                                        + (*h).mb.i_mb_stride
                                            * (i_mb_y & 1 as ::core::ffi::c_int == 0)
                                                as ::core::ffi::c_int;
                                } else {
                                    (*h).sh.i_last_mb = (i_mb_y - 2 as ::core::ffi::c_int
                                        + (i_mb_y & 1 as ::core::ffi::c_int == 0)
                                            as ::core::ffi::c_int)
                                        * (*h).mb.i_mb_stride
                                        + (*h).mb.i_mb_width
                                        - 1 as ::core::ffi::c_int;
                                }
                            } else {
                                (*h).sh.i_last_mb = mb_xy - 1 as ::core::ffi::c_int;
                            }
                            break;
                        } else {
                            (*h).sh.i_last_mb = mb_xy;
                        }
                    } else {
                        slice_max_size = 0 as ::core::ffi::c_int;
                    }
                }
            }
            (*h).mb.b_reencode_mb = 0 as ::core::ffi::c_int;
            crate::src::common::macroblock::x264_8_macroblock_cache_save(
                h as *mut crate::src::common::common::x264_t,
            );
            if crate::src::encoder::ratecontrol::x264_8_ratecontrol_mb(
                h as *mut crate::src::common::common::x264_t,
                mb_size,
            ) < 0 as ::core::ffi::c_int
            {
                bitstream_restore(
                    h,
                    (&raw mut bs_bak as *mut x264_bs_bak_t).offset(BS_BAK_ROW_VBV as isize)
                        as *mut x264_bs_bak_t,
                    &raw mut i_skip,
                    1 as ::core::ffi::c_int,
                );
                (*h).mb.b_reencode_mb = 1 as ::core::ffi::c_int;
                i_mb_x = 0 as ::core::ffi::c_int;
                i_mb_y = i_mb_y - (*h).sh.b_mbaff;
                (*h).mb.i_mb_prev_xy = i_mb_y * (*h).mb.i_mb_stride - 1 as ::core::ffi::c_int;
                (*h).sh.i_last_mb = orig_last_mb;
            } else {
                (*h).stat.frame.i_mb_count[(*h).mb.i_type as usize] += 1;
                let mut b_intra: ::core::ffi::c_int = ((*h).mb.i_type
                    == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                let mut b_skip: ::core::ffi::c_int = ((*h).mb.i_type
                    == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                    || (*h).mb.i_type
                        == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                if (*h).param.i_log_level >= crate::x264_h::X264_LOG_INFO
                    || (*h).param.rc.b_stat_write != 0
                {
                    if b_intra == 0
                        && b_skip == 0
                        && !((*h).mb.i_type
                            == crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int)
                    {
                        if (*h).mb.i_partition
                            != crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int
                        {
                            (*h).stat.frame.i_mb_partition[(*h).mb.i_partition as usize] +=
                                4 as ::core::ffi::c_int;
                        } else {
                            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i < 4 as ::core::ffi::c_int {
                                (*h).stat.frame.i_mb_partition
                                    [(*h).mb.i_sub_partition[i as usize] as usize] += 1;
                                i += 1;
                            }
                        }
                        if (*h).param.i_frame_reference > 1 as ::core::ffi::c_int {
                            let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_list
                                <= ((*h).sh.i_type
                                    == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                            {
                                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while i_0 < 4 as ::core::ffi::c_int {
                                    let mut i_ref: ::core::ffi::c_int = (*h).mb.cache.ref_0
                                        [i_list as usize]
                                        [x264_scan8[(4 as ::core::ffi::c_int * i_0) as usize]
                                            as usize]
                                        as ::core::ffi::c_int;
                                    if i_ref >= 0 as ::core::ffi::c_int {
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
                        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        {
                            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_1 < 4 as ::core::ffi::c_int {
                                if (*h).mb.i_cbp_luma & (1 as ::core::ffi::c_int) << i_1 != 0 {
                                    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    while p < 3 as ::core::ffi::c_int {
                                        let mut s8: ::core::ffi::c_int = i_1
                                            * 4 as ::core::ffi::c_int
                                            + p * 16 as ::core::ffi::c_int;
                                        let mut nnz8x8: ::core::ffi::c_int = (*((&raw mut (*h)
                                            .mb
                                            .cache
                                            .non_zero_count
                                            as *mut crate::stdlib::uint8_t)
                                            .offset(
                                                (*(&raw const x264_scan8
                                                    as *const crate::stdlib::uint8_t)
                                                    .offset(s8 as isize)
                                                    as ::core::ffi::c_int
                                                    + 0 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            as *mut crate::stdlib::uint8_t
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
                                                        + 8 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                as *mut crate::stdlib::uint8_t
                                                as *mut crate::src::common::base::x264_union16_t))
                                                .i
                                                as ::core::ffi::c_int;
                                        (*h).stat.frame.i_mb_cbp[((b_intra == 0)
                                            as ::core::ffi::c_int
                                            + p * 2 as ::core::ffi::c_int)
                                            as usize] += (nnz8x8 != 0) as ::core::ffi::c_int;
                                        p += 1;
                                    }
                                }
                                i_1 += 1;
                            }
                        } else {
                            let mut cbpsum: ::core::ffi::c_int = ((*h).mb.i_cbp_luma
                                & 1 as ::core::ffi::c_int)
                                + ((*h).mb.i_cbp_luma >> 1 as ::core::ffi::c_int
                                    & 1 as ::core::ffi::c_int)
                                + ((*h).mb.i_cbp_luma >> 2 as ::core::ffi::c_int
                                    & 1 as ::core::ffi::c_int)
                                + ((*h).mb.i_cbp_luma >> 3 as ::core::ffi::c_int);
                            (*h).stat.frame.i_mb_cbp[((b_intra == 0) as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int)
                                as usize] += cbpsum;
                            (*h).stat.frame.i_mb_cbp[((b_intra == 0) as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int)
                                as usize] += ((*h).mb.i_cbp_chroma != 0) as ::core::ffi::c_int;
                            (*h).stat.frame.i_mb_cbp[((b_intra == 0) as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int)
                                as usize] += (*h).mb.i_cbp_chroma >> 1 as ::core::ffi::c_int;
                        }
                    }
                    if (*h).mb.i_cbp_luma != 0 && b_intra == 0 {
                        (*h).stat.frame.i_mb_count_8x8dct[0 as ::core::ffi::c_int as usize] += 1;
                        (*h).stat.frame.i_mb_count_8x8dct[1 as ::core::ffi::c_int as usize] +=
                            (*h).mb.b_transform_8x8;
                    }
                    if b_intra != 0
                        && (*h).mb.i_type
                            != crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
                    {
                        if (*h).mb.i_type
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        {
                            (*h).stat.frame.i_mb_pred_mode[0 as ::core::ffi::c_int as usize]
                                [(*h).mb.i_intra16x16_pred_mode as usize] += 1;
                        } else if (*h).mb.i_type
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        {
                            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_2 < 16 as ::core::ffi::c_int {
                                (*h).stat.frame.i_mb_pred_mode[1 as ::core::ffi::c_int as usize]
                                    [(*h).mb.cache.intra4x4_pred_mode
                                        [x264_scan8[i_2 as usize] as usize]
                                        as usize] += 1;
                                i_2 += 4 as ::core::ffi::c_int;
                            }
                        } else {
                            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_3 < 16 as ::core::ffi::c_int {
                                (*h).stat.frame.i_mb_pred_mode[2 as ::core::ffi::c_int as usize]
                                    [(*h).mb.cache.intra4x4_pred_mode
                                        [x264_scan8[i_3 as usize] as usize]
                                        as usize] += 1;
                                i_3 += 1;
                            }
                        }
                        (*h).stat.frame.i_mb_pred_mode[3 as ::core::ffi::c_int as usize]
                            [x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize]
                                as usize] += 1;
                    }
                    (*h).stat.frame.i_mb_field[(if b_intra != 0 {
                        0 as ::core::ffi::c_int
                    } else if b_skip != 0 {
                        2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as usize] += (*h).mb.b_interlaced;
                }
                if b_deblock != 0 {
                    crate::src::common::macroblock::x264_8_macroblock_deblock_strength(
                        h as *mut crate::src::common::common::x264_t,
                    );
                }
                if mb_xy == (*h).sh.i_last_mb {
                    break;
                }
                if (*h).sh.b_mbaff != 0 {
                    i_mb_x += i_mb_y & 1 as ::core::ffi::c_int;
                    i_mb_y ^= (i_mb_x < (*h).mb.i_mb_width) as ::core::ffi::c_int;
                } else {
                    i_mb_x += 1;
                }
                if i_mb_x == (*h).mb.i_mb_width {
                    i_mb_y += 1;
                    i_mb_x = 0 as ::core::ffi::c_int;
                }
            }
        }
        if (*h).sh.i_last_mb < (*h).sh.i_first_mb {
            return 0 as crate::stdlib::intptr_t;
        }
        (*(*h).out.nal.offset((*h).out.i_nal as isize)).i_last_mb = (*h).sh.i_last_mb;
        if (*h).param.b_cabac != 0 {
            crate::src::common::cabac::x264_8_cabac_encode_flush(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).cabac as *mut _ as *mut crate::src::common::cabac::x264_cabac_t,
            );
            (*h).out.bs.p = (*h).cabac.p;
        } else {
            if i_skip > 0 as ::core::ffi::c_int {
                bs_write_ue_big(&raw mut (*h).out.bs, i_skip as ::core::ffi::c_uint);
            }
            bs_rbsp_trailing(&raw mut (*h).out.bs);
            bs_flush(&raw mut (*h).out.bs);
        }
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t;
        }
        if (*h).sh.i_last_mb
            == (*h).i_threadslice_end * (*h).mb.i_mb_width - 1 as ::core::ffi::c_int
        {
            (*h).stat.frame.i_misc_bits = bs_pos(&raw mut (*h).out.bs)
                + (*h).out.i_nal
                    * crate::src::common::common::NALU_OVERHEAD
                    * 8 as ::core::ffi::c_int
                - (*h).stat.frame.i_tex_bits
                - (*h).stat.frame.i_mv_bits;
            fdec_filter_row(h, (*h).i_threadslice_end, 0 as ::core::ffi::c_int);
            if (*h).param.b_sliced_threads != 0 {
                crate::src::common::frame::x264_8_threadslice_cond_broadcast(
                    h as *mut crate::src::common::common::x264_t,
                    1 as ::core::ffi::c_int,
                );
                let mut mb_y: ::core::ffi::c_int = (*h).i_threadslice_start;
                while mb_y <= (*h).i_threadslice_end {
                    fdec_filter_row(h, mb_y, 1 as ::core::ffi::c_int);
                    mb_y += 1;
                }
                crate::src::common::frame::x264_8_threadslice_cond_broadcast(
                    h as *mut crate::src::common::common::x264_t,
                    2 as ::core::ffi::c_int,
                );
                if (*h).i_thread_idx > 0 as ::core::ffi::c_int {
                    crate::src::common::frame::x264_8_threadslice_cond_wait(
                        (*h).thread[((*h).i_thread_idx - 1 as ::core::ffi::c_int) as usize]
                            as *mut crate::src::common::common::x264_t,
                        2 as ::core::ffi::c_int,
                    );
                    fdec_filter_row(
                        h,
                        (*h).i_threadslice_start + ((1 as ::core::ffi::c_int) << (*h).sh.b_mbaff),
                        2 as ::core::ffi::c_int,
                    );
                }
            }
            if (*(*h).fdec).mb_info_free.is_some()
                && ((*h).param.b_sliced_threads == 0
                    || (*h).i_thread_idx == (*h).param.i_threads - 1 as ::core::ffi::c_int)
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
        return 0 as crate::stdlib::intptr_t;
    }
}

pub const BS_BAK_SLICE_MAX_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const BS_BAK_CAVLC_OVERFLOW: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const BS_BAK_SLICE_MIN_MBS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const BS_BAK_ROW_VBV: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

unsafe extern "C" fn thread_sync_context(
    mut dst: *mut crate::src::common::common::x264_t,
    mut src: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        if dst == src {
            return;
        }
        let mut f: *mut *mut crate::src::common::frame::x264_frame_t =
            &raw mut (*src).frames.reference
                as *mut *mut crate::src::common::frame::x264_frame_t;
        while !(*f).is_null() {
            (**f).i_reference_count += 1;
            f = f.offset(1);
        }
        let mut f_0: *mut *mut crate::src::common::frame::x264_frame_t =
            &raw mut (*dst).frames.reference
                as *mut *mut crate::src::common::frame::x264_frame_t;
        while !(*f_0).is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(
                src as *mut crate::src::common::common::x264_t,
                *f_0 as *mut crate::src::common::frame::x264_frame,
            );
            f_0 = f_0.offset(1);
        }
        (*(*src).fdec).i_reference_count += 1;
        crate::src::common::frame::x264_8_frame_push_unused(
            src as *mut crate::src::common::common::x264_t,
            (*dst).fdec as *mut crate::src::common::frame::x264_frame,
        );
        crate::stdlib::memcpy(
            &raw mut (*dst).i_frame as *mut ::core::ffi::c_void,
            &raw mut (*src).i_frame as *const ::core::ffi::c_void,
            (24912 as crate::__stddef_size_t_h::size_t)
                .wrapping_sub(2420 as crate::__stddef_size_t_h::size_t),
        );
        (*dst).param = (*src).param;
        (*dst).stat = (*src).stat;
        (*dst).pixf = (*src).pixf;
        (*dst).reconfig = (*src).reconfig;
    }
}

unsafe extern "C" fn thread_sync_stat(
    mut dst: *mut crate::src::common::common::x264_t,
    mut src: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        if dst != src {
            crate::stdlib::memcpy(
                &raw mut (*dst).stat as *mut ::core::ffi::c_void,
                &raw mut (*src).stat as *const ::core::ffi::c_void,
                (42848 as crate::__stddef_size_t_h::size_t)
                    .wrapping_sub(40264 as crate::__stddef_size_t_h::size_t),
            );
        }
    }
}

unsafe extern "C" fn slices_write(
    mut h: *mut crate::src::common::common::x264_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut i_slice_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut last_thread_mb: ::core::ffi::c_int = (*h).sh.i_last_mb;
        let mut round_bias: ::core::ffi::c_int = if (*h).param.i_avcintra_class != 0 {
            0 as ::core::ffi::c_int
        } else {
            (*h).param.i_slice_count / 2 as ::core::ffi::c_int
        };
        crate::stdlib::memset(
            &raw mut (*h).stat.frame as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::common::common::x264_frame_stat_t>()
                as crate::__stddef_size_t_h::size_t,
        );
        (*h).mb.b_reencode_mb = 0 as ::core::ffi::c_int;
        loop {
            if !((*h).sh.i_first_mb + (*h).sh.b_mbaff * (*h).mb.i_mb_stride <= last_thread_mb) {
                c2rust_current_block = 5634871135123216486;
                break;
            }
            (*h).sh.i_last_mb = last_thread_mb;
            if i_slice_num == 0
                || crate::src::common::frame::x264_8_frame_new_slice(
                    h as *mut crate::src::common::common::x264_t,
                    (*h).fdec as *mut crate::src::common::frame::x264_frame,
                ) == 0
            {
                if (*h).param.i_slice_max_mbs != 0 {
                    if (*h).sh.b_mbaff != 0 {
                        let mut last_mbaff: ::core::ffi::c_int = 2 as ::core::ffi::c_int
                            * ((*h).sh.i_first_mb % (*h).mb.i_mb_width)
                            + (*h).mb.i_mb_width * ((*h).sh.i_first_mb / (*h).mb.i_mb_width)
                            + (*h).param.i_slice_max_mbs
                            - 1 as ::core::ffi::c_int;
                        let mut last_x: ::core::ffi::c_int = last_mbaff
                            % (2 as ::core::ffi::c_int * (*h).mb.i_mb_width)
                            / 2 as ::core::ffi::c_int;
                        let mut last_y: ::core::ffi::c_int = last_mbaff
                            / (2 as ::core::ffi::c_int * (*h).mb.i_mb_width)
                            * 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int;
                        (*h).sh.i_last_mb = last_x + (*h).mb.i_mb_stride * last_y;
                    } else {
                        (*h).sh.i_last_mb = (*h).sh.i_first_mb + (*h).param.i_slice_max_mbs
                            - 1 as ::core::ffi::c_int;
                        if (*h).sh.i_last_mb < last_thread_mb
                            && last_thread_mb - (*h).sh.i_last_mb < (*h).param.i_slice_min_mbs
                        {
                            (*h).sh.i_last_mb = last_thread_mb - (*h).param.i_slice_min_mbs;
                        }
                    }
                    i_slice_num += 1;
                } else if (*h).param.i_slice_count != 0 && (*h).param.b_sliced_threads == 0 {
                    let mut height: ::core::ffi::c_int =
                        (*h).mb.i_mb_height >> (*h).param.b_interlaced;
                    let mut width: ::core::ffi::c_int =
                        (*h).mb.i_mb_width << (*h).param.b_interlaced;
                    i_slice_num += 1;
                    (*h).sh.i_last_mb =
                        (height * i_slice_num + round_bias) / (*h).param.i_slice_count * width
                            - 1 as ::core::ffi::c_int;
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
            (*h).sh.i_first_mb = (*h).sh.i_last_mb + 1 as ::core::ffi::c_int;
            if (*h).sh.b_mbaff != 0 && (*h).sh.i_first_mb % (*h).mb.i_mb_width != 0 {
                (*h).sh.i_first_mb -= (*h).mb.i_mb_stride;
            }
        }
        match c2rust_current_block {
            5634871135123216486 => return ::core::ptr::null_mut::<::core::ffi::c_void>(),
            _ => {
                if (*h).param.b_sliced_threads != 0 {
                    crate::src::common::frame::x264_8_threadslice_cond_broadcast(
                        h as *mut crate::src::common::common::x264_t,
                        2 as ::core::ffi::c_int,
                    );
                }
                return -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
            }
        };
    }
}

unsafe extern "C" fn threaded_slices_write(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut round_bias: ::core::ffi::c_int = if (*h).param.i_avcintra_class != 0 {
            0 as ::core::ffi::c_int
        } else {
            (*h).param.i_slice_count / 2 as ::core::ffi::c_int
        };
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).param.i_threads {
            let mut t: *mut crate::src::common::common::x264_t = (*h).thread[i as usize];
            if i != 0 {
                (*t).param = (*h).param;
                crate::stdlib::memcpy(
                    &raw mut (*t).i_frame as *mut ::core::ffi::c_void,
                    &raw mut (*h).i_frame as *const ::core::ffi::c_void,
                    (40256 as crate::__stddef_size_t_h::size_t)
                        .wrapping_sub(2420 as crate::__stddef_size_t_h::size_t),
                );
            }
            let mut height: ::core::ffi::c_int = (*h).mb.i_mb_height >> (*h).param.b_interlaced;
            (*t).i_threadslice_start =
                (height * i + round_bias) / (*h).param.i_threads << (*h).param.b_interlaced;
            (*t).i_threadslice_end = (height * (i + 1 as ::core::ffi::c_int) + round_bias)
                / (*h).param.i_threads
                << (*h).param.b_interlaced;
            (*t).sh.i_first_mb = (*t).i_threadslice_start * (*h).mb.i_mb_width;
            (*t).sh.i_last_mb =
                (*t).i_threadslice_end * (*h).mb.i_mb_width - 1 as ::core::ffi::c_int;
            i += 1;
        }
        crate::src::encoder::analyse::x264_8_analyse_weight_frame(
            h as *mut crate::src::common::common::x264_t,
            (*h).mb.i_mb_height * 16 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
        );
        crate::src::encoder::ratecontrol::x264_8_threads_distribute_ratecontrol(
            h as *mut crate::src::common::common::x264_t,
        );
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*h).param.i_threads {
            (*(*h).thread[i_0 as usize]).i_thread_idx = i_0;
            (*(*h).thread[i_0 as usize]).b_thread_active = 1 as ::core::ffi::c_int;
            crate::src::common::frame::x264_8_threadslice_cond_broadcast(
                (*h).thread[i_0 as usize] as *mut crate::src::common::common::x264_t,
                0 as ::core::ffi::c_int,
            );
            i_0 += 1;
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*h).param.i_threads {
            crate::src::common::threadpool::x264_8_threadpool_run(
                (*h).threadpool,
                ::core::mem::transmute::<
                    *mut ::core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
                    >,
                >(::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut crate::src::common::common::x264_t,
                        ) -> *mut ::core::ffi::c_void,
                    >,
                    *mut ::core::ffi::c_void,
                >(Some(
                    slices_write
                        as unsafe extern "C" fn(
                            *mut crate::src::common::common::x264_t,
                        )
                            -> *mut ::core::ffi::c_void,
                ))),
                (*h).thread[i_1 as usize] as *mut ::core::ffi::c_void,
            );
            i_1 += 1;
        }
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < (*h).param.i_threads {
            crate::src::common::frame::x264_8_threadslice_cond_wait(
                (*h).thread[i_2 as usize] as *mut crate::src::common::common::x264_t,
                1 as ::core::ffi::c_int,
            );
            i_2 += 1;
        }
        crate::src::encoder::ratecontrol::x264_8_threads_merge_ratecontrol(
            h as *mut crate::src::common::common::x264_t,
        );
        let mut i_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i_3 < (*h).param.i_threads {
            let mut t_0: *mut crate::src::common::common::x264_t = (*h).thread[i_3 as usize];
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < (*t_0).out.i_nal {
                *(*h).out.nal.offset((*h).out.i_nal as isize) = *(*t_0).out.nal.offset(j as isize);
                (*h).out.i_nal += 1;
                nal_check_buffer(h);
                j += 1;
            }
            let mut j_0: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
            while j_0
                < (43536 as usize)
                    .wrapping_sub(42848 as usize)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
            {
                *(&raw mut (*h).stat.frame as *mut ::core::ffi::c_int).offset(j_0 as isize) +=
                    *(&raw mut (*t_0).stat.frame as *mut ::core::ffi::c_int).offset(j_0 as isize);
                j_0 = j_0.wrapping_add(1);
            }
            let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_1 < 3 as ::core::ffi::c_int {
                (*h).stat.frame.i_ssd[j_1 as usize] += (*t_0).stat.frame.i_ssd[j_1 as usize];
                j_1 += 1;
            }
            (*h).stat.frame.f_ssim += (*t_0).stat.frame.f_ssim;
            (*h).stat.frame.i_ssim_cnt += (*t_0).stat.frame.i_ssim_cnt;
            i_3 += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_intra_refresh(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        h = (*h).thread[(*h).i_thread_phase as usize];
        (*h).b_queued_intra_refresh = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_invalidate_reference(
    mut h: *mut crate::src::common::common::x264_t,
    mut pts: crate::stdlib::int64_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*h).param.i_bframe != 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"x264_encoder_invalidate_reference is not supported with B-frames enabled\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.b_intra_refresh != 0 {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"x264_encoder_invalidate_reference is not supported with intra refresh enabled\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        h = (*h).thread[(*h).i_thread_phase as usize];
        if pts >= (*h).i_last_idr_pts {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !(*h).frames.reference[i as usize].is_null() {
                if pts <= (*(*h).frames.reference[i as usize]).i_pts {
                    (*(*h).frames.reference[i as usize]).b_corrupt = 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
            if pts <= (*(*h).fdec).i_pts {
                (*(*h).fdec).b_corrupt = 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_encode(
    mut h: *mut crate::src::common::common::x264_t,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_in: *mut crate::x264_h::x264_picture_t_3,
    mut pic_out: *mut crate::x264_h::x264_picture_t_3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut thread_current: *mut crate::src::common::common::x264_t =
            ::core::ptr::null_mut::<crate::src::common::common::x264_t>();
        let mut thread_prev: *mut crate::src::common::common::x264_t =
            ::core::ptr::null_mut::<crate::src::common::common::x264_t>();
        let mut thread_oldest: *mut crate::src::common::common::x264_t =
            ::core::ptr::null_mut::<crate::src::common::common::x264_t>();
        let mut i_nal_type: ::core::ffi::c_int = 0;
        let mut i_nal_ref_idc: ::core::ffi::c_int = 0;
        let mut i_global_qp: ::core::ffi::c_int = 0;
        let mut overhead: ::core::ffi::c_int = crate::src::common::common::NALU_OVERHEAD;
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            thread_prev = (*h).thread[(*h).i_thread_phase as usize];
            (*h).i_thread_phase =
                ((*h).i_thread_phase + 1 as ::core::ffi::c_int) % (*h).i_thread_frames;
            thread_current = (*h).thread[(*h).i_thread_phase as usize];
            thread_oldest = (*h).thread
                [(((*h).i_thread_phase + 1 as ::core::ffi::c_int) % (*h).i_thread_frames) as usize];
            thread_sync_context(thread_current, thread_prev);
            crate::src::encoder::ratecontrol::x264_8_thread_sync_ratecontrol(
                thread_current as *mut crate::src::common::common::x264_t,
                thread_prev as *mut crate::src::common::common::x264_t,
                thread_oldest as *mut crate::src::common::common::x264_t,
            );
            h = thread_current;
        } else {
            thread_oldest = h;
            thread_current = thread_oldest;
        }
        (*h).i_cpb_delay_pir_offset = (*h).i_cpb_delay_pir_offset_next;
        *pi_nal = 0 as ::core::ffi::c_int;
        *pp_nal = ::core::ptr::null_mut::<crate::x264_h::x264_nal_t>();
        if !pic_in.is_null() {
            if (*(*h).lookahead).b_exit_thread != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"lookahead thread is already stopped\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            let mut fenc: *mut crate::src::common::frame::x264_frame_t =
                crate::src::common::frame::x264_8_frame_pop_unused(
                    h as *mut crate::src::common::common::x264_t,
                    0 as ::core::ffi::c_int,
                ) as *mut crate::src::common::frame::x264_frame;
            if fenc.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
            if crate::src::common::frame::x264_8_frame_copy_picture(
                h as *mut crate::src::common::common::x264_t,
                fenc as *mut crate::src::common::frame::x264_frame,
                pic_in as *mut crate::x264_h::x264_picture_t_1,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            if (*h).param.i_width != 16 as ::core::ffi::c_int * (*h).mb.i_mb_width
                || (*h).param.i_height != 16 as ::core::ffi::c_int * (*h).mb.i_mb_height
            {
                crate::src::common::frame::x264_8_frame_expand_border_mod16(
                    h as *mut crate::src::common::common::x264_t,
                    fenc as *mut crate::src::common::frame::x264_frame,
                );
            }
            let c2rust_fresh8 = (*h).frames.i_input;
            (*h).frames.i_input = (*h).frames.i_input + 1;
            (*fenc).i_frame = c2rust_fresh8;
            if (*fenc).i_frame == 0 as ::core::ffi::c_int {
                (*h).frames.i_first_pts = (*fenc).i_pts;
            }
            if (*h).frames.i_bframe_delay != 0 && (*fenc).i_frame == (*h).frames.i_bframe_delay {
                (*h).frames.i_bframe_delay_time = (*fenc).i_pts - (*h).frames.i_first_pts;
            }
            if (*h).param.b_vfr_input != 0 && (*fenc).i_pts <= (*h).frames.i_largest_pts {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_WARNING_1,
                    b"non-strictly-monotonic PTS\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*h).frames.i_second_largest_pts = (*h).frames.i_largest_pts;
            (*h).frames.i_largest_pts = (*fenc).i_pts;
            if (*fenc).i_pic_struct < crate::x264_h::PIC_STRUCT_AUTO as ::core::ffi::c_int
                || (*fenc).i_pic_struct > crate::x264_h::PIC_STRUCT_TRIPLE as ::core::ffi::c_int
            {
                (*fenc).i_pic_struct = crate::x264_h::PIC_STRUCT_AUTO as ::core::ffi::c_int;
            }
            if (*fenc).i_pic_struct == crate::x264_h::PIC_STRUCT_AUTO as ::core::ffi::c_int {
                let mut b_interlaced: ::core::ffi::c_int = if !(*fenc).param.is_null() {
                    (*(*fenc).param).b_interlaced
                } else {
                    (*h).param.b_interlaced
                };
                if b_interlaced != 0 {
                    let mut b_tff: ::core::ffi::c_int = if !(*fenc).param.is_null() {
                        (*(*fenc).param).b_tff
                    } else {
                        (*h).param.b_tff
                    };
                    (*fenc).i_pic_struct = if b_tff != 0 {
                        crate::x264_h::PIC_STRUCT_TOP_BOTTOM as ::core::ffi::c_int
                    } else {
                        crate::x264_h::PIC_STRUCT_BOTTOM_TOP as ::core::ffi::c_int
                    };
                } else {
                    (*fenc).i_pic_struct =
                        crate::x264_h::PIC_STRUCT_PROGRESSIVE as ::core::ffi::c_int;
                }
            }
            if (*h).param.rc.b_mb_tree != 0 && (*h).param.rc.b_stat_read != 0 {
                if crate::src::encoder::ratecontrol::x264_8_macroblock_tree_read(
                    h as *mut crate::src::common::common::x264_t,
                    fenc as *mut crate::src::common::frame::x264_frame,
                    (*pic_in).prop.quant_offsets,
                ) != 0
                {
                    return -(1 as ::core::ffi::c_int);
                }
            } else {
                crate::src::encoder::ratecontrol::x264_8_adaptive_quant_frame(
                    h as *mut crate::src::common::common::x264_t,
                    fenc as *mut crate::src::common::frame::x264_frame,
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
            if (*h).frames.b_have_lowres != 0 {
                crate::src::common::mc::x264_8_frame_init_lowres(
                    h as *mut crate::src::common::common::x264_t,
                    fenc as *mut crate::src::common::frame::x264_frame,
                );
            }
            crate::src::encoder::lookahead::x264_8_lookahead_put_frame(
                h as *mut crate::src::common::common::x264_t,
                fenc as *mut crate::src::common::frame::x264_frame,
            );
            if (*h).frames.i_input
                <= (*h).frames.i_delay + 1 as ::core::ffi::c_int - (*h).i_thread_frames
            {
                (*pic_out).i_type = crate::x264_h::X264_TYPE_AUTO;
                return 0 as ::core::ffi::c_int;
            }
        } else {
            crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ifbuf.mutex);
            ::core::ptr::write_volatile(
                &mut (*(*h).lookahead).b_exit_thread as *mut crate::stdlib::uint8_t,
                1 as crate::stdlib::uint8_t,
            );
            crate::stdlib::pthread_cond_broadcast(&raw mut (*(*h).lookahead).ifbuf.cv_fill);
            crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
        }
        (*h).i_frame += 1;
        if (*(*h).frames.current.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            crate::src::encoder::lookahead::x264_8_lookahead_get_frames(
                h as *mut crate::src::common::common::x264_t,
            );
        }
        if (*(*h).frames.current.offset(0 as ::core::ffi::c_int as isize)).is_null()
            && crate::src::encoder::lookahead::x264_8_lookahead_is_empty(
                h as *mut crate::src::common::common::x264_t,
            ) != 0
        {
            return encoder_frame_end(thread_oldest, thread_current, pp_nal, pi_nal, pic_out);
        }
        (*h).fenc = crate::src::common::frame::x264_8_frame_shift(
            (*h).frames.current as *mut *mut crate::src::common::frame::x264_frame,
        ) as *mut crate::src::common::frame::x264_frame;
        if (*h).param.b_sliced_threads != 0 {
            if threadpool_wait_all(h) < 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if (*h).i_frame == 0 as ::core::ffi::c_int {
            (*h).i_reordered_pts_delay = (*(*h).fenc).i_reordered_pts;
        }
        if (*h).reconfig != 0 {
            x264_8_encoder_reconfig_apply(h, &raw mut (*(*h).reconfig_h).param);
            (*h).reconfig = 0 as ::core::ffi::c_int;
        }
        if !(*(*h).fenc).param.is_null() {
            x264_8_encoder_reconfig_apply(h, (*(*h).fenc).param);
            if (*(*(*h).fenc).param).param_free.is_some() {
                crate::src::common::base::x264_param_cleanup(
                    (*(*h).fenc).param as *mut crate::x264_h::x264_param_t,
                );
                (*(*(*h).fenc).param)
                    .param_free
                    .expect("non-null function pointer")(
                    (*(*h).fenc).param as *mut ::core::ffi::c_void,
                );
                (*(*h).fenc).param = ::core::ptr::null_mut::<crate::x264_h::x264_param_t>();
            }
        }
        crate::src::encoder::ratecontrol::x264_8_ratecontrol_zone_init(
            h as *mut crate::src::common::common::x264_t,
        );
        if reference_update(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        (*(*h).fdec).i_lines_completed = -(1 as ::core::ffi::c_int);
        if !((*(*h).fenc).i_type == crate::x264_h::X264_TYPE_I
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_KEYFRAME)
        {
            let mut valid_refs_left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !(*h).frames.reference[i as usize].is_null() {
                if (*(*h).frames.reference[i as usize]).b_corrupt == 0 {
                    valid_refs_left += 1;
                }
                i += 1;
            }
            if valid_refs_left == 0 {
                (*(*h).fenc).b_keyframe = 1 as ::core::ffi::c_int;
                (*(*h).fenc).i_type = crate::x264_h::X264_TYPE_IDR;
            }
        }
        if (*(*h).fenc).b_keyframe != 0 {
            (*h).frames.i_last_keyframe = (*(*h).fenc).i_frame;
            if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR {
                (*h).i_frame_num = 0 as ::core::ffi::c_int;
                (*h).frames.i_last_idr = (*(*h).fenc).i_frame;
            }
        }
        (*h).sh.i_mmco_remove_from_end = 0 as ::core::ffi::c_int;
        (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_remove_from_end;
        (*h).b_ref_reorder[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
        (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize] =
            (*h).b_ref_reorder[1 as ::core::ffi::c_int as usize];
        (*(*h).fenc).i_poc = 2 as ::core::ffi::c_int
            * ((*(*h).fenc).i_frame
                - (if (*h).frames.i_last_idr > 0 as ::core::ffi::c_int {
                    (*h).frames.i_last_idr
                } else {
                    0 as ::core::ffi::c_int
                }));
        (*(*h).fdec).i_poc = (*(*h).fenc).i_poc;
        if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR {
            i_nal_type = crate::x264_h::NAL_SLICE_IDR as ::core::ffi::c_int;
            i_nal_ref_idc = crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int;
            (*h).sh.i_type = crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int;
            reference_reset(h);
            (*h).frames.i_poc_last_open_gop = -(1 as ::core::ffi::c_int);
        } else if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_I {
            i_nal_type = crate::x264_h::NAL_SLICE as ::core::ffi::c_int;
            i_nal_ref_idc = crate::x264_h::NAL_PRIORITY_HIGH as ::core::ffi::c_int;
            (*h).sh.i_type = crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int;
            reference_hierarchy_reset(h);
            if (*h).param.b_open_gop != 0 {
                (*h).frames.i_poc_last_open_gop = if (*(*h).fenc).b_keyframe != 0 {
                    (*(*h).fenc).i_poc
                } else {
                    -(1 as ::core::ffi::c_int)
                };
            }
        } else if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_P {
            i_nal_type = crate::x264_h::NAL_SLICE as ::core::ffi::c_int;
            i_nal_ref_idc = crate::x264_h::NAL_PRIORITY_HIGH as ::core::ffi::c_int;
            (*h).sh.i_type = crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int;
            reference_hierarchy_reset(h);
            (*h).frames.i_poc_last_open_gop = -(1 as ::core::ffi::c_int);
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
        (*(*h).fdec).b_kept_as_ref = (i_nal_ref_idc
            != crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int
            && (*h).param.i_keyint_max > 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*(*h).fenc).b_kept_as_ref = (*(*h).fdec).b_kept_as_ref;
        (*(*h).fdec).mb_info = (*(*h).fenc).mb_info;
        (*(*h).fdec).mb_info_free = (*(*h).fenc).mb_info_free;
        (*(*h).fenc).mb_info = ::core::ptr::null_mut::<crate::stdlib::uint8_t>();
        (*(*h).fenc).mb_info_free = None;
        (*(*h).fdec).i_pts = (*(*h).fenc).i_pts;
        if (*h).frames.i_bframe_delay != 0 {
            let mut prev_reordered_pts: *mut crate::stdlib::int64_t =
                &raw mut (*thread_current).frames.i_prev_reordered_pts
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
        if (*h).param.b_sliced_threads != 0 {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < (*h).param.i_threads {
                bs_init(
                    &raw mut (**(&raw mut (*h).thread
                        as *mut *mut crate::src::common::common::x264_t)
                        .offset(i_0 as isize))
                    .out
                    .bs,
                    (*(*h).thread[i_0 as usize]).out.p_bitstream as *mut ::core::ffi::c_void,
                    (*(*h).thread[i_0 as usize]).out.i_bitstream,
                );
                (*(*h).thread[i_0 as usize]).out.i_nal = 0 as ::core::ffi::c_int;
                i_0 += 1;
            }
        } else {
            bs_init(
                &raw mut (*h).out.bs,
                (*h).out.p_bitstream as *mut ::core::ffi::c_void,
                (*h).out.i_bitstream,
            );
            (*h).out.i_nal = 0 as ::core::ffi::c_int;
        }
        if (*h).param.b_aud != 0 {
            let mut pic_type: ::core::ffi::c_int = 0;
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
                pic_type = 0 as ::core::ffi::c_int;
            } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            {
                pic_type = 1 as ::core::ffi::c_int;
            } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            {
                pic_type = 2 as ::core::ffi::c_int;
            } else {
                pic_type = 7 as ::core::ffi::c_int;
            }
            nal_start(
                h,
                crate::x264_h::NAL_AUD as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            bs_write(
                &raw mut (*h).out.bs,
                3 as ::core::ffi::c_int,
                pic_type as crate::stdlib::uint32_t,
            );
            bs_rbsp_trailing(&raw mut (*h).out.bs);
            bs_flush(&raw mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_payload
                + crate::src::common::common::NALU_OVERHEAD;
        }
        (*h).i_nal_type = i_nal_type;
        (*h).i_nal_ref_idc = i_nal_ref_idc;
        if (*h).param.b_intra_refresh != 0 {
            if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_I
                || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_IDR
                || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_KEYFRAME
            {
                (*(*h).fdec).i_frames_since_pir = 0 as ::core::ffi::c_int;
                (*h).b_queued_intra_refresh = 0 as ::core::ffi::c_int;
                (*(*h).fdec).f_pir_position = (*h).mb.i_mb_width as ::core::ffi::c_float;
            } else if (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_P {
                let mut pocdiff: ::core::ffi::c_int = ((*(*h).fdec).i_poc
                    - (*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .i_poc)
                    / 2 as ::core::ffi::c_int;
                let mut increment: ::core::ffi::c_float = if ((*h).mb.i_mb_width
                    as ::core::ffi::c_float
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_float)
                    / (*h).param.i_keyint_max as ::core::ffi::c_float
                    > 1 as ::core::ffi::c_int as ::core::ffi::c_float
                {
                    ((*h).mb.i_mb_width as ::core::ffi::c_float
                        - 1 as ::core::ffi::c_int as ::core::ffi::c_float)
                        / (*h).param.i_keyint_max as ::core::ffi::c_float
                } else {
                    1 as ::core::ffi::c_int as ::core::ffi::c_float
                };
                (*(*h).fdec).f_pir_position = (*(*h).fref[0 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize])
                    .f_pir_position;
                (*(*h).fdec).i_frames_since_pir = (*(*h).fref[0 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize])
                    .i_frames_since_pir
                    + pocdiff;
                if (*(*h).fdec).i_frames_since_pir >= (*h).param.i_keyint_max
                    || (*h).b_queued_intra_refresh != 0
                        && (*(*h).fdec).f_pir_position as ::core::ffi::c_double + 0.5f64
                            >= (*h).mb.i_mb_width as ::core::ffi::c_double
                {
                    (*(*h).fdec).f_pir_position = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
                    (*(*h).fdec).i_frames_since_pir = 0 as ::core::ffi::c_int;
                    (*h).b_queued_intra_refresh = 0 as ::core::ffi::c_int;
                    (*(*h).fenc).b_keyframe = 1 as ::core::ffi::c_int;
                }
                (*(*h).fdec).i_pir_start_col = ((*(*h).fdec).f_pir_position
                    as ::core::ffi::c_double
                    + 0.5f64) as ::core::ffi::c_int;
                (*(*h).fdec).f_pir_position += increment * pocdiff as ::core::ffi::c_float;
                (*(*h).fdec).i_pir_end_col = ((*(*h).fdec).f_pir_position as ::core::ffi::c_double
                    + 0.5f64) as ::core::ffi::c_int;
                if (*(*h).fdec).i_pir_end_col >= (*h).mb.i_mb_width - 1 as ::core::ffi::c_int {
                    (*(*h).fdec).f_pir_position = (*h).mb.i_mb_width as ::core::ffi::c_float;
                    (*(*h).fdec).i_pir_end_col = (*h).mb.i_mb_width - 1 as ::core::ffi::c_int;
                }
            }
        }
        if (*(*h).fenc).b_keyframe != 0 {
            if (*h).param.b_repeat_headers != 0 {
                nal_start(
                    h,
                    crate::x264_h::NAL_SPS as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sps_write(
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                    &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t
                        as *mut crate::src::common::set::x264_sps_t,
                );
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                if (*h).param.i_avcintra_class != 0 {
                    (*(*h)
                        .out
                        .nal
                        .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_padding = 256 as ::core::ffi::c_int
                        - bs_pos(&raw mut (*h).out.bs) / 8 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int * crate::src::common::common::NALU_OVERHEAD;
                }
                overhead += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                    + (*(*h)
                        .out
                        .nal
                        .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_padding
                    + crate::src::common::common::NALU_OVERHEAD;
                nal_start(
                    h,
                    crate::x264_h::NAL_PPS as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_pps_write(
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                    &raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t
                        as *mut crate::src::common::set::x264_sps_t,
                    &raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t
                        as *mut crate::src::common::set::x264_pps_t,
                );
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                if (*h).param.i_avcintra_class != 0 {
                    let mut total_len: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
                    if (*h).param.i_avcintra_flavor == crate::x264_h::X264_AVCINTRA_FLAVOR_SONY {
                        total_len += if (*h).param.i_height >= 1080 as ::core::ffi::c_int {
                            18 as ::core::ffi::c_int * 512 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int * 512 as ::core::ffi::c_int
                        };
                    }
                    (*(*h)
                        .out
                        .nal
                        .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_padding = total_len
                        - (*(*h)
                            .out
                            .nal
                            .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                        .i_payload
                        - crate::src::common::common::NALU_OVERHEAD;
                }
                overhead += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                    + (*(*h)
                        .out
                        .nal
                        .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_padding
                    + crate::src::common::common::NALU_OVERHEAD;
            }
            if (*h).i_thread_frames == 1 as ::core::ffi::c_int
                && (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                    .vui
                    .b_nal_hrd_parameters_present
                    != 0
            {
                crate::src::encoder::ratecontrol::x264_8_hrd_fullness(
                    h as *mut crate::src::common::common::x264_t,
                );
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_buffering_period_write(
                    h as *mut crate::src::common::common::x264_t,
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                );
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                overhead += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
            }
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*(*h).fenc).extra_sei.num_payloads {
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_write(
                &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload,
                (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload_size,
                (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload_type,
            );
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
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
        if (*(*h).fenc).b_keyframe != 0 {
            if (*h).param.b_repeat_headers != 0
                && (*(*h).fenc).i_frame == 0 as ::core::ffi::c_int
                && (*h).param.i_avcintra_class == 0
            {
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                if crate::src::encoder::set::x264_8_sei_version_write(
                    h as *mut crate::src::common::common::x264_t,
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                ) != 0
                {
                    return -(1 as ::core::ffi::c_int);
                }
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                overhead += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
            }
            if (*(*h).fenc).i_type != crate::x264_h::X264_TYPE_IDR {
                let mut time_to_recovery: ::core::ffi::c_int = if (*h).param.b_open_gop != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    (if ((*h).mb.i_mb_width - 1 as ::core::ffi::c_int) < (*h).param.i_keyint_max {
                        (*h).mb.i_mb_width - 1 as ::core::ffi::c_int
                    } else {
                        (*h).param.i_keyint_max
                    }) + (*h).param.i_bframe
                        - 1 as ::core::ffi::c_int
                };
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_recovery_point_write(
                    h as *mut crate::src::common::common::x264_t,
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                    time_to_recovery,
                );
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                overhead += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
            }
            if (*h).param.mastering_display.b_mastering_display != 0 {
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_mastering_display_write(
                    h as *mut crate::src::common::common::x264_t,
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                );
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                overhead += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
            }
            if (*h).param.content_light_level.b_cll != 0 {
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_content_light_level_write(
                    h as *mut crate::src::common::common::x264_t,
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                );
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                overhead += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
            }
            if (*h).param.i_alternative_transfer != 2 as ::core::ffi::c_int {
                nal_start(
                    h,
                    crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_sei_alternative_transfer_write(
                    h as *mut crate::src::common::common::x264_t,
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                );
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                overhead += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                    + (crate::src::common::common::NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0
                            && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
            }
        }
        if (*h).param.i_frame_packing >= 0 as ::core::ffi::c_int
            && ((*(*h).fenc).b_keyframe != 0
                || (*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
        {
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_frame_packing_write(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
            );
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
        }
        if (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
            .vui
            .b_pic_struct_present
            != 0
            || (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .b_nal_hrd_parameters_present
                != 0
        {
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_pic_timing_write(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
            );
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
        }
        if !((*(*h).fenc).i_type == crate::x264_h::X264_TYPE_B
            || (*(*h).fenc).i_type == crate::x264_h::X264_TYPE_BREF)
            && (*h).b_sh_backup != 0
        {
            (*h).b_sh_backup = 0 as ::core::ffi::c_int;
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_dec_ref_pic_marking_write(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
            );
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
        }
        if (*(*h).fenc).b_keyframe != 0 && (*h).param.b_intra_refresh != 0 {
            (*h).i_cpb_delay_pir_offset_next = (*(*h).fenc).i_cpb_delay;
        }
        if (*h).param.i_avcintra_class != 0
            && (*h).param.i_avcintra_flavor != crate::x264_h::X264_AVCINTRA_FLAVOR_SONY
        {
            nal_start(
                h,
                crate::x264_h::NAL_FILLER as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_filler_write(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                0 as ::core::ffi::c_int,
            );
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_payload
                + crate::src::common::common::NALU_OVERHEAD;
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            if crate::src::encoder::set::x264_8_sei_avcintra_umid_write(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_payload
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
            let mut unpadded_len: ::core::ffi::c_int = 0;
            let mut total_len_0: ::core::ffi::c_int = 0;
            if (*h).param.i_height == 1080 as ::core::ffi::c_int {
                unpadded_len = 5780 as ::core::ffi::c_int;
                total_len_0 = 17 as ::core::ffi::c_int * 512 as ::core::ffi::c_int;
            } else {
                unpadded_len = 2900 as ::core::ffi::c_int;
                total_len_0 = 9 as ::core::ffi::c_int * 512 as ::core::ffi::c_int;
            }
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            if crate::src::encoder::set::x264_8_sei_avcintra_vanc_write(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                unpadded_len,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_padding = total_len_0
                - (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                - (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
            overhead += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_payload
                + (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_padding
                + (crate::src::common::common::NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0
                        && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
        }
        crate::src::encoder::ratecontrol::x264_8_ratecontrol_start(
            h as *mut crate::src::common::common::x264_t,
            (*(*h).fenc).i_qpplus1,
            overhead * 8 as ::core::ffi::c_int,
        );
        i_global_qp = crate::src::encoder::ratecontrol::x264_8_ratecontrol_qp(
            h as *mut crate::src::common::common::x264_t,
        );
        (*(*h).fdec).i_qpplus1 = i_global_qp + 1 as ::core::ffi::c_int;
        (*pic_out).i_qpplus1 = (*(*h).fdec).i_qpplus1;
        if (*h).param.rc.b_stat_read != 0
            && (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
        {
            crate::src::encoder::ratecontrol::x264_8_reference_build_list_optimal(
                h as *mut crate::src::common::common::x264_t,
            );
            reference_check_reorder(h);
        }
        if (*h).i_ref[0 as ::core::ffi::c_int as usize] != 0 {
            (*(*h).fdec).i_poc_l0ref0 = (*(*h).fref[0 as ::core::ffi::c_int as usize]
                [0 as ::core::ffi::c_int as usize])
                .i_poc;
        }
        slice_init(h, i_nal_type, i_global_qp);
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            crate::src::common::macroblock::x264_8_macroblock_bipred_init(
                h as *mut crate::src::common::common::x264_t,
            );
        }
        weighted_pred_init(h);
        if i_nal_ref_idc != crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int {
            (*h).i_frame_num += 1;
        }
        (*h).i_threadslice_start = 0 as ::core::ffi::c_int;
        (*h).i_threadslice_end = (*h).mb.i_mb_height;
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            crate::src::common::threadpool::x264_8_threadpool_run(
                (*h).threadpool,
                ::core::mem::transmute::<
                    *mut ::core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
                    >,
                >(::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut crate::src::common::common::x264_t,
                        ) -> *mut ::core::ffi::c_void,
                    >,
                    *mut ::core::ffi::c_void,
                >(Some(
                    slices_write
                        as unsafe extern "C" fn(
                            *mut crate::src::common::common::x264_t,
                        )
                            -> *mut ::core::ffi::c_void,
                ))),
                h as *mut ::core::ffi::c_void,
            );
            (*h).b_thread_active = 1 as ::core::ffi::c_int;
        } else if (*h).param.b_sliced_threads != 0 {
            if threaded_slices_write(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
        } else if slices_write(h) as crate::stdlib::intptr_t != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        return encoder_frame_end(thread_oldest, thread_current, pp_nal, pi_nal, pic_out);
    }
}

unsafe extern "C" fn encoder_frame_end(
    mut h: *mut crate::src::common::common::x264_t,
    mut thread_current: *mut crate::src::common::common::x264_t,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_out: *mut crate::x264_h::x264_picture_t_3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut psz_message: [::core::ffi::c_char; 80] = [0; 80];
        if (*h).param.b_sliced_threads == 0 && (*h).b_thread_active != 0 {
            (*h).b_thread_active = 0 as ::core::ffi::c_int;
            if crate::src::common::threadpool::x264_8_threadpool_wait(
                (*h).threadpool,
                h as *mut ::core::ffi::c_void,
            ) as crate::stdlib::intptr_t
                != 0
            {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if (*h).out.i_nal == 0 {
            (*pic_out).i_type = crate::x264_h::X264_TYPE_AUTO;
            return 0 as ::core::ffi::c_int;
        }
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int
            && (*(*h).fenc).b_keyframe != 0
            && (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                .vui
                .b_nal_hrd_parameters_present
                != 0
        {
            crate::src::encoder::ratecontrol::x264_8_hrd_fullness(
                h as *mut crate::src::common::common::x264_t,
            );
            nal_start(
                h,
                crate::x264_h::NAL_SEI as ::core::ffi::c_int,
                crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            crate::src::encoder::set::x264_8_sei_buffering_period_write(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
            );
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while (*(*h).out.nal.offset(idx as isize)).i_type
                == crate::x264_h::NAL_AUD as ::core::ffi::c_int
                || (*(*h).out.nal.offset(idx as isize)).i_type
                    == crate::x264_h::NAL_SPS as ::core::ffi::c_int
                || (*(*h).out.nal.offset(idx as isize)).i_type
                    == crate::x264_h::NAL_PPS as ::core::ffi::c_int
            {
                idx += 1;
            }
            let mut nal_tmp: crate::x264_h::x264_nal_t = *(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize);
            crate::stdlib::memmove(
                (*h).out
                    .nal
                    .offset((idx + 1 as ::core::ffi::c_int) as isize)
                    as *mut crate::x264_h::x264_nal_t as *mut ::core::ffi::c_void,
                (*h).out.nal.offset(idx as isize) as *mut crate::x264_h::x264_nal_t
                    as *const ::core::ffi::c_void,
                (((*h).out.i_nal - idx - 1 as ::core::ffi::c_int)
                    as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::x264_h::x264_nal_t>()
                        as crate::__stddef_size_t_h::size_t),
            );
            *(*h).out.nal.offset(idx as isize) = nal_tmp;
        }
        let mut frame_size: ::core::ffi::c_int =
            encoder_encapsulate_nals(h, 0 as ::core::ffi::c_int);
        if frame_size < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        (*pic_out).i_type = (*(*h).fenc).i_type;
        (*pic_out).b_keyframe = (*(*h).fenc).b_keyframe;
        (*pic_out).i_pic_struct = (*(*h).fenc).i_pic_struct;
        (*pic_out).i_pts = (*(*h).fdec).i_pts;
        (*pic_out).i_dts = (*(*h).fdec).i_dts;
        if (*pic_out).i_pts < (*pic_out).i_dts {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_WARNING_1,
                b"invalid DTS: PTS is less than DTS\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        (*pic_out).opaque = (*(*h).fenc).opaque;
        (*pic_out).img.i_csp = (*(*h).fdec).i_csp;
        (*pic_out).img.i_plane = (*(*h).fdec).i_plane;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*pic_out).img.i_plane {
            (*pic_out).img.i_stride[i as usize] =
                (*(*h).fdec).i_stride[i as usize] * crate::src::common::common::SIZEOF_PIXEL;
            (*pic_out).img.plane[i as usize] =
                (*(*h).fdec).plane[i as usize] as *mut crate::stdlib::uint8_t;
            i += 1;
        }
        crate::src::common::frame::x264_8_frame_push_unused(
            thread_current as *mut crate::src::common::common::x264_t,
            (*h).fenc as *mut crate::src::common::frame::x264_frame,
        );
        let mut filler: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if crate::src::encoder::ratecontrol::x264_8_ratecontrol_end(
            h as *mut crate::src::common::common::x264_t,
            frame_size * 8 as ::core::ffi::c_int,
            &raw mut filler,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        (*pic_out).hrd_timing = (*(*h).fenc).hrd_timing;
        (*pic_out).prop.f_crf_avg = (*(*h).fdec).f_crf_avg as ::core::ffi::c_double;
        if (*h).param.i_avcintra_class != 0 {
            if check_encapsulated_buffer(
                h,
                (*h).thread[0 as ::core::ffi::c_int as usize],
                (*h).out.i_nal,
                frame_size as crate::stdlib::int64_t,
                frame_size as crate::stdlib::int64_t + filler as crate::stdlib::int64_t,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            let mut nal: *mut crate::x264_h::x264_nal_t = (*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize)
                as *mut crate::x264_h::x264_nal_t;
            crate::stdlib::memset(
                (*nal).p_payload.offset((*nal).i_payload as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                filler as crate::__stddef_size_t_h::size_t,
            );
            (*nal).i_payload += filler;
            (*nal).i_padding = filler;
            frame_size += filler;
            if (*h).param.b_annexb == 0 {
                let mut nal_data: *mut crate::stdlib::uint8_t = (*nal).p_payload;
                let mut chunk_size: ::core::ffi::c_int = (*nal).i_payload - 4 as ::core::ffi::c_int;
                *nal_data.offset(0 as ::core::ffi::c_int as isize) =
                    (chunk_size >> 24 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
                *nal_data.offset(1 as ::core::ffi::c_int as isize) =
                    (chunk_size >> 16 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
                *nal_data.offset(2 as ::core::ffi::c_int as isize) =
                    (chunk_size >> 8 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
                *nal_data.offset(3 as ::core::ffi::c_int as isize) =
                    (chunk_size >> 0 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
            }
        } else {
            while filler > 0 as ::core::ffi::c_int {
                let mut f: ::core::ffi::c_int = 0;
                let mut overhead: ::core::ffi::c_int =
                    crate::src::common::common::FILLER_OVERHEAD - (*h).param.b_annexb;
                if (*h).param.i_slice_max_size != 0 && filler > (*h).param.i_slice_max_size {
                    let mut next_size: ::core::ffi::c_int = filler - (*h).param.i_slice_max_size;
                    let mut overflow: ::core::ffi::c_int =
                        if overhead - next_size > 0 as ::core::ffi::c_int {
                            overhead - next_size
                        } else {
                            0 as ::core::ffi::c_int
                        };
                    f = (*h).param.i_slice_max_size - overhead - overflow;
                } else {
                    f = if 0 as ::core::ffi::c_int > filler - overhead {
                        0 as ::core::ffi::c_int
                    } else {
                        filler - overhead
                    };
                }
                if bitstream_check_buffer_filler(h, f) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                nal_start(
                    h,
                    crate::x264_h::NAL_FILLER as ::core::ffi::c_int,
                    crate::x264_h::NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
                );
                crate::src::encoder::set::x264_8_filler_write(
                    h as *mut crate::src::common::common::x264_t,
                    &raw mut (*h).out.bs as *mut _ as *mut crate::src::common::bitstream::bs_s,
                    f,
                );
                if nal_end(h) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                let mut total_size: ::core::ffi::c_int =
                    encoder_encapsulate_nals(h, (*h).out.i_nal - 1 as ::core::ffi::c_int);
                if total_size < 0 as ::core::ffi::c_int {
                    return -(1 as ::core::ffi::c_int);
                }
                frame_size += total_size;
                filler -= total_size;
            }
        }
        *pi_nal = (*h).out.i_nal;
        *pp_nal = (*h).out.nal;
        (*h).out.i_nal = 0 as ::core::ffi::c_int;
        crate::src::encoder::macroblock::x264_8_noise_reduction_update(
            h as *mut crate::src::common::common::x264_t,
        );
        thread_sync_stat(h, (*h).thread[0 as ::core::ffi::c_int as usize]);
        (*h).stat.i_frame_count[(*h).sh.i_type as usize] += 1;
        (*h).stat.i_frame_size[(*h).sh.i_type as usize] += frame_size as crate::stdlib::int64_t;
        (*h).stat.f_frame_qp[(*h).sh.i_type as usize] +=
            (*(*h).fdec).f_qp_avg_aq as ::core::ffi::c_double;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < crate::src::common::macroblock::X264_MBTYPE_MAX as ::core::ffi::c_int {
            (*h).stat.i_mb_count[(*h).sh.i_type as usize][i_0 as usize] +=
                (*h).stat.frame.i_mb_count[i_0 as usize] as crate::stdlib::int64_t;
            i_0 += 1;
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < 2 as ::core::ffi::c_int {
            (*h).stat.i_mb_count_8x8dct[i_1 as usize] +=
                (*h).stat.frame.i_mb_count_8x8dct[i_1 as usize] as crate::stdlib::int64_t;
            i_1 += 1;
        }
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < 6 as ::core::ffi::c_int {
            (*h).stat.i_mb_cbp[i_2 as usize] +=
                (*h).stat.frame.i_mb_cbp[i_2 as usize] as crate::stdlib::int64_t;
            i_2 += 1;
        }
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < 4 as ::core::ffi::c_int {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < 13 as ::core::ffi::c_int {
                (*h).stat.i_mb_pred_mode[i_3 as usize][j as usize] += (*h).stat.frame.i_mb_pred_mode
                    [i_3 as usize][j as usize]
                    as crate::stdlib::int64_t;
                j += 1;
            }
            i_3 += 1;
        }
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_4 < crate::src::common::macroblock::X264_PARTTYPE_MAX as ::core::ffi::c_int {
                (*h).stat.i_mb_partition[(*h).sh.i_type as usize][i_4 as usize] +=
                    (*h).stat.frame.i_mb_partition[i_4 as usize] as crate::stdlib::int64_t;
                i_4 += 1;
            }
            let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_list < 2 as ::core::ffi::c_int {
                let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_5 < crate::src::common::base::X264_REF_MAX * 2 as ::core::ffi::c_int {
                    (*h).stat.i_mb_count_ref[(*h).sh.i_type as usize][i_list as usize]
                        [i_5 as usize] += (*h).stat.frame.i_mb_count_ref[i_list as usize]
                        [i_5 as usize]
                        as crate::stdlib::int64_t;
                    i_5 += 1;
                }
                i_list += 1;
            }
        }
        let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_6 < 3 as ::core::ffi::c_int {
            (*h).stat.i_mb_field[i_6 as usize] +=
                (*h).stat.frame.i_mb_field[i_6 as usize] as crate::stdlib::int64_t;
            i_6 += 1;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
            && (*h).param.analyse.i_weighted_pred >= crate::x264_h::X264_WEIGHTP_SIMPLE
        {
            (*h).stat.i_wpred[0 as ::core::ffi::c_int as usize] +=
                !(*h).sh.weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null() as ::core::ffi::c_int;
            (*h).stat.i_wpred[1 as ::core::ffi::c_int as usize] += (!(*h).sh.weight
                [0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
                || !(*h).sh.weight[0 as ::core::ffi::c_int as usize]
                    [2 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null())
                as ::core::ffi::c_int;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            (*h).stat.i_direct_frames[(*h).sh.b_direct_spatial_mv_pred as usize] += 1;
            if (*h).mb.b_direct_auto_write != 0 {
                if (*h).stat.i_direct_score[0 as ::core::ffi::c_int as usize]
                    + (*h).stat.i_direct_score[1 as ::core::ffi::c_int as usize]
                    > (*h).mb.i_mb_count
                {
                    let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_7 < 2 as ::core::ffi::c_int {
                        (*h).stat.i_direct_score[i_7 as usize] =
                            (*h).stat.i_direct_score[i_7 as usize] * 9 as ::core::ffi::c_int
                                / 10 as ::core::ffi::c_int;
                        i_7 += 1;
                    }
                }
                let mut i_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_8 < 2 as ::core::ffi::c_int {
                    (*h).stat.i_direct_score[i_8 as usize] +=
                        (*h).stat.frame.i_direct_score[i_8 as usize];
                    i_8 += 1;
                }
            }
        } else {
            (*h).stat.i_consecutive_bframes[(*(*h).fenc).i_bframes as usize] += 1;
        }
        psz_message[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        let mut dur: ::core::ffi::c_double = (*(*h).fenc).f_duration as ::core::ffi::c_double;
        (*h).stat.f_frame_duration[(*h).sh.i_type as usize] += dur;
        if (*h).param.analyse.b_psnr != 0 {
            let mut ssd: [crate::stdlib::int64_t; 3] = [
                (*h).stat.frame.i_ssd[0 as ::core::ffi::c_int as usize],
                (*h).stat.frame.i_ssd[1 as ::core::ffi::c_int as usize],
                (*h).stat.frame.i_ssd[2 as ::core::ffi::c_int as usize],
            ];
            let mut luma_size: ::core::ffi::c_int = (*h).param.i_width * (*h).param.i_height;
            let mut chroma_size: ::core::ffi::c_int =
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    luma_size
                        >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                            || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            + (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
            (*pic_out).prop.f_psnr[0 as ::core::ffi::c_int as usize] = calc_psnr(
                ssd[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                luma_size as ::core::ffi::c_double,
            );
            (*pic_out).prop.f_psnr[1 as ::core::ffi::c_int as usize] = calc_psnr(
                ssd[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                chroma_size as ::core::ffi::c_double,
            );
            (*pic_out).prop.f_psnr[2 as ::core::ffi::c_int as usize] = calc_psnr(
                ssd[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                chroma_size as ::core::ffi::c_double,
            );
            (*pic_out).prop.f_psnr_avg = calc_psnr(
                (ssd[0 as ::core::ffi::c_int as usize]
                    + ssd[1 as ::core::ffi::c_int as usize]
                    + ssd[2 as ::core::ffi::c_int as usize])
                    as ::core::ffi::c_double,
                (luma_size + chroma_size * 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            (*h).stat.f_ssd_global[(*h).sh.i_type as usize] += dur
                * (ssd[0 as ::core::ffi::c_int as usize]
                    + ssd[1 as ::core::ffi::c_int as usize]
                    + ssd[2 as ::core::ffi::c_int as usize])
                    as ::core::ffi::c_double;
            (*h).stat.f_psnr_average[(*h).sh.i_type as usize] += dur * (*pic_out).prop.f_psnr_avg;
            (*h).stat.f_psnr_mean_y[(*h).sh.i_type as usize] +=
                dur * (*pic_out).prop.f_psnr[0 as ::core::ffi::c_int as usize];
            (*h).stat.f_psnr_mean_u[(*h).sh.i_type as usize] +=
                dur * (*pic_out).prop.f_psnr[1 as ::core::ffi::c_int as usize];
            (*h).stat.f_psnr_mean_v[(*h).sh.i_type as usize] +=
                dur * (*pic_out).prop.f_psnr[2 as ::core::ffi::c_int as usize];
            crate::stdlib::snprintf(
                &raw mut psz_message as *mut ::core::ffi::c_char,
                80 as crate::__stddef_size_t_h::size_t,
                b" PSNR Y:%5.2f U:%5.2f V:%5.2f\0".as_ptr() as *const ::core::ffi::c_char,
                (*pic_out).prop.f_psnr[0 as ::core::ffi::c_int as usize],
                (*pic_out).prop.f_psnr[1 as ::core::ffi::c_int as usize],
                (*pic_out).prop.f_psnr[2 as ::core::ffi::c_int as usize],
            );
        }
        if (*h).param.analyse.b_ssim != 0 {
            (*pic_out).prop.f_ssim =
                (*h).stat.frame.f_ssim / (*h).stat.frame.i_ssim_cnt as ::core::ffi::c_double;
            (*h).stat.f_ssim_mean_y[(*h).sh.i_type as usize] += (*pic_out).prop.f_ssim * dur;
            let mut msg_len: ::core::ffi::c_int =
                crate::stdlib::strlen(&raw mut psz_message as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_int;
            crate::stdlib::snprintf(
                (&raw mut psz_message as *mut ::core::ffi::c_char).offset(msg_len as isize),
                (80 as ::core::ffi::c_int - msg_len) as crate::__stddef_size_t_h::size_t,
                b" SSIM Y:%.5f\0".as_ptr() as *const ::core::ffi::c_char,
                (*pic_out).prop.f_ssim,
            );
        }
        psz_message[79 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        crate::src::common::common::x264_8_log(

            h as *mut crate::src::common::common::x264_t,
            crate::x264_h::X264_LOG_DEBUG_1,
            b"frame=%4d QP=%.2f NAL=%d Slice:%c Poc:%-3d I:%-4d P:%-4d SKIP:%-4d size=%d bytes%s\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*h).i_frame,
            (*(*h).fdec).f_qp_avg_aq as ::core::ffi::c_double,
            (*h).i_nal_ref_idc,
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
                'I' as i32
            } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
                'P' as i32
            } else {
                'B' as i32
            },
            (*(*h).fdec).i_poc,
            (*h).stat.frame.i_mb_count_i,
            (*h).stat.frame.i_mb_count_p,
            (*h).stat.frame.i_mb_count_skip,
            frame_size,
            &raw mut psz_message as *mut ::core::ffi::c_char,
        );
        thread_sync_stat((*h).thread[0 as ::core::ffi::c_int as usize], h);
        thread_sync_stat(thread_current, h);
        let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_9 < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
            if !(*h).fref[0 as ::core::ffi::c_int as usize][i_9 as usize].is_null()
                && (*(*h).fref[0 as ::core::ffi::c_int as usize][i_9 as usize]).b_duplicate != 0
            {
                crate::src::common::frame::x264_8_frame_push_blank_unused(
                    h as *mut crate::src::common::common::x264_t,
                    (*h).fref[0 as ::core::ffi::c_int as usize][i_9 as usize]
                        as *mut crate::src::common::frame::x264_frame,
                );
                (*h).fref[0 as ::core::ffi::c_int as usize][i_9 as usize] =
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
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_close(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut i_yuv_size: crate::stdlib::int64_t = ((*h).param.i_width * (*h).param.i_height
            + 2 as ::core::ffi::c_int
                * (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    (*h).param.i_width * (*h).param.i_height
                        >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                            || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                            + (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as crate::stdlib::int64_t;
        let mut i_mb_count_size: [[crate::stdlib::int64_t; 7]; 2] = [
            [
                0 as ::core::ffi::c_int as crate::stdlib::int64_t,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [0; 7],
        ];
        let mut buf: [::core::ffi::c_char; 200] = [0; 200];
        let mut b_print_pcm: ::core::ffi::c_int = ((*h).stat.i_mb_count
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
        crate::src::encoder::lookahead::x264_8_lookahead_delete(
            h as *mut crate::src::common::common::x264_t,
        );
        if (*h).param.b_sliced_threads != 0 {
            threadpool_wait_all(h);
        }
        if (*h).param.i_threads > 1 as ::core::ffi::c_int {
            crate::src::common::threadpool::x264_8_threadpool_delete((*h).threadpool);
        }
        if (*h).param.i_lookahead_threads > 1 as ::core::ffi::c_int {
            crate::src::common::threadpool::x264_8_threadpool_delete((*h).lookaheadpool);
        }
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*h).i_thread_frames {
                if (*(*h).thread[i as usize]).b_thread_active != 0 {
                    '_c2rust_label: {
                        if (*(*(*h).thread[i as usize]).fenc).i_reference_count
                            == 1 as ::core::ffi::c_int
                        {
                        } else {
                            crate::stdlib::__assert_fail(
                                b"h->thread[i]->fenc->i_reference_count == 1\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                                4223 as ::core::ffi::c_uint,
                                b"void x264_8_encoder_close(x264_t *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    crate::src::common::frame::x264_8_frame_delete(
                        (*(*h).thread[i as usize]).fenc
                            as *mut crate::src::common::frame::x264_frame,
                    );
                }
                i += 1;
            }
            let mut thread_prev: *mut crate::src::common::common::x264_t =
                (*h).thread[(*h).i_thread_phase as usize];
            crate::src::encoder::ratecontrol::x264_8_thread_sync_ratecontrol(
                h as *mut crate::src::common::common::x264_t,
                thread_prev as *mut crate::src::common::common::x264_t,
                h as *mut crate::src::common::common::x264_t,
            );
            crate::src::encoder::ratecontrol::x264_8_thread_sync_ratecontrol(
                thread_prev as *mut crate::src::common::common::x264_t,
                thread_prev as *mut crate::src::common::common::x264_t,
                h as *mut crate::src::common::common::x264_t,
            );
            (*h).i_frame = (*thread_prev).i_frame + 1 as ::core::ffi::c_int - (*h).i_thread_frames;
        }
        (*h).i_frame += 1;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 3 as ::core::ffi::c_int {
            static mut slice_order: [crate::stdlib::uint8_t; 3] = [
                crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                    as crate::stdlib::uint8_t,
                crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                    as crate::stdlib::uint8_t,
                crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
                    as crate::stdlib::uint8_t,
            ];
            let mut i_slice: ::core::ffi::c_int = slice_order[i_0 as usize] as ::core::ffi::c_int;
            if (*h).stat.i_frame_count[i_slice as usize] > 0 as ::core::ffi::c_int {
                let mut i_count: ::core::ffi::c_int = (*h).stat.i_frame_count[i_slice as usize];
                let mut dur: ::core::ffi::c_double = (*h).stat.f_frame_duration[i_slice as usize];
                if (*h).param.analyse.b_psnr != 0 {
                    crate::src::common::common::x264_8_log(

                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_INFO,
                        b"frame %c:%-5d Avg QP:%5.2f  size:%6.0f  PSNR Mean Y:%5.2f U:%5.2f V:%5.2f Avg:%5.2f Global:%5.2f\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        slice_type_to_char[i_slice as usize] as ::core::ffi::c_int,
                        i_count,
                        (*h).stat.f_frame_qp[i_slice as usize]
                            / i_count as ::core::ffi::c_double,
                        (*h).stat.i_frame_size[i_slice as usize] as ::core::ffi::c_double
                            / i_count as ::core::ffi::c_double,
                        (*h).stat.f_psnr_mean_y[i_slice as usize] / dur,
                        (*h).stat.f_psnr_mean_u[i_slice as usize] / dur,
                        (*h).stat.f_psnr_mean_v[i_slice as usize] / dur,
                        (*h).stat.f_psnr_average[i_slice as usize] / dur,
                        calc_psnr(
                            (*h).stat.f_ssd_global[i_slice as usize],
                            dur * i_yuv_size as ::core::ffi::c_double,
                        ),
                    );
                } else {
                    crate::src::common::common::x264_8_log(
                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_INFO,
                        b"frame %c:%-5d Avg QP:%5.2f  size:%6.0f\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        slice_type_to_char[i_slice as usize] as ::core::ffi::c_int,
                        i_count,
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
            let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
            let mut den: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 <= (*h).param.i_bframe {
                den +=
                    (i_1 + 1 as ::core::ffi::c_int) * (*h).stat.i_consecutive_bframes[i_1 as usize];
                i_1 += 1;
            }
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 <= (*h).param.i_bframe {
                p = p.offset(crate::stdlib::sprintf(
                    p,
                    b" %4.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                    100.0f64
                        * (i_2 + 1 as ::core::ffi::c_int) as ::core::ffi::c_double
                        * (*h).stat.i_consecutive_bframes[i_2 as usize] as ::core::ffi::c_double
                        / den as ::core::ffi::c_double,
                ) as isize);
                i_2 += 1;
            }
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_INFO,
                b"consecutive B-frames:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
        }
        let mut i_type: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_type < 2 as ::core::ffi::c_int {
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
            > 0 as ::core::ffi::c_int
        {
            let mut i_mb_count: *mut crate::stdlib::int64_t =
                &raw mut *(&raw mut (*h).stat.i_mb_count as *mut [crate::stdlib::int64_t; 19])
                    .offset(crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::int64_t;
            let mut i_count_0: ::core::ffi::c_double = (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                as ::core::ffi::c_double
                * (*h).mb.i_mb_count as ::core::ffi::c_double
                / 100.0f64;
            print_intra(
                i_mb_count,
                i_count_0,
                b_print_pcm,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_INFO,
                b"mb I  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
        }
        if (*h).stat.i_frame_count
            [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
            > 0 as ::core::ffi::c_int
        {
            let mut i_mb_count_0: *mut crate::stdlib::int64_t =
                &raw mut *(&raw mut (*h).stat.i_mb_count as *mut [crate::stdlib::int64_t; 19])
                    .offset(crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::int64_t;
            let mut i_count_1: ::core::ffi::c_double = (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                as ::core::ffi::c_double
                * (*h).mb.i_mb_count as ::core::ffi::c_double
                / 100.0f64;
            let mut i_mb_size: *mut crate::stdlib::int64_t = &raw mut *(&raw mut i_mb_count_size
                as *mut [crate::stdlib::int64_t; 7])
                .offset(crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int64_t;
            print_intra(
                i_mb_count_0,
                i_count_1,
                b_print_pcm,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_INFO,
                b"mb P  %s  P16..4: %4.1f%% %4.1f%% %4.1f%% %4.1f%% %4.1f%%    skip:%4.1f%%\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
                *i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
                (*i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as isize)
                    + *i_mb_size.offset(
                        crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as isize,
                    )) as ::core::ffi::c_double
                    / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
                *i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
                (*i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as isize)
                    + *i_mb_size.offset(
                        crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as isize,
                    )) as ::core::ffi::c_double
                    / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
                *i_mb_size
                    .offset(crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
                *i_mb_count_0
                    .offset(crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / i_count_1,
            );
        }
        if (*h).stat.i_frame_count
            [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
            > 0 as ::core::ffi::c_int
        {
            let mut i_mb_count_1: *mut crate::stdlib::int64_t =
                &raw mut *(&raw mut (*h).stat.i_mb_count as *mut [crate::stdlib::int64_t; 19])
                    .offset(crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::int64_t;
            let mut i_count_2: ::core::ffi::c_double = (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                as ::core::ffi::c_double
                * (*h).mb.i_mb_count as ::core::ffi::c_double
                / 100.0f64;
            let mut i_mb_list_count: ::core::ffi::c_double = 0.;
            let mut i_mb_size_0: *mut crate::stdlib::int64_t = &raw mut *(&raw mut i_mb_count_size
                as *mut [crate::stdlib::int64_t; 7])
                .offset(crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int64_t;
            let mut list_count: [crate::stdlib::int64_t; 3] =
                [0 as ::core::ffi::c_int as crate::stdlib::int64_t, 0, 0];
            print_intra(
                i_mb_count_1,
                i_count_2,
                b_print_pcm,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_4 < crate::src::common::macroblock::X264_PARTTYPE_MAX as ::core::ffi::c_int {
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < 2 as ::core::ffi::c_int {
                    let mut l0: ::core::ffi::c_int = x264_mb_type_list_table[i_4 as usize]
                        [0 as ::core::ffi::c_int as usize][j as usize]
                        as ::core::ffi::c_int;
                    let mut l1: ::core::ffi::c_int = x264_mb_type_list_table[i_4 as usize]
                        [1 as ::core::ffi::c_int as usize][j as usize]
                        as ::core::ffi::c_int;
                    if l0 != 0 || l1 != 0 {
                        list_count[(l1 + l0 * l1) as usize] += (*h).stat.i_mb_count
                            [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                            [i_4 as usize]
                            * 2 as crate::stdlib::int64_t;
                    }
                    j += 1;
                }
                i_4 += 1;
            }
            list_count[0 as ::core::ffi::c_int as usize] += (*h).stat.i_mb_partition
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::D_L0_8x8 as ::core::ffi::c_int as usize];
            list_count[1 as ::core::ffi::c_int as usize] += (*h).stat.i_mb_partition
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::D_L1_8x8 as ::core::ffi::c_int as usize];
            list_count[2 as ::core::ffi::c_int as usize] += (*h).stat.i_mb_partition
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::D_BI_8x8 as ::core::ffi::c_int as usize];
            *i_mb_count_1
                .offset(crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int as isize) +=
                ((*h).stat.i_mb_partition
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int as usize]
                    + 2 as crate::stdlib::int64_t)
                    / 4 as crate::stdlib::int64_t;
            i_mb_list_count = (list_count[0 as ::core::ffi::c_int as usize]
                + list_count[1 as ::core::ffi::c_int as usize]
                + list_count[2 as ::core::ffi::c_int as usize])
                as ::core::ffi::c_double
                / 100.0f64;
            crate::stdlib::sprintf(
                (&raw mut buf as *mut ::core::ffi::c_char).offset(crate::stdlib::strlen(
                    &raw mut buf as *mut ::core::ffi::c_char,
                ) as isize),
                b"  B16..8: %4.1f%% %4.1f%% %4.1f%%  direct:%4.1f%%  skip:%4.1f%%\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *i_mb_size_0
                    .offset(crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_2 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
                (*i_mb_size_0
                    .offset(crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as isize)
                    + *i_mb_size_0.offset(
                        crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as isize,
                    )) as ::core::ffi::c_double
                    / (i_count_2 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
                *i_mb_size_0
                    .offset(crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / (i_count_2 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
                *i_mb_count_1
                    .offset(crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / i_count_2,
                *i_mb_count_1
                    .offset(crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double
                    / i_count_2,
            );
            if i_mb_list_count != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                crate::stdlib::sprintf(
                    (&raw mut buf as *mut ::core::ffi::c_char).offset(crate::stdlib::strlen(
                        &raw mut buf as *mut ::core::ffi::c_char,
                    )
                        as isize),
                    b"  L0:%4.1f%% L1:%4.1f%% BI:%4.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                    list_count[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        / i_mb_list_count,
                    list_count[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        / i_mb_list_count,
                    list_count[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        / i_mb_list_count,
                );
            }
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_INFO,
                b"mb B  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
        }
        crate::src::encoder::ratecontrol::x264_8_ratecontrol_summary(
            h as *mut crate::src::common::common::x264_t,
        );
        if (*h).stat.i_frame_count
            [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
            > 0 as ::core::ffi::c_int
        {
            let mut i_i8x8: crate::stdlib::int64_t = (*h).stat.i_mb_count
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                [crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int as usize]
                + (*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int as usize]
                + (*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int as usize];
            let mut i_intra: crate::stdlib::int64_t = i_i8x8
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
            let mut i_all_intra: crate::stdlib::int64_t = i_intra
                + ((*h).stat.i_mb_count
                    [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]
                    + (*h).stat.i_mb_count
                        [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                        [crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as usize]);
            let mut i_skip: crate::stdlib::int64_t = (*h).stat.i_mb_count
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
            let i_count_3: ::core::ffi::c_int = (*h).stat.i_frame_count
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize];
            let mut i_mb_count_2: crate::stdlib::int64_t =
                i_count_3 as crate::stdlib::int64_t * (*h).mb.i_mb_count as crate::stdlib::int64_t;
            let mut i_inter: crate::stdlib::int64_t = i_mb_count_2 - i_skip - i_all_intra;
            let duration: ::core::ffi::c_double = (*h).stat.f_frame_duration
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                + (*h).stat.f_frame_duration
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                + (*h).stat.f_frame_duration
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize];
            let mut f_bitrate: ::core::ffi::c_float = (((*h).stat.i_frame_size
                [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_size
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                + (*h).stat.i_frame_size
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize])
                as ::core::ffi::c_double
                / duration
                / 125 as ::core::ffi::c_int as ::core::ffi::c_double)
                as ::core::ffi::c_float;
            if (*h).param.b_interlaced != 0 {
                let mut fieldstats: *mut ::core::ffi::c_char =
                    &raw mut buf as *mut ::core::ffi::c_char;
                *fieldstats.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
                if i_inter != 0 {
                    fieldstats = fieldstats.offset(crate::stdlib::sprintf(
                        fieldstats,
                        b" inter:%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_mb_field[1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
                            * 100.0f64
                            / i_inter as ::core::ffi::c_double,
                    ) as isize);
                }
                if i_skip != 0 {
                    fieldstats = fieldstats.offset(crate::stdlib::sprintf(
                        fieldstats,
                        b" skip:%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_mb_field[2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
                            * 100.0f64
                            / i_skip as ::core::ffi::c_double,
                    ) as isize);
                }
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"field mbs: intra: %.1f%%%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*h).stat.i_mb_field[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        * 100.0f64
                        / i_all_intra as ::core::ffi::c_double,
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
            }
            if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                .b_transform_8x8_mode
                != 0
            {
                buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                if (*h).stat.i_mb_count_8x8dct[0 as ::core::ffi::c_int as usize] != 0 {
                    crate::stdlib::sprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b" inter:%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        100.0f64
                            * (*h).stat.i_mb_count_8x8dct[1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_double
                            / (*h).stat.i_mb_count_8x8dct[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_double,
                    );
                }
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"8x8 transform intra:%.1f%%%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    100.0f64 * i_i8x8 as ::core::ffi::c_double
                        / (if i_intra > 1 as crate::stdlib::int64_t {
                            i_intra
                        } else {
                            1 as crate::stdlib::int64_t
                        }) as ::core::ffi::c_double,
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
            }
            if ((*h).param.analyse.i_direct_mv_pred == crate::x264_h::X264_DIRECT_PRED_AUTO
                || (*h).stat.i_direct_frames[0 as ::core::ffi::c_int as usize] != 0
                    && (*h).stat.i_direct_frames[1 as ::core::ffi::c_int as usize] != 0)
                && (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                    != 0
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"direct mvs  spatial:%.1f%% temporal:%.1f%%\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*h).stat.i_direct_frames[1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / (*h).stat.i_frame_count
                            [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double,
                    (*h).stat.i_direct_frames[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / (*h).stat.i_frame_count
                            [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double,
                );
            }
            buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                let mut csize: ::core::ffi::c_int = if crate::src::common::base::CHROMA_444
                    as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                if i_mb_count_2 != i_all_intra {
                    crate::stdlib::sprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b" inter: %.1f%% %.1f%% %.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_mb_cbp[1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
                            * 100.0f64
                            / ((i_mb_count_2 - i_all_intra) * 4 as crate::stdlib::int64_t)
                                as ::core::ffi::c_double,
                        (*h).stat.i_mb_cbp[3 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
                            * 100.0f64
                            / ((i_mb_count_2 - i_all_intra) * csize as crate::stdlib::int64_t)
                                as ::core::ffi::c_double,
                        (*h).stat.i_mb_cbp[5 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
                            * 100.0f64
                            / ((i_mb_count_2 - i_all_intra) * csize as crate::stdlib::int64_t)
                                as ::core::ffi::c_double,
                    );
                }
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"coded y,%s,%s intra: %.1f%% %.1f%% %.1f%%%s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    {
                        b"u\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"uvDC\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    {
                        b"v\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"uvAC\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    (*h).stat.i_mb_cbp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        * 100.0f64
                        / (i_all_intra * 4 as crate::stdlib::int64_t) as ::core::ffi::c_double,
                    (*h).stat.i_mb_cbp[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        * 100.0f64
                        / (i_all_intra * csize as crate::stdlib::int64_t) as ::core::ffi::c_double,
                    (*h).stat.i_mb_cbp[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        * 100.0f64
                        / (i_all_intra * csize as crate::stdlib::int64_t) as ::core::ffi::c_double,
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
            } else {
                if i_mb_count_2 != i_all_intra {
                    crate::stdlib::sprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b" inter: %.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_mb_cbp[1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
                            * 100.0f64
                            / ((i_mb_count_2 - i_all_intra) * 4 as crate::stdlib::int64_t)
                                as ::core::ffi::c_double,
                    );
                }
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"coded y intra: %.1f%%%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*h).stat.i_mb_cbp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        * 100.0f64
                        / (i_all_intra * 4 as crate::stdlib::int64_t) as ::core::ffi::c_double,
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
            }
            let mut fixed_pred_modes: [[crate::stdlib::int64_t; 9]; 4] = [
                [
                    0 as ::core::ffi::c_int as crate::stdlib::int64_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [0; 9],
                [0; 9],
                [0; 9],
            ];
            let mut sum_pred_modes: [crate::stdlib::int64_t; 4] =
                [0 as ::core::ffi::c_int as crate::stdlib::int64_t, 0, 0, 0];
            let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_5 <= crate::src::common::predict::I_PRED_16x16_DC_128 as ::core::ffi::c_int {
                fixed_pred_modes[0 as ::core::ffi::c_int as usize]
                    [x264_mb_pred_mode16x16_fix[i_5 as usize] as usize] +=
                    (*h).stat.i_mb_pred_mode[0 as ::core::ffi::c_int as usize][i_5 as usize];
                sum_pred_modes[0 as ::core::ffi::c_int as usize] +=
                    (*h).stat.i_mb_pred_mode[0 as ::core::ffi::c_int as usize][i_5 as usize];
                i_5 += 1;
            }
            if sum_pred_modes[0 as ::core::ffi::c_int as usize] != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"i16 v,h,dc,p: %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    fixed_pred_modes[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / sum_pred_modes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[0 as ::core::ffi::c_int as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / sum_pred_modes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[0 as ::core::ffi::c_int as usize]
                        [2 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / sum_pred_modes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[0 as ::core::ffi::c_int as usize]
                        [3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / sum_pred_modes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                );
            }
            let mut i_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i_6 <= 2 as ::core::ffi::c_int {
                let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j_0 <= crate::src::common::predict::I_PRED_8x8_DC_128 as ::core::ffi::c_int {
                    fixed_pred_modes[i_6 as usize][x264_mb_pred_mode4x4_fix
                        [(j_0 + 1 as ::core::ffi::c_int) as usize]
                        as usize] += (*h).stat.i_mb_pred_mode[i_6 as usize][j_0 as usize];
                    sum_pred_modes[i_6 as usize] +=
                        (*h).stat.i_mb_pred_mode[i_6 as usize][j_0 as usize];
                    j_0 += 1;
                }
                if sum_pred_modes[i_6 as usize] != 0 {
                    crate::src::common::common::x264_8_log(

                        h as *mut crate::src::common::common::x264_t,
                        crate::x264_h::X264_LOG_INFO,
                        b"i%d v,h,dc,ddl,ddr,vr,hd,vl,hu: %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (3 as ::core::ffi::c_int - i_6) * 4 as ::core::ffi::c_int,
                        fixed_pred_modes[i_6 as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                        fixed_pred_modes[i_6 as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                        fixed_pred_modes[i_6 as usize][2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                        fixed_pred_modes[i_6 as usize][3 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                        fixed_pred_modes[i_6 as usize][4 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                        fixed_pred_modes[i_6 as usize][5 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                        fixed_pred_modes[i_6 as usize][6 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                        fixed_pred_modes[i_6 as usize][7 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                        fixed_pred_modes[i_6 as usize][8 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double * 100.0f64
                            / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    );
                }
                i_6 += 1;
            }
            let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_7 <= crate::src::common::predict::I_PRED_CHROMA_DC_128 as ::core::ffi::c_int {
                fixed_pred_modes[3 as ::core::ffi::c_int as usize]
                    [x264_mb_chroma_pred_mode_fix[i_7 as usize] as usize] +=
                    (*h).stat.i_mb_pred_mode[3 as ::core::ffi::c_int as usize][i_7 as usize];
                sum_pred_modes[3 as ::core::ffi::c_int as usize] +=
                    (*h).stat.i_mb_pred_mode[3 as ::core::ffi::c_int as usize][i_7 as usize];
                i_7 += 1;
            }
            if sum_pred_modes[3 as ::core::ffi::c_int as usize] != 0
                && !(crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int)
            {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"i8c dc,h,v,p: %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    fixed_pred_modes[3 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / sum_pred_modes[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[3 as ::core::ffi::c_int as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / sum_pred_modes[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[3 as ::core::ffi::c_int as usize]
                        [2 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / sum_pred_modes[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[3 as ::core::ffi::c_int as usize]
                        [3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double
                        * 100.0f64
                        / sum_pred_modes[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
                );
            }
            if (*h).param.analyse.i_weighted_pred >= crate::x264_h::X264_WEIGHTP_SIMPLE
                && (*h).stat.i_frame_count
                    [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    > 0 as ::core::ffi::c_int
            {
                buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    crate::stdlib::sprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b" UV:%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).stat.i_wpred[1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
                            * 100.0f64
                            / (*h).stat.i_frame_count[crate::src::common::base::SLICE_TYPE_P
                                as ::core::ffi::c_int
                                as usize] as ::core::ffi::c_double,
                    );
                }
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"Weighted P-Frames: Y:%.1f%%%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*h).stat.i_wpred[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                        * 100.0f64
                        / (*h).stat.i_frame_count
                            [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double,
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
            }
            let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_list < 2 as ::core::ffi::c_int {
                let mut i_slice_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_slice_0 < 2 as ::core::ffi::c_int {
                    let mut p_0: *mut ::core::ffi::c_char =
                        &raw mut buf as *mut ::core::ffi::c_char;
                    let mut i_den: crate::stdlib::int64_t = 0 as crate::stdlib::int64_t;
                    let mut i_max: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut i_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_8 < crate::src::common::base::X264_REF_MAX * 2 as ::core::ffi::c_int {
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
                    if !(i_max == 0 as ::core::ffi::c_int) {
                        let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
                        crate::src::common::common::x264_8_log(
                            h as *mut crate::src::common::common::x264_t,
                            crate::x264_h::X264_LOG_INFO,
                            b"ref %c L%d:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"PB\0")
                                [i_slice_0 as usize]
                                as ::core::ffi::c_int,
                            i_list,
                            &raw mut buf as *mut ::core::ffi::c_char,
                        );
                    }
                    i_slice_0 += 1;
                }
                i_list += 1;
            }
            if (*h).param.analyse.b_ssim != 0 {
                let mut ssim: ::core::ffi::c_float = (((*h).stat.f_ssim_mean_y
                    [crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    + (*h).stat.f_ssim_mean_y
                        [crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    + (*h).stat.f_ssim_mean_y
                        [crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int as usize])
                    / duration)
                    as ::core::ffi::c_float;
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"SSIM Mean Y:%.7f (%6.3fdb)\n\0".as_ptr() as *const ::core::ffi::c_char,
                    ssim as ::core::ffi::c_double,
                    calc_ssim_db(ssim as ::core::ffi::c_double),
                );
            }
            if (*h).param.analyse.b_psnr != 0 {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"PSNR Mean Y:%6.3f U:%6.3f V:%6.3f Avg:%6.3f Global:%6.3f kb/s:%.2f\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
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
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_INFO,
                    b"kb/s:%.2f\n\0".as_ptr() as *const ::core::ffi::c_char,
                    f_bitrate as ::core::ffi::c_double,
                );
            }
        }
        crate::src::encoder::ratecontrol::x264_8_ratecontrol_delete(
            h as *mut crate::src::common::common::x264_t,
        );
        crate::src::common::base::x264_param_cleanup(
            &raw mut (*h).param as *mut _ as *mut crate::x264_h::x264_param_t,
        );
        crate::src::common::set::x264_8_cqm_delete(h as *mut crate::src::common::common::x264_t);
        crate::src::common::base::x264_free((*h).nal_buffer as *mut ::core::ffi::c_void);
        crate::src::common::base::x264_free((*h).reconfig_h as *mut ::core::ffi::c_void);
        crate::src::encoder::analyse::x264_8_analyse_free_costs(
            h as *mut crate::src::common::common::x264_t,
        );
        crate::src::common::base::x264_free((*h).cost_table as *mut ::core::ffi::c_void);
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            h = (*h).thread[(*h).i_thread_phase as usize];
        }
        crate::src::common::frame::x264_8_frame_delete_list(
            (*h).frames.unused[0 as ::core::ffi::c_int as usize]
                as *mut *mut crate::src::common::frame::x264_frame,
        );
        crate::src::common::frame::x264_8_frame_delete_list(
            (*h).frames.unused[1 as ::core::ffi::c_int as usize]
                as *mut *mut crate::src::common::frame::x264_frame,
        );
        crate::src::common::frame::x264_8_frame_delete_list(
            (*h).frames.current as *mut *mut crate::src::common::frame::x264_frame,
        );
        crate::src::common::frame::x264_8_frame_delete_list(
            (*h).frames.blank_unused as *mut *mut crate::src::common::frame::x264_frame,
        );
        h = (*h).thread[0 as ::core::ffi::c_int as usize];
        let mut i_10: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_10 < (*h).i_thread_frames {
            if (*(*h).thread[i_10 as usize]).b_thread_active != 0 {
                let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j_1 < (*(*h).thread[i_10 as usize]).i_ref[0 as ::core::ffi::c_int as usize] {
                    if !(*(*h).thread[i_10 as usize]).fref[0 as ::core::ffi::c_int as usize]
                        [j_1 as usize]
                        .is_null()
                        && (*(*(*h).thread[i_10 as usize]).fref[0 as ::core::ffi::c_int as usize]
                            [j_1 as usize])
                            .b_duplicate
                            != 0
                    {
                        crate::src::common::frame::x264_8_frame_delete(
                            (*(*h).thread[i_10 as usize]).fref[0 as ::core::ffi::c_int as usize]
                                [j_1 as usize]
                                as *mut crate::src::common::frame::x264_frame,
                        );
                    }
                    j_1 += 1;
                }
            }
            i_10 += 1;
        }
        if (*h).param.i_lookahead_threads > 1 as ::core::ffi::c_int {
            let mut i_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_11 < (*h).param.i_lookahead_threads {
                crate::src::common::base::x264_free(
                    (*h).lookahead_thread[i_11 as usize] as *mut ::core::ffi::c_void,
                );
                i_11 += 1;
            }
        }
        let mut i_12: ::core::ffi::c_int = (*h).param.i_threads - 1 as ::core::ffi::c_int;
        while i_12 >= 0 as ::core::ffi::c_int {
            let mut frame: *mut *mut crate::src::common::frame::x264_frame_t =
                ::core::ptr::null_mut::<*mut crate::src::common::frame::x264_frame_t>();
            if (*h).param.b_sliced_threads == 0 || i_12 == 0 as ::core::ffi::c_int {
                frame = &raw mut (**(&raw mut (*h).thread
                    as *mut *mut crate::src::common::common::x264_t)
                    .offset(i_12 as isize))
                .frames
                .reference
                    as *mut *mut crate::src::common::frame::x264_frame_t;
                while !(*frame).is_null() {
                    '_c2rust_label_0: {
                        if (**frame).i_reference_count > 0 as ::core::ffi::c_int {
                        } else {
                            crate::stdlib::__assert_fail(
                                b"(*frame)->i_reference_count > 0\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                                4552 as ::core::ffi::c_uint,
                                b"void x264_8_encoder_close(x264_t *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    (**frame).i_reference_count -= 1;
                    if (**frame).i_reference_count == 0 as ::core::ffi::c_int {
                        crate::src::common::frame::x264_8_frame_delete(
                            *frame as *mut crate::src::common::frame::x264_frame,
                        );
                    }
                    frame = frame.offset(1);
                }
                frame = &raw mut (**(&raw mut (*h).thread
                    as *mut *mut crate::src::common::common::x264_t)
                    .offset(i_12 as isize))
                .fdec;
                if !(*frame).is_null() {
                    '_c2rust_label_1: {
                        if (**frame).i_reference_count > 0 as ::core::ffi::c_int {
                        } else {
                            crate::stdlib::__assert_fail(
                                b"(*frame)->i_reference_count > 0\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"encoder/encoder.c\0".as_ptr() as *const ::core::ffi::c_char,
                                4560 as ::core::ffi::c_uint,
                                b"void x264_8_encoder_close(x264_t *)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    (**frame).i_reference_count -= 1;
                    if (**frame).i_reference_count == 0 as ::core::ffi::c_int {
                        crate::src::common::frame::x264_8_frame_delete(
                            *frame as *mut crate::src::common::frame::x264_frame,
                        );
                    }
                }
                crate::src::common::macroblock::x264_8_macroblock_cache_free(
                    (*h).thread[i_12 as usize] as *mut crate::src::common::common::x264_t,
                );
            }
            crate::src::common::macroblock::x264_8_macroblock_thread_free(
                (*h).thread[i_12 as usize] as *mut crate::src::common::common::x264_t,
                0 as ::core::ffi::c_int,
            );
            crate::src::common::base::x264_free(
                (*(*h).thread[i_12 as usize]).out.p_bitstream as *mut ::core::ffi::c_void,
            );
            crate::src::common::base::x264_free(
                (*(*h).thread[i_12 as usize]).out.nal as *mut ::core::ffi::c_void,
            );
            crate::stdlib::pthread_mutex_destroy(
                &raw mut (**(&raw mut (*h).thread
                    as *mut *mut crate::src::common::common::x264_t)
                    .offset(i_12 as isize))
                .mutex,
            );
            crate::stdlib::pthread_cond_destroy(
                &raw mut (**(&raw mut (*h).thread
                    as *mut *mut crate::src::common::common::x264_t)
                    .offset(i_12 as isize))
                .cv,
            );
            crate::src::common::base::x264_free(
                (*h).thread[i_12 as usize] as *mut ::core::ffi::c_void,
            );
            i_12 -= 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_delayed_frames(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut delayed_frames: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*h).i_thread_frames {
                delayed_frames += (*(*h).thread[i as usize]).b_thread_active;
                i += 1;
            }
            h = (*h).thread[(*h).i_thread_phase as usize];
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
#[no_mangle]

pub unsafe extern "C" fn x264_8_encoder_maximum_delayed_frames(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        return (*h).frames.i_delay;
    }
}
