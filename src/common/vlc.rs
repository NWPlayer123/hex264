use core::ffi::c_int;

use crate::bitstream_h::{vlc_large_t, x264_run_level_t, LEVEL_TABLE_SIZE};
use crate::common_h::{dctcoef, x264_t};
use crate::macroblock_h::DCT_LUMA_4x4;
use crate::stdint_intn_h::int16_t;
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::tables_h::x264_run_before_init;
#[no_mangle]
#[c2rust::src_loc = "30:13"]
static mut x264_10_level_token: [[vlc_large_t; 128]; 7] = [[vlc_large_t {
    i_bits: 0,
    i_size: 0,
    i_next: 0,
}; 128]; 7];
#[no_mangle]
#[c2rust::src_loc = "31:10"]
static mut x264_10_run_before: [uint32_t; 65536] = [0; 65536];
#[no_mangle]
#[c2rust::src_loc = "33:1"]
unsafe extern "C" fn x264_10_cavlc_init(mut h: *mut x264_t) {
    let mut i_suffix: c_int = 0 as c_int;
    while i_suffix < 7 as c_int {
        let mut level: int16_t = (-LEVEL_TABLE_SIZE / 2 as c_int) as int16_t;
        while (level as c_int) < LEVEL_TABLE_SIZE / 2 as c_int {
            let mut mask: c_int = level as c_int >> 15 as c_int;
            let mut abs_level: c_int = (level as c_int ^ mask) - mask;
            let mut i_level_code: c_int = if abs_level != 0 {
                abs_level * 2 as c_int - mask - 2 as c_int
            } else {
                0 as c_int
            };
            let mut i_next: c_int = i_suffix;
            let mut vlc: *mut vlc_large_t =
                &mut *(*x264_10_level_token.as_mut_ptr().offset(i_suffix as isize))
                    .as_mut_ptr()
                    .offset((level as c_int + LEVEL_TABLE_SIZE / 2 as c_int) as isize)
                    as *mut vlc_large_t;
            if i_level_code >> i_suffix < 14 as c_int {
                (*vlc).i_size = ((i_level_code >> i_suffix) + 1 as c_int + i_suffix) as uint8_t;
                (*vlc).i_bits = (((1 as c_int) << i_suffix)
                    + (i_level_code & ((1 as c_int) << i_suffix) - 1 as c_int))
                    as uint16_t;
            } else if i_suffix == 0 as c_int && i_level_code < 30 as c_int {
                (*vlc).i_size = 19 as uint8_t;
                (*vlc).i_bits =
                    (((1 as c_int) << 4 as c_int) + (i_level_code - 14 as c_int)) as uint16_t;
            } else if i_suffix > 0 as c_int && i_level_code >> i_suffix == 14 as c_int {
                (*vlc).i_size = (15 as c_int + i_suffix) as uint8_t;
                (*vlc).i_bits = (((1 as c_int) << i_suffix)
                    + (i_level_code & ((1 as c_int) << i_suffix) - 1 as c_int))
                    as uint16_t;
            } else {
                i_level_code -= (15 as c_int) << i_suffix;
                if i_suffix == 0 as c_int {
                    i_level_code -= 15 as c_int;
                }
                (*vlc).i_size = 28 as uint8_t;
                (*vlc).i_bits = (((1 as c_int) << 12 as c_int) + i_level_code) as uint16_t;
            }
            if i_next == 0 as c_int {
                i_next += 1;
            }
            if abs_level > (3 as c_int) << i_next - 1 as c_int && i_next < 6 as c_int {
                i_next += 1;
            }
            (*vlc).i_next = i_next as uint8_t;
            level += 1;
        }
        i_suffix += 1;
    }
    x264_10_run_before[0] = 0 as uint32_t;
    x264_10_run_before[1] = 0 as uint32_t;
    let mut i: uint32_t = 2 as uint32_t;
    while i < ((1 as c_int) << 16 as c_int) as uint32_t {
        let mut runlevel: x264_run_level_t = x264_run_level_t {
            last: 0,
            mask: 0,
            level: [0; 18],
        };
        let mut dct: [dctcoef; 16] = [0; 16];
        let mut size: c_int = 0 as c_int;
        let mut bits: c_int = 0 as c_int;
        let mut j: c_int = 0 as c_int;
        while j < 16 as c_int {
            dct[j as usize] = (i & ((1 as c_int) << j) as uint32_t) as dctcoef;
            j += 1;
        }
        let mut total: c_int =
            (*h).quantf.coeff_level_run[DCT_LUMA_4x4 as c_int as usize]
                .expect("non-null function pointer")(dct.as_mut_ptr(), &mut runlevel);
        let mut zeros: c_int = runlevel.last as c_int + 1 as c_int - total;
        let mut mask_0: uint32_t = i << i.leading_zeros() as i32 + 1 as c_int;
        let mut j_0: c_int = 0 as c_int;
        while j_0 < total - 1 as c_int && zeros > 0 as c_int {
            let mut idx: c_int = (if zeros < 7 as c_int {
                zeros
            } else {
                7 as c_int
            }) - 1 as c_int;
            let mut run: c_int = mask_0.leading_zeros() as i32;
            let mut len: c_int = x264_run_before_init[idx as usize][run as usize].i_size as c_int;
            size += len;
            bits <<= len;
            bits |= x264_run_before_init[idx as usize][run as usize].i_bits as c_int;
            zeros -= run;
            mask_0 <<= run + 1 as c_int;
            j_0 += 1;
        }
        x264_10_run_before[i as usize] = ((bits << 5 as c_int) + size) as uint32_t;
        i = i.wrapping_add(1);
    }
}
