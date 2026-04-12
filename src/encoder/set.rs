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
    pub unsafe extern "C" fn bs_write32(
        mut s: *mut crate::src::common::bitstream::bs_t,
        mut i_bits: crate::stdlib::uint32_t,
    ) {
        unsafe {
            bs_write(s, 16i32, i_bits >> 16i32);
            bs_write(s, 16i32, i_bits);
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
    pub unsafe extern "C" fn bs_align_10(mut s: *mut crate::src::common::bitstream::bs_t) {
        unsafe {
            if (*s).i_left & 7i32 != 0 {
                bs_write(
                    s,
                    (*s).i_left & 7i32,
                    ((1i32) << (((*s).i_left & 7i32) - 1i32)) as crate::stdlib::uint32_t,
                );
            }
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
    pub unsafe extern "C" fn bs_size_se(mut val: ::core::ffi::c_int) -> ::core::ffi::c_int {
        unsafe {
            let mut tmp = 1i32 - val * 2i32;
            if tmp < 0i32 {
                tmp = val * 2i32;
            }
            if tmp < 256i32 {
                x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int
            } else {
                x264_ue_size_tab[(tmp >> 8i32) as usize] as ::core::ffi::c_int + 16i32
            }
        }
    }
    use crate::src::encoder::set::osdep_h::endian_fix;
    use crate::src::encoder::set::osdep_h::endian_fix32;
}
pub mod osdep_h {
    #[inline(always)]
    pub extern "C" fn endian_fix32(mut x: crate::stdlib::uint32_t) -> crate::stdlib::uint32_t {
        (x << 24i32)
            .wrapping_add(x << 8i32 & 0xff0000u32)
            .wrapping_add(x >> 8i32 & 0xff00u32)
            .wrapping_add(x >> 24i32)
    }
    #[inline(always)]
    pub extern "C" fn endian_fix64(mut x: crate::stdlib::uint64_t) -> crate::stdlib::uint64_t {
        (endian_fix32((x >> 32i32) as crate::stdlib::uint32_t) as crate::stdlib::uint64_t)
            .wrapping_add(
                (endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uint64_t) << 32i32,
            )
    }
    #[inline(always)]
    pub extern "C" fn endian_fix(mut x: crate::stdlib::uintptr_t) -> crate::stdlib::uintptr_t {
        if crate::osdep_h::WORD_SIZE == 8i32 {
            endian_fix64(x as crate::stdlib::uint64_t) as crate::stdlib::uintptr_t
        } else {
            endian_fix32(x as crate::stdlib::uint32_t) as crate::stdlib::uintptr_t
        }
    }
}
pub mod macroblock_h {
    pub static mut x264_zigzag_scan4: [[crate::stdlib::uint8_t; 16]; 2] = [
        [
            0u8, 4u8, 1u8, 2u8, 5u8, 8u8, 12u8, 9u8, 6u8, 3u8, 7u8, 10u8, 13u8, 14u8, 11u8, 15u8,
        ],
        [
            0u8, 1u8, 4u8, 2u8, 3u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
        ],
    ];
    pub static mut x264_zigzag_scan8: [[crate::stdlib::uint8_t; 64]; 2] = [
        [
            0u8, 8u8, 1u8, 2u8, 9u8, 16u8, 24u8, 17u8, 10u8, 3u8, 4u8, 11u8, 18u8, 25u8, 32u8,
            40u8, 33u8, 26u8, 19u8, 12u8, 5u8, 6u8, 13u8, 20u8, 27u8, 34u8, 41u8, 48u8, 56u8, 49u8,
            42u8, 35u8, 28u8, 21u8, 14u8, 7u8, 15u8, 22u8, 29u8, 36u8, 43u8, 50u8, 57u8, 58u8,
            51u8, 44u8, 37u8, 30u8, 23u8, 31u8, 38u8, 45u8, 52u8, 59u8, 60u8, 53u8, 46u8, 39u8,
            47u8, 54u8, 61u8, 62u8, 55u8, 63u8,
        ],
        [
            0u8, 1u8, 2u8, 8u8, 9u8, 3u8, 4u8, 10u8, 16u8, 11u8, 5u8, 6u8, 7u8, 12u8, 17u8, 24u8,
            18u8, 13u8, 14u8, 15u8, 19u8, 25u8, 32u8, 26u8, 20u8, 21u8, 22u8, 23u8, 27u8, 33u8,
            40u8, 34u8, 28u8, 29u8, 30u8, 31u8, 35u8, 41u8, 48u8, 42u8, 36u8, 37u8, 38u8, 39u8,
            43u8, 49u8, 50u8, 44u8, 45u8, 46u8, 47u8, 51u8, 56u8, 57u8, 52u8, 53u8, 54u8, 55u8,
            58u8, 59u8, 60u8, 61u8, 62u8, 63u8,
        ],
    ];
}
use crate::src::common::base::ChromaFormat;
use crate::src::encoder::set::bitstream_h::bs_align_10;
use crate::src::encoder::set::bitstream_h::bs_flush;
use crate::src::encoder::set::bitstream_h::bs_init;
use crate::src::encoder::set::bitstream_h::bs_pos;
use crate::src::encoder::set::bitstream_h::bs_rbsp_trailing;
use crate::src::encoder::set::bitstream_h::bs_realign;
use crate::src::encoder::set::bitstream_h::bs_size_se;
use crate::src::encoder::set::bitstream_h::bs_write;
use crate::src::encoder::set::bitstream_h::bs_write_se;
use crate::src::encoder::set::bitstream_h::bs_write_ue_big;
use crate::src::encoder::set::bitstream_h::bs_write1;
use crate::src::encoder::set::bitstream_h::bs_write32;
use crate::src::encoder::set::macroblock_h::x264_zigzag_scan4;
use crate::src::encoder::set::macroblock_h::x264_zigzag_scan8;
use crate::x264_h::X264_CSP_BGR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub w: crate::stdlib::uint8_t,
    pub h: crate::stdlib::uint8_t,
    pub sar: crate::stdlib::uint8_t,
}
static mut num_clock_ts: [crate::stdlib::uint8_t; 10] =
    [0u8, 1u8, 1u8, 1u8, 2u8, 2u8, 3u8, 3u8, 2u8, 3u8];
static mut avcintra_uuid: [crate::stdlib::uint8_t; 16] = [
    0xf7u8, 0x49u8, 0x3eu8, 0xb3u8, 0xd4u8, 0u8, 0x47u8, 0x96u8, 0x86u8, 0x86u8, 0xc9u8, 0x70u8,
    0x7bu8, 0x64u8, 0x37u8, 0x2au8,
];
unsafe extern "C" fn transpose(mut buf: *mut crate::stdlib::uint8_t, mut w: ::core::ffi::c_int) {
    unsafe {
        let mut i = 0i32;
        while i < w {
            let mut j = 0i32;
            while j < i {
                let mut t = *buf.offset((w * i + j) as isize);
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
    sps: &crate::src::common::set::x264_sps_t,
    mut idx: ::core::ffi::c_int,
) {
    unsafe {
        let len = if idx < 4i32 { 16i32 } else { 64i32 };
        let mut zigzag = if idx < 4i32 {
            &raw const *(&raw const x264_zigzag_scan4 as *const [crate::stdlib::uint8_t; 16])
                .offset(0isize) as *const crate::stdlib::uint8_t
        } else {
            &raw const *(&raw const x264_zigzag_scan8 as *const [crate::stdlib::uint8_t; 64])
                .offset(0isize) as *const crate::stdlib::uint8_t
        };
        let mut list = sps.scaling_list[idx as usize];
        let mut def_list = if idx == crate::src::common::set::CQM_4IC as ::core::ffi::c_int {
            sps.scaling_list[crate::src::common::set::CQM_4IY as ::core::ffi::c_int as usize]
        } else if idx == crate::src::common::set::CQM_4PC as ::core::ffi::c_int {
            sps.scaling_list[crate::src::common::set::CQM_4PY as ::core::ffi::c_int as usize]
        } else if idx == crate::src::common::set::CQM_8IC as ::core::ffi::c_int + 4i32 {
            sps.scaling_list
                [(crate::src::common::set::CQM_8IY as ::core::ffi::c_int + 4i32) as usize]
        } else if idx == crate::src::common::set::CQM_8PC as ::core::ffi::c_int + 4i32 {
            sps.scaling_list
                [(crate::src::common::set::CQM_8PY as ::core::ffi::c_int + 4i32) as usize]
        } else {
            crate::src::common::tables::x264_cqm_jvt[idx as usize]
        };
        if crate::stdlib::memcmp(
            list as *const ::core::ffi::c_void,
            def_list as *const ::core::ffi::c_void,
            len as crate::__stddef_size_t_h::size_t,
        ) == 0
        {
            bs_write1(s, 0u32);
        } else if crate::stdlib::memcmp(
            list as *const ::core::ffi::c_void,
            crate::src::common::tables::x264_cqm_jvt[idx as usize] as *const ::core::ffi::c_void,
            len as crate::__stddef_size_t_h::size_t,
        ) == 0
        {
            bs_write1(s, 1u32);
            bs_write_se(s, -(8i32));
        } else {
            let mut j = 0i32;
            bs_write1(s, 1u32);
            let mut run = len;
            while run > 1i32 {
                if *list.offset(*zigzag.offset((run - 1i32) as isize) as isize)
                    as ::core::ffi::c_int
                    != *list.offset(*zigzag.offset((run - 2i32) as isize) as isize)
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
            while j < run {
                bs_write_se(
                    s,
                    (*list.offset(*zigzag.offset(j as isize) as isize) as ::core::ffi::c_int
                        - (if j > 0i32 {
                            *list.offset(*zigzag.offset((j - 1i32) as isize) as isize)
                                as ::core::ffi::c_int
                        } else {
                            8i32
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
pub unsafe extern "C" fn x264_8_sei_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut payload: *mut crate::stdlib::uint8_t,
    mut payload_size: ::core::ffi::c_int,
    mut payload_type: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        bs_realign(s);
        while i <= payload_type - 255i32 {
            bs_write(s, 8i32, 255u32);
            i += 255i32;
        }
        bs_write(s, 8i32, (payload_type - i) as crate::stdlib::uint32_t);
        i = 0i32;
        while i <= payload_size - 255i32 {
            bs_write(s, 8i32, 255u32);
            i += 255i32;
        }
        bs_write(s, 8i32, (payload_size - i) as crate::stdlib::uint32_t);
        i = 0i32;
        while i < payload_size {
            bs_write(
                s,
                8i32,
                *payload.offset(i as isize) as crate::stdlib::uint32_t,
            );
            i += 1;
        }
        bs_rbsp_trailing(s);
        bs_flush(s);
    }
}
pub unsafe extern "C" fn x264_8_sps_init(
    sps: &mut crate::src::common::set::x264_sps_t,
    mut i_id: ::core::ffi::c_int,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        let mut csp = (*param).i_csp & crate::x264_h::X264_CSP_MASK;
        sps.i_id = i_id;
        sps.i_mb_width = ((*param).i_width + 15i32) / 16i32;
        sps.i_mb_height = ((*param).i_height + 15i32) / 16i32;
        sps.frame_mbs_only = !((*param).interlaced || (*param).fake_interlaced);
        if !sps.frame_mbs_only {
            sps.i_mb_height = (sps.i_mb_height + 1i32) & !(1i32);
        }
        sps.i_chroma_format_idc = match csp {
            csp if csp >= crate::x264_h::X264_CSP_I444 => ChromaFormat::Chroma444,
            csp if csp >= crate::x264_h::X264_CSP_I422 => ChromaFormat::Chroma422,
            csp if csp >= crate::x264_h::X264_CSP_I420 => ChromaFormat::Chroma420,
            _ => ChromaFormat::Chroma400,
        };
        sps.qpprime_y_zero_transform_bypass = (*param).rc.i_rc_method == crate::x264_h::X264_RC_CQP
            && (*param).rc.i_qp_constant == 0i32;
        if sps.qpprime_y_zero_transform_bypass || sps.i_chroma_format_idc.is_444() {
            sps.i_profile_idc =
                crate::src::common::base::PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int;
        } else if sps.i_chroma_format_idc.is_422() {
            sps.i_profile_idc = crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int;
        } else if crate::internal::BIT_DEPTH > 8i32 {
            sps.i_profile_idc = crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int;
        } else if (*param).analyse.transform_8x8
            || (*param).i_cqm_preset != crate::x264_h::X264_CQM_FLAT
            || sps.i_chroma_format_idc.is_400()
        {
            sps.i_profile_idc = crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int;
        } else if (*param).cabac
            || (*param).i_bframe > 0i32
            || (*param).interlaced
            || (*param).fake_interlaced
            || (*param).analyse.i_weighted_pred > 0i32
        {
            sps.i_profile_idc = crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int;
        } else {
            sps.i_profile_idc = crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int;
        }
        sps.constraint_set0 =
            sps.i_profile_idc == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int;
        sps.constraint_set1 =
            sps.i_profile_idc <= crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int;
        sps.constraint_set2 = false;
        sps.constraint_set3 = false;
        sps.i_level_idc = (*param).i_level_idc;
        if (*param).i_level_idc == 9i32
            && (sps.i_profile_idc
                == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int
                || sps.i_profile_idc
                    == crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int)
        {
            sps.constraint_set3 = true;
            sps.i_level_idc = 11i32;
        }
        if (*param).i_keyint_max == 1i32
            && sps.i_profile_idc >= crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
        {
            sps.constraint_set3 = true;
        }
        sps.vui.i_num_reorder_frames = if (*param).i_bframe_pyramid != 0 {
            2i32
        } else if (*param).i_bframe != 0 {
            1i32
        } else {
            0i32
        };
        sps.i_num_ref_frames = if (16i32)
            < (if (*param).i_frame_reference
                > (if 1i32 + sps.vui.i_num_reorder_frames
                    > (if (if (*param).i_bframe_pyramid != 0 {
                        4i32
                    } else {
                        1i32
                    }) > (*param).i_dpb_size
                    {
                        if (*param).i_bframe_pyramid != 0 {
                            4i32
                        } else {
                            1i32
                        }
                    } else {
                        (*param).i_dpb_size
                    })
                {
                    1i32 + sps.vui.i_num_reorder_frames
                } else {
                    if (if (*param).i_bframe_pyramid != 0 {
                        4i32
                    } else {
                        1i32
                    }) > (*param).i_dpb_size
                    {
                        if (*param).i_bframe_pyramid != 0 {
                            4i32
                        } else {
                            1i32
                        }
                    } else {
                        (*param).i_dpb_size
                    }
                })
            {
                (*param).i_frame_reference
            } else {
                if 1i32 + sps.vui.i_num_reorder_frames
                    > (if (if (*param).i_bframe_pyramid != 0 {
                        4i32
                    } else {
                        1i32
                    }) > (*param).i_dpb_size
                    {
                        if (*param).i_bframe_pyramid != 0 {
                            4i32
                        } else {
                            1i32
                        }
                    } else {
                        (*param).i_dpb_size
                    })
                {
                    1i32 + sps.vui.i_num_reorder_frames
                } else {
                    if (if (*param).i_bframe_pyramid != 0 {
                        4i32
                    } else {
                        1i32
                    }) > (*param).i_dpb_size
                    {
                        if (*param).i_bframe_pyramid != 0 {
                            4i32
                        } else {
                            1i32
                        }
                    } else {
                        (*param).i_dpb_size
                    }
                }
            }) {
            16i32
        } else if (*param).i_frame_reference
            > (if 1i32 + sps.vui.i_num_reorder_frames
                > (if (if (*param).i_bframe_pyramid != 0 {
                    4i32
                } else {
                    1i32
                }) > (*param).i_dpb_size
                {
                    if (*param).i_bframe_pyramid != 0 {
                        4i32
                    } else {
                        1i32
                    }
                } else {
                    (*param).i_dpb_size
                })
            {
                1i32 + sps.vui.i_num_reorder_frames
            } else {
                if (if (*param).i_bframe_pyramid != 0 {
                    4i32
                } else {
                    1i32
                }) > (*param).i_dpb_size
                {
                    if (*param).i_bframe_pyramid != 0 {
                        4i32
                    } else {
                        1i32
                    }
                } else {
                    (*param).i_dpb_size
                }
            })
        {
            (*param).i_frame_reference
        } else if 1i32 + sps.vui.i_num_reorder_frames
            > (if (if (*param).i_bframe_pyramid != 0 {
                4i32
            } else {
                1i32
            }) > (*param).i_dpb_size
            {
                if (*param).i_bframe_pyramid != 0 {
                    4i32
                } else {
                    1i32
                }
            } else {
                (*param).i_dpb_size
            })
        {
            1i32 + sps.vui.i_num_reorder_frames
        } else if (if (*param).i_bframe_pyramid != 0 {
            4i32
        } else {
            1i32
        }) > (*param).i_dpb_size
        {
            if (*param).i_bframe_pyramid != 0 {
                4i32
            } else {
                1i32
            }
        } else {
            (*param).i_dpb_size
        };
        sps.vui.i_max_dec_frame_buffering = sps.i_num_ref_frames;
        sps.i_num_ref_frames -= ((*param).i_bframe_pyramid == crate::x264_h::X264_B_PYRAMID_STRICT)
            as ::core::ffi::c_int;
        if (*param).i_keyint_max == 1i32 {
            sps.i_num_ref_frames = 0i32;
            sps.vui.i_max_dec_frame_buffering = 0i32;
        }
        let mut max_frame_num = sps.vui.i_max_dec_frame_buffering
            * (((*param).i_bframe_pyramid != 0) as ::core::ffi::c_int + 1i32)
            + 1i32;
        if (*param).intra_refresh {
            let mut time_to_recovery = (if (sps.i_mb_width - 1i32) < (*param).i_keyint_max {
                sps.i_mb_width - 1i32
            } else {
                (*param).i_keyint_max
            }) + (*param).i_bframe
                - 1i32;
            max_frame_num = if max_frame_num > time_to_recovery + 1i32 {
                max_frame_num
            } else {
                time_to_recovery + 1i32
            };
        }
        sps.i_log2_max_frame_num = 4i32;
        while (1i32) << sps.i_log2_max_frame_num <= max_frame_num {
            sps.i_log2_max_frame_num += 1;
        }
        sps.i_poc_type =
            if (*param).i_bframe != 0 || (*param).interlaced || (*param).i_avcintra_class != 0 {
                0i32
            } else {
                2i32
            };
        if sps.i_poc_type == 0i32 {
            let mut max_delta_poc = ((*param).i_bframe + 2i32)
                * (((*param).i_bframe_pyramid != 0) as ::core::ffi::c_int + 1i32)
                * 2i32;
            sps.i_log2_max_poc_lsb = 4i32;
            while (1i32) << sps.i_log2_max_poc_lsb <= max_delta_poc * 2i32 {
                sps.i_log2_max_poc_lsb += 1;
            }
        }
        sps.has_vui = true;
        sps.gaps_in_frame_num_value_allowed = false;
        sps.mb_adaptive_frame_field = (*param).interlaced;
        sps.direct8x8_inference = true;
        x264_8_sps_init_reconfigurable(sps, param);
        sps.vui.overscan_info_present =
            (*param).vui.i_overscan > 0i32 && (*param).vui.i_overscan <= 2i32;
        if sps.vui.overscan_info_present {
            sps.vui.overscan_info = if (*param).vui.i_overscan == 2i32 {
                true
            } else {
                false
            };
        }
        sps.vui.signal_type_present = false;
        sps.vui.i_vidformat =
            if (*param).vui.i_vidformat >= 0i32 && (*param).vui.i_vidformat <= 5i32 {
                (*param).vui.i_vidformat
            } else {
                5i32
            };
        sps.vui.fullrange = (*param)
            .vui
            .fullrange
            .unwrap_or_else(|| csp >= X264_CSP_BGR);
        sps.vui.color_description_present = false;
        sps.vui.i_colorprim =
            if (*param).vui.i_colorprim >= 0i32 && (*param).vui.i_colorprim <= 12i32 {
                (*param).vui.i_colorprim
            } else {
                2i32
            };
        sps.vui.i_transfer = if (*param).vui.i_transfer >= 0i32 && (*param).vui.i_transfer <= 18i32
        {
            (*param).vui.i_transfer
        } else {
            2i32
        };
        sps.vui.i_colmatrix =
            if (*param).vui.i_colmatrix >= 0i32 && (*param).vui.i_colmatrix <= 14i32 {
                (*param).vui.i_colmatrix
            } else if csp >= crate::x264_h::X264_CSP_BGR {
                0i32
            } else {
                2i32
            };
        if sps.vui.i_colorprim != 2i32 || sps.vui.i_transfer != 2i32 || sps.vui.i_colmatrix != 2i32
        {
            sps.vui.color_description_present = true;
        }
        if sps.vui.i_vidformat != 5i32 || sps.vui.fullrange || sps.vui.color_description_present {
            sps.vui.signal_type_present = true;
        }
        sps.vui.chroma_loc_info_present = (*param).vui.i_chroma_loc > 0i32
            && (*param).vui.i_chroma_loc <= 5i32
            && sps.i_chroma_format_idc.is_420();
        if sps.vui.chroma_loc_info_present {
            sps.vui.i_chroma_loc_top = (*param).vui.i_chroma_loc;
            sps.vui.i_chroma_loc_bottom = (*param).vui.i_chroma_loc;
        }
        sps.vui.timing_info_present =
            (*param).i_timebase_num > 0u32 && (*param).i_timebase_den > 0u32;
        if sps.vui.timing_info_present {
            sps.vui.i_num_units_in_tick = (*param).i_timebase_num;
            sps.vui.i_time_scale = (*param).i_timebase_den.wrapping_mul(2u32);
            sps.vui.fixed_frame_rate = !(*param).vfr_input;
        }
        sps.vui.vcl_hrd_parameters_present = false;
        sps.vui.nal_hrd_parameters_present = (*param).i_nal_hrd != 0;
        sps.vui.pic_struct_present = (*param).pic_struct;
        sps.vui.bitstream_restriction = !(sps.constraint_set3
            && sps.i_profile_idc >= crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int);
        if sps.vui.bitstream_restriction {
            sps.vui.motion_vectors_over_pic_boundaries = true;
            sps.vui.i_max_bytes_per_pic_denom = 0i32;
            sps.vui.i_max_bits_per_mb_denom = 0i32;
            sps.vui.i_log2_max_mv_length_vertical = crate::stdlib::log2f(
                (if 1i32 > (*param).analyse.i_mv_range * 4i32 - 1i32 {
                    1i32
                } else {
                    (*param).analyse.i_mv_range * 4i32 - 1i32
                }) as ::core::ffi::c_float,
            ) as ::core::ffi::c_int
                + 1i32;
            sps.vui.i_log2_max_mv_length_horizontal = sps.vui.i_log2_max_mv_length_vertical;
        }
        sps.avcintra_hd = (*param).i_avcintra_class != 0 && (*param).i_avcintra_class <= 200i32;
        sps.avcintra_4k = (*param).i_avcintra_class > 200i32;
        sps.i_cqm_preset = (*param).i_cqm_preset;
    }
}
pub unsafe extern "C" fn x264_8_sps_init_reconfigurable(
    sps: &mut crate::src::common::set::x264_sps_t,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        sps.crop.i_left = (*param).crop_rect.i_left;
        sps.crop.i_top = (*param).crop_rect.i_top;
        sps.crop.i_right = (*param).crop_rect.i_right + sps.i_mb_width * 16i32 - (*param).i_width;
        sps.crop.i_bottom =
            (*param).crop_rect.i_bottom + sps.i_mb_height * 16i32 - (*param).i_height;
        sps.has_crop = sps.crop.i_left != 0
            || sps.crop.i_top != 0
            || sps.crop.i_right != 0
            || sps.crop.i_bottom != 0;
        sps.vui.aspect_ratio_info_present = false;
        if (*param).vui.i_sar_width > 0i32 && (*param).vui.i_sar_height > 0i32 {
            sps.vui.aspect_ratio_info_present = true;
            sps.vui.i_sar_width = (*param).vui.i_sar_width;
            sps.vui.i_sar_height = (*param).vui.i_sar_height;
        }
    }
}
pub unsafe extern "C" fn x264_8_sps_init_scaling_list(
    sps: &mut crate::src::common::set::x264_sps_t,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        match sps.i_cqm_preset {
            crate::x264_h::X264_CQM_FLAT => {
                let mut i = 0i32;
                while i < 8i32 {
                    sps.scaling_list[i as usize] =
                        &raw const crate::src::common::tables::x264_cqm_flat16
                            as *const crate::stdlib::uint8_t;
                    i += 1;
                }
            }
            crate::x264_h::X264_CQM_JVT_1 => {
                let mut i_0 = 0i32;
                while i_0 < 8i32 {
                    sps.scaling_list[i_0 as usize] =
                        crate::src::common::tables::x264_cqm_jvt[i_0 as usize];
                    i_0 += 1;
                }
            }
            crate::x264_h::X264_CQM_CUSTOM_1 => {
                let mut i_1 = 0i32;
                transpose(
                    &raw mut (*param).cqm_4iy as *mut crate::stdlib::uint8_t,
                    4i32,
                );
                transpose(
                    &raw mut (*param).cqm_4py as *mut crate::stdlib::uint8_t,
                    4i32,
                );
                transpose(
                    &raw mut (*param).cqm_4ic as *mut crate::stdlib::uint8_t,
                    4i32,
                );
                transpose(
                    &raw mut (*param).cqm_4pc as *mut crate::stdlib::uint8_t,
                    4i32,
                );
                transpose(
                    &raw mut (*param).cqm_8iy as *mut crate::stdlib::uint8_t,
                    8i32,
                );
                transpose(
                    &raw mut (*param).cqm_8py as *mut crate::stdlib::uint8_t,
                    8i32,
                );
                transpose(
                    &raw mut (*param).cqm_8ic as *mut crate::stdlib::uint8_t,
                    8i32,
                );
                transpose(
                    &raw mut (*param).cqm_8pc as *mut crate::stdlib::uint8_t,
                    8i32,
                );
                sps.scaling_list[crate::src::common::set::CQM_4IY as ::core::ffi::c_int as usize] =
                    &raw mut (*param).cqm_4iy as *mut crate::stdlib::uint8_t;
                sps.scaling_list[crate::src::common::set::CQM_4PY as ::core::ffi::c_int as usize] =
                    &raw mut (*param).cqm_4py as *mut crate::stdlib::uint8_t;
                sps.scaling_list[crate::src::common::set::CQM_4IC as ::core::ffi::c_int as usize] =
                    &raw mut (*param).cqm_4ic as *mut crate::stdlib::uint8_t;
                sps.scaling_list[crate::src::common::set::CQM_4PC as ::core::ffi::c_int as usize] =
                    &raw mut (*param).cqm_4pc as *mut crate::stdlib::uint8_t;
                sps.scaling_list
                    [(crate::src::common::set::CQM_8IY as ::core::ffi::c_int + 4i32) as usize] =
                    &raw mut (*param).cqm_8iy as *mut crate::stdlib::uint8_t;
                sps.scaling_list
                    [(crate::src::common::set::CQM_8PY as ::core::ffi::c_int + 4i32) as usize] =
                    &raw mut (*param).cqm_8py as *mut crate::stdlib::uint8_t;
                sps.scaling_list
                    [(crate::src::common::set::CQM_8IC as ::core::ffi::c_int + 4i32) as usize] =
                    &raw mut (*param).cqm_8ic as *mut crate::stdlib::uint8_t;
                sps.scaling_list
                    [(crate::src::common::set::CQM_8PC as ::core::ffi::c_int + 4i32) as usize] =
                    &raw mut (*param).cqm_8pc as *mut crate::stdlib::uint8_t;
                while i_1 < 8i32 {
                    let mut j = 0i32;
                    while j < (if i_1 < 4i32 { 16i32 } else { 64i32 }) {
                        if *sps.scaling_list[i_1 as usize].offset(j as isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            sps.scaling_list[i_1 as usize] =
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
pub unsafe extern "C" fn x264_8_sps_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    sps: &crate::src::common::set::x264_sps_t,
) {
    unsafe {
        bs_realign(s);
        bs_write(s, 8i32, sps.i_profile_idc as crate::stdlib::uint32_t);
        bs_write1(s, sps.constraint_set0 as crate::stdlib::uint32_t);
        bs_write1(s, sps.constraint_set1 as crate::stdlib::uint32_t);
        bs_write1(s, sps.constraint_set2 as crate::stdlib::uint32_t);
        bs_write1(s, sps.constraint_set3 as crate::stdlib::uint32_t);
        bs_write(s, 4i32, 0u32);
        bs_write(s, 8i32, sps.i_level_idc as crate::stdlib::uint32_t);
        bs_write_ue_big(s, sps.i_id as ::core::ffi::c_uint);
        if sps.i_profile_idc >= crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int {
            bs_write_ue_big(s, sps.i_chroma_format_idc as ::core::ffi::c_uint);
            if sps.i_chroma_format_idc.is_444() {
                bs_write1(s, 0u32);
            }
            bs_write_ue_big(
                s,
                (crate::internal::BIT_DEPTH - 8i32) as ::core::ffi::c_uint,
            );
            bs_write_ue_big(
                s,
                (crate::internal::BIT_DEPTH - 8i32) as ::core::ffi::c_uint,
            );
            bs_write1(
                s,
                sps.qpprime_y_zero_transform_bypass as crate::stdlib::uint32_t,
            );
            bs_write1(s, sps.avcintra_hd as crate::stdlib::uint32_t);
            if sps.avcintra_hd {
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
                bs_write1(s, 0u32);
                bs_write1(s, 0u32);
                bs_write1(s, 0u32);
                scaling_list_write(
                    s,
                    sps,
                    crate::src::common::set::CQM_8IY as ::core::ffi::c_int + 4i32,
                );
                bs_write1(s, 0u32);
                if sps.i_chroma_format_idc.is_444() {
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_8IC as ::core::ffi::c_int + 4i32,
                    );
                    bs_write1(s, 0u32);
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_8IC as ::core::ffi::c_int + 4i32,
                    );
                    bs_write1(s, 0u32);
                }
            }
        }
        bs_write_ue_big(s, (sps.i_log2_max_frame_num - 4i32) as ::core::ffi::c_uint);
        bs_write_ue_big(s, sps.i_poc_type as ::core::ffi::c_uint);
        if sps.i_poc_type == 0i32 {
            bs_write_ue_big(s, (sps.i_log2_max_poc_lsb - 4i32) as ::core::ffi::c_uint);
        }
        bs_write_ue_big(s, sps.i_num_ref_frames as ::core::ffi::c_uint);
        bs_write1(
            s,
            sps.gaps_in_frame_num_value_allowed as crate::stdlib::uint32_t,
        );
        bs_write_ue_big(s, (sps.i_mb_width - 1i32) as ::core::ffi::c_uint);
        bs_write_ue_big(
            s,
            ((sps.i_mb_height >> (!sps.frame_mbs_only) as ::core::ffi::c_int) - 1i32)
                as ::core::ffi::c_uint,
        );
        bs_write1(s, sps.frame_mbs_only as crate::stdlib::uint32_t);
        if !sps.frame_mbs_only {
            bs_write1(s, sps.mb_adaptive_frame_field as crate::stdlib::uint32_t);
        }
        bs_write1(s, sps.direct8x8_inference as crate::stdlib::uint32_t);
        bs_write1(s, sps.has_crop as crate::stdlib::uint32_t);
        if sps.has_crop {
            let mut h_shift = (sps.i_chroma_format_idc.is_420() || sps.i_chroma_format_idc.is_422())
                as ::core::ffi::c_int;
            let mut v_shift = (sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int
                + (!sps.frame_mbs_only) as ::core::ffi::c_int;
            bs_write_ue_big(s, (sps.crop.i_left >> h_shift) as ::core::ffi::c_uint);
            bs_write_ue_big(s, (sps.crop.i_right >> h_shift) as ::core::ffi::c_uint);
            bs_write_ue_big(s, (sps.crop.i_top >> v_shift) as ::core::ffi::c_uint);
            bs_write_ue_big(s, (sps.crop.i_bottom >> v_shift) as ::core::ffi::c_uint);
        }
        bs_write1(s, sps.has_vui as crate::stdlib::uint32_t);
        if sps.has_vui {
            bs_write1(
                s,
                sps.vui.aspect_ratio_info_present as crate::stdlib::uint32_t,
            );
            if sps.vui.aspect_ratio_info_present {
                let mut i = 0i32;
                static mut sar: [C2Rust_Unnamed_24; 17] = [
                    C2Rust_Unnamed_24 {
                        w: 1u8,
                        h: 1u8,
                        sar: 1u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 12u8,
                        h: 11u8,
                        sar: 2u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 10u8,
                        h: 11u8,
                        sar: 3u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 16u8,
                        h: 11u8,
                        sar: 4u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 40u8,
                        h: 33u8,
                        sar: 5u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 24u8,
                        h: 11u8,
                        sar: 6u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 20u8,
                        h: 11u8,
                        sar: 7u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 32u8,
                        h: 11u8,
                        sar: 8u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 80u8,
                        h: 33u8,
                        sar: 9u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 18u8,
                        h: 11u8,
                        sar: 10u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 15u8,
                        h: 11u8,
                        sar: 11u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 64u8,
                        h: 33u8,
                        sar: 12u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 160u8,
                        h: 99u8,
                        sar: 13u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 4u8,
                        h: 3u8,
                        sar: 14u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 3u8,
                        h: 2u8,
                        sar: 15u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 2u8,
                        h: 1u8,
                        sar: 16u8,
                    },
                    C2Rust_Unnamed_24 {
                        w: 0u8,
                        h: 0u8,
                        sar: 255u8,
                    },
                ];
                while sar[i as usize].sar as ::core::ffi::c_int != 255i32 {
                    if sar[i as usize].w as ::core::ffi::c_int == sps.vui.i_sar_width
                        && sar[i as usize].h as ::core::ffi::c_int == sps.vui.i_sar_height
                    {
                        break;
                    }
                    i += 1;
                }
                bs_write(s, 8i32, sar[i as usize].sar as crate::stdlib::uint32_t);
                if sar[i as usize].sar as ::core::ffi::c_int == 255i32 {
                    bs_write(s, 16i32, sps.vui.i_sar_width as crate::stdlib::uint32_t);
                    bs_write(s, 16i32, sps.vui.i_sar_height as crate::stdlib::uint32_t);
                }
            }
            bs_write1(s, sps.vui.overscan_info_present as crate::stdlib::uint32_t);
            if sps.vui.overscan_info_present {
                bs_write1(s, sps.vui.overscan_info as crate::stdlib::uint32_t);
            }
            bs_write1(s, sps.vui.signal_type_present as crate::stdlib::uint32_t);
            if sps.vui.signal_type_present {
                bs_write(s, 3i32, sps.vui.i_vidformat as crate::stdlib::uint32_t);
                bs_write1(s, sps.vui.fullrange as u32);
                bs_write1(
                    s,
                    sps.vui.color_description_present as crate::stdlib::uint32_t,
                );
                if sps.vui.color_description_present {
                    bs_write(s, 8i32, sps.vui.i_colorprim as crate::stdlib::uint32_t);
                    bs_write(s, 8i32, sps.vui.i_transfer as crate::stdlib::uint32_t);
                    bs_write(s, 8i32, sps.vui.i_colmatrix as crate::stdlib::uint32_t);
                }
            }
            bs_write1(
                s,
                sps.vui.chroma_loc_info_present as crate::stdlib::uint32_t,
            );
            if sps.vui.chroma_loc_info_present {
                bs_write_ue_big(s, sps.vui.i_chroma_loc_top as ::core::ffi::c_uint);
                bs_write_ue_big(s, sps.vui.i_chroma_loc_bottom as ::core::ffi::c_uint);
            }
            bs_write1(s, sps.vui.timing_info_present as crate::stdlib::uint32_t);
            if sps.vui.timing_info_present {
                bs_write32(s, sps.vui.i_num_units_in_tick);
                bs_write32(s, sps.vui.i_time_scale);
                bs_write1(s, sps.vui.fixed_frame_rate as crate::stdlib::uint32_t);
            }
            bs_write1(
                s,
                sps.vui.nal_hrd_parameters_present as crate::stdlib::uint32_t,
            );
            if sps.vui.nal_hrd_parameters_present {
                bs_write_ue_big(s, (sps.vui.hrd.i_cpb_cnt - 1i32) as ::core::ffi::c_uint);
                bs_write(
                    s,
                    4i32,
                    sps.vui.hrd.i_bit_rate_scale as crate::stdlib::uint32_t,
                );
                bs_write(
                    s,
                    4i32,
                    sps.vui.hrd.i_cpb_size_scale as crate::stdlib::uint32_t,
                );
                bs_write_ue_big(
                    s,
                    (sps.vui.hrd.i_bit_rate_value - 1i32) as ::core::ffi::c_uint,
                );
                bs_write_ue_big(
                    s,
                    (sps.vui.hrd.i_cpb_size_value - 1i32) as ::core::ffi::c_uint,
                );
                bs_write1(s, sps.vui.hrd.cbr_hrd as crate::stdlib::uint32_t);
                bs_write(
                    s,
                    5i32,
                    (sps.vui.hrd.i_initial_cpb_removal_delay_length - 1i32)
                        as crate::stdlib::uint32_t,
                );
                bs_write(
                    s,
                    5i32,
                    (sps.vui.hrd.i_cpb_removal_delay_length - 1i32) as crate::stdlib::uint32_t,
                );
                bs_write(
                    s,
                    5i32,
                    (sps.vui.hrd.i_dpb_output_delay_length - 1i32) as crate::stdlib::uint32_t,
                );
                bs_write(
                    s,
                    5i32,
                    sps.vui.hrd.i_time_offset_length as crate::stdlib::uint32_t,
                );
            }
            bs_write1(
                s,
                sps.vui.vcl_hrd_parameters_present as crate::stdlib::uint32_t,
            );
            if sps.vui.nal_hrd_parameters_present || sps.vui.vcl_hrd_parameters_present {
                bs_write1(s, 0u32);
            }
            bs_write1(s, sps.vui.pic_struct_present as crate::stdlib::uint32_t);
            bs_write1(s, sps.vui.bitstream_restriction as crate::stdlib::uint32_t);
            if sps.vui.bitstream_restriction {
                bs_write1(
                    s,
                    sps.vui.motion_vectors_over_pic_boundaries as crate::stdlib::uint32_t,
                );
                bs_write_ue_big(s, sps.vui.i_max_bytes_per_pic_denom as ::core::ffi::c_uint);
                bs_write_ue_big(s, sps.vui.i_max_bits_per_mb_denom as ::core::ffi::c_uint);
                bs_write_ue_big(
                    s,
                    sps.vui.i_log2_max_mv_length_horizontal as ::core::ffi::c_uint,
                );
                bs_write_ue_big(
                    s,
                    sps.vui.i_log2_max_mv_length_vertical as ::core::ffi::c_uint,
                );
                bs_write_ue_big(s, sps.vui.i_num_reorder_frames as ::core::ffi::c_uint);
                bs_write_ue_big(s, sps.vui.i_max_dec_frame_buffering as ::core::ffi::c_uint);
            }
        }
        bs_rbsp_trailing(s);
        bs_flush(s);
    }
}
pub unsafe extern "C" fn x264_8_pps_init(
    mut pps: *mut crate::src::common::set::x264_pps_t,
    mut i_id: ::core::ffi::c_int,
    mut param: *mut crate::x264_h::x264_param_t,
    sps: &crate::src::common::set::x264_sps_t,
) {
    unsafe {
        (*pps).i_id = i_id;
        (*pps).i_sps_id = sps.i_id;
        (*pps).cabac = (*param).cabac;
        (*pps).pic_order = (*param).i_avcintra_class == 0 && (*param).interlaced;
        (*pps).i_num_slice_groups = 1i32;
        (*pps).i_num_ref_idx_l0_default_active = (*param).i_frame_reference;
        (*pps).i_num_ref_idx_l1_default_active = 1i32;
        (*pps).weighted_pred = (*param).analyse.i_weighted_pred > 0i32;
        (*pps).weighted_bipred = if (*param).analyse.weighted_bipred {
            2i32
        } else {
            0i32
        };
        (*pps).i_pic_init_qp =
            if (*param).rc.i_rc_method == crate::x264_h::X264_RC_ABR || (*param).stitchable {
                26i32 + crate::src::common::common::QP_BD_OFFSET
            } else if (*param).rc.i_qp_constant < 51i32 + 6i32 * (8i32 - 8i32) {
                (*param).rc.i_qp_constant
            } else {
                51i32 + 6i32 * (8i32 - 8i32)
            };
        (*pps).i_pic_init_qs = 26i32 + crate::src::common::common::QP_BD_OFFSET;
        (*pps).i_chroma_qp_index_offset = (*param).analyse.i_chroma_qp_offset;
        (*pps).deblocking_filter_control = true;
        (*pps).constrained_intra_pred = (*param).constrained_intra;
        (*pps).redundant_pic_cnt = false;
        (*pps).transform_8x8_mode = (*param).analyse.transform_8x8;
    }
}
pub unsafe extern "C" fn x264_8_pps_write(
    mut s: *mut crate::src::common::bitstream::bs_t,
    sps: &crate::src::common::set::x264_sps_t,
    mut pps: *mut crate::src::common::set::x264_pps_t,
) {
    unsafe {
        bs_realign(s);
        bs_write_ue_big(s, (*pps).i_id as ::core::ffi::c_uint);
        bs_write_ue_big(s, (*pps).i_sps_id as ::core::ffi::c_uint);
        bs_write1(s, (*pps).cabac as crate::stdlib::uint32_t);
        bs_write1(s, (*pps).pic_order as crate::stdlib::uint32_t);
        bs_write_ue_big(s, ((*pps).i_num_slice_groups - 1i32) as ::core::ffi::c_uint);
        bs_write_ue_big(
            s,
            ((*pps).i_num_ref_idx_l0_default_active - 1i32) as ::core::ffi::c_uint,
        );
        bs_write_ue_big(
            s,
            ((*pps).i_num_ref_idx_l1_default_active - 1i32) as ::core::ffi::c_uint,
        );
        bs_write1(s, (*pps).weighted_pred as crate::stdlib::uint32_t);
        bs_write(s, 2i32, (*pps).weighted_bipred as crate::stdlib::uint32_t);
        bs_write_se(
            s,
            (*pps).i_pic_init_qp - 26i32 - crate::src::common::common::QP_BD_OFFSET,
        );
        bs_write_se(
            s,
            (*pps).i_pic_init_qs - 26i32 - crate::src::common::common::QP_BD_OFFSET,
        );
        bs_write_se(s, (*pps).i_chroma_qp_index_offset);
        bs_write1(
            s,
            (*pps).deblocking_filter_control as crate::stdlib::uint32_t,
        );
        bs_write1(s, (*pps).constrained_intra_pred as crate::stdlib::uint32_t);
        bs_write1(s, (*pps).redundant_pic_cnt as crate::stdlib::uint32_t);
        let mut b_scaling_list =
            !sps.avcintra_hd && sps.i_cqm_preset != crate::x264_h::X264_CQM_FLAT;
        if (*pps).transform_8x8_mode || b_scaling_list {
            bs_write1(s, (*pps).transform_8x8_mode as crate::stdlib::uint32_t);
            bs_write1(s, b_scaling_list as crate::stdlib::uint32_t);
            if b_scaling_list {
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
                if sps.avcintra_4k {
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_4IC as ::core::ffi::c_int,
                    );
                    bs_write1(s, 0u32);
                    bs_write1(s, 0u32);
                    bs_write1(s, 0u32);
                } else {
                    bs_write1(s, 0u32);
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
                    bs_write1(s, 0u32);
                }
                if (*pps).transform_8x8_mode {
                    scaling_list_write(
                        s,
                        sps,
                        crate::src::common::set::CQM_8IY as ::core::ffi::c_int + 4i32,
                    );
                    if sps.avcintra_4k {
                        bs_write1(s, 0u32);
                    } else {
                        scaling_list_write(
                            s,
                            sps,
                            crate::src::common::set::CQM_8PY as ::core::ffi::c_int + 4i32,
                        );
                    }
                    if sps.i_chroma_format_idc.is_444() {
                        scaling_list_write(
                            s,
                            sps,
                            crate::src::common::set::CQM_8IC as ::core::ffi::c_int + 4i32,
                        );
                        scaling_list_write(
                            s,
                            sps,
                            crate::src::common::set::CQM_8PC as ::core::ffi::c_int + 4i32,
                        );
                        bs_write1(s, 0u32);
                        bs_write1(s, 0u32);
                    }
                }
            }
            bs_write_se(s, (*pps).i_chroma_qp_index_offset);
        }
        bs_rbsp_trailing(s);
        bs_flush(s);
    }
}
pub unsafe extern "C" fn x264_8_sei_recovery_point_write(
    mut _h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut recovery_frame_cnt: ::core::ffi::c_int,
) {
    unsafe {
        let mut q = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut ::core::ffi::c_void,
            100i32,
        );
        bs_realign(&raw mut q);
        bs_write_ue_big(&raw mut q, recovery_frame_cnt as ::core::ffi::c_uint);
        bs_write1(&raw mut q, 1u32);
        bs_write1(&raw mut q, 0u32);
        bs_write(&raw mut q, 2i32, 0u32);
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8i32,
            crate::src::common::base::SEI_RECOVERY_POINT as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_sei_version_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) -> ::core::ffi::c_int {
    unsafe {
        static mut uuid: [crate::stdlib::uint8_t; 16] = [
            0xdcu8, 0x45u8, 0xe9u8, 0xbdu8, 0xe6u8, 0xd9u8, 0x48u8, 0xb7u8, 0x96u8, 0x2cu8, 0xd8u8,
            0x20u8, 0xd9u8, 0x23u8, 0xeeu8, 0xefu8,
        ];
        let mut opts = crate::src::common::base::x264_param2string(&raw mut (*h).param, 0i32);
        if opts.is_null() {
            return -(1i32);
        }
        let mut payload = crate::src::common::base::x264_malloc(
            (200usize).wrapping_add(crate::stdlib::strlen(opts)) as crate::stdlib::int64_t,
        ) as *mut ::core::ffi::c_char;
        if payload.is_null() {
            crate::src::common::base::x264_free(opts as *mut ::core::ffi::c_void);
            -(1i32)
        } else {
            let mut length = 0;
            crate::stdlib::memcpy(
                payload as *mut ::core::ffi::c_void,
                &raw const uuid as *const ::core::ffi::c_void,
                16usize,
            );
            crate::stdlib::sprintf(
                payload.offset(16isize),
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
            length = crate::stdlib::strlen(payload).wrapping_add(1usize) as ::core::ffi::c_int;
            x264_8_sei_write(
                s,
                payload as *mut crate::stdlib::uint8_t,
                length,
                crate::src::common::base::SEI_USER_DATA_UNREGISTERED as ::core::ffi::c_int,
            );
            crate::src::common::base::x264_free(opts as *mut ::core::ffi::c_void);
            crate::src::common::base::x264_free(payload as *mut ::core::ffi::c_void);
            0i32
        }
    }
}
pub unsafe extern "C" fn x264_8_sei_buffering_period_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut ::core::ffi::c_void,
            100i32,
        );
        bs_realign(&raw mut q);
        bs_write_ue_big(&raw mut q, (*h).sps.i_id as ::core::ffi::c_uint);
        if (*h).sps.vui.nal_hrd_parameters_present {
            bs_write(
                &raw mut q,
                (*h).sps.vui.hrd.i_initial_cpb_removal_delay_length,
                (*h).initial_cpb_removal_delay as crate::stdlib::uint32_t,
            );
            bs_write(
                &raw mut q,
                (*h).sps.vui.hrd.i_initial_cpb_removal_delay_length,
                (*h).initial_cpb_removal_delay_offset as crate::stdlib::uint32_t,
            );
        }
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8i32,
            crate::src::common::base::SEI_BUFFERING_PERIOD as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_sei_pic_timing_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut ::core::ffi::c_void,
            100i32,
        );
        bs_realign(&raw mut q);
        if (*h).sps.vui.nal_hrd_parameters_present || (*h).sps.vui.vcl_hrd_parameters_present {
            bs_write(
                &raw mut q,
                (*h).sps.vui.hrd.i_cpb_removal_delay_length,
                ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset) as crate::stdlib::uint32_t,
            );
            bs_write(
                &raw mut q,
                (*h).sps.vui.hrd.i_dpb_output_delay_length,
                (*(*h).fenc).i_dpb_output_delay as crate::stdlib::uint32_t,
            );
        }
        if (*h).sps.vui.pic_struct_present {
            let mut i = 0i32;
            bs_write(
                &raw mut q,
                4i32,
                ((*(*h).fenc).i_pic_struct - 1i32) as crate::stdlib::uint32_t,
            );
            while i < num_clock_ts[(*(*h).fenc).i_pic_struct as usize] as ::core::ffi::c_int {
                bs_write1(&raw mut q, 0u32);
                i += 1;
            }
        }
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8i32,
            crate::src::common::base::SEI_PIC_TIMING as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_sei_frame_packing_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf = [0; 100];
        let mut quincunx_sampling_flag = ((*h).param.i_frame_packing == 0i32) as ::core::ffi::c_int;
        (*(&raw mut tmp_buf as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut ::core::ffi::c_void,
            100i32,
        );
        bs_realign(&raw mut q);
        bs_write_ue_big(&raw mut q, 0u32);
        bs_write1(&raw mut q, 0u32);
        bs_write(
            &raw mut q,
            7i32,
            (*h).param.i_frame_packing as crate::stdlib::uint32_t,
        );
        bs_write1(
            &raw mut q,
            quincunx_sampling_flag as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            6i32,
            ((*h).param.i_frame_packing != 6i32) as crate::stdlib::uint32_t,
        );
        bs_write1(&raw mut q, 0u32);
        bs_write1(&raw mut q, 0u32);
        bs_write1(&raw mut q, 0u32);
        bs_write1(
            &raw mut q,
            ((*h).param.i_frame_packing == 5i32 && (*(*h).fenc).i_frame & 1i32 == 0)
                as crate::stdlib::uint32_t,
        );
        bs_write1(&raw mut q, 0u32);
        bs_write1(&raw mut q, 0u32);
        if quincunx_sampling_flag == 0i32 && (*h).param.i_frame_packing != 5i32 {
            bs_write(&raw mut q, 4i32, 0u32);
            bs_write(&raw mut q, 4i32, 0u32);
            bs_write(&raw mut q, 4i32, 0u32);
            bs_write(&raw mut q, 4i32, 0u32);
        }
        bs_write(&raw mut q, 8i32, 0u32);
        bs_write_ue_big(
            &raw mut q,
            ((*h).param.i_frame_packing != 5i32) as ::core::ffi::c_uint,
        );
        bs_write1(&raw mut q, 0u32);
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8i32,
            crate::src::common::base::SEI_FRAME_PACKING as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_sei_mastering_display_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut ::core::ffi::c_void,
            100i32,
        );
        bs_realign(&raw mut q);
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.mastering_display.i_green_x as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.mastering_display.i_green_y as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.mastering_display.i_blue_x as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.mastering_display.i_blue_y as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.mastering_display.i_red_x as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.mastering_display.i_red_y as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.mastering_display.i_white_x as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16i32,
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
            bs_pos(&raw mut q) / 8i32,
            crate::src::common::base::SEI_MASTERING_DISPLAY as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_sei_content_light_level_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut ::core::ffi::c_void,
            100i32,
        );
        bs_realign(&raw mut q);
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.content_light_level.i_max_cll as crate::stdlib::uint32_t,
        );
        bs_write(
            &raw mut q,
            16i32,
            (*h).param.content_light_level.i_max_fall as crate::stdlib::uint32_t,
        );
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8i32,
            crate::src::common::base::SEI_CONTENT_LIGHT_LEVEL as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_sei_alternative_transfer_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf = [0; 100];
        (*(&raw mut tmp_buf as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut ::core::ffi::c_void,
            100i32,
        );
        bs_realign(&raw mut q);
        bs_write(
            &raw mut q,
            8i32,
            (*h).param.i_alternative_transfer as crate::stdlib::uint32_t,
        );
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8i32,
            crate::src::common::base::SEI_ALTERNATIVE_TRANSFER as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_filler_write(
    mut _h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
    mut filler: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        bs_realign(s);
        while i < filler {
            bs_write(s, 8i32, 0xffu32);
            i += 1;
        }
        bs_rbsp_trailing(s);
        bs_flush(s);
    }
}
pub unsafe extern "C" fn x264_8_sei_dec_ref_pic_marking_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut s: *mut crate::src::common::bitstream::bs_t,
) {
    unsafe {
        let mut q = crate::src::common::bitstream::bs_s {
            p_start: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            p_end: ::core::ptr::null_mut::<crate::stdlib::uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        let mut tmp_buf = [0; 100];
        let mut sh = &raw mut (*h).sh_backup;
        (*(&raw mut tmp_buf as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        bs_init(
            &raw mut q,
            &raw mut tmp_buf as *mut ::core::ffi::c_void,
            100i32,
        );
        bs_realign(&raw mut q);
        bs_write1(&raw mut q, 0u32);
        bs_write_ue_big(&raw mut q, (*sh).i_frame_num as ::core::ffi::c_uint);
        if !(*h).sps.frame_mbs_only {
            bs_write1(&raw mut q, 0u32);
        }
        bs_write1(
            &raw mut q,
            ((*sh).i_mmco_command_count > 0i32) as crate::stdlib::uint32_t,
        );
        if (*sh).i_mmco_command_count > 0i32 {
            let mut i = 0i32;
            while i < (*sh).i_mmco_command_count {
                bs_write_ue_big(&raw mut q, 1u32);
                bs_write_ue_big(
                    &raw mut q,
                    ((*sh).mmco[i as usize].i_difference_of_pic_nums - 1i32) as ::core::ffi::c_uint,
                );
                i += 1;
            }
            bs_write_ue_big(&raw mut q, 0u32);
        }
        bs_align_10(&raw mut q);
        x264_8_sei_write(
            s,
            &raw mut tmp_buf as *mut crate::stdlib::uint8_t,
            bs_pos(&raw mut q) / 8i32,
            crate::src::common::base::SEI_DEC_REF_PIC_MARKING as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_sei_avcintra_umid_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut _s: *mut crate::src::common::bitstream::bs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut data = [0; 512];
        let len = 497i32;
        let mut msg = b"UMID\0".as_ptr() as *const ::core::ffi::c_char;
        crate::stdlib::memset(
            &raw mut data as *mut ::core::ffi::c_void,
            0xffi32,
            len as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut data as *mut ::core::ffi::c_void,
            &raw const avcintra_uuid as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>(),
        );
        crate::stdlib::memcpy(
            (&raw mut data as *mut crate::stdlib::uint8_t).offset(16isize)
                as *mut ::core::ffi::c_void,
            msg as *const ::core::ffi::c_void,
            crate::stdlib::strlen(msg),
        );
        data[20usize] = 0x13u8;
        data[26usize] = 0u8;
        data[25usize] = data[26usize];
        data[23usize] = data[25usize];
        data[22usize] = data[23usize];
        data[28usize] = 0x14u8;
        data[34usize] = 0u8;
        data[33usize] = data[34usize];
        data[31usize] = data[33usize];
        data[30usize] = data[31usize];
        data[36usize] = 0x60u8;
        data[41usize] = 0x22u8;
        data[60usize] = 0x62u8;
        data[66usize] = 0u8;
        data[65usize] = data[66usize];
        data[63usize] = data[65usize];
        data[62usize] = data[63usize];
        data[68usize] = 0x63u8;
        data[74usize] = 0u8;
        data[73usize] = data[74usize];
        data[71usize] = data[73usize];
        data[70usize] = data[71usize];
        x264_8_sei_write(
            &raw mut (*h).out.bs,
            &raw mut data as *mut crate::stdlib::uint8_t,
            len,
            crate::src::common::base::SEI_USER_DATA_UNREGISTERED as ::core::ffi::c_int,
        );
        0i32
    }
}
pub unsafe extern "C" fn x264_8_sei_avcintra_vanc_write(
    mut h: *mut crate::src::common::common::x264_t,
    mut _s: *mut crate::src::common::bitstream::bs_t,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut data = [0; 6000];
        let mut msg = b"VANC\0".as_ptr() as *const ::core::ffi::c_char;
        if len < 0i32
            || len as ::core::ffi::c_uint as usize
                > ::core::mem::size_of::<[crate::stdlib::uint8_t; 6000]>()
        {
            log::error!("AVC-Intra SEI is too large ({})", len);
            return -(1i32);
        }
        crate::stdlib::memset(
            &raw mut data as *mut ::core::ffi::c_void,
            0xffi32,
            len as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            &raw mut data as *mut ::core::ffi::c_void,
            &raw const avcintra_uuid as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>(),
        );
        crate::stdlib::memcpy(
            (&raw mut data as *mut crate::stdlib::uint8_t).offset(16isize)
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
        0i32
    }
}
pub unsafe extern "C" fn x264_8_validate_levels(
    mut h: *mut crate::src::common::common::x264_t,
    mut verbose: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret = 0i32;
        let mut mbs = (*h).sps.i_mb_width * (*h).sps.i_mb_height;
        let mut dpb = mbs * (*h).sps.vui.i_max_dec_frame_buffering;
        let mut cbp_factor = if (*h).sps.i_profile_idc
            >= crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int
        {
            16i32
        } else if (*h).sps.i_profile_idc
            == crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int
        {
            12i32
        } else if (*h).sps.i_profile_idc
            == crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
        {
            5i32
        } else {
            4i32
        };
        let mut l = &raw const crate::src::common::tables::x264_levels
            as *const crate::x264_h::x264_level_t;
        while (*l).level_idc as ::core::ffi::c_int != 0i32
            && (*l).level_idc as ::core::ffi::c_int != (*h).param.i_level_idc
        {
            l = l.offset(1);
        }
        if (*l).frame_size < mbs
            || ((*l).frame_size * 8i32) < (*h).sps.i_mb_width * (*h).sps.i_mb_width
            || ((*l).frame_size * 8i32) < (*h).sps.i_mb_height * (*h).sps.i_mb_height
        {
            if verbose != 0 {
                log::warn!(
                    "frame MB size ({}x{}) > level limit ({})",
                    (*h).sps.i_mb_width,
                    (*h).sps.i_mb_height,
                    (*l).frame_size
                );
            }
            ret = 1i32;
        }
        if dpb > (*l).dpb {
            if verbose != 0 {
                log::warn!(
                    "DPB size ({} frames, {} mbs) > level limit ({} frames, {} mbs)",
                    (*h).sps.vui.i_max_dec_frame_buffering,
                    dpb,
                    (*l).dpb / mbs,
                    (*l).dpb
                );
            }
            ret = 1i32;
        }
        if (*h).param.rc.i_vbv_max_bitrate > (*l).bitrate * cbp_factor / 4i32 {
            if verbose != 0 {
                log::warn!(
                    "VBV bitrate ({}) > level limit ({})",
                    (*h).param.rc.i_vbv_max_bitrate as crate::stdlib::int64_t,
                    (*l).bitrate * cbp_factor / 4i32
                );
            }
            ret = 1i32;
        }
        if (*h).param.rc.i_vbv_buffer_size > (*l).cpb * cbp_factor / 4i32 {
            if verbose != 0 {
                log::warn!(
                    "VBV buffer ({}) > level limit ({})",
                    (*h).param.rc.i_vbv_buffer_size as crate::stdlib::int64_t,
                    (*l).cpb * cbp_factor / 4i32
                );
            }
            ret = 1i32;
        }
        if (*h).param.analyse.i_mv_range > (*l).mv_range as ::core::ffi::c_int {
            if verbose != 0 {
                log::warn!(
                    "MV range ({}) > level limit ({})",
                    (*h).param.analyse.i_mv_range as crate::stdlib::int64_t,
                    (*l).mv_range as ::core::ffi::c_int
                );
            }
            ret = 1i32;
        }
        if (*h).param.interlaced > ((*l).frame_only == 0) {
            if verbose != 0 {
                log::warn!(
                    "interlaced ({}) > level limit ({})",
                    (*h).param.interlaced as crate::stdlib::int64_t,
                    ((*l).frame_only == 0) as ::core::ffi::c_int
                );
            }
            ret = 1i32;
        }
        if (*h).param.fake_interlaced > ((*l).frame_only == 0) {
            if verbose != 0 {
                log::warn!(
                    "fake interlaced ({}) > level limit ({})",
                    (*h).param.fake_interlaced as crate::stdlib::int64_t,
                    ((*l).frame_only == 0) as ::core::ffi::c_int
                );
            }
            ret = 1i32;
        }
        if (*h).param.i_fps_den > 0u32 {
            if mbs as crate::stdlib::int64_t * (*h).param.i_fps_num as crate::stdlib::int64_t
                / (*h).param.i_fps_den as crate::stdlib::int64_t
                > (*l).mbps as crate::stdlib::int64_t
            {
                if verbose != 0 {
                    log::warn!(
                        "MB rate ({}) > level limit ({})",
                        mbs as crate::stdlib::int64_t
                            * (*h).param.i_fps_num as crate::stdlib::int64_t
                            / (*h).param.i_fps_den as crate::stdlib::int64_t,
                        (*l).mbps
                    );
                }
                ret = 1i32;
            }
        }
        ret
    }
}
