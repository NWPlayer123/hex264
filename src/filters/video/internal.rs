#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:26"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = i64;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:26"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::__int64_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:26"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:26"]
pub mod input_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:9"]
    pub struct cli_image_t {
        pub csp: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub planes: ::core::ffi::c_int,
        pub plane: [*mut uint8_t; 4],
        pub stride: [::core::ffi::c_int; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:9"]
    pub struct cli_pic_t {
        pub img: cli_image_t,
        pub pts: int64_t,
        pub duration: int64_t,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:9"]
    pub struct x264_cli_csp_t {
        pub name: *const ::core::ffi::c_char,
        pub planes: ::core::ffi::c_int,
        pub width: [::core::ffi::c_float; 4],
        pub height: [::core::ffi::c_float; 4],
        pub mod_width: ::core::ffi::c_int,
        pub mod_height: ::core::ffi::c_int,
    }
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "127:29"]
        pub static x264_cli_csps: [x264_cli_csp_t; 0];
        #[c2rust::src_loc = "129:1"]
        pub fn x264_cli_csp_is_invalid(csp: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "130:1"]
        pub fn x264_cli_csp_depth_factor(csp: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:26"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:26"]
pub mod x264_h {
    #[c2rust::src_loc = "254:9"]
    pub const X264_CSP_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:26"]
pub mod x264cli_h {
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn x264_cli_log(
            name: *const ::core::ffi::c_char,
            i_level: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::input_h::{
    cli_image_t, cli_pic_t, x264_cli_csp_depth_factor, x264_cli_csp_is_invalid, x264_cli_csp_t,
    x264_cli_csps,
};
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::uint8_t;
use self::string_h::memcpy;
pub use self::types_h::{__int64_t, __uint8_t};
pub use self::x264_h::{X264_CSP_MASK, X264_LOG_ERROR};
use self::x264cli_h::x264_cli_log;
#[no_mangle]
#[c2rust::src_loc = "30:1"]
pub unsafe extern "C" fn x264_cli_plane_copy(
    mut dst: *mut uint8_t,
    mut i_dst: ::core::ffi::c_int,
    mut src: *mut uint8_t,
    mut i_src: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    loop {
        let fresh0 = h;
        h = h - 1;
        if !(fresh0 != 0) {
            break;
        }
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            w as size_t,
        );
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    }
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
pub unsafe extern "C" fn x264_cli_pic_copy(
    mut out: *mut cli_pic_t,
    mut in_0: *mut cli_pic_t,
) -> ::core::ffi::c_int {
    let mut csp: ::core::ffi::c_int = (*in_0).img.csp & X264_CSP_MASK;
    if x264_cli_csp_is_invalid((*in_0).img.csp) != 0 {
        x264_cli_log(
            b"x264\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"invalid colorspace arg %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*in_0).img.csp,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*in_0).img.csp != (*out).img.csp
        || (*in_0).img.height != (*out).img.height
        || (*in_0).img.width != (*out).img.width
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"incompatible frame properties\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*out).duration = (*in_0).duration;
    (*out).pts = (*in_0).pts;
    (*out).opaque = (*in_0).opaque;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*out).img.planes {
        let mut height: ::core::ffi::c_int = ((*in_0).img.height as ::core::ffi::c_float
            * (*x264_cli_csps.as_ptr().offset(csp as isize)).height[i as usize])
            as ::core::ffi::c_int;
        let mut width: ::core::ffi::c_int = ((*in_0).img.width as ::core::ffi::c_float
            * (*x264_cli_csps.as_ptr().offset(csp as isize)).width[i as usize])
            as ::core::ffi::c_int;
        width *= x264_cli_csp_depth_factor((*in_0).img.csp);
        x264_cli_plane_copy(
            (*out).img.plane[i as usize],
            (*out).img.stride[i as usize],
            (*in_0).img.plane[i as usize],
            (*in_0).img.stride[i as usize],
            width,
            height,
        );
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
