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
    pub static mut x264_direct_pred_names: [*const ::core::ffi::c_char; 5] = [
        b"none\0".as_ptr() as *const ::core::ffi::c_char,
        b"spatial\0".as_ptr() as *const ::core::ffi::c_char,
        b"temporal\0".as_ptr() as *const ::core::ffi::c_char,
        b"auto\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_motion_est_names: [*const ::core::ffi::c_char; 6] = [
        b"dia\0".as_ptr() as *const ::core::ffi::c_char,
        b"hex\0".as_ptr() as *const ::core::ffi::c_char,
        b"umh\0".as_ptr() as *const ::core::ffi::c_char,
        b"esa\0".as_ptr() as *const ::core::ffi::c_char,
        b"tesa\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_b_pyramid_names: [*const ::core::ffi::c_char; 4] = [
        b"none\0".as_ptr() as *const ::core::ffi::c_char,
        b"strict\0".as_ptr() as *const ::core::ffi::c_char,
        b"normal\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_overscan_names: [*const ::core::ffi::c_char; 4] = [
        b"undef\0".as_ptr() as *const ::core::ffi::c_char,
        b"show\0".as_ptr() as *const ::core::ffi::c_char,
        b"crop\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_vidformat_names: [*const ::core::ffi::c_char; 7] = [
        b"component\0".as_ptr() as *const ::core::ffi::c_char,
        b"pal\0".as_ptr() as *const ::core::ffi::c_char,
        b"ntsc\0".as_ptr() as *const ::core::ffi::c_char,
        b"secam\0".as_ptr() as *const ::core::ffi::c_char,
        b"mac\0".as_ptr() as *const ::core::ffi::c_char,
        b"undef\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_fullrange_names: [*const ::core::ffi::c_char; 3] = [
        b"off\0".as_ptr() as *const ::core::ffi::c_char,
        b"on\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_colorprim_names: [*const ::core::ffi::c_char; 14] = [
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt709\0".as_ptr() as *const ::core::ffi::c_char,
        b"undef\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt470m\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt470bg\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte170m\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte240m\0".as_ptr() as *const ::core::ffi::c_char,
        b"film\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt2020\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte428\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte431\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte432\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_transfer_names: [*const ::core::ffi::c_char; 20] = [
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt709\0".as_ptr() as *const ::core::ffi::c_char,
        b"undef\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt470m\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt470bg\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte170m\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte240m\0".as_ptr() as *const ::core::ffi::c_char,
        b"linear\0".as_ptr() as *const ::core::ffi::c_char,
        b"log100\0".as_ptr() as *const ::core::ffi::c_char,
        b"log316\0".as_ptr() as *const ::core::ffi::c_char,
        b"iec61966-2-4\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt1361e\0".as_ptr() as *const ::core::ffi::c_char,
        b"iec61966-2-1\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt2020-10\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt2020-12\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte2084\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte428\0".as_ptr() as *const ::core::ffi::c_char,
        b"arib-std-b67\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_colmatrix_names: [*const ::core::ffi::c_char; 16] = [
        b"GBR\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt709\0".as_ptr() as *const ::core::ffi::c_char,
        b"undef\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"fcc\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt470bg\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte170m\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte240m\0".as_ptr() as *const ::core::ffi::c_char,
        b"YCgCo\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt2020nc\0".as_ptr() as *const ::core::ffi::c_char,
        b"bt2020c\0".as_ptr() as *const ::core::ffi::c_char,
        b"smpte2085\0".as_ptr() as *const ::core::ffi::c_char,
        b"chroma-derived-nc\0".as_ptr() as *const ::core::ffi::c_char,
        b"chroma-derived-c\0".as_ptr() as *const ::core::ffi::c_char,
        b"ICtCp\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_nal_hrd_names: [*const ::core::ffi::c_char; 4] = [
        b"none\0".as_ptr() as *const ::core::ffi::c_char,
        b"vbr\0".as_ptr() as *const ::core::ffi::c_char,
        b"cbr\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_avcintra_flavor_names: [*const ::core::ffi::c_char; 3] = [
        b"panasonic\0".as_ptr() as *const ::core::ffi::c_char,
        b"sony\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    pub static mut x264_preset_names: [*const ::core::ffi::c_char; 11] = [
        b"ultrafast\0".as_ptr() as *const ::core::ffi::c_char,
        b"superfast\0".as_ptr() as *const ::core::ffi::c_char,
        b"veryfast\0".as_ptr() as *const ::core::ffi::c_char,
        b"faster\0".as_ptr() as *const ::core::ffi::c_char,
        b"fast\0".as_ptr() as *const ::core::ffi::c_char,
        b"medium\0".as_ptr() as *const ::core::ffi::c_char,
        b"slow\0".as_ptr() as *const ::core::ffi::c_char,
        b"slower\0".as_ptr() as *const ::core::ffi::c_char,
        b"veryslow\0".as_ptr() as *const ::core::ffi::c_char,
        b"placebo\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
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
use crate::src::common::base::base_h::x264_clip3;
use crate::src::common::base::x264_h::x264_avcintra_flavor_names;
use crate::src::common::base::x264_h::x264_b_pyramid_names;
use crate::src::common::base::x264_h::x264_colmatrix_names;
use crate::src::common::base::x264_h::x264_colorprim_names;
use crate::src::common::base::x264_h::x264_direct_pred_names;
use crate::src::common::base::x264_h::x264_fullrange_names;
use crate::src::common::base::x264_h::x264_motion_est_names;
use crate::src::common::base::x264_h::x264_nal_hrd_names;
use crate::src::common::base::x264_h::x264_overscan_names;
use crate::src::common::base::x264_h::x264_preset_names;
use crate::src::common::base::x264_h::x264_transfer_names;
use crate::src::common::base::x264_h::x264_vidformat_names;
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
pub unsafe extern "C" fn x264_log_default(
    mut _p_unused: *mut ::core::ffi::c_void,
    mut i_level: ::core::ffi::c_int,
    mut psz_fmt: *const ::core::ffi::c_char,
    mut arg: ::core::ffi::VaList,
) {
    unsafe {
        let mut psz_prefix = ::core::ptr::null_mut::<::core::ffi::c_char>();
        match i_level {
            crate::x264_h::X264_LOG_ERROR => {
                psz_prefix = b"error\0".as_ptr() as *mut ::core::ffi::c_char;
            }
            crate::x264_h::X264_LOG_WARNING => {
                psz_prefix = b"warning\0".as_ptr() as *mut ::core::ffi::c_char;
            }
            crate::x264_h::X264_LOG_INFO => {
                psz_prefix = b"info\0".as_ptr() as *mut ::core::ffi::c_char;
            }
            crate::x264_h::X264_LOG_DEBUG => {
                psz_prefix = b"debug\0".as_ptr() as *mut ::core::ffi::c_char;
            }
            _ => {
                psz_prefix = b"unknown\0".as_ptr() as *mut ::core::ffi::c_char;
            }
        }
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"x264 [%s]: \0".as_ptr() as *const ::core::ffi::c_char,
            psz_prefix,
        );
        crate::stdlib::vfprintf(crate::stdlib::stderr, psz_fmt, arg);
    }
}
pub unsafe extern "C" fn x264_log_internal(
    mut i_level: ::core::ffi::c_int,
    mut psz_fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut arg = c2rust_args.clone();
        x264_log_default(crate::__stddef_null_h::NULL, i_level, psz_fmt, arg);
    }
}
pub unsafe extern "C" fn x264_malloc(
    mut i_size: crate::stdlib::int64_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut align_buf = ::core::ptr::null_mut::<crate::stdlib::uint8_t>();
        if i_size < 0i64
            || i_size as crate::stdlib::uint64_t
                > (u64::MAX).wrapping_sub(HUGE_PAGE_SIZE as crate::stdlib::uint64_t)
        {
            x264_log_internal(
                crate::x264_h::X264_LOG_ERROR,
                b"invalid size of malloc: %ld\n\0".as_ptr() as *const ::core::ffi::c_char,
                i_size,
            );
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
            x264_log_internal(
                crate::x264_h::X264_LOG_ERROR,
                b"malloc of size %ld failed\n\0".as_ptr() as *const ::core::ffi::c_char,
                i_size,
            );
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
        let mut fh = crate::stdlib::fopen(filename, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
        if fh.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        b_error |=
            (crate::stdlib::fseeko(fh, 0i64, crate::stdlib::SEEK_END) < 0i32) as ::core::ffi::c_int;
        let mut i_size = crate::stdlib::ftello(fh);
        b_error |= (i_size <= 0i64) as ::core::ffi::c_int;
        if crate::osdep_h::WORD_SIZE == 4i32 {
            b_error |= (i_size > i32::MAX as crate::stdlib::int64_t) as ::core::ffi::c_int;
        }
        b_error |=
            (crate::stdlib::fseeko(fh, 0i64, crate::stdlib::SEEK_SET) < 0i32) as ::core::ffi::c_int;
        if !(b_error != 0) {
            let mut buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
            buf = x264_malloc(i_size + 2i64) as *mut ::core::ffi::c_char;
            if !buf.is_null() {
                b_error |= (crate::stdlib::fread(
                    buf as *mut ::core::ffi::c_void,
                    1usize,
                    i_size as crate::__stddef_size_t_h::size_t,
                    fh,
                ) != i_size as crate::stdlib::uint64_t)
                    as ::core::ffi::c_int;
                crate::stdlib::fclose(fh);
                if b_error != 0 {
                    x264_free(buf as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if *buf.offset((i_size - 1i64) as isize) as ::core::ffi::c_int != '\n' as i32 {
                    let c2rust_fresh11 = i_size;
                    i_size = i_size + 1;
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
pub unsafe extern "C" fn x264_param_strdup(
    mut param: *mut crate::x264_h::x264_param_t,
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
                    (*buf).count = (*buf).count + 1;
                    let ref mut c2rust_fresh1 = *(&raw mut (*buf).ptr
                        as *mut *mut ::core::ffi::c_void)
                        .offset(c2rust_fresh0 as isize);
                    *c2rust_fresh1 = res as *mut ::core::ffi::c_void;
                    return res;
                }
            }
            _ => {}
        }
        x264_log_internal(
            crate::x264_h::X264_LOG_ERROR,
            b"x264_param_strdup failed\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }
}
pub unsafe extern "C" fn x264_param_cleanup(mut param: *mut crate::x264_h::x264_param_t) {
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
            x264_csp_tab_t {
                planes: 0,
                width_fix8: [0; 3],
                height_fix8: [0; 3],
            },
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
            x264_csp_tab_t {
                planes: 0,
                width_fix8: [0; 3],
                height_fix8: [0; 3],
            },
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
        let mut depth_factor = if i_csp & crate::x264_h::X264_CSP_HIGH_DEPTH != 0 {
            2i32
        } else {
            1i32
        };
        while i < (*pic).img.i_plane {
            let mut stride = (((i_width as crate::stdlib::int64_t
                * csp_tab[csp as usize].width_fix8[i as usize] as crate::stdlib::int64_t)
                >> 8i32)
                * depth_factor as crate::stdlib::int64_t)
                as ::core::ffi::c_int;
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
pub unsafe extern "C" fn x264_param_default(mut param: *mut crate::x264_h::x264_param_t) {
    unsafe {
        crate::stdlib::memset(
            param as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<crate::x264_h::x264_param_t>(),
        );
        (*param).cpu = crate::src::common::cpu::x264_cpu_detect();
        (*param).i_threads = crate::x264_h::X264_THREADS_AUTO;
        (*param).i_lookahead_threads = crate::x264_h::X264_THREADS_AUTO;
        (*param).deterministic = true;
        (*param).i_sync_lookahead = crate::x264_h::X264_SYNC_LOOKAHEAD_AUTO;
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
        (*param).i_scenecut_threshold = 40i32;
        (*param).i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_FAST;
        (*param).i_bframe_bias = 0i32;
        (*param).i_bframe_pyramid = crate::x264_h::X264_B_PYRAMID_NORMAL;
        (*param).interlaced = false;
        (*param).constrained_intra = false;
        (*param).deblocking_filter = true;
        (*param).i_deblocking_filter_alphac0 = 0i32;
        (*param).i_deblocking_filter_beta = 0i32;
        (*param).cabac = true;
        (*param).i_cabac_init_idc = 0i32;
        (*param).rc.i_rc_method = crate::x264_h::X264_RC_CRF;
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
        (*param).rc.psz_stat_out = b"x264_2pass.log\0".as_ptr() as *mut ::core::ffi::c_char;
        (*param).rc.stat_read = false;
        (*param).rc.psz_stat_in = b"x264_2pass.log\0".as_ptr() as *mut ::core::ffi::c_char;
        (*param).rc.f_qcompress = 0.6f32;
        (*param).rc.f_qblur = 0.5f32;
        (*param).rc.f_complexity_blur = 20f32;
        (*param).rc.i_zones = 0i32;
        (*param).rc.mb_tree = true;
        (*param).pf_log = Some(
            x264_log_default
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
        );
        (*param).p_log_private = crate::__stddef_null_h::NULL;
        (*param).i_log_level = crate::x264_h::X264_LOG_INFO;
        (*param).analyse.intra =
            crate::x264_h::X264_ANALYSE_I4x4 | crate::x264_h::X264_ANALYSE_I8x8;
        (*param).analyse.inter = crate::x264_h::X264_ANALYSE_I4x4
            | crate::x264_h::X264_ANALYSE_I8x8
            | crate::x264_h::X264_ANALYSE_PSUB16x16
            | crate::x264_h::X264_ANALYSE_BSUB16x16;
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
    mut param: *mut crate::x264_h::x264_param_t,
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
            preset = x264_preset_names[i as usize];
        }
        if crate::stdlib::strcasecmp(
            preset,
            b"ultrafast\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
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
        } else if crate::stdlib::strcasecmp(
            preset,
            b"superfast\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*param).analyse.inter =
                crate::x264_h::X264_ANALYSE_I8x8 | crate::x264_h::X264_ANALYSE_I4x4;
            (*param).analyse.i_me_method = crate::x264_h::X264_ME_DIA;
            (*param).analyse.i_subpel_refine = 1i32;
            (*param).i_frame_reference = 1i32;
            (*param).analyse.mixed_references = false;
            (*param).analyse.i_trellis = 0i32;
            (*param).rc.mb_tree = false;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SIMPLE;
            (*param).rc.i_lookahead = 0i32;
        } else if crate::stdlib::strcasecmp(
            preset,
            b"veryfast\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*param).analyse.i_subpel_refine = 2i32;
            (*param).i_frame_reference = 1i32;
            (*param).analyse.mixed_references = false;
            (*param).analyse.i_trellis = 0i32;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SIMPLE;
            (*param).rc.i_lookahead = 10i32;
        } else if crate::stdlib::strcasecmp(
            preset,
            b"faster\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*param).analyse.mixed_references = false;
            (*param).i_frame_reference = 2i32;
            (*param).analyse.i_subpel_refine = 4i32;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SIMPLE;
            (*param).rc.i_lookahead = 20i32;
        } else if crate::stdlib::strcasecmp(
            preset,
            b"fast\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*param).i_frame_reference = 2i32;
            (*param).analyse.i_subpel_refine = 6i32;
            (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_SIMPLE;
            (*param).rc.i_lookahead = 30i32;
        } else if !(crate::stdlib::strcasecmp(
            preset,
            b"medium\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0)
        {
            if crate::stdlib::strcasecmp(preset, b"slow\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
            {
                (*param).analyse.i_subpel_refine = 8i32;
                (*param).i_frame_reference = 5i32;
                (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_AUTO;
                (*param).analyse.i_trellis = 2i32;
                (*param).rc.i_lookahead = 50i32;
            } else if crate::stdlib::strcasecmp(
                preset,
                b"slower\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                (*param).analyse.i_me_method = crate::x264_h::X264_ME_UMH;
                (*param).analyse.i_subpel_refine = 9i32;
                (*param).i_frame_reference = 8i32;
                (*param).i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_TRELLIS;
                (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_AUTO;
                (*param).analyse.inter |= crate::x264_h::X264_ANALYSE_PSUB8x8;
                (*param).analyse.i_trellis = 2i32;
                (*param).rc.i_lookahead = 60i32;
            } else if crate::stdlib::strcasecmp(
                preset,
                b"veryslow\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                (*param).analyse.i_me_method = crate::x264_h::X264_ME_UMH;
                (*param).analyse.i_subpel_refine = 10i32;
                (*param).analyse.i_me_range = 24i32;
                (*param).i_frame_reference = 16i32;
                (*param).i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_TRELLIS;
                (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_AUTO;
                (*param).analyse.inter |= crate::x264_h::X264_ANALYSE_PSUB8x8;
                (*param).analyse.i_trellis = 2i32;
                (*param).i_bframe = 8i32;
                (*param).rc.i_lookahead = 60i32;
            } else if crate::stdlib::strcasecmp(
                preset,
                b"placebo\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                (*param).analyse.i_me_method = crate::x264_h::X264_ME_TESA;
                (*param).analyse.i_subpel_refine = 11i32;
                (*param).analyse.i_me_range = 24i32;
                (*param).i_frame_reference = 16i32;
                (*param).i_bframe_adaptive = crate::x264_h::X264_B_ADAPT_TRELLIS;
                (*param).analyse.i_direct_mv_pred = crate::x264_h::X264_DIRECT_PRED_AUTO;
                (*param).analyse.inter |= crate::x264_h::X264_ANALYSE_PSUB8x8;
                (*param).analyse.fast_pskip = false;
                (*param).analyse.i_trellis = 2i32;
                (*param).i_bframe = 16i32;
                (*param).rc.i_lookahead = 60i32;
            } else {
                x264_log_internal(
                    crate::x264_h::X264_LOG_ERROR,
                    b"invalid preset '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    preset,
                );
                return -(1i32);
            }
        }
        0i32
    }
}
unsafe extern "C" fn param_apply_tune(
    mut param: *mut crate::x264_h::x264_param_t,
    mut tune: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        loop {
            let mut c2rust_current_block: u64;
            let mut psy_tuning_used = 0i32;
            let mut len = 0;
            tune = tune.offset(crate::stdlib::strspn(
                tune,
                b",./-+\0".as_ptr() as *const ::core::ffi::c_char,
            ) as isize);
            len = crate::stdlib::strcspn(tune, b",./-+\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
            if !(len != 0) {
                break;
            }
            if len == 4i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"film\0".as_ptr() as *const ::core::ffi::c_char,
                    4usize,
                ) == 0
            {
                let c2rust_fresh4 = psy_tuning_used;
                psy_tuning_used = psy_tuning_used + 1;
                if c2rust_fresh4 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).i_deblocking_filter_alphac0 = -(1i32);
                    (*param).i_deblocking_filter_beta = -(1i32);
                    (*param).analyse.f_psy_trellis = 0.15f32;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 9i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"animation\0".as_ptr() as *const ::core::ffi::c_char,
                    9usize,
                ) == 0
            {
                let c2rust_fresh5 = psy_tuning_used;
                psy_tuning_used = psy_tuning_used + 1;
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
            } else if len == 5i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"grain\0".as_ptr() as *const ::core::ffi::c_char,
                    5usize,
                ) == 0
            {
                let c2rust_fresh6 = psy_tuning_used;
                psy_tuning_used = psy_tuning_used + 1;
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
            } else if len == 10i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"stillimage\0".as_ptr() as *const ::core::ffi::c_char,
                    10usize,
                ) == 0
            {
                let c2rust_fresh7 = psy_tuning_used;
                psy_tuning_used = psy_tuning_used + 1;
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
            } else if len == 4i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"psnr\0".as_ptr() as *const ::core::ffi::c_char,
                    4usize,
                ) == 0
            {
                let c2rust_fresh8 = psy_tuning_used;
                psy_tuning_used = psy_tuning_used + 1;
                if c2rust_fresh8 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).rc.i_aq_mode = crate::x264_h::X264_AQ_NONE;
                    (*param).analyse.psy = false;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 4i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"ssim\0".as_ptr() as *const ::core::ffi::c_char,
                    4usize,
                ) == 0
            {
                let c2rust_fresh9 = psy_tuning_used;
                psy_tuning_used = psy_tuning_used + 1;
                if c2rust_fresh9 != 0 {
                    c2rust_current_block = 11543922235941715107;
                } else {
                    (*param).rc.i_aq_mode = crate::x264_h::X264_AQ_AUTOVARIANCE;
                    (*param).analyse.psy = false;
                    c2rust_current_block = 11174649648027449784;
                }
            } else if len == 10i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"fastdecode\0".as_ptr() as *const ::core::ffi::c_char,
                    10usize,
                ) == 0
            {
                (*param).deblocking_filter = false;
                (*param).cabac = false;
                (*param).analyse.weighted_bipred = false;
                (*param).analyse.i_weighted_pred = crate::x264_h::X264_WEIGHTP_NONE;
                c2rust_current_block = 11174649648027449784;
            } else if len == 11i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"zerolatency\0".as_ptr() as *const ::core::ffi::c_char,
                    11usize,
                ) == 0
            {
                (*param).rc.i_lookahead = 0i32;
                (*param).i_sync_lookahead = 0i32;
                (*param).i_bframe = 0i32;
                (*param).sliced_threads = true;
                (*param).vfr_input = false;
                (*param).rc.mb_tree = false;
                c2rust_current_block = 11174649648027449784;
            } else if len == 6i32
                && crate::stdlib::strncasecmp(
                    tune,
                    b"touhou\0".as_ptr() as *const ::core::ffi::c_char,
                    6usize,
                ) == 0
            {
                let c2rust_fresh10 = psy_tuning_used;
                psy_tuning_used = psy_tuning_used + 1;
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
                    if (*param).analyse.inter & crate::x264_h::X264_ANALYSE_PSUB16x16 != 0 {
                        (*param).analyse.inter |= crate::x264_h::X264_ANALYSE_PSUB8x8;
                    }
                    c2rust_current_block = 11174649648027449784;
                }
            } else {
                x264_log_internal(
                    crate::x264_h::X264_LOG_ERROR,
                    b"invalid tune '%.*s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    len,
                    tune,
                );
                return -(1i32);
            }
            match c2rust_current_block {
                11543922235941715107 => {
                    x264_log_internal(
                        crate::x264_h::X264_LOG_WARNING,
                        b"only 1 psy tuning can be used: ignoring tune %.*s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        len,
                        tune,
                    );
                }
                _ => {}
            }
            tune = tune.offset(len as isize);
        }
        0i32
    }
}
pub unsafe extern "C" fn x264_param_default_preset(
    mut param: *mut crate::x264_h::x264_param_t,
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
pub unsafe extern "C" fn x264_param_apply_fastfirstpass(
    mut param: *mut crate::x264_h::x264_param_t,
) {
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
unsafe extern "C" fn profile_string_to_int(
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if crate::stdlib::strcasecmp(str, b"baseline\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            return crate::src::common::base::PROFILE_BASELINE as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, b"main\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            return crate::src::common::base::PROFILE_MAIN as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, b"high\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            return crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, b"high10\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            return crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, b"high422\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            return crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int;
        }
        if crate::stdlib::strcasecmp(str, b"high444\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            return crate::src::common::base::PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int;
        }
        -(1i32)
    }
}
pub unsafe extern "C" fn x264_param_apply_profile(
    mut param: *mut crate::x264_h::x264_param_t,
    mut profile: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if profile.is_null() {
            return 0i32;
        }
        let qp_bd_offset = 6i32 * ((*param).i_bitdepth - 8i32);
        let mut p = profile_string_to_int(profile);
        if p < 0i32 {
            x264_log_internal(
                crate::x264_h::X264_LOG_ERROR,
                b"invalid profile: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                profile,
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int
            && ((*param).rc.i_rc_method == crate::x264_h::X264_RC_CQP
                && (*param).rc.i_qp_constant <= 0i32
                || (*param).rc.i_rc_method == crate::x264_h::X264_RC_CRF
                    && ((*param).rc.f_rf_constant + qp_bd_offset as ::core::ffi::c_float)
                        as ::core::ffi::c_int
                        <= 0i32)
        {
            x264_log_internal(
                crate::x264_h::X264_LOG_ERROR,
                b"%s profile doesn't support lossless\n\0".as_ptr() as *const ::core::ffi::c_char,
                profile,
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int
            && (*param).i_csp & crate::x264_h::X264_CSP_MASK >= crate::x264_h::X264_CSP_I444
        {
            x264_log_internal(
                crate::x264_h::X264_LOG_ERROR,
                b"%s profile doesn't support 4:4:4\n\0".as_ptr() as *const ::core::ffi::c_char,
                profile,
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH422 as ::core::ffi::c_int
            && (*param).i_csp & crate::x264_h::X264_CSP_MASK >= crate::x264_h::X264_CSP_I422
        {
            x264_log_internal(
                crate::x264_h::X264_LOG_ERROR,
                b"%s profile doesn't support 4:2:2\n\0".as_ptr() as *const ::core::ffi::c_char,
                profile,
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH10 as ::core::ffi::c_int
            && (*param).i_bitdepth > 8i32
        {
            x264_log_internal(
                crate::x264_h::X264_LOG_ERROR,
                b"%s profile doesn't support a bit depth of %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                profile,
                (*param).i_bitdepth,
            );
            return -(1i32);
        }
        if p < crate::src::common::base::PROFILE_HIGH as ::core::ffi::c_int
            && (*param).i_csp & crate::x264_h::X264_CSP_MASK == crate::x264_h::X264_CSP_I400
        {
            x264_log_internal(
                crate::x264_h::X264_LOG_ERROR,
                b"%s profile doesn't support 4:0:0\n\0".as_ptr() as *const ::core::ffi::c_char,
                profile,
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
                x264_log_internal(
                    crate::x264_h::X264_LOG_ERROR,
                    b"baseline profile doesn't support interlacing\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -(1i32);
            }
            if (*param).fake_interlaced {
                x264_log_internal(
                    crate::x264_h::X264_LOG_ERROR,
                    b"baseline profile doesn't support fake interlacing\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
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
unsafe extern "C" fn parse_enum(
    mut arg: *const ::core::ffi::c_char,
    mut names: *const *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i = 0i32;
        while !(*names.offset(i as isize)).is_null() {
            if **names.offset(i as isize) as ::core::ffi::c_int != 0
                && crate::stdlib::strcasecmp(arg, *names.offset(i as isize)) == 0
            {
                *dst = i;
                return 0i32;
            }
            i += 1;
        }
        -(1i32)
    }
}
unsafe extern "C" fn parse_cqm(
    mut str: *const ::core::ffi::c_char,
    mut cqm: *mut crate::stdlib::uint8_t,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i = 0i32;
        loop {
            let mut coef = 0;
            if crate::stdlib::sscanf(
                str,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut coef,
            ) == 0
                || coef < 1i32
                || coef > 255i32
            {
                return -(1i32);
            }
            let c2rust_fresh2 = i;
            i = i + 1;
            *cqm.offset(c2rust_fresh2 as isize) = coef as crate::stdlib::uint8_t;
            if !(i < length
                && {
                    str = crate::stdlib::strchr(str, ',' as i32);
                    !str.is_null()
                }
                && {
                    let c2rust_fresh3 = str;
                    str = str.offset(1);
                    !c2rust_fresh3.is_null()
                })
            {
                break;
            }
        }
        if i == length { 0i32 } else { -(1i32) }
    }
}
unsafe extern "C" fn atobool_internal(
    mut str: *const ::core::ffi::c_char,
    mut b_error: *mut ::core::ffi::c_int,
) -> bool {
    unsafe {
        if crate::stdlib::strcmp(str, b"1\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || crate::stdlib::strcasecmp(str, b"true\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || crate::stdlib::strcasecmp(str, b"yes\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            return true;
        }
        if crate::stdlib::strcmp(str, b"0\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || crate::stdlib::strcasecmp(str, b"false\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
            || crate::stdlib::strcasecmp(str, b"no\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            return false;
        }
        *b_error = 1i32;
        false
    }
}
unsafe extern "C" fn atoi_internal(
    mut str: *const ::core::ffi::c_char,
    mut b_error: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut v = crate::stdlib::strtol(str, &raw mut end, 0i32) as ::core::ffi::c_int;
        if end == str as *mut ::core::ffi::c_char || *end as ::core::ffi::c_int != '\0' as i32 {
            *b_error = 1i32;
        }
        v
    }
}
unsafe extern "C" fn atof_internal(
    mut str: *const ::core::ffi::c_char,
    mut b_error: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    unsafe {
        let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut v = crate::stdlib::strtod(str, &raw mut end);
        if end == str as *mut ::core::ffi::c_char || *end as ::core::ffi::c_int != '\0' as i32 {
            *b_error = 1i32;
        }
        v
    }
}
pub unsafe extern "C" fn x264_param_parse(
    mut p: *mut crate::x264_h::x264_param_t,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut name_buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut b_error = 0i32;
        let mut name_was_bool = 0;
        let mut errortype = crate::x264_h::X264_PARAM_BAD_VALUE;
        let mut value_was_null = value.is_null() as ::core::ffi::c_int;
        if name.is_null() {
            return crate::x264_h::X264_PARAM_BAD_NAME;
        }
        if value.is_null() {
            value = b"true\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if *value.offset(0isize) as ::core::ffi::c_int == '=' as i32 {
            value = value.offset(1);
        }
        if !crate::stdlib::strchr(name, '_' as i32).is_null() {
            name_buf = crate::stdlib::strdup(name);
            if name_buf.is_null() {
                return crate::x264_h::X264_PARAM_ALLOC_FAILED;
            }
            loop {
                let mut c = ::core::ptr::null_mut::<::core::ffi::c_char>();
                c = crate::stdlib::strchr(name_buf, '_' as i32);
                if c.is_null() {
                    break;
                }
                *c = '-' as ::core::ffi::c_char;
            }
            name = name_buf;
        }
        if crate::stdlib::strncmp(name, b"no\0".as_ptr() as *const ::core::ffi::c_char, 2usize) == 0
        {
            name = name.offset(2isize);
            if *name.offset(0isize) as ::core::ffi::c_int == '-' as i32 {
                name = name.offset(1);
            }
            name_was_bool = 1i32;
            value = if atobool_internal(value, &raw mut b_error) {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
        name_was_bool = 0i32;
        if crate::stdlib::strcmp(name, b"asm\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            (*p).cpu = if *(*crate::stdlib::__ctype_b_loc())
                .offset(
                    *value.offset(0isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                ) as ::core::ffi::c_int
                & crate::stdlib::_ISdigit as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
            {
                atoi_internal(value, &raw mut b_error) as crate::stdlib::uint32_t
            } else if crate::stdlib::strcasecmp(
                value,
                b"auto\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
                || {
                    name_was_bool = 1i32;
                    atobool_internal(value, &raw mut b_error)
                }
            {
                crate::src::common::cpu::x264_cpu_detect()
            } else {
                0u32
            };
            if b_error != 0 {
                let mut buf = crate::stdlib::strdup(value);
                if !buf.is_null() {
                    b_error = 0i32;
                    (*p).cpu = 0u32;
                    let mut init = buf;
                    loop {
                        let mut tok = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut saveptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        tok = libc::strtok_r(init, c",".as_ptr(), &raw mut saveptr);
                        if tok.is_null() {
                            break;
                        }
                        let found = crate::src::common::cpu::X264_CPU_NAMES
                            .iter()
                            .find(|(name, _)| libc::strcasecmp(tok, name.as_ptr()) == 0);
                        match found {
                            Some((_, flags)) => (*p).cpu |= flags,
                            None => b_error = 1,
                        }
                        init = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    crate::stdlib::free(buf as *mut ::core::ffi::c_void);
                    if (*p).cpu & crate::x264_h::X264_CPU_SSSE3 != 0
                        && (*p).cpu & crate::x264_h::X264_CPU_SSE2_IS_SLOW == 0
                    {
                        (*p).cpu = (*p).cpu | crate::x264_h::X264_CPU_SSE2_IS_FAST;
                    }
                } else {
                    errortype = crate::x264_h::X264_PARAM_ALLOC_FAILED;
                }
            }
        } else if crate::stdlib::strcmp(name, b"threads\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            if crate::stdlib::strcasecmp(value, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
            {
                (*p).i_threads = crate::x264_h::X264_THREADS_AUTO;
            } else {
                (*p).i_threads = atoi_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(
            name,
            b"lookahead-threads\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            if crate::stdlib::strcasecmp(value, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
            {
                (*p).i_lookahead_threads = crate::x264_h::X264_THREADS_AUTO;
            } else {
                (*p).i_lookahead_threads = atoi_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(
            name,
            b"sliced-threads\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).sliced_threads = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"sync-lookahead\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            if crate::stdlib::strcasecmp(value, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
            {
                (*p).i_sync_lookahead = crate::x264_h::X264_SYNC_LOOKAHEAD_AUTO;
            } else {
                (*p).i_sync_lookahead = atoi_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(
            name,
            b"deterministic\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
            || crate::stdlib::strcmp(
                name,
                b"n-deterministic\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            name_was_bool = 1i32;
            (*p).deterministic = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"cpu-independent\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).cpu_independent = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"level\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"level-idc\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            if crate::stdlib::strcmp(value, b"1b\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                (*p).i_level_idc = 9i32;
            } else if atof_internal(value, &raw mut b_error) < 7f64 {
                (*p).i_level_idc =
                    (10f64 * atof_internal(value, &raw mut b_error) + 0.5f64) as ::core::ffi::c_int;
            } else {
                (*p).i_level_idc = atoi_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(
            name,
            b"bluray-compat\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).bluray_compat = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"avcintra-class\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).i_avcintra_class = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"avcintra-flavor\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_avcintra_flavor_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).i_avcintra_flavor,
            );
        } else if crate::stdlib::strcmp(name, b"sar\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            b_error |= (2i32
                != crate::stdlib::sscanf(
                    value,
                    b"%d:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*p).vui.i_sar_width,
                    &raw mut (*p).vui.i_sar_height,
                )
                && 2i32
                    != crate::stdlib::sscanf(
                        value,
                        b"%d/%d\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*p).vui.i_sar_width,
                        &raw mut (*p).vui.i_sar_height,
                    )) as ::core::ffi::c_int;
        } else if crate::stdlib::strcmp(name, b"overscan\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_overscan_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).vui.i_overscan,
            );
        } else if crate::stdlib::strcmp(
            name,
            b"videoformat\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_vidformat_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).vui.i_vidformat,
            );
        } else if crate::stdlib::strcmp(name, b"fullrange\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            let mut fullrange_tmp: ::core::ffi::c_int = (*p)
                .vui
                .fullrange
                .map(|b| b as ::core::ffi::c_int)
                .unwrap_or(-1);
            let err = parse_enum(
                value,
                &raw const x264_fullrange_names as *const *const ::core::ffi::c_char,
                &raw mut fullrange_tmp,
            );
            b_error |= err;
            if err == 0 {
                (*p).vui.fullrange = Some(fullrange_tmp != 0);
            }
        } else if crate::stdlib::strcmp(name, b"colorprim\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_colorprim_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).vui.i_colorprim,
            );
        } else if crate::stdlib::strcmp(name, b"transfer\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_transfer_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).vui.i_transfer,
            );
        } else if crate::stdlib::strcmp(
            name,
            b"colormatrix\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_colmatrix_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).vui.i_colmatrix,
            );
        } else if crate::stdlib::strcmp(name, b"chromaloc\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).vui.i_chroma_loc = atoi_internal(value, &raw mut b_error);
            b_error |= ((*p).vui.i_chroma_loc < 0i32 || (*p).vui.i_chroma_loc > 5i32)
                as ::core::ffi::c_int;
        } else if crate::stdlib::strcmp(
            name,
            b"mastering-display\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            if crate::stdlib::strcasecmp(value, b"undef\0".as_ptr() as *const ::core::ffi::c_char)
                != 0
            {
                b_error |= (crate::stdlib::sscanf(
                    value,
                    b"G(%d,%d)B(%d,%d)R(%d,%d)WP(%d,%d)L(%ld,%ld)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*p).mastering_display.i_green_x,
                    &raw mut (*p).mastering_display.i_green_y,
                    &raw mut (*p).mastering_display.i_blue_x,
                    &raw mut (*p).mastering_display.i_blue_y,
                    &raw mut (*p).mastering_display.i_red_x,
                    &raw mut (*p).mastering_display.i_red_y,
                    &raw mut (*p).mastering_display.i_white_x,
                    &raw mut (*p).mastering_display.i_white_y,
                    &raw mut (*p).mastering_display.i_display_max,
                    &raw mut (*p).mastering_display.i_display_min,
                ) != 10i32) as ::core::ffi::c_int;
                (*p).mastering_display.mastering_display = b_error == 0;
            } else {
                (*p).mastering_display.mastering_display = false;
            }
        } else if crate::stdlib::strcmp(name, b"cll\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            if crate::stdlib::strcasecmp(value, b"undef\0".as_ptr() as *const ::core::ffi::c_char)
                != 0
            {
                b_error |= (crate::stdlib::sscanf(
                    value,
                    b"%d,%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*p).content_light_level.i_max_cll,
                    &raw mut (*p).content_light_level.i_max_fall,
                ) != 2i32) as ::core::ffi::c_int;
                (*p).content_light_level.cll = b_error == 0;
            } else {
                (*p).content_light_level.cll = false;
            }
        } else if crate::stdlib::strcmp(
            name,
            b"alternative-transfer\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_transfer_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).i_alternative_transfer,
            );
        } else if crate::stdlib::strcmp(name, b"fps\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            let mut i_fps_num = 0;
            let mut i_fps_den = 0;
            if crate::stdlib::sscanf(
                value,
                b"%ld/%ld\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut i_fps_num,
                &raw mut i_fps_den,
            ) == 2i32
            {
                (*p).i_fps_num = i_fps_num as crate::stdlib::uint32_t;
                (*p).i_fps_den = i_fps_den as crate::stdlib::uint32_t;
                b_error |= (i_fps_num < 1i64
                    || i_fps_num > u32::MAX as crate::stdlib::int64_t
                    || i_fps_den < 1i64
                    || i_fps_den > u32::MAX as crate::stdlib::int64_t)
                    as ::core::ffi::c_int;
            } else {
                let mut fps = atof_internal(value, &raw mut b_error);
                if fps < 0.0005 || fps > crate::limits_h::INT_MAX as ::core::ffi::c_double {
                    b_error = 1i32;
                } else if fps <= crate::limits_h::INT_MAX as ::core::ffi::c_double / 1000.0 {
                    (*p).i_fps_num = (fps * 1000.0 + 0.5) as crate::stdlib::uint32_t;
                    (*p).i_fps_den = 1000u32;
                } else {
                    (*p).i_fps_num =
                        atoi_internal(value, &raw mut b_error) as crate::stdlib::uint32_t;
                    (*p).i_fps_den = 1u32;
                }
            }
        } else if crate::stdlib::strcmp(name, b"ref\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || crate::stdlib::strcmp(name, b"frameref\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            (*p).i_frame_reference = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"dpb-size\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_dpb_size = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"keyint\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            if !crate::stdlib::strstr(value, b"infinite\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).i_keyint_max = crate::x264_h::X264_KEYINT_MAX_INFINITE;
            } else {
                (*p).i_keyint_max = atoi_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(
            name,
            b"min-keyint\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
            || crate::stdlib::strcmp(name, b"keyint-min\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            (*p).i_keyint_min = atoi_internal(value, &raw mut b_error);
            if (*p).i_keyint_max < (*p).i_keyint_min {
                (*p).i_keyint_max = (*p).i_keyint_min;
            }
        } else if crate::stdlib::strcmp(name, b"scenecut\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).i_scenecut_threshold =
                atobool_internal(value, &raw mut b_error) as ::core::ffi::c_int;
            if b_error != 0 || (*p).i_scenecut_threshold != 0 {
                b_error = 0i32;
                (*p).i_scenecut_threshold = atoi_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(
            name,
            b"intra-refresh\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).intra_refresh = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"bframes\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_bframe = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"b-adapt\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).i_bframe_adaptive =
                atobool_internal(value, &raw mut b_error) as ::core::ffi::c_int;
            if b_error != 0 {
                b_error = 0i32;
                (*p).i_bframe_adaptive = atoi_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(name, b"b-bias\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_bframe_bias = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"b-pyramid\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_b_pyramid_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).i_bframe_pyramid,
            );
            if b_error != 0 {
                b_error = 0i32;
                (*p).i_bframe_pyramid = atoi_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(name, b"open-gop\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).open_gop = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"nf\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            name_was_bool = 1i32;
            (*p).deblocking_filter = !atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"filter\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"deblock\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            if 2i32
                == crate::stdlib::sscanf(
                    value,
                    b"%d:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*p).i_deblocking_filter_alphac0,
                    &raw mut (*p).i_deblocking_filter_beta,
                )
                || 2i32
                    == crate::stdlib::sscanf(
                        value,
                        b"%d,%d\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*p).i_deblocking_filter_alphac0,
                        &raw mut (*p).i_deblocking_filter_beta,
                    )
            {
                (*p).deblocking_filter = true;
            } else if crate::stdlib::sscanf(
                value,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*p).i_deblocking_filter_alphac0,
            ) != 0
            {
                (*p).deblocking_filter = true;
                (*p).i_deblocking_filter_beta = (*p).i_deblocking_filter_alphac0;
            } else {
                name_was_bool = 1i32;
                (*p).deblocking_filter = atobool_internal(value, &raw mut b_error);
            }
        } else if crate::stdlib::strcmp(
            name,
            b"slice-max-size\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).i_slice_max_size = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"slice-max-mbs\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).i_slice_max_mbs = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"slice-min-mbs\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).i_slice_min_mbs = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"slices\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_slice_count = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"slices-max\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).i_slice_count_max = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"cabac\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).cabac = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"cabac-idc\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cabac_init_idc = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"interlaced\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).interlaced = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"tff\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            name_was_bool = 1i32;
            (*p).tff = atobool_internal(value, &raw mut b_error);
            (*p).interlaced = (*p).tff;
        } else if crate::stdlib::strcmp(name, b"bff\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            name_was_bool = 1i32;
            (*p).interlaced = atobool_internal(value, &raw mut b_error);
            (*p).tff = !(*p).interlaced;
        } else if crate::stdlib::strcmp(
            name,
            b"constrained-intra\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).constrained_intra = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"cqm\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            if !crate::stdlib::strstr(value, b"flat\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).i_cqm_preset = crate::x264_h::X264_CQM_FLAT;
            } else if !crate::stdlib::strstr(value, b"jvt\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).i_cqm_preset = crate::x264_h::X264_CQM_JVT;
            } else {
                (*p).psz_cqm_file = x264_param_strdup(p, value);
                if (*p).psz_cqm_file.is_null() {
                    b_error = 1i32;
                    errortype = crate::x264_h::X264_PARAM_ALLOC_FAILED;
                }
            }
        } else if crate::stdlib::strcmp(name, b"cqmfile\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).psz_cqm_file = x264_param_strdup(p, value);
            if (*p).psz_cqm_file.is_null() {
                b_error = 1i32;
                errortype = crate::x264_h::X264_PARAM_ALLOC_FAILED;
            }
        } else if crate::stdlib::strcmp(name, b"cqm4\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4iy as *mut crate::stdlib::uint8_t,
                16i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4py as *mut crate::stdlib::uint8_t,
                16i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4ic as *mut crate::stdlib::uint8_t,
                16i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4pc as *mut crate::stdlib::uint8_t,
                16i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm8\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_8iy as *mut crate::stdlib::uint8_t,
                64i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_8py as *mut crate::stdlib::uint8_t,
                64i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_8ic as *mut crate::stdlib::uint8_t,
                64i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_8pc as *mut crate::stdlib::uint8_t,
                64i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm4i\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4iy as *mut crate::stdlib::uint8_t,
                16i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4ic as *mut crate::stdlib::uint8_t,
                16i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm4p\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4py as *mut crate::stdlib::uint8_t,
                16i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4pc as *mut crate::stdlib::uint8_t,
                16i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm4iy\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4iy as *mut crate::stdlib::uint8_t,
                16i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm4ic\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4ic as *mut crate::stdlib::uint8_t,
                16i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm4py\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4py as *mut crate::stdlib::uint8_t,
                16i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm4pc\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_4pc as *mut crate::stdlib::uint8_t,
                16i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm8i\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_8iy as *mut crate::stdlib::uint8_t,
                64i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_8ic as *mut crate::stdlib::uint8_t,
                64i32,
            );
        } else if crate::stdlib::strcmp(name, b"cqm8p\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_8py as *mut crate::stdlib::uint8_t,
                64i32,
            );
            b_error |= parse_cqm(
                value,
                &raw mut (*p).cqm_8pc as *mut crate::stdlib::uint8_t,
                64i32,
            );
        } else if crate::stdlib::strcmp(name, b"log\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            (*p).i_log_level = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"dump-yuv\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).psz_dump_yuv = x264_param_strdup(p, value);
            if (*p).psz_dump_yuv.is_null() {
                b_error = 1i32;
                errortype = crate::x264_h::X264_PARAM_ALLOC_FAILED;
            }
        } else if crate::stdlib::strcmp(name, b"analyse\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"partitions\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            (*p).analyse.inter = 0u32;
            if !crate::stdlib::strstr(value, b"none\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).analyse.inter = 0u32;
            }
            if !crate::stdlib::strstr(value, b"all\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).analyse.inter = !(0i32) as ::core::ffi::c_uint;
            }
            if !crate::stdlib::strstr(value, b"i4x4\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).analyse.inter |= crate::x264_h::X264_ANALYSE_I4x4;
            }
            if !crate::stdlib::strstr(value, b"i8x8\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).analyse.inter |= crate::x264_h::X264_ANALYSE_I8x8;
            }
            if !crate::stdlib::strstr(value, b"p8x8\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).analyse.inter |= crate::x264_h::X264_ANALYSE_PSUB16x16;
            }
            if !crate::stdlib::strstr(value, b"p4x4\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).analyse.inter |= crate::x264_h::X264_ANALYSE_PSUB8x8;
            }
            if !crate::stdlib::strstr(value, b"b8x8\0".as_ptr() as *const ::core::ffi::c_char)
                .is_null()
            {
                (*p).analyse.inter |= crate::x264_h::X264_ANALYSE_BSUB16x16;
            }
        } else if crate::stdlib::strcmp(name, b"8x8dct\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.transform_8x8 = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"weightb\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"weight-b\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.weighted_bipred = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"weightp\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).analyse.i_weighted_pred = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"direct\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(
                name,
                b"direct-pred\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_direct_pred_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).analyse.i_direct_mv_pred,
            );
        } else if crate::stdlib::strcmp(
            name,
            b"chroma-qp-offset\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).analyse.i_chroma_qp_offset = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"me\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            b_error |= parse_enum(
                value,
                &raw const x264_motion_est_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).analyse.i_me_method,
            );
        } else if crate::stdlib::strcmp(name, b"merange\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"me-range\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            (*p).analyse.i_me_range = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"mvrange\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"mv-range\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            (*p).analyse.i_mv_range = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"mvrange-thread\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
            || crate::stdlib::strcmp(
                name,
                b"mv-range-thread\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            (*p).analyse.i_mv_range_thread = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"subme\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"subq\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            (*p).analyse.i_subpel_refine = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"psy-rd\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            if !(2i32
                == crate::stdlib::sscanf(
                    value,
                    b"%f:%f\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*p).analyse.f_psy_rd,
                    &raw mut (*p).analyse.f_psy_trellis,
                )
                || 2i32
                    == crate::stdlib::sscanf(
                        value,
                        b"%f,%f\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*p).analyse.f_psy_rd,
                        &raw mut (*p).analyse.f_psy_trellis,
                    )
                || 2i32
                    == crate::stdlib::sscanf(
                        value,
                        b"%f|%f\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*p).analyse.f_psy_rd,
                        &raw mut (*p).analyse.f_psy_trellis,
                    ))
            {
                if crate::stdlib::sscanf(
                    value,
                    b"%f\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*p).analyse.f_psy_rd,
                ) != 0
                {
                    (*p).analyse.f_psy_trellis = 0f32;
                } else {
                    (*p).analyse.f_psy_rd = 0f32;
                    (*p).analyse.f_psy_trellis = 0f32;
                }
            }
        } else if crate::stdlib::strcmp(name, b"psy\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.psy = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"chroma-me\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.chroma_me = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"mixed-refs\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.mixed_references = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"trellis\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).analyse.i_trellis = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"fast-pskip\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.fast_pskip = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"dct-decimate\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.dct_decimate = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"deadzone-inter\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).analyse.i_luma_deadzone[0usize] = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"deadzone-intra\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).analyse.i_luma_deadzone[1usize] = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"nr\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            (*p).analyse.i_noise_reduction = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"bitrate\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.i_bitrate = atoi_internal(value, &raw mut b_error);
            (*p).rc.i_rc_method = crate::x264_h::X264_RC_ABR;
        } else if crate::stdlib::strcmp(name, b"qp\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || crate::stdlib::strcmp(
                name,
                b"qp_constant\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            (*p).rc.i_qp_constant = atoi_internal(value, &raw mut b_error);
            (*p).rc.i_rc_method = crate::x264_h::X264_RC_CQP;
        } else if crate::stdlib::strcmp(name, b"crf\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            (*p).rc.f_rf_constant = atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
            (*p).rc.i_rc_method = crate::x264_h::X264_RC_CRF;
        } else if crate::stdlib::strcmp(name, b"crf-max\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.f_rf_constant_max =
                atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(
            name,
            b"rc-lookahead\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).rc.i_lookahead = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"qpmin\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"qp-min\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            (*p).rc.i_qp_min = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"qpmax\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"qp-max\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            (*p).rc.i_qp_max = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"qpstep\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"qp-step\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            (*p).rc.i_qp_step = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"ratetol\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.f_rate_tolerance = (if crate::stdlib::strncmp(
                b"inf\0".as_ptr() as *const ::core::ffi::c_char,
                value,
                3usize,
            ) == 0
            {
                1e9f64
            } else {
                atof_internal(value, &raw mut b_error)
            }) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(
            name,
            b"vbv-maxrate\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).rc.i_vbv_max_bitrate = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"vbv-bufsize\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).rc.i_vbv_buffer_size = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"vbv-init\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.f_vbv_buffer_init =
                atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(name, b"ipratio\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"ip-factor\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            (*p).rc.f_ip_factor = atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(name, b"pbratio\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"pb-factor\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            (*p).rc.f_pb_factor = atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(name, b"aq-mode\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.i_aq_mode = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"aq-strength\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).rc.f_aq_strength = atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(name, b"pass\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            let mut pass = x264_clip3(atoi_internal(value, &raw mut b_error), 0i32, 3i32);
            (*p).rc.stat_write = pass & 1i32 != 0;
            (*p).rc.stat_read = pass & 2i32 != 0;
        } else if crate::stdlib::strcmp(name, b"stats\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.psz_stat_in = x264_param_strdup(p, value);
            if (*p).rc.psz_stat_in.is_null() {
                b_error = 1i32;
                errortype = crate::x264_h::X264_PARAM_ALLOC_FAILED;
            }
            (*p).rc.psz_stat_out = x264_param_strdup(p, value);
            if (*p).rc.psz_stat_out.is_null() {
                b_error = 1i32;
                errortype = crate::x264_h::X264_PARAM_ALLOC_FAILED;
            }
        } else if crate::stdlib::strcmp(name, b"qcomp\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.f_qcompress = atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(name, b"mbtree\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).rc.mb_tree = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"qblur\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.f_qblur = atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(name, b"cplxblur\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
            || crate::stdlib::strcmp(name, b"cplx-blur\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            (*p).rc.f_complexity_blur =
                atof_internal(value, &raw mut b_error) as ::core::ffi::c_float;
        } else if crate::stdlib::strcmp(name, b"zones\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).rc.psz_zones = x264_param_strdup(p, value);
            if (*p).rc.psz_zones.is_null() {
                b_error = 1i32;
                errortype = crate::x264_h::X264_PARAM_ALLOC_FAILED;
            }
        } else if crate::stdlib::strcmp(name, b"crop-rect\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            b_error |= (crate::stdlib::sscanf(
                value,
                b"%d,%d,%d,%d\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*p).crop_rect.i_left,
                &raw mut (*p).crop_rect.i_top,
                &raw mut (*p).crop_rect.i_right,
                &raw mut (*p).crop_rect.i_bottom,
            ) != 4i32) as ::core::ffi::c_int;
        } else if crate::stdlib::strcmp(name, b"psnr\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.psnr = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"ssim\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            name_was_bool = 1i32;
            (*p).analyse.ssim = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"aud\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            name_was_bool = 1i32;
            (*p).aud = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"sps-id\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            (*p).i_sps_id = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"global-header\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).repeat_headers = !atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"repeat-headers\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).repeat_headers = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"annexb\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).annexb = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"force-cfr\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).vfr_input = !atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"nal-hrd\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            b_error |= parse_enum(
                value,
                &raw const x264_nal_hrd_names as *const *const ::core::ffi::c_char,
                &raw mut (*p).i_nal_hrd,
            );
        } else if crate::stdlib::strcmp(name, b"filler\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).rc.filler = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"pic-struct\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).pic_struct = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"fake-interlaced\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).fake_interlaced = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"frame-packing\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).i_frame_packing = atoi_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"stitchable\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            name_was_bool = 1i32;
            (*p).stitchable = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(name, b"opencl\0".as_ptr() as *const ::core::ffi::c_char)
            == 0
        {
            name_was_bool = 1i32;
            (*p).opencl = atobool_internal(value, &raw mut b_error);
        } else if crate::stdlib::strcmp(
            name,
            b"opencl-clbin\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).psz_clbin_file = x264_param_strdup(p, value);
            if (*p).psz_clbin_file.is_null() {
                b_error = 1i32;
                errortype = crate::x264_h::X264_PARAM_ALLOC_FAILED;
            }
        } else if crate::stdlib::strcmp(
            name,
            b"opencl-device\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            (*p).i_opencl_device = atoi_internal(value, &raw mut b_error);
        } else {
            b_error = 1i32;
            errortype = crate::x264_h::X264_PARAM_BAD_NAME;
        }
        if !name_buf.is_null() {
            crate::stdlib::free(name_buf as *mut ::core::ffi::c_void);
        }
        b_error |= (value_was_null != 0 && name_was_bool == 0) as ::core::ffi::c_int;
        if b_error != 0 { errortype } else { 0i32 }
    }
}
pub unsafe extern "C" fn x264_param2string(
    mut p: *mut crate::x264_h::x264_param_t,
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
            s = s.offset(crate::stdlib::sprintf(
                s,
                b"%dx%d \0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_width,
                (*p).i_height,
            ) as isize);
            s = s.offset(crate::stdlib::sprintf(
                s,
                b"fps=%u/%u \0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_fps_num,
                (*p).i_fps_den,
            ) as isize);
            s = s.offset(crate::stdlib::sprintf(
                s,
                b"timebase=%u/%u \0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_timebase_num,
                (*p).i_timebase_den,
            ) as isize);
            s = s.offset(crate::stdlib::sprintf(
                s,
                b"bitdepth=%d \0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_bitdepth,
            ) as isize);
        }
        if (*p).opencl {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b"opencl=%d \0".as_ptr() as *const ::core::ffi::c_char,
                (*p).opencl as ::core::ffi::c_int,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            b"cabac=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).cabac as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" ref=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).i_frame_reference,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" deblock=%d:%d:%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).deblocking_filter as ::core::ffi::c_int,
            (*p).i_deblocking_filter_alphac0,
            (*p).i_deblocking_filter_beta,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" analyse=%#x:%#x\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.intra,
            (*p).analyse.inter,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" me=%s\0".as_ptr() as *const ::core::ffi::c_char,
            x264_motion_est_names[(*p).analyse.i_me_method as usize],
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" subme=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.i_subpel_refine,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" psy=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.psy as ::core::ffi::c_int,
        ) as isize);
        if (*p).analyse.psy {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" psy_rd=%.2f:%.2f\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).analyse.f_psy_rd as ::core::ffi::c_double,
                (*p).analyse.f_psy_trellis as ::core::ffi::c_double,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" mixed_ref=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.mixed_references as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" me_range=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.i_me_range,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" chroma_me=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.chroma_me as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" trellis=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.i_trellis,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" 8x8dct=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.transform_8x8 as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" cqm=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).i_cqm_preset,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" deadzone=%d,%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.i_luma_deadzone[0usize],
            (*p).analyse.i_luma_deadzone[1usize],
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" fast_pskip=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.fast_pskip as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" chroma_qp_offset=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.i_chroma_qp_offset,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" threads=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).i_threads,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" lookahead_threads=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).i_lookahead_threads,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" sliced_threads=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).sliced_threads as ::core::ffi::c_int,
        ) as isize);
        if (*p).i_slice_count != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" slices=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_slice_count,
            ) as isize);
        }
        if (*p).i_slice_count_max != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" slices_max=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_slice_count_max,
            ) as isize);
        }
        if (*p).i_slice_max_size != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" slice_max_size=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_slice_max_size,
            ) as isize);
        }
        if (*p).i_slice_max_mbs != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" slice_max_mbs=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_slice_max_mbs,
            ) as isize);
        }
        if (*p).i_slice_min_mbs != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" slice_min_mbs=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_slice_min_mbs,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" nr=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.i_noise_reduction,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" decimate=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).analyse.dct_decimate as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" interlaced=%s\0".as_ptr() as *const ::core::ffi::c_char,
            if (*p).interlaced {
                if (*p).tff {
                    b"tff\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"bff\0".as_ptr() as *const ::core::ffi::c_char
                }
            } else if (*p).fake_interlaced {
                b"fake\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"0\0".as_ptr() as *const ::core::ffi::c_char
            },
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" bluray_compat=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).bluray_compat as ::core::ffi::c_int,
        ) as isize);
        if (*p).stitchable {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" stitchable=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).stitchable as ::core::ffi::c_int,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" constrained_intra=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).constrained_intra as ::core::ffi::c_int,
        ) as isize);
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" bframes=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).i_bframe,
        ) as isize);
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
            b" weightp=%d\0".as_ptr() as *const ::core::ffi::c_char,
            if (*p).analyse.i_weighted_pred > 0i32 {
                (*p).analyse.i_weighted_pred
            } else {
                0i32
            },
        ) as isize);
        if (*p).i_keyint_max == crate::x264_h::X264_KEYINT_MAX_INFINITE {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" keyint=infinite\0".as_ptr() as *const ::core::ffi::c_char,
            ) as isize);
        } else {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" keyint=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_keyint_max,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" keyint_min=%d scenecut=%d intra_refresh=%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).i_keyint_min,
            (*p).i_scenecut_threshold,
            (*p).intra_refresh as ::core::ffi::c_int,
        ) as isize);
        if (*p).rc.mb_tree || (*p).rc.i_vbv_buffer_size != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" rc_lookahead=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).rc.i_lookahead,
            ) as isize);
        }
        s = s.offset(crate::stdlib::sprintf(
            s,
            b" rc=%s mbtree=%d\0".as_ptr() as *const ::core::ffi::c_char,
            if (*p).rc.i_rc_method == crate::x264_h::X264_RC_ABR {
                if (*p).rc.stat_read {
                    b"2pass\0".as_ptr() as *const ::core::ffi::c_char
                } else if (*p).rc.i_vbv_max_bitrate == (*p).rc.i_bitrate {
                    b"cbr\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"abr\0".as_ptr() as *const ::core::ffi::c_char
                }
            } else if (*p).rc.i_rc_method == crate::x264_h::X264_RC_CRF {
                b"crf\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"cqp\0".as_ptr() as *const ::core::ffi::c_char
            },
            (*p).rc.mb_tree as ::core::ffi::c_int,
        ) as isize);
        if (*p).rc.i_rc_method == crate::x264_h::X264_RC_ABR
            || (*p).rc.i_rc_method == crate::x264_h::X264_RC_CRF
        {
            if (*p).rc.i_rc_method == crate::x264_h::X264_RC_CRF {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    b" crf=%.1f\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).rc.f_rf_constant as ::core::ffi::c_double,
                ) as isize);
            } else {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    b" bitrate=%d ratetol=%.1f\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).rc.i_bitrate,
                    (*p).rc.f_rate_tolerance as ::core::ffi::c_double,
                ) as isize);
            }
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" qcomp=%.2f qpmin=%d qpmax=%d qpstep=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).rc.f_qcompress as ::core::ffi::c_double,
                (*p).rc.i_qp_min,
                (*p).rc.i_qp_max,
                (*p).rc.i_qp_step,
            ) as isize);
            if (*p).rc.stat_read {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    b" cplxblur=%.1f qblur=%.1f\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).rc.f_complexity_blur as ::core::ffi::c_double,
                    (*p).rc.f_qblur as ::core::ffi::c_double,
                ) as isize);
            }
            if (*p).rc.i_vbv_buffer_size != 0 {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    b" vbv_maxrate=%d vbv_bufsize=%d\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).rc.i_vbv_max_bitrate,
                    (*p).rc.i_vbv_buffer_size,
                ) as isize);
                if (*p).rc.i_rc_method == crate::x264_h::X264_RC_CRF {
                    s = s.offset(crate::stdlib::sprintf(
                        s,
                        b" crf_max=%.1f\0".as_ptr() as *const ::core::ffi::c_char,
                        (*p).rc.f_rf_constant_max as ::core::ffi::c_double,
                    ) as isize);
                }
            }
        } else if (*p).rc.i_rc_method == crate::x264_h::X264_RC_CQP {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" qp=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).rc.i_qp_constant,
            ) as isize);
        }
        if (*p).rc.i_vbv_buffer_size != 0 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" nal_hrd=%s filler=%d\0".as_ptr() as *const ::core::ffi::c_char,
                x264_nal_hrd_names[(*p).i_nal_hrd as usize],
                (*p).rc.filler as ::core::ffi::c_int,
            ) as isize);
        }
        if (*p).crop_rect.i_left
            | (*p).crop_rect.i_top
            | (*p).crop_rect.i_right
            | (*p).crop_rect.i_bottom
            != 0
        {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" crop_rect=%d,%d,%d,%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).crop_rect.i_left,
                (*p).crop_rect.i_top,
                (*p).crop_rect.i_right,
                (*p).crop_rect.i_bottom,
            ) as isize);
        }
        if (*p).mastering_display.mastering_display {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" mastering-display=G(%d,%d)B(%d,%d)R(%d,%d)WP(%d,%d)L(%ld,%ld)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).mastering_display.i_green_x as ::core::ffi::c_uint,
                (*p).mastering_display.i_green_y as ::core::ffi::c_uint,
                (*p).mastering_display.i_blue_x as ::core::ffi::c_uint,
                (*p).mastering_display.i_blue_y as ::core::ffi::c_uint,
                (*p).mastering_display.i_red_x as ::core::ffi::c_uint,
                (*p).mastering_display.i_red_y as ::core::ffi::c_uint,
                (*p).mastering_display.i_white_x as ::core::ffi::c_uint,
                (*p).mastering_display.i_white_y as ::core::ffi::c_uint,
                (*p).mastering_display.i_display_max,
                (*p).mastering_display.i_display_min,
            ) as isize);
        }
        if (*p).content_light_level.cll {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" cll=%d,%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).content_light_level.i_max_cll as ::core::ffi::c_uint,
                (*p).content_light_level.i_max_fall as ::core::ffi::c_uint,
            ) as isize);
        }
        if (*p).i_frame_packing >= 0i32 {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" frame-packing=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).i_frame_packing,
            ) as isize);
        }
        if !((*p).rc.i_rc_method == crate::x264_h::X264_RC_CQP && (*p).rc.i_qp_constant == 0i32) {
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" ip_ratio=%.2f\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).rc.f_ip_factor as ::core::ffi::c_double,
            ) as isize);
            if (*p).i_bframe != 0 && !(*p).rc.mb_tree {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    b" pb_ratio=%.2f\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).rc.f_pb_factor as ::core::ffi::c_double,
                ) as isize);
            }
            s = s.offset(crate::stdlib::sprintf(
                s,
                b" aq=%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).rc.i_aq_mode,
            ) as isize);
            if (*p).rc.i_aq_mode != 0 {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    b":%.2f\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).rc.f_aq_strength as ::core::ffi::c_double,
                ) as isize);
            }
            if !(*p).rc.psz_zones.is_null() {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    b" zones=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).rc.psz_zones,
                ) as isize);
            } else if (*p).rc.i_zones != 0 {
                s = s.offset(crate::stdlib::sprintf(
                    s,
                    b" zones\0".as_ptr() as *const ::core::ffi::c_char,
                ) as isize);
            }
        }
        buf
    }
}
