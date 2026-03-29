// =============== BEGIN cabac_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_cabac_t {
    pub i_low: ::core::ffi::c_int,
    pub i_range: ::core::ffi::c_int,
    pub i_queue: ::core::ffi::c_int,
    pub i_bytes_outstanding: ::core::ffi::c_int,
    pub p_start: *mut crate::stdlib::uint8_t,
    pub p: *mut crate::stdlib::uint8_t,
    pub p_end: *mut crate::stdlib::uint8_t,
    pub f8_bits_encoded: ::core::ffi::c_int,
    pub state: [crate::stdlib::uint8_t; 1024],
    pub padding: [crate::stdlib::uint8_t; 12],
}
pub mod base_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return if v < i_min {
            i_min
        } else if v > i_max {
            i_max
        } else {
            v
        };
    }
}
use crate::src::common::cabac::base_h::x264_clip3;
static mut cabac_contexts: [[[crate::stdlib::uint8_t; 1024]; 52]; 4] = [[[0; 1024]; 52]; 4];
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_init(mut _h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut ctx_count: ::core::ffi::c_int = if crate::src::common::base::CHROMA_444
            as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            1024 as ::core::ffi::c_int
        } else {
            460 as ::core::ffi::c_int
        };
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut cabac_context_init: *const [[crate::stdlib::int8_t; 2]; 1024] =
                if i == 0 as ::core::ffi::c_int {
                    &raw const crate::src::common::tables::x264_cabac_context_init_I
                } else {
                    (&raw const crate::src::common::tables::x264_cabac_context_init_PB
                        as *const [[crate::stdlib::int8_t; 2]; 1024])
                        .offset((i - 1 as ::core::ffi::c_int) as isize)
                        as *const [[crate::stdlib::int8_t; 2]; 1024]
                };
            let mut qp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while qp <= crate::src::common::common::QP_MAX_SPEC {
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < ctx_count {
                    let mut state: ::core::ffi::c_int = x264_clip3(
                        ((*cabac_context_init)[j as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * qp
                            >> 4 as ::core::ffi::c_int)
                            + (*cabac_context_init)[j as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        126 as ::core::ffi::c_int,
                    );
                    cabac_contexts[i as usize][qp as usize][j as usize] =
                        ((if state < 127 as ::core::ffi::c_int - state {
                            state
                        } else {
                            127 as ::core::ffi::c_int - state
                        }) << 1 as ::core::ffi::c_int
                            | state >> 6 as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                    j += 1;
                }
                qp += 1;
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_context_init(
    mut _h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_slice_type: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
    mut i_model: ::core::ffi::c_int,
) {
    unsafe {
        crate::stdlib::memcpy(
            &raw mut (*cb).state as *mut crate::stdlib::uint8_t as *mut ::core::ffi::c_void,
            &raw mut *(&raw mut *(&raw mut cabac_contexts
                as *mut [[crate::stdlib::uint8_t; 1024]; 52])
                .offset(
                    (if i_slice_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        i_model + 1 as ::core::ffi::c_int
                    }) as isize,
                ) as *mut [crate::stdlib::uint8_t; 1024])
                .offset(i_qp as isize) as *mut crate::stdlib::uint8_t
                as *const ::core::ffi::c_void,
            (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                1024 as ::core::ffi::c_int
            } else {
                460 as ::core::ffi::c_int
            }) as crate::__stddef_size_t_h::size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_init_core(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        (*cb).i_low = 0 as ::core::ffi::c_int;
        (*cb).i_range = 0x1fe as ::core::ffi::c_int;
        (*cb).i_queue = -(9 as ::core::ffi::c_int);
        (*cb).i_bytes_outstanding = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_init(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut p_data: *mut crate::stdlib::uint8_t,
    mut p_end: *mut crate::stdlib::uint8_t,
) {
    unsafe {
        x264_8_cabac_encode_init_core(cb);
        (*cb).p_start = p_data;
        (*cb).p = p_data;
        (*cb).p_end = p_end;
    }
}
#[inline]
unsafe extern "C" fn cabac_putbyte(mut cb: *mut crate::src::common::cabac::x264_cabac_t) {
    unsafe {
        if (*cb).i_queue >= 0 as ::core::ffi::c_int {
            let mut out: ::core::ffi::c_int =
                (*cb).i_low >> (*cb).i_queue + 10 as ::core::ffi::c_int;
            (*cb).i_low &=
                ((0x400 as ::core::ffi::c_int) << (*cb).i_queue) - 1 as ::core::ffi::c_int;
            (*cb).i_queue -= 8 as ::core::ffi::c_int;
            if out & 0xff as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
                (*cb).i_bytes_outstanding += 1;
            } else {
                let mut carry: ::core::ffi::c_int = out >> 8 as ::core::ffi::c_int;
                let mut bytes_outstanding: ::core::ffi::c_int = (*cb).i_bytes_outstanding;
                let ref mut c2rust_fresh0 = *(*cb).p.offset(-(1 as ::core::ffi::c_int) as isize);
                *c2rust_fresh0 =
                    (*c2rust_fresh0 as ::core::ffi::c_int + carry) as crate::stdlib::uint8_t;
                while bytes_outstanding > 0 as ::core::ffi::c_int {
                    let c2rust_fresh1 = (*cb).p;
                    (*cb).p = (*cb).p.offset(1);
                    *c2rust_fresh1 = (carry - 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
                    bytes_outstanding -= 1;
                }
                let c2rust_fresh2 = (*cb).p;
                (*cb).p = (*cb).p.offset(1);
                *c2rust_fresh2 = out as crate::stdlib::uint8_t;
                (*cb).i_bytes_outstanding = 0 as ::core::ffi::c_int;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn cabac_encode_renorm(mut cb: *mut crate::src::common::cabac::x264_cabac_t) {
    unsafe {
        let mut shift: ::core::ffi::c_int = crate::src::common::tables::x264_cabac_renorm_shift
            [((*cb).i_range >> 3 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        (*cb).i_range <<= shift;
        (*cb).i_low <<= shift;
        (*cb).i_queue += shift;
        cabac_putbyte(cb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_decision_c(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_ctx: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) {
    unsafe {
        let mut i_state: ::core::ffi::c_int = (*cb).state[i_ctx as usize] as ::core::ffi::c_int;
        let mut i_range_lps: ::core::ffi::c_int = crate::src::common::tables::x264_cabac_range_lps
            [(i_state >> 1 as ::core::ffi::c_int) as usize]
            [(((*cb).i_range >> 6 as ::core::ffi::c_int) - 4 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        (*cb).i_range -= i_range_lps;
        if b != i_state & 1 as ::core::ffi::c_int {
            (*cb).i_low += (*cb).i_range;
            (*cb).i_range = i_range_lps;
        }
        (*cb).state[i_ctx as usize] =
            crate::src::common::tables::x264_cabac_transition[i_state as usize][b as usize];
        cabac_encode_renorm(cb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_bypass_c(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut b: ::core::ffi::c_int,
) {
    unsafe {
        (*cb).i_low <<= 1 as ::core::ffi::c_int;
        (*cb).i_low += b & (*cb).i_range;
        (*cb).i_queue += 1 as ::core::ffi::c_int;
        cabac_putbyte(cb);
    }
}
static mut bypass_lut: [::core::ffi::c_int; 16] = [
    -(1 as ::core::ffi::c_int),
    0x2 as ::core::ffi::c_int,
    0x14 as ::core::ffi::c_int,
    0x68 as ::core::ffi::c_int,
    0x1d0 as ::core::ffi::c_int,
    0x7a0 as ::core::ffi::c_int,
    0x1f40 as ::core::ffi::c_int,
    0x7e80 as ::core::ffi::c_int,
    0x1fd00 as ::core::ffi::c_int,
    0x7fa00 as ::core::ffi::c_int,
    0x1ff400 as ::core::ffi::c_int,
    0x7fe800 as ::core::ffi::c_int,
    0x1ffd000 as ::core::ffi::c_int,
    0x7ffa000 as ::core::ffi::c_int,
    0x1fff4000 as ::core::ffi::c_int,
    0x7ffe8000 as ::core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_ue_bypass(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut exp_bits: ::core::ffi::c_int,
    mut val: ::core::ffi::c_int,
) {
    unsafe {
        let mut v: crate::stdlib::uint32_t =
            (val + ((1 as ::core::ffi::c_int) << exp_bits)) as crate::stdlib::uint32_t;
        let mut k: ::core::ffi::c_int = 31 as ::core::ffi::c_int - v.leading_zeros() as i32;
        let mut x: crate::stdlib::uint32_t =
            ((bypass_lut[(k - exp_bits) as usize] as crate::stdlib::uint32_t) << exp_bits)
                .wrapping_add(v);
        k = 2 as ::core::ffi::c_int * k + 1 as ::core::ffi::c_int - exp_bits;
        let mut i: ::core::ffi::c_int =
            (k - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int;
        loop {
            k -= i;
            (*cb).i_low <<= i;
            (*cb).i_low = ((*cb).i_low as crate::stdlib::uint32_t).wrapping_add(
                (x >> k & 0xff as crate::stdlib::uint32_t)
                    .wrapping_mul((*cb).i_range as crate::stdlib::uint32_t),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
            (*cb).i_queue += i;
            cabac_putbyte(cb);
            i = 8 as ::core::ffi::c_int;
            if !(k > 0 as ::core::ffi::c_int) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_terminal_c(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        (*cb).i_range -= 2 as ::core::ffi::c_int;
        cabac_encode_renorm(cb);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_flush(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        (*cb).i_low += (*cb).i_range - 2 as ::core::ffi::c_int;
        (*cb).i_low |= 1 as ::core::ffi::c_int;
        (*cb).i_low <<= 9 as ::core::ffi::c_int;
        (*cb).i_queue += 9 as ::core::ffi::c_int;
        cabac_putbyte(cb);
        cabac_putbyte(cb);
        (*cb).i_low <<= -(*cb).i_queue;
        (*cb).i_low |= (0x35a4e4f5 as ::core::ffi::c_int
            >> ((*h).i_frame & 31 as ::core::ffi::c_int)
            & 1 as ::core::ffi::c_int)
            << 10 as ::core::ffi::c_int;
        (*cb).i_queue = 0 as ::core::ffi::c_int;
        cabac_putbyte(cb);
        while (*cb).i_bytes_outstanding > 0 as ::core::ffi::c_int {
            let c2rust_fresh3 = (*cb).p;
            (*cb).p = (*cb).p.offset(1);
            *c2rust_fresh3 = 0xff as crate::stdlib::uint8_t;
            (*cb).i_bytes_outstanding -= 1;
        }
    }
}
