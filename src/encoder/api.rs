use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::base_h::x264_log_internal;
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::uint8_t;
use crate::stdlib_h::{calloc, free};
use crate::x264_config_h::X264_CHROMA_FORMAT;
use crate::x264_h::{x264_nal_t, x264_param_t, x264_picture_t, x264_t, X264_LOG_ERROR};
extern "C" {
    #[c2rust::src_loc = "34:1"]
    fn x264_8_encoder_open(_: *mut x264_param_t, _: *mut c_void) -> *mut x264_t;
    #[c2rust::src_loc = "35:1"]
    fn x264_8_nal_encode(h: *mut x264_t, dst: *mut uint8_t, nal: *mut x264_nal_t);
    #[c2rust::src_loc = "36:1"]
    fn x264_8_encoder_reconfig(_: *mut x264_t, _: *mut x264_param_t) -> c_int;
    #[c2rust::src_loc = "37:1"]
    fn x264_8_encoder_parameters(_: *mut x264_t, _: *mut x264_param_t);
    #[c2rust::src_loc = "38:1"]
    fn x264_8_encoder_headers(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut c_int,
    ) -> c_int;
    #[c2rust::src_loc = "39:1"]
    fn x264_8_encoder_encode(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut c_int,
        pic_in: *mut x264_picture_t,
        pic_out: *mut x264_picture_t,
    ) -> c_int;
    #[c2rust::src_loc = "40:1"]
    fn x264_8_encoder_close(_: *mut x264_t);
    #[c2rust::src_loc = "41:1"]
    fn x264_8_encoder_delayed_frames(_: *mut x264_t) -> c_int;
    #[c2rust::src_loc = "42:1"]
    fn x264_8_encoder_maximum_delayed_frames(_: *mut x264_t) -> c_int;
    #[c2rust::src_loc = "43:1"]
    fn x264_8_encoder_intra_refresh(_: *mut x264_t);
    #[c2rust::src_loc = "44:1"]
    fn x264_8_encoder_invalidate_reference(_: *mut x264_t, pts: int64_t) -> c_int;
    #[c2rust::src_loc = "46:1"]
    fn x264_10_encoder_open(_: *mut x264_param_t, _: *mut c_void) -> *mut x264_t;
    #[c2rust::src_loc = "47:1"]
    fn x264_10_nal_encode(h: *mut x264_t, dst: *mut uint8_t, nal: *mut x264_nal_t);
    #[c2rust::src_loc = "48:1"]
    fn x264_10_encoder_reconfig(_: *mut x264_t, _: *mut x264_param_t) -> c_int;
    #[c2rust::src_loc = "49:1"]
    fn x264_10_encoder_parameters(_: *mut x264_t, _: *mut x264_param_t);
    #[c2rust::src_loc = "50:1"]
    fn x264_10_encoder_headers(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut c_int,
    ) -> c_int;
    #[c2rust::src_loc = "51:1"]
    fn x264_10_encoder_encode(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut c_int,
        pic_in: *mut x264_picture_t,
        pic_out: *mut x264_picture_t,
    ) -> c_int;
    #[c2rust::src_loc = "52:1"]
    fn x264_10_encoder_close(_: *mut x264_t);
    #[c2rust::src_loc = "53:1"]
    fn x264_10_encoder_delayed_frames(_: *mut x264_t) -> c_int;
    #[c2rust::src_loc = "54:1"]
    fn x264_10_encoder_maximum_delayed_frames(_: *mut x264_t) -> c_int;
    #[c2rust::src_loc = "55:1"]
    fn x264_10_encoder_intra_refresh(_: *mut x264_t);
    #[c2rust::src_loc = "56:1"]
    fn x264_10_encoder_invalidate_reference(_: *mut x264_t, pts: int64_t) -> c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "58:16"]
struct x264_api_t {
    x264: *mut x264_t,
    nal_encode: Option<unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> ()>,
    encoder_reconfig: Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> c_int>,
    encoder_parameters: Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ()>,
    encoder_headers:
        Option<unsafe extern "C" fn(*mut x264_t, *mut *mut x264_nal_t, *mut c_int) -> c_int>,
    encoder_encode: Option<
        unsafe extern "C" fn(
            *mut x264_t,
            *mut *mut x264_nal_t,
            *mut c_int,
            *mut x264_picture_t,
            *mut x264_picture_t,
        ) -> c_int,
    >,
    encoder_close: Option<unsafe extern "C" fn(*mut x264_t) -> ()>,
    encoder_delayed_frames: Option<unsafe extern "C" fn(*mut x264_t) -> c_int>,
    encoder_maximum_delayed_frames: Option<unsafe extern "C" fn(*mut x264_t) -> c_int>,
    encoder_intra_refresh: Option<unsafe extern "C" fn(*mut x264_t) -> ()>,
    encoder_invalidate_reference: Option<unsafe extern "C" fn(*mut x264_t, int64_t) -> c_int>,
}
#[no_mangle]
#[c2rust::src_loc = "32:11"]
static mut x264_chroma_format: c_int = X264_CHROMA_FORMAT;
#[no_mangle]
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn x264_encoder_open_165(mut param: *mut x264_param_t) -> *mut x264_t {
    let mut api: *mut x264_api_t =
        calloc(1 as size_t, ::core::mem::size_of::<x264_api_t>() as size_t) as *mut x264_api_t;
    if api.is_null() {
        return 0 as *mut x264_t;
    }
    if (*param).i_bitdepth == 8 as c_int {
        (*api).nal_encode = Some(
            x264_8_nal_encode
                as unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> (),
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> ()>;
        (*api).encoder_reconfig = Some(
            x264_8_encoder_reconfig
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> c_int>;
        (*api).encoder_parameters = Some(
            x264_8_encoder_parameters as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> (),
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ()>;
        (*api).encoder_headers = Some(
            x264_8_encoder_headers
                as unsafe extern "C" fn(*mut x264_t, *mut *mut x264_nal_t, *mut c_int) -> c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut *mut x264_nal_t, *mut c_int) -> c_int>;
        (*api).encoder_encode = Some(
            x264_8_encoder_encode
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> c_int,
            >;
        (*api).encoder_close = Some(x264_8_encoder_close as unsafe extern "C" fn(*mut x264_t) -> ())
            as Option<unsafe extern "C" fn(*mut x264_t) -> ()>;
        (*api).encoder_delayed_frames =
            Some(x264_8_encoder_delayed_frames as unsafe extern "C" fn(*mut x264_t) -> c_int)
                as Option<unsafe extern "C" fn(*mut x264_t) -> c_int>;
        (*api).encoder_maximum_delayed_frames = Some(
            x264_8_encoder_maximum_delayed_frames as unsafe extern "C" fn(*mut x264_t) -> c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t) -> c_int>;
        (*api).encoder_intra_refresh =
            Some(x264_8_encoder_intra_refresh as unsafe extern "C" fn(*mut x264_t) -> ())
                as Option<unsafe extern "C" fn(*mut x264_t) -> ()>;
        (*api).encoder_invalidate_reference = Some(
            x264_8_encoder_invalidate_reference
                as unsafe extern "C" fn(*mut x264_t, int64_t) -> c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, int64_t) -> c_int>;
        (*api).x264 = x264_8_encoder_open(param, api as *mut c_void);
    } else if (*param).i_bitdepth == 10 as c_int {
        (*api).nal_encode = Some(
            x264_10_nal_encode
                as unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> (),
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> ()>;
        (*api).encoder_reconfig = Some(
            x264_10_encoder_reconfig
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> c_int>;
        (*api).encoder_parameters = Some(
            x264_10_encoder_parameters
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> (),
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ()>;
        (*api).encoder_headers = Some(
            x264_10_encoder_headers
                as unsafe extern "C" fn(*mut x264_t, *mut *mut x264_nal_t, *mut c_int) -> c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut *mut x264_nal_t, *mut c_int) -> c_int>;
        (*api).encoder_encode = Some(
            x264_10_encoder_encode
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> c_int,
            >;
        (*api).encoder_close =
            Some(x264_10_encoder_close as unsafe extern "C" fn(*mut x264_t) -> ())
                as Option<unsafe extern "C" fn(*mut x264_t) -> ()>;
        (*api).encoder_delayed_frames =
            Some(x264_10_encoder_delayed_frames as unsafe extern "C" fn(*mut x264_t) -> c_int)
                as Option<unsafe extern "C" fn(*mut x264_t) -> c_int>;
        (*api).encoder_maximum_delayed_frames = Some(
            x264_10_encoder_maximum_delayed_frames as unsafe extern "C" fn(*mut x264_t) -> c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t) -> c_int>;
        (*api).encoder_intra_refresh =
            Some(x264_10_encoder_intra_refresh as unsafe extern "C" fn(*mut x264_t) -> ())
                as Option<unsafe extern "C" fn(*mut x264_t) -> ()>;
        (*api).encoder_invalidate_reference = Some(
            x264_10_encoder_invalidate_reference
                as unsafe extern "C" fn(*mut x264_t, int64_t) -> c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, int64_t) -> c_int>;
        (*api).x264 = x264_10_encoder_open(param, api as *mut c_void);
    } else {
        x264_log_internal(
            X264_LOG_ERROR,
            b"not compiled with %d bit depth support\n\0" as *const u8 as *const c_char,
            (*param).i_bitdepth,
        );
    }
    if (*api).x264.is_null() {
        free(api as *mut c_void);
        return 0 as *mut x264_t;
    }
    return api as *mut x264_t;
}
#[no_mangle]
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn x264_encoder_close(mut h: *mut x264_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    (*api).encoder_close.expect("non-null function pointer")((*api).x264);
    free(api as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn x264_nal_encode(
    mut h: *mut x264_t,
    mut dst: *mut uint8_t,
    mut nal: *mut x264_nal_t,
) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    (*api).nal_encode.expect("non-null function pointer")((*api).x264, dst, nal);
}
#[no_mangle]
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn x264_encoder_reconfig(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) -> c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api).encoder_reconfig.expect("non-null function pointer")((*api).x264, param);
}
#[no_mangle]
#[c2rust::src_loc = "152:1"]
unsafe extern "C" fn x264_encoder_parameters(mut h: *mut x264_t, mut param: *mut x264_param_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    (*api)
        .encoder_parameters
        .expect("non-null function pointer")((*api).x264, param);
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn x264_encoder_headers(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut c_int,
) -> c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api).encoder_headers.expect("non-null function pointer")((*api).x264, pp_nal, pi_nal);
}
#[no_mangle]
#[c2rust::src_loc = "166:1"]
unsafe extern "C" fn x264_encoder_encode(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut c_int,
    mut pic_in: *mut x264_picture_t,
    mut pic_out: *mut x264_picture_t,
) -> c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api).encoder_encode.expect("non-null function pointer")(
        (*api).x264,
        pp_nal,
        pi_nal,
        pic_in,
        pic_out,
    );
}
#[no_mangle]
#[c2rust::src_loc = "173:1"]
unsafe extern "C" fn x264_encoder_delayed_frames(mut h: *mut x264_t) -> c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api)
        .encoder_delayed_frames
        .expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
#[c2rust::src_loc = "180:1"]
unsafe extern "C" fn x264_encoder_maximum_delayed_frames(mut h: *mut x264_t) -> c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api)
        .encoder_maximum_delayed_frames
        .expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
#[c2rust::src_loc = "187:1"]
unsafe extern "C" fn x264_encoder_intra_refresh(mut h: *mut x264_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    (*api)
        .encoder_intra_refresh
        .expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
#[c2rust::src_loc = "194:1"]
unsafe extern "C" fn x264_encoder_invalidate_reference(
    mut h: *mut x264_t,
    mut pts: int64_t,
) -> c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api)
        .encoder_invalidate_reference
        .expect("non-null function pointer")((*api).x264, pts);
}
