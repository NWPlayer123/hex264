// =============== BEGIN quant_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_quant_function_t {
    pub quant_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::udctcoef,
            *mut crate::src::common::common::udctcoef,
        ) -> ::core::ffi::c_int,
    >,
    pub quant_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::udctcoef,
            *mut crate::src::common::common::udctcoef,
        ) -> ::core::ffi::c_int,
    >,
    pub quant_4x4x4: Option<
        unsafe extern "C" fn(
            *mut [crate::src::common::common::dctcoef; 16],
            *mut crate::src::common::common::udctcoef,
            *mut crate::src::common::common::udctcoef,
        ) -> ::core::ffi::c_int,
    >,
    pub quant_4x4_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub quant_2x2_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub dequant_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut [::core::ffi::c_int; 64],
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub dequant_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut [::core::ffi::c_int; 16],
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub dequant_4x4_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut [::core::ffi::c_int; 16],
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub idct_dequant_2x4_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut [crate::src::common::common::dctcoef; 16],
            *mut [::core::ffi::c_int; 16],
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub idct_dequant_2x4_dconly: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut [::core::ffi::c_int; 16],
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub optimize_chroma_2x2_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub optimize_chroma_2x4_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub denoise_dct: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::stdlib::uint32_t,
            *mut crate::src::common::common::udctcoef,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub decimate_score15: Option<
        unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ::core::ffi::c_int,
    >,
    pub decimate_score16: Option<
        unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ::core::ffi::c_int,
    >,
    pub decimate_score64: Option<
        unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ::core::ffi::c_int,
    >,
    pub coeff_last: [Option<
        unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ::core::ffi::c_int,
    >; 14],
    pub coeff_last4: Option<
        unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ::core::ffi::c_int,
    >,
    pub coeff_last8: Option<
        unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ::core::ffi::c_int,
    >,
    pub coeff_level_run: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::bitstream::x264_run_level_t,
        ) -> ::core::ffi::c_int,
    >; 13],
    pub coeff_level_run4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::bitstream::x264_run_level_t,
        ) -> ::core::ffi::c_int,
    >,
    pub coeff_level_run8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::bitstream::x264_run_level_t,
        ) -> ::core::ffi::c_int,
    >,
    pub trellis_cabac_4x4: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_int,
            *const crate::stdlib::uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::stdlib::uint8_t,
            *mut crate::stdlib::uint8_t,
            crate::stdlib::uint64_t,
            crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub trellis_cabac_8x8: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_int,
            *const crate::stdlib::uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::stdlib::uint8_t,
            *mut crate::stdlib::uint8_t,
            crate::stdlib::uint64_t,
            crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub trellis_cabac_4x4_psy: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_int,
            *const crate::stdlib::uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::stdlib::uint8_t,
            *mut crate::stdlib::uint8_t,
            crate::stdlib::uint64_t,
            crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub trellis_cabac_8x8_psy: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_int,
            *const crate::stdlib::uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::stdlib::uint8_t,
            *mut crate::stdlib::uint8_t,
            crate::stdlib::uint64_t,
            crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub trellis_cabac_dc: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_int,
            *const crate::stdlib::uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::stdlib::uint8_t,
            *mut crate::stdlib::uint8_t,
            crate::stdlib::uint64_t,
            crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub trellis_cabac_chroma_422_dc: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_int,
            *const crate::stdlib::uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::stdlib::uint8_t,
            *mut crate::stdlib::uint8_t,
            crate::stdlib::uint64_t,
            crate::stdlib::uint16_t,
        ) -> ::core::ffi::c_int,
    >,
}
unsafe extern "C" fn quant_8x8(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut mf: *mut crate::src::common::common::udctcoef,
    mut bias: *mut crate::src::common::common::udctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 64 as ::core::ffi::c_int {
            if *dct.offset(i as isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *dct.offset(i as isize) = ((*bias.offset(i as isize) as crate::stdlib::uint32_t)
                    .wrapping_add(*dct.offset(i as isize) as crate::stdlib::uint32_t)
                    .wrapping_mul(*mf.offset(i as isize) as crate::stdlib::uint32_t)
                    >> 16 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            } else {
                *dct.offset(i as isize) = -(((*bias.offset(i as isize) as crate::stdlib::uint32_t)
                    .wrapping_add(
                        -(*dct.offset(i as isize) as ::core::ffi::c_int) as crate::stdlib::uint32_t,
                    )
                    .wrapping_mul(*mf.offset(i as isize) as crate::stdlib::uint32_t)
                    >> 16 as ::core::ffi::c_int)
                    as crate::stdlib::int32_t)
                    as crate::src::common::common::dctcoef;
            }
            nz |= *dct.offset(i as isize) as ::core::ffi::c_int;
            i += 1;
        }
        return (nz != 0) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn quant_4x4(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut mf: *mut crate::src::common::common::udctcoef,
    mut bias: *mut crate::src::common::common::udctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            if *dct.offset(i as isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *dct.offset(i as isize) = ((*bias.offset(i as isize) as crate::stdlib::uint32_t)
                    .wrapping_add(*dct.offset(i as isize) as crate::stdlib::uint32_t)
                    .wrapping_mul(*mf.offset(i as isize) as crate::stdlib::uint32_t)
                    >> 16 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            } else {
                *dct.offset(i as isize) = -(((*bias.offset(i as isize) as crate::stdlib::uint32_t)
                    .wrapping_add(
                        -(*dct.offset(i as isize) as ::core::ffi::c_int) as crate::stdlib::uint32_t,
                    )
                    .wrapping_mul(*mf.offset(i as isize) as crate::stdlib::uint32_t)
                    >> 16 as ::core::ffi::c_int)
                    as crate::stdlib::int32_t)
                    as crate::src::common::common::dctcoef;
            }
            nz |= *dct.offset(i as isize) as ::core::ffi::c_int;
            i += 1;
        }
        return (nz != 0) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn quant_4x4x4(
    mut dct: *mut [crate::src::common::common::dctcoef; 16],
    mut mf: *mut crate::src::common::common::udctcoef,
    mut bias: *mut crate::src::common::common::udctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nza: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < 4 as ::core::ffi::c_int {
            let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                if (*dct.offset(j as isize))[i as usize] as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    (*dct.offset(j as isize))[i as usize] = ((*bias.offset(i as isize)
                        as crate::stdlib::uint32_t)
                        .wrapping_add(
                            (*dct.offset(j as isize))[i as usize] as crate::stdlib::uint32_t,
                        )
                        .wrapping_mul(*mf.offset(i as isize) as crate::stdlib::uint32_t)
                        >> 16 as ::core::ffi::c_int)
                        as crate::src::common::common::dctcoef;
                } else {
                    (*dct.offset(j as isize))[i as usize] = -(((*bias.offset(i as isize)
                        as crate::stdlib::uint32_t)
                        .wrapping_add(
                            -((*dct.offset(j as isize))[i as usize] as ::core::ffi::c_int)
                                as crate::stdlib::uint32_t,
                        )
                        .wrapping_mul(*mf.offset(i as isize) as crate::stdlib::uint32_t)
                        >> 16 as ::core::ffi::c_int)
                        as crate::stdlib::int32_t)
                        as crate::src::common::common::dctcoef;
                }
                nz |= (*dct.offset(j as isize))[i as usize] as ::core::ffi::c_int;
                i += 1;
            }
            nza |= ((nz != 0) as ::core::ffi::c_int) << j;
            j += 1;
        }
        return nza;
    }
}
unsafe extern "C" fn quant_4x4_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut mf: ::core::ffi::c_int,
    mut bias: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            if *dct.offset(i as isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *dct.offset(i as isize) = ((bias as crate::stdlib::uint32_t)
                    .wrapping_add(*dct.offset(i as isize) as crate::stdlib::uint32_t)
                    .wrapping_mul(mf as crate::stdlib::uint32_t)
                    >> 16 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            } else {
                *dct.offset(i as isize) = -(((bias as crate::stdlib::uint32_t)
                    .wrapping_add(
                        -(*dct.offset(i as isize) as ::core::ffi::c_int) as crate::stdlib::uint32_t,
                    )
                    .wrapping_mul(mf as crate::stdlib::uint32_t)
                    >> 16 as ::core::ffi::c_int)
                    as crate::stdlib::int32_t)
                    as crate::src::common::common::dctcoef;
            }
            nz |= *dct.offset(i as isize) as ::core::ffi::c_int;
            i += 1;
        }
        return (nz != 0) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn quant_2x2_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut mf: ::core::ffi::c_int,
    mut bias: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if *dct.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            *dct.offset(0 as ::core::ffi::c_int as isize) = ((bias as crate::stdlib::uint32_t)
                .wrapping_add(
                    *dct.offset(0 as ::core::ffi::c_int as isize) as crate::stdlib::uint32_t
                )
                .wrapping_mul(mf as crate::stdlib::uint32_t)
                >> 16 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        } else {
            *dct.offset(0 as ::core::ffi::c_int as isize) = -(((bias as crate::stdlib::uint32_t)
                .wrapping_add(
                    -(*dct.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                )
                .wrapping_mul(mf as crate::stdlib::uint32_t)
                >> 16 as ::core::ffi::c_int)
                as crate::stdlib::int32_t)
                as crate::src::common::common::dctcoef;
        }
        nz |= *dct.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            *dct.offset(1 as ::core::ffi::c_int as isize) = ((bias as crate::stdlib::uint32_t)
                .wrapping_add(
                    *dct.offset(1 as ::core::ffi::c_int as isize) as crate::stdlib::uint32_t
                )
                .wrapping_mul(mf as crate::stdlib::uint32_t)
                >> 16 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        } else {
            *dct.offset(1 as ::core::ffi::c_int as isize) = -(((bias as crate::stdlib::uint32_t)
                .wrapping_add(
                    -(*dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                )
                .wrapping_mul(mf as crate::stdlib::uint32_t)
                >> 16 as ::core::ffi::c_int)
                as crate::stdlib::int32_t)
                as crate::src::common::common::dctcoef;
        }
        nz |= *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if *dct.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            *dct.offset(2 as ::core::ffi::c_int as isize) = ((bias as crate::stdlib::uint32_t)
                .wrapping_add(
                    *dct.offset(2 as ::core::ffi::c_int as isize) as crate::stdlib::uint32_t
                )
                .wrapping_mul(mf as crate::stdlib::uint32_t)
                >> 16 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        } else {
            *dct.offset(2 as ::core::ffi::c_int as isize) = -(((bias as crate::stdlib::uint32_t)
                .wrapping_add(
                    -(*dct.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                )
                .wrapping_mul(mf as crate::stdlib::uint32_t)
                >> 16 as ::core::ffi::c_int)
                as crate::stdlib::int32_t)
                as crate::src::common::common::dctcoef;
        }
        nz |= *dct.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            *dct.offset(3 as ::core::ffi::c_int as isize) = ((bias as crate::stdlib::uint32_t)
                .wrapping_add(
                    *dct.offset(3 as ::core::ffi::c_int as isize) as crate::stdlib::uint32_t
                )
                .wrapping_mul(mf as crate::stdlib::uint32_t)
                >> 16 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        } else {
            *dct.offset(3 as ::core::ffi::c_int as isize) = -(((bias as crate::stdlib::uint32_t)
                .wrapping_add(
                    -(*dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                )
                .wrapping_mul(mf as crate::stdlib::uint32_t)
                >> 16 as ::core::ffi::c_int)
                as crate::stdlib::int32_t)
                as crate::src::common::common::dctcoef;
        }
        nz |= *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        return (nz != 0) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn dequant_4x4(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let i_mf: ::core::ffi::c_int = i_qp % 6 as ::core::ffi::c_int;
        let i_qbits: ::core::ffi::c_int = i_qp / 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        if i_qbits >= 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                *dct.offset(i as isize) = (*dct.offset(i as isize) as ::core::ffi::c_int
                    * (*dequant_mf.offset(i_mf as isize))[i as usize]
                    * ((1 as ::core::ffi::c_int) << i_qbits))
                    as crate::src::common::common::dctcoef;
                i += 1;
            }
        } else {
            let f: ::core::ffi::c_int =
                (1 as ::core::ffi::c_int) << -i_qbits - 1 as ::core::ffi::c_int;
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 16 as ::core::ffi::c_int {
                *dct.offset(i_0 as isize) = (*dct.offset(i_0 as isize) as ::core::ffi::c_int
                    * (*dequant_mf.offset(i_mf as isize))[i_0 as usize]
                    + f
                    >> -i_qbits)
                    as crate::src::common::common::dctcoef;
                i_0 += 1;
            }
        };
    }
}
unsafe extern "C" fn dequant_8x8(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: *mut [::core::ffi::c_int; 64],
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let i_mf: ::core::ffi::c_int = i_qp % 6 as ::core::ffi::c_int;
        let i_qbits: ::core::ffi::c_int = i_qp / 6 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        if i_qbits >= 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 64 as ::core::ffi::c_int {
                *dct.offset(i as isize) = (*dct.offset(i as isize) as ::core::ffi::c_int
                    * (*dequant_mf.offset(i_mf as isize))[i as usize]
                    * ((1 as ::core::ffi::c_int) << i_qbits))
                    as crate::src::common::common::dctcoef;
                i += 1;
            }
        } else {
            let f: ::core::ffi::c_int =
                (1 as ::core::ffi::c_int) << -i_qbits - 1 as ::core::ffi::c_int;
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 64 as ::core::ffi::c_int {
                *dct.offset(i_0 as isize) = (*dct.offset(i_0 as isize) as ::core::ffi::c_int
                    * (*dequant_mf.offset(i_mf as isize))[i_0 as usize]
                    + f
                    >> -i_qbits)
                    as crate::src::common::common::dctcoef;
                i_0 += 1;
            }
        };
    }
}
unsafe extern "C" fn dequant_4x4_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let i_qbits: ::core::ffi::c_int = i_qp / 6 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        if i_qbits >= 0 as ::core::ffi::c_int {
            let i_dmf: ::core::ffi::c_int = (*dequant_mf
                .offset((i_qp % 6 as ::core::ffi::c_int) as isize))
                [0 as ::core::ffi::c_int as usize]
                << i_qbits;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let ref mut c2rust_fresh6 = *dct.offset(i as isize);
                *c2rust_fresh6 = (*c2rust_fresh6 as ::core::ffi::c_int * i_dmf)
                    as crate::src::common::common::dctcoef;
                i += 1;
            }
        } else {
            let i_dmf_0: ::core::ffi::c_int = (*dequant_mf
                .offset((i_qp % 6 as ::core::ffi::c_int) as isize))
                [0 as ::core::ffi::c_int as usize];
            let f: ::core::ffi::c_int =
                (1 as ::core::ffi::c_int) << -i_qbits - 1 as ::core::ffi::c_int;
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 16 as ::core::ffi::c_int {
                *dct.offset(i_0 as isize) =
                    (*dct.offset(i_0 as isize) as ::core::ffi::c_int * i_dmf_0 + f >> -i_qbits)
                        as crate::src::common::common::dctcoef;
                i_0 += 1;
            }
        };
    }
}
unsafe extern "C" fn idct_dequant_2x4_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dct4x4: *mut [crate::src::common::common::dctcoef; 16],
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let mut a0: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a1: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a2: ::core::ffi::c_int = *dct.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a3: ::core::ffi::c_int = *dct.offset(6 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a4: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a5: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a6: ::core::ffi::c_int = *dct.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a7: ::core::ffi::c_int = *dct.offset(6 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut b0: ::core::ffi::c_int = a0 + a1;
        let mut b1: ::core::ffi::c_int = a2 + a3;
        let mut b2: ::core::ffi::c_int = a4 + a5;
        let mut b3: ::core::ffi::c_int = a6 + a7;
        let mut b4: ::core::ffi::c_int = a0 - a1;
        let mut b5: ::core::ffi::c_int = a2 - a3;
        let mut b6: ::core::ffi::c_int = a4 - a5;
        let mut b7: ::core::ffi::c_int = a6 - a7;
        let mut dmf: ::core::ffi::c_int = (*dequant_mf
            .offset((i_qp % 6 as ::core::ffi::c_int) as isize))[0 as ::core::ffi::c_int as usize]
            << i_qp / 6 as ::core::ffi::c_int;
        (*dct4x4.offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            ((b0 + b1) * dmf + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        (*dct4x4.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            ((b2 + b3) * dmf + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        (*dct4x4.offset(2 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            ((b0 - b1) * dmf + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        (*dct4x4.offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            ((b2 - b3) * dmf + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        (*dct4x4.offset(4 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            ((b4 - b5) * dmf + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        (*dct4x4.offset(5 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            ((b6 - b7) * dmf + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        (*dct4x4.offset(6 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            ((b4 + b5) * dmf + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        (*dct4x4.offset(7 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            ((b6 + b7) * dmf + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
    }
}
unsafe extern "C" fn idct_dequant_2x4_dconly(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
) {
    unsafe {
        let mut a0: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a1: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a2: ::core::ffi::c_int = *dct.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a3: ::core::ffi::c_int = *dct.offset(6 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a4: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a5: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a6: ::core::ffi::c_int = *dct.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a7: ::core::ffi::c_int = *dct.offset(6 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut b0: ::core::ffi::c_int = a0 + a1;
        let mut b1: ::core::ffi::c_int = a2 + a3;
        let mut b2: ::core::ffi::c_int = a4 + a5;
        let mut b3: ::core::ffi::c_int = a6 + a7;
        let mut b4: ::core::ffi::c_int = a0 - a1;
        let mut b5: ::core::ffi::c_int = a2 - a3;
        let mut b6: ::core::ffi::c_int = a4 - a5;
        let mut b7: ::core::ffi::c_int = a6 - a7;
        let mut dmf: ::core::ffi::c_int = (*dequant_mf
            .offset((i_qp % 6 as ::core::ffi::c_int) as isize))[0 as ::core::ffi::c_int as usize]
            << i_qp / 6 as ::core::ffi::c_int;
        *dct.offset(0 as ::core::ffi::c_int as isize) = ((b0 + b1) * dmf + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *dct.offset(1 as ::core::ffi::c_int as isize) = ((b2 + b3) * dmf + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *dct.offset(2 as ::core::ffi::c_int as isize) = ((b0 - b1) * dmf + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *dct.offset(3 as ::core::ffi::c_int as isize) = ((b2 - b3) * dmf + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *dct.offset(4 as ::core::ffi::c_int as isize) = ((b4 - b5) * dmf + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *dct.offset(5 as ::core::ffi::c_int as isize) = ((b6 - b7) * dmf + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *dct.offset(6 as ::core::ffi::c_int as isize) = ((b4 + b5) * dmf + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *dct.offset(7 as ::core::ffi::c_int as isize) = ((b6 + b7) * dmf + 32 as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
    }
}
#[inline(always)]
unsafe extern "C" fn optimize_chroma_idct_dequant_2x4(
    mut out: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dmf: ::core::ffi::c_int,
) {
    unsafe {
        let mut a0: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a1: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a2: ::core::ffi::c_int = *dct.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a3: ::core::ffi::c_int = *dct.offset(6 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a4: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a5: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a6: ::core::ffi::c_int = *dct.offset(4 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut a7: ::core::ffi::c_int = *dct.offset(6 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut b0: ::core::ffi::c_int = a0 + a1;
        let mut b1: ::core::ffi::c_int = a2 + a3;
        let mut b2: ::core::ffi::c_int = a4 + a5;
        let mut b3: ::core::ffi::c_int = a6 + a7;
        let mut b4: ::core::ffi::c_int = a0 - a1;
        let mut b5: ::core::ffi::c_int = a2 - a3;
        let mut b6: ::core::ffi::c_int = a4 - a5;
        let mut b7: ::core::ffi::c_int = a6 - a7;
        *out.offset(0 as ::core::ffi::c_int as isize) =
            ((b0 + b1) * dmf + 2080 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(1 as ::core::ffi::c_int as isize) =
            ((b2 + b3) * dmf + 2080 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(2 as ::core::ffi::c_int as isize) =
            ((b0 - b1) * dmf + 2080 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(3 as ::core::ffi::c_int as isize) =
            ((b2 - b3) * dmf + 2080 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(4 as ::core::ffi::c_int as isize) =
            ((b4 - b5) * dmf + 2080 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(5 as ::core::ffi::c_int as isize) =
            ((b6 - b7) * dmf + 2080 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(6 as ::core::ffi::c_int as isize) =
            ((b4 + b5) * dmf + 2080 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(7 as ::core::ffi::c_int as isize) =
            ((b6 + b7) * dmf + 2080 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
    }
}
#[inline(always)]
unsafe extern "C" fn optimize_chroma_idct_dequant_2x2(
    mut out: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dmf: ::core::ffi::c_int,
) {
    unsafe {
        let mut d0: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut d1: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut d2: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut d3: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *out.offset(0 as ::core::ffi::c_int as isize) =
            (((d0 + d1) * dmf >> 5 as ::core::ffi::c_int) + 32 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(1 as ::core::ffi::c_int as isize) =
            (((d0 - d1) * dmf >> 5 as ::core::ffi::c_int) + 32 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(2 as ::core::ffi::c_int as isize) =
            (((d2 + d3) * dmf >> 5 as ::core::ffi::c_int) + 32 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
        *out.offset(3 as ::core::ffi::c_int as isize) =
            (((d2 - d3) * dmf >> 5 as ::core::ffi::c_int) + 32 as ::core::ffi::c_int)
                as crate::src::common::common::dctcoef;
    }
}
#[inline(always)]
unsafe extern "C" fn optimize_chroma_round(
    mut ref_0: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: ::core::ffi::c_int,
    mut chroma422: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut out: [crate::src::common::common::dctcoef; 8] = [0; 8];
        if chroma422 != 0 {
            optimize_chroma_idct_dequant_2x4(
                &raw mut out as *mut crate::src::common::common::dctcoef,
                dct as *mut crate::src::common::common::dctcoef,
                dequant_mf,
            );
        } else {
            optimize_chroma_idct_dequant_2x2(
                &raw mut out as *mut crate::src::common::common::dctcoef,
                dct as *mut crate::src::common::common::dctcoef,
                dequant_mf,
            );
        }
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i
            < (if chroma422 != 0 {
                8 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            })
        {
            sum |= *ref_0.offset(i as isize) as ::core::ffi::c_int
                ^ out[i as usize] as ::core::ffi::c_int;
            i += 1;
        }
        return sum >> 6 as ::core::ffi::c_int;
    }
}
#[inline(always)]
unsafe extern "C" fn optimize_chroma_dc_internal(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: ::core::ffi::c_int,
    mut chroma422: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dct_orig: [crate::src::common::common::dctcoef; 8] = [0; 8];
        if chroma422 != 0 {
            optimize_chroma_idct_dequant_2x4(
                &raw mut dct_orig as *mut crate::src::common::common::dctcoef,
                dct as *mut crate::src::common::common::dctcoef,
                dequant_mf,
            );
        } else {
            optimize_chroma_idct_dequant_2x2(
                &raw mut dct_orig as *mut crate::src::common::common::dctcoef,
                dct as *mut crate::src::common::common::dctcoef,
                dequant_mf,
            );
        }
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i
            < (if chroma422 != 0 {
                8 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            })
        {
            sum |= dct_orig[i as usize] as ::core::ffi::c_int;
            i += 1;
        }
        if sum >> 6 as ::core::ffi::c_int == 0 {
            return 0 as ::core::ffi::c_int;
        }
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut coeff: ::core::ffi::c_int = if chroma422 != 0 {
            7 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        };
        while coeff >= 0 as ::core::ffi::c_int {
            let mut level: ::core::ffi::c_int = *dct.offset(coeff as isize) as ::core::ffi::c_int;
            let mut sign: ::core::ffi::c_int =
                level >> 31 as ::core::ffi::c_int | 1 as ::core::ffi::c_int;
            while level != 0 {
                *dct.offset(coeff as isize) = (level - sign) as crate::src::common::common::dctcoef;
                if optimize_chroma_round(
                    &raw mut dct_orig as *mut crate::src::common::common::dctcoef,
                    dct,
                    dequant_mf,
                    chroma422,
                ) != 0
                {
                    nz = 1 as ::core::ffi::c_int;
                    *dct.offset(coeff as isize) = level as crate::src::common::common::dctcoef;
                    break;
                } else {
                    level -= sign;
                }
            }
            coeff -= 1;
        }
        return nz;
    }
}
unsafe extern "C" fn optimize_chroma_2x2_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return optimize_chroma_dc_internal(
            dct as *mut crate::src::common::common::dctcoef,
            dequant_mf,
            0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn optimize_chroma_2x4_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dequant_mf: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return optimize_chroma_dc_internal(
            dct as *mut crate::src::common::common::dctcoef,
            dequant_mf,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn denoise_dct(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut sum: *mut crate::stdlib::uint32_t,
    mut offset: *mut crate::src::common::common::udctcoef,
    mut size: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < size {
            let mut level: ::core::ffi::c_int = *dct.offset(i as isize) as ::core::ffi::c_int;
            let mut sign: ::core::ffi::c_int = level >> 31 as ::core::ffi::c_int;
            level = level + sign ^ sign;
            let ref mut c2rust_fresh5 = *sum.offset(i as isize);
            *c2rust_fresh5 = (*c2rust_fresh5).wrapping_add(level as crate::stdlib::uint32_t);
            level -= *offset.offset(i as isize) as ::core::ffi::c_int;
            *dct.offset(i as isize) = (if level < 0 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                (level ^ sign) - sign
            }) as crate::src::common::common::dctcoef;
            i += 1;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn decimate_score_internal(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut i_max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ds_table: *const crate::stdlib::uint8_t = if i_max == 64 as ::core::ffi::c_int {
            &raw const crate::src::common::tables::x264_decimate_table8
                as *const crate::stdlib::uint8_t
        } else {
            &raw const crate::src::common::tables::x264_decimate_table4
                as *const crate::stdlib::uint8_t
        };
        let mut i_score: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut idx: ::core::ffi::c_int = i_max - 1 as ::core::ffi::c_int;
        while idx >= 0 as ::core::ffi::c_int
            && *dct.offset(idx as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            idx -= 1;
        }
        while idx >= 0 as ::core::ffi::c_int {
            let c2rust_fresh4 = idx;
            idx = idx - 1;
            if (*dct.offset(c2rust_fresh4 as isize) as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
                > 2 as ::core::ffi::c_uint
            {
                return 9 as ::core::ffi::c_int;
            }
            let mut i_run: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while idx >= 0 as ::core::ffi::c_int
                && *dct.offset(idx as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                idx -= 1;
                i_run += 1;
            }
            i_score += *ds_table.offset(i_run as isize) as ::core::ffi::c_int;
        }
        return i_score;
    }
}
unsafe extern "C" fn decimate_score15(
    mut dct: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        return decimate_score_internal(
            dct.offset(1 as ::core::ffi::c_int as isize),
            15 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn decimate_score16(
    mut dct: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        return decimate_score_internal(dct, 16 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn decimate_score64(
    mut dct: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        return decimate_score_internal(dct, 64 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn coeff_last4(
    mut l: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_last: ::core::ffi::c_int = 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i_last >= 0 as ::core::ffi::c_int
            && *l.offset(i_last as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            i_last -= 1;
        }
        return i_last;
    }
}
unsafe extern "C" fn coeff_last8(
    mut l: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_last: ::core::ffi::c_int = 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i_last >= 0 as ::core::ffi::c_int
            && *l.offset(i_last as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            i_last -= 1;
        }
        return i_last;
    }
}
unsafe extern "C" fn coeff_last15(
    mut l: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_last: ::core::ffi::c_int = 15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i_last >= 0 as ::core::ffi::c_int
            && *l.offset(i_last as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            i_last -= 1;
        }
        return i_last;
    }
}
unsafe extern "C" fn coeff_last16(
    mut l: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_last: ::core::ffi::c_int = 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i_last >= 0 as ::core::ffi::c_int
            && *l.offset(i_last as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            i_last -= 1;
        }
        return i_last;
    }
}
unsafe extern "C" fn coeff_last64(
    mut l: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_last: ::core::ffi::c_int = 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i_last >= 0 as ::core::ffi::c_int
            && *l.offset(i_last as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            i_last -= 1;
        }
        return i_last;
    }
}
unsafe extern "C" fn coeff_level_run4(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut runlevel: *mut crate::src::common::bitstream::x264_run_level_t,
) -> ::core::ffi::c_int {
    unsafe {
        (*runlevel).last = coeff_last4(dct) as crate::stdlib::int32_t;
        let mut i_last: ::core::ffi::c_int = (*runlevel).last;
        let mut i_total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            let c2rust_fresh3 = i_total;
            i_total = i_total + 1;
            (*runlevel).level[c2rust_fresh3 as usize] = *dct.offset(i_last as isize);
            mask |= (1 as ::core::ffi::c_int) << i_last;
            loop {
                i_last -= 1;
                if !(i_last >= 0 as ::core::ffi::c_int
                    && *dct.offset(i_last as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if !(i_last >= 0 as ::core::ffi::c_int) {
                break;
            }
        }
        (*runlevel).mask = mask as crate::stdlib::int32_t;
        return i_total;
    }
}
unsafe extern "C" fn coeff_level_run8(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut runlevel: *mut crate::src::common::bitstream::x264_run_level_t,
) -> ::core::ffi::c_int {
    unsafe {
        (*runlevel).last = coeff_last8(dct) as crate::stdlib::int32_t;
        let mut i_last: ::core::ffi::c_int = (*runlevel).last;
        let mut i_total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            let c2rust_fresh2 = i_total;
            i_total = i_total + 1;
            (*runlevel).level[c2rust_fresh2 as usize] = *dct.offset(i_last as isize);
            mask |= (1 as ::core::ffi::c_int) << i_last;
            loop {
                i_last -= 1;
                if !(i_last >= 0 as ::core::ffi::c_int
                    && *dct.offset(i_last as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if !(i_last >= 0 as ::core::ffi::c_int) {
                break;
            }
        }
        (*runlevel).mask = mask as crate::stdlib::int32_t;
        return i_total;
    }
}
unsafe extern "C" fn coeff_level_run15(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut runlevel: *mut crate::src::common::bitstream::x264_run_level_t,
) -> ::core::ffi::c_int {
    unsafe {
        (*runlevel).last = coeff_last15(dct) as crate::stdlib::int32_t;
        let mut i_last: ::core::ffi::c_int = (*runlevel).last;
        let mut i_total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            let c2rust_fresh1 = i_total;
            i_total = i_total + 1;
            (*runlevel).level[c2rust_fresh1 as usize] = *dct.offset(i_last as isize);
            mask |= (1 as ::core::ffi::c_int) << i_last;
            loop {
                i_last -= 1;
                if !(i_last >= 0 as ::core::ffi::c_int
                    && *dct.offset(i_last as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if !(i_last >= 0 as ::core::ffi::c_int) {
                break;
            }
        }
        (*runlevel).mask = mask as crate::stdlib::int32_t;
        return i_total;
    }
}
unsafe extern "C" fn coeff_level_run16(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut runlevel: *mut crate::src::common::bitstream::x264_run_level_t,
) -> ::core::ffi::c_int {
    unsafe {
        (*runlevel).last = coeff_last16(dct) as crate::stdlib::int32_t;
        let mut i_last: ::core::ffi::c_int = (*runlevel).last;
        let mut i_total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            let c2rust_fresh0 = i_total;
            i_total = i_total + 1;
            (*runlevel).level[c2rust_fresh0 as usize] = *dct.offset(i_last as isize);
            mask |= (1 as ::core::ffi::c_int) << i_last;
            loop {
                i_last -= 1;
                if !(i_last >= 0 as ::core::ffi::c_int
                    && *dct.offset(i_last as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if !(i_last >= 0 as ::core::ffi::c_int) {
                break;
            }
        }
        (*runlevel).mask = mask as crate::stdlib::int32_t;
        return i_total;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_quant_init(
    mut _h: *mut crate::src::common::common::x264_t,
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::quant::x264_quant_function_t,
) {
    unsafe {
        (*pf).quant_8x8 = Some(
            quant_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::udctcoef,
                    *mut crate::src::common::common::udctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::udctcoef,
                    *mut crate::src::common::common::udctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).quant_4x4 = Some(
            quant_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::udctcoef,
                    *mut crate::src::common::common::udctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::udctcoef,
                    *mut crate::src::common::common::udctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).quant_4x4x4 = Some(
            quant_4x4x4
                as unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut crate::src::common::common::udctcoef,
                    *mut crate::src::common::common::udctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut crate::src::common::common::udctcoef,
                    *mut crate::src::common::common::udctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).quant_4x4_dc = Some(
            quant_4x4_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).quant_2x2_dc = Some(
            quant_2x2_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).dequant_4x4 = Some(
            dequant_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [::core::ffi::c_int; 16],
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [::core::ffi::c_int; 16],
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*pf).dequant_4x4_dc = Some(
            dequant_4x4_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [::core::ffi::c_int; 16],
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [::core::ffi::c_int; 16],
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*pf).dequant_8x8 = Some(
            dequant_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [::core::ffi::c_int; 64],
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [::core::ffi::c_int; 64],
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*pf).idct_dequant_2x4_dc = Some(
            idct_dequant_2x4_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut [::core::ffi::c_int; 16],
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut [::core::ffi::c_int; 16],
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*pf).idct_dequant_2x4_dconly = Some(
            idct_dequant_2x4_dconly
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [::core::ffi::c_int; 16],
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [::core::ffi::c_int; 16],
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*pf).optimize_chroma_2x2_dc = Some(
            optimize_chroma_2x2_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).optimize_chroma_2x4_dc = Some(
            optimize_chroma_2x4_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).denoise_dct = Some(
            denoise_dct
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::stdlib::uint32_t,
                    *mut crate::src::common::common::udctcoef,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::stdlib::uint32_t,
                    *mut crate::src::common::common::udctcoef,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*pf).decimate_score15 = Some(
            decimate_score15
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).decimate_score16 = Some(
            decimate_score16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).decimate_score64 = Some(
            decimate_score64
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_last4 = Some(
            coeff_last4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_last8 = Some(
            coeff_last8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int as usize] = Some(
            coeff_last15
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int as usize] = Some(
            coeff_last16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_LUMA_8x8 as ::core::ffi::c_int as usize] = Some(
            coeff_last64
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_level_run4 = Some(
            coeff_level_run4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::bitstream::x264_run_level_t,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::bitstream::x264_run_level_t,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_level_run8 = Some(
            coeff_level_run8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::bitstream::x264_run_level_t,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::bitstream::x264_run_level_t,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int as usize] = Some(
            coeff_level_run15
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::bitstream::x264_run_level_t,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::bitstream::x264_run_level_t,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int as usize] = Some(
            coeff_level_run16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::bitstream::x264_run_level_t,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::bitstream::x264_run_level_t,
                ) -> ::core::ffi::c_int,
            >;
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMAV_4x4 as ::core::ffi::c_int as usize] =
            (*pf).coeff_last
                [crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMAU_4x4 as ::core::ffi::c_int as usize] =
            (*pf).coeff_last
                [crate::src::common::macroblock::DCT_CHROMAV_4x4 as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMAV_DC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_last
            [crate::src::common::macroblock::DCT_CHROMAU_4x4 as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMAU_DC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_last
            [crate::src::common::macroblock::DCT_CHROMAV_DC as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_last
            [crate::src::common::macroblock::DCT_CHROMAU_DC as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMAV_AC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_last[crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMAU_AC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_last
            [crate::src::common::macroblock::DCT_CHROMAV_AC as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_last
            [crate::src::common::macroblock::DCT_CHROMAU_AC as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMAV_8x8 as ::core::ffi::c_int as usize] =
            (*pf).coeff_last
                [crate::src::common::macroblock::DCT_LUMA_8x8 as ::core::ffi::c_int as usize];
        (*pf).coeff_last
            [crate::src::common::macroblock::DCT_CHROMAU_8x8 as ::core::ffi::c_int as usize] =
            (*pf).coeff_last
                [crate::src::common::macroblock::DCT_CHROMAV_8x8 as ::core::ffi::c_int as usize];
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAV_4x4 as ::core::ffi::c_int as usize] =
            (*pf).coeff_level_run
                [crate::src::common::macroblock::DCT_LUMA_4x4 as ::core::ffi::c_int as usize];
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAU_4x4 as ::core::ffi::c_int as usize] =
            (*pf).coeff_level_run
                [crate::src::common::macroblock::DCT_CHROMAV_4x4 as ::core::ffi::c_int as usize];
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAV_DC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAU_4x4 as ::core::ffi::c_int as usize];
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAU_DC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAV_DC as ::core::ffi::c_int as usize];
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_LUMA_DC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAU_DC as ::core::ffi::c_int as usize];
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAV_AC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_level_run
            [crate::src::common::macroblock::DCT_LUMA_AC as ::core::ffi::c_int as usize];
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAU_AC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAV_AC as ::core::ffi::c_int as usize];
        (*pf).coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMA_AC as ::core::ffi::c_int as usize] = (*pf)
            .coeff_level_run
            [crate::src::common::macroblock::DCT_CHROMAU_AC as ::core::ffi::c_int as usize];
    }
}
