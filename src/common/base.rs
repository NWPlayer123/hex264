use ::core::ffi::{c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use ::core::mem::size_of;
use std::str::FromStr;

use crate::__stddef_null_h::NULL;
use crate::__stddef_size_t_h::size_t;
use crate::base_h::{
    x264_clip3, PROFILE_BASELINE, PROFILE_HIGH, PROFILE_HIGH10, PROFILE_HIGH422,
    PROFILE_HIGH444_PREDICTIVE, PROFILE_MAIN,
};
use crate::cpu_h::{x264_cpu_detect, x264_cpu_names};
use crate::ctype_h::{_ISdigit, __ctype_b_loc};
use crate::limits_h::INT_MAX;
use crate::malloc_h::{free, malloc, memalign, realloc};
use crate::mman_h::madvise;
use crate::mman_linux_h::MADV_HUGEPAGE;
use crate::osdep_h::{NATIVE_ALIGN, WORD_SIZE};
use crate::stdint_h::{INT32_MAX, SIZE_MAX, UINT32_MAX};
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{
    fclose, fopen, fprintf, fread, fseeko, ftello, sprintf, sscanf, stderr, vfprintf, SEEK_END,
    SEEK_SET,
};
use crate::stdlib_h::{strtod, strtol};
use crate::string_h::{
    memset, strchr, strcmp, strcspn, strdup, strlen, strncmp, strspn, strstr, strtok_r,
};
use crate::strings_h::{strcasecmp, strncasecmp};
use crate::types_h::__off64_t;
use crate::x264_config_h::X264_CHROMA_FORMAT;
use crate::x264_h::{
    x264_avcintra_flavor_names, x264_colmatrix_names, x264_colorprim_names, x264_direct_pred_names,
    x264_fullrange_names, x264_nal_hrd_names, x264_overscan_names, x264_param_t, x264_picture_t,
    x264_preset_names, x264_transfer_names, x264_vidformat_names, BPyramid, ContentLightLevel,
    CropRectangle, FramePacking, MasteringDisplay, MotionEstimation, X264_ANALYSE_BSUB16x16,
    X264_ANALYSE_I4x4, X264_ANALYSE_I8x8, X264_ANALYSE_PSUB16x16, X264_ANALYSE_PSUB8x8,
    PIC_STRUCT_AUTO, X264_AQ_AUTOVARIANCE, X264_AQ_NONE, X264_AQ_VARIANCE,
    X264_AVCINTRA_FLAVOR_PANASONIC, X264_B_ADAPT_FAST, X264_B_ADAPT_NONE, X264_B_ADAPT_TRELLIS,
    X264_CPU_SSE2_IS_FAST, X264_CPU_SSE2_IS_SLOW, X264_CPU_SSSE3, X264_CQM_CUSTOM, X264_CQM_FLAT,
    X264_CQM_JVT, X264_CSP_HIGH_DEPTH, X264_CSP_I400, X264_CSP_I420, X264_CSP_I422, X264_CSP_I444,
    X264_CSP_MASK, X264_CSP_MAX, X264_CSP_NONE, X264_CSP_V210, X264_DIRECT_PRED_AUTO,
    X264_DIRECT_PRED_SPATIAL, X264_KEYINT_MAX_INFINITE, X264_KEYINT_MIN_AUTO, X264_LOG_DEBUG,
    X264_LOG_ERROR, X264_LOG_INFO, X264_LOG_WARNING, X264_NAL_HRD_NONE, X264_PARAM_ALLOC_FAILED,
    X264_PARAM_BAD_NAME, X264_PARAM_BAD_VALUE, X264_QP_AUTO, X264_RC_ABR, X264_RC_CQP, X264_RC_CRF,
    X264_SYNC_LOOKAHEAD_AUTO, X264_THREADS_AUTO, X264_TYPE_AUTO, X264_WEIGHTP_NONE,
    X264_WEIGHTP_SIMPLE, X264_WEIGHTP_SMART,
};
use crate::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "206:9"]
struct strdup_buffer {
    size: c_int,
    count: c_int,
    ptr: [*mut c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "279:13"]
struct x264_csp_tab_t {
    planes: c_int,
    width_fix8: [c_int; 3],
    height_fix8: [c_int; 3],
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn x264_reduce_fraction(mut n: *mut uint32_t, mut d: *mut uint32_t) {
    let mut a: uint32_t = *n;
    let mut b: uint32_t = *d;
    let mut c: uint32_t = 0;
    if a == 0 || b == 0 {
        return;
    }
    c = a.wrapping_rem(b);
    while c != 0 {
        a = b;
        b = c;
        c = a.wrapping_rem(b);
    }
    *n = (*n).wrapping_div(b);
    *d = (*d).wrapping_div(b);
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn x264_reduce_fraction64(mut n: *mut uint64_t, mut d: *mut uint64_t) {
    let mut a: uint64_t = *n;
    let mut b: uint64_t = *d;
    let mut c: uint64_t = 0;
    if a == 0 || b == 0 {
        return;
    }
    c = a.wrapping_rem(b);
    while c != 0 {
        a = b;
        b = c;
        c = a.wrapping_rem(b);
    }
    *n = (*n).wrapping_div(b);
    *d = (*d).wrapping_div(b);
}
#[no_mangle]
#[c2rust::src_loc = "68:1"]
unsafe extern "C" fn x264_log_default(
    mut _p_unused: *mut c_void,
    mut i_level: c_int,
    mut psz_fmt: *const c_char,
    mut arg: ::core::ffi::VaList,
) {
    let mut psz_prefix: *mut c_char = 0 as *mut c_char;
    match i_level {
        X264_LOG_ERROR => {
            psz_prefix = b"error\0" as *const u8 as *const c_char as *mut c_char;
        }
        X264_LOG_WARNING => {
            psz_prefix = b"warning\0" as *const u8 as *const c_char as *mut c_char;
        }
        X264_LOG_INFO => {
            psz_prefix = b"info\0" as *const u8 as *const c_char as *mut c_char;
        }
        X264_LOG_DEBUG => {
            psz_prefix = b"debug\0" as *const u8 as *const c_char as *mut c_char;
        }
        _ => {
            psz_prefix = b"unknown\0" as *const u8 as *const c_char as *mut c_char;
        }
    }
    fprintf(
        stderr,
        b"x264 [%s]: \0" as *const u8 as *const c_char,
        psz_prefix,
    );
    vfprintf(stderr, psz_fmt, arg.as_va_list() as ::core::ffi::VaList);
}
#[no_mangle]
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn x264_log_internal(
    mut i_level: c_int,
    mut psz_fmt: *const c_char,
    mut args: ...
) {
    let mut arg: ::core::ffi::VaListImpl;
    arg = args.clone();
    x264_log_default(NULL, i_level, psz_fmt, arg.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn x264_malloc(mut i_size: int64_t) -> *mut c_void {
    if i_size < 0 as int64_t
        || i_size as uint64_t > (SIZE_MAX as uint64_t).wrapping_sub(HUGE_PAGE_SIZE as uint64_t)
    {
        x264_log_internal(
            X264_LOG_ERROR,
            b"invalid size of malloc: %ld\n\0" as *const u8 as *const c_char,
            i_size,
        );
        return NULL;
    }
    let mut align_buf: *mut uint8_t = 0 as *mut uint8_t;
    if i_size >= (HUGE_PAGE_SIZE * 7 as c_int / 8 as c_int) as int64_t {
        align_buf = memalign(HUGE_PAGE_SIZE as size_t, i_size as size_t) as *mut uint8_t;
        if !align_buf.is_null() {
            let mut madv_size: size_t = (i_size + HUGE_PAGE_SIZE as int64_t
                - (HUGE_PAGE_SIZE * 7 as c_int / 8 as c_int) as int64_t
                & !(HUGE_PAGE_SIZE - 1 as c_int) as int64_t)
                as size_t;
            madvise(align_buf as *mut c_void, madv_size, MADV_HUGEPAGE);
        }
    } else {
        align_buf = memalign(NATIVE_ALIGN as size_t, i_size as size_t) as *mut uint8_t;
    }
    if align_buf.is_null() {
        x264_log_internal(
            X264_LOG_ERROR,
            b"malloc of size %ld failed\n\0" as *const u8 as *const c_char,
            i_size,
        );
    }
    return align_buf as *mut c_void;
}
#[c2rust::src_loc = "106:9"]
const HUGE_PAGE_SIZE: c_int = 2 as c_int * 1024 as c_int * 1024 as c_int;
#[no_mangle]
#[c2rust::src_loc = "149:1"]
unsafe extern "C" fn x264_free(mut p: *mut c_void) {
    if !p.is_null() {
        free(p);
    }
}
#[no_mangle]
#[c2rust::src_loc = "164:1"]
unsafe extern "C" fn x264_slurp_file(mut filename: *const c_char) -> *mut c_char {
    let mut b_error: c_int = 0 as c_int;
    let mut i_size: int64_t = 0;
    let mut buf: *mut c_char = 0 as *mut c_char;
    let mut fh: *mut FILE = fopen(filename, b"rb\0" as *const u8 as *const c_char) as *mut FILE;
    if fh.is_null() {
        return 0 as *mut c_char;
    }
    b_error |= (fseeko(fh, 0 as __off64_t, SEEK_END) < 0 as c_int) as c_int;
    i_size = ftello(fh) as int64_t;
    b_error |= (i_size <= 0 as int64_t) as c_int;
    if WORD_SIZE == 4 as uint64_t {
        b_error |= (i_size > INT32_MAX as int64_t) as c_int;
    }
    b_error |= (fseeko(fh, 0 as __off64_t, SEEK_SET) < 0 as c_int) as c_int;
    if !(b_error != 0) {
        buf = x264_malloc(i_size + 2 as int64_t) as *mut c_char;
        if !buf.is_null() {
            b_error |= (fread(buf as *mut c_void, 1 as size_t, i_size as size_t, fh) as uint64_t
                != i_size as uint64_t) as c_int;
            fclose(fh);
            if b_error != 0 {
                x264_free(buf as *mut c_void);
                return 0 as *mut c_char;
            }
            if *buf.offset((i_size - 1 as int64_t) as isize) as c_int != '\n' as i32 {
                let fresh11 = i_size;
                i_size = i_size + 1;
                *buf.offset(fresh11 as isize) = '\n' as i32 as c_char;
            }
            *buf.offset(i_size as isize) = '\0' as i32 as c_char;
            return buf;
        }
    }
    fclose(fh);
    return 0 as *mut c_char;
}
#[c2rust::src_loc = "213:9"]
const BUFFER_DEFAULT_SIZE: c_int = 16 as c_int;
#[no_mangle]
#[c2rust::src_loc = "215:1"]
unsafe extern "C" fn x264_param_strdup(
    mut param: *mut x264_param_t,
    mut src: *const c_char,
) -> *mut c_char {
    let mut res: *mut c_char = 0 as *mut c_char;
    let mut current_block: u64;
    let mut buf: *mut strdup_buffer = (*param).opaque as *mut strdup_buffer;
    if buf.is_null() {
        buf = malloc((8 as c_ulong as c_int as size_t).wrapping_add(
            (BUFFER_DEFAULT_SIZE as size_t).wrapping_mul(size_of::<*mut c_void>() as size_t),
        )) as *mut strdup_buffer;
        if buf.is_null() {
            current_block = 14623623056652471472;
        } else {
            (*buf).size = BUFFER_DEFAULT_SIZE;
            (*buf).count = 0 as c_int;
            (*param).opaque = buf as *mut c_void;
            current_block = 11650488183268122163;
        }
    } else if (*buf).count == (*buf).size {
        if (*buf).size
            > (INT_MAX - 8 as c_ulong as c_int) / 2 as c_int / size_of::<*mut c_void>() as c_int
        {
            current_block = 14623623056652471472;
        } else {
            let mut new_size: c_int = (*buf).size * 2 as c_int;
            buf = realloc(
                buf as *mut c_void,
                (8 as c_ulong as c_int as size_t).wrapping_add(
                    (new_size as size_t).wrapping_mul(size_of::<*mut c_void>() as size_t),
                ),
            ) as *mut strdup_buffer;
            if buf.is_null() {
                current_block = 14623623056652471472;
            } else {
                (*buf).size = new_size;
                (*param).opaque = buf as *mut c_void;
                current_block = 11650488183268122163;
            }
        }
    } else {
        current_block = 11650488183268122163;
    }
    match current_block {
        11650488183268122163 => {
            res = strdup(src);
            if !res.is_null() {
                let fresh0 = (*buf).count;
                (*buf).count = (*buf).count + 1;
                let ref mut fresh1 = *(*buf).ptr.as_mut_ptr().offset(fresh0 as isize);
                *fresh1 = res as *mut c_void;
                return res;
            }
        }
        _ => {}
    }
    x264_log_internal(
        X264_LOG_ERROR,
        b"x264_param_strdup failed\n\0" as *const u8 as *const c_char,
    );
    return 0 as *mut c_char;
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
unsafe extern "C" fn x264_param_cleanup(mut param: *mut x264_param_t) {
    let mut buf: *mut strdup_buffer = (*param).opaque as *mut strdup_buffer;
    if !buf.is_null() {
        let mut i: c_int = 0 as c_int;
        while i < (*buf).count {
            free(*(*buf).ptr.as_mut_ptr().offset(i as isize));
            i += 1;
        }
        free(buf as *mut c_void);
        (*param).opaque = NULL;
    }
}
#[no_mangle]
#[c2rust::src_loc = "266:1"]
unsafe extern "C" fn x264_picture_init(mut pic: *mut x264_picture_t) {
    memset(
        pic as *mut c_void,
        0 as c_int,
        size_of::<x264_picture_t>() as size_t,
    );
    (*pic).i_type = X264_TYPE_AUTO;
    (*pic).i_qpplus1 = X264_QP_AUTO;
    (*pic).i_pic_struct = PIC_STRUCT_AUTO as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "277:1"]
unsafe extern "C" fn x264_picture_alloc(
    mut pic: *mut x264_picture_t,
    mut i_csp: c_int,
    mut i_width: c_int,
    mut i_height: c_int,
) -> c_int {
    static mut csp_tab: [x264_csp_tab_t; 17] = [
        x264_csp_tab_t {
            planes: 0,
            width_fix8: [0; 3],
            height_fix8: [0; 3],
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as c_int,
                width_fix8: [256 as c_int * 1 as c_int, 0, 0],
                height_fix8: [256 as c_int * 1 as c_int, 0, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as c_int,
                width_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int / 2 as c_int,
                    256 as c_int / 2 as c_int,
                ],
                height_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int / 2 as c_int,
                    256 as c_int / 2 as c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as c_int,
                width_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int / 2 as c_int,
                    256 as c_int / 2 as c_int,
                ],
                height_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int / 2 as c_int,
                    256 as c_int / 2 as c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 2 as c_int,
                width_fix8: [256 as c_int * 1 as c_int, 256 as c_int * 1 as c_int, 0],
                height_fix8: [256 as c_int * 1 as c_int, 256 as c_int / 2 as c_int, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 2 as c_int,
                width_fix8: [256 as c_int * 1 as c_int, 256 as c_int * 1 as c_int, 0],
                height_fix8: [256 as c_int * 1 as c_int, 256 as c_int / 2 as c_int, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as c_int,
                width_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int / 2 as c_int,
                    256 as c_int / 2 as c_int,
                ],
                height_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as c_int,
                width_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int / 2 as c_int,
                    256 as c_int / 2 as c_int,
                ],
                height_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 2 as c_int,
                width_fix8: [256 as c_int * 1 as c_int, 256 as c_int * 1 as c_int, 0],
                height_fix8: [256 as c_int * 1 as c_int, 256 as c_int * 1 as c_int, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as c_int,
                width_fix8: [256 as c_int * 2 as c_int, 0, 0],
                height_fix8: [256 as c_int * 1 as c_int, 0, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as c_int,
                width_fix8: [256 as c_int * 2 as c_int, 0, 0],
                height_fix8: [256 as c_int * 1 as c_int, 0, 0],
            };
            init
        },
        x264_csp_tab_t {
            planes: 0,
            width_fix8: [0; 3],
            height_fix8: [0; 3],
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as c_int,
                width_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                ],
                height_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as c_int,
                width_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                ],
                height_fix8: [
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                    256 as c_int * 1 as c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as c_int,
                width_fix8: [256 as c_int * 3 as c_int, 0, 0],
                height_fix8: [256 as c_int * 1 as c_int, 0, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as c_int,
                width_fix8: [256 as c_int * 4 as c_int, 0, 0],
                height_fix8: [256 as c_int * 1 as c_int, 0, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as c_int,
                width_fix8: [256 as c_int * 3 as c_int, 0, 0],
                height_fix8: [256 as c_int * 1 as c_int, 0, 0],
            };
            init
        },
    ];
    let mut csp: c_int = i_csp & X264_CSP_MASK;
    if csp <= X264_CSP_NONE || csp >= X264_CSP_MAX || csp == X264_CSP_V210 {
        return -1;
    }
    x264_picture_init(pic);
    (*pic).img.i_csp = i_csp;
    (*pic).img.i_plane = csp_tab[csp as usize].planes;
    let mut depth_factor: c_int = if i_csp & X264_CSP_HIGH_DEPTH != 0 {
        2 as c_int
    } else {
        1 as c_int
    };
    let mut plane_offset: [int64_t; 3] = [0 as c_int as int64_t, 0, 0];
    let mut frame_size: int64_t = 0 as int64_t;
    let mut i: c_int = 0 as c_int;
    while i < (*pic).img.i_plane {
        let mut stride: c_int = ((i_width as int64_t
            * csp_tab[csp as usize].width_fix8[i as usize] as int64_t
            >> 8 as c_int)
            * depth_factor as int64_t) as c_int;
        let mut plane_size: int64_t = (i_height as int64_t
            * csp_tab[csp as usize].height_fix8[i as usize] as int64_t
            >> 8 as c_int)
            * stride as int64_t;
        (*pic).img.i_stride[i as usize] = stride;
        plane_offset[i as usize] = frame_size;
        frame_size += plane_size;
        i += 1;
    }
    (*pic).img.plane[0] = x264_malloc(frame_size) as *mut uint8_t;
    if (*pic).img.plane[0].is_null() {
        return -1;
    }
    let mut i_0: c_int = 1 as c_int;
    while i_0 < (*pic).img.i_plane {
        (*pic).img.plane[i_0 as usize] =
            (*pic).img.plane[0].offset(plane_offset[i_0 as usize] as isize);
        i_0 += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "333:1"]
unsafe extern "C" fn x264_picture_clean(mut pic: *mut x264_picture_t) {
    x264_free((*pic).img.plane[0] as *mut c_void);
    memset(
        pic as *mut c_void,
        0 as c_int,
        size_of::<x264_picture_t>() as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "344:1"]
pub unsafe extern "C" fn x264_param_default(mut param: *mut x264_param_t) {
    memset(
        param as *mut c_void,
        0 as c_int,
        size_of::<x264_param_t>() as size_t,
    );
    (*param).cpu = x264_cpu_detect();
    (*param).i_threads = X264_THREADS_AUTO;
    (*param).i_lookahead_threads = X264_THREADS_AUTO;
    (*param).b_deterministic = 1 as c_int;
    (*param).i_sync_lookahead = X264_SYNC_LOOKAHEAD_AUTO;
    (*param).i_csp = if X264_CHROMA_FORMAT != 0 {
        X264_CHROMA_FORMAT
    } else {
        X264_CSP_I420
    };
    (*param).width = 0;
    (*param).height = 0;
    (*param).vui.i_sar_width = 0 as c_int;
    (*param).vui.i_sar_height = 0 as c_int;
    (*param).vui.i_overscan = 0 as c_int;
    (*param).vui.i_vidformat = 5 as c_int;
    (*param).vui.b_fullrange = -1;
    (*param).vui.i_colorprim = 2 as c_int;
    (*param).vui.i_transfer = 2 as c_int;
    (*param).vui.i_colmatrix = -1;
    (*param).vui.i_chroma_loc = 0 as c_int;
    (*param).i_fps_num = 25 as uint32_t;
    (*param).i_fps_den = 1 as uint32_t;
    (*param).i_level_idc = -1;
    (*param).i_slice_max_size = 0 as c_int;
    (*param).i_slice_max_mbs = 0 as c_int;
    (*param).i_slice_count = 0 as c_int;
    (*param).i_bitdepth = 8 as c_int;
    (*param).i_frame_reference = 3 as c_int;
    (*param).i_keyint_max = 250 as c_int;
    (*param).i_keyint_min = X264_KEYINT_MIN_AUTO;
    (*param).i_bframe = 3 as c_int;
    (*param).i_scenecut_threshold = 40 as c_int;
    (*param).i_bframe_adaptive = X264_B_ADAPT_FAST;
    (*param).i_bframe_bias = 0 as c_int;
    (*param).bframe_pyramid = BPyramid::default();
    (*param).b_interlaced = 0 as c_int;
    (*param).b_constrained_intra = 0 as c_int;
    (*param).b_deblocking_filter = 1 as c_int;
    (*param).i_deblocking_filter_alphac0 = 0 as c_int;
    (*param).i_deblocking_filter_beta = 0 as c_int;
    (*param).b_cabac = 1 as c_int;
    (*param).i_cabac_init_idc = 0 as c_int;
    (*param).rc.i_rc_method = X264_RC_CRF;
    (*param).rc.i_bitrate = 0 as c_int;
    (*param).rc.f_rate_tolerance = 1.0f32;
    (*param).rc.i_vbv_max_bitrate = 0 as c_int;
    (*param).rc.i_vbv_buffer_size = 0 as c_int;
    (*param).rc.f_vbv_buffer_init = 0.9f32;
    (*param).rc.i_qp_constant = -1;
    (*param).rc.f_rf_constant = 23 as c_int as c_float;
    (*param).rc.i_qp_min = 0 as c_int;
    (*param).rc.i_qp_max = INT_MAX;
    (*param).rc.i_qp_step = 4 as c_int;
    (*param).rc.f_ip_factor = 1.4f32;
    (*param).rc.f_pb_factor = 1.3f32;
    (*param).rc.i_aq_mode = X264_AQ_VARIANCE;
    (*param).rc.f_aq_strength = 1.0f32;
    (*param).rc.i_lookahead = 40 as c_int;
    (*param).rc.b_stat_write = 0 as c_int;
    (*param).rc.psz_stat_out = b"x264_2pass.log\0" as *const u8 as *const c_char as *mut c_char;
    (*param).rc.b_stat_read = 0 as c_int;
    (*param).rc.psz_stat_in = b"x264_2pass.log\0" as *const u8 as *const c_char as *mut c_char;
    (*param).rc.f_qcompress = 0.6f32;
    (*param).rc.f_qblur = 0.5f32;
    (*param).rc.f_complexity_blur = 20 as c_int as c_float;
    (*param).rc.i_zones = 0 as c_int;
    (*param).rc.b_mb_tree = 1 as c_int;
    (*param).pf_log = Some(
        x264_log_default
            as unsafe extern "C" fn(*mut c_void, c_int, *const c_char, ::core::ffi::VaList) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut c_void, c_int, *const c_char, ::core::ffi::VaList) -> (),
        >;
    (*param).p_log_private = NULL;
    (*param).i_log_level = X264_LOG_INFO;
    (*param).analyse.intra = X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8;
    (*param).analyse.inter =
        X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8 | X264_ANALYSE_PSUB16x16 | X264_ANALYSE_BSUB16x16;
    (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_SPATIAL;
    (*param).analyse.me_method = MotionEstimation::default();
    (*param).analyse.f_psy_rd = 1.0f32;
    (*param).analyse.b_psy = 1 as c_int;
    (*param).analyse.f_psy_trellis = 0 as c_int as c_float;
    (*param).analyse.i_me_range = 16 as c_int;
    (*param).analyse.i_subpel_refine = 7 as c_int;
    (*param).analyse.b_mixed_references = 1 as c_int;
    (*param).analyse.b_chroma_me = 1 as c_int;
    (*param).analyse.i_mv_range_thread = -1;
    (*param).analyse.i_mv_range = -1;
    (*param).analyse.i_chroma_qp_offset = 0 as c_int;
    (*param).analyse.b_fast_pskip = 1 as c_int;
    (*param).analyse.b_weighted_bipred = 1 as c_int;
    (*param).analyse.i_weighted_pred = X264_WEIGHTP_SMART;
    (*param).analyse.b_dct_decimate = 1 as c_int;
    (*param).analyse.b_transform_8x8 = 1 as c_int;
    (*param).analyse.i_trellis = 1 as c_int;
    (*param).analyse.i_luma_deadzone[0] = 21 as c_int;
    (*param).analyse.i_luma_deadzone[1] = 11 as c_int;
    (*param).analyse.b_psnr = 0 as c_int;
    (*param).analyse.b_ssim = 0 as c_int;
    (*param).i_cqm_preset = X264_CQM_FLAT;
    memset(
        (*param).cqm_4iy.as_mut_ptr() as *mut c_void,
        16 as c_int,
        size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        (*param).cqm_4py.as_mut_ptr() as *mut c_void,
        16 as c_int,
        size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        (*param).cqm_4ic.as_mut_ptr() as *mut c_void,
        16 as c_int,
        size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        (*param).cqm_4pc.as_mut_ptr() as *mut c_void,
        16 as c_int,
        size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        (*param).cqm_8iy.as_mut_ptr() as *mut c_void,
        16 as c_int,
        size_of::<[uint8_t; 64]>() as size_t,
    );
    memset(
        (*param).cqm_8py.as_mut_ptr() as *mut c_void,
        16 as c_int,
        size_of::<[uint8_t; 64]>() as size_t,
    );
    memset(
        (*param).cqm_8ic.as_mut_ptr() as *mut c_void,
        16 as c_int,
        size_of::<[uint8_t; 64]>() as size_t,
    );
    memset(
        (*param).cqm_8pc.as_mut_ptr() as *mut c_void,
        16 as c_int,
        size_of::<[uint8_t; 64]>() as size_t,
    );
    (*param).b_repeat_headers = 1 as c_int;
    (*param).b_annexb = 1 as c_int;
    (*param).b_aud = 0 as c_int;
    (*param).b_vfr_input = 1 as c_int;
    (*param).i_nal_hrd = X264_NAL_HRD_NONE;
    (*param).b_tff = 1 as c_int;
    (*param).b_pic_struct = 0 as c_int;
    (*param).b_fake_interlaced = 0 as c_int;
    (*param).frame_packing = None;
    (*param).i_alternative_transfer = 2 as c_int;
    (*param).b_opencl = 0 as c_int;
    (*param).i_opencl_device = 0 as c_int;
    (*param).opencl_device_id = NULL;
    (*param).psz_clbin_file = 0 as *mut c_char;
    (*param).i_avcintra_class = 0 as c_int;
    (*param).i_avcintra_flavor = X264_AVCINTRA_FLAVOR_PANASONIC;
}
#[c2rust::src_loc = "489:1"]
unsafe extern "C" fn param_apply_preset(
    mut param: *mut x264_param_t,
    mut preset: *const c_char,
) -> c_int {
    let mut end: *mut c_char = 0 as *mut c_char;
    let mut i: c_int = strtol(preset, &mut end, 10 as c_int) as c_int;
    if *end as c_int == 0 as c_int
        && i >= 0 as c_int
        && i < (size_of::<[*const c_char; 11]>() as usize)
            .wrapping_div(size_of::<*const c_char>() as usize) as c_int
            - 1 as c_int
    {
        preset = x264_preset_names[i as usize];
    }
    if strcasecmp(preset, b"ultrafast\0" as *const u8 as *const c_char) == 0 {
        (*param).i_frame_reference = 1 as c_int;
        (*param).i_scenecut_threshold = 0 as c_int;
        (*param).b_deblocking_filter = 0 as c_int;
        (*param).b_cabac = 0 as c_int;
        (*param).i_bframe = 0 as c_int;
        (*param).analyse.intra = 0 as c_uint;
        (*param).analyse.inter = 0 as c_uint;
        (*param).analyse.b_transform_8x8 = 0 as c_int;
        (*param).analyse.me_method = MotionEstimation::Dia;
        (*param).analyse.i_subpel_refine = 0 as c_int;
        (*param).rc.i_aq_mode = 0 as c_int;
        (*param).analyse.b_mixed_references = 0 as c_int;
        (*param).analyse.i_trellis = 0 as c_int;
        (*param).i_bframe_adaptive = X264_B_ADAPT_NONE;
        (*param).rc.b_mb_tree = 0 as c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_NONE;
        (*param).analyse.b_weighted_bipred = 0 as c_int;
        (*param).rc.i_lookahead = 0 as c_int;
    } else if strcasecmp(preset, b"superfast\0" as *const u8 as *const c_char) == 0 {
        (*param).analyse.inter = X264_ANALYSE_I8x8 | X264_ANALYSE_I4x4;
        (*param).analyse.me_method = MotionEstimation::Dia;
        (*param).analyse.i_subpel_refine = 1 as c_int;
        (*param).i_frame_reference = 1 as c_int;
        (*param).analyse.b_mixed_references = 0 as c_int;
        (*param).analyse.i_trellis = 0 as c_int;
        (*param).rc.b_mb_tree = 0 as c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_SIMPLE;
        (*param).rc.i_lookahead = 0 as c_int;
    } else if strcasecmp(preset, b"veryfast\0" as *const u8 as *const c_char) == 0 {
        (*param).analyse.i_subpel_refine = 2 as c_int;
        (*param).i_frame_reference = 1 as c_int;
        (*param).analyse.b_mixed_references = 0 as c_int;
        (*param).analyse.i_trellis = 0 as c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_SIMPLE;
        (*param).rc.i_lookahead = 10 as c_int;
    } else if strcasecmp(preset, b"faster\0" as *const u8 as *const c_char) == 0 {
        (*param).analyse.b_mixed_references = 0 as c_int;
        (*param).i_frame_reference = 2 as c_int;
        (*param).analyse.i_subpel_refine = 4 as c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_SIMPLE;
        (*param).rc.i_lookahead = 20 as c_int;
    } else if strcasecmp(preset, b"fast\0" as *const u8 as *const c_char) == 0 {
        (*param).i_frame_reference = 2 as c_int;
        (*param).analyse.i_subpel_refine = 6 as c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_SIMPLE;
        (*param).rc.i_lookahead = 30 as c_int;
    } else if !(strcasecmp(preset, b"medium\0" as *const u8 as *const c_char) == 0) {
        if strcasecmp(preset, b"slow\0" as *const u8 as *const c_char) == 0 {
            (*param).analyse.i_subpel_refine = 8 as c_int;
            (*param).i_frame_reference = 5 as c_int;
            (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_AUTO;
            (*param).analyse.i_trellis = 2 as c_int;
            (*param).rc.i_lookahead = 50 as c_int;
        } else if strcasecmp(preset, b"slower\0" as *const u8 as *const c_char) == 0 {
            (*param).analyse.me_method = MotionEstimation::Umh;
            (*param).analyse.i_subpel_refine = 9 as c_int;
            (*param).i_frame_reference = 8 as c_int;
            (*param).i_bframe_adaptive = X264_B_ADAPT_TRELLIS;
            (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_AUTO;
            (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
            (*param).analyse.i_trellis = 2 as c_int;
            (*param).rc.i_lookahead = 60 as c_int;
        } else if strcasecmp(preset, b"veryslow\0" as *const u8 as *const c_char) == 0 {
            (*param).analyse.me_method = MotionEstimation::Umh;
            (*param).analyse.i_subpel_refine = 10 as c_int;
            (*param).analyse.i_me_range = 24 as c_int;
            (*param).i_frame_reference = 16 as c_int;
            (*param).i_bframe_adaptive = X264_B_ADAPT_TRELLIS;
            (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_AUTO;
            (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
            (*param).analyse.i_trellis = 2 as c_int;
            (*param).i_bframe = 8 as c_int;
            (*param).rc.i_lookahead = 60 as c_int;
        } else if strcasecmp(preset, b"placebo\0" as *const u8 as *const c_char) == 0 {
            (*param).analyse.me_method = MotionEstimation::Tesa;
            (*param).analyse.i_subpel_refine = 11 as c_int;
            (*param).analyse.i_me_range = 24 as c_int;
            (*param).i_frame_reference = 16 as c_int;
            (*param).i_bframe_adaptive = X264_B_ADAPT_TRELLIS;
            (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_AUTO;
            (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
            (*param).analyse.b_fast_pskip = 0 as c_int;
            (*param).analyse.i_trellis = 2 as c_int;
            (*param).i_bframe = 16 as c_int;
            (*param).rc.i_lookahead = 60 as c_int;
        } else {
            x264_log_internal(
                X264_LOG_ERROR,
                b"invalid preset '%s'\n\0" as *const u8 as *const c_char,
                preset,
            );
            return -1;
        }
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "611:1"]
unsafe extern "C" fn param_apply_tune(
    mut param: *mut x264_param_t,
    mut tune: *const c_char,
) -> c_int {
    let mut current_block: u64;
    let mut psy_tuning_used: c_int = 0 as c_int;
    let mut len: c_int = 0;
    loop {
        tune = tune.offset(strspn(tune, b",./-+\0" as *const u8 as *const c_char) as isize);
        len = strcspn(tune, b",./-+\0" as *const u8 as *const c_char) as c_int;
        if !(len != 0) {
            break;
        }
        if len == 4 as c_int
            && strncasecmp(tune, b"film\0" as *const u8 as *const c_char, 4 as size_t) == 0
        {
            let fresh4 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh4 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_deblocking_filter_alphac0 = -1;
                (*param).i_deblocking_filter_beta = -1;
                (*param).analyse.f_psy_trellis = 0.15f32;
                current_block = 11174649648027449784;
            }
        } else if len == 9 as c_int
            && strncasecmp(
                tune,
                b"animation\0" as *const u8 as *const c_char,
                9 as size_t,
            ) == 0
        {
            let fresh5 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh5 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_frame_reference = if (*param).i_frame_reference > 1 as c_int {
                    (*param).i_frame_reference * 2 as c_int
                } else {
                    1 as c_int
                };
                (*param).i_deblocking_filter_alphac0 = 1 as c_int;
                (*param).i_deblocking_filter_beta = 1 as c_int;
                (*param).analyse.f_psy_rd = 0.4f32;
                (*param).rc.f_aq_strength = 0.6f32;
                (*param).i_bframe += 2 as c_int;
                current_block = 11174649648027449784;
            }
        } else if len == 5 as c_int
            && strncasecmp(tune, b"grain\0" as *const u8 as *const c_char, 5 as size_t) == 0
        {
            let fresh6 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh6 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_deblocking_filter_alphac0 = -(2 as c_int);
                (*param).i_deblocking_filter_beta = -(2 as c_int);
                (*param).analyse.f_psy_trellis = 0.25f32;
                (*param).analyse.b_dct_decimate = 0 as c_int;
                (*param).rc.f_pb_factor = 1.1f32;
                (*param).rc.f_ip_factor = 1.1f32;
                (*param).rc.f_aq_strength = 0.5f32;
                (*param).analyse.i_luma_deadzone[0] = 6 as c_int;
                (*param).analyse.i_luma_deadzone[1] = 6 as c_int;
                (*param).rc.f_qcompress = 0.8f32;
                current_block = 11174649648027449784;
            }
        } else if len == 10 as c_int
            && strncasecmp(
                tune,
                b"stillimage\0" as *const u8 as *const c_char,
                10 as size_t,
            ) == 0
        {
            let fresh7 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh7 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_deblocking_filter_alphac0 = -(3 as c_int);
                (*param).i_deblocking_filter_beta = -(3 as c_int);
                (*param).analyse.f_psy_rd = 2.0f32;
                (*param).analyse.f_psy_trellis = 0.7f32;
                (*param).rc.f_aq_strength = 1.2f32;
                current_block = 11174649648027449784;
            }
        } else if len == 4 as c_int
            && strncasecmp(tune, b"psnr\0" as *const u8 as *const c_char, 4 as size_t) == 0
        {
            let fresh8 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh8 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).rc.i_aq_mode = X264_AQ_NONE;
                (*param).analyse.b_psy = 0 as c_int;
                current_block = 11174649648027449784;
            }
        } else if len == 4 as c_int
            && strncasecmp(tune, b"ssim\0" as *const u8 as *const c_char, 4 as size_t) == 0
        {
            let fresh9 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh9 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).rc.i_aq_mode = X264_AQ_AUTOVARIANCE;
                (*param).analyse.b_psy = 0 as c_int;
                current_block = 11174649648027449784;
            }
        } else if len == 10 as c_int
            && strncasecmp(
                tune,
                b"fastdecode\0" as *const u8 as *const c_char,
                10 as size_t,
            ) == 0
        {
            (*param).b_deblocking_filter = 0 as c_int;
            (*param).b_cabac = 0 as c_int;
            (*param).analyse.b_weighted_bipred = 0 as c_int;
            (*param).analyse.i_weighted_pred = X264_WEIGHTP_NONE;
            current_block = 11174649648027449784;
        } else if len == 11 as c_int
            && strncasecmp(
                tune,
                b"zerolatency\0" as *const u8 as *const c_char,
                11 as size_t,
            ) == 0
        {
            (*param).rc.i_lookahead = 0 as c_int;
            (*param).i_sync_lookahead = 0 as c_int;
            (*param).i_bframe = 0 as c_int;
            (*param).b_sliced_threads = 1 as c_int;
            (*param).b_vfr_input = 0 as c_int;
            (*param).rc.b_mb_tree = 0 as c_int;
            current_block = 11174649648027449784;
        } else if len == 6 as c_int
            && strncasecmp(tune, b"touhou\0" as *const u8 as *const c_char, 6 as size_t) == 0
        {
            let fresh10 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh10 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_frame_reference = if (*param).i_frame_reference > 1 as c_int {
                    (*param).i_frame_reference * 2 as c_int
                } else {
                    1 as c_int
                };
                (*param).i_deblocking_filter_alphac0 = -1;
                (*param).i_deblocking_filter_beta = -1;
                (*param).analyse.f_psy_trellis = 0.2f32;
                (*param).rc.f_aq_strength = 1.3f32;
                if (*param).analyse.inter & X264_ANALYSE_PSUB16x16 != 0 {
                    (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
                }
                current_block = 11174649648027449784;
            }
        } else {
            x264_log_internal(
                X264_LOG_ERROR,
                b"invalid tune '%.*s'\n\0" as *const u8 as *const c_char,
                len,
                tune,
            );
            return -1;
        }
        match current_block {
            11494378617088087400 => {
                x264_log_internal(
                    X264_LOG_WARNING,
                    b"only 1 psy tuning can be used: ignoring tune %.*s\n\0" as *const u8
                        as *const c_char,
                    len,
                    tune,
                );
            }
            _ => {}
        }
        tune = tune.offset(len as isize);
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "706:1"]
pub unsafe extern "C" fn x264_param_default_preset(
    mut param: *mut x264_param_t,
    mut preset: *const c_char,
    mut tune: *const c_char,
) -> c_int {
    x264_param_default(param);
    if !preset.is_null() && param_apply_preset(param, preset) < 0 as c_int {
        return -1;
    }
    if !tune.is_null() && param_apply_tune(param, tune) < 0 as c_int {
        return -1;
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "717:1"]
unsafe extern "C" fn x264_param_apply_fastfirstpass(mut param: *mut x264_param_t) {
    if (*param).rc.b_stat_write != 0 && (*param).rc.b_stat_read == 0 {
        (*param).i_frame_reference = 1 as c_int;
        (*param).analyse.b_transform_8x8 = 0 as c_int;
        (*param).analyse.inter = 0 as c_uint;
        (*param).analyse.me_method = MotionEstimation::Dia;
        (*param).analyse.i_subpel_refine = if (2 as c_int) < (*param).analyse.i_subpel_refine {
            2 as c_int
        } else {
            (*param).analyse.i_subpel_refine
        };
        (*param).analyse.i_trellis = 0 as c_int;
        (*param).analyse.b_fast_pskip = 1 as c_int;
    }
}
#[c2rust::src_loc = "732:1"]
unsafe extern "C" fn profile_string_to_int(mut str: *const c_char) -> c_int {
    if strcasecmp(str, b"baseline\0" as *const u8 as *const c_char) == 0 {
        return PROFILE_BASELINE as c_int;
    }
    if strcasecmp(str, b"main\0" as *const u8 as *const c_char) == 0 {
        return PROFILE_MAIN as c_int;
    }
    if strcasecmp(str, b"high\0" as *const u8 as *const c_char) == 0 {
        return PROFILE_HIGH as c_int;
    }
    if strcasecmp(str, b"high10\0" as *const u8 as *const c_char) == 0 {
        return PROFILE_HIGH10 as c_int;
    }
    if strcasecmp(str, b"high422\0" as *const u8 as *const c_char) == 0 {
        return PROFILE_HIGH422 as c_int;
    }
    if strcasecmp(str, b"high444\0" as *const u8 as *const c_char) == 0 {
        return PROFILE_HIGH444_PREDICTIVE as c_int;
    }
    return -1;
}
#[no_mangle]
#[c2rust::src_loc = "749:1"]
unsafe extern "C" fn x264_param_apply_profile(
    mut param: *mut x264_param_t,
    mut profile: *const c_char,
) -> c_int {
    if profile.is_null() {
        return 0 as c_int;
    }
    let qp_bd_offset: c_int = 6 as c_int * ((*param).i_bitdepth - 8 as c_int);
    let mut p: c_int = profile_string_to_int(profile);
    if p < 0 as c_int {
        x264_log_internal(
            X264_LOG_ERROR,
            b"invalid profile: %s\n\0" as *const u8 as *const c_char,
            profile,
        );
        return -1;
    }
    if p < PROFILE_HIGH444_PREDICTIVE as c_int
        && ((*param).rc.i_rc_method == X264_RC_CQP && (*param).rc.i_qp_constant <= 0 as c_int
            || (*param).rc.i_rc_method == X264_RC_CRF
                && ((*param).rc.f_rf_constant + qp_bd_offset as c_float) as c_int <= 0 as c_int)
    {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support lossless\n\0" as *const u8 as *const c_char,
            profile,
        );
        return -1;
    }
    if p < PROFILE_HIGH444_PREDICTIVE as c_int && (*param).i_csp & X264_CSP_MASK >= X264_CSP_I444 {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support 4:4:4\n\0" as *const u8 as *const c_char,
            profile,
        );
        return -1;
    }
    if p < PROFILE_HIGH422 as c_int && (*param).i_csp & X264_CSP_MASK >= X264_CSP_I422 {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support 4:2:2\n\0" as *const u8 as *const c_char,
            profile,
        );
        return -1;
    }
    if p < PROFILE_HIGH10 as c_int && (*param).i_bitdepth > 8 as c_int {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support a bit depth of %d\n\0" as *const u8 as *const c_char,
            profile,
            (*param).i_bitdepth,
        );
        return -1;
    }
    if p < PROFILE_HIGH as c_int && (*param).i_csp & X264_CSP_MASK == X264_CSP_I400 {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support 4:0:0\n\0" as *const u8 as *const c_char,
            profile,
        );
        return -1;
    }
    if p == PROFILE_BASELINE as c_int {
        (*param).analyse.b_transform_8x8 = 0 as c_int;
        (*param).b_cabac = 0 as c_int;
        (*param).i_cqm_preset = X264_CQM_FLAT;
        (*param).psz_cqm_file = 0 as *mut c_char;
        (*param).i_bframe = 0 as c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_NONE;
        if (*param).b_interlaced != 0 {
            x264_log_internal(
                X264_LOG_ERROR,
                b"baseline profile doesn't support interlacing\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        if (*param).b_fake_interlaced != 0 {
            x264_log_internal(
                X264_LOG_ERROR,
                b"baseline profile doesn't support fake interlacing\n\0" as *const u8
                    as *const c_char,
            );
            return -1;
        }
    } else if p == PROFILE_MAIN as c_int {
        (*param).analyse.b_transform_8x8 = 0 as c_int;
        (*param).i_cqm_preset = X264_CQM_FLAT;
        (*param).psz_cqm_file = 0 as *mut c_char;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "816:1"]
unsafe extern "C" fn parse_enum(
    mut arg: *const c_char,
    mut names: *const *const c_char,
    mut dst: *mut i32,
) -> c_int {
    let mut i = 0;
    while !(*names.offset(i)).is_null() {
        if **names.offset(i) != 0 && strcasecmp(arg, *names.offset(i)) == 0 {
            *dst = i as i32;
            return 0;
        }
        i += 1;
    }
    return -1;
}
use core::ffi::CStr;

// TODO: is this used enough to justify? Most just do atoi directly instead of trying to extract a
// string representation.
macro_rules! parse_enum_param {
    ($value:expr, $enum_type:ty) => {
        (|| -> Result<$enum_type, ()> {
            let s = CStr::from_ptr($value).to_str().map_err(|_| ())?;
            <$enum_type>::from_str(s).or_else(|_| {
                let num = s.parse::<usize>().map_err(|_| ())?;
                <$enum_type>::from_repr(num).ok_or(())
            })
        })()
    };
}

#[c2rust::src_loc = "827:1"]
unsafe extern "C" fn parse_cqm(
    mut str: *const c_char,
    mut cqm: *mut uint8_t,
    mut length: c_int,
) -> c_int {
    let mut i: c_int = 0 as c_int;
    loop {
        let mut coef: c_int = 0;
        if sscanf(
            str,
            b"%d\0" as *const u8 as *const c_char,
            &mut coef as *mut c_int,
        ) == 0
            || coef < 1 as c_int
            || coef > 255 as c_int
        {
            return -1;
        }
        let fresh2 = i;
        i = i + 1;
        *cqm.offset(fresh2 as isize) = coef as uint8_t;
        if !(i < length
            && {
                str = strchr(str, ',' as i32);
                !str.is_null()
            }
            && {
                let fresh3 = str;
                str = str.offset(1);
                !fresh3.is_null()
            })
        {
            break;
        }
    }
    return if i == length { 0 as c_int } else { -1 };
}
#[c2rust::src_loc = "839:1"]
unsafe extern "C" fn atobool_internal(mut str: *const c_char, mut b_error: *mut c_int) -> c_int {
    if strcmp(str, b"1\0" as *const u8 as *const c_char) == 0
        || strcasecmp(str, b"true\0" as *const u8 as *const c_char) == 0
        || strcasecmp(str, b"yes\0" as *const u8 as *const c_char) == 0
    {
        return 1 as c_int;
    }
    if strcmp(str, b"0\0" as *const u8 as *const c_char) == 0
        || strcasecmp(str, b"false\0" as *const u8 as *const c_char) == 0
        || strcasecmp(str, b"no\0" as *const u8 as *const c_char) == 0
    {
        return 0 as c_int;
    }
    *b_error = 1 as c_int;
    return 0 as c_int;
}
#[c2rust::src_loc = "853:1"]
unsafe extern "C" fn atoi_internal(mut str: *const c_char, mut b_error: *mut c_int) -> c_int {
    let mut end: *mut c_char = 0 as *mut c_char;
    let mut v: c_int = strtol(str, &mut end, 0 as c_int) as c_int;
    if end == str as *mut c_char || *end as c_int != '\0' as i32 {
        *b_error = 1 as c_int;
    }
    return v;
}
#[c2rust::src_loc = "862:1"]
unsafe extern "C" fn atof_internal(mut str: *const c_char, mut b_error: *mut c_int) -> c_double {
    let mut end: *mut c_char = 0 as *mut c_char;
    let mut v: c_double = strtod(str, &mut end);
    if end == str as *mut c_char || *end as c_int != '\0' as i32 {
        *b_error = 1 as c_int;
    }
    return v;
}
#[no_mangle]
#[c2rust::src_loc = "886:1"]
pub unsafe extern "C" fn x264_param_parse(
    mut p: *mut x264_param_t,
    mut name: *const c_char,
    mut value: *const c_char,
) -> c_int {
    let mut name_buf: *mut c_char = 0 as *mut c_char;
    let mut b_error: c_int = 0 as c_int;
    let mut errortype: c_int = X264_PARAM_BAD_VALUE;
    let mut name_was_bool: c_int = 0;
    let mut value_was_null: c_int = value.is_null() as c_int;
    if name.is_null() {
        return X264_PARAM_BAD_NAME;
    }
    if value.is_null() {
        value = b"true\0" as *const u8 as *const c_char;
    }
    if *value.offset(0) as c_int == '=' as i32 {
        value = value.offset(1);
    }
    if !strchr(name, '_' as i32).is_null() {
        let mut c: *mut c_char = 0 as *mut c_char;
        name_buf = strdup(name);
        if name_buf.is_null() {
            return X264_PARAM_ALLOC_FAILED;
        }
        loop {
            c = strchr(name_buf, '_' as i32);
            if c.is_null() {
                break;
            }
            *c = '-' as i32 as c_char;
        }
        name = name_buf;
    }
    if strncmp(name, b"no\0" as *const u8 as *const c_char, 2 as size_t) == 0 {
        name = name.offset(2);
        if *name.offset(0) as c_int == '-' as i32 {
            name = name.offset(1);
        }
        name_was_bool = 1 as c_int;
        value = if atobool_internal(value, &mut b_error) != 0 {
            b"false\0" as *const u8 as *const c_char
        } else {
            b"true\0" as *const u8 as *const c_char
        };
    }
    name_was_bool = 0 as c_int;
    if strcmp(name, b"asm\0" as *const u8 as *const c_char) == 0 {
        (*p).cpu = if *(*__ctype_b_loc()).offset(*value.offset(0) as c_uchar as c_int as isize)
            as c_int
            & _ISdigit as c_int as c_ushort as c_int
            != 0
        {
            atoi_internal(value, &mut b_error) as uint32_t
        } else if strcasecmp(value, b"auto\0" as *const u8 as *const c_char) == 0 || {
            name_was_bool = 1 as c_int;
            atobool_internal(value, &mut b_error) != 0
        } {
            x264_cpu_detect()
        } else {
            0 as uint32_t
        };
        if b_error != 0 {
            let mut buf: *mut c_char = strdup(value);
            if !buf.is_null() {
                let mut tok: *mut c_char = 0 as *mut c_char;
                let mut saveptr: *mut c_char = 0 as *mut c_char;
                let mut init: *mut c_char = 0 as *mut c_char;
                b_error = 0 as c_int;
                (*p).cpu = 0 as uint32_t;
                init = buf;
                loop {
                    tok = strtok_r(init, b",\0" as *const u8 as *const c_char, &mut saveptr);
                    if tok.is_null() {
                        break;
                    }
                    let mut i: c_int = 0 as c_int;
                    while (*x264_cpu_names.as_ptr().offset(i as isize)).flags != 0
                        && strcasecmp(tok, (*x264_cpu_names.as_ptr().offset(i as isize)).name) != 0
                    {
                        i += 1;
                    }
                    (*p).cpu |= (*x264_cpu_names.as_ptr().offset(i as isize)).flags;
                    if (*x264_cpu_names.as_ptr().offset(i as isize)).flags == 0 {
                        b_error = 1 as c_int;
                    }
                    init = 0 as *mut c_char;
                }
                free(buf as *mut c_void);
                if (*p).cpu & X264_CPU_SSSE3 as uint32_t != 0
                    && (*p).cpu & X264_CPU_SSE2_IS_SLOW as uint32_t == 0
                {
                    (*p).cpu = ((*p).cpu as c_uint | X264_CPU_SSE2_IS_FAST) as uint32_t;
                }
            } else {
                errortype = X264_PARAM_ALLOC_FAILED;
            }
        }
    } else if strcmp(name, b"threads\0" as *const u8 as *const c_char) == 0 {
        if strcasecmp(value, b"auto\0" as *const u8 as *const c_char) == 0 {
            (*p).i_threads = X264_THREADS_AUTO;
        } else {
            (*p).i_threads = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"lookahead-threads\0" as *const u8 as *const c_char) == 0 {
        if strcasecmp(value, b"auto\0" as *const u8 as *const c_char) == 0 {
            (*p).i_lookahead_threads = X264_THREADS_AUTO;
        } else {
            (*p).i_lookahead_threads = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"sliced-threads\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_sliced_threads = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"sync-lookahead\0" as *const u8 as *const c_char) == 0 {
        if strcasecmp(value, b"auto\0" as *const u8 as *const c_char) == 0 {
            (*p).i_sync_lookahead = X264_SYNC_LOOKAHEAD_AUTO;
        } else {
            (*p).i_sync_lookahead = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"deterministic\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"n-deterministic\0" as *const u8 as *const c_char) == 0
    {
        name_was_bool = 1 as c_int;
        (*p).b_deterministic = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"cpu-independent\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_cpu_independent = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"level\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"level-idc\0" as *const u8 as *const c_char) == 0
    {
        if strcmp(value, b"1b\0" as *const u8 as *const c_char) == 0 {
            (*p).i_level_idc = 9 as c_int;
        } else if atof_internal(value, &mut b_error) < 7 as c_int as c_double {
            (*p).i_level_idc =
                (10 as c_int as c_double * atof_internal(value, &mut b_error) + 0.5f64) as c_int;
        } else {
            (*p).i_level_idc = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"bluray-compat\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_bluray_compat = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"avcintra-class\0" as *const u8 as *const c_char) == 0 {
        (*p).i_avcintra_class = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"avcintra-flavor\0" as *const u8 as *const c_char) == 0 {
        b_error |= parse_enum(
            value,
            x264_avcintra_flavor_names.as_ptr(),
            &mut (*p).i_avcintra_flavor,
        );
    } else if strcmp(name, b"sar\0" as *const u8 as *const c_char) == 0 {
        b_error |= (2 as c_int
            != sscanf(
                value,
                b"%d:%d\0" as *const u8 as *const c_char,
                &mut (*p).vui.i_sar_width as *mut c_int,
                &mut (*p).vui.i_sar_height as *mut c_int,
            )
            && 2 as c_int
                != sscanf(
                    value,
                    b"%d/%d\0" as *const u8 as *const c_char,
                    &mut (*p).vui.i_sar_width as *mut c_int,
                    &mut (*p).vui.i_sar_height as *mut c_int,
                )) as c_int;
    } else if strcmp(name, b"overscan\0" as *const u8 as *const c_char) == 0 {
        b_error |= parse_enum(
            value,
            x264_overscan_names.as_ptr(),
            &mut (*p).vui.i_overscan,
        );
    } else if strcmp(name, b"videoformat\0" as *const u8 as *const c_char) == 0 {
        b_error |= parse_enum(
            value,
            x264_vidformat_names.as_ptr(),
            &mut (*p).vui.i_vidformat,
        );
    } else if strcmp(name, b"fullrange\0" as *const u8 as *const c_char) == 0 {
        b_error |= parse_enum(
            value,
            x264_fullrange_names.as_ptr(),
            &mut (*p).vui.b_fullrange,
        );
    } else if strcmp(name, b"colorprim\0" as *const u8 as *const c_char) == 0 {
        b_error |= parse_enum(
            value,
            x264_colorprim_names.as_ptr(),
            &mut (*p).vui.i_colorprim,
        );
    } else if strcmp(name, b"transfer\0" as *const u8 as *const c_char) == 0 {
        b_error |= parse_enum(
            value,
            x264_transfer_names.as_ptr(),
            &mut (*p).vui.i_transfer,
        );
    } else if strcmp(name, b"colormatrix\0" as *const u8 as *const c_char) == 0 {
        b_error |= parse_enum(
            value,
            x264_colmatrix_names.as_ptr(),
            &mut (*p).vui.i_colmatrix,
        );
    } else if strcmp(name, b"chromaloc\0" as *const u8 as *const c_char) == 0 {
        (*p).vui.i_chroma_loc = atoi_internal(value, &mut b_error);
        b_error |=
            ((*p).vui.i_chroma_loc < 0 as c_int || (*p).vui.i_chroma_loc > 5 as c_int) as c_int;
    } else if strcmp(name, b"mastering-display\0" as *const u8 as *const c_char) == 0 {
        if strcasecmp(value, b"undef\0" as *const u8 as *const c_char) != 0 {
            let mut green_x: i32 = 0;
            let mut green_y: i32 = 0;
            let mut blue_x: i32 = 0;
            let mut blue_y: i32 = 0;
            let mut red_x: i32 = 0;
            let mut red_y: i32 = 0;
            let mut white_x: i32 = 0;
            let mut white_y: i32 = 0;
            let mut display_max: i64 = 0;
            let mut display_min: i64 = 0;

            let num_values = sscanf(
                value,
                c"G(%d,%d)B(%d,%d)R(%d,%d)WP(%d,%d)L(%ld,%ld)".as_ptr(),
                &mut green_x,
                &mut green_y,
                &mut blue_x,
                &mut blue_y,
                &mut red_x,
                &mut red_y,
                &mut white_x,
                &mut white_y,
                &mut display_max,
                &mut display_min,
            );

            b_error |= (num_values != 10) as i32;

            (*p).mastering_display = if num_values == 10
                && let Ok(green_x) = u16::try_from(green_x)
                && let Ok(green_y) = u16::try_from(green_y)
                && let Ok(blue_x) = u16::try_from(blue_x)
                && let Ok(blue_y) = u16::try_from(blue_y)
                && let Ok(red_x) = u16::try_from(red_x)
                && let Ok(red_y) = u16::try_from(red_y)
                && let Ok(white_x) = u16::try_from(white_x)
                && let Ok(white_y) = u16::try_from(white_y)
                && let Ok(display_max) = u32::try_from(display_max)
                && let Ok(display_min) = u32::try_from(display_min)
            {
                Some(MasteringDisplay {
                    green: (green_x, green_y),
                    blue: (blue_x, blue_y),
                    red: (red_x, red_y),
                    white: (white_x, white_y),
                    display_max,
                    display_min,
                })
            } else {
                None
            };
        } else {
            (*p).mastering_display = None;
        }
    } else if strcmp(name, b"cll\0" as *const u8 as *const c_char) == 0 {
        if strcasecmp(value, b"undef\0" as *const u8 as *const c_char) != 0 {
            let mut max_cll: i32 = 0;
            let mut max_fall: i32 = 0;

            let num_values = sscanf(value, c"%d,%d".as_ptr(), &mut max_cll, &mut max_fall);

            b_error |= (num_values != 2) as i32;

            (*p).content_light_level = if num_values == 2
                && let Ok(max_cll) = u16::try_from(max_cll)
                && let Ok(max_fall) = u16::try_from(max_fall)
            {
                Some(ContentLightLevel { max_cll, max_fall })
            } else {
                None
            };
        } else {
            (*p).content_light_level = None;
        }
    } else if strcmp(
        name,
        b"alternative-transfer\0" as *const u8 as *const c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_transfer_names.as_ptr(),
            &mut (*p).i_alternative_transfer,
        );
    } else if strcmp(name, b"fps\0" as *const u8 as *const c_char) == 0 {
        let mut i_fps_num: int64_t = 0;
        let mut i_fps_den: int64_t = 0;
        if sscanf(
            value,
            b"%ld/%ld\0" as *const u8 as *const c_char,
            &mut i_fps_num as *mut int64_t,
            &mut i_fps_den as *mut int64_t,
        ) == 2 as c_int
        {
            (*p).i_fps_num = i_fps_num as uint32_t;
            (*p).i_fps_den = i_fps_den as uint32_t;
            b_error |= (i_fps_num < 1 as int64_t
                || i_fps_num > UINT32_MAX as int64_t
                || i_fps_den < 1 as int64_t
                || i_fps_den > UINT32_MAX as int64_t) as c_int;
        } else {
            let mut fps: c_double = atof_internal(value, &mut b_error);
            if fps < 0.0005f64 || fps > INT_MAX as c_double {
                b_error = 1 as c_int;
            } else if fps <= INT_MAX as c_double / 1000.0f64 {
                (*p).i_fps_num = (fps * 1000.0f64 + 0.5f64) as c_int as uint32_t;
                (*p).i_fps_den = 1000 as uint32_t;
            } else {
                (*p).i_fps_num = atoi_internal(value, &mut b_error) as uint32_t;
                (*p).i_fps_den = 1 as uint32_t;
            }
        }
    } else if strcmp(name, b"ref\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"frameref\0" as *const u8 as *const c_char) == 0
    {
        (*p).i_frame_reference = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"dpb-size\0" as *const u8 as *const c_char) == 0 {
        (*p).i_dpb_size = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"keyint\0" as *const u8 as *const c_char) == 0 {
        if !strstr(value, b"infinite\0" as *const u8 as *const c_char).is_null() {
            (*p).i_keyint_max = X264_KEYINT_MAX_INFINITE;
        } else {
            (*p).i_keyint_max = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"min-keyint\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"keyint-min\0" as *const u8 as *const c_char) == 0
    {
        (*p).i_keyint_min = atoi_internal(value, &mut b_error);
        if (*p).i_keyint_max < (*p).i_keyint_min {
            (*p).i_keyint_max = (*p).i_keyint_min;
        }
    } else if strcmp(name, b"scenecut\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).i_scenecut_threshold = atobool_internal(value, &mut b_error);
        if b_error != 0 || (*p).i_scenecut_threshold != 0 {
            b_error = 0 as c_int;
            (*p).i_scenecut_threshold = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"intra-refresh\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_intra_refresh = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"bframes\0" as *const u8 as *const c_char) == 0 {
        (*p).i_bframe = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"b-adapt\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).i_bframe_adaptive = atobool_internal(value, &mut b_error);
        if b_error != 0 {
            b_error = 0 as c_int;
            (*p).i_bframe_adaptive = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"b-bias\0" as *const u8 as *const c_char) == 0 {
        (*p).i_bframe_bias = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"b-pyramid\0" as *const u8 as *const c_char) == 0 {
        (*p).bframe_pyramid = parse_enum_param!(value, BPyramid).unwrap_or_else(|_| {
            b_error = 1;
            BPyramid::None
        });
    } else if strcmp(name, b"open-gop\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_open_gop = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"nf\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_deblocking_filter = (atobool_internal(value, &mut b_error) == 0) as c_int;
    } else if strcmp(name, b"filter\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"deblock\0" as *const u8 as *const c_char) == 0
    {
        if 2 as c_int
            == sscanf(
                value,
                b"%d:%d\0" as *const u8 as *const c_char,
                &mut (*p).i_deblocking_filter_alphac0 as *mut c_int,
                &mut (*p).i_deblocking_filter_beta as *mut c_int,
            )
            || 2 as c_int
                == sscanf(
                    value,
                    b"%d,%d\0" as *const u8 as *const c_char,
                    &mut (*p).i_deblocking_filter_alphac0 as *mut c_int,
                    &mut (*p).i_deblocking_filter_beta as *mut c_int,
                )
        {
            (*p).b_deblocking_filter = 1 as c_int;
        } else if sscanf(
            value,
            b"%d\0" as *const u8 as *const c_char,
            &mut (*p).i_deblocking_filter_alphac0 as *mut c_int,
        ) != 0
        {
            (*p).b_deblocking_filter = 1 as c_int;
            (*p).i_deblocking_filter_beta = (*p).i_deblocking_filter_alphac0;
        } else {
            name_was_bool = 1 as c_int;
            (*p).b_deblocking_filter = atobool_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"slice-max-size\0" as *const u8 as *const c_char) == 0 {
        (*p).i_slice_max_size = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"slice-max-mbs\0" as *const u8 as *const c_char) == 0 {
        (*p).i_slice_max_mbs = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"slice-min-mbs\0" as *const u8 as *const c_char) == 0 {
        (*p).i_slice_min_mbs = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"slices\0" as *const u8 as *const c_char) == 0 {
        (*p).i_slice_count = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"slices-max\0" as *const u8 as *const c_char) == 0 {
        (*p).i_slice_count_max = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"cabac\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_cabac = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"cabac-idc\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cabac_init_idc = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"interlaced\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_interlaced = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"tff\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_tff = atobool_internal(value, &mut b_error);
        (*p).b_interlaced = (*p).b_tff;
    } else if strcmp(name, b"bff\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_interlaced = atobool_internal(value, &mut b_error);
        (*p).b_tff = ((*p).b_interlaced == 0) as c_int;
    } else if strcmp(name, b"constrained-intra\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_constrained_intra = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"cqm\0" as *const u8 as *const c_char) == 0 {
        if !strstr(value, b"flat\0" as *const u8 as *const c_char).is_null() {
            (*p).i_cqm_preset = X264_CQM_FLAT;
        } else if !strstr(value, b"jvt\0" as *const u8 as *const c_char).is_null() {
            (*p).i_cqm_preset = X264_CQM_JVT;
        } else {
            (*p).psz_cqm_file = x264_param_strdup(p, value);
            if (*p).psz_cqm_file.is_null() {
                b_error = 1 as c_int;
                errortype = X264_PARAM_ALLOC_FAILED;
            }
        }
    } else if strcmp(name, b"cqmfile\0" as *const u8 as *const c_char) == 0 {
        (*p).psz_cqm_file = x264_param_strdup(p, value);
        if (*p).psz_cqm_file.is_null() {
            b_error = 1 as c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(name, b"cqm4\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4iy.as_mut_ptr(), 16 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_4py.as_mut_ptr(), 16 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_4ic.as_mut_ptr(), 16 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_4pc.as_mut_ptr(), 16 as c_int);
    } else if strcmp(name, b"cqm8\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_8iy.as_mut_ptr(), 64 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_8py.as_mut_ptr(), 64 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_8ic.as_mut_ptr(), 64 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_8pc.as_mut_ptr(), 64 as c_int);
    } else if strcmp(name, b"cqm4i\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4iy.as_mut_ptr(), 16 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_4ic.as_mut_ptr(), 16 as c_int);
    } else if strcmp(name, b"cqm4p\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4py.as_mut_ptr(), 16 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_4pc.as_mut_ptr(), 16 as c_int);
    } else if strcmp(name, b"cqm4iy\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4iy.as_mut_ptr(), 16 as c_int);
    } else if strcmp(name, b"cqm4ic\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4ic.as_mut_ptr(), 16 as c_int);
    } else if strcmp(name, b"cqm4py\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4py.as_mut_ptr(), 16 as c_int);
    } else if strcmp(name, b"cqm4pc\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4pc.as_mut_ptr(), 16 as c_int);
    } else if strcmp(name, b"cqm8i\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_8iy.as_mut_ptr(), 64 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_8ic.as_mut_ptr(), 64 as c_int);
    } else if strcmp(name, b"cqm8p\0" as *const u8 as *const c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_8py.as_mut_ptr(), 64 as c_int);
        b_error |= parse_cqm(value, (*p).cqm_8pc.as_mut_ptr(), 64 as c_int);
    } else if strcmp(name, b"log\0" as *const u8 as *const c_char) == 0 {
        (*p).i_log_level = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"dump-yuv\0" as *const u8 as *const c_char) == 0 {
        (*p).psz_dump_yuv = x264_param_strdup(p, value);
        if (*p).psz_dump_yuv.is_null() {
            b_error = 1 as c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(name, b"analyse\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"partitions\0" as *const u8 as *const c_char) == 0
    {
        (*p).analyse.inter = 0 as c_uint;
        if !strstr(value, b"none\0" as *const u8 as *const c_char).is_null() {
            (*p).analyse.inter = 0 as c_uint;
        }
        if !strstr(value, b"all\0" as *const u8 as *const c_char).is_null() {
            (*p).analyse.inter = !(0 as c_int) as c_uint;
        }
        if !strstr(value, b"i4x4\0" as *const u8 as *const c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_I4x4;
        }
        if !strstr(value, b"i8x8\0" as *const u8 as *const c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_I8x8;
        }
        if !strstr(value, b"p8x8\0" as *const u8 as *const c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_PSUB16x16;
        }
        if !strstr(value, b"p4x4\0" as *const u8 as *const c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_PSUB8x8;
        }
        if !strstr(value, b"b8x8\0" as *const u8 as *const c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_BSUB16x16;
        }
    } else if strcmp(name, b"8x8dct\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_transform_8x8 = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"weightb\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"weight-b\0" as *const u8 as *const c_char) == 0
    {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_weighted_bipred = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"weightp\0" as *const u8 as *const c_char) == 0 {
        (*p).analyse.i_weighted_pred = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"direct\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"direct-pred\0" as *const u8 as *const c_char) == 0
    {
        b_error |= parse_enum(
            value,
            x264_direct_pred_names.as_ptr(),
            &mut (*p).analyse.i_direct_mv_pred,
        );
    } else if strcmp(name, b"chroma-qp-offset\0" as *const u8 as *const c_char) == 0 {
        (*p).analyse.i_chroma_qp_offset = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"me\0" as *const u8 as *const c_char) == 0 {
        (*p).analyse.me_method = CStr::from_ptr(value)
            .to_str()
            .ok()
            .and_then(|s| MotionEstimation::from_str(s).ok())
            .unwrap_or_else(|| {
                b_error = 1;
                MotionEstimation::Dia
            });
    } else if strcmp(name, b"merange\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"me-range\0" as *const u8 as *const c_char) == 0
    {
        (*p).analyse.i_me_range = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"mvrange\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"mv-range\0" as *const u8 as *const c_char) == 0
    {
        (*p).analyse.i_mv_range = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"mvrange-thread\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"mv-range-thread\0" as *const u8 as *const c_char) == 0
    {
        (*p).analyse.i_mv_range_thread = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"subme\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"subq\0" as *const u8 as *const c_char) == 0
    {
        (*p).analyse.i_subpel_refine = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"psy-rd\0" as *const u8 as *const c_char) == 0 {
        if !(2 as c_int
            == sscanf(
                value,
                b"%f:%f\0" as *const u8 as *const c_char,
                &mut (*p).analyse.f_psy_rd as *mut c_float,
                &mut (*p).analyse.f_psy_trellis as *mut c_float,
            )
            || 2 as c_int
                == sscanf(
                    value,
                    b"%f,%f\0" as *const u8 as *const c_char,
                    &mut (*p).analyse.f_psy_rd as *mut c_float,
                    &mut (*p).analyse.f_psy_trellis as *mut c_float,
                )
            || 2 as c_int
                == sscanf(
                    value,
                    b"%f|%f\0" as *const u8 as *const c_char,
                    &mut (*p).analyse.f_psy_rd as *mut c_float,
                    &mut (*p).analyse.f_psy_trellis as *mut c_float,
                ))
        {
            if sscanf(
                value,
                b"%f\0" as *const u8 as *const c_char,
                &mut (*p).analyse.f_psy_rd as *mut c_float,
            ) != 0
            {
                (*p).analyse.f_psy_trellis = 0 as c_int as c_float;
            } else {
                (*p).analyse.f_psy_rd = 0 as c_int as c_float;
                (*p).analyse.f_psy_trellis = 0 as c_int as c_float;
            }
        }
    } else if strcmp(name, b"psy\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_psy = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"chroma-me\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_chroma_me = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"mixed-refs\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_mixed_references = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"trellis\0" as *const u8 as *const c_char) == 0 {
        (*p).analyse.i_trellis = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"fast-pskip\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_fast_pskip = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"dct-decimate\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_dct_decimate = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"deadzone-inter\0" as *const u8 as *const c_char) == 0 {
        (*p).analyse.i_luma_deadzone[0] = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"deadzone-intra\0" as *const u8 as *const c_char) == 0 {
        (*p).analyse.i_luma_deadzone[1] = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"nr\0" as *const u8 as *const c_char) == 0 {
        (*p).analyse.i_noise_reduction = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"bitrate\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.i_bitrate = atoi_internal(value, &mut b_error);
        (*p).rc.i_rc_method = X264_RC_ABR;
    } else if strcmp(name, b"qp\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"qp_constant\0" as *const u8 as *const c_char) == 0
    {
        (*p).rc.i_qp_constant = atoi_internal(value, &mut b_error);
        (*p).rc.i_rc_method = X264_RC_CQP;
    } else if strcmp(name, b"crf\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.f_rf_constant = atof_internal(value, &mut b_error) as c_float;
        (*p).rc.i_rc_method = X264_RC_CRF;
    } else if strcmp(name, b"crf-max\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.f_rf_constant_max = atof_internal(value, &mut b_error) as c_float;
    } else if strcmp(name, b"rc-lookahead\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.i_lookahead = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"qpmin\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"qp-min\0" as *const u8 as *const c_char) == 0
    {
        (*p).rc.i_qp_min = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"qpmax\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"qp-max\0" as *const u8 as *const c_char) == 0
    {
        (*p).rc.i_qp_max = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"qpstep\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"qp-step\0" as *const u8 as *const c_char) == 0
    {
        (*p).rc.i_qp_step = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"ratetol\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.f_rate_tolerance =
            (if strncmp(b"inf\0" as *const u8 as *const c_char, value, 3 as size_t) == 0 {
                1e9f64
            } else {
                atof_internal(value, &mut b_error)
            }) as c_float;
    } else if strcmp(name, b"vbv-maxrate\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.i_vbv_max_bitrate = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"vbv-bufsize\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.i_vbv_buffer_size = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"vbv-init\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.f_vbv_buffer_init = atof_internal(value, &mut b_error) as c_float;
    } else if strcmp(name, b"ipratio\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"ip-factor\0" as *const u8 as *const c_char) == 0
    {
        (*p).rc.f_ip_factor = atof_internal(value, &mut b_error) as c_float;
    } else if strcmp(name, b"pbratio\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"pb-factor\0" as *const u8 as *const c_char) == 0
    {
        (*p).rc.f_pb_factor = atof_internal(value, &mut b_error) as c_float;
    } else if strcmp(name, b"aq-mode\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.i_aq_mode = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"aq-strength\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.f_aq_strength = atof_internal(value, &mut b_error) as c_float;
    } else if strcmp(name, b"pass\0" as *const u8 as *const c_char) == 0 {
        let mut pass: c_int =
            x264_clip3(atoi_internal(value, &mut b_error), 0 as c_int, 3 as c_int);
        (*p).rc.b_stat_write = pass & 1 as c_int;
        (*p).rc.b_stat_read = pass & 2 as c_int;
    } else if strcmp(name, b"stats\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.psz_stat_in = x264_param_strdup(p, value);
        if (*p).rc.psz_stat_in.is_null() {
            b_error = 1 as c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
        (*p).rc.psz_stat_out = x264_param_strdup(p, value);
        if (*p).rc.psz_stat_out.is_null() {
            b_error = 1 as c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(name, b"qcomp\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.f_qcompress = atof_internal(value, &mut b_error) as c_float;
    } else if strcmp(name, b"mbtree\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).rc.b_mb_tree = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"qblur\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.f_qblur = atof_internal(value, &mut b_error) as c_float;
    } else if strcmp(name, b"cplxblur\0" as *const u8 as *const c_char) == 0
        || strcmp(name, b"cplx-blur\0" as *const u8 as *const c_char) == 0
    {
        (*p).rc.f_complexity_blur = atof_internal(value, &mut b_error) as c_float;
    } else if strcmp(name, b"zones\0" as *const u8 as *const c_char) == 0 {
        (*p).rc.psz_zones = x264_param_strdup(p, value);
        if (*p).rc.psz_zones.is_null() {
            b_error = 1 as c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(name, b"crop-rect\0" as *const u8 as *const c_char) == 0 {
        let mut crop_left: c_int = 0;
        let mut crop_top: c_int = 0;
        let mut crop_right: c_int = 0;
        let mut crop_bottom: c_int = 0;

        let num_values = sscanf(
            value,
            c"%d,%d,%d,%d".as_ptr(),
            &mut crop_left,
            &mut crop_top,
            &mut crop_right,
            &mut crop_bottom,
        );

        b_error |= (num_values != 4) as i32;

        if num_values == 4
            && let Ok(left) = u32::try_from(crop_left)
            && let Ok(top) = u32::try_from(crop_top)
            && let Ok(right) = u32::try_from(crop_right)
            && let Ok(bottom) = u32::try_from(crop_bottom)
        {
            (*p).crop_rect = CropRectangle {
                left,
                top,
                right,
                bottom,
            };
        }
    } else if strcmp(name, b"psnr\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_psnr = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"ssim\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).analyse.b_ssim = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"aud\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_aud = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"sps-id\0" as *const u8 as *const c_char) == 0 {
        (*p).i_sps_id = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"global-header\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_repeat_headers = (atobool_internal(value, &mut b_error) == 0) as c_int;
    } else if strcmp(name, b"repeat-headers\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_repeat_headers = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"annexb\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_annexb = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"force-cfr\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_vfr_input = (atobool_internal(value, &mut b_error) == 0) as c_int;
    } else if strcmp(name, b"nal-hrd\0" as *const u8 as *const c_char) == 0 {
        b_error |= parse_enum(value, x264_nal_hrd_names.as_ptr(), &mut (*p).i_nal_hrd);
    } else if strcmp(name, b"filler\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).rc.b_filler = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"pic-struct\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_pic_struct = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"fake-interlaced\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_fake_interlaced = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"frame-packing\0" as *const u8 as *const c_char) == 0 {
        (*p).frame_packing = FramePacking::from_i32(atoi_internal(value, &mut b_error));
    } else if strcmp(name, b"stitchable\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_stitchable = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"opencl\0" as *const u8 as *const c_char) == 0 {
        name_was_bool = 1 as c_int;
        (*p).b_opencl = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"opencl-clbin\0" as *const u8 as *const c_char) == 0 {
        (*p).psz_clbin_file = x264_param_strdup(p, value);
        if (*p).psz_clbin_file.is_null() {
            b_error = 1 as c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(name, b"opencl-device\0" as *const u8 as *const c_char) == 0 {
        (*p).i_opencl_device = atoi_internal(value, &mut b_error);
    } else {
        b_error = 1 as c_int;
        errortype = X264_PARAM_BAD_NAME;
    }
    if !name_buf.is_null() {
        free(name_buf as *mut c_void);
    }
    b_error |= (value_was_null != 0 && name_was_bool == 0) as c_int;
    return if b_error != 0 { errortype } else { 0 as c_int };
}
#[no_mangle]
#[c2rust::src_loc = "1428:1"]
unsafe extern "C" fn x264_param2string(mut p: *mut x264_param_t, mut b_res: c_int) -> *mut c_char {
    let mut len: c_int = 2000 as c_int;
    let mut buf: *mut c_char = 0 as *mut c_char;
    let mut s: *mut c_char = 0 as *mut c_char;
    if !(*p).rc.psz_zones.is_null() {
        len = (len as size_t).wrapping_add(strlen((*p).rc.psz_zones)) as c_int as c_int;
    }
    s = x264_malloc(len as int64_t) as *mut c_char;
    buf = s;
    if buf.is_null() {
        return 0 as *mut c_char;
    }
    if b_res != 0 {
        s = s.offset(sprintf(
            s,
            b"%dx%d \0" as *const u8 as *const c_char,
            (*p).width,
            (*p).height,
        ) as isize);
        s = s.offset(sprintf(
            s,
            b"fps=%u/%u \0" as *const u8 as *const c_char,
            (*p).i_fps_num,
            (*p).i_fps_den,
        ) as isize);
        s = s.offset(sprintf(
            s,
            b"timebase=%u/%u \0" as *const u8 as *const c_char,
            (*p).i_timebase_num,
            (*p).i_timebase_den,
        ) as isize);
        s = s.offset(sprintf(
            s,
            b"bitdepth=%d \0" as *const u8 as *const c_char,
            (*p).i_bitdepth,
        ) as isize);
    }
    if (*p).b_opencl != 0 {
        s = s.offset(sprintf(
            s,
            b"opencl=%d \0" as *const u8 as *const c_char,
            (*p).b_opencl,
        ) as isize);
    }
    s = s.offset(sprintf(s, b"cabac=%d\0" as *const u8 as *const c_char, (*p).b_cabac) as isize);
    s = s.offset(sprintf(
        s,
        b" ref=%d\0" as *const u8 as *const c_char,
        (*p).i_frame_reference,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" deblock=%d:%d:%d\0" as *const u8 as *const c_char,
        (*p).b_deblocking_filter,
        (*p).i_deblocking_filter_alphac0,
        (*p).i_deblocking_filter_beta,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" analyse=%#x:%#x\0" as *const u8 as *const c_char,
        (*p).analyse.intra,
        (*p).analyse.inter,
    ) as isize);
    s = s.offset(sprintf(s, c" me=%s".as_ptr(), (*p).analyse.me_method.as_ref()) as isize);
    s = s.offset(sprintf(
        s,
        b" subme=%d\0" as *const u8 as *const c_char,
        (*p).analyse.i_subpel_refine,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" psy=%d\0" as *const u8 as *const c_char,
        (*p).analyse.b_psy,
    ) as isize);
    if (*p).analyse.b_psy != 0 {
        s = s.offset(sprintf(
            s,
            b" psy_rd=%.2f:%.2f\0" as *const u8 as *const c_char,
            (*p).analyse.f_psy_rd as c_double,
            (*p).analyse.f_psy_trellis as c_double,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" mixed_ref=%d\0" as *const u8 as *const c_char,
        (*p).analyse.b_mixed_references,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" me_range=%d\0" as *const u8 as *const c_char,
        (*p).analyse.i_me_range,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" chroma_me=%d\0" as *const u8 as *const c_char,
        (*p).analyse.b_chroma_me,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" trellis=%d\0" as *const u8 as *const c_char,
        (*p).analyse.i_trellis,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" 8x8dct=%d\0" as *const u8 as *const c_char,
        (*p).analyse.b_transform_8x8,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" cqm=%d\0" as *const u8 as *const c_char,
        (*p).i_cqm_preset,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" deadzone=%d,%d\0" as *const u8 as *const c_char,
        (*p).analyse.i_luma_deadzone[0],
        (*p).analyse.i_luma_deadzone[1],
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" fast_pskip=%d\0" as *const u8 as *const c_char,
        (*p).analyse.b_fast_pskip,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" chroma_qp_offset=%d\0" as *const u8 as *const c_char,
        (*p).analyse.i_chroma_qp_offset,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" threads=%d\0" as *const u8 as *const c_char,
        (*p).i_threads,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" lookahead_threads=%d\0" as *const u8 as *const c_char,
        (*p).i_lookahead_threads,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" sliced_threads=%d\0" as *const u8 as *const c_char,
        (*p).b_sliced_threads,
    ) as isize);
    if (*p).i_slice_count != 0 {
        s = s.offset(sprintf(
            s,
            b" slices=%d\0" as *const u8 as *const c_char,
            (*p).i_slice_count,
        ) as isize);
    }
    if (*p).i_slice_count_max != 0 {
        s = s.offset(sprintf(
            s,
            b" slices_max=%d\0" as *const u8 as *const c_char,
            (*p).i_slice_count_max,
        ) as isize);
    }
    if (*p).i_slice_max_size != 0 {
        s = s.offset(sprintf(
            s,
            b" slice_max_size=%d\0" as *const u8 as *const c_char,
            (*p).i_slice_max_size,
        ) as isize);
    }
    if (*p).i_slice_max_mbs != 0 {
        s = s.offset(sprintf(
            s,
            b" slice_max_mbs=%d\0" as *const u8 as *const c_char,
            (*p).i_slice_max_mbs,
        ) as isize);
    }
    if (*p).i_slice_min_mbs != 0 {
        s = s.offset(sprintf(
            s,
            b" slice_min_mbs=%d\0" as *const u8 as *const c_char,
            (*p).i_slice_min_mbs,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" nr=%d\0" as *const u8 as *const c_char,
        (*p).analyse.i_noise_reduction,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" decimate=%d\0" as *const u8 as *const c_char,
        (*p).analyse.b_dct_decimate,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" interlaced=%s\0" as *const u8 as *const c_char,
        if (*p).b_interlaced != 0 {
            if (*p).b_tff != 0 {
                b"tff\0" as *const u8 as *const c_char
            } else {
                b"bff\0" as *const u8 as *const c_char
            }
        } else if (*p).b_fake_interlaced != 0 {
            b"fake\0" as *const u8 as *const c_char
        } else {
            b"0\0" as *const u8 as *const c_char
        },
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" bluray_compat=%d\0" as *const u8 as *const c_char,
        (*p).b_bluray_compat,
    ) as isize);
    if (*p).b_stitchable != 0 {
        s = s.offset(sprintf(
            s,
            b" stitchable=%d\0" as *const u8 as *const c_char,
            (*p).b_stitchable,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" constrained_intra=%d\0" as *const u8 as *const c_char,
        (*p).b_constrained_intra,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" bframes=%d\0" as *const u8 as *const c_char,
        (*p).i_bframe,
    ) as isize);
    if (*p).i_bframe != 0 {
        s = s.offset(sprintf(
            s,
            b" b_pyramid=%d b_adapt=%d b_bias=%d direct=%d weightb=%d open_gop=%d\0" as *const u8
                as *const c_char,
            (*p).bframe_pyramid as i32,
            (*p).i_bframe_adaptive,
            (*p).i_bframe_bias,
            (*p).analyse.i_direct_mv_pred,
            (*p).analyse.b_weighted_bipred,
            (*p).b_open_gop,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" weightp=%d\0" as *const u8 as *const c_char,
        if (*p).analyse.i_weighted_pred > 0 as c_int {
            (*p).analyse.i_weighted_pred
        } else {
            0 as c_int
        },
    ) as isize);
    if (*p).i_keyint_max == X264_KEYINT_MAX_INFINITE {
        s = s.offset(sprintf(s, b" keyint=infinite\0" as *const u8 as *const c_char) as isize);
    } else {
        s = s.offset(sprintf(
            s,
            b" keyint=%d\0" as *const u8 as *const c_char,
            (*p).i_keyint_max,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" keyint_min=%d scenecut=%d intra_refresh=%d\0" as *const u8 as *const c_char,
        (*p).i_keyint_min,
        (*p).i_scenecut_threshold,
        (*p).b_intra_refresh,
    ) as isize);
    if (*p).rc.b_mb_tree != 0 || (*p).rc.i_vbv_buffer_size != 0 {
        s = s.offset(sprintf(
            s,
            b" rc_lookahead=%d\0" as *const u8 as *const c_char,
            (*p).rc.i_lookahead,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" rc=%s mbtree=%d\0" as *const u8 as *const c_char,
        if (*p).rc.i_rc_method == X264_RC_ABR {
            if (*p).rc.b_stat_read != 0 {
                b"2pass\0" as *const u8 as *const c_char
            } else if (*p).rc.i_vbv_max_bitrate == (*p).rc.i_bitrate {
                b"cbr\0" as *const u8 as *const c_char
            } else {
                b"abr\0" as *const u8 as *const c_char
            }
        } else if (*p).rc.i_rc_method == X264_RC_CRF {
            b"crf\0" as *const u8 as *const c_char
        } else {
            b"cqp\0" as *const u8 as *const c_char
        },
        (*p).rc.b_mb_tree,
    ) as isize);
    if (*p).rc.i_rc_method == X264_RC_ABR || (*p).rc.i_rc_method == X264_RC_CRF {
        if (*p).rc.i_rc_method == X264_RC_CRF {
            s = s.offset(sprintf(
                s,
                b" crf=%.1f\0" as *const u8 as *const c_char,
                (*p).rc.f_rf_constant as c_double,
            ) as isize);
        } else {
            s = s.offset(sprintf(
                s,
                b" bitrate=%d ratetol=%.1f\0" as *const u8 as *const c_char,
                (*p).rc.i_bitrate,
                (*p).rc.f_rate_tolerance as c_double,
            ) as isize);
        }
        s = s.offset(sprintf(
            s,
            b" qcomp=%.2f qpmin=%d qpmax=%d qpstep=%d\0" as *const u8 as *const c_char,
            (*p).rc.f_qcompress as c_double,
            (*p).rc.i_qp_min,
            (*p).rc.i_qp_max,
            (*p).rc.i_qp_step,
        ) as isize);
        if (*p).rc.b_stat_read != 0 {
            s = s.offset(sprintf(
                s,
                b" cplxblur=%.1f qblur=%.1f\0" as *const u8 as *const c_char,
                (*p).rc.f_complexity_blur as c_double,
                (*p).rc.f_qblur as c_double,
            ) as isize);
        }
        if (*p).rc.i_vbv_buffer_size != 0 {
            s = s.offset(sprintf(
                s,
                b" vbv_maxrate=%d vbv_bufsize=%d\0" as *const u8 as *const c_char,
                (*p).rc.i_vbv_max_bitrate,
                (*p).rc.i_vbv_buffer_size,
            ) as isize);
            if (*p).rc.i_rc_method == X264_RC_CRF {
                s = s.offset(sprintf(
                    s,
                    b" crf_max=%.1f\0" as *const u8 as *const c_char,
                    (*p).rc.f_rf_constant_max as c_double,
                ) as isize);
            }
        }
    } else if (*p).rc.i_rc_method == X264_RC_CQP {
        s = s.offset(sprintf(
            s,
            b" qp=%d\0" as *const u8 as *const c_char,
            (*p).rc.i_qp_constant,
        ) as isize);
    }
    if (*p).rc.i_vbv_buffer_size != 0 {
        s = s.offset(sprintf(
            s,
            b" nal_hrd=%s filler=%d\0" as *const u8 as *const c_char,
            x264_nal_hrd_names[(*p).i_nal_hrd as usize],
            (*p).rc.b_filler,
        ) as isize);
    }
    if (*p).crop_rect.left | (*p).crop_rect.top | (*p).crop_rect.right | (*p).crop_rect.bottom != 0
    {
        s = s.offset(sprintf(
            s,
            b" crop_rect=%d,%d,%d,%d\0" as *const u8 as *const c_char,
            (*p).crop_rect.left,
            (*p).crop_rect.top,
            (*p).crop_rect.right,
            (*p).crop_rect.bottom,
        ) as isize);
    }
    if let Some(mastering_display) = (*p).mastering_display {
        s = s.offset(sprintf(
            s,
            c" mastering-display=G(%d,%d)B(%d,%d)R(%d,%d)WP(%d,%d)L(%ld,%ld)".as_ptr(),
            mastering_display.green.0 as i32,
            mastering_display.green.1 as i32,
            mastering_display.blue.0 as i32,
            mastering_display.blue.1 as i32,
            mastering_display.red.0 as i32,
            mastering_display.red.1 as i32,
            mastering_display.white.0 as i32,
            mastering_display.white.1 as i32,
            mastering_display.display_max as i64,
            mastering_display.display_min as i64,
        ) as isize);
    }
    if let Some(light_level) = (*p).content_light_level {
        s = s.offset(sprintf(
            s,
            c" cll=%d,%d".as_ptr(),
            light_level.max_cll as i32,
            light_level.max_fall as i32,
        ) as isize);
    }
    if let Some(frame_packing) = (*p).frame_packing {
        s = s.offset(sprintf(s, c" frame-packing=%d".as_ptr(), frame_packing as i32) as isize);
    }
    if !((*p).rc.i_rc_method == X264_RC_CQP && (*p).rc.i_qp_constant == 0 as c_int) {
        s = s.offset(sprintf(
            s,
            b" ip_ratio=%.2f\0" as *const u8 as *const c_char,
            (*p).rc.f_ip_factor as c_double,
        ) as isize);
        if (*p).i_bframe != 0 && (*p).rc.b_mb_tree == 0 {
            s = s.offset(sprintf(
                s,
                b" pb_ratio=%.2f\0" as *const u8 as *const c_char,
                (*p).rc.f_pb_factor as c_double,
            ) as isize);
        }
        s = s.offset(sprintf(
            s,
            b" aq=%d\0" as *const u8 as *const c_char,
            (*p).rc.i_aq_mode,
        ) as isize);
        if (*p).rc.i_aq_mode != 0 {
            s = s.offset(sprintf(
                s,
                b":%.2f\0" as *const u8 as *const c_char,
                (*p).rc.f_aq_strength as c_double,
            ) as isize);
        }
        if !(*p).rc.psz_zones.is_null() {
            s = s.offset(sprintf(
                s,
                b" zones=%s\0" as *const u8 as *const c_char,
                (*p).rc.psz_zones,
            ) as isize);
        } else if (*p).rc.i_zones != 0 {
            s = s.offset(sprintf(s, b" zones\0" as *const u8 as *const c_char) as isize);
        }
    }
    return buf;
}
