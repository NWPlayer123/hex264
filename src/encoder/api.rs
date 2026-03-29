
extern "C" {

    pub fn x264_8_encoder_open(
        _: *mut crate::x264_h::x264_param_t,
        _: *mut ::core::ffi::c_void,
    ) -> *mut crate::src::common::common::x264_t;

    pub fn x264_8_nal_encode(
        h: *mut crate::src::common::common::x264_t,
        dst: *mut crate::stdlib::uint8_t,
        nal: *mut crate::x264_h::x264_nal_t,
    );

    pub fn x264_8_encoder_reconfig(
        _: *mut crate::src::common::common::x264_t,
        _: *mut crate::x264_h::x264_param_t,
    ) -> ::core::ffi::c_int;

    pub fn x264_8_encoder_parameters(
        _: *mut crate::src::common::common::x264_t,
        _: *mut crate::x264_h::x264_param_t,
    );

    pub fn x264_8_encoder_headers(
        _: *mut crate::src::common::common::x264_t,
        pp_nal: *mut *mut crate::x264_h::x264_nal_t,
        pi_nal: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn x264_8_encoder_encode(
        _: *mut crate::src::common::common::x264_t,
        pp_nal: *mut *mut crate::x264_h::x264_nal_t,
        pi_nal: *mut ::core::ffi::c_int,
        pic_in: *mut crate::x264_h::x264_picture_t_2,
        pic_out: *mut crate::x264_h::x264_picture_t_2,
    ) -> ::core::ffi::c_int;

    pub fn x264_8_encoder_close(_: *mut crate::src::common::common::x264_t);

    pub fn x264_8_encoder_delayed_frames(
        _: *mut crate::src::common::common::x264_t,
    ) -> ::core::ffi::c_int;

    pub fn x264_8_encoder_maximum_delayed_frames(
        _: *mut crate::src::common::common::x264_t,
    ) -> ::core::ffi::c_int;

    pub fn x264_8_encoder_intra_refresh(_: *mut crate::src::common::common::x264_t);

    pub fn x264_8_encoder_invalidate_reference(
        _: *mut crate::src::common::common::x264_t,
        pts: crate::stdlib::int64_t,
    ) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct x264_api_t {
    pub x264: *mut crate::src::common::common::x264_t,
    pub nal_encode: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint8_t,
            *mut crate::x264_h::x264_nal_t,
        ) -> (),
    >,
    pub encoder_reconfig: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::x264_h::x264_param_t,
        ) -> ::core::ffi::c_int,
    >,
    pub encoder_parameters: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::x264_h::x264_param_t,
        ) -> (),
    >,
    pub encoder_headers: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut *mut crate::x264_h::x264_nal_t,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub encoder_encode: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut *mut crate::x264_h::x264_nal_t,
            *mut ::core::ffi::c_int,
            *mut crate::x264_h::x264_picture_t_2,
            *mut crate::x264_h::x264_picture_t_2,
        ) -> ::core::ffi::c_int,
    >,
    pub encoder_close: Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t) -> ()>,
    pub encoder_delayed_frames:
        Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t) -> ::core::ffi::c_int>,
    pub encoder_maximum_delayed_frames:
        Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t) -> ::core::ffi::c_int>,
    pub encoder_intra_refresh:
        Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t) -> ()>,
    pub encoder_invalidate_reference: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            crate::stdlib::int64_t,
        ) -> ::core::ffi::c_int,
    >,
}
#[no_mangle]

pub static mut x264_chroma_format: ::core::ffi::c_int = crate::x264_config_h::X264_CHROMA_FORMAT;
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_open_165(
    mut param: *mut crate::x264_h::x264_param_t,
) -> *mut crate::src::common::common::x264_t {
    unsafe {
        let mut api: *mut x264_api_t = crate::stdlib::calloc(
            1 as crate::__stddef_size_t_h::size_t,
            ::core::mem::size_of::<x264_api_t>() as crate::__stddef_size_t_h::size_t,
        ) as *mut x264_api_t;
        if api.is_null() {
            return ::core::ptr::null_mut::<crate::src::common::common::x264_t>();
        }
        if (*param).i_bitdepth == 8 as ::core::ffi::c_int {
            (*api).nal_encode = Some(
                x264_8_nal_encode
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut crate::stdlib::uint8_t,
                        *mut crate::x264_h::x264_nal_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut crate::stdlib::uint8_t,
                        *mut crate::x264_h::x264_nal_t,
                    ) -> (),
                >;
            (*api).encoder_reconfig = Some(
                x264_8_encoder_reconfig
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut crate::x264_h::x264_param_t,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut crate::x264_h::x264_param_t,
                    ) -> ::core::ffi::c_int,
                >;
            (*api).encoder_parameters = Some(
                x264_8_encoder_parameters
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut crate::x264_h::x264_param_t,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut crate::x264_h::x264_param_t,
                    ) -> (),
                >;
            (*api).encoder_headers = Some(
                x264_8_encoder_headers
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut *mut crate::x264_h::x264_nal_t,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut *mut crate::x264_h::x264_nal_t,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            (*api).encoder_encode = Some(
                x264_8_encoder_encode
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut *mut crate::x264_h::x264_nal_t,
                        *mut ::core::ffi::c_int,
                        *mut crate::x264_h::x264_picture_t_2,
                        *mut crate::x264_h::x264_picture_t_2,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut *mut crate::x264_h::x264_nal_t,
                        *mut ::core::ffi::c_int,
                        *mut crate::x264_h::x264_picture_t_2,
                        *mut crate::x264_h::x264_picture_t_2,
                    ) -> ::core::ffi::c_int,
                >;
            (*api).encoder_close = Some(
                x264_8_encoder_close
                    as unsafe extern "C" fn(*mut crate::src::common::common::x264_t) -> (),
            )
                as Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t) -> ()>;
            (*api).encoder_delayed_frames = Some(
                x264_8_encoder_delayed_frames
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                    ) -> ::core::ffi::c_int,
                >;
            (*api).encoder_maximum_delayed_frames = Some(
                x264_8_encoder_maximum_delayed_frames
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                    ) -> ::core::ffi::c_int,
                >;
            (*api).encoder_intra_refresh = Some(
                x264_8_encoder_intra_refresh
                    as unsafe extern "C" fn(*mut crate::src::common::common::x264_t) -> (),
            )
                as Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t) -> ()>;
            (*api).encoder_invalidate_reference = Some(
                x264_8_encoder_invalidate_reference
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        crate::stdlib::int64_t,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        crate::stdlib::int64_t,
                    ) -> ::core::ffi::c_int,
                >;
            (*api).x264 = x264_8_encoder_open(param, api as *mut ::core::ffi::c_void);
        } else {
            crate::src::common::base::x264_log_internal(
                crate::x264_h::X264_LOG_ERROR_1,
                b"not compiled with %d bit depth support\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*param).i_bitdepth,
            );
        }
        if (*api).x264.is_null() {
            crate::stdlib::free(api as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<crate::src::common::common::x264_t>();
        }
        return api as *mut crate::src::common::common::x264_t;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_close(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        (*api).encoder_close.expect("non-null function pointer")((*api).x264);
        crate::stdlib::free(api as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_nal_encode(
    mut h: *mut crate::src::common::common::x264_t,
    mut dst: *mut crate::stdlib::uint8_t,
    mut nal: *mut crate::x264_h::x264_nal_t,
) {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        (*api).nal_encode.expect("non-null function pointer")((*api).x264, dst, nal);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_reconfig(
    mut h: *mut crate::src::common::common::x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        return (*api).encoder_reconfig.expect("non-null function pointer")((*api).x264, param);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_parameters(
    mut h: *mut crate::src::common::common::x264_t,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        (*api)
            .encoder_parameters
            .expect("non-null function pointer")((*api).x264, param);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_headers(
    mut h: *mut crate::src::common::common::x264_t,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        return (*api).encoder_headers.expect("non-null function pointer")(
            (*api).x264,
            pp_nal,
            pi_nal,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_encode(
    mut h: *mut crate::src::common::common::x264_t,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_in: *mut crate::x264_h::x264_picture_t_2,
    mut pic_out: *mut crate::x264_h::x264_picture_t_2,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        return (*api).encoder_encode.expect("non-null function pointer")(
            (*api).x264,
            pp_nal,
            pi_nal,
            pic_in,
            pic_out,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_delayed_frames(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        return (*api)
            .encoder_delayed_frames
            .expect("non-null function pointer")((*api).x264);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_maximum_delayed_frames(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        return (*api)
            .encoder_maximum_delayed_frames
            .expect("non-null function pointer")((*api).x264);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_intra_refresh(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        (*api)
            .encoder_intra_refresh
            .expect("non-null function pointer")((*api).x264);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_encoder_invalidate_reference(
    mut h: *mut crate::src::common::common::x264_t,
    mut pts: crate::stdlib::int64_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api: *mut x264_api_t = h as *mut x264_api_t;
        return (*api)
            .encoder_invalidate_reference
            .expect("non-null function pointer")((*api).x264, pts);
    }
}
