// =============== BEGIN mc_h ================
pub type weight_fn_t = Option<
    unsafe extern "C" fn(
        *mut crate::src::common::common::pixel,
        crate::stdlib::intptr_t,
        *mut crate::src::common::common::pixel,
        crate::stdlib::intptr_t,
        *const crate::src::common::mc::x264_weight_t,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_weight_t {
    pub cachea: [crate::stdlib::int16_t; 8],
    pub cacheb: [crate::stdlib::int16_t; 8],
    pub i_denom: crate::stdlib::int32_t,
    pub i_scale: crate::stdlib::int32_t,
    pub i_offset: crate::stdlib::int32_t,
    pub weightfn: *mut crate::src::common::mc::weight_fn_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_1 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_2 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_3 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_4 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_5 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_6 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_7 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_8 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_9 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_10 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_11 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_12 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_13 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_14 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_15 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_16 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_17 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_18 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_19 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_20 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t_21 {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::intptr_t,
            *mut *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const crate::src::common::mc::x264_weight_t,
        ) -> *mut crate::src::common::common::pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::stdlib::uint32_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::int16_t,
        ) -> (),
    >,
    pub prefetch_fenc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub memzero_aligned:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ()>,
    pub integral_init4h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8h: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init4v: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            crate::stdlib::intptr_t,
        ) -> (),
    >,
    pub integral_init8v:
        Option<unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub weight: *mut crate::src::common::mc::weight_fn_t,
    pub offsetadd: *mut crate::src::common::mc::weight_fn_t,
    pub offsetsub: *mut crate::src::common::mc::weight_fn_t,
    pub weight_cache: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::src::common::mc::x264_weight_t,
        ) -> (),
    >,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::x264_t,
            *mut crate::stdlib::uint16_t,
            *mut [crate::stdlib::int16_t; 2],
            *mut crate::stdlib::int16_t,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint16_t,
            *mut ::core::ffi::c_float,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_unpack: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_float,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
pub mod common_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_clip_pixel(mut x: ::core::ffi::c_int) -> crate::src::common::common::pixel {
        (if x & !crate::src::common::common::PIXEL_MAX != 0 {
            -x >> 31i32 & crate::src::common::common::PIXEL_MAX
        } else {
            x
        }) as crate::src::common::common::pixel
    }
}
pub mod osdep_h {
    #[inline(always)]
    pub unsafe extern "C" fn endian_fix16(mut x: crate::stdlib::uint16_t) -> crate::stdlib::uint16_t {
        ((x as ::core::ffi::c_int) << 8i32 | x as ::core::ffi::c_int >> 8i32) as crate::stdlib::uint16_t
    }
}
use crate::src::common::mc::{common_h::x264_clip_pixel, osdep_h::endian_fix16};
#[inline]
unsafe extern "C" fn pixel_avg(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src1: *mut crate::src::common::common::pixel,
    mut i_src1_stride: crate::stdlib::intptr_t,
    mut src2: *mut crate::src::common::common::pixel,
    mut i_src2_stride: crate::stdlib::intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < i_height {
            let mut x = 0i32;
            while x < i_width {
                *dst.offset(x as isize) = ((*src1.offset(x as isize) as ::core::ffi::c_int
                    + *src2.offset(x as isize) as ::core::ffi::c_int
                    + 1i32)
                    >> 1i32) as crate::src::common::common::pixel;
                x += 1;
            }
            dst = dst.offset(i_dst_stride);
            src1 = src1.offset(i_src1_stride);
            src2 = src2.offset(i_src2_stride);
            y += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn pixel_avg_wxh(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut src1: *mut crate::src::common::common::pixel,
    mut i_src1: crate::stdlib::intptr_t,
    mut src2: *mut crate::src::common::common::pixel,
    mut i_src2: crate::stdlib::intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < height {
            let mut x = 0i32;
            while x < width {
                *dst.offset(x as isize) = ((*src1.offset(x as isize) as ::core::ffi::c_int
                    + *src2.offset(x as isize) as ::core::ffi::c_int
                    + 1i32)
                    >> 1i32) as crate::src::common::common::pixel;
                x += 1;
            }
            src1 = src1.offset(i_src1);
            src2 = src2.offset(i_src2);
            dst = dst.offset(i_dst);
            y += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn pixel_avg_weight_wxh(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut src1: *mut crate::src::common::common::pixel,
    mut i_src1: crate::stdlib::intptr_t,
    mut src2: *mut crate::src::common::common::pixel,
    mut i_src2: crate::stdlib::intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut i_weight1: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        let mut i_weight2 = 64i32 - i_weight1;
        while y < height {
            let mut x = 0i32;
            while x < width {
                *dst.offset(x as isize) = x264_clip_pixel(
                    (*src1.offset(x as isize) as ::core::ffi::c_int * i_weight1
                        + *src2.offset(x as isize) as ::core::ffi::c_int * i_weight2
                        + ((1i32) << 5i32))
                        >> 6i32,
                );
                x += 1;
            }
            y += 1;
            dst = dst.offset(i_dst);
            src1 = src1.offset(i_src1);
            src2 = src2.offset(i_src2);
        }
    }
}
unsafe extern "C" fn pixel_avg_16x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 16i32, 16i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                16i32,
                16i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_16x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 16i32, 8i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                16i32,
                8i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_8x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 8i32, 16i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                8i32,
                16i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 8i32, 8i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                8i32,
                8i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_8x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 8i32, 4i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                8i32,
                4i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_4x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 4i32, 16i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                4i32,
                16i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_4x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 4i32, 8i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                4i32,
                8i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_4x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 4i32, 4i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                4i32,
                4i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_4x2(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 4i32, 2i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                4i32,
                2i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_2x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 2i32, 8i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                2i32,
                8i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_2x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 2i32, 4i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                2i32,
                4i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn pixel_avg_2x2(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride_pix3: crate::stdlib::intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    unsafe {
        if weight == 32i32 {
            pixel_avg_wxh(pix1, i_stride_pix1, pix2, i_stride_pix2, pix3, i_stride_pix3, 2i32, 2i32);
        } else {
            pixel_avg_weight_wxh(
                pix1,
                i_stride_pix1,
                pix2,
                i_stride_pix2,
                pix3,
                i_stride_pix3,
                2i32,
                2i32,
                weight,
            );
        };
    }
}
unsafe extern "C" fn weight_cache(
    mut h: *mut crate::src::common::common::x264_t,
    mut w: *mut crate::src::common::mc::x264_weight_t,
) {
    unsafe {
        (*w).weightfn = (*h).mc.weight;
    }
}
unsafe extern "C" fn mc_weight(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut weight: *const crate::src::common::mc::x264_weight_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) {
    unsafe {
        let mut offset = (*weight).i_offset * ((1i32) << (crate::internal::BIT_DEPTH - 8i32));
        let mut scale = (*weight).i_scale;
        let mut denom = (*weight).i_denom;
        if denom >= 1i32 {
            let mut y = 0i32;
            while y < i_height {
                let mut x = 0i32;
                while x < i_width {
                    *dst.offset(x as isize) = x264_clip_pixel(
                        ((*src.offset(x as isize) as ::core::ffi::c_int * scale
                            + ((1i32) << (denom - 1i32)))
                            >> denom)
                            + offset,
                    );
                    x += 1;
                }
                y += 1;
                dst = dst.offset(i_dst_stride);
                src = src.offset(i_src_stride);
            }
        } else {
            let mut y_0 = 0i32;
            while y_0 < i_height {
                let mut x_0 = 0i32;
                while x_0 < i_width {
                    *dst.offset(x_0 as isize) =
                        x264_clip_pixel(*src.offset(x_0 as isize) as ::core::ffi::c_int * scale + offset);
                    x_0 += 1;
                }
                y_0 += 1;
                dst = dst.offset(i_dst_stride);
                src = src.offset(i_src_stride);
            }
        };
    }
}
unsafe extern "C" fn mc_weight_w20(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut weight: *const crate::src::common::mc::x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 20i32, height);
    }
}
unsafe extern "C" fn mc_weight_w16(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut weight: *const crate::src::common::mc::x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 16i32, height);
    }
}
unsafe extern "C" fn mc_weight_w12(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut weight: *const crate::src::common::mc::x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 12i32, height);
    }
}
unsafe extern "C" fn mc_weight_w8(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut weight: *const crate::src::common::mc::x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 8i32, height);
    }
}
unsafe extern "C" fn mc_weight_w4(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut weight: *const crate::src::common::mc::x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 4i32, height);
    }
}
unsafe extern "C" fn mc_weight_w2(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut weight: *const crate::src::common::mc::x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 2i32, height);
    }
}
static mut mc_weight_wtab: [crate::src::common::mc::weight_fn_t; 6] = [
    Some(
        mc_weight_w2
            as unsafe extern "C" fn(
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *const crate::src::common::mc::x264_weight_t,
                ::core::ffi::c_int,
            ) -> (),
    ),
    Some(
        mc_weight_w4
            as unsafe extern "C" fn(
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *const crate::src::common::mc::x264_weight_t,
                ::core::ffi::c_int,
            ) -> (),
    ),
    Some(
        mc_weight_w8
            as unsafe extern "C" fn(
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *const crate::src::common::mc::x264_weight_t,
                ::core::ffi::c_int,
            ) -> (),
    ),
    Some(
        mc_weight_w12
            as unsafe extern "C" fn(
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *const crate::src::common::mc::x264_weight_t,
                ::core::ffi::c_int,
            ) -> (),
    ),
    Some(
        mc_weight_w16
            as unsafe extern "C" fn(
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *const crate::src::common::mc::x264_weight_t,
                ::core::ffi::c_int,
            ) -> (),
    ),
    Some(
        mc_weight_w20
            as unsafe extern "C" fn(
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *mut crate::src::common::common::pixel,
                crate::stdlib::intptr_t,
                *const crate::src::common::mc::x264_weight_t,
                ::core::ffi::c_int,
            ) -> (),
    ),
];
unsafe extern "C" fn mc_copy(
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < i_height {
            crate::stdlib::memcpy(
                dst as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                (i_width * crate::src::common::common::SIZEOF_PIXEL) as crate::__stddef_size_t_h::size_t,
            );
            src = src.offset(i_src_stride);
            dst = dst.offset(i_dst_stride);
            y += 1;
        }
    }
}
unsafe extern "C" fn hpel_filter(
    mut dsth: *mut crate::src::common::common::pixel,
    mut dstv: *mut crate::src::common::common::pixel,
    mut dstc: *mut crate::src::common::common::pixel,
    mut src: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut buf: *mut crate::stdlib::int16_t,
) {
    unsafe {
        let mut y = 0i32;
        let pad = if crate::internal::BIT_DEPTH > 9i32 {
            -(10i32) * crate::src::common::common::PIXEL_MAX
        } else {
            0i32
        };
        while y < height {
            let mut x_0 = 0i32;
            let mut x_1 = 0i32;
            let mut x = -(2i32);
            while x < width + 3i32 {
                let mut v = *src.offset(x as crate::stdlib::intptr_t - 2isize * stride) as ::core::ffi::c_int
                    + *src.offset(x as crate::stdlib::intptr_t + 3isize * stride) as ::core::ffi::c_int
                    - 5i32
                        * (*src.offset(x as crate::stdlib::intptr_t - stride) as ::core::ffi::c_int
                            + *src.offset(x as crate::stdlib::intptr_t + 2isize * stride)
                                as ::core::ffi::c_int)
                    + 20i32
                        * (*src.offset(x as isize) as ::core::ffi::c_int
                            + *src.offset(x as crate::stdlib::intptr_t + stride) as ::core::ffi::c_int);
                *dstv.offset(x as isize) = x264_clip_pixel((v + 16i32) >> 5i32);
                *buf.offset((x + 2i32) as isize) = (v + pad) as crate::stdlib::int16_t;
                x += 1;
            }
            while x_0 < width {
                *dstc.offset(x_0 as isize) = x264_clip_pixel(
                    (*buf.offset(2isize).offset((x_0 - 2i32 * 1i32) as isize) as ::core::ffi::c_int
                        + *buf.offset(2isize).offset((x_0 + 3i32 * 1i32) as isize) as ::core::ffi::c_int
                        - 5i32
                            * (*buf.offset(2isize).offset((x_0 - 1i32) as isize) as ::core::ffi::c_int
                                + *buf.offset(2isize).offset((x_0 + 2i32 * 1i32) as isize)
                                    as ::core::ffi::c_int)
                        + 20i32
                            * (*buf.offset(2isize).offset(x_0 as isize) as ::core::ffi::c_int
                                + *buf.offset(2isize).offset((x_0 + 1i32) as isize) as ::core::ffi::c_int)
                        - 32i32 * pad
                        + 512i32)
                        >> 10i32,
                );
                x_0 += 1;
            }
            while x_1 < width {
                *dsth.offset(x_1 as isize) = x264_clip_pixel(
                    (*src.offset((x_1 - 2i32 * 1i32) as isize) as ::core::ffi::c_int
                        + *src.offset((x_1 + 3i32 * 1i32) as isize) as ::core::ffi::c_int
                        - 5i32
                            * (*src.offset((x_1 - 1i32) as isize) as ::core::ffi::c_int
                                + *src.offset((x_1 + 2i32 * 1i32) as isize) as ::core::ffi::c_int)
                        + 20i32
                            * (*src.offset(x_1 as isize) as ::core::ffi::c_int
                                + *src.offset((x_1 + 1i32) as isize) as ::core::ffi::c_int)
                        + 16i32)
                        >> 5i32,
                );
                x_1 += 1;
            }
            dsth = dsth.offset(stride);
            dstv = dstv.offset(stride);
            dstc = dstc.offset(stride);
            src = src.offset(stride);
            y += 1;
        }
    }
}
unsafe extern "C" fn mc_luma(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut mvx: ::core::ffi::c_int,
    mut mvy: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut weight: *const crate::src::common::mc::x264_weight_t,
) {
    unsafe {
        let mut qpel_idx = ((mvy & 3i32) << 2i32) + (mvx & 3i32);
        let mut offset = ((mvy >> 2i32) as crate::stdlib::intptr_t * i_src_stride
            + (mvx >> 2i32) as crate::stdlib::intptr_t) as ::core::ffi::c_int;
        let mut src1 = (*src.offset(crate::src::common::tables::x264_hpel_ref0[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset((mvy & 3i32 == 3i32) as ::core::ffi::c_int as crate::stdlib::intptr_t * i_src_stride);
        if qpel_idx & 5i32 != 0 {
            let mut src2 = (*src
                .offset(crate::src::common::tables::x264_hpel_ref1[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset((mvx & 3i32 == 3i32) as ::core::ffi::c_int as isize);
            pixel_avg(dst, i_dst_stride, src1, i_src_stride, src2, i_src_stride, i_width, i_height);
            if !(*weight).weightfn.is_null() {
                mc_weight(dst, i_dst_stride, dst, i_dst_stride, weight, i_width, i_height);
            }
        } else if !(*weight).weightfn.is_null() {
            mc_weight(dst, i_dst_stride, src1, i_src_stride, weight, i_width, i_height);
        } else {
            mc_copy(src1, i_src_stride, dst, i_dst_stride, i_width, i_height);
        };
    }
}
unsafe extern "C" fn get_ref(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: *mut crate::stdlib::intptr_t,
    mut src: *mut *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut mvx: ::core::ffi::c_int,
    mut mvy: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut weight: *const crate::src::common::mc::x264_weight_t,
) -> *mut crate::src::common::common::pixel {
    unsafe {
        let mut qpel_idx = ((mvy & 3i32) << 2i32) + (mvx & 3i32);
        let mut offset = ((mvy >> 2i32) as crate::stdlib::intptr_t * i_src_stride
            + (mvx >> 2i32) as crate::stdlib::intptr_t) as ::core::ffi::c_int;
        let mut src1 = (*src.offset(crate::src::common::tables::x264_hpel_ref0[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset((mvy & 3i32 == 3i32) as ::core::ffi::c_int as crate::stdlib::intptr_t * i_src_stride);
        if qpel_idx & 5i32 != 0 {
            let mut src2 = (*src
                .offset(crate::src::common::tables::x264_hpel_ref1[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset((mvx & 3i32 == 3i32) as ::core::ffi::c_int as isize);
            pixel_avg(dst, *i_dst_stride, src1, i_src_stride, src2, i_src_stride, i_width, i_height);
            if !(*weight).weightfn.is_null() {
                mc_weight(dst, *i_dst_stride, dst, *i_dst_stride, weight, i_width, i_height);
            }
            dst
        } else if !(*weight).weightfn.is_null() {
            mc_weight(dst, *i_dst_stride, src1, i_src_stride, weight, i_width, i_height);
            dst
        } else {
            *i_dst_stride = i_src_stride;
            src1
        }
    }
}
unsafe extern "C" fn mc_chroma(
    mut dstu: *mut crate::src::common::common::pixel,
    mut dstv: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut mvx: ::core::ffi::c_int,
    mut mvy: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        let mut d8x = mvx & 0x7i32;
        let mut d8y = mvy & 0x7i32;
        let mut cA = (8i32 - d8x) * (8i32 - d8y);
        let mut cB = d8x * (8i32 - d8y);
        let mut cC = (8i32 - d8x) * d8y;
        let mut cD = d8x * d8y;
        src = src.offset(
            (mvy >> 3i32) as crate::stdlib::intptr_t * i_src_stride
                + ((mvx >> 3i32) * 2i32) as crate::stdlib::intptr_t,
        );
        let mut srcp = src.offset(i_src_stride);
        while y < i_height {
            let mut x = 0i32;
            while x < i_width {
                *dstu.offset(x as isize) = ((cA * *src.offset((2i32 * x) as isize) as ::core::ffi::c_int
                    + cB * *src.offset((2i32 * x + 2i32) as isize) as ::core::ffi::c_int
                    + cC * *srcp.offset((2i32 * x) as isize) as ::core::ffi::c_int
                    + cD * *srcp.offset((2i32 * x + 2i32) as isize) as ::core::ffi::c_int
                    + 32i32)
                    >> 6i32) as crate::src::common::common::pixel;
                *dstv.offset(x as isize) = ((cA
                    * *src.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                    + cB * *src.offset((2i32 * x + 3i32) as isize) as ::core::ffi::c_int
                    + cC * *srcp.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                    + cD * *srcp.offset((2i32 * x + 3i32) as isize) as ::core::ffi::c_int
                    + 32i32)
                    >> 6i32) as crate::src::common::common::pixel;
                x += 1;
            }
            dstu = dstu.offset(i_dst_stride);
            dstv = dstv.offset(i_dst_stride);
            src = srcp;
            srcp = srcp.offset(i_src_stride);
            y += 1;
        }
    }
}
unsafe extern "C" fn mc_copy_w16(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut i_height: ::core::ffi::c_int,
) {
    unsafe {
        mc_copy(src, i_src, dst, i_dst, 16i32, i_height);
    }
}
unsafe extern "C" fn mc_copy_w8(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut i_height: ::core::ffi::c_int,
) {
    unsafe {
        mc_copy(src, i_src, dst, i_dst, 8i32, i_height);
    }
}
unsafe extern "C" fn mc_copy_w4(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut i_height: ::core::ffi::c_int,
) {
    unsafe {
        mc_copy(src, i_src, dst, i_dst, 4i32, i_height);
    }
}
pub unsafe extern "C" fn x264_8_plane_copy_c(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    unsafe {
        loop {
            let c2rust_fresh0 = h;
            h -= 1;
            if c2rust_fresh0 == 0 {
                break;
            }
            crate::stdlib::memcpy(
                dst as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                (w * crate::src::common::common::SIZEOF_PIXEL) as crate::__stddef_size_t_h::size_t,
            );
            dst = dst.offset(i_dst);
            src = src.offset(i_src);
        }
    }
}
pub unsafe extern "C" fn x264_8_plane_copy_swap_c(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < h {
            let mut x = 0i32;
            while x < 2i32 * w {
                *dst.offset(x as isize) = *src.offset((x + 1i32) as isize);
                *dst.offset((x + 1i32) as isize) = *src.offset(x as isize);
                x += 2i32;
            }
            y += 1;
            dst = dst.offset(i_dst);
            src = src.offset(i_src);
        }
    }
}
pub unsafe extern "C" fn x264_8_plane_copy_interleave_c(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut srcu: *mut crate::src::common::common::pixel,
    mut i_srcu: crate::stdlib::intptr_t,
    mut srcv: *mut crate::src::common::common::pixel,
    mut i_srcv: crate::stdlib::intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < h {
            let mut x = 0i32;
            while x < w {
                *dst.offset((2i32 * x) as isize) = *srcu.offset(x as isize);
                *dst.offset((2i32 * x + 1i32) as isize) = *srcv.offset(x as isize);
                x += 1;
            }
            y += 1;
            dst = dst.offset(i_dst);
            srcu = srcu.offset(i_srcu);
            srcv = srcv.offset(i_srcv);
        }
    }
}
pub unsafe extern "C" fn x264_8_plane_copy_deinterleave_c(
    mut dsta: *mut crate::src::common::common::pixel,
    mut i_dsta: crate::stdlib::intptr_t,
    mut dstb: *mut crate::src::common::common::pixel,
    mut i_dstb: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < h {
            let mut x = 0i32;
            while x < w {
                *dsta.offset(x as isize) = *src.offset((2i32 * x) as isize);
                *dstb.offset(x as isize) = *src.offset((2i32 * x + 1i32) as isize);
                x += 1;
            }
            y += 1;
            dsta = dsta.offset(i_dsta);
            dstb = dstb.offset(i_dstb);
            src = src.offset(i_src);
        }
    }
}
unsafe extern "C" fn plane_copy_deinterleave_rgb_c(
    mut dsta: *mut crate::src::common::common::pixel,
    mut i_dsta: crate::stdlib::intptr_t,
    mut dstb: *mut crate::src::common::common::pixel,
    mut i_dstb: crate::stdlib::intptr_t,
    mut dstc: *mut crate::src::common::common::pixel,
    mut i_dstc: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut pw: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < h {
            let mut x = 0i32;
            while x < w {
                *dsta.offset(x as isize) = *src.offset((x * pw) as isize);
                *dstb.offset(x as isize) = *src.offset((x * pw + 1i32) as isize);
                *dstc.offset(x as isize) = *src.offset((x * pw + 2i32) as isize);
                x += 1;
            }
            y += 1;
            dsta = dsta.offset(i_dsta);
            dstb = dstb.offset(i_dstb);
            dstc = dstc.offset(i_dstc);
            src = src.offset(i_src);
        }
    }
}
unsafe extern "C" fn plane_copy_deinterleave_v210_c(
    mut dsty: *mut crate::src::common::common::pixel,
    mut i_dsty: crate::stdlib::intptr_t,
    mut dstc: *mut crate::src::common::common::pixel,
    mut i_dstc: crate::stdlib::intptr_t,
    mut src: *mut crate::stdlib::uint32_t,
    mut i_src: crate::stdlib::intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    unsafe {
        let mut l = 0i32;
        while l < h {
            let mut n = 0i32;
            let mut dsty0 = dsty;
            let mut dstc0 = dstc;
            let mut src0 = src;
            while n < w {
                let c2rust_fresh1 = src0;
                src0 = src0.offset(1);
                let mut s = *c2rust_fresh1;
                let c2rust_fresh2 = dstc0;
                dstc0 = dstc0.offset(1);
                *c2rust_fresh2 = (s & 0x3FFu32) as crate::src::common::common::pixel;
                let c2rust_fresh3 = dsty0;
                dsty0 = dsty0.offset(1);
                *c2rust_fresh3 = (s >> 10i32 & 0x3FFu32) as crate::src::common::common::pixel;
                let c2rust_fresh4 = dstc0;
                dstc0 = dstc0.offset(1);
                *c2rust_fresh4 = (s >> 20i32 & 0x3FFu32) as crate::src::common::common::pixel;
                let c2rust_fresh5 = src0;
                src0 = src0.offset(1);
                s = *c2rust_fresh5;
                let c2rust_fresh6 = dsty0;
                dsty0 = dsty0.offset(1);
                *c2rust_fresh6 = (s & 0x3FFu32) as crate::src::common::common::pixel;
                let c2rust_fresh7 = dstc0;
                dstc0 = dstc0.offset(1);
                *c2rust_fresh7 = (s >> 10i32 & 0x3FFu32) as crate::src::common::common::pixel;
                let c2rust_fresh8 = dsty0;
                dsty0 = dsty0.offset(1);
                *c2rust_fresh8 = (s >> 20i32 & 0x3FFu32) as crate::src::common::common::pixel;
                n += 3i32;
            }
            dsty = dsty.offset(i_dsty);
            dstc = dstc.offset(i_dstc);
            src = src.offset(i_src);
            l += 1;
        }
    }
}
unsafe extern "C" fn store_interleave_chroma(
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst: crate::stdlib::intptr_t,
    mut srcu: *mut crate::src::common::common::pixel,
    mut srcv: *mut crate::src::common::common::pixel,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < height {
            let mut x = 0i32;
            while x < 8i32 {
                *dst.offset((2i32 * x) as isize) = *srcu.offset(x as isize);
                *dst.offset((2i32 * x + 1i32) as isize) = *srcv.offset(x as isize);
                x += 1;
            }
            y += 1;
            dst = dst.offset(i_dst);
            srcu = srcu.offset(crate::src::common::common::FDEC_STRIDE as isize);
            srcv = srcv.offset(crate::src::common::common::FDEC_STRIDE as isize);
        }
    }
}
unsafe extern "C" fn load_deinterleave_chroma_fenc(
    mut dst: *mut crate::src::common::common::pixel,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        x264_8_plane_copy_deinterleave_c(
            dst,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            dst.offset((crate::src::common::common::FENC_STRIDE / 2i32) as isize),
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            src,
            i_src,
            8i32,
            height,
        );
    }
}
unsafe extern "C" fn load_deinterleave_chroma_fdec(
    mut dst: *mut crate::src::common::common::pixel,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src: crate::stdlib::intptr_t,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        x264_8_plane_copy_deinterleave_c(
            dst,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            dst.offset((crate::src::common::common::FDEC_STRIDE / 2i32) as isize),
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            src,
            i_src,
            8i32,
            height,
        );
    }
}
unsafe extern "C" fn prefetch_fenc_null(
    mut _pix_y: *mut crate::src::common::common::pixel,
    mut _stride_y: crate::stdlib::intptr_t,
    mut _pix_uv: *mut crate::src::common::common::pixel,
    mut _stride_uv: crate::stdlib::intptr_t,
    mut _mb_x: ::core::ffi::c_int,
) {
}
unsafe extern "C" fn prefetch_ref_null(
    mut _pix: *mut crate::src::common::common::pixel,
    mut _stride: crate::stdlib::intptr_t,
    mut _parity: ::core::ffi::c_int,
) {
}
unsafe extern "C" fn memzero_aligned(
    mut dst: *mut ::core::ffi::c_void,
    mut n: crate::__stddef_size_t_h::size_t,
) {
    unsafe {
        crate::stdlib::memset(dst, 0i32, n);
    }
}
unsafe extern "C" fn integral_init4h(
    mut sum: *mut crate::stdlib::uint16_t,
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) {
    unsafe {
        let mut x = 0i32;
        let mut v = *pix.offset(0isize) as ::core::ffi::c_int
            + *pix.offset(1isize) as ::core::ffi::c_int
            + *pix.offset(2isize) as ::core::ffi::c_int
            + *pix.offset(3isize) as ::core::ffi::c_int;
        while (x as crate::stdlib::intptr_t) < stride - 4isize {
            *sum.offset(x as isize) = (v + *sum.offset(x as crate::stdlib::intptr_t - stride)
                as ::core::ffi::c_int) as crate::stdlib::uint16_t;
            v += *pix.offset((x + 4i32) as isize) as ::core::ffi::c_int
                - *pix.offset(x as isize) as ::core::ffi::c_int;
            x += 1;
        }
    }
}
unsafe extern "C" fn integral_init8h(
    mut sum: *mut crate::stdlib::uint16_t,
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) {
    unsafe {
        let mut x = 0i32;
        let mut v = *pix.offset(0isize) as ::core::ffi::c_int
            + *pix.offset(1isize) as ::core::ffi::c_int
            + *pix.offset(2isize) as ::core::ffi::c_int
            + *pix.offset(3isize) as ::core::ffi::c_int
            + *pix.offset(4isize) as ::core::ffi::c_int
            + *pix.offset(5isize) as ::core::ffi::c_int
            + *pix.offset(6isize) as ::core::ffi::c_int
            + *pix.offset(7isize) as ::core::ffi::c_int;
        while (x as crate::stdlib::intptr_t) < stride - 8isize {
            *sum.offset(x as isize) = (v + *sum.offset(x as crate::stdlib::intptr_t - stride)
                as ::core::ffi::c_int) as crate::stdlib::uint16_t;
            v += *pix.offset((x + 8i32) as isize) as ::core::ffi::c_int
                - *pix.offset(x as isize) as ::core::ffi::c_int;
            x += 1;
        }
    }
}
unsafe extern "C" fn integral_init4v(
    mut sum8: *mut crate::stdlib::uint16_t,
    mut sum4: *mut crate::stdlib::uint16_t,
    mut stride: crate::stdlib::intptr_t,
) {
    unsafe {
        let mut x = 0i32;
        let mut x_0 = 0i32;
        while (x as crate::stdlib::intptr_t) < stride - 8isize {
            *sum4.offset(x as isize) = (*sum8.offset(x as crate::stdlib::intptr_t + 4isize * stride)
                as ::core::ffi::c_int
                - *sum8.offset(x as isize) as ::core::ffi::c_int)
                as crate::stdlib::uint16_t;
            x += 1;
        }
        while (x_0 as crate::stdlib::intptr_t) < stride - 8isize {
            *sum8.offset(x_0 as isize) = (*sum8.offset(x_0 as crate::stdlib::intptr_t + 8isize * stride)
                as ::core::ffi::c_int
                + *sum8.offset(x_0 as crate::stdlib::intptr_t + 8isize * stride + 4isize)
                    as ::core::ffi::c_int
                - *sum8.offset(x_0 as isize) as ::core::ffi::c_int
                - *sum8.offset((x_0 + 4i32) as isize) as ::core::ffi::c_int)
                as crate::stdlib::uint16_t;
            x_0 += 1;
        }
    }
}
unsafe extern "C" fn integral_init8v(
    mut sum8: *mut crate::stdlib::uint16_t,
    mut stride: crate::stdlib::intptr_t,
) {
    unsafe {
        let mut x = 0i32;
        while (x as crate::stdlib::intptr_t) < stride - 8isize {
            *sum8.offset(x as isize) = (*sum8.offset(x as crate::stdlib::intptr_t + 8isize * stride)
                as ::core::ffi::c_int
                - *sum8.offset(x as isize) as ::core::ffi::c_int)
                as crate::stdlib::uint16_t;
            x += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_init_lowres(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        let mut y = 0i32;
        let mut y_0 = 0i32;
        let mut y_1 = 0i32;
        let mut src = (*frame).plane[0usize];
        let mut i_stride = (*frame).i_stride[0usize];
        let mut i_height = (*frame).i_lines[0usize];
        let mut i_width = (*frame).i_width[0usize];
        while y < i_height {
            *src.offset((i_width + y * i_stride) as isize) =
                *src.offset((i_width - 1i32 + y * i_stride) as isize);
            y += 1;
        }
        crate::stdlib::memcpy(
            src.offset((i_stride * i_height) as isize) as *mut ::core::ffi::c_void,
            src.offset((i_stride * (i_height - 1i32)) as isize) as *const ::core::ffi::c_void,
            ((i_width + 1i32) * crate::src::common::common::SIZEOF_PIXEL) as crate::__stddef_size_t_h::size_t,
        );
        (*h).mc.frame_init_lowres_core.expect("non-null function pointer")(
            src,
            (*frame).lowres[0usize],
            (*frame).lowres[1usize],
            (*frame).lowres[2usize],
            (*frame).lowres[3usize],
            i_stride as crate::stdlib::intptr_t,
            (*frame).i_stride_lowres as crate::stdlib::intptr_t,
            (*frame).i_width_lowres,
            (*frame).i_lines_lowres,
        );
        crate::src::common::frame::x264_8_frame_expand_border_lowres(frame);
        crate::stdlib::memset(
            &raw mut (*frame).i_cost_est as *mut ::core::ffi::c_void,
            -(1i32),
            ::core::mem::size_of::<[[::core::ffi::c_int; 18]; 18]>(),
        );
        while y_0 < (*h).param.i_bframe + 2i32 {
            let mut x = 0i32;
            while x < (*h).param.i_bframe + 2i32 {
                *(*frame).i_row_satds[y_0 as usize][x as usize].offset(0isize) = -(1i32);
                x += 1;
            }
            y_0 += 1;
        }
        while y_1 <= ((*h).param.i_bframe != 0) as ::core::ffi::c_int {
            let mut x_0 = 0i32;
            while x_0 <= (*h).param.i_bframe {
                (*(*frame).lowres_mvs[y_1 as usize][x_0 as usize].offset(0isize))[0usize] = 0x7FFFi16;
                x_0 += 1;
            }
            y_1 += 1;
        }
    }
}
unsafe extern "C" fn frame_init_lowres_core(
    mut src0: *mut crate::src::common::common::pixel,
    mut dst0: *mut crate::src::common::common::pixel,
    mut dsth: *mut crate::src::common::common::pixel,
    mut dstv: *mut crate::src::common::common::pixel,
    mut dstc: *mut crate::src::common::common::pixel,
    mut src_stride: crate::stdlib::intptr_t,
    mut dst_stride: crate::stdlib::intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < height {
            let mut x = 0i32;
            let mut src1 = src0.offset(src_stride);
            let mut src2 = src1.offset(src_stride);
            while x < width {
                *dst0.offset(x as isize) = ((((*src0.offset((2i32 * x) as isize) as ::core::ffi::c_int
                    + *src1.offset((2i32 * x) as isize) as ::core::ffi::c_int
                    + 1i32)
                    >> 1i32)
                    + ((*src0.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                        + *src1.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                        + 1i32)
                        >> 1i32)
                    + 1i32)
                    >> 1i32) as crate::src::common::common::pixel;
                *dsth.offset(x as isize) = ((((*src0.offset((2i32 * x + 1i32) as isize)
                    as ::core::ffi::c_int
                    + *src1.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                    + 1i32)
                    >> 1i32)
                    + ((*src0.offset((2i32 * x + 2i32) as isize) as ::core::ffi::c_int
                        + *src1.offset((2i32 * x + 2i32) as isize) as ::core::ffi::c_int
                        + 1i32)
                        >> 1i32)
                    + 1i32)
                    >> 1i32) as crate::src::common::common::pixel;
                *dstv.offset(x as isize) = ((((*src1.offset((2i32 * x) as isize) as ::core::ffi::c_int
                    + *src2.offset((2i32 * x) as isize) as ::core::ffi::c_int
                    + 1i32)
                    >> 1i32)
                    + ((*src1.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                        + *src2.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                        + 1i32)
                        >> 1i32)
                    + 1i32)
                    >> 1i32) as crate::src::common::common::pixel;
                *dstc.offset(x as isize) = ((((*src1.offset((2i32 * x + 1i32) as isize)
                    as ::core::ffi::c_int
                    + *src2.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                    + 1i32)
                    >> 1i32)
                    + ((*src1.offset((2i32 * x + 2i32) as isize) as ::core::ffi::c_int
                        + *src2.offset((2i32 * x + 2i32) as isize) as ::core::ffi::c_int
                        + 1i32)
                        >> 1i32)
                    + 1i32)
                    >> 1i32) as crate::src::common::common::pixel;
                x += 1;
            }
            src0 = src0.offset(src_stride * 2isize);
            dst0 = dst0.offset(dst_stride);
            dsth = dsth.offset(dst_stride);
            dstv = dstv.offset(dst_stride);
            dstc = dstc.offset(dst_stride);
            y += 1;
        }
    }
}
unsafe extern "C" fn mbtree_propagate_cost(
    mut dst: *mut crate::stdlib::int16_t,
    mut propagate_in: *mut crate::stdlib::uint16_t,
    mut intra_costs: *mut crate::stdlib::uint16_t,
    mut inter_costs: *mut crate::stdlib::uint16_t,
    mut inv_qscales: *mut crate::stdlib::uint16_t,
    mut fps_factor: *mut ::core::ffi::c_float,
    mut len: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        let mut fps = *fps_factor;
        while i < len {
            let mut intra_cost = *intra_costs.offset(i as isize) as ::core::ffi::c_int;
            let mut inter_cost = if (*intra_costs.offset(i as isize) as ::core::ffi::c_int)
                < *inter_costs.offset(i as isize) as ::core::ffi::c_int & (((1i32) << 14i32) - 1i32)
            {
                *intra_costs.offset(i as isize) as ::core::ffi::c_int
            } else {
                *inter_costs.offset(i as isize) as ::core::ffi::c_int & (((1i32) << 14i32) - 1i32)
            };
            let mut propagate_intra =
                (intra_cost * *inv_qscales.offset(i as isize) as ::core::ffi::c_int) as ::core::ffi::c_float;
            let mut propagate_amount = *propagate_in.offset(i as isize) as ::core::ffi::c_int
                as ::core::ffi::c_float
                + propagate_intra * fps;
            let mut propagate_num = (intra_cost - inter_cost) as ::core::ffi::c_float;
            let mut propagate_denom = intra_cost as ::core::ffi::c_float;
            *dst.offset(i as isize) = (if ((propagate_amount * propagate_num / propagate_denom + 0.5)
                as ::core::ffi::c_int)
                < 32767i32
            {
                (propagate_amount * propagate_num / propagate_denom + 0.5) as ::core::ffi::c_int
            } else {
                32767i32
            }) as crate::stdlib::int16_t;
            i += 1;
        }
    }
}
unsafe extern "C" fn mbtree_propagate_list(
    mut h: *mut crate::src::common::common::x264_t,
    mut ref_costs: *mut crate::stdlib::uint16_t,
    mut mvs: *mut [crate::stdlib::int16_t; 2],
    mut propagate_amount: *mut crate::stdlib::int16_t,
    mut lowres_costs: *mut crate::stdlib::uint16_t,
    mut bipred_weight: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        let mut stride = (*h).mb.i_mb_stride as ::core::ffi::c_uint;
        let mut width = (*h).mb.i_mb_width as ::core::ffi::c_uint;
        let mut height = (*h).mb.i_mb_height as ::core::ffi::c_uint;
        while i < len {
            let mut lists_used = *lowres_costs.offset(i as isize) as ::core::ffi::c_int
                >> crate::src::common::frame::LOWRES_COST_SHIFT;
            if lists_used & (1i32) << list != 0 {
                let mut listamount = *propagate_amount.offset(i as isize) as ::core::ffi::c_int;
                if lists_used == 3i32 {
                    listamount = (listamount * bipred_weight + 32i32) >> 6i32;
                }
                if (*(&raw mut *mvs.offset(i as isize) as *mut crate::src::common::base::x264_union32_t)).i
                    == 0
                {
                    *ref_costs.offset(
                        (mb_y as ::core::ffi::c_uint)
                            .wrapping_mul(stride)
                            .wrapping_add(i as ::core::ffi::c_uint) as isize,
                    ) = (if *ref_costs.offset(
                        (mb_y as ::core::ffi::c_uint)
                            .wrapping_mul(stride)
                            .wrapping_add(i as ::core::ffi::c_uint) as isize,
                    ) as ::core::ffi::c_int
                        + listamount
                        < ((1i32) << 15i32) - 1i32
                    {
                        *ref_costs.offset(
                            (mb_y as ::core::ffi::c_uint)
                                .wrapping_mul(stride)
                                .wrapping_add(i as ::core::ffi::c_uint) as isize,
                        ) as ::core::ffi::c_int
                            + listamount
                    } else {
                        ((1i32) << 15i32) - 1i32
                    }) as crate::stdlib::uint16_t;
                } else {
                    let mut x = (*mvs.offset(i as isize))[0usize] as ::core::ffi::c_int;
                    let mut y = (*mvs.offset(i as isize))[1usize] as ::core::ffi::c_int;
                    let mut mbx = ((x >> 5i32) + i) as ::core::ffi::c_uint;
                    let mut mby = ((y >> 5i32) + mb_y) as ::core::ffi::c_uint;
                    let mut idx0 = mbx.wrapping_add(mby.wrapping_mul(stride));
                    let mut idx2 = idx0.wrapping_add(stride);
                    x &= 31i32;
                    y &= 31i32;
                    let mut idx0weight = (32i32 - y) * (32i32 - x);
                    let mut idx1weight = (32i32 - y) * x;
                    let mut idx2weight = y * (32i32 - x);
                    let mut idx3weight = y * x;
                    idx0weight = (idx0weight * listamount + 512i32) >> 10i32;
                    idx1weight = (idx1weight * listamount + 512i32) >> 10i32;
                    idx2weight = (idx2weight * listamount + 512i32) >> 10i32;
                    idx3weight = (idx3weight * listamount + 512i32) >> 10i32;
                    if mbx < width.wrapping_sub(1u32) && mby < height.wrapping_sub(1u32) {
                        *ref_costs.offset(idx0.wrapping_add(0u32) as isize) =
                            (if *ref_costs.offset(idx0.wrapping_add(0u32) as isize) as ::core::ffi::c_int
                                + idx0weight
                                < ((1i32) << 15i32) - 1i32
                            {
                                *ref_costs.offset(idx0.wrapping_add(0u32) as isize) as ::core::ffi::c_int
                                    + idx0weight
                            } else {
                                ((1i32) << 15i32) - 1i32
                            }) as crate::stdlib::uint16_t;
                        *ref_costs.offset(idx0.wrapping_add(1u32) as isize) =
                            (if *ref_costs.offset(idx0.wrapping_add(1u32) as isize) as ::core::ffi::c_int
                                + idx1weight
                                < ((1i32) << 15i32) - 1i32
                            {
                                *ref_costs.offset(idx0.wrapping_add(1u32) as isize) as ::core::ffi::c_int
                                    + idx1weight
                            } else {
                                ((1i32) << 15i32) - 1i32
                            }) as crate::stdlib::uint16_t;
                        *ref_costs.offset(idx2.wrapping_add(0u32) as isize) =
                            (if *ref_costs.offset(idx2.wrapping_add(0u32) as isize) as ::core::ffi::c_int
                                + idx2weight
                                < ((1i32) << 15i32) - 1i32
                            {
                                *ref_costs.offset(idx2.wrapping_add(0u32) as isize) as ::core::ffi::c_int
                                    + idx2weight
                            } else {
                                ((1i32) << 15i32) - 1i32
                            }) as crate::stdlib::uint16_t;
                        *ref_costs.offset(idx2.wrapping_add(1u32) as isize) =
                            (if *ref_costs.offset(idx2.wrapping_add(1u32) as isize) as ::core::ffi::c_int
                                + idx3weight
                                < ((1i32) << 15i32) - 1i32
                            {
                                *ref_costs.offset(idx2.wrapping_add(1u32) as isize) as ::core::ffi::c_int
                                    + idx3weight
                            } else {
                                ((1i32) << 15i32) - 1i32
                            }) as crate::stdlib::uint16_t;
                    } else {
                        if mby < height {
                            if mbx < width {
                                *ref_costs.offset(idx0.wrapping_add(0u32) as isize) = (if *ref_costs
                                    .offset(idx0.wrapping_add(0u32) as isize)
                                    as ::core::ffi::c_int
                                    + idx0weight
                                    < ((1i32) << 15i32) - 1i32
                                {
                                    *ref_costs.offset(idx0.wrapping_add(0u32) as isize) as ::core::ffi::c_int
                                        + idx0weight
                                } else {
                                    ((1i32) << 15i32) - 1i32
                                })
                                    as crate::stdlib::uint16_t;
                            }
                            if mbx.wrapping_add(1u32) < width {
                                *ref_costs.offset(idx0.wrapping_add(1u32) as isize) = (if *ref_costs
                                    .offset(idx0.wrapping_add(1u32) as isize)
                                    as ::core::ffi::c_int
                                    + idx1weight
                                    < ((1i32) << 15i32) - 1i32
                                {
                                    *ref_costs.offset(idx0.wrapping_add(1u32) as isize) as ::core::ffi::c_int
                                        + idx1weight
                                } else {
                                    ((1i32) << 15i32) - 1i32
                                })
                                    as crate::stdlib::uint16_t;
                            }
                        }
                        if mby.wrapping_add(1u32) < height {
                            if mbx < width {
                                *ref_costs.offset(idx2.wrapping_add(0u32) as isize) = (if *ref_costs
                                    .offset(idx2.wrapping_add(0u32) as isize)
                                    as ::core::ffi::c_int
                                    + idx2weight
                                    < ((1i32) << 15i32) - 1i32
                                {
                                    *ref_costs.offset(idx2.wrapping_add(0u32) as isize) as ::core::ffi::c_int
                                        + idx2weight
                                } else {
                                    ((1i32) << 15i32) - 1i32
                                })
                                    as crate::stdlib::uint16_t;
                            }
                            if mbx.wrapping_add(1u32) < width {
                                *ref_costs.offset(idx2.wrapping_add(1u32) as isize) = (if *ref_costs
                                    .offset(idx2.wrapping_add(1u32) as isize)
                                    as ::core::ffi::c_int
                                    + idx3weight
                                    < ((1i32) << 15i32) - 1i32
                                {
                                    *ref_costs.offset(idx2.wrapping_add(1u32) as isize) as ::core::ffi::c_int
                                        + idx3weight
                                } else {
                                    ((1i32) << 15i32) - 1i32
                                })
                                    as crate::stdlib::uint16_t;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn mbtree_fix8_pack(
    mut dst: *mut crate::stdlib::uint16_t,
    mut src: *mut ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        while i < count {
            *dst.offset(i as isize) =
                endian_fix16((*src.offset(i as isize) * 256.0f32) as crate::stdlib::uint16_t);
            i += 1;
        }
    }
}
unsafe extern "C" fn mbtree_fix8_unpack(
    mut dst: *mut ::core::ffi::c_float,
    mut src: *mut crate::stdlib::uint16_t,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        while i < count {
            *dst.offset(i as isize) = endian_fix16(*src.offset(i as isize)) as crate::stdlib::int16_t
                as ::core::ffi::c_int as ::core::ffi::c_float
                * (1.0 / 256.0);
            i += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_mc_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::mc::x264_mc_functions_t_6,
    mut cpu_independent: ::core::ffi::c_int,
) {
    unsafe {
        (*pf).mc_luma = Some(
            mc_luma
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *const crate::src::common::mc::x264_weight_t,
                ) -> (),
        );
        (*pf).get_ref = Some(
            get_ref
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::stdlib::intptr_t,
                    *mut *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *const crate::src::common::mc::x264_weight_t,
                ) -> *mut crate::src::common::common::pixel,
        );
        (*pf).mc_chroma = Some(
            mc_chroma
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_4x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_4x2 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_4x2
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_2x8 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_2x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_2x4 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_2x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).avg[crate::src::common::pixel::PIXEL_2x2 as ::core::ffi::c_int as usize] = Some(
            pixel_avg_2x2
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).weight = &raw mut mc_weight_wtab as *mut crate::src::common::mc::weight_fn_t;
        (*pf).offsetadd = &raw mut mc_weight_wtab as *mut crate::src::common::mc::weight_fn_t;
        (*pf).offsetsub = &raw mut mc_weight_wtab as *mut crate::src::common::mc::weight_fn_t;
        (*pf).weight_cache = Some(
            weight_cache
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::x264_t,
                    *mut crate::src::common::mc::x264_weight_t,
                ) -> (),
        );
        (*pf).copy_16x16_unaligned = Some(
            mc_copy_w16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            mc_copy_w16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).copy[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            mc_copy_w8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).copy[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            mc_copy_w4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).store_interleave_chroma = Some(
            store_interleave_chroma
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).load_deinterleave_chroma_fenc = Some(
            load_deinterleave_chroma_fenc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).load_deinterleave_chroma_fdec = Some(
            load_deinterleave_chroma_fdec
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).plane_copy = Some(
            x264_8_plane_copy_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).plane_copy_swap = Some(
            x264_8_plane_copy_swap_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).plane_copy_interleave = Some(
            x264_8_plane_copy_interleave_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).plane_copy_deinterleave = Some(
            x264_8_plane_copy_deinterleave_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).plane_copy_deinterleave_yuyv = Some(
            x264_8_plane_copy_deinterleave_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).plane_copy_deinterleave_rgb = Some(
            plane_copy_deinterleave_rgb_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).plane_copy_deinterleave_v210 = Some(
            plane_copy_deinterleave_v210_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::stdlib::uint32_t,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).hpel_filter = Some(
            hpel_filter
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int16_t,
                ) -> (),
        );
        (*pf).prefetch_fenc_400 = Some(
            prefetch_fenc_null
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).prefetch_fenc_420 = Some(
            prefetch_fenc_null
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).prefetch_fenc_422 = Some(
            prefetch_fenc_null
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).prefetch_ref = Some(
            prefetch_ref_null
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).memcpy_aligned = Some(
            crate::stdlib::memcpy
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    crate::__stddef_size_t_h::size_t,
                ) -> *mut ::core::ffi::c_void,
        );
        (*pf).memzero_aligned = Some(
            memzero_aligned
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> (),
        );
        (*pf).frame_init_lowres_core = Some(
            frame_init_lowres_core
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).integral_init4h = Some(
            integral_init4h
                as unsafe extern "C" fn(
                    *mut crate::stdlib::uint16_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> (),
        );
        (*pf).integral_init8h = Some(
            integral_init8h
                as unsafe extern "C" fn(
                    *mut crate::stdlib::uint16_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> (),
        );
        (*pf).integral_init4v = Some(
            integral_init4v
                as unsafe extern "C" fn(
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::uint16_t,
                    crate::stdlib::intptr_t,
                ) -> (),
        );
        (*pf).integral_init8v = Some(
            integral_init8v
                as unsafe extern "C" fn(*mut crate::stdlib::uint16_t, crate::stdlib::intptr_t) -> (),
        );
        (*pf).mbtree_propagate_cost = Some(
            mbtree_propagate_cost
                as unsafe extern "C" fn(
                    *mut crate::stdlib::int16_t,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::uint16_t,
                    *mut ::core::ffi::c_float,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).mbtree_propagate_list = Some(
            mbtree_propagate_list
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::x264_t,
                    *mut crate::stdlib::uint16_t,
                    *mut [crate::stdlib::int16_t; 2],
                    *mut crate::stdlib::int16_t,
                    *mut crate::stdlib::uint16_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).mbtree_fix8_pack = Some(
            mbtree_fix8_pack
                as unsafe extern "C" fn(
                    *mut crate::stdlib::uint16_t,
                    *mut ::core::ffi::c_float,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).mbtree_fix8_unpack = Some(
            mbtree_fix8_unpack
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_float,
                    *mut crate::stdlib::uint16_t,
                    ::core::ffi::c_int,
                ) -> (),
        );
        if cpu_independent != 0 {
            (*pf).mbtree_propagate_cost = Some(
                mbtree_propagate_cost
                    as unsafe extern "C" fn(
                        *mut crate::stdlib::int16_t,
                        *mut crate::stdlib::uint16_t,
                        *mut crate::stdlib::uint16_t,
                        *mut crate::stdlib::uint16_t,
                        *mut crate::stdlib::uint16_t,
                        *mut ::core::ffi::c_float,
                        ::core::ffi::c_int,
                    ) -> (),
            );
            (*pf).mbtree_propagate_list = Some(
                mbtree_propagate_list
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::x264_t,
                        *mut crate::stdlib::uint16_t,
                        *mut [crate::stdlib::int16_t; 2],
                        *mut crate::stdlib::int16_t,
                        *mut crate::stdlib::uint16_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> (),
            );
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_filter(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut mb_y: ::core::ffi::c_int,
    mut b_end: ::core::ffi::c_int,
) {
    unsafe {
        let mut p = 0i32;
        let b_interlaced = (*h).param.interlaced;
        let mut start = mb_y * 16i32 - 8i32;
        let mut height = (if b_end != 0 {
            (*frame).i_lines[0usize] + 16i32 * (*h).param.interlaced as ::core::ffi::c_int
        } else {
            (mb_y + b_interlaced as ::core::ffi::c_int) * 16i32
        }) + 8i32;
        if mb_y & b_interlaced as ::core::ffi::c_int != 0 {
            return;
        }
        while p < (if (*h).sps.i_chroma_format_idc.is_444() { 3i32 } else { 1i32 }) {
            let mut stride = (*frame).i_stride[p as usize];
            let width = (*frame).i_width[p as usize];
            let mut offs = start * stride - 8i32;
            if !b_interlaced || (*h).mb.adaptive_mbaff {
                (*h).mc.hpel_filter.expect("non-null function pointer")(
                    (*frame).filtered[p as usize][1usize].offset(offs as isize),
                    (*frame).filtered[p as usize][2usize].offset(offs as isize),
                    (*frame).filtered[p as usize][3usize].offset(offs as isize),
                    (*frame).plane[p as usize].offset(offs as isize),
                    stride as crate::stdlib::intptr_t,
                    width + 16i32,
                    height - start,
                    (*h).scratch_buffer as *mut crate::stdlib::int16_t,
                );
            }
            if b_interlaced {
                let mut i = 0i32;
                stride = (*frame).i_stride[p as usize] << 1i32;
                start = ((mb_y * 16i32) >> 1i32) - 8i32;
                let mut height_fld =
                    ((if b_end != 0 { (*frame).i_lines[p as usize] } else { mb_y * 16i32 }) >> 1i32) + 8i32;
                offs = start * stride - 8i32;
                while i < 2i32 {
                    (*h).mc.hpel_filter.expect("non-null function pointer")(
                        (*frame).filtered_fld[p as usize][1usize].offset(offs as isize),
                        (*frame).filtered_fld[p as usize][2usize].offset(offs as isize),
                        (*frame).filtered_fld[p as usize][3usize].offset(offs as isize),
                        (*frame).plane_fld[p as usize].offset(offs as isize),
                        stride as crate::stdlib::intptr_t,
                        width + 16i32,
                        height_fld - start,
                        (*h).scratch_buffer as *mut crate::stdlib::int16_t,
                    );
                    i += 1;
                    offs += (*frame).i_stride[p as usize];
                }
            }
            p += 1;
        }
        if !(*frame).integral.is_null() {
            let mut stride_0 = (*frame).i_stride[0usize];
            if start < 0i32 {
                crate::stdlib::memset(
                    (*frame).integral.offset(-((crate::src::common::frame::PADV * stride_0) as isize)).offset(
                        -((if 32i32
                            > 64i32
                                / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                    as ::core::ffi::c_int
                        {
                            32i32
                        } else {
                            64i32
                                / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                    as ::core::ffi::c_int
                        }) as isize),
                    ) as *mut ::core::ffi::c_void,
                    0i32,
                    (stride_0 as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>()),
                );
                start = -crate::src::common::frame::PADV;
            }
            if b_end != 0 {
                height += crate::src::common::frame::PADV - 9i32;
            }
            let mut y = start;
            while y < height {
                let mut pix = (*frame).plane[0usize].offset((y * stride_0) as isize).offset(
                    -((if 32i32
                        > 64i32
                            / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                    {
                        32i32
                    } else {
                        64i32
                            / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                    }) as isize),
                );
                let mut sum8 = (*frame).integral.offset(((y + 1i32) * stride_0) as isize).offset(
                    -((if 32i32
                        > 64i32
                            / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                    {
                        32i32
                    } else {
                        64i32
                            / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                    }) as isize),
                );
                if (*h).frames.have_sub8x8_esa {
                    let mut sum4 = ::core::ptr::null_mut::<crate::stdlib::uint16_t>();
                    (*h).mc.integral_init4h.expect("non-null function pointer")(
                        sum8,
                        pix,
                        stride_0 as crate::stdlib::intptr_t,
                    );
                    sum8 = sum8.offset(-((8i32 * stride_0) as isize));
                    sum4 = sum8.offset(
                        (stride_0 * ((*frame).i_lines[0usize] + crate::src::common::frame::PADV * 2i32))
                            as isize,
                    );
                    if y >= 8i32 - crate::src::common::frame::PADV {
                        (*h).mc.integral_init4v.expect("non-null function pointer")(
                            sum8,
                            sum4,
                            stride_0 as crate::stdlib::intptr_t,
                        );
                    }
                } else {
                    (*h).mc.integral_init8h.expect("non-null function pointer")(
                        sum8,
                        pix,
                        stride_0 as crate::stdlib::intptr_t,
                    );
                    if y >= 8i32 - crate::src::common::frame::PADV {
                        (*h).mc.integral_init8v.expect("non-null function pointer")(
                            sum8.offset(-((8i32 * stride_0) as isize)),
                            stride_0 as crate::stdlib::intptr_t,
                        );
                    }
                }
                y += 1;
            }
        }
    }
}
