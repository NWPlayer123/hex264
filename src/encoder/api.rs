#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_api_t<'a> {
    pub x264: *mut crate::src::common::common::x264_t<'a>,
    pub nal_encode: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t<'_>,
            *mut crate::stdlib::uint8_t,
            *mut crate::x264_h::x264_nal_t,
        ) -> (),
    >,
    pub encoder_reconfig: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t<'_>,
            *mut crate::x264_h::x264_param_t,
        ) -> ::core::ffi::c_int,
    >,
    pub encoder_parameters: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t<'_>,
            *mut crate::x264_h::x264_param_t,
        ) -> (),
    >,
    pub encoder_headers: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t<'_>,
            *mut *mut crate::x264_h::x264_nal_t,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub encoder_encode: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t<'_>,
            *mut *mut crate::x264_h::x264_nal_t,
            *mut ::core::ffi::c_int,
            *mut crate::x264_h::x264_picture_t,
            *mut crate::x264_h::x264_picture_t,
        ) -> ::core::ffi::c_int,
    >,
    pub encoder_close: Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t<'_>) -> ()>,
    pub encoder_delayed_frames:
        Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t<'_>) -> ::core::ffi::c_int>,
    pub encoder_maximum_delayed_frames:
        Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t<'_>) -> ::core::ffi::c_int>,
    pub encoder_intra_refresh:
        Option<unsafe extern "C" fn(*mut crate::src::common::common::x264_t<'_>) -> ()>,
    pub encoder_invalidate_reference: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t<'_>,
            crate::stdlib::int64_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub static mut x264_chroma_format: ::core::ffi::c_int = crate::x264_config_h::X264_CHROMA_FORMAT;
pub unsafe extern "C" fn x264_encoder_open_165<'a>(
    mut param: *mut crate::x264_h::x264_param_t,
) -> *mut crate::src::common::common::x264_t<'a> {
    unsafe {
        let mut api =
            crate::stdlib::calloc(1usize, ::core::mem::size_of::<x264_api_t<'_>>()) as *mut x264_api_t<'_>;
        if api.is_null() {
            return ::core::ptr::null_mut::<crate::src::common::common::x264_t<'_>>();
        }
        if (*param).i_bitdepth == 8i32 {
            (*api).nal_encode = Some(
                crate::src::common::bitstream::x264_8_nal_encode
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t<'_>,
                        *mut crate::stdlib::uint8_t,
                        *mut crate::x264_h::x264_nal_t,
                    ) -> (),
            );
            (*api).encoder_reconfig = Some(
                crate::src::encoder::encoder::x264_8_encoder_reconfig
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t<'_>,
                        *mut crate::x264_h::x264_param_t,
                    ) -> ::core::ffi::c_int,
            );
            (*api).encoder_parameters = Some(
                crate::src::encoder::encoder::x264_8_encoder_parameters
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t<'_>,
                        *mut crate::x264_h::x264_param_t,
                    ) -> (),
            );
            (*api).encoder_headers = Some(
                crate::src::encoder::encoder::x264_8_encoder_headers
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t<'_>,
                        *mut *mut crate::x264_h::x264_nal_t,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            );
            (*api).encoder_encode = Some(
                crate::src::encoder::encoder::x264_8_encoder_encode
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t<'_>,
                        *mut *mut crate::x264_h::x264_nal_t,
                        *mut ::core::ffi::c_int,
                        *mut crate::x264_h::x264_picture_t,
                        *mut crate::x264_h::x264_picture_t,
                    ) -> ::core::ffi::c_int,
            );
            (*api).encoder_close = Some(
                crate::src::encoder::encoder::x264_8_encoder_close
                    as unsafe extern "C" fn(*mut crate::src::common::common::x264_t<'_>) -> (),
            );
            (*api).encoder_delayed_frames = Some(
                crate::src::encoder::encoder::x264_8_encoder_delayed_frames
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t<'_>,
                    ) -> ::core::ffi::c_int,
            );
            (*api).encoder_maximum_delayed_frames = Some(
                crate::src::encoder::encoder::x264_8_encoder_maximum_delayed_frames
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t<'_>,
                    ) -> ::core::ffi::c_int,
            );
            (*api).encoder_intra_refresh = Some(
                crate::src::encoder::encoder::x264_8_encoder_intra_refresh
                    as unsafe extern "C" fn(*mut crate::src::common::common::x264_t<'_>) -> (),
            );
            (*api).encoder_invalidate_reference = Some(
                crate::src::encoder::encoder::x264_8_encoder_invalidate_reference
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t<'_>,
                        crate::stdlib::int64_t,
                    ) -> ::core::ffi::c_int,
            );
            (*api).x264 =
                crate::src::encoder::encoder::x264_8_encoder_open(param, api as *mut ::core::ffi::c_void);
        } else {
            crate::src::common::base::x264_log_internal(
                crate::x264_h::X264_LOG_ERROR_1,
                b"not compiled with %d bit depth support\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*param).i_bitdepth,
            );
        }
        if (*api).x264.is_null() {
            crate::stdlib::free(api as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<crate::src::common::common::x264_t<'_>>();
        }
        api as *mut crate::src::common::common::x264_t
    }
}
pub unsafe extern "C" fn x264_encoder_close(mut h: *mut crate::src::common::common::x264_t<'_>) {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_close.expect("non-null function pointer")((*api).x264);
        crate::stdlib::free(api as *mut ::core::ffi::c_void);
    }
}
pub unsafe extern "C" fn x264_nal_encode(
    mut h: *mut crate::src::common::common::x264_t<'_>,
    mut dst: *mut crate::stdlib::uint8_t,
    mut nal: *mut crate::x264_h::x264_nal_t,
) {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).nal_encode.expect("non-null function pointer")((*api).x264, dst, nal);
    }
}
pub unsafe extern "C" fn x264_encoder_reconfig(
    mut h: *mut crate::src::common::common::x264_t<'_>,
    mut param: *mut crate::x264_h::x264_param_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_reconfig.expect("non-null function pointer")((*api).x264, param)
    }
}
pub unsafe extern "C" fn x264_encoder_parameters(
    mut h: *mut crate::src::common::common::x264_t<'_>,
    mut param: *mut crate::x264_h::x264_param_t,
) {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_parameters.expect("non-null function pointer")((*api).x264, param);
    }
}
pub unsafe extern "C" fn x264_encoder_headers(
    mut h: *mut crate::src::common::common::x264_t<'_>,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_headers.expect("non-null function pointer")((*api).x264, pp_nal, pi_nal)
    }
}
pub unsafe extern "C" fn x264_encoder_encode(
    mut h: *mut crate::src::common::common::x264_t<'_>,
    mut pp_nal: *mut *mut crate::x264_h::x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_in: *mut crate::x264_h::x264_picture_t,
    mut pic_out: *mut crate::x264_h::x264_picture_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_encode.expect("non-null function pointer")(
            (*api).x264,
            pp_nal,
            pi_nal,
            pic_in,
            pic_out,
        )
    }
}
pub unsafe extern "C" fn x264_encoder_delayed_frames(
    mut h: *mut crate::src::common::common::x264_t<'_>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_delayed_frames.expect("non-null function pointer")((*api).x264)
    }
}
pub unsafe extern "C" fn x264_encoder_maximum_delayed_frames(
    mut h: *mut crate::src::common::common::x264_t<'_>,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_maximum_delayed_frames.expect("non-null function pointer")((*api).x264)
    }
}
pub unsafe extern "C" fn x264_encoder_intra_refresh(mut h: *mut crate::src::common::common::x264_t<'_>) {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_intra_refresh.expect("non-null function pointer")((*api).x264);
    }
}
pub unsafe extern "C" fn x264_encoder_invalidate_reference(
    mut h: *mut crate::src::common::common::x264_t<'_>,
    mut pts: crate::stdlib::int64_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut api = h as *mut x264_api_t<'_>;
        (*api).encoder_invalidate_reference.expect("non-null function pointer")((*api).x264, pts)
    }
}
