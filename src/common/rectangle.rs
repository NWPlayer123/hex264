use core::ffi::{c_int, c_void};

use crate::rectangle_h::x264_macroblock_cache_rect;
use crate::stdint_uintn_h::uint32_t;
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_4_4(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 4 as c_int * 4 as c_int, 4 as c_int, 4 as c_int, val);
}
#[no_mangle]
#[c2rust::src_loc = "56:1"]
static mut x264_10_cache_mv_func_table: [Option<
    unsafe extern "C" fn(*mut c_void, uint32_t) -> (),
>; 10] = [
    Some(macroblock_cache_mv_1_1 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_mv_2_1 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_mv_1_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_mv_2_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_mv_4_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_mv_2_4 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_mv_4_4 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
];
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_4_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 4 as c_int * 4 as c_int, 2 as c_int, 4 as c_int, val);
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_2_4(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 4 as c_int, 4 as c_int, 4 as c_int, val);
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_1_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 1 as c_int * 4 as c_int, 2 as c_int, 4 as c_int, val);
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_2_1(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 4 as c_int, 1 as c_int, 4 as c_int, val);
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_1_1(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 1 as c_int * 4 as c_int, 1 as c_int, 4 as c_int, val);
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_2_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 4 as c_int, 2 as c_int, 4 as c_int, val);
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_1_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 1 as c_int * 2 as c_int, 2 as c_int, 2 as c_int, val);
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_4_4(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 4 as c_int * 2 as c_int, 4 as c_int, 2 as c_int, val);
}
#[no_mangle]
#[c2rust::src_loc = "57:1"]
static mut x264_10_cache_mvd_func_table: [Option<
    unsafe extern "C" fn(*mut c_void, uint32_t) -> (),
>; 10] = [
    Some(macroblock_cache_mvd_1_1 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_mvd_2_1 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_mvd_1_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_mvd_2_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_mvd_4_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_mvd_2_4 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_mvd_4_4 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
];
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_2_4(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 2 as c_int, 4 as c_int, 2 as c_int, val);
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_4_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 4 as c_int * 2 as c_int, 2 as c_int, 2 as c_int, val);
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_2_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 2 as c_int, 2 as c_int, 2 as c_int, val);
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_2_1(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 2 as c_int, 1 as c_int, 2 as c_int, val);
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_1_1(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 1 as c_int * 2 as c_int, 1 as c_int, 2 as c_int, val);
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_4_4(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 4 as c_int * 1 as c_int, 4 as c_int, 1 as c_int, val);
}
#[no_mangle]
#[c2rust::src_loc = "58:1"]
static mut x264_10_cache_ref_func_table: [Option<
    unsafe extern "C" fn(*mut c_void, uint32_t) -> (),
>; 10] = [
    Some(macroblock_cache_ref_1_1 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_ref_2_1 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_ref_1_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    Some(macroblock_cache_ref_2_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_ref_4_2 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_ref_2_4 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
    None,
    Some(macroblock_cache_ref_4_4 as unsafe extern "C" fn(*mut c_void, uint32_t) -> ()),
];
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_4_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 4 as c_int * 1 as c_int, 2 as c_int, 1 as c_int, val);
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_2_4(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 1 as c_int, 4 as c_int, 1 as c_int, val);
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_2_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 1 as c_int, 2 as c_int, 1 as c_int, val);
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_2_1(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 2 as c_int * 1 as c_int, 1 as c_int, 1 as c_int, val);
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_1_2(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 1 as c_int * 1 as c_int, 2 as c_int, 1 as c_int, val);
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_1_1(mut target: *mut c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(target, 1 as c_int * 1 as c_int, 1 as c_int, 1 as c_int, val);
}
