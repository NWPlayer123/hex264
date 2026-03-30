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
        let mut i_suffix = 0i32;
        while i_suffix < 7i32 {
            let mut level =
                (-crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2i32) as crate::stdlib::int16_t;
            while (level as ::core::ffi::c_int)
                < crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2i32
            {
                let mut mask = level as ::core::ffi::c_int >> 15i32;
                let mut abs_level = (level as ::core::ffi::c_int ^ mask) - mask;
                let mut i_level_code = if abs_level != 0 {
                    abs_level * 2i32 - mask - 2i32
                } else {
                    0i32
                };
                let mut i_next = i_suffix;
                let mut vlc = (&raw mut *(&raw mut x264_8_level_token
                    as *mut [crate::src::common::bitstream::vlc_large_t; 128])
                    .offset(i_suffix as isize)
                    as *mut crate::src::common::bitstream::vlc_large_t)
                    .offset(
                        (level as ::core::ffi::c_int
                            + crate::src::common::bitstream::LEVEL_TABLE_SIZE / 2i32)
                            as isize,
                    );
                if i_level_code >> i_suffix < 14i32 {
                    (*vlc).i_size =
                        ((i_level_code >> i_suffix) + 1i32 + i_suffix) as crate::stdlib::uint8_t;
                    (*vlc).i_bits = (((1i32) << i_suffix)
                        + (i_level_code & ((1i32) << i_suffix) - 1i32))
                        as crate::stdlib::uint16_t;
                } else if i_suffix == 0i32 && i_level_code < 30i32 {
                    (*vlc).i_size = 19u8;
                    (*vlc).i_bits =
                        (((1i32) << 4i32) + (i_level_code - 14i32)) as crate::stdlib::uint16_t;
                } else if i_suffix > 0i32 && i_level_code >> i_suffix == 14i32 {
                    (*vlc).i_size = (15i32 + i_suffix) as crate::stdlib::uint8_t;
                    (*vlc).i_bits = (((1i32) << i_suffix)
                        + (i_level_code & ((1i32) << i_suffix) - 1i32))
                        as crate::stdlib::uint16_t;
                } else {
                    i_level_code -= (15i32) << i_suffix;
                    if i_suffix == 0i32 {
                        i_level_code -= 15i32;
                    }
                    (*vlc).i_size = 28u8;
                    (*vlc).i_bits = (((1i32) << 12i32) + i_level_code) as crate::stdlib::uint16_t;
                }
                if i_next == 0i32 {
                    i_next += 1;
                }
                if abs_level > (3i32) << i_next - 1i32 && i_next < 6i32 {
                    i_next += 1;
                }
                (*vlc).i_next = i_next as crate::stdlib::uint8_t;
                level += 1;
            }
            i_suffix += 1;
        }
        x264_8_run_before[0usize] = 0u32;
        x264_8_run_before[1usize] = 0u32;
        let mut i = 2u32;
        while i < ((1i32) << 16i32) as crate::stdlib::uint32_t {
            let mut runlevel = crate::src::common::bitstream::x264_run_level_t {
                last: 0,
                mask: 0,
                level: [0; 18],
            };
            let mut dct = [0; 16];
            let mut size = 0i32;
            let mut bits = 0i32;
            let mut j = 0i32;
            while j < 16i32 {
                dct[j as usize] = (i & ((1i32) << j) as crate::stdlib::uint32_t)
                    as crate::src::common::common::dctcoef;
                j += 1;
            }
            let mut total = (*h).quantf.coeff_level_run
                [crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                &raw mut dct as *mut crate::src::common::common::dctcoef,
                &raw mut runlevel,
            );
            let mut zeros = runlevel.last + 1i32 - total;
            let mut mask_0 = i << i.leading_zeros() as i32 + 1i32;
            let mut j_0 = 0i32;
            while j_0 < total - 1i32 && zeros > 0i32 {
                let mut idx = (if zeros < 7i32 { zeros } else { 7i32 }) - 1i32;
                let mut run = mask_0.leading_zeros() as i32;
                let mut len = crate::src::common::tables::x264_run_before_init[idx as usize]
                    [run as usize]
                    .i_size as ::core::ffi::c_int;
                size += len;
                bits <<= len;
                bits |= crate::src::common::tables::x264_run_before_init[idx as usize][run as usize]
                    .i_bits as ::core::ffi::c_int;
                zeros -= run;
                mask_0 <<= run + 1i32;
                j_0 += 1;
            }
            x264_8_run_before[i as usize] = ((bits << 5i32) + size) as crate::stdlib::uint32_t;
            i = i.wrapping_add(1);
        }
    }
}
