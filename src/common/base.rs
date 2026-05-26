// =============== BEGIN base_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union16_t {
    pub i: crate::stdlib::uint16_t,
    pub b: [crate::stdlib::uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union32_t {
    pub i: crate::stdlib::uint32_t,
    pub w: [crate::stdlib::uint16_t; 2],
    pub b: [crate::stdlib::uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union64_t {
    pub i: crate::stdlib::uint64_t,
    pub d: [crate::stdlib::uint32_t; 2],
    pub w: [crate::stdlib::uint16_t; 4],
    pub b: [crate::stdlib::uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_uint128_t {
    pub i: [crate::stdlib::uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union128_t {
    pub i: crate::src::common::base::x264_uint128_t,
    pub q: [crate::stdlib::uint64_t; 2],
    pub d: [crate::stdlib::uint32_t; 4],
    pub w: [crate::stdlib::uint16_t; 8],
    pub b: [crate::stdlib::uint8_t; 16],
}
pub const M128_ZERO: crate::src::common::base::x264_uint128_t =
    crate::src::common::base::x264_uint128_t { i: [0u64, 0u64] };
pub type profile_e = ::core::ffi::c_uint;
pub const PROFILE_BASELINE: crate::src::common::base::profile_e = 66;
pub const PROFILE_MAIN: crate::src::common::base::profile_e = 77;
pub const PROFILE_HIGH: crate::src::common::base::profile_e = 100;
pub const PROFILE_HIGH10: crate::src::common::base::profile_e = 110;
pub const PROFILE_HIGH422: crate::src::common::base::profile_e = 122;
pub const PROFILE_HIGH444_PREDICTIVE: crate::src::common::base::profile_e = 244;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum ChromaFormat {
    Chroma400 = 0,
    Chroma420 = 1,
    Chroma422 = 2,
    Chroma444 = 3,
}
impl ChromaFormat {
    pub fn is_444(self) -> bool {
        self == ChromaFormat::Chroma444
    }
    pub fn is_422(self) -> bool {
        self == ChromaFormat::Chroma422
    }
    pub fn is_420(self) -> bool {
        self == ChromaFormat::Chroma420
    }
    pub fn is_400(self) -> bool {
        self == ChromaFormat::Chroma400
    }
}
pub type slice_type_e = ::core::ffi::c_uint;
pub const SLICE_TYPE_P: crate::src::common::base::slice_type_e = 0;
pub const SLICE_TYPE_B: crate::src::common::base::slice_type_e = 1;
pub const SLICE_TYPE_I: crate::src::common::base::slice_type_e = 2;
pub type sei_payload_type_e = ::core::ffi::c_uint;
pub const SEI_BUFFERING_PERIOD: crate::src::common::base::sei_payload_type_e = 0;
pub const SEI_PIC_TIMING: crate::src::common::base::sei_payload_type_e = 1;
pub const SEI_PAN_SCAN_RECT: crate::src::common::base::sei_payload_type_e = 2;
pub const SEI_FILLER: crate::src::common::base::sei_payload_type_e = 3;
pub const SEI_USER_DATA_REGISTERED: crate::src::common::base::sei_payload_type_e = 4;
pub const SEI_USER_DATA_UNREGISTERED: crate::src::common::base::sei_payload_type_e = 5;
pub const SEI_RECOVERY_POINT: crate::src::common::base::sei_payload_type_e = 6;
pub const SEI_DEC_REF_PIC_MARKING: crate::src::common::base::sei_payload_type_e = 7;
pub const SEI_FRAME_PACKING: crate::src::common::base::sei_payload_type_e = 45;
pub const SEI_MASTERING_DISPLAY: crate::src::common::base::sei_payload_type_e = 137;
pub const SEI_CONTENT_LIGHT_LEVEL: crate::src::common::base::sei_payload_type_e = 144;
pub const SEI_ALTERNATIVE_TRANSFER: crate::src::common::base::sei_payload_type_e = 147;
pub const X264_BFRAME_MAX: ::core::ffi::c_int = 16i32;
pub const X264_REF_MAX: ::core::ffi::c_int = 16i32;
pub const X264_THREAD_MAX: ::core::ffi::c_int = 128i32;
pub const X264_LOOKAHEAD_MAX: ::core::ffi::c_int = 250i32;
pub const X264_THREAD_HEIGHT: ::core::ffi::c_int = 24i32;
pub const X264_WEIGHTP_FAKE: ::core::ffi::c_int = -(1i32);
pub const X264_SCAN8_0: ::core::ffi::c_int = 4i32 + 1i32 * 8i32;
pub const LUMA_DC: ::core::ffi::c_int = 48i32;
pub const CHROMA_DC: ::core::ffi::c_int = 49i32;
pub mod x264_h {
    pub const X264_DIRECT_PRED_NAMES: &[&str; 4] = &["none", "spatial", "temporal", "auto"];
    pub const X264_MOTION_EST_NAMES: &[&str; 5] = &["dia", "hex", "umh", "esa", "tesa"];
    pub const X264_B_PYRAMID_NAMES: &[&str; 3] = &["none", "strict", "normal"];
    pub const X264_OVERSCAN_NAMES: &[&str; 3] = &["undef", "show", "crop"];
    pub const X264_VIDFORMAT_NAMES: &[&str; 6] = &["component", "pal", "ntsc", "secam", "mac", "undef"];
    pub const X264_FULLRANGE_NAMES: &[&str; 2] = &["off", "on"];
    pub const X264_COLORPRIM_NAMES: &[&str; 13] = &[
        "",
        "bt709",
        "undef",
        "",
        "bt470m",
        "bt470bg",
        "smpte170m",
        "smpte240m",
        "film",
        "bt2020",
        "smpte428",
        "smpte431",
        "smpte432",
    ];
    pub const X264_TRANSFER_NAMES: &[&str; 19] = &[
        "",
        "bt709",
        "undef",
        "",
        "bt470m",
        "bt470bg",
        "smpte170m",
        "smpte240m",
        "linear",
        "log100",
        "log316",
        "iec61966-2-4",
        "bt1361e",
        "iec61966-2-1",
        "bt2020-10",
        "bt2020-12",
        "smpte2084",
        "smpte428",
        "arib-std-b67",
    ];
    pub const X264_COLMATRIX_NAMES: &[&str; 15] = &[
        "GBR",
        "bt709",
        "undef",
        "",
        "fcc",
        "bt470bg",
        "smpte170m",
        "smpte240m",
        "YCgCo",
        "bt2020nc",
        "bt2020c",
        "smpte2085",
        "chroma-derived-nc",
        "chroma-derived-c",
        "ICtCp",
    ];
    pub const X264_NAL_HRD_NAMES: &[&str; 3] = &["none", "vbr", "cbr"];
    pub const X264_AVCINTRA_FLAVOR_NAMES: &[&str; 2] = &["panasonic", "sony"];
    pub const X264_PRESET_NAMES: &[&str; 10] = &[
        "ultrafast",
        "superfast",
        "veryfast",
        "faster",
        "fast",
        "medium",
        "slow",
        "slower",
        "veryslow",
        "placebo",
    ];
}
pub mod base_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        if v < i_min {
            i_min
        } else if v > i_max {
            i_max
        } else {
            v
        }
    }
}
use std::ffi::CString;

use crate::{
    src::common::{
        base::x264_h::{
            X264_AVCINTRA_FLAVOR_NAMES, X264_B_PYRAMID_NAMES, X264_COLMATRIX_NAMES, X264_COLORPRIM_NAMES,
            X264_DIRECT_PRED_NAMES, X264_FULLRANGE_NAMES, X264_MOTION_EST_NAMES, X264_NAL_HRD_NAMES,
            X264_OVERSCAN_NAMES, X264_PRESET_NAMES, X264_TRANSFER_NAMES, X264_VIDFORMAT_NAMES,
        },
        cpu::{X264_CPU_NAMES, x264_cpu_detect},
    },
    x264_h::{
        MasteringDisplay, X264_ANALYSE_BSUB16x16, X264_ANALYSE_I4x4, X264_ANALYSE_I8x8, X264_ANALYSE_PSUB8x8,
        X264_ANALYSE_PSUB16x16, X264_B_ADAPT_DEFAULT, X264_CPU_SSE2_IS_FAST, X264_CPU_SSE2_IS_SLOW,
        X264_CPU_SSSE3, X264_CQM_CUSTOM, X264_CQM_FLAT, X264_CQM_JVT, X264_KEYINT_MAX_INFINITE,
        X264_PARAM_ALLOC_FAILED, X264_PARAM_BAD_NAME, X264_PARAM_BAD_VALUE, X264_RC_ABR, X264_RC_CQP,
        X264_RC_CRF, X264_SCENECUT_THRESHOLD_DEFAULT, X264_SYNC_LOOKAHEAD_AUTO, X264_THREADS_AUTO,
        x264_param_t,
    },
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strdup_buffer {
    pub size: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub ptr: [*mut ::core::ffi::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_csp_tab_t {
    pub planes: ::core::ffi::c_int,
    pub width_fix8: [::core::ffi::c_int; 3],
    pub height_fix8: [::core::ffi::c_int; 3],
}
pub unsafe extern "C" fn x264_reduce_fraction(
    mut n: *mut crate::stdlib::uint32_t,
    mut d: *mut crate::stdlib::uint32_t,
) {
    unsafe {
        let mut a = *n;
        let mut b = *d;
        if a == 0 || b == 0 {
            return;
        }
        let mut c = a.wrapping_rem(b);
        while c != 0 {
            a = b;
            b = c;
            c = a.wrapping_rem(b);
        }
        *n = (*n).wrapping_div(b);
        *d = (*d).wrapping_div(b);
    }
}
pub unsafe extern "C" fn x264_reduce_fraction64(
    mut n: *mut crate::stdlib::uint64_t,
    mut d: *mut crate::stdlib::uint64_t,
) {
    unsafe {
        let mut a = *n;
        let mut b = *d;
        if a == 0 || b == 0 {
            return;
        }
        let mut c = a.wrapping_rem(b);
        while c != 0 {
            a = b;
            b = c;
            c = a.wrapping_rem(b);
        }
        *n = (*n).wrapping_div(b);
        *d = (*d).wrapping_div(b);
    }
}
pub unsafe extern "C" fn x264_malloc(mut i_size: crate::stdlib::int64_t) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut align_buf = ::core::ptr::null_mut::<crate::stdlib::uint8_t>();
        if i_size < 0i64
            || i_size as crate::stdlib::uint64_t
                > (u64::MAX).wrapping_sub(HUGE_PAGE_SIZE as crate::stdlib::uint64_t)
        {
            log::error!("invalid size of malloc: {i_size}");
            return crate::__stddef_null_h::NULL;
        }
        if i_size >= (HUGE_PAGE_SIZE * 7i32 / 8i32) as crate::stdlib::int64_t {
            align_buf = crate::stdlib::memalign(
                HUGE_PAGE_SIZE as crate::__stddef_size_t_h::size_t,
                i_size as crate::__stddef_size_t_h::size_t,
            ) as *mut crate::stdlib::uint8_t;
            if !align_buf.is_null() {
                let mut madv_size = ((i_size + HUGE_PAGE_SIZE as crate::stdlib::int64_t
                    - (HUGE_PAGE_SIZE * 7i32 / 8i32) as crate::stdlib::int64_t)
                    & !(HUGE_PAGE_SIZE - 1i32) as crate::stdlib::int64_t)
                    as crate::__stddef_size_t_h::size_t;
                crate::stdlib::madvise(
                    align_buf as *mut ::core::ffi::c_void,
                    madv_size,
                    crate::stdlib::MADV_HUGEPAGE,
                );
            }
        } else {
            align_buf = crate::stdlib::memalign(
                crate::osdep_h::NATIVE_ALIGN as crate::__stddef_size_t_h::size_t,
                i_size as crate::__stddef_size_t_h::size_t,
            ) as *mut crate::stdlib::uint8_t;
        }
        if align_buf.is_null() {
            log::error!("malloc of size {i_size} failed");
        }
        align_buf as *mut ::core::ffi::c_void
    }
}
pub const HUGE_PAGE_SIZE: ::core::ffi::c_int = 2i32 * 1024i32 * 1024i32;
pub unsafe extern "C" fn x264_free(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        if !p.is_null() {
            crate::stdlib::free(p);
        }
    }
}
pub unsafe extern "C" fn x264_slurp_file(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut b_error = 0i32;
        let mut fh = crate::stdlib::fopen(filename, c"rb".as_ptr());
        if fh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        b_error |= (crate::stdlib::fseeko(fh, 0i64, crate::stdlib::SEEK_END) < 0i32) as ::core::ffi::c_int;
        let mut i_size = crate::stdlib::ftello(fh);
        b_error |= (i_size <= 0i64) as ::core::ffi::c_int;
        if crate::osdep_h::WORD_SIZE == 4i32 {
            b_error |= (i_size > i32::MAX as crate::stdlib::int64_t) as ::core::ffi::c_int;
        }
        b_error |= (crate::stdlib::fseeko(fh, 0i64, crate::stdlib::SEEK_SET) < 0i32) as ::core::ffi::c_int;
        if b_error == 0 {
            let mut buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
            buf = x264_malloc(i_size + 2i64) as *mut ::core::ffi::c_char;
            if !buf.is_null() {
                b_error |= (crate::stdlib::fread(
                    buf as *mut ::core::ffi::c_void,
                    1usize,
                    i_size as crate::__stddef_size_t_h::size_t,
                    fh,
                ) != i_size as crate::stdlib::uint64_t) as ::core::ffi::c_int;
                crate::stdlib::fclose(fh);
                if b_error != 0 {
                    x264_free(buf as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if *buf.offset((i_size - 1i64) as isize) as ::core::ffi::c_int != '\n' as i32 {
                    let c2rust_fresh11 = i_size;
                    i_size += 1;
                    *buf.offset(c2rust_fresh11 as isize) = '\n' as ::core::ffi::c_char;
                }
                *buf.offset(i_size as isize) = '\0' as ::core::ffi::c_char;
                return buf;
            }
        }
        crate::stdlib::fclose(fh);
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }
}
pub const BUFFER_DEFAULT_SIZE: ::core::ffi::c_int = 16i32;
pub fn x264_param_strdup(
    mut param: *mut x264_param_t,
    mut src: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut buf = (*param).opaque as *mut strdup_buffer;
        if buf.is_null() {
            buf = crate::stdlib::malloc(
                (8usize).wrapping_add(
                    (BUFFER_DEFAULT_SIZE as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>()),
                ),
            ) as *mut strdup_buffer;
            if buf.is_null() {
                c2rust_current_block = 5854223930975509075;
            } else {
                (*buf).size = BUFFER_DEFAULT_SIZE;
                (*buf).count = 0i32;
                (*param).opaque = buf as *mut ::core::ffi::c_void;
                c2rust_current_block = 11650488183268122163;
            }
        } else if (*buf).count == (*buf).size {
            if (*buf).size
                > (crate::limits_h::INT_MAX - 8i32)
                    / 2i32
                    / ::core::mem::size_of::<*mut ::core::ffi::c_void>() as ::core::ffi::c_int
            {
                c2rust_current_block = 5854223930975509075;
            } else {
                let mut new_size = (*buf).size * 2i32;
                buf = crate::stdlib::realloc(
                    buf as *mut ::core::ffi::c_void,
                    (8usize).wrapping_add(
                        (new_size as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>()),
                    ),
                ) as *mut strdup_buffer;
                if buf.is_null() {
                    c2rust_current_block = 5854223930975509075;
                } else {
                    (*buf).size = new_size;
                    (*param).opaque = buf as *mut ::core::ffi::c_void;
                    c2rust_current_block = 11650488183268122163;
                }
            }
        } else {
            c2rust_current_block = 11650488183268122163;
        }
        match c2rust_current_block {
            11650488183268122163 => {
                let mut res = ::core::ptr::null_mut::<::core::ffi::c_char>();
                res = crate::stdlib::strdup(src);
                if !res.is_null() {
                    let c2rust_fresh0 = (*buf).count;
                    (*buf).count += 1;
                    let ref mut c2rust_fresh1 = *(&raw mut (*buf).ptr as *mut *mut ::core::ffi::c_void)
                        .offset(c2rust_fresh0 as isize);
                    *c2rust_fresh1 = res as *mut ::core::ffi::c_void;
                    return res;
                }
            }
            _ => {}
        }
        log::error!("x264_param_strdup failed");
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }
}
pub unsafe extern "C" fn x264_param_cleanup(mut param: *mut x264_param_t) {
    unsafe {
        let mut buf = (*param).opaque as *mut strdup_buffer;
        if !buf.is_null() {
            let mut i = 0i32;
            while i < (*buf).count {
                crate::stdlib::free(
                    *(&raw mut (*buf).ptr as *mut *mut ::core::ffi::c_void).offset(i as isize),
                );
                i += 1;
            }
            crate::stdlib::free(buf as *mut ::core::ffi::c_void);
            (*param).opaque = crate::__stddef_null_h::NULL;
        }
    }
}
pub unsafe extern "C" fn x264_picture_init(mut pic: *mut crate::x264_h::x264_picture_t) {
    unsafe {
        crate::stdlib::memset(
            pic as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<crate::x264_h::x264_picture_t>(),
        );
        (*pic).i_type = crate::x264_h::X264_TYPE_AUTO;
        (*pic).i_qpplus1 = crate::x264_h::X264_QP_AUTO;
        (*pic).i_pic_struct = crate::x264_h::PIC_STRUCT_AUTO as ::core::ffi::c_int;
    }
}
pub unsafe extern "C" fn x264_picture_alloc(
    mut pic: *mut crate::x264_h::x264_picture_t,
    mut i_csp: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut plane_offset = [0i64, 0, 0];
        let mut frame_size = 0i64;
        let mut i = 0i32;
        let mut i_0 = 1i32;
        static mut csp_tab: [x264_csp_tab_t; 17] = [
            x264_csp_tab_t { planes: 0, width_fix8: [0; 3], height_fix8: [0; 3] },
            x264_csp_tab_t {
                planes: 1i32,
                width_fix8: [256i32 * 1i32, 0, 0],
                height_fix8: [256i32 * 1i32, 0, 0],
            },
            x264_csp_tab_t {
                planes: 3i32,
                width_fix8: [256i32 * 1i32, 256i32 / 2i32, 256i32 / 2i32],
                height_fix8: [256i32 * 1i32, 256i32 / 2i32, 256i32 / 2i32],
            },
            x264_csp_tab_t {
                planes: 3i32,
                width_fix8: [256i32 * 1i32, 256i32 / 2i32, 256i32 / 2i32],
                height_fix8: [256i32 * 1i32, 256i32 / 2i32, 256i32 / 2i32],
            },
            x264_csp_tab_t {
                planes: 2i32,
                width_fix8: [256i32 * 1i32, 256i32 * 1i32, 0],
                height_fix8: [256i32 * 1i32, 256i32 / 2i32, 0],
            },
            x264_csp_tab_t {
                planes: 2i32,
                width_fix8: [256i32 * 1i32, 256i32 * 1i32, 0],
                height_fix8: [256i32 * 1i32, 256i32 / 2i32, 0],
            },
            x264_csp_tab_t {
                planes: 3i32,
                width_fix8: [256i32 * 1i32, 256i32 / 2i32, 256i32 / 2i32],
                height_fix8: [256i32 * 1i32, 256i32 * 1i32, 256i32 * 1i32],
            },
            x264_csp_tab_t {
                planes: 3i32,
                width_fix8: [256i32 * 1i32, 256i32 / 2i32, 256i32 / 2i32],
                height_fix8: [256i32 * 1i32, 256i32 * 1i32, 256i32 * 1i32],
            },
            x264_csp_tab_t {
                planes: 2i32,
                width_fix8: [256i32 * 1i32, 256i32 * 1i32, 0],
                height_fix8: [256i32 * 1i32, 256i32 * 1i32, 0],
            },
            x264_csp_tab_t {
                planes: 1i32,
                width_fix8: [256i32 * 2i32, 0, 0],
                height_fix8: [256i32 * 1i32, 0, 0],
            },
            x264_csp_tab_t {
                planes: 1i32,
                width_fix8: [256i32 * 2i32, 0, 0],
                height_fix8: [256i32 * 1i32, 0, 0],
            },
            x264_csp_tab_t { planes: 0, width_fix8: [0; 3], height_fix8: [0; 3] },
            x264_csp_tab_t {
                planes: 3i32,
                width_fix8: [256i32 * 1i32, 256i32 * 1i32, 256i32 * 1i32],
                height_fix8: [256i32 * 1i32, 256i32 * 1i32, 256i32 * 1i32],
            },
            x264_csp_tab_t {
                planes: 3i32,
                width_fix8: [256i32 * 1i32, 256i32 * 1i32, 256i32 * 1i32],
                height_fix8: [256i32 * 1i32, 256i32 * 1i32, 256i32 * 1i32],
            },
            x264_csp_tab_t {
                planes: 1i32,
                width_fix8: [256i32 * 3i32, 0, 0],
                height_fix8: [256i32 * 1i32, 0, 0],
            },
            x264_csp_tab_t {
                planes: 1i32,
                width_fix8: [256i32 * 4i32, 0, 0],
                height_fix8: [256i32 * 1i32, 0, 0],
            },
            x264_csp_tab_t {
                planes: 1i32,
                width_fix8: [256i32 * 3i32, 0, 0],
                height_fix8: [256i32 * 1i32, 0, 0],
            },
        ];
        let mut csp = i_csp & crate::x264_h::X264_CSP_MASK;
        if csp <= crate::x264_h::X264_CSP_NONE
            || csp >= crate::x264_h::X264_CSP_MAX
            || csp == crate::x264_h::X264_CSP_V210
        {
            return -(1i32);
        }
        x264_picture_init(pic);
        (*pic).img.i_csp = i_csp;
        (*pic).img.i_plane = csp_tab[csp as usize].planes;
        let mut depth_factor = if i_csp & crate::x264_h::X264_CSP_HIGH_DEPTH != 0 { 2i32 } else { 1i32 };
        while i < (*pic).img.i_plane {
            let mut stride = (((i_width as crate::stdlib::int64_t
                * csp_tab[csp as usize].width_fix8[i as usize] as crate::stdlib::int64_t)
                >> 8i32)
                * depth_factor as crate::stdlib::int64_t) as ::core::ffi::c_int;
            let mut plane_size = ((i_height as crate::stdlib::int64_t
                * csp_tab[csp as usize].height_fix8[i as usize] as crate::stdlib::int64_t)
                >> 8i32)
                * stride as crate::stdlib::int64_t;
            (*pic).img.i_stride[i as usize] = stride;
            plane_offset[i as usize] = frame_size;
            frame_size += plane_size;
            i += 1;
        }
        (*pic).img.plane[0usize] = x264_malloc(frame_size) as *mut crate::stdlib::uint8_t;
        if (*pic).img.plane[0usize].is_null() {
            return -(1i32);
        }
        while i_0 < (*pic).img.i_plane {
            (*pic).img.plane[i_0 as usize] =
                (*pic).img.plane[0usize].offset(plane_offset[i_0 as usize] as isize);
            i_0 += 1;
        }
        0i32
    }
}
pub unsafe extern "C" fn x264_picture_clean(mut pic: *mut crate::x264_h::x264_picture_t) {
    unsafe {
        x264_free((*pic).img.plane[0usize] as *mut ::core::ffi::c_void);
        crate::stdlib::memset(
            pic as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<crate::x264_h::x264_picture_t>(),
        );
    }
}
pub unsafe extern "C" fn x264_param_default(mut param: *mut x264_param_t) {
    unsafe {
        crate::stdlib::memset(
            param as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<x264_param_t>(),
        );
        (*param).cpu = x264_cpu_detect();
        (*param).i_threads = X264_THREADS_AUTO;
        (*param).i_lookahead_threads = X264_THREADS_AUTO;
        (*param).deterministic = true;
        (*param).i_sync_lookahead = X264_SYNC_LOOKAHEAD_AUTO;
        (*param).i_csp = if crate::x264_config_h::X264_CHROMA_FORMAT != 0 {
            crate::x264_config_h::X264_CHROMA_FORMAT
        } else {
            crate::x264_h::X264_CSP_I420
        };
        (*param).i_width = 0i32;
        (*param).i_height = 0i32;
        (*param).vui.i_sar_width = 0i32;
        (*param).vui.i_sar_height = 0i32;
        (*param).vui.i_overscan = 0i32;
        (*param).vui.i_vidformat = 5i32;
        (*param).vui.fullrange = None;
        (*param).vui.i_colorprim = 2i32;
        (*param).vui.i_transfer = 2i32;
        (*param).vui.i_colmatrix = -(1i32);
        (*param).vui.i_chroma_loc = 0i32;
        (*param).i_fps_num = 25u32;
        (*param).i_fps_den = 1u32;
        (*param).i_level_idc = -(1i32);
        (*param).i_slice_max_size = 0i32;
        (*param).i_slice_max_mbs = 0i32;
        (*param).i_slice_count = 0i32;
        (*param).i_bitdepth = 8i32;
        (*param).i_frame_reference = 3i32;
        (*param).i_keyint_max = 250i32;
        (*param).i_keyint_min = crate::x264_h::X264_KEYINT_MIN_AUTO;
        (*param).i_bframe = 3i32;
        (*param).i_scenecut_threshold = X264_SCENECUT_THRESHOLD_DEFAULT;
        (*param).i_bframe_adaptive = X264_B_ADAPT_DEFAULT;
        (*param).i_bframe_bias = 0i32;
        (*param).i_bframe_pyramid = crate::x264_h::X264_B_PYRAMID_NORMAL;
        (*param).interlaced = false;
        (*param).constrained_intra = false;
        (*param).deblocking_filter = true;
        (*param).i_deblocking_filter_alphac0 = 0i32;
        (*param).i_deblocking_filter_beta = 0i32;
        (*param).cabac = true;
        (*param).i_cabac_init_idc = 0i32;
        (*param).rc.i_rc_method = X264_RC_CRF;
        (*param).rc.i_bitrate = 0i32;
        (*param).rc.f_rate_tolerance = 1.0f32;
        (*param).rc.i_vbv_max_bitrate = 0i32;
        (*param).rc.i_vbv_buffer_size = 0i32;
        (*param).rc.f_vbv_buffer_init = 0.9f32;
        (*param).rc.i_qp_constant = -(1i32);
        (*param).rc.f_rf_constant = 23f32;
        (*param).rc.i_qp_min = 0i32;
        (*param).rc.i_qp_max = crate::limits_h::INT_MAX;
        (*param).rc.i_qp_step = 4i32;
        (*param).rc.f_ip_factor = 1.4f32;
        (*param).rc.f_pb_factor = 1.3f32;
        (*param).rc.i_aq_mode = crate::x264_h::X264_AQ_VARIANCE;
        (*param).rc.f_aq_strength = 1.0f32;
        (*param).rc.i_lookahead = 40i32;
        (*param).rc.stat_write = false;
        (*param).rc.psz_stat_out = "x264_2pass.log".to_string();
        (*param).rc.stat_read = false;
        (*param).rc.psz_stat_in = "x264_2pass.log".to_string();
        (*param).rc.f_qcompress = 0.6f32;
        (*param).rc.f_qblur = 0.5f32;
        (*param).rc.f_complexity_blur = 20f32;
        (*param).rc.i_zones = 0i32;
        (*param).rc.mb_tree = true;
        (*param).p_log_private = crate::__stddef_null_h::NULL;
        (*param).i_log_level = crate::x264_h::X264_LOG_INFO;
        (*param).analyse.intra = X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8;
        (*param).analyse.inter =
            X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8 | X264_ANALYSE_PSUB16x16 | X264_ANALYSE_BSUB16x16;
        (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_SPATIAL;
        (*param).analyse.i_me_method = crate::x264_h::X264_ME_HEX;
        (*param).analyse.f_psy_rd = 1.0f32;
        (*param).analyse.psy = true;
        (*param).analyse.f_psy_trellis = 0f32;
        (*param).analyse.i_me_range = 16i32;
        (*param).analyse.i_subpel_refine = 7i32;
        (*param).analyse.mixed_references = true;
        (*param).analyse.chroma_me = true;
        (*param).analyse.i_mv_range_thread = -(1i32);
        (*param).analyse.i_mv_range = -(1i32);
        (*param).analyse.i_chroma_qp_offset = 0i32;
        (*param).analyse.fast_pskip = true;
        (*param).analyse.weighted_bipred = true;
        (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SMART;
        (*param).analyse.dct_decimate = true;
        (*param).analyse.transform_8x8 = true;
        (*param).analyse.i_trellis = 1i32;
        (*param).analyse.i_luma_deadzone[0usize] = 21i32;
        (*param).analyse.i_luma_deadzone[1usize] = 11i32;
        (*param).analyse.psnr = false;
        (*param).analyse.ssim = false;
        (*param).i_cqm_preset = crate::x264_h::X264_CQM_FLAT;
        crate::stdlib::memset(
            &raw mut (*param).cqm_4iy as *mut ::core::ffi::c_void,
            16i32,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>(),
        );
        crate::stdlib::memset(
            &raw mut (*param).cqm_4py as *mut ::core::ffi::c_void,
            16i32,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>(),
        );
        crate::stdlib::memset(
            &raw mut (*param).cqm_4ic as *mut ::core::ffi::c_void,
            16i32,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>(),
        );
        crate::stdlib::memset(
            &raw mut (*param).cqm_4pc as *mut ::core::ffi::c_void,
            16i32,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 16]>(),
        );
        crate::stdlib::memset(
            &raw mut (*param).cqm_8iy as *mut ::core::ffi::c_void,
            16i32,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 64]>(),
        );
        crate::stdlib::memset(
            &raw mut (*param).cqm_8py as *mut ::core::ffi::c_void,
            16i32,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 64]>(),
        );
        crate::stdlib::memset(
            &raw mut (*param).cqm_8ic as *mut ::core::ffi::c_void,
            16i32,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 64]>(),
        );
        crate::stdlib::memset(
            &raw mut (*param).cqm_8pc as *mut ::core::ffi::c_void,
            16i32,
            ::core::mem::size_of::<[crate::stdlib::uint8_t; 64]>(),
        );
        (*param).repeat_headers = true;
        (*param).annexb = true;
        (*param).aud = false;
        (*param).vfr_input = true;
        (*param).i_nal_hrd = crate::x264_h::X264_NAL_HRD_NONE;
        (*param).tff = true;
        (*param).pic_struct = false;
        (*param).fake_interlaced = false;
        (*param).i_frame_packing = -(1i32);
        (*param).i_alternative_transfer = 2i32;
        (*param).opencl = false;
        (*param).i_opencl_device = 0i32;
        (*param).opencl_device_id = crate::__stddef_null_h::NULL;
        (*param).psz_clbin_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*param).i_avcintra_class = 0i32;
        (*param).i_avcintra_flavor = crate::x264_h::X264_AVCINTRA_FLAVOR_PANASONIC;
    }
}
unsafe extern "C" fn param_apply_preset(
    mut param: *mut x264_param_t,
    mut preset: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i = crate::stdlib::strtol(preset, &raw mut end, 10i32) as ::core::ffi::c_int;
        if *end as ::core::ffi::c_int == 0i32
            && i >= 0i32
            && i < (::core::mem::size_of::<[*const ::core::ffi::c_char; 11]>())
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
                as ::core::ffi::c_int
                - 1i32
        {
            preset = X264_PRESET_NAMES[i as usize];
        }
        if crate::stdlib::strcasecmp(preset, c"ultrafast".as_ptr()) == 0 {
            (*param).i_frame_reference = 1i32;
            (*param).i_scenecut_threshold = 0i32;
            (*param).deblocking_filter = false;
            (*param).cabac = false;
            (*param).i_bframe = 0i32;
            (*param).analyse.intra = 0u32;
            (*param).analyse.inter = 0u32;
            (*param).analyse.transform_8x8 = false;
            (*param).analyse.i_me_method = crate::x264_h::X264_ME_DIA;
            (*param).analyse.i_subpel_refine = 0i32;
            (*param).rc.i_aq_mode = 0i32;
            (*param).analyse.mixed_references = false;
            (*param).analyse.i_trellis = 0i32;
            (*param).i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_NONE;
            (*param).rc.mb_tree = false;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_NONE;
            (*param).analyse.weighted_bipred = false;
            (*param).rc.i_lookahead = 0i32;
        } else if crate::stdlib::strcasecmp(preset, c"superfast".as_ptr()) == 0 {
            (*param).analyse.inter = X264_ANALYSE_I8x8 | X264_ANALYSE_I4x4;
            (*param).analyse.i_me_method = crate::x264_h::X264_ME_DIA;
            (*param).analyse.i_subpel_refine = 1i32;
            (*param).i_frame_reference = 1i32;
            (*param).analyse.mixed_references = false;
            (*param).analyse.i_trellis = 0i32;
            (*param).rc.mb_tree = false;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SIMPLE;
            (*param).rc.i_lookahead = 0i32;
        } else if crate::stdlib::strcasecmp(preset, c"veryfast".as_ptr()) == 0 {
            (*param).analyse.i_subpel_refine = 2i32;
            (*param).i_frame_reference = 1i32;
            (*param).analyse.mixed_references = false;
            (*param).analyse.i_trellis = 0i32;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SIMPLE;
            (*param).rc.i_lookahead = 10i32;
        } else if crate::stdlib::strcasecmp(preset, c"faster".as_ptr()) == 0 {
            (*param).analyse.mixed_references = false;
            (*param).i_frame_reference = 2i32;
            (*param).analyse.i_subpel_refine = 4i32;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SIMPLE;
            (*param).rc.i_lookahead = 20i32;
        } else if crate::stdlib::strcasecmp(preset, c"fast".as_ptr()) == 0 {
            (*param).i_frame_reference = 2i32;
            (*param).analyse.i_subpel_refine = 6i32;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SIMPLE;
            (*param).rc.i_lookahead = 30i32;
        } else if crate::stdlib::strcasecmp(preset, c"medium".as_ptr()) != 0 {
            if crate::stdlib::strcasecmp(preset, c"slow".as_ptr()) == 0 {
                (*param).analyse.i_subpel_refine = 8i32;
                (*param).i_frame_reference = 5i32;
                (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_AUTO;
                (*param).analyse.i_trellis = 2i32;
                (*param).rc.i_lookahead = 50i32;
            } else if crate::stdlib::strcasecmp(preset, c"slower".as_ptr()) == 0 {
                (*param).analyse.i_me_method = crate::x264_h::X264_ME_UMH;
                (*param).analyse.i_subpel_refine = 9i32;
                (*param).i_frame_reference = 8i32;
                (*param).i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_TRELLIS;
                (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_AUTO;
                (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
                (*param).analyse.i_trellis = 2i32;
                (*param).rc.i_lookahead = 60i32;
            } else if crate::stdlib::strcasecmp(preset, c"veryslow".as_ptr()) == 0 {
                (*param).analyse.i_me_method = crate::x264_h::X264_ME_UMH;
                (*param).analyse.i_subpel_refine = 10i32;
                (*param).analyse.i_me_range = 24i32;
                (*param).i_frame_reference = 16i32;
                (*param).i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_TRELLIS;
                (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_AUTO;
                (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
                (*param).analyse.i_trellis = 2i32;
                (*param).i_bframe = 8i32;
                (*param).rc.i_lookahead = 60i32;
            } else if crate::stdlib::strcasecmp(preset, c"placebo".as_ptr()) == 0 {
                (*param).analyse.i_me_method = crate::x264_h::X264_ME_TESA;
                (*param).analyse.i_subpel_refine = 11i32;
                (*param).analyse.i_me_range = 24i32;
                (*param).i_frame_reference = 16i32;
                (*param).i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_TRELLIS;
                (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_AUTO;
                (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
                (*param).analyse.fast_pskip = false;
                (*param).analyse.i_trellis = 2i32;
                (*param).i_bframe = 16i32;
                (*param).rc.i_lookahead = 60i32;
            } else {
                log::error!("invalid preset {:?}", std::ffi::CStr::from_ptr(preset).to_string_lossy());
                return -(1i32);
            }
        }
        0i32
    }
}
unsafe extern "C" fn param_apply_tune(
    mut param: *mut x264_param_t,
    mut tune: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        loop {
            let mut c2rust_current_block: u64;
            let mut psy_tuning_used = 0i32;
            let mut len = 0;
            tune = tune.offset(crate::stdlib::strspn(tune, c",./-+".as_ptr()) as isize);
            len = crate::stdlib::strcspn(tune, c",./-+".as_ptr()) as ::core::ffi::c_int;
            if len == 0 {
                break;
            }
            if len == 4i32 && crate::stdlib::strncasecmp(tune, c"film".as_ptr(), 4usize) == 0 {
                let c2rust_fresh4 = psy_tuning_used;
                psy_tuning_used += 1;
                if c2rust_fresh4 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).i_deblocking_filter_alphac0 = -(1i32);
                    (*param).i_deblocking_filter_beta = -(1i32);
                    (*param).analyse.f_psy_trellis = 0.15f32;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 9i32 && crate::stdlib::strncasecmp(tune, c"animation".as_ptr(), 9usize) == 0 {
                let c2rust_fresh5 = psy_tuning_used;
                psy_tuning_used += 1;
                if c2rust_fresh5 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).i_frame_reference = if (*param).i_frame_reference > 1i32 {
                        (*param).i_frame_reference * 2i32
                    } else {
                        1i32
                    };
                    (*param).i_deblocking_filter_alphac0 = 1i32;
                    (*param).i_deblocking_filter_beta = 1i32;
                    (*param).analyse.f_psy_rd = 0.4f32;
                    (*param).rc.f_aq_strength = 0.6f32;
                    (*param).i_bframe += 2i32;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 5i32 && crate::stdlib::strncasecmp(tune, c"grain".as_ptr(), 5usize) == 0 {
                let c2rust_fresh6 = psy_tuning_used;
                psy_tuning_used += 1;
                if c2rust_fresh6 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).i_deblocking_filter_alphac0 = -(2i32);
                    (*param).i_deblocking_filter_beta = -(2i32);
                    (*param).analyse.f_psy_trellis = 0.25f32;
                    (*param).analyse.dct_decimate = false;
                    (*param).rc.f_pb_factor = 1.1f32;
                    (*param).rc.f_ip_factor = 1.1f32;
                    (*param).rc.f_aq_strength = 0.5f32;
                    (*param).analyse.i_luma_deadzone[0usize] = 6i32;
                    (*param).analyse.i_luma_deadzone[1usize] = 6i32;
                    (*param).rc.f_qcompress = 0.8f32;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 10i32 && crate::stdlib::strncasecmp(tune, c"stillimage".as_ptr(), 10usize) == 0 {
                let c2rust_fresh7 = psy_tuning_used;
                psy_tuning_used += 1;
                if c2rust_fresh7 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).i_deblocking_filter_alphac0 = -(3i32);
                    (*param).i_deblocking_filter_beta = -(3i32);
                    (*param).analyse.f_psy_rd = 2.0f32;
                    (*param).analyse.f_psy_trellis = 0.7f32;
                    (*param).rc.f_aq_strength = 1.2f32;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 4i32 && crate::stdlib::strncasecmp(tune, c"psnr".as_ptr(), 4usize) == 0 {
                let c2rust_fresh8 = psy_tuning_used;
                psy_tuning_used += 1;
                if c2rust_fresh8 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).rc.i_aq_mode = crate::x264_h::X264_AQ_NONE;
                    (*param).analyse.psy = false;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 4i32 && crate::stdlib::strncasecmp(tune, c"ssim".as_ptr(), 4usize) == 0 {
                let c2rust_fresh9 = psy_tuning_used;
                psy_tuning_used += 1;
                if c2rust_fresh9 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).rc.i_aq_mode = crate::x264_h::X264_AQ_AUTOVARIANCE;
                    (*param).analyse.psy = false;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 10i32 && crate::stdlib::strncasecmp(tune, c"fastdecode".as_ptr(), 10usize) == 0 {
                (*param).deblocking_filter = false;
                (*param).cabac = false;
                (*param).analyse.weighted_bipred = false;
                (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_NONE;
                c2rust_current_block = 11174649648027449784;
            } else if len == 11i32 && crate::stdlib::strncasecmp(tune, c"zerolatency".as_ptr(), 11usize) == 0
            {
                (*param).rc.i_lookahead = 0i32;
                (*param).i_sync_lookahead = 0i32;
                (*param).i_bframe = 0i32;
                (*param).sliced_threads = true;
                (*param).vfr_input = false;
                (*param).rc.mb_tree = false;
                c2rust_current_block = 11174649648027449784;
            } else if len == 6i32 && crate::stdlib::strncasecmp(tune, c"touhou".as_ptr(), 6usize) == 0 {
                let c2rust_fresh10 = psy_tuning_used;
                psy_tuning_used += 1;
                if c2rust_fresh10 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).i_frame_reference = if (*param).i_frame_reference > 1i32 {
                        (*param).i_frame_reference * 2i32
                    } else {
                        1i32
                    };
                    (*param).i_deblocking_filter_alphac0 = -(1i32);
                    (*param).i_deblocking_filter_beta = -(1i32);
                    (*param).analyse.f_psy_trellis = 0.2f32;
                    (*param).rc.f_aq_strength = 1.3f32;
                    if (*param).analyse.inter & X264_ANALYSE_PSUB16x16 != 0 {
                        (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
                    }
                    c2rust_current_block = 11174649648027449784;
                }
            } else {
                let tune_str = unsafe {
                    std::str::from_utf8_unchecked(std::slice::from_raw_parts(tune as *const u8, len as usize))
                };
                log::error!("invalid tune {tune_str}");
                return -(1i32);
            }
            match c2rust_current_block {
                11543922235941715107 => {
                    let tune_str = unsafe {
                        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                            tune as *const u8,
                            len as usize,
                        ))
                    };
                    log::warn!("only 1 psy tuning can be used: ignoring tune {tune_str}");
                }
                _ => {}
            }
            tune = tune.offset(len as isize);
        }
        0i32
    }
}
pub unsafe extern "C" fn x264_param_default_preset(
    mut param: *mut x264_param_t,
    mut preset: *const ::core::ffi::c_char,
    mut tune: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        x264_param_default(param);
        if !preset.is_null() && param_apply_preset(param, preset) < 0i32 {
            return -(1i32);
        }
        if !tune.is_null() && param_apply_tune(param, tune) < 0i32 {
            return -(1i32);
        }
        0i32
    }
}
pub unsafe extern "C" fn x264_param_apply_fastfirstpass(mut param: *mut x264_param_t) {
    unsafe {
        if (*param).rc.stat_write && !(*param).rc.stat_read {
            (*param).i_frame_reference = 1i32;
            (*param).analyse.transform_8x8 = false;
            (*param).analyse.inter = 0u32;
            (*param).analyse.i_me_method = crate::x264_h::X264_ME_DIA;
            (*param).analyse.i_subpel_refine = if (2i32) < (*param).analyse.i_subpel_refine {
                2i32
            } else {
                (*param).analyse.i_subpel_refine
            };
            (*param).analyse.i_trellis = 0i32;
            (*param).analyse.fast_pskip = true;
        }
    }
}
unsafe extern "C" fn profile_string_to_int(mut str: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        if crate::stdlib::strcasecmp(str, c"baseline".as_ptr()) == 0 {
            return crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, c"main".as_ptr()) == 0 {
            return crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, c"high".as_ptr()) == 0 {
            return crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, c"high10".as_ptr()) == 0 {
            return crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, c"high422".as_ptr()) == 0 {
            return crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, c"high444".as_ptr()) == 0 {
            return crate::src::common::base::PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int;
        }
        -(1i32)
    }
}
pub unsafe extern "C" fn x264_param_apply_profile(
    mut param: *mut x264_param_t,
    mut profile: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if profile.is_null() {
            return 0i32;
        }
        let qp_bd_offset = 6i32 * ((*param).i_bitdepth - 8i32);
        let mut p = profile_string_to_int(profile);
        if p < 0i32 {
            log::error!("invalid profile: {}", std::ffi::CStr::from_ptr(profile).to_string_lossy());
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int
            && ((*param).rc.i_rc_method == X264_RC_CQP && (*param).rc.i_qp_constant <= 0i32
                || (*param).rc.i_rc_method == X264_RC_CRF
                    && ((*param).rc.f_rf_constant + qp_bd_offset as ::core::ffi::c_float)
                        as ::core::ffi::c_int
                        <= 0i32)
        {
            log::error!(
                "{} profile doesn't support lossless",
                std::ffi::CStr::from_ptr(profile).to_string_lossy(),
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int
            && (*param).i_csp & crate::x264_h::X264_CSP_MASK >= crate::x264_h::X264_CSP_I444
        {
            log::error!(
                "{} profile doesn't support 4:4:4",
                std::ffi::CStr::from_ptr(profile).to_string_lossy(),
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int
            && (*param).i_csp & crate::x264_h::X264_CSP_MASK >= crate::x264_h::X264_CSP_I422
        {
            log::error!(
                "{} profile doesn't support 4:2:2",
                std::ffi::CStr::from_ptr(profile).to_string_lossy(),
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int && (*param).i_bitdepth > 8i32 {
            log::error!(
                "{} profile doesn't support a bit depth of {}",
                std::ffi::CStr::from_ptr(profile).to_string_lossy(),
                (*param).i_bitdepth
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
            && (*param).i_csp & crate::x264_h::X264_CSP_MASK == crate::x264_h::X264_CSP_I400
        {
            log::error!(
                "{} profile doesn't support 4:0:0",
                std::ffi::CStr::from_ptr(profile).to_string_lossy()
            );
            return -(1i32);
        }
        if p == crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int {
            (*param).analyse.transform_8x8 = false;
            (*param).cabac = false;
            (*param).i_cqm_preset = crate::x264_h::X264_CQM_FLAT;
            (*param).psz_cqm_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*param).i_bframe = 0i32;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_NONE;
            if (*param).interlaced {
                log::error!("baseline profile doesn't support interlacing");
                return -(1i32);
            }
            if (*param).fake_interlaced {
                log::error!("baseline profile doesn't support fake interlacing");
                return -(1i32);
            }
        } else if p == crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int {
            (*param).analyse.transform_8x8 = false;
            (*param).i_cqm_preset = crate::x264_h::X264_CQM_FLAT;
            (*param).psz_cqm_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        0i32
    }
}

fn parse_enum(arg: Option<&str>, names: &[&str]) -> Result<i32, i32> {
    let Some(arg) = arg else { return Err(X264_PARAM_BAD_VALUE) };
    names
        .iter()
        .position(|n| !n.is_empty() && n.eq_ignore_ascii_case(arg))
        .map(|i| i as i32)
        .ok_or(X264_PARAM_BAD_VALUE)
}

fn parse_cqm<const N: usize>(s: &str) -> Result<[u8; N], i32> {
    let mut out = [0u8; N];
    let mut tokens = s.split(',');
    for slot in &mut out {
        let token = tokens.next().ok_or(X264_PARAM_BAD_VALUE)?;
        let v: u8 = token.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
        if v == 0 {
            return Err(X264_PARAM_BAD_VALUE);
        }
        *slot = v;
    }
    if tokens.next().is_some() {
        return Err(X264_PARAM_BAD_VALUE);
    }
    Ok(out)
}

fn atobool_internal(s: Option<&str>) -> Result<bool, i32> {
    let Some(s) = s else { return Ok(true) };
    if matches!(s, "1") || s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("yes") {
        Ok(true)
    } else if matches!(s, "0") || s.eq_ignore_ascii_case("false") || s.eq_ignore_ascii_case("no") {
        Ok(false)
    } else {
        Err(X264_PARAM_BAD_VALUE)
    }
}

fn atoi_internal(s: Option<&str>) -> Result<i32, i32> {
    let Some(s) = s else {
        return Err(X264_PARAM_BAD_VALUE);
    };
    s.parse().map_err(|_| X264_PARAM_BAD_VALUE)
}

fn atof_internal(s: Option<&str>) -> Result<f64, i32> {
    let Some(s) = s else {
        return Err(X264_PARAM_BAD_VALUE);
    };
    s.parse().map_err(|_| X264_PARAM_BAD_VALUE)
}

const NEGATABLE: &[&str] = &[
    // Pure bool options
    "sliced-threads",
    "deterministic",
    "n-deterministic",
    "cpu-independent",
    "bluray-compat",
    "intra-refresh",
    "open-gop",
    "cabac",
    "interlaced",
    "tff",
    "bff",
    "constrained-intra",
    "8x8dct",
    "weightb",
    "weight-b",
    "psy",
    "chroma-me",
    "mixed-refs",
    "fast-pskip",
    "dct-decimate",
    "mbtree",
    "psnr",
    "ssim",
    "aud",
    "repeat-headers",
    "annexb",
    "filler",
    "pic-struct",
    "fake-interlaced",
    "stitchable",
    "opencl",
    // Inverted-bool options
    "nf",
    "global-header",
    "force-cfr",
    // Hybrids with a bool fallback path
    "asm",
    "scenecut",
    "b-adapt",
    "filter",
    "deblock",
];

// TODO: decide if we should allow hex/octal again like strol allows wrt atoi.
// TODO: do we want to accept value=None as "auto" for values that support it?
// TODO: need to write some tests wrt the "is 1 an integer or a boolean true" ambiguity, as well as
// fallthrough.
// TODO: strdup refactor
pub fn x264_param_parse(p: &mut x264_param_t, name: &str, value: Option<&str>) -> Result<(), i32> {
    let value = value.map(|v| v.strip_prefix('=').unwrap_or(v));
    let name_owned = name.replace('_', "-");
    let mut name: &str = &name_owned;

    if let Some(stripped) = name.strip_prefix("no").map(|r| r.strip_prefix('-').unwrap_or(r)) {
        if !NEGATABLE.contains(&stripped) {
            return Err(X264_PARAM_BAD_NAME);
        }

        let inverted = match atobool_internal(value)? {
            true => Some("false"),
            false => Some("true"),
        };
        return x264_param_dispatch(p, stripped, inverted);
    }

    x264_param_dispatch(p, name, value)
}

pub fn x264_param_dispatch(p: &mut x264_param_t, name: &str, value: Option<&str>) -> Result<(), i32> {
    match name {
        "asm" => {
            // If passed a default value or using auto, auto-detect CPU. If not, try parsing it
            // first as an integer to catch 0 and 1 as integer literals, then try to parse it as a
            // boolean to catch "true"/"yes"/"false"/"no" (asm=true should also auto-detect), and if
            // all else fails, try to parse it as a list of comma-separated values and build up the
            // bitfield that way.
            p.cpu = match value {
                None => x264_cpu_detect(),
                Some(v) if v.eq_ignore_ascii_case("auto") => x264_cpu_detect(),
                Some(v) => {
                    // NOTE: We treat 1 as an integer instead of a boolean true.
                    if let Ok(v) = v.parse::<u32>() {
                        v
                    } else {
                        match atobool_internal(Some(v)) {
                            Ok(true) => x264_cpu_detect(),
                            Ok(false) => 0,
                            Err(_) => {
                                let mut cpu = 0;
                                for token in v.split(',') {
                                    let (_, flags) = X264_CPU_NAMES
                                        .iter()
                                        .find(|(n, _)| n.eq_ignore_ascii_case(token))
                                        .ok_or(X264_PARAM_BAD_VALUE)?;
                                    cpu |= flags;
                                }
                                if cpu & X264_CPU_SSSE3 != 0 && cpu & X264_CPU_SSE2_IS_SLOW == 0 {
                                    cpu |= X264_CPU_SSE2_IS_FAST;
                                }
                                cpu
                            }
                        }
                    }
                }
            };
        }
        "threads" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            p.i_threads = match v.eq_ignore_ascii_case("auto") {
                true => X264_THREADS_AUTO,
                false => atoi_internal(value)?,
            };
        }
        "lookahead-threads" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            p.i_lookahead_threads = match v.eq_ignore_ascii_case("auto") {
                true => X264_THREADS_AUTO,
                false => atoi_internal(value)?,
            };
        }
        "sliced-threads" => {
            p.sliced_threads = atobool_internal(value)?;
        }
        "sync-lookahead" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            p.i_sync_lookahead = match v.eq_ignore_ascii_case("auto") {
                true => X264_SYNC_LOOKAHEAD_AUTO,
                false => atoi_internal(value)?,
            };
        }
        "deterministic" | "n-deterministic" => {
            p.deterministic = atobool_internal(value)?;
        }
        "cpu-independent" => {
            p.cpu_independent = atobool_internal(value)?;
        }
        "level" | "level-idc" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            // `1b` is a special level, mapped to 9 in the spec. Otherwise, try parsing a float,
            // which will also accept all integers. If it's under 7.0, it's e.g. `5.2` and needs to
            // be scaled up to `52`, otherwise we assume it's a valid idc integer value.
            p.i_level_idc = if v.eq_ignore_ascii_case("1b") {
                9
            } else {
                let f = atof_internal(value)?;
                if f < 7.0 { (10.0 * f + 0.5) as i32 } else { atoi_internal(value)? }
            };
        }
        "bluray-compat" => {
            p.bluray_compat = atobool_internal(value)?;
        }
        "avcintra-class" => {
            p.i_avcintra_class = atoi_internal(value)?;
        }
        "avcintra-flavor" => {
            p.i_avcintra_flavor = parse_enum(value, X264_AVCINTRA_FLAVOR_NAMES)?;
        }
        "sar" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            let (w, h) = v.split_once([':', '/']).ok_or(X264_PARAM_BAD_VALUE)?;
            p.vui.i_sar_width = w.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
            p.vui.i_sar_height = h.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
        }
        "overscan" => {
            p.vui.i_overscan = parse_enum(value, X264_OVERSCAN_NAMES)?;
        }
        "videoformat" => {
            p.vui.i_vidformat = parse_enum(value, X264_VIDFORMAT_NAMES)?;
        }
        "fullrange" => {
            p.vui.fullrange = Some(parse_enum(value, X264_FULLRANGE_NAMES)? != 0);
        }
        "colorprim" => {
            p.vui.i_colorprim = parse_enum(value, X264_COLORPRIM_NAMES)?;
        }
        "transfer" => {
            p.vui.i_transfer = parse_enum(value, X264_TRANSFER_NAMES)?;
        }
        "colormatrix" => {
            p.vui.i_colmatrix = parse_enum(value, X264_COLMATRIX_NAMES)?;
        }
        "chromaloc" => {
            let n = atoi_internal(value)?;
            // TODO: figure out when and where we want to do validation here, since it's inconsistent.
            if !(0..=5).contains(&n) {
                return Err(X264_PARAM_BAD_VALUE);
            }
            p.vui.i_chroma_loc = n;
        }
        "mastering-display" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            if v.eq_ignore_ascii_case("undef") {
                p.mastering_display = None;
            } else {
                p.mastering_display = Some(MasteringDisplay::parse(v)?);
            }
        }
        "cll" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            if v.eq_ignore_ascii_case("undef") {
                p.content_light_level.cll = false;
            } else {
                let (max_cll, max_fall) = v.split_once(',').ok_or(X264_PARAM_BAD_VALUE)?;
                p.content_light_level.i_max_cll = max_cll.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
                p.content_light_level.i_max_fall = max_fall.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
                p.content_light_level.cll = true;
            }
        }
        "alternative-transfer" => {
            p.i_alternative_transfer = parse_enum(value, X264_TRANSFER_NAMES)?;
        }
        "fps" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            match v.split_once('/') {
                Some((fps_num, fps_den)) => {
                    let fps_num = fps_num.parse::<u32>().map_err(|_| X264_PARAM_BAD_VALUE)?;
                    let fps_den = fps_den.parse::<u32>().map_err(|_| X264_PARAM_BAD_VALUE)?;
                    if fps_num == 0 || fps_den == 0 {
                        return Err(X264_PARAM_BAD_VALUE);
                    }
                    p.i_fps_num = fps_num;
                    p.i_fps_den = fps_den;
                }
                None => {
                    const FPS_SCALE: f64 = 1000.0;
                    const MIN_FPS: f64 = 0.5 / FPS_SCALE;

                    let fps = atof_internal(value)?;
                    let max = u32::MAX as f64;
                    if !(MIN_FPS..=max).contains(&fps) {
                        return Err(X264_PARAM_BAD_VALUE);
                    }

                    if fps <= max / FPS_SCALE {
                        p.i_fps_num = (fps * FPS_SCALE + 0.5) as u32;
                        p.i_fps_den = FPS_SCALE as u32;
                    } else {
                        p.i_fps_num = atoi_internal(value)? as u32;
                        p.i_fps_den = 1;
                    }
                }
            }
        }
        "ref" | "frameref" => {
            p.i_frame_reference = atoi_internal(value)?;
        }
        "dpb-size" => {
            p.i_dpb_size = atoi_internal(value)?;
        }
        "keyint" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            let n = match v.eq_ignore_ascii_case("infinite") {
                true => X264_KEYINT_MAX_INFINITE,
                false => atoi_internal(value)?,
            };
            p.i_keyint_max = n;
            p.i_keyint_min = p.i_keyint_min.min(n);
        }
        "min-keyint" | "keyint-min" => {
            let n = atoi_internal(value)?;
            p.i_keyint_min = n;
            p.i_keyint_max = p.i_keyint_max.max(n);
        }
        "scenecut" => {
            // Try parsing an integer (matches 0/1 as integers). If that fails, try parsing a bool,
            // false/no maps to 0, true/yes maps to the default value (40).
            p.i_scenecut_threshold = match atoi_internal(value) {
                Ok(n) => n,
                Err(_) => {
                    match atobool_internal(value)? {
                        true => X264_SCENECUT_THRESHOLD_DEFAULT,
                        false => 0,
                    }
                }
            };
        }
        "intra-refresh" => {
            p.intra_refresh = atobool_internal(value)?;
        }
        "bframes" => {
            p.i_bframe = atoi_internal(value)?;
        }
        "b-adapt" => {
            p.i_bframe_adaptive = match atoi_internal(value) {
                Ok(n) => n,
                Err(_) => {
                    match atobool_internal(value)? {
                        true => X264_B_ADAPT_DEFAULT,
                        false => 0,
                    }
                }
            };
        }
        "b-bias" => {
            p.i_bframe_bias = atoi_internal(value)?;
        }
        "b-pyramid" => {
            p.i_bframe_pyramid = match parse_enum(value, X264_B_PYRAMID_NAMES) {
                Ok(n) => n,
                Err(_) => atoi_internal(value)?,
            };
        }
        "open-gop" => {
            p.open_gop = atobool_internal(value)?;
        }
        "nf" => {
            p.deblocking_filter = !atobool_internal(value)?;
        }
        "filter" | "deblock" => {
            // Try splitting two values as `a:b` or `a,b` and store them. If that fails, try parsing
            // a single integer and use that for both, and if that fails, then we try to set
            // true/yes/false/no on deblocking_filter directly and use the default values in
            // alphac0/beta.
            match value.and_then(|v| v.split_once([':', ','])) {
                Some((alphac0, beta)) => {
                    // TODO: make into an Option<DeblockingFilter> (or just a tuple)
                    p.deblocking_filter = true;
                    p.i_deblocking_filter_alphac0 = alphac0.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
                    p.i_deblocking_filter_beta = beta.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
                }
                None => {
                    match atoi_internal(value) {
                        Ok(n) => {
                            p.deblocking_filter = true;
                            p.i_deblocking_filter_alphac0 = n;
                            p.i_deblocking_filter_beta = n;
                        }
                        Err(_) => p.deblocking_filter = atobool_internal(value)?,
                    }
                }
            }
        }
        "slice-max-size" => {
            p.i_slice_max_size = atoi_internal(value)?;
        }
        "slice-max-mbs" => {
            p.i_slice_max_mbs = atoi_internal(value)?;
        }
        "slice-min-mbs" => {
            p.i_slice_min_mbs = atoi_internal(value)?;
        }
        "slices" => {
            p.i_slice_count = atoi_internal(value)?;
        }
        "slices-max" => {
            p.i_slice_count_max = atoi_internal(value)?;
        }
        "cabac" => {
            p.cabac = atobool_internal(value)?;
        }
        "cabac-idc" => {
            p.i_cabac_init_idc = atoi_internal(value)?;
        }
        "interlaced" => {
            p.interlaced = atobool_internal(value)?;
        }
        "tff" => {
            p.tff = atobool_internal(value)?;
            p.interlaced = p.tff;
        }
        "bff" => {
            p.interlaced = atobool_internal(value)?;
            p.tff = !p.interlaced;
        }
        "constrained-intra" => {
            p.constrained_intra = atobool_internal(value)?;
        }
        "cqm" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            if v.eq_ignore_ascii_case("flat") {
                p.i_cqm_preset = X264_CQM_FLAT;
            } else if v.eq_ignore_ascii_case("jvt") {
                p.i_cqm_preset = X264_CQM_JVT;
            } else {
                let c_str = CString::new(v).map_err(|_| X264_PARAM_BAD_VALUE)?;
                let ptr = x264_param_strdup(p, c_str.as_ptr());
                if ptr.is_null() {
                    return Err(X264_PARAM_ALLOC_FAILED);
                }
                p.psz_cqm_file = ptr;
            }
        }
        "cqmfile" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            let c_str = CString::new(v).map_err(|_| X264_PARAM_BAD_VALUE)?;
            let ptr = x264_param_strdup(p, c_str.as_ptr());
            if ptr.is_null() {
                return Err(X264_PARAM_ALLOC_FAILED);
            }
            p.psz_cqm_file = ptr;
        }
        "cqm4" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            let parsed = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
            p.cqm_4iy = parsed;
            p.cqm_4py = parsed;
            p.cqm_4ic = parsed;
            p.cqm_4pc = parsed;
        }
        "cqm8" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            let parsed = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
            p.cqm_8iy = parsed;
            p.cqm_8py = parsed;
            p.cqm_8ic = parsed;
            p.cqm_8pc = parsed;
        }
        "cqm4i" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            let parsed = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
            p.cqm_4iy = parsed;
            p.cqm_4ic = parsed;
        }
        "cqm4p" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            let parsed = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
            p.cqm_4py = parsed;
            p.cqm_4pc = parsed;
        }
        "cqm4iy" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            p.cqm_4iy = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
        }
        "cqm4ic" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            p.cqm_4ic = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
        }
        "cqm4py" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            p.cqm_4py = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
        }
        "cqm4pc" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            p.cqm_4pc = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
        }
        "cqm8i" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            let parsed = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
            p.cqm_8iy = parsed;
            p.cqm_8ic = parsed;
        }
        "cqm8p" => {
            p.i_cqm_preset = X264_CQM_CUSTOM;
            let parsed = parse_cqm(value.ok_or(X264_PARAM_BAD_VALUE)?)?;
            p.cqm_8py = parsed;
            p.cqm_8pc = parsed;
        }
        "log" => {
            p.i_log_level = atoi_internal(value)?;
        }
        "dump-yuv" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            let c_str = CString::new(v).map_err(|_| X264_PARAM_BAD_VALUE)?;
            let ptr = x264_param_strdup(p, c_str.as_ptr());
            if ptr.is_null() {
                return Err(X264_PARAM_ALLOC_FAILED);
            }
            p.psz_dump_yuv = ptr;
        }
        "analyse" | "partitions" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            p.analyse.inter = 0;
            if v.contains("none") {
                p.analyse.inter = 0;
            }
            if v.contains("all") {
                p.analyse.inter = !0;
            }
            if v.contains("i4x4") {
                p.analyse.inter |= X264_ANALYSE_I4x4;
            }
            if v.contains("i8x8") {
                p.analyse.inter |= X264_ANALYSE_I8x8;
            }
            if v.contains("p8x8") {
                p.analyse.inter |= X264_ANALYSE_PSUB16x16;
            }
            if v.contains("p4x4") {
                p.analyse.inter |= X264_ANALYSE_PSUB8x8;
            }
            if v.contains("b8x8") {
                p.analyse.inter |= X264_ANALYSE_BSUB16x16;
            }
        }
        "8x8dct" => {
            p.analyse.transform_8x8 = atobool_internal(value)?;
        }
        "weightb" | "weight-b" => {
            p.analyse.weighted_bipred = atobool_internal(value)?;
        }
        "weightp" => {
            p.analyse.i_weighted_pred = atoi_internal(value)?;
        }
        "direct" | "direct-pred" => {
            p.analyse.i_direct_mv_pred = parse_enum(value, X264_DIRECT_PRED_NAMES)?;
        }
        "chroma-qp-offset" => {
            p.analyse.i_chroma_qp_offset = atoi_internal(value)?;
        }
        "me" => {
            p.analyse.i_me_method = parse_enum(value, X264_MOTION_EST_NAMES)?;
        }
        "merange" | "me-range" => {
            p.analyse.i_me_range = atoi_internal(value)?;
        }
        "mvrange" | "mv-range" => {
            p.analyse.i_mv_range = atoi_internal(value)?;
        }
        "mvrange-thread" | "mv-range-thread" => {
            p.analyse.i_mv_range_thread = atoi_internal(value)?;
        }
        "subme" | "subq" => {
            p.analyse.i_subpel_refine = atoi_internal(value)?;
        }
        "psy-rd" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            match v.split_once([':', ',', '|']) {
                Some((psy_rd, psy_trellis)) => {
                    p.analyse.f_psy_rd = psy_rd.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
                    p.analyse.f_psy_trellis = psy_trellis.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
                }
                None => {
                    (*p).analyse.f_psy_rd = v.parse().map_err(|_| X264_PARAM_BAD_VALUE)?;
                    (*p).analyse.f_psy_trellis = 0.0;
                }
            }
        }
        "psy" => {
            p.analyse.psy = atobool_internal(value)?;
        }
        "chroma-me" => {
            p.analyse.chroma_me = atobool_internal(value)?;
        }
        "mixed-refs" => {
            p.analyse.mixed_references = atobool_internal(value)?;
        }
        "trellis" => {
            p.analyse.i_trellis = atoi_internal(value)?;
        }
        "fast-pskip" => {
            p.analyse.fast_pskip = atobool_internal(value)?;
        }
        "dct-decimate" => {
            p.analyse.dct_decimate = atobool_internal(value)?;
        }
        "deadzone-inter" => {
            p.analyse.i_luma_deadzone[0] = atoi_internal(value)?;
        }
        "deadzone-intra" => {
            p.analyse.i_luma_deadzone[1] = atoi_internal(value)?;
        }
        "nr" => {
            p.analyse.i_noise_reduction = atoi_internal(value)?;
        }
        "bitrate" => {
            p.rc.i_bitrate = atoi_internal(value)?;
            p.rc.i_rc_method = X264_RC_ABR;
        }
        "qp" | "qp-constant" => {
            p.rc.i_qp_constant = atoi_internal(value)?;
            p.rc.i_rc_method = X264_RC_CQP;
        }
        "crf" => {
            p.rc.f_rf_constant = atof_internal(value)? as f32;
            p.rc.i_rc_method = X264_RC_CRF;
        }
        "crf-max" => {
            p.rc.f_rf_constant_max = atof_internal(value)? as f32;
        }
        "rc-lookahead" => {
            p.rc.i_lookahead = atoi_internal(value)?;
        }
        "qpmin" | "qp-min" => {
            p.rc.i_qp_min = atoi_internal(value)?;
        }
        "qpmax" | "qp-max" => {
            p.rc.i_qp_max = atoi_internal(value)?;
        }
        "qpstep" | "qp-step" => {
            p.rc.i_qp_step = atoi_internal(value)?;
        }
        "ratetol" => {
            p.rc.f_rate_tolerance = match value {
                None => return Err(X264_PARAM_BAD_VALUE),
                Some("inf") => 1e9f64,
                value => atof_internal(value)?,
            } as f32;
        }
        "vbv-maxrate" => {
            p.rc.i_vbv_max_bitrate = atoi_internal(value)?;
        }
        "vbv-bufsize" => {
            p.rc.i_vbv_buffer_size = atoi_internal(value)?;
        }
        "vbv-init" => {
            p.rc.f_vbv_buffer_init = atof_internal(value)? as f32;
        }
        "ipratio" | "ip-factor" => {
            p.rc.f_ip_factor = atof_internal(value)? as f32;
        }
        "pbratio" | "pb-factor" => {
            p.rc.f_pb_factor = atof_internal(value)? as f32;
        }
        "aq-mode" => {
            p.rc.i_aq_mode = atoi_internal(value)?;
        }
        "aq-strength" => {
            p.rc.f_aq_strength = atof_internal(value)? as f32;
        }
        "pass" => {
            let pass = atoi_internal(value)?.clamp(0, 3);
            p.rc.stat_write = pass & 1 != 0;
            p.rc.stat_read = pass & 2 != 0;
        }
        "stats" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            p.rc.psz_stat_in = v.to_owned();
            p.rc.psz_stat_out = v.to_owned();
        }
        "qcomp" => {
            p.rc.f_qcompress = atof_internal(value)? as f32;
        }
        "mbtree" => {
            p.rc.mb_tree = atobool_internal(value)?;
        }
        "qblur" => {
            p.rc.f_qblur = atof_internal(value)? as f32;
        }
        "cplxblur" | "cplx-blur" => {
            p.rc.f_complexity_blur = atof_internal(value)? as f32;
        }
        "zones" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            let c_str = CString::new(v).map_err(|_| X264_PARAM_BAD_VALUE)?;
            let ptr = x264_param_strdup(p, c_str.as_ptr());
            if ptr.is_null() {
                return Err(X264_PARAM_ALLOC_FAILED);
            }
            p.rc.psz_zones = ptr;
        }
        "crop-rect" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            let mut parts = v.split(',');
            p.crop_rect.i_left = parts.next().and_then(|s| s.parse().ok()).ok_or(X264_PARAM_BAD_VALUE)?;
            p.crop_rect.i_top = parts.next().and_then(|s| s.parse().ok()).ok_or(X264_PARAM_BAD_VALUE)?;
            p.crop_rect.i_right = parts.next().and_then(|s| s.parse().ok()).ok_or(X264_PARAM_BAD_VALUE)?;
            p.crop_rect.i_bottom = parts.next().and_then(|s| s.parse().ok()).ok_or(X264_PARAM_BAD_VALUE)?;
            if parts.next().is_some() {
                return Err(X264_PARAM_BAD_VALUE);
            }
        }
        "psnr" => {
            p.analyse.psnr = atobool_internal(value)?;
        }
        "ssim" => {
            p.analyse.ssim = atobool_internal(value)?;
        }
        "aud" => {
            p.aud = atobool_internal(value)?;
        }
        "sps-id" => {
            p.i_sps_id = atoi_internal(value)?;
        }
        "global-header" => {
            p.repeat_headers = !atobool_internal(value)?;
        }
        "repeat-headers" => {
            p.repeat_headers = atobool_internal(value)?;
        }
        "annexb" => {
            p.annexb = atobool_internal(value)?;
        }
        "force-cfr" => {
            p.vfr_input = !atobool_internal(value)?;
        }
        "nal-hrd" => {
            p.i_nal_hrd = parse_enum(value, X264_NAL_HRD_NAMES)?;
        }
        "filler" => {
            p.rc.filler = atobool_internal(value)?;
        }
        "pic-struct" => {
            p.pic_struct = atobool_internal(value)?;
        }
        "fake-interlaced" => {
            p.fake_interlaced = atobool_internal(value)?;
        }
        "frame-packing" => {
            p.i_frame_packing = atoi_internal(value)?;
        }
        "stitchable" => {
            p.stitchable = atobool_internal(value)?;
        }
        "opencl" => {
            p.opencl = atobool_internal(value)?;
        }
        "opencl-clbin" => {
            let v = value.ok_or(X264_PARAM_BAD_VALUE)?;
            let c_str = CString::new(v).map_err(|_| X264_PARAM_BAD_VALUE)?;
            let ptr = x264_param_strdup(p, c_str.as_ptr());
            if ptr.is_null() {
                return Err(X264_PARAM_ALLOC_FAILED);
            }
            p.psz_clbin_file = ptr;
        }
        "opencl-device" => {
            p.i_opencl_device = atoi_internal(value)?;
        }
        _ => return Err(X264_PARAM_BAD_NAME),
    }

    Ok(())
}

pub unsafe extern "C" fn x264_param2string(
    mut p: *mut x264_param_t,
    mut b_res: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut len = 2000i32;
        if !(*p).rc.psz_zones.is_null() {
            len = (len as crate::__stddef_size_t_h::size_t)
                .wrapping_add(crate::stdlib::strlen((*p).rc.psz_zones))
                as ::core::ffi::c_int;
        }
        let mut s = x264_malloc(len as crate::stdlib::int64_t) as *mut ::core::ffi::c_char;
        let mut buf = s;
        if buf.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if b_res != 0 {
            s = s.offset(crate::stdlib::sprintf(s, c"%dx%d ".as_ptr(), (*p).i_width, (*p).i_height) as isize);
            s = s
                .offset(crate::stdlib::sprintf(s, c"fps=%u/%u ".as_ptr(), (*p).i_fps_num, (*p).i_fps_den)
                    as isize);
            s = s.offset(crate::stdlib::sprintf(
                s,
                c"timebase=%u/%u ".as_ptr(),
                (*p).i_timebase_num,
                (*p).i_timebase_den,
            ) as isize);
            s = s.offset(crate::stdlib::sprintf(s, c"bitdepth=%d ".as_ptr(), (*p).i_bitdepth) as isize);
        }
        if (*p).opencl {
            s = s
                .offset(crate::stdlib::sprintf(s, c"opencl=%d ".as_ptr(), (*p).opencl as ::core::ffi::c_int)
                    as isize);
        }
        s = s.offset(
            crate::stdlib::sprintf(s, c"cabac=%d".as_ptr(), (*p).cabac as ::core::ffi::c_int) as isize
        );
        s = s.offset(crate::stdlib::sprintf(s, c" ref=%d".as_ptr(), (*p).i_frame_reference) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" deblock=%d:%d:%d".as_ptr(),
            (*p).deblocking_filter as ::core::ffi::c_int,
            (*p).i_deblocking_filter_alphac0,
            (*p).i_deblocking_filter_beta,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" analyse=%#x:%#x".as_ptr(),
            (*p).analyse.intra,
            (*p).analyse.inter,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" me=%s".as_ptr(),
            X264_MOTION_EST_NAMES[(*p).analyse.i_me_method as usize],
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(s, c" subme=%d".as_ptr(), (*p).analyse.i_subpel_refine) as isize);
        s = s
            .offset(crate::stdlib::sprintf(s, c" psy=%d".as_ptr(), (*p).analyse.psy as ::core::ffi::c_int)
                as isize);
        if (*p).analyse.psy {
            s = s.offset(crate::stdlib::sprintf(
                s,
                c" psy_rd=%.2f:%.2f".as_ptr(),
                (*p).analyse.f_psy_rd as ::core::ffi::c_double,
                (*p).analyse.f_psy_trellis as ::core::ffi::c_double,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" mixed_ref=%d".as_ptr(),
            (*p).analyse.mixed_references as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(s, c" me_range=%d".as_ptr(), (*p).analyse.i_me_range) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" chroma_me=%d".as_ptr(),
            (*p).analyse.chroma_me as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(s, c" trellis=%d".as_ptr(), (*p).analyse.i_trellis) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" 8x8dct=%d".as_ptr(),
            (*p).analyse.transform_8x8 as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(s, c" cqm=%d".as_ptr(), (*p).i_cqm_preset) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" deadzone=%d,%d".as_ptr(),
            (*p).analyse.i_luma_deadzone[0usize],
            (*p).analyse.i_luma_deadzone[1usize],
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" fast_pskip=%d".as_ptr(),
            (*p).analyse.fast_pskip as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" chroma_qp_offset=%d".as_ptr(),
            (*p).analyse.i_chroma_qp_offset,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(s, c" threads=%d".as_ptr(), (*p).i_threads) as isize);
        s = s
            .offset(crate::stdlib::sprintf(s, c" lookahead_threads=%d".as_ptr(), (*p).i_lookahead_threads)
                as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" sliced_threads=%d".as_ptr(),
            (*p).sliced_threads as ::core::ffi::c_int,
        ) as isize);
        if (*p).i_slice_count != 0 {
            s = s.offset(crate::stdlib::sprintf(s, c" slices=%d".as_ptr(), (*p).i_slice_count) as isize);
        }
        if (*p).i_slice_count_max != 0 {
            s = s.offset(
                crate::stdlib::sprintf(s, c" slices_max=%d".as_ptr(), (*p).i_slice_count_max) as isize
            );
        }
        if (*p).i_slice_max_size != 0 {
            s = s.offset(
                crate::stdlib::sprintf(s, c" slice_max_size=%d".as_ptr(), (*p).i_slice_max_size) as isize
            );
        }
        if (*p).i_slice_max_mbs != 0 {
            s = s.offset(
                crate::stdlib::sprintf(s, c" slice_max_mbs=%d".as_ptr(), (*p).i_slice_max_mbs) as isize
            );
        }
        if (*p).i_slice_min_mbs != 0 {
            s = s.offset(
                crate::stdlib::sprintf(s, c" slice_min_mbs=%d".as_ptr(), (*p).i_slice_min_mbs) as isize
            );
        }
        s = s.offset(crate::stdlib::sprintf(s, c" nr=%d".as_ptr(), (*p).analyse.i_noise_reduction) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" decimate=%d".as_ptr(),
            (*p).analyse.dct_decimate as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" interlaced=%s".as_ptr(),
            if (*p).interlaced {
                if (*p).tff { c"tff".as_ptr() } else { c"bff".as_ptr() }
            } else if (*p).fake_interlaced {
                c"fake".as_ptr()
            } else {
                c"0".as_ptr()
            },
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" bluray_compat=%d".as_ptr(),
            (*p).bluray_compat as ::core::ffi::c_int,
        ) as isize);
        if (*p).stitchable {
            s = s.offset(crate::stdlib::sprintf(
                s,
                c" stitchable=%d".as_ptr(),
                (*p).stitchable as ::core::ffi::c_int,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" constrained_intra=%d".as_ptr(),
            (*p).constrained_intra as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(s, c" bframes=%d".as_ptr(), (*p).i_bframe) as isize);
        if (*p).i_bframe != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" b_pyramid=%d b_adapt=%d b_bias=%d direct=%d weightb=%d open_gop=%d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).i_bframe_pyramid,
                (*p).i_bframe_adaptive,
                (*p).i_bframe_bias,
                (*p).analyse.i_direct_mv_pred,
                (*p).analyse.weighted_bipred as ::core::ffi::c_int,
                (*p).open_gop as ::core::ffi::c_int,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" weightp=%d".as_ptr(),
            if (*p).analyse.i_weighted_pred > 0i32 { (*p).analyse.i_weighted_pred } else { 0i32 },
        ) as isize);
        if (*p).i_keyint_max == X264_KEYINT_MAX_INFINITE {
            s = s.offset(crate::stdlib::sprintf(s, c" keyint=infinite".as_ptr()) as isize);
        } else {
            s = s.offset(crate::stdlib::sprintf(s, c" keyint=%d".as_ptr(), (*p).i_keyint_max) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" keyint_min=%d scenecut=%d intra_refresh=%d".as_ptr(),
            (*p).i_keyint_min,
            (*p).i_scenecut_threshold,
            (*p).intra_refresh as ::core::ffi::c_int,
        ) as isize);
        if (*p).rc.mb_tree || (*p).rc.i_vbv_buffer_size != 0 {
            s =
                s.offset(
                    crate::stdlib::sprintf(s, c" rc_lookahead=%d".as_ptr(), (*p).rc.i_lookahead) as isize
                );
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            c" rc=%s mbtree=%d".as_ptr(),
            if (*p).rc.i_rc_method == X264_RC_ABR {
                if (*p).rc.stat_read {
                    c"2pass".as_ptr()
                } else if (*p).rc.i_vbv_max_bitrate == (*p).rc.i_bitrate {
                    c"cbr".as_ptr()
                } else {
                    c"abr".as_ptr()
                }
            } else if (*p).rc.i_rc_method == X264_RC_CRF {
                c"crf".as_ptr()
            } else {
                c"cqp".as_ptr()
            },
            (*p).rc.mb_tree as ::core::ffi::c_int,
        ) as isize);
        if (*p).rc.i_rc_method == X264_RC_ABR || (*p).rc.i_rc_method == X264_RC_CRF {
            if (*p).rc.i_rc_method == X264_RC_CRF {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    c" crf=%.1f".as_ptr(),
                    (*p).rc.f_rf_constant as ::core::ffi::c_double,
                ) as isize);
            } else {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    c" bitrate=%d ratetol=%.1f".as_ptr(),
                    (*p).rc.i_bitrate,
                    (*p).rc.f_rate_tolerance as ::core::ffi::c_double,
                ) as isize);
            }
            s = s.offset(crate::stdlib::sprintf(
                s,
                c" qcomp=%.2f qpmin=%d qpmax=%d qpstep=%d".as_ptr(),
                (*p).rc.f_qcompress as ::core::ffi::c_double,
                (*p).rc.i_qp_min,
                (*p).rc.i_qp_max,
                (*p).rc.i_qp_step,
            ) as isize);
            if (*p).rc.stat_read {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    c" cplxblur=%.1f qblur=%.1f".as_ptr(),
                    (*p).rc.f_complexity_blur as ::core::ffi::c_double,
                    (*p).rc.f_qblur as ::core::ffi::c_double,
                ) as isize);
            }
            if (*p).rc.i_vbv_buffer_size != 0 {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    c" vbv_maxrate=%d vbv_bufsize=%d".as_ptr(),
                    (*p).rc.i_vbv_max_bitrate,
                    (*p).rc.i_vbv_buffer_size,
                ) as isize);
                if (*p).rc.i_rc_method == X264_RC_CRF {
                    s = s.offset(crate::stdlib::sprintf(
                        s,
                        c" crf_max=%.1f".as_ptr(),
                        (*p).rc.f_rf_constant_max as ::core::ffi::c_double,
                    ) as isize);
                }
            }
        } else if (*p).rc.i_rc_method == X264_RC_CQP {
            s = s.offset(crate::stdlib::sprintf(s, c" qp=%d".as_ptr(), (*p).rc.i_qp_constant) as isize);
        }
        if (*p).rc.i_vbv_buffer_size != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                c" nal_hrd=%s filler=%d".as_ptr(),
                X264_NAL_HRD_NAMES[(*p).i_nal_hrd as usize],
                (*p).rc.filler as ::core::ffi::c_int,
            ) as isize);
        }
        if (*p).crop_rect.i_left | (*p).crop_rect.i_top | (*p).crop_rect.i_right | (*p).crop_rect.i_bottom
            != 0
        {
            s = s.offset(crate::stdlib::sprintf(
                s,
                c" crop_rect=%d,%d,%d,%d".as_ptr(),
                (*p).crop_rect.i_left,
                (*p).crop_rect.i_top,
                (*p).crop_rect.i_right,
                (*p).crop_rect.i_bottom,
            ) as isize);
        }
        if let Some(display) = (*p).mastering_display {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" mastering-display=G(%d,%d)B(%d,%d)R(%d,%d)WP(%d,%d)L(%ld,%ld)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                display.i_green_x as ::core::ffi::c_uint,
                display.i_green_y as ::core::ffi::c_uint,
                display.i_blue_x as ::core::ffi::c_uint,
                display.i_blue_y as ::core::ffi::c_uint,
                display.i_red_x as ::core::ffi::c_uint,
                display.i_red_y as ::core::ffi::c_uint,
                display.i_white_x as ::core::ffi::c_uint,
                display.i_white_y as ::core::ffi::c_uint,
                display.max_luminance,
                display.min_luminance,
            ) as isize);
        }
        if (*p).content_light_level.cll {
            s = s.offset(crate::stdlib::sprintf(
                s,
                c" cll=%d,%d".as_ptr(),
                (*p).content_light_level.i_max_cll as ::core::ffi::c_uint,
                (*p).content_light_level.i_max_fall as ::core::ffi::c_uint,
            ) as isize);
        }
        if (*p).i_frame_packing >= 0i32 {
            s = s.offset(
                crate::stdlib::sprintf(s, c" frame-packing=%d".as_ptr(), (*p).i_frame_packing) as isize
            );
        }
        if !((*p).rc.i_rc_method == X264_RC_CQP && (*p).rc.i_qp_constant == 0i32) {
            s = s.offset(crate::stdlib::sprintf(
                s,
                c" ip_ratio=%.2f".as_ptr(),
                (*p).rc.f_ip_factor as ::core::ffi::c_double,
            ) as isize);
            if (*p).i_bframe != 0 && !(*p).rc.mb_tree {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    c" pb_ratio=%.2f".as_ptr(),
                    (*p).rc.f_pb_factor as ::core::ffi::c_double,
                ) as isize);
            }
            s = s.offset(crate::stdlib::sprintf(s, c" aq=%d".as_ptr(), (*p).rc.i_aq_mode) as isize);
            if (*p).rc.i_aq_mode != 0 {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    c":%.2f".as_ptr(),
                    (*p).rc.f_aq_strength as ::core::ffi::c_double,
                ) as isize);
            }
            if !(*p).rc.psz_zones.is_null() {
                s = s.offset(crate::stdlib::sprintf(s, c" zones=%s".as_ptr(), (*p).rc.psz_zones) as isize);
            } else if (*p).rc.i_zones != 0 {
                s = s.offset(crate::stdlib::sprintf(s, c" zones".as_ptr()) as isize);
            }
        }
        buf
    }
}
