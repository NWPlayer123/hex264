use core::ffi::{c_int, c_uint};

use crate::bitstream_h::x264_run_level_t;
use crate::common_h::{dctcoef, udctcoef, x264_t};
use crate::macroblock_h::{
    DCT_CHROMAU_4x4, DCT_CHROMAU_8x8, DCT_CHROMAV_4x4, DCT_CHROMAV_8x8, DCT_LUMA_4x4, DCT_LUMA_8x8,
    DCT_CHROMAU_AC, DCT_CHROMAU_DC, DCT_CHROMAV_AC, DCT_CHROMAV_DC, DCT_CHROMA_AC, DCT_LUMA_AC,
    DCT_LUMA_DC,
};
use crate::stdint_intn_h::int32_t;
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::tables_h::{x264_decimate_table4, x264_decimate_table8};

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "30:9"]
pub struct x264_quant_function_t {
    pub quant_8x8:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> c_int>,
    pub quant_4x4:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> c_int>,
    pub quant_4x4x4:
        Option<unsafe extern "C" fn(*mut [dctcoef; 16], *mut udctcoef, *mut udctcoef) -> c_int>,
    pub quant_4x4_dc: Option<unsafe extern "C" fn(*mut dctcoef, c_int, c_int) -> c_int>,
    pub quant_2x2_dc: Option<unsafe extern "C" fn(*mut dctcoef, c_int, c_int) -> c_int>,
    pub dequant_8x8: Option<unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 64], c_int) -> ()>,
    pub dequant_4x4: Option<unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> ()>,
    pub dequant_4x4_dc: Option<unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> ()>,
    pub idct_dequant_2x4_dc: Option<
        unsafe extern "C" fn(*mut dctcoef, *mut [dctcoef; 16], *mut [c_int; 16], c_int) -> (),
    >,
    pub idct_dequant_2x4_dconly:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> ()>,
    pub optimize_chroma_2x2_dc: Option<unsafe extern "C" fn(*mut dctcoef, c_int) -> c_int>,
    pub optimize_chroma_2x4_dc: Option<unsafe extern "C" fn(*mut dctcoef, c_int) -> c_int>,
    pub denoise_dct:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut uint32_t, *mut udctcoef, c_int) -> ()>,
    pub decimate_score15: Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>,
    pub decimate_score16: Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>,
    pub decimate_score64: Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>,
    pub coeff_last: [Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>; 14],
    pub coeff_last4: Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>,
    pub coeff_last8: Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>,
    pub coeff_level_run:
        [Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int>; 13],
    pub coeff_level_run4:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int>,
    pub coeff_level_run8:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int>,
    pub trellis_cabac_4x4: Option<
        unsafe extern "C" fn(
            *const c_int,
            *const uint8_t,
            c_int,
            c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            c_int,
        ) -> c_int,
    >,
    pub trellis_cabac_8x8: Option<
        unsafe extern "C" fn(
            *const c_int,
            *const uint8_t,
            c_int,
            c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            c_int,
        ) -> c_int,
    >,
    pub trellis_cabac_4x4_psy: Option<
        unsafe extern "C" fn(
            *const c_int,
            *const uint8_t,
            c_int,
            c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            c_int,
            *mut dctcoef,
            c_int,
        ) -> c_int,
    >,
    pub trellis_cabac_8x8_psy: Option<
        unsafe extern "C" fn(
            *const c_int,
            *const uint8_t,
            c_int,
            c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            c_int,
            *mut dctcoef,
            c_int,
        ) -> c_int,
    >,
    pub trellis_cabac_dc: Option<
        unsafe extern "C" fn(
            *const c_int,
            *const uint8_t,
            c_int,
            c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            c_int,
        ) -> c_int,
    >,
    pub trellis_cabac_chroma_422_dc: Option<
        unsafe extern "C" fn(
            *const c_int,
            *const uint8_t,
            c_int,
            c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
        ) -> c_int,
    >,
}

#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn quant_8x8(
    mut dct: *mut dctcoef,
    mut mf: *mut udctcoef,
    mut bias: *mut udctcoef,
) -> c_int {
    let mut nz: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 64 as c_int {
        if *dct.offset(i as isize) > 0 as dctcoef {
            *dct.offset(i as isize) = ((*bias.offset(i as isize))
                .wrapping_add(*dct.offset(i as isize) as udctcoef)
                .wrapping_mul(*mf.offset(i as isize))
                >> 16 as c_int) as dctcoef;
        } else {
            *dct.offset(i as isize) = -(((*bias.offset(i as isize))
                .wrapping_add(-*dct.offset(i as isize) as udctcoef)
                .wrapping_mul(*mf.offset(i as isize))
                >> 16 as c_int) as int32_t) as dctcoef;
        }
        nz |= *dct.offset(i as isize) as c_int;
        i += 1;
    }
    return (nz != 0) as c_int;
}
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn quant_4x4(
    mut dct: *mut dctcoef,
    mut mf: *mut udctcoef,
    mut bias: *mut udctcoef,
) -> c_int {
    let mut nz: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 16 as c_int {
        if *dct.offset(i as isize) > 0 as dctcoef {
            *dct.offset(i as isize) = ((*bias.offset(i as isize))
                .wrapping_add(*dct.offset(i as isize) as udctcoef)
                .wrapping_mul(*mf.offset(i as isize))
                >> 16 as c_int) as dctcoef;
        } else {
            *dct.offset(i as isize) = -(((*bias.offset(i as isize))
                .wrapping_add(-*dct.offset(i as isize) as udctcoef)
                .wrapping_mul(*mf.offset(i as isize))
                >> 16 as c_int) as int32_t) as dctcoef;
        }
        nz |= *dct.offset(i as isize) as c_int;
        i += 1;
    }
    return (nz != 0) as c_int;
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn quant_4x4x4(
    mut dct: *mut [dctcoef; 16],
    mut mf: *mut udctcoef,
    mut bias: *mut udctcoef,
) -> c_int {
    let mut nza: c_int = 0 as c_int;
    let mut j: c_int = 0 as c_int;
    while j < 4 as c_int {
        let mut nz: c_int = 0 as c_int;
        let mut i: c_int = 0 as c_int;
        while i < 16 as c_int {
            if (*dct.offset(j as isize))[i as usize] > 0 as dctcoef {
                (*dct.offset(j as isize))[i as usize] = ((*bias.offset(i as isize))
                    .wrapping_add((*dct.offset(j as isize))[i as usize] as udctcoef)
                    .wrapping_mul(*mf.offset(i as isize))
                    >> 16 as c_int)
                    as dctcoef;
            } else {
                (*dct.offset(j as isize))[i as usize] = -(((*bias.offset(i as isize))
                    .wrapping_add(-(*dct.offset(j as isize))[i as usize] as udctcoef)
                    .wrapping_mul(*mf.offset(i as isize))
                    >> 16 as c_int)
                    as int32_t) as dctcoef;
            }
            nz |= (*dct.offset(j as isize))[i as usize] as c_int;
            i += 1;
        }
        nza |= ((nz != 0) as c_int) << j;
        j += 1;
    }
    return nza;
}
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn quant_4x4_dc(mut dct: *mut dctcoef, mut mf: c_int, mut bias: c_int) -> c_int {
    let mut nz: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < 16 as c_int {
        if *dct.offset(i as isize) > 0 as dctcoef {
            *dct.offset(i as isize) = ((bias as uint32_t)
                .wrapping_add(*dct.offset(i as isize) as uint32_t)
                .wrapping_mul(mf as uint32_t)
                >> 16 as c_int) as dctcoef;
        } else {
            *dct.offset(i as isize) = -(((bias as uint32_t)
                .wrapping_add(-*dct.offset(i as isize) as uint32_t)
                .wrapping_mul(mf as uint32_t)
                >> 16 as c_int) as int32_t) as dctcoef;
        }
        nz |= *dct.offset(i as isize) as c_int;
        i += 1;
    }
    return (nz != 0) as c_int;
}
#[c2rust::src_loc = "96:1"]
unsafe extern "C" fn quant_2x2_dc(mut dct: *mut dctcoef, mut mf: c_int, mut bias: c_int) -> c_int {
    let mut nz: c_int = 0 as c_int;
    if *dct.offset(0 as c_int as isize) > 0 as dctcoef {
        *dct.offset(0 as c_int as isize) = ((bias as uint32_t)
            .wrapping_add(*dct.offset(0 as c_int as isize) as uint32_t)
            .wrapping_mul(mf as uint32_t)
            >> 16 as c_int) as dctcoef;
    } else {
        *dct.offset(0 as c_int as isize) = -(((bias as uint32_t)
            .wrapping_add(-*dct.offset(0 as c_int as isize) as uint32_t)
            .wrapping_mul(mf as uint32_t)
            >> 16 as c_int) as int32_t) as dctcoef;
    }
    nz |= *dct.offset(0 as c_int as isize) as c_int;
    if *dct.offset(1 as c_int as isize) > 0 as dctcoef {
        *dct.offset(1 as c_int as isize) = ((bias as uint32_t)
            .wrapping_add(*dct.offset(1 as c_int as isize) as uint32_t)
            .wrapping_mul(mf as uint32_t)
            >> 16 as c_int) as dctcoef;
    } else {
        *dct.offset(1 as c_int as isize) = -(((bias as uint32_t)
            .wrapping_add(-*dct.offset(1 as c_int as isize) as uint32_t)
            .wrapping_mul(mf as uint32_t)
            >> 16 as c_int) as int32_t) as dctcoef;
    }
    nz |= *dct.offset(1 as c_int as isize) as c_int;
    if *dct.offset(2 as c_int as isize) > 0 as dctcoef {
        *dct.offset(2 as c_int as isize) = ((bias as uint32_t)
            .wrapping_add(*dct.offset(2 as c_int as isize) as uint32_t)
            .wrapping_mul(mf as uint32_t)
            >> 16 as c_int) as dctcoef;
    } else {
        *dct.offset(2 as c_int as isize) = -(((bias as uint32_t)
            .wrapping_add(-*dct.offset(2 as c_int as isize) as uint32_t)
            .wrapping_mul(mf as uint32_t)
            >> 16 as c_int) as int32_t) as dctcoef;
    }
    nz |= *dct.offset(2 as c_int as isize) as c_int;
    if *dct.offset(3 as c_int as isize) > 0 as dctcoef {
        *dct.offset(3 as c_int as isize) = ((bias as uint32_t)
            .wrapping_add(*dct.offset(3 as c_int as isize) as uint32_t)
            .wrapping_mul(mf as uint32_t)
            >> 16 as c_int) as dctcoef;
    } else {
        *dct.offset(3 as c_int as isize) = -(((bias as uint32_t)
            .wrapping_add(-*dct.offset(3 as c_int as isize) as uint32_t)
            .wrapping_mul(mf as uint32_t)
            >> 16 as c_int) as int32_t) as dctcoef;
    }
    nz |= *dct.offset(3 as c_int as isize) as c_int;
    return (nz != 0) as c_int;
}
#[c2rust::src_loc = "112:1"]
unsafe extern "C" fn dequant_4x4(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [c_int; 16],
    mut i_qp: c_int,
) {
    let i_mf: c_int = i_qp % 6 as c_int;
    let i_qbits: c_int = i_qp / 6 as c_int - 4 as c_int;
    if i_qbits >= 0 as c_int {
        let mut i: c_int = 0 as c_int;
        while i < 16 as c_int {
            *dct.offset(i as isize) = *dct.offset(i as isize)
                * (*dequant_mf.offset(i_mf as isize))[i as usize]
                * ((1 as dctcoef) << i_qbits);
            i += 1;
        }
    } else {
        let f: c_int = (1 as c_int) << -i_qbits - 1 as c_int;
        let mut i_0: c_int = 0 as c_int;
        while i_0 < 16 as c_int {
            *dct.offset(i_0 as isize) = *dct.offset(i_0 as isize)
                * (*dequant_mf.offset(i_mf as isize))[i_0 as usize]
                + f as dctcoef
                >> -i_qbits;
            i_0 += 1;
        }
    };
}
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn dequant_8x8(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [c_int; 64],
    mut i_qp: c_int,
) {
    let i_mf: c_int = i_qp % 6 as c_int;
    let i_qbits: c_int = i_qp / 6 as c_int - 6 as c_int;
    if i_qbits >= 0 as c_int {
        let mut i: c_int = 0 as c_int;
        while i < 64 as c_int {
            *dct.offset(i as isize) = *dct.offset(i as isize)
                * (*dequant_mf.offset(i_mf as isize))[i as usize]
                * ((1 as dctcoef) << i_qbits);
            i += 1;
        }
    } else {
        let f: c_int = (1 as c_int) << -i_qbits - 1 as c_int;
        let mut i_0: c_int = 0 as c_int;
        while i_0 < 64 as c_int {
            *dct.offset(i_0 as isize) = *dct.offset(i_0 as isize)
                * (*dequant_mf.offset(i_mf as isize))[i_0 as usize]
                + f as dctcoef
                >> -i_qbits;
            i_0 += 1;
        }
    };
}
#[c2rust::src_loc = "148:1"]
unsafe extern "C" fn dequant_4x4_dc(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [c_int; 16],
    mut i_qp: c_int,
) {
    let i_qbits: c_int = i_qp / 6 as c_int - 6 as c_int;
    if i_qbits >= 0 as c_int {
        let i_dmf: c_int =
            (*dequant_mf.offset((i_qp % 6 as c_int) as isize))[0 as c_int as usize] << i_qbits;
        let mut i: c_int = 0 as c_int;
        while i < 16 as c_int {
            let ref mut fresh6 = *dct.offset(i as isize);
            *fresh6 = (*fresh6 as c_int * i_dmf) as dctcoef;
            i += 1;
        }
    } else {
        let i_dmf_0: c_int =
            (*dequant_mf.offset((i_qp % 6 as c_int) as isize))[0 as c_int as usize];
        let f: c_int = (1 as c_int) << -i_qbits - 1 as c_int;
        let mut i_0: c_int = 0 as c_int;
        while i_0 < 16 as c_int {
            *dct.offset(i_0 as isize) =
                *dct.offset(i_0 as isize) * i_dmf_0 as dctcoef + f as dctcoef >> -i_qbits;
            i_0 += 1;
        }
    };
}
#[c2rust::src_loc = "185:1"]
unsafe extern "C" fn idct_dequant_2x4_dc(
    mut dct: *mut dctcoef,
    mut dct4x4: *mut [dctcoef; 16],
    mut dequant_mf: *mut [c_int; 16],
    mut i_qp: c_int,
) {
    let mut a0: c_int = *dct.offset(0 as c_int as isize) + *dct.offset(1 as c_int as isize);
    let mut a1: c_int = *dct.offset(2 as c_int as isize) + *dct.offset(3 as c_int as isize);
    let mut a2: c_int = *dct.offset(4 as c_int as isize) + *dct.offset(5 as c_int as isize);
    let mut a3: c_int = *dct.offset(6 as c_int as isize) + *dct.offset(7 as c_int as isize);
    let mut a4: c_int = *dct.offset(0 as c_int as isize) - *dct.offset(1 as c_int as isize);
    let mut a5: c_int = *dct.offset(2 as c_int as isize) - *dct.offset(3 as c_int as isize);
    let mut a6: c_int = *dct.offset(4 as c_int as isize) - *dct.offset(5 as c_int as isize);
    let mut a7: c_int = *dct.offset(6 as c_int as isize) - *dct.offset(7 as c_int as isize);
    let mut b0: c_int = a0 + a1;
    let mut b1: c_int = a2 + a3;
    let mut b2: c_int = a4 + a5;
    let mut b3: c_int = a6 + a7;
    let mut b4: c_int = a0 - a1;
    let mut b5: c_int = a2 - a3;
    let mut b6: c_int = a4 - a5;
    let mut b7: c_int = a6 - a7;
    let mut dmf: c_int = (*dequant_mf.offset((i_qp % 6 as c_int) as isize))[0 as c_int as usize]
        << i_qp / 6 as c_int;
    (*dct4x4.offset(0 as c_int as isize))[0 as c_int as usize] =
        ((b0 + b1) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    (*dct4x4.offset(1 as c_int as isize))[0 as c_int as usize] =
        ((b2 + b3) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    (*dct4x4.offset(2 as c_int as isize))[0 as c_int as usize] =
        ((b0 - b1) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    (*dct4x4.offset(3 as c_int as isize))[0 as c_int as usize] =
        ((b2 - b3) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    (*dct4x4.offset(4 as c_int as isize))[0 as c_int as usize] =
        ((b4 - b5) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    (*dct4x4.offset(5 as c_int as isize))[0 as c_int as usize] =
        ((b6 - b7) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    (*dct4x4.offset(6 as c_int as isize))[0 as c_int as usize] =
        ((b4 + b5) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    (*dct4x4.offset(7 as c_int as isize))[0 as c_int as usize] =
        ((b6 + b7) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
}
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn idct_dequant_2x4_dconly(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [c_int; 16],
    mut i_qp: c_int,
) {
    let mut a0: c_int = *dct.offset(0 as c_int as isize) + *dct.offset(1 as c_int as isize);
    let mut a1: c_int = *dct.offset(2 as c_int as isize) + *dct.offset(3 as c_int as isize);
    let mut a2: c_int = *dct.offset(4 as c_int as isize) + *dct.offset(5 as c_int as isize);
    let mut a3: c_int = *dct.offset(6 as c_int as isize) + *dct.offset(7 as c_int as isize);
    let mut a4: c_int = *dct.offset(0 as c_int as isize) - *dct.offset(1 as c_int as isize);
    let mut a5: c_int = *dct.offset(2 as c_int as isize) - *dct.offset(3 as c_int as isize);
    let mut a6: c_int = *dct.offset(4 as c_int as isize) - *dct.offset(5 as c_int as isize);
    let mut a7: c_int = *dct.offset(6 as c_int as isize) - *dct.offset(7 as c_int as isize);
    let mut b0: c_int = a0 + a1;
    let mut b1: c_int = a2 + a3;
    let mut b2: c_int = a4 + a5;
    let mut b3: c_int = a6 + a7;
    let mut b4: c_int = a0 - a1;
    let mut b5: c_int = a2 - a3;
    let mut b6: c_int = a4 - a5;
    let mut b7: c_int = a6 - a7;
    let mut dmf: c_int = (*dequant_mf.offset((i_qp % 6 as c_int) as isize))[0 as c_int as usize]
        << i_qp / 6 as c_int;
    *dct.offset(0 as c_int as isize) = ((b0 + b1) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    *dct.offset(1 as c_int as isize) = ((b2 + b3) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    *dct.offset(2 as c_int as isize) = ((b0 - b1) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    *dct.offset(3 as c_int as isize) = ((b2 - b3) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    *dct.offset(4 as c_int as isize) = ((b4 - b5) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    *dct.offset(5 as c_int as isize) = ((b6 - b7) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    *dct.offset(6 as c_int as isize) = ((b4 + b5) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
    *dct.offset(7 as c_int as isize) = ((b6 + b7) * dmf + 32 as c_int >> 6 as c_int) as dctcoef;
}
#[inline(always)]
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn optimize_chroma_idct_dequant_2x4(
    mut out: *mut dctcoef,
    mut dct: *mut dctcoef,
    mut dmf: c_int,
) {
    let mut a0: c_int = *dct.offset(0 as c_int as isize) + *dct.offset(1 as c_int as isize);
    let mut a1: c_int = *dct.offset(2 as c_int as isize) + *dct.offset(3 as c_int as isize);
    let mut a2: c_int = *dct.offset(4 as c_int as isize) + *dct.offset(5 as c_int as isize);
    let mut a3: c_int = *dct.offset(6 as c_int as isize) + *dct.offset(7 as c_int as isize);
    let mut a4: c_int = *dct.offset(0 as c_int as isize) - *dct.offset(1 as c_int as isize);
    let mut a5: c_int = *dct.offset(2 as c_int as isize) - *dct.offset(3 as c_int as isize);
    let mut a6: c_int = *dct.offset(4 as c_int as isize) - *dct.offset(5 as c_int as isize);
    let mut a7: c_int = *dct.offset(6 as c_int as isize) - *dct.offset(7 as c_int as isize);
    let mut b0: c_int = a0 + a1;
    let mut b1: c_int = a2 + a3;
    let mut b2: c_int = a4 + a5;
    let mut b3: c_int = a6 + a7;
    let mut b4: c_int = a0 - a1;
    let mut b5: c_int = a2 - a3;
    let mut b6: c_int = a4 - a5;
    let mut b7: c_int = a6 - a7;
    *out.offset(0 as c_int as isize) = ((b0 + b1) * dmf + 2080 as c_int >> 6 as c_int) as dctcoef;
    *out.offset(1 as c_int as isize) = ((b2 + b3) * dmf + 2080 as c_int >> 6 as c_int) as dctcoef;
    *out.offset(2 as c_int as isize) = ((b0 - b1) * dmf + 2080 as c_int >> 6 as c_int) as dctcoef;
    *out.offset(3 as c_int as isize) = ((b2 - b3) * dmf + 2080 as c_int >> 6 as c_int) as dctcoef;
    *out.offset(4 as c_int as isize) = ((b4 - b5) * dmf + 2080 as c_int >> 6 as c_int) as dctcoef;
    *out.offset(5 as c_int as isize) = ((b6 - b7) * dmf + 2080 as c_int >> 6 as c_int) as dctcoef;
    *out.offset(6 as c_int as isize) = ((b4 + b5) * dmf + 2080 as c_int >> 6 as c_int) as dctcoef;
    *out.offset(7 as c_int as isize) = ((b6 + b7) * dmf + 2080 as c_int >> 6 as c_int) as dctcoef;
}
#[inline(always)]
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn optimize_chroma_idct_dequant_2x2(
    mut out: *mut dctcoef,
    mut dct: *mut dctcoef,
    mut dmf: c_int,
) {
    let mut d0: c_int = *dct.offset(0 as c_int as isize) + *dct.offset(1 as c_int as isize);
    let mut d1: c_int = *dct.offset(2 as c_int as isize) + *dct.offset(3 as c_int as isize);
    let mut d2: c_int = *dct.offset(0 as c_int as isize) - *dct.offset(1 as c_int as isize);
    let mut d3: c_int = *dct.offset(2 as c_int as isize) - *dct.offset(3 as c_int as isize);
    *out.offset(0 as c_int as isize) = (((d0 + d1) * dmf >> 5 as c_int) + 32 as c_int) as dctcoef;
    *out.offset(1 as c_int as isize) = (((d0 - d1) * dmf >> 5 as c_int) + 32 as c_int) as dctcoef;
    *out.offset(2 as c_int as isize) = (((d2 + d3) * dmf >> 5 as c_int) + 32 as c_int) as dctcoef;
    *out.offset(3 as c_int as isize) = (((d2 - d3) * dmf >> 5 as c_int) + 32 as c_int) as dctcoef;
}
#[inline(always)]
#[c2rust::src_loc = "239:1"]
unsafe extern "C" fn optimize_chroma_round(
    mut ref_0: *mut dctcoef,
    mut dct: *mut dctcoef,
    mut dequant_mf: c_int,
    mut chroma422: c_int,
) -> c_int {
    let mut out: [dctcoef; 8] = [0; 8];
    if chroma422 != 0 {
        optimize_chroma_idct_dequant_2x4(out.as_mut_ptr(), dct as *mut dctcoef, dequant_mf);
    } else {
        optimize_chroma_idct_dequant_2x2(out.as_mut_ptr(), dct as *mut dctcoef, dequant_mf);
    }
    let mut sum: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i
        < (if chroma422 != 0 {
            8 as c_int
        } else {
            4 as c_int
        })
    {
        sum |= (*ref_0.offset(i as isize) ^ out[i as usize]) as c_int;
        i += 1;
    }
    return sum >> 6 as c_int;
}
#[inline(always)]
#[c2rust::src_loc = "254:1"]
unsafe extern "C" fn optimize_chroma_dc_internal(
    mut dct: *mut dctcoef,
    mut dequant_mf: c_int,
    mut chroma422: c_int,
) -> c_int {
    let mut dct_orig: [dctcoef; 8] = [0; 8];
    let mut coeff: c_int = 0;
    let mut nz: c_int = 0;
    if chroma422 != 0 {
        optimize_chroma_idct_dequant_2x4(dct_orig.as_mut_ptr(), dct as *mut dctcoef, dequant_mf);
    } else {
        optimize_chroma_idct_dequant_2x2(dct_orig.as_mut_ptr(), dct as *mut dctcoef, dequant_mf);
    }
    let mut sum: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i
        < (if chroma422 != 0 {
            8 as c_int
        } else {
            4 as c_int
        })
    {
        sum |= dct_orig[i as usize] as c_int;
        i += 1;
    }
    if sum >> 6 as c_int == 0 {
        return 0 as c_int;
    }
    nz = 0 as c_int;
    coeff = if chroma422 != 0 {
        7 as c_int
    } else {
        3 as c_int
    };
    while coeff >= 0 as c_int {
        let mut level: c_int = *dct.offset(coeff as isize);
        let mut sign: c_int = level >> 31 as c_int | 1 as c_int;
        while level != 0 {
            *dct.offset(coeff as isize) = (level - sign) as dctcoef;
            if optimize_chroma_round(dct_orig.as_mut_ptr(), dct, dequant_mf, chroma422) != 0 {
                nz = 1 as c_int;
                *dct.offset(coeff as isize) = level as dctcoef;
                break;
            } else {
                level -= sign;
            }
        }
        coeff -= 1;
    }
    return nz;
}
#[c2rust::src_loc = "294:1"]
unsafe extern "C" fn optimize_chroma_2x2_dc(mut dct: *mut dctcoef, mut dequant_mf: c_int) -> c_int {
    return optimize_chroma_dc_internal(dct as *mut dctcoef, dequant_mf, 0 as c_int);
}
#[c2rust::src_loc = "299:1"]
unsafe extern "C" fn optimize_chroma_2x4_dc(mut dct: *mut dctcoef, mut dequant_mf: c_int) -> c_int {
    return optimize_chroma_dc_internal(dct as *mut dctcoef, dequant_mf, 1 as c_int);
}
#[c2rust::src_loc = "304:1"]
unsafe extern "C" fn denoise_dct(
    mut dct: *mut dctcoef,
    mut sum: *mut uint32_t,
    mut offset: *mut udctcoef,
    mut size: c_int,
) {
    let mut i: c_int = 0 as c_int;
    while i < size {
        let mut level: c_int = *dct.offset(i as isize);
        let mut sign: c_int = level >> 31 as c_int;
        level = level + sign ^ sign;
        let ref mut fresh5 = *sum.offset(i as isize);
        *fresh5 = (*fresh5).wrapping_add(level as uint32_t);
        level = (level as udctcoef).wrapping_sub(*offset.offset(i as isize)) as c_int as c_int;
        *dct.offset(i as isize) = (if level < 0 as c_int {
            0 as c_int
        } else {
            (level ^ sign) - sign
        }) as dctcoef;
        i += 1;
    }
}
#[inline(always)]
#[c2rust::src_loc = "326:1"]
unsafe extern "C" fn decimate_score_internal(mut dct: *mut dctcoef, mut i_max: c_int) -> c_int {
    let mut ds_table: *const uint8_t = if i_max == 64 as c_int {
        x264_decimate_table8.as_ptr()
    } else {
        x264_decimate_table4.as_ptr()
    };
    let mut i_score: c_int = 0 as c_int;
    let mut idx: c_int = i_max - 1 as c_int;
    while idx >= 0 as c_int && *dct.offset(idx as isize) == 0 as dctcoef {
        idx -= 1;
    }
    while idx >= 0 as c_int {
        let mut i_run: c_int = 0;
        let fresh4 = idx;
        idx = idx - 1;
        if (*dct.offset(fresh4 as isize) + 1 as dctcoef) as c_uint > 2 as c_uint {
            return 9 as c_int;
        }
        i_run = 0 as c_int;
        while idx >= 0 as c_int && *dct.offset(idx as isize) == 0 as dctcoef {
            idx -= 1;
            i_run += 1;
        }
        i_score += *ds_table.offset(i_run as isize) as c_int;
    }
    return i_score;
}
#[c2rust::src_loc = "353:1"]
unsafe extern "C" fn decimate_score15(mut dct: *mut dctcoef) -> c_int {
    return decimate_score_internal(dct.offset(1 as c_int as isize), 15 as c_int);
}
#[c2rust::src_loc = "357:1"]
unsafe extern "C" fn decimate_score16(mut dct: *mut dctcoef) -> c_int {
    return decimate_score_internal(dct, 16 as c_int);
}
#[c2rust::src_loc = "361:1"]
unsafe extern "C" fn decimate_score64(mut dct: *mut dctcoef) -> c_int {
    return decimate_score_internal(dct, 64 as c_int);
}
#[c2rust::src_loc = "375:1"]
unsafe extern "C" fn coeff_last4(mut l: *mut dctcoef) -> c_int {
    let mut i_last: c_int = 4 as c_int - 1 as c_int;
    while i_last >= 0 as c_int && *l.offset(i_last as isize) == 0 as dctcoef {
        i_last -= 1;
    }
    return i_last;
}
#[c2rust::src_loc = "376:1"]
unsafe extern "C" fn coeff_last8(mut l: *mut dctcoef) -> c_int {
    let mut i_last: c_int = 8 as c_int - 1 as c_int;
    while i_last >= 0 as c_int && *l.offset(i_last as isize) == 0 as dctcoef {
        i_last -= 1;
    }
    return i_last;
}
#[c2rust::src_loc = "377:1"]
unsafe extern "C" fn coeff_last15(mut l: *mut dctcoef) -> c_int {
    let mut i_last: c_int = 15 as c_int - 1 as c_int;
    while i_last >= 0 as c_int && *l.offset(i_last as isize) == 0 as dctcoef {
        i_last -= 1;
    }
    return i_last;
}
#[c2rust::src_loc = "378:1"]
unsafe extern "C" fn coeff_last16(mut l: *mut dctcoef) -> c_int {
    let mut i_last: c_int = 16 as c_int - 1 as c_int;
    while i_last >= 0 as c_int && *l.offset(i_last as isize) == 0 as dctcoef {
        i_last -= 1;
    }
    return i_last;
}
#[c2rust::src_loc = "379:1"]
unsafe extern "C" fn coeff_last64(mut l: *mut dctcoef) -> c_int {
    let mut i_last: c_int = 64 as c_int - 1 as c_int;
    while i_last >= 0 as c_int && *l.offset(i_last as isize) == 0 as dctcoef {
        i_last -= 1;
    }
    return i_last;
}
#[c2rust::src_loc = "397:1"]
unsafe extern "C" fn coeff_level_run4(
    mut dct: *mut dctcoef,
    mut runlevel: *mut x264_run_level_t,
) -> c_int {
    (*runlevel).last = coeff_last4(dct) as int32_t;
    let mut i_last: c_int = (*runlevel).last;
    let mut i_total: c_int = 0 as c_int;
    let mut mask: c_int = 0 as c_int;
    loop {
        let fresh3 = i_total;
        i_total = i_total + 1;
        (*runlevel).level[fresh3 as usize] = *dct.offset(i_last as isize);
        mask |= (1 as c_int) << i_last;
        loop {
            i_last -= 1;
            if !(i_last >= 0 as c_int && *dct.offset(i_last as isize) == 0 as dctcoef) {
                break;
            }
        }
        if !(i_last >= 0 as c_int) {
            break;
        }
    }
    (*runlevel).mask = mask as int32_t;
    return i_total;
}
#[c2rust::src_loc = "398:1"]
unsafe extern "C" fn coeff_level_run8(
    mut dct: *mut dctcoef,
    mut runlevel: *mut x264_run_level_t,
) -> c_int {
    (*runlevel).last = coeff_last8(dct) as int32_t;
    let mut i_last: c_int = (*runlevel).last;
    let mut i_total: c_int = 0 as c_int;
    let mut mask: c_int = 0 as c_int;
    loop {
        let fresh2 = i_total;
        i_total = i_total + 1;
        (*runlevel).level[fresh2 as usize] = *dct.offset(i_last as isize);
        mask |= (1 as c_int) << i_last;
        loop {
            i_last -= 1;
            if !(i_last >= 0 as c_int && *dct.offset(i_last as isize) == 0 as dctcoef) {
                break;
            }
        }
        if !(i_last >= 0 as c_int) {
            break;
        }
    }
    (*runlevel).mask = mask as int32_t;
    return i_total;
}
#[c2rust::src_loc = "399:1"]
unsafe extern "C" fn coeff_level_run15(
    mut dct: *mut dctcoef,
    mut runlevel: *mut x264_run_level_t,
) -> c_int {
    (*runlevel).last = coeff_last15(dct) as int32_t;
    let mut i_last: c_int = (*runlevel).last;
    let mut i_total: c_int = 0 as c_int;
    let mut mask: c_int = 0 as c_int;
    loop {
        let fresh1 = i_total;
        i_total = i_total + 1;
        (*runlevel).level[fresh1 as usize] = *dct.offset(i_last as isize);
        mask |= (1 as c_int) << i_last;
        loop {
            i_last -= 1;
            if !(i_last >= 0 as c_int && *dct.offset(i_last as isize) == 0 as dctcoef) {
                break;
            }
        }
        if !(i_last >= 0 as c_int) {
            break;
        }
    }
    (*runlevel).mask = mask as int32_t;
    return i_total;
}
#[c2rust::src_loc = "400:1"]
unsafe extern "C" fn coeff_level_run16(
    mut dct: *mut dctcoef,
    mut runlevel: *mut x264_run_level_t,
) -> c_int {
    (*runlevel).last = coeff_last16(dct) as int32_t;
    let mut i_last: c_int = (*runlevel).last;
    let mut i_total: c_int = 0 as c_int;
    let mut mask: c_int = 0 as c_int;
    loop {
        let fresh0 = i_total;
        i_total = i_total + 1;
        (*runlevel).level[fresh0 as usize] = *dct.offset(i_last as isize);
        mask |= (1 as c_int) << i_last;
        loop {
            i_last -= 1;
            if !(i_last >= 0 as c_int && *dct.offset(i_last as isize) == 0 as dctcoef) {
                break;
            }
        }
        if !(i_last >= 0 as c_int) {
            break;
        }
    }
    (*runlevel).mask = mask as int32_t;
    return i_total;
}
#[no_mangle]
#[c2rust::src_loc = "414:1"]
pub unsafe extern "C" fn x264_quant_init(
    mut _h: *mut x264_t,
    mut _cpu: uint32_t,
    mut pf: *mut x264_quant_function_t,
) {
    (*pf).quant_8x8 = Some(
        quant_8x8 as unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> c_int>;
    (*pf).quant_4x4 = Some(
        quant_4x4 as unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> c_int>;
    (*pf).quant_4x4x4 = Some(
        quant_4x4x4
            as unsafe extern "C" fn(*mut [dctcoef; 16], *mut udctcoef, *mut udctcoef) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut [dctcoef; 16], *mut udctcoef, *mut udctcoef) -> c_int>;
    (*pf).quant_4x4_dc =
        Some(quant_4x4_dc as unsafe extern "C" fn(*mut dctcoef, c_int, c_int) -> c_int)
            as Option<unsafe extern "C" fn(*mut dctcoef, c_int, c_int) -> c_int>;
    (*pf).quant_2x2_dc =
        Some(quant_2x2_dc as unsafe extern "C" fn(*mut dctcoef, c_int, c_int) -> c_int)
            as Option<unsafe extern "C" fn(*mut dctcoef, c_int, c_int) -> c_int>;
    (*pf).dequant_4x4 =
        Some(dequant_4x4 as unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> ())
            as Option<unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> ()>;
    (*pf).dequant_4x4_dc =
        Some(dequant_4x4_dc as unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> ())
            as Option<unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> ()>;
    (*pf).dequant_8x8 =
        Some(dequant_8x8 as unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 64], c_int) -> ())
            as Option<unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 64], c_int) -> ()>;
    (*pf).idct_dequant_2x4_dc = Some(
        idct_dequant_2x4_dc
            as unsafe extern "C" fn(
                *mut dctcoef,
                *mut [dctcoef; 16],
                *mut [c_int; 16],
                c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut dctcoef, *mut [dctcoef; 16], *mut [c_int; 16], c_int) -> (),
        >;
    (*pf).idct_dequant_2x4_dconly = Some(
        idct_dequant_2x4_dconly
            as unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut dctcoef, *mut [c_int; 16], c_int) -> ()>;
    (*pf).optimize_chroma_2x2_dc =
        Some(optimize_chroma_2x2_dc as unsafe extern "C" fn(*mut dctcoef, c_int) -> c_int)
            as Option<unsafe extern "C" fn(*mut dctcoef, c_int) -> c_int>;
    (*pf).optimize_chroma_2x4_dc =
        Some(optimize_chroma_2x4_dc as unsafe extern "C" fn(*mut dctcoef, c_int) -> c_int)
            as Option<unsafe extern "C" fn(*mut dctcoef, c_int) -> c_int>;
    (*pf).denoise_dct = Some(
        denoise_dct
            as unsafe extern "C" fn(*mut dctcoef, *mut uint32_t, *mut udctcoef, c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut dctcoef, *mut uint32_t, *mut udctcoef, c_int) -> ()>;
    (*pf).decimate_score15 = Some(decimate_score15 as unsafe extern "C" fn(*mut dctcoef) -> c_int)
        as Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>;
    (*pf).decimate_score16 = Some(decimate_score16 as unsafe extern "C" fn(*mut dctcoef) -> c_int)
        as Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>;
    (*pf).decimate_score64 = Some(decimate_score64 as unsafe extern "C" fn(*mut dctcoef) -> c_int)
        as Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>;
    (*pf).coeff_last4 = Some(coeff_last4 as unsafe extern "C" fn(*mut dctcoef) -> c_int)
        as Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>;
    (*pf).coeff_last8 = Some(coeff_last8 as unsafe extern "C" fn(*mut dctcoef) -> c_int)
        as Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>;
    (*pf).coeff_last[DCT_LUMA_AC as c_int as usize] =
        Some(coeff_last15 as unsafe extern "C" fn(*mut dctcoef) -> c_int)
            as Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>;
    (*pf).coeff_last[DCT_LUMA_4x4 as c_int as usize] =
        Some(coeff_last16 as unsafe extern "C" fn(*mut dctcoef) -> c_int)
            as Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>;
    (*pf).coeff_last[DCT_LUMA_8x8 as c_int as usize] =
        Some(coeff_last64 as unsafe extern "C" fn(*mut dctcoef) -> c_int)
            as Option<unsafe extern "C" fn(*mut dctcoef) -> c_int>;
    (*pf).coeff_level_run4 = Some(
        coeff_level_run4 as unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int>;
    (*pf).coeff_level_run8 = Some(
        coeff_level_run8 as unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int>;
    (*pf).coeff_level_run[DCT_LUMA_AC as c_int as usize] = Some(
        coeff_level_run15 as unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int>;
    (*pf).coeff_level_run[DCT_LUMA_4x4 as c_int as usize] = Some(
        coeff_level_run16 as unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> c_int>;
    (*pf).coeff_last[DCT_CHROMAV_4x4 as c_int as usize] =
        (*pf).coeff_last[DCT_LUMA_4x4 as c_int as usize];
    (*pf).coeff_last[DCT_CHROMAU_4x4 as c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAV_4x4 as c_int as usize];
    (*pf).coeff_last[DCT_CHROMAV_DC as c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAU_4x4 as c_int as usize];
    (*pf).coeff_last[DCT_CHROMAU_DC as c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAV_DC as c_int as usize];
    (*pf).coeff_last[DCT_LUMA_DC as c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAU_DC as c_int as usize];
    (*pf).coeff_last[DCT_CHROMAV_AC as c_int as usize] =
        (*pf).coeff_last[DCT_LUMA_AC as c_int as usize];
    (*pf).coeff_last[DCT_CHROMAU_AC as c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAV_AC as c_int as usize];
    (*pf).coeff_last[DCT_CHROMA_AC as c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAU_AC as c_int as usize];
    (*pf).coeff_last[DCT_CHROMAV_8x8 as c_int as usize] =
        (*pf).coeff_last[DCT_LUMA_8x8 as c_int as usize];
    (*pf).coeff_last[DCT_CHROMAU_8x8 as c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAV_8x8 as c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAV_4x4 as c_int as usize] =
        (*pf).coeff_level_run[DCT_LUMA_4x4 as c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAU_4x4 as c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAV_4x4 as c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAV_DC as c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAU_4x4 as c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAU_DC as c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAV_DC as c_int as usize];
    (*pf).coeff_level_run[DCT_LUMA_DC as c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAU_DC as c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAV_AC as c_int as usize] =
        (*pf).coeff_level_run[DCT_LUMA_AC as c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAU_AC as c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAV_AC as c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMA_AC as c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAU_AC as c_int as usize];
}
