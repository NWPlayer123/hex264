
#[no_mangle]

pub static mut x264_8_level_token: [[crate::src::common::bitstream::vlc_large_t; 128]; 7] =
    [[crate::src::common::bitstream::vlc_large_t {
        i_bits: 0,
        i_size: 0,
        i_next: 0,
    }; 128]; 7];
#[no_mangle]

pub static mut x264_8_run_before: [crate::stdlib::uint32_t; 65536] = [0; 65536];
#[no_mangle]

pub unsafe extern "C" fn x264_8_cavlc_init(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut i_suffix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_suffix < 7 as ::core::ffi::c_int {
            let mut level: crate::stdlib::int16_t =
                (-crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2 as ::core::ffi::c_int)
                    as crate::stdlib::int16_t;
            while (level as ::core::ffi::c_int)
                < crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2 as ::core::ffi::c_int
            {
                let mut mask: ::core::ffi::c_int =
                    level as ::core::ffi::c_int >> 15 as ::core::ffi::c_int;
                let mut abs_level: ::core::ffi::c_int = (level as ::core::ffi::c_int ^ mask) - mask;
                let mut i_level_code: ::core::ffi::c_int = if abs_level != 0 {
                    abs_level * 2 as ::core::ffi::c_int - mask - 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
                let mut i_next: ::core::ffi::c_int = i_suffix;
                let mut vlc: *mut crate::src::common::bitstream::vlc_large_t =
                    (&raw mut *(&raw mut x264_8_level_token
                        as *mut [crate::src::common::bitstream::vlc_large_t; 128])
                        .offset(i_suffix as isize)
                        as *mut crate::src::common::bitstream::vlc_large_t)
                        .offset(
                            (level as ::core::ffi::c_int
                                + crate::src::common::bitstream::LEVEL_TABLE_SIZE
                                    / 2 as ::core::ffi::c_int) as isize,
                        ) as *mut crate::src::common::bitstream::vlc_large_t;
                if i_level_code >> i_suffix < 14 as ::core::ffi::c_int {
                    (*vlc).i_size = ((i_level_code >> i_suffix)
                        + 1 as ::core::ffi::c_int
                        + i_suffix) as crate::stdlib::uint8_t;
                    (*vlc).i_bits = (((1 as ::core::ffi::c_int) << i_suffix)
                        + (i_level_code
                            & ((1 as ::core::ffi::c_int) << i_suffix) - 1 as ::core::ffi::c_int))
                        as crate::stdlib::uint16_t;
                } else if i_suffix == 0 as ::core::ffi::c_int
                    && i_level_code < 30 as ::core::ffi::c_int
                {
                    (*vlc).i_size = 19 as crate::stdlib::uint8_t;
                    (*vlc).i_bits = (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        + (i_level_code - 14 as ::core::ffi::c_int))
                        as crate::stdlib::uint16_t;
                } else if i_suffix > 0 as ::core::ffi::c_int
                    && i_level_code >> i_suffix == 14 as ::core::ffi::c_int
                {
                    (*vlc).i_size = (15 as ::core::ffi::c_int + i_suffix) as crate::stdlib::uint8_t;
                    (*vlc).i_bits = (((1 as ::core::ffi::c_int) << i_suffix)
                        + (i_level_code
                            & ((1 as ::core::ffi::c_int) << i_suffix) - 1 as ::core::ffi::c_int))
                        as crate::stdlib::uint16_t;
                } else {
                    i_level_code -= (15 as ::core::ffi::c_int) << i_suffix;
                    if i_suffix == 0 as ::core::ffi::c_int {
                        i_level_code -= 15 as ::core::ffi::c_int;
                    }
                    (*vlc).i_size = 28 as crate::stdlib::uint8_t;
                    (*vlc).i_bits = (((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                        + i_level_code)
                        as crate::stdlib::uint16_t;
                }
                if i_next == 0 as ::core::ffi::c_int {
                    i_next += 1;
                }
                if abs_level > (3 as ::core::ffi::c_int) << i_next - 1 as ::core::ffi::c_int
                    && i_next < 6 as ::core::ffi::c_int
                {
                    i_next += 1;
                }
                (*vlc).i_next = i_next as crate::stdlib::uint8_t;
                level += 1;
            }
            i_suffix += 1;
        }
        x264_8_run_before[0 as ::core::ffi::c_int as usize] = 0 as crate::stdlib::uint32_t;
        x264_8_run_before[1 as ::core::ffi::c_int as usize] = 0 as crate::stdlib::uint32_t;
        let mut i: crate::stdlib::uint32_t = 2 as crate::stdlib::uint32_t;
        while i < ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as crate::stdlib::uint32_t
        {
            let mut runlevel: crate::src::common::bitstream::x264_run_level_t =
                crate::src::common::bitstream::x264_run_level_t {
                    last: 0,
                    mask: 0,
                    level: [0; 18],
                };
            let mut dct: [crate::src::common::common::dctcoef; 16] = [0; 16];
            let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < 16 as ::core::ffi::c_int {
                dct[j as usize] = (i & ((1 as ::core::ffi::c_int) << j) as crate::stdlib::uint32_t)
                    as crate::src::common::common::dctcoef;
                j += 1;
            }
            let mut total: ::core::ffi::c_int = (*h).quantf.coeff_level_run
                [crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                &raw mut dct as *mut crate::src::common::common::dctcoef,
                &raw mut runlevel,
            );
            let mut zeros: ::core::ffi::c_int =
                runlevel.last as ::core::ffi::c_int + 1 as ::core::ffi::c_int - total;
            let mut mask_0: crate::stdlib::uint32_t =
                i << i.leading_zeros() as i32 + 1 as ::core::ffi::c_int;
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 < total - 1 as ::core::ffi::c_int && zeros > 0 as ::core::ffi::c_int {
                let mut idx: ::core::ffi::c_int = (if zeros < 7 as ::core::ffi::c_int {
                    zeros
                } else {
                    7 as ::core::ffi::c_int
                }) - 1 as ::core::ffi::c_int;
                let mut run: ::core::ffi::c_int = mask_0.leading_zeros() as i32;
                let mut len: ::core::ffi::c_int = crate::src::common::tables::x264_run_before_init
                    [idx as usize][run as usize]
                    .i_size as ::core::ffi::c_int;
                size += len;
                bits <<= len;
                bits |= crate::src::common::tables::x264_run_before_init[idx as usize][run as usize]
                    .i_bits as ::core::ffi::c_int;
                zeros -= run;
                mask_0 <<= run + 1 as ::core::ffi::c_int;
                j_0 += 1;
            }
            x264_8_run_before[i as usize] =
                ((bits << 5 as ::core::ffi::c_int) + size) as crate::stdlib::uint32_t;
            i = i.wrapping_add(1);
        }
    }
}
