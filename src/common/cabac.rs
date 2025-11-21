use core::ffi::{c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::base_h::{x264_clip3, CHROMA_444, SLICE_TYPE_I};
use crate::cabac_h::x264_cabac_t;
use crate::common_h::{x264_t, QP_MAX_SPEC};
use crate::stdint_intn_h::int8_t;
use crate::stdint_uintn_h::{uint32_t, uint8_t};
use crate::string_h::memcpy;
use crate::tables_h::{
    x264_cabac_context_init_I, x264_cabac_context_init_PB, x264_cabac_range_lps,
    x264_cabac_renorm_shift, x264_cabac_transition,
};
#[c2rust::src_loc = "30:16"]
static mut cabac_contexts: [[[uint8_t; 1024]; 64]; 4] = [[[0; 1024]; 64]; 4];
#[no_mangle]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn x264_10_cabac_init(mut h: *mut x264_t) {
    let mut ctx_count: c_int =
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            1024 as c_int
        } else {
            460 as c_int
        };
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        let mut cabac_context_init: *const [[int8_t; 2]; 1024] = if i == 0 as c_int {
            &x264_cabac_context_init_I
        } else {
            &*x264_cabac_context_init_PB
                .as_ptr()
                .offset((i - 1 as c_int) as isize) as *const [[int8_t; 2]; 1024]
        };
        let mut qp: c_int = 0 as c_int;
        while qp <= QP_MAX_SPEC {
            let mut j: c_int = 0 as c_int;
            while j < ctx_count {
                let mut state: c_int = x264_clip3(
                    ((*cabac_context_init)[j as usize][0] as c_int * qp >> 4 as c_int)
                        + (*cabac_context_init)[j as usize][1] as c_int,
                    1 as c_int,
                    126 as c_int,
                );
                cabac_contexts[i as usize][qp as usize][j as usize] =
                    ((if state < 127 as c_int - state {
                        state
                    } else {
                        127 as c_int - state
                    }) << 1 as c_int
                        | state >> 6 as c_int) as uint8_t;
                j += 1;
            }
            qp += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn x264_10_cabac_context_init(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_slice_type: c_int,
    mut i_qp: c_int,
    mut i_model: c_int,
) {
    memcpy(
        (*cb).state.as_mut_ptr() as *mut c_void,
        (*(*cabac_contexts.as_mut_ptr().offset(
            (if i_slice_type == SLICE_TYPE_I as c_int {
                0 as c_int
            } else {
                i_model + 1 as c_int
            }) as isize,
        ))
        .as_mut_ptr()
        .offset(i_qp as isize))
        .as_mut_ptr() as *const c_void,
        (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as c_int {
            1024 as c_int
        } else {
            460 as c_int
        }) as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn x264_10_cabac_encode_init_core(mut cb: *mut x264_cabac_t) {
    (*cb).i_low = 0 as c_int;
    (*cb).i_range = 0x1fe as c_int;
    (*cb).i_queue = -(9 as c_int);
    (*cb).i_bytes_outstanding = 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn x264_10_cabac_encode_init(
    mut cb: *mut x264_cabac_t,
    mut p_data: *mut uint8_t,
    mut p_end: *mut uint8_t,
) {
    x264_10_cabac_encode_init_core(cb);
    (*cb).p_start = p_data;
    (*cb).p = p_data;
    (*cb).p_end = p_end;
}
#[inline]
#[c2rust::src_loc = "69:1"]
unsafe extern "C" fn cabac_putbyte(mut cb: *mut x264_cabac_t) {
    if (*cb).i_queue >= 0 as c_int {
        let mut out: c_int = (*cb).i_low >> (*cb).i_queue + 10 as c_int;
        (*cb).i_low &= ((0x400 as c_int) << (*cb).i_queue) - 1 as c_int;
        (*cb).i_queue -= 8 as c_int;
        if out & 0xff as c_int == 0xff as c_int {
            (*cb).i_bytes_outstanding += 1;
        } else {
            let mut carry: c_int = out >> 8 as c_int;
            let mut bytes_outstanding: c_int = (*cb).i_bytes_outstanding;
            let ref mut fresh0 = *(*cb).p.offset(-1 as isize);
            *fresh0 = (*fresh0 as c_int + carry) as uint8_t;
            while bytes_outstanding > 0 as c_int {
                let fresh1 = (*cb).p;
                (*cb).p = (*cb).p.offset(1);
                *fresh1 = (carry - 1 as c_int) as uint8_t;
                bytes_outstanding -= 1;
            }
            let fresh2 = (*cb).p;
            (*cb).p = (*cb).p.offset(1);
            *fresh2 = out as uint8_t;
            (*cb).i_bytes_outstanding = 0 as c_int;
        }
    }
}
#[inline]
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn cabac_encode_renorm(mut cb: *mut x264_cabac_t) {
    let mut shift: c_int = x264_cabac_renorm_shift[((*cb).i_range >> 3 as c_int) as usize] as c_int;
    (*cb).i_range <<= shift;
    (*cb).i_low <<= shift;
    (*cb).i_queue += shift;
    cabac_putbyte(cb);
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn x264_10_cabac_encode_decision_c(
    mut cb: *mut x264_cabac_t,
    mut i_ctx: c_int,
    mut b: c_int,
) {
    let mut i_state: c_int = (*cb).state[i_ctx as usize] as c_int;
    let mut i_range_lps: c_int = x264_cabac_range_lps[(i_state >> 1 as c_int) as usize]
        [(((*cb).i_range >> 6 as c_int) - 4 as c_int) as usize]
        as c_int;
    (*cb).i_range -= i_range_lps;
    if b != i_state & 1 as c_int {
        (*cb).i_low += (*cb).i_range;
        (*cb).i_range = i_range_lps;
    }
    (*cb).state[i_ctx as usize] = x264_cabac_transition[i_state as usize][b as usize];
    cabac_encode_renorm(cb);
}
#[no_mangle]
#[c2rust::src_loc = "128:1"]
unsafe extern "C" fn x264_10_cabac_encode_bypass_c(mut cb: *mut x264_cabac_t, mut b: c_int) {
    (*cb).i_low <<= 1 as c_int;
    (*cb).i_low += b & (*cb).i_range;
    (*cb).i_queue += 1 as c_int;
    cabac_putbyte(cb);
}
#[c2rust::src_loc = "136:18"]
static mut bypass_lut: [c_int; 16] = [
    -1,
    0x2 as c_int,
    0x14 as c_int,
    0x68 as c_int,
    0x1d0 as c_int,
    0x7a0 as c_int,
    0x1f40 as c_int,
    0x7e80 as c_int,
    0x1fd00 as c_int,
    0x7fa00 as c_int,
    0x1ff400 as c_int,
    0x7fe800 as c_int,
    0x1ffd000 as c_int,
    0x7ffa000 as c_int,
    0x1fff4000 as c_int,
    0x7ffe8000 as c_int,
];
#[no_mangle]
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn x264_10_cabac_encode_ue_bypass(
    mut cb: *mut x264_cabac_t,
    mut exp_bits: c_int,
    mut val: c_int,
) {
    let mut v: uint32_t = (val + ((1 as c_int) << exp_bits)) as uint32_t;
    let mut k: c_int = 31 as c_int - v.leading_zeros() as i32;
    let mut x: uint32_t =
        ((bypass_lut[(k - exp_bits) as usize] as uint32_t) << exp_bits).wrapping_add(v);
    k = 2 as c_int * k + 1 as c_int - exp_bits;
    let mut i: c_int = (k - 1 as c_int & 7 as c_int) + 1 as c_int;
    loop {
        k -= i;
        (*cb).i_low <<= i;
        (*cb).i_low = ((*cb).i_low as uint32_t)
            .wrapping_add((x >> k & 0xff as uint32_t).wrapping_mul((*cb).i_range as uint32_t))
            as c_int as c_int;
        (*cb).i_queue += i;
        cabac_putbyte(cb);
        i = 8 as c_int;
        if !(k > 0 as c_int) {
            break;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn x264_10_cabac_encode_terminal_c(mut cb: *mut x264_cabac_t) {
    (*cb).i_range -= 2 as c_int;
    cabac_encode_renorm(cb);
}
#[no_mangle]
#[c2rust::src_loc = "165:1"]
unsafe extern "C" fn x264_10_cabac_encode_flush(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    (*cb).i_low += (*cb).i_range - 2 as c_int;
    (*cb).i_low |= 1 as c_int;
    (*cb).i_low <<= 9 as c_int;
    (*cb).i_queue += 9 as c_int;
    cabac_putbyte(cb);
    cabac_putbyte(cb);
    (*cb).i_low <<= -(*cb).i_queue;
    (*cb).i_low |=
        (0x35a4e4f5 as c_int >> ((*h).i_frame & 31 as c_int) & 1 as c_int) << 10 as c_int;
    (*cb).i_queue = 0 as c_int;
    cabac_putbyte(cb);
    while (*cb).i_bytes_outstanding > 0 as c_int {
        let fresh3 = (*cb).p;
        (*cb).p = (*cb).p.offset(1);
        *fresh3 = 0xff as uint8_t;
        (*cb).i_bytes_outstanding -= 1;
    }
}
