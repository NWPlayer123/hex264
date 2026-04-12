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
pub unsafe extern "C" fn x264_8_cabac_init(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut i = 0i32;
        let mut ctx_count = if (*h).sps.i_chroma_format_idc.is_444() {
            1024i32
        } else {
            460i32
        };
        while i < 4i32 {
            let mut qp = 0i32;
            let mut cabac_context_init = if i == 0i32 {
                &raw const crate::src::common::tables::x264_cabac_context_init_I
            } else {
                (&raw const crate::src::common::tables::x264_cabac_context_init_PB
                    as *const [[crate::stdlib::int8_t; 2]; 1024])
                    .offset((i - 1i32) as isize)
            };
            while qp <= crate::src::common::common::QP_MAX_SPEC {
                let mut j = 0i32;
                while j < ctx_count {
                    let mut state = x264_clip3(
                        (((*cabac_context_init)[j as usize][0usize] as ::core::ffi::c_int * qp)
                            >> 4i32)
                            + (*cabac_context_init)[j as usize][1usize] as ::core::ffi::c_int,
                        1i32,
                        126i32,
                    );
                    cabac_contexts[i as usize][qp as usize][j as usize] =
                        ((if state < 127i32 - state {
                            state
                        } else {
                            127i32 - state
                        }) << 1i32
                            | state >> 6i32) as crate::stdlib::uint8_t;
                    j += 1;
                }
                qp += 1;
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_cabac_context_init(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_slice_type: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
    mut i_model: ::core::ffi::c_int,
) {
    unsafe {
        crate::stdlib::memcpy(
            &raw mut (*cb).state as *mut ::core::ffi::c_void,
            &raw mut *(&raw mut *(&raw mut cabac_contexts
                as *mut [[crate::stdlib::uint8_t; 1024]; 52])
                .offset(
                    (if i_slice_type == crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int
                    {
                        0i32
                    } else {
                        i_model + 1i32
                    }) as isize,
                ) as *mut [crate::stdlib::uint8_t; 1024])
                .offset(i_qp as isize) as *const ::core::ffi::c_void,
            (if (*h).sps.i_chroma_format_idc.is_444() {
                1024i32
            } else {
                460i32
            }) as crate::__stddef_size_t_h::size_t,
        );
    }
}
pub unsafe extern "C" fn x264_8_cabac_encode_init_core(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        (*cb).i_low = 0i32;
        (*cb).i_range = 0x1fei32;
        (*cb).i_queue = -(9i32);
        (*cb).i_bytes_outstanding = 0i32;
    }
}
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
        if (*cb).i_queue >= 0i32 {
            let mut out = (*cb).i_low >> ((*cb).i_queue + 10i32);
            (*cb).i_low &= ((0x400i32) << (*cb).i_queue) - 1i32;
            (*cb).i_queue -= 8i32;
            if out & 0xffi32 == 0xffi32 {
                (*cb).i_bytes_outstanding += 1;
            } else {
                let mut carry = out >> 8i32;
                let mut bytes_outstanding = (*cb).i_bytes_outstanding;
                let ref mut c2rust_fresh0 = *(*cb).p.offset(-1isize);
                *c2rust_fresh0 =
                    (*c2rust_fresh0 as ::core::ffi::c_int + carry) as crate::stdlib::uint8_t;
                while bytes_outstanding > 0i32 {
                    let c2rust_fresh1 = (*cb).p;
                    (*cb).p = (*cb).p.offset(1);
                    *c2rust_fresh1 = (carry - 1i32) as crate::stdlib::uint8_t;
                    bytes_outstanding -= 1;
                }
                let c2rust_fresh2 = (*cb).p;
                (*cb).p = (*cb).p.offset(1);
                *c2rust_fresh2 = out as crate::stdlib::uint8_t;
                (*cb).i_bytes_outstanding = 0i32;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn cabac_encode_renorm(mut cb: *mut crate::src::common::cabac::x264_cabac_t) {
    unsafe {
        let mut shift = crate::src::common::tables::x264_cabac_renorm_shift
            [((*cb).i_range >> 3i32) as usize] as ::core::ffi::c_int;
        (*cb).i_range <<= shift;
        (*cb).i_low <<= shift;
        (*cb).i_queue += shift;
        cabac_putbyte(cb);
    }
}
pub unsafe extern "C" fn x264_8_cabac_encode_decision_c(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut i_ctx: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) {
    unsafe {
        let mut i_state = (*cb).state[i_ctx as usize] as ::core::ffi::c_int;
        let mut i_range_lps = crate::src::common::tables::x264_cabac_range_lps
            [(i_state >> 1i32) as usize][(((*cb).i_range >> 6i32) - 4i32) as usize]
            as ::core::ffi::c_int;
        (*cb).i_range -= i_range_lps;
        if b != i_state & 1i32 {
            (*cb).i_low += (*cb).i_range;
            (*cb).i_range = i_range_lps;
        }
        (*cb).state[i_ctx as usize] =
            crate::src::common::tables::x264_cabac_transition[i_state as usize][b as usize];
        cabac_encode_renorm(cb);
    }
}
pub unsafe extern "C" fn x264_8_cabac_encode_bypass_c(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut b: ::core::ffi::c_int,
) {
    unsafe {
        (*cb).i_low <<= 1i32;
        (*cb).i_low += b & (*cb).i_range;
        (*cb).i_queue += 1i32;
        cabac_putbyte(cb);
    }
}
static mut bypass_lut: [::core::ffi::c_int; 16] = [
    -(1i32),
    0x2i32,
    0x14i32,
    0x68i32,
    0x1d0i32,
    0x7a0i32,
    0x1f40i32,
    0x7e80i32,
    0x1fd00i32,
    0x7fa00i32,
    0x1ff400i32,
    0x7fe800i32,
    0x1ffd000i32,
    0x7ffa000i32,
    0x1fff4000i32,
    0x7ffe8000i32,
];
pub unsafe extern "C" fn x264_8_cabac_encode_ue_bypass(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
    mut exp_bits: ::core::ffi::c_int,
    mut val: ::core::ffi::c_int,
) {
    unsafe {
        let mut v = (val + ((1i32) << exp_bits)) as crate::stdlib::uint32_t;
        let mut k = 31i32 - v.leading_zeros() as i32;
        let mut x = ((bypass_lut[(k - exp_bits) as usize] as crate::stdlib::uint32_t) << exp_bits)
            .wrapping_add(v);
        k = 2i32 * k + 1i32 - exp_bits;
        let mut i = ((k - 1i32) & 7i32) + 1i32;
        loop {
            k -= i;
            (*cb).i_low <<= i;
            (*cb).i_low = ((*cb).i_low as crate::stdlib::uint32_t).wrapping_add(
                (x >> k & 0xffu32).wrapping_mul((*cb).i_range as crate::stdlib::uint32_t),
            ) as ::core::ffi::c_int;
            (*cb).i_queue += i;
            cabac_putbyte(cb);
            i = 8i32;
            if !(k > 0i32) {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_cabac_encode_terminal_c(
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        (*cb).i_range -= 2i32;
        cabac_encode_renorm(cb);
    }
}
pub unsafe extern "C" fn x264_8_cabac_encode_flush(
    mut h: *mut crate::src::common::common::x264_t,
    mut cb: *mut crate::src::common::cabac::x264_cabac_t,
) {
    unsafe {
        (*cb).i_low += (*cb).i_range - 2i32;
        (*cb).i_low |= 1i32;
        (*cb).i_low <<= 9i32;
        (*cb).i_queue += 9i32;
        cabac_putbyte(cb);
        cabac_putbyte(cb);
        (*cb).i_low <<= -(*cb).i_queue;
        (*cb).i_low |= (0x35a4e4f5i32 >> ((*h).i_frame & 31i32) & 1i32) << 10i32;
        (*cb).i_queue = 0i32;
        cabac_putbyte(cb);
        while (*cb).i_bytes_outstanding > 0i32 {
            let c2rust_fresh3 = (*cb).p;
            (*cb).p = (*cb).p.offset(1);
            *c2rust_fresh3 = 0xffu8;
            (*cb).i_bytes_outstanding -= 1;
        }
    }
}
