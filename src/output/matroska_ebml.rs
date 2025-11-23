use ::core::mem::size_of;
use core::ffi::{c_char, c_double, c_float, c_int, c_longlong, c_uint, c_ulong, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::osdep_h::x264_is_regular_file;
use crate::stdint_intn_h::{int64_t, int8_t};
use crate::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{fclose, fopen, fseeko, fwrite, stdout, SEEK_SET};
use crate::stdlib_h::{calloc, free, realloc};
use crate::string_h::{memcpy, strcmp, strlen};
use crate::types_h::__off64_t;
use crate::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "48:8"]
struct mk_writer {
    fp: *mut FILE,
    duration_ptr: c_uint,
    root: *mut mk_context,
    cluster: *mut mk_context,
    frame: *mut mk_context,
    freelist: *mut mk_context,
    actlist: *mut mk_context,
    def_duration: int64_t,
    timescale: int64_t,
    cluster_tc_scaled: int64_t,
    frame_tc: int64_t,
    max_frame_tc: int64_t,
    wrote_header: int8_t,
    in_frame: int8_t,
    keyframe: int8_t,
    skippable: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "36:8"]
struct mk_context {
    next: *mut mk_context,
    prev: *mut *mut mk_context,
    parent: *mut mk_context,
    owner: *mut mk_writer,
    id: c_uint,
    data: *mut c_void,
    d_cur: c_uint,
    d_max: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "267:5"]
union C2RustUnnamed {
    f: c_float,
    u: uint32_t,
}
#[c2rust::src_loc = "29:9"]
const CLSIZE: c_int = 1048576 as c_int;
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn mk_create_context(
    mut w: *mut mk_writer,
    mut parent: *mut mk_context,
    mut id: c_uint,
) -> *mut mk_context {
    let mut c: *mut mk_context = 0 as *mut mk_context;
    if !(*w).freelist.is_null() {
        c = (*w).freelist;
        (*w).freelist = (*(*w).freelist).next as *mut mk_context;
    } else {
        c = calloc(1 as size_t, size_of::<mk_context>() as size_t) as *mut mk_context;
        if c.is_null() {
            return 0 as *mut mk_context;
        }
    }
    (*c).parent = parent as *mut mk_context;
    (*c).owner = w;
    (*c).id = id;
    if !(*(*c).owner).actlist.is_null() {
        (*(*(*c).owner).actlist).prev = &mut (*c).next;
    }
    (*c).next = (*(*c).owner).actlist as *mut mk_context;
    (*c).prev = &mut (*(*c).owner).actlist as *mut *mut mk_context;
    (*(*c).owner).actlist = c;
    return c;
}
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn mk_append_context_data(
    mut c: *mut mk_context,
    mut data: *const c_void,
    mut size: c_uint,
) -> c_int {
    let mut ns: c_uint = (*c).d_cur.wrapping_add(size);
    if ns > (*c).d_max {
        let mut dp: *mut c_void = 0 as *mut c_void;
        let mut dn: c_uint = if (*c).d_max != 0 {
            (*c).d_max << 1 as c_int
        } else {
            16 as c_uint
        };
        while ns > dn {
            dn <<= 1 as c_int;
        }
        dp = realloc((*c).data, dn as size_t);
        if dp.is_null() {
            return -1;
        }
        (*c).data = dp;
        (*c).d_max = dn;
    }
    memcpy(
        ((*c).data as *mut uint8_t).offset((*c).d_cur as isize) as *mut c_void,
        data,
        size as size_t,
    );
    (*c).d_cur = ns;
    return 0 as c_int;
}
#[c2rust::src_loc = "121:1"]
unsafe extern "C" fn mk_write_id(mut c: *mut mk_context, mut id: c_uint) -> c_int {
    let mut c_id: [uint8_t; 4] = [
        (id >> 24 as c_int) as uint8_t,
        (id >> 16 as c_int) as uint8_t,
        (id >> 8 as c_int) as uint8_t,
        id as uint8_t,
    ];
    if c_id[0] != 0 {
        return mk_append_context_data(c, c_id.as_mut_ptr() as *const c_void, 4 as c_uint);
    }
    if c_id[1] != 0 {
        return mk_append_context_data(
            c,
            c_id.as_mut_ptr().offset(1) as *const c_void,
            3 as c_uint,
        );
    }
    if c_id[2] != 0 {
        return mk_append_context_data(
            c,
            c_id.as_mut_ptr().offset(2) as *const c_void,
            2 as c_uint,
        );
    }
    return mk_append_context_data(c, c_id.as_mut_ptr().offset(3) as *const c_void, 1 as c_uint);
}
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn mk_write_size(mut c: *mut mk_context, mut size: c_uint) -> c_int {
    let mut c_size: [uint8_t; 5] = [
        0x8 as c_int as uint8_t,
        (size >> 24 as c_int) as uint8_t,
        (size >> 16 as c_int) as uint8_t,
        (size >> 8 as c_int) as uint8_t,
        size as uint8_t,
    ];
    if size < 0x7f as c_uint {
        c_size[4] = (c_size[4] as c_int | 0x80 as c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(4) as *const c_void,
            1 as c_uint,
        );
    }
    if size < 0x3fff as c_uint {
        c_size[3] = (c_size[3] as c_int | 0x40 as c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(3) as *const c_void,
            2 as c_uint,
        );
    }
    if size < 0x1fffff as c_uint {
        c_size[2] = (c_size[2] as c_int | 0x20 as c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(2) as *const c_void,
            3 as c_uint,
        );
    }
    if size < 0xfffffff as c_uint {
        c_size[1] = (c_size[1] as c_int | 0x10 as c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(1) as *const c_void,
            4 as c_uint,
        );
    }
    return mk_append_context_data(c, c_size.as_mut_ptr() as *const c_void, 5 as c_uint);
}
#[c2rust::src_loc = "161:1"]
unsafe extern "C" fn mk_flush_context_id(mut c: *mut mk_context) -> c_int {
    let mut ff: uint8_t = 0xff as uint8_t;
    if (*c).id == 0 {
        return 0 as c_int;
    }
    if mk_write_id((*c).parent as *mut mk_context, (*c).id) < 0 as c_int {
        return -1;
    }
    if mk_append_context_data(
        (*c).parent as *mut mk_context,
        &mut ff as *mut uint8_t as *const c_void,
        1 as c_uint,
    ) < 0 as c_int
    {
        return -1;
    }
    (*c).id = 0 as c_uint;
    return 0 as c_int;
}
#[c2rust::src_loc = "176:1"]
unsafe extern "C" fn mk_flush_context_data(mut c: *mut mk_context) -> c_int {
    if (*c).d_cur == 0 {
        return 0 as c_int;
    }
    if !(*c).parent.is_null() {
        if mk_append_context_data((*c).parent as *mut mk_context, (*c).data, (*c).d_cur)
            < 0 as c_int
        {
            return -1;
        }
    } else if fwrite(
        (*c).data,
        (*c).d_cur as size_t,
        1 as size_t,
        (*(*c).owner).fp,
    ) != 1 as c_ulong
    {
        return -1;
    }
    (*c).d_cur = 0 as c_uint;
    return 0 as c_int;
}
#[c2rust::src_loc = "191:1"]
unsafe extern "C" fn mk_close_context(mut c: *mut mk_context, mut off: *mut c_uint) -> c_int {
    if (*c).id != 0 {
        if mk_write_id((*c).parent as *mut mk_context, (*c).id) < 0 as c_int {
            return -1;
        }
        if mk_write_size((*c).parent as *mut mk_context, (*c).d_cur) < 0 as c_int {
            return -1;
        }
    }
    if !(*c).parent.is_null() && !off.is_null() {
        *off = (*off).wrapping_add((*(*c).parent).d_cur);
    }
    if mk_flush_context_data(c) < 0 as c_int {
        return -1;
    }
    if !(*c).next.is_null() {
        (*(*c).next).prev = (*c).prev;
    }
    *(*c).prev = (*c).next;
    (*c).next = (*(*c).owner).freelist as *mut mk_context;
    (*(*c).owner).freelist = c;
    return 0 as c_int;
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn mk_destroy_contexts(mut w: *mut mk_writer) {
    let mut next: *mut mk_context = 0 as *mut mk_context;
    let mut cur: *mut mk_context = (*w).freelist;
    while !cur.is_null() {
        next = (*cur).next as *mut mk_context;
        free((*cur).data);
        free(cur as *mut c_void);
        cur = next;
    }
    let mut cur_0: *mut mk_context = (*w).actlist;
    while !cur_0.is_null() {
        next = (*cur_0).next as *mut mk_context;
        free((*cur_0).data);
        free(cur_0 as *mut c_void);
        cur_0 = next;
    }
    (*w).root = 0 as *mut mk_context;
    (*w).actlist = (*w).root;
    (*w).freelist = (*w).actlist;
}
#[c2rust::src_loc = "234:1"]
unsafe extern "C" fn mk_write_string(
    mut c: *mut mk_context,
    mut id: c_uint,
    mut str: *const c_char,
) -> c_int {
    let mut len: size_t = strlen(str);
    if mk_write_id(c, id) < 0 as c_int {
        return -1;
    }
    if mk_write_size(c, len as c_uint) < 0 as c_int {
        return -1;
    }
    if mk_append_context_data(c, str as *const c_void, len as c_uint) < 0 as c_int {
        return -1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "244:1"]
unsafe extern "C" fn mk_write_bin(
    mut c: *mut mk_context,
    mut id: c_uint,
    mut data: *const c_void,
    mut size: c_uint,
) -> c_int {
    if mk_write_id(c, id) < 0 as c_int {
        return -1;
    }
    if mk_write_size(c, size) < 0 as c_int {
        return -1;
    }
    if mk_append_context_data(c, data, size) < 0 as c_int {
        return -1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "252:1"]
unsafe extern "C" fn mk_write_uint(
    mut c: *mut mk_context,
    mut id: c_uint,
    mut ui: uint64_t,
) -> c_int {
    let mut c_ui: [uint8_t; 8] = [
        (ui >> 56 as c_int) as uint8_t,
        (ui >> 48 as c_int) as uint8_t,
        (ui >> 40 as c_int) as uint8_t,
        (ui >> 32 as c_int) as uint8_t,
        (ui >> 24 as c_int) as uint8_t,
        (ui >> 16 as c_int) as uint8_t,
        (ui >> 8 as c_int) as uint8_t,
        ui as uint8_t,
    ];
    let mut i: c_uint = 0 as c_uint;
    if mk_write_id(c, id) < 0 as c_int {
        return -1;
    }
    while i < 7 as c_uint && c_ui[i as usize] == 0 {
        i = i.wrapping_add(1);
    }
    if mk_write_size(c, (8 as c_uint).wrapping_sub(i)) < 0 as c_int {
        return -1;
    }
    if mk_append_context_data(
        c,
        c_ui.as_mut_ptr().offset(i as isize) as *const c_void,
        (8 as c_uint).wrapping_sub(i),
    ) < 0 as c_int
    {
        return -1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "265:1"]
unsafe extern "C" fn mk_write_float_raw(mut c: *mut mk_context, mut f: c_float) -> c_int {
    let mut u: C2RustUnnamed = C2RustUnnamed { f: 0. };
    let mut c_f: [uint8_t; 4] = [0; 4];
    u.f = f;
    c_f[0] = (u.u >> 24 as c_int) as uint8_t;
    c_f[1] = (u.u >> 16 as c_int) as uint8_t;
    c_f[2] = (u.u >> 8 as c_int) as uint8_t;
    c_f[3] = u.u as uint8_t;
    return mk_append_context_data(c, c_f.as_mut_ptr() as *const c_void, 4 as c_uint);
}
#[c2rust::src_loc = "283:1"]
unsafe extern "C" fn mk_write_float(
    mut c: *mut mk_context,
    mut id: c_uint,
    mut f: c_float,
) -> c_int {
    if mk_write_id(c, id) < 0 as c_int {
        return -1;
    }
    if mk_write_size(c, 4 as c_uint) < 0 as c_int {
        return -1;
    }
    if mk_write_float_raw(c, f) < 0 as c_int {
        return -1;
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "291:1"]
unsafe extern "C" fn mk_create_writer(mut filename: *const c_char) -> *mut mk_writer {
    let mut w: *mut mk_writer =
        calloc(1 as size_t, size_of::<mk_writer>() as size_t) as *mut mk_writer;
    if w.is_null() {
        return 0 as *mut mk_writer;
    }
    (*w).root = mk_create_context(w, 0 as *mut mk_context, 0 as c_uint);
    if (*w).root.is_null() {
        free(w as *mut c_void);
        return 0 as *mut mk_writer;
    }
    if strcmp(filename, b"-\0" as *const u8 as *const c_char) == 0 {
        (*w).fp = stdout;
    } else {
        (*w).fp = fopen(filename, b"wb\0" as *const u8 as *const c_char) as *mut FILE;
    }
    if (*w).fp.is_null() {
        mk_destroy_contexts(w);
        free(w as *mut c_void);
        return 0 as *mut mk_writer;
    }
    (*w).timescale = 1000000 as int64_t;
    return w;
}
#[no_mangle]
#[c2rust::src_loc = "320:1"]
unsafe extern "C" fn mk_write_header(
    mut w: *mut mk_writer,
    mut writing_app: *const c_char,
    mut codec_id: *const c_char,
    mut codec_private: *const c_void,
    mut codec_private_size: c_uint,
    mut default_frame_duration: int64_t,
    mut timescale: int64_t,
    mut width: c_uint,
    mut height: c_uint,
    mut d_width: c_uint,
    mut d_height: c_uint,
    mut display_size_units: c_int,
    mut stereo_mode: c_int,
) -> c_int {
    let mut c: *mut mk_context = 0 as *mut mk_context;
    let mut ti: *mut mk_context = 0 as *mut mk_context;
    let mut v: *mut mk_context = 0 as *mut mk_context;
    if (*w).wrote_header != 0 {
        return -1;
    }
    (*w).timescale = timescale;
    (*w).def_duration = default_frame_duration;
    c = mk_create_context(w, (*w).root, 0x1a45dfa3 as c_uint);
    if c.is_null() {
        return -1;
    }
    if mk_write_uint(c, 0x4286 as c_uint, 1 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(c, 0x42f7 as c_uint, 1 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(c, 0x42f2 as c_uint, 4 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(c, 0x42f3 as c_uint, 8 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_string(
        c,
        0x4282 as c_uint,
        b"matroska\0" as *const u8 as *const c_char,
    ) < 0 as c_int
    {
        return -1;
    }
    if mk_write_uint(
        c,
        0x4287 as c_uint,
        (if stereo_mode >= 0 as c_int {
            3 as c_int
        } else {
            2 as c_int
        }) as uint64_t,
    ) < 0 as c_int
    {
        return -1;
    }
    if mk_write_uint(c, 0x4285 as c_uint, 2 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_close_context(c, 0 as *mut c_uint) < 0 as c_int {
        return -1;
    }
    c = mk_create_context(w, (*w).root, 0x18538067 as c_uint);
    if c.is_null() {
        return -1;
    }
    if mk_flush_context_id(c) < 0 as c_int {
        return -1;
    }
    if mk_close_context(c, 0 as *mut c_uint) < 0 as c_int {
        return -1;
    }
    c = mk_create_context(w, (*w).root, 0x1549a966 as c_uint);
    if c.is_null() {
        return -1;
    }
    if mk_write_string(
        c,
        0x4d80 as c_uint,
        b"Haali Matroska Writer b0\0" as *const u8 as *const c_char,
    ) < 0 as c_int
    {
        return -1;
    }
    if mk_write_string(c, 0x5741 as c_uint, writing_app) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(c, 0x2ad7b1 as c_uint, (*w).timescale as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_float(c, 0x4489 as c_uint, 0 as c_int as c_float) < 0 as c_int {
        return -1;
    }
    (*w).duration_ptr = (*c).d_cur.wrapping_sub(4 as c_uint);
    if mk_close_context(c, &mut (*w).duration_ptr) < 0 as c_int {
        return -1;
    }
    c = mk_create_context(w, (*w).root, 0x1654ae6b as c_uint);
    if c.is_null() {
        return -1;
    }
    ti = mk_create_context(w, c, 0xae as c_uint);
    if ti.is_null() {
        return -1;
    }
    if mk_write_uint(ti, 0xd7 as c_uint, 1 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(ti, 0x73c5 as c_uint, 1 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(ti, 0x83 as c_uint, 1 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(ti, 0x9c as c_uint, 0 as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_string(ti, 0x86 as c_uint, codec_id) < 0 as c_int {
        return -1;
    }
    if codec_private_size != 0 {
        if mk_write_bin(ti, 0x63a2 as c_uint, codec_private, codec_private_size) < 0 as c_int {
            return -1;
        }
    }
    if default_frame_duration != 0 {
        if mk_write_uint(ti, 0x23e383 as c_uint, default_frame_duration as uint64_t) < 0 as c_int {
            return -1;
        }
    }
    v = mk_create_context(w, ti, 0xe0 as c_uint);
    if v.is_null() {
        return -1;
    }
    if mk_write_uint(v, 0xb0 as c_uint, width as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(v, 0xba as c_uint, height as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(v, 0x54b2 as c_uint, display_size_units as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(v, 0x54b0 as c_uint, d_width as uint64_t) < 0 as c_int {
        return -1;
    }
    if mk_write_uint(v, 0x54ba as c_uint, d_height as uint64_t) < 0 as c_int {
        return -1;
    }
    if stereo_mode >= 0 as c_int {
        if mk_write_uint(v, 0x53b8 as c_uint, stereo_mode as uint64_t) < 0 as c_int {
            return -1;
        }
    }
    if mk_close_context(v, 0 as *mut c_uint) < 0 as c_int {
        return -1;
    }
    if mk_close_context(ti, 0 as *mut c_uint) < 0 as c_int {
        return -1;
    }
    if mk_close_context(c, 0 as *mut c_uint) < 0 as c_int {
        return -1;
    }
    if mk_flush_context_data((*w).root) < 0 as c_int {
        return -1;
    }
    (*w).wrote_header = 1 as int8_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "397:1"]
unsafe extern "C" fn mk_close_cluster(mut w: *mut mk_writer) -> c_int {
    if (*w).cluster.is_null() {
        return 0 as c_int;
    }
    if mk_close_context((*w).cluster, 0 as *mut c_uint) < 0 as c_int {
        return -1;
    }
    (*w).cluster = 0 as *mut mk_context;
    if mk_flush_context_data((*w).root) < 0 as c_int {
        return -1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "407:1"]
unsafe extern "C" fn mk_flush_frame(mut w: *mut mk_writer) -> c_int {
    let mut delta: int64_t = 0;
    let mut fsize: c_uint = 0;
    let mut c_delta_flags: [uint8_t; 3] = [0; 3];
    if (*w).in_frame == 0 {
        return 0 as c_int;
    }
    delta = (*w).frame_tc / (*w).timescale - (*w).cluster_tc_scaled;
    if delta as c_longlong > 32767 as c_longlong || (delta as c_longlong) < -(32768 as c_longlong) {
        if mk_close_cluster(w) < 0 as c_int {
            return -1;
        }
    }
    if (*w).cluster.is_null() {
        (*w).cluster_tc_scaled = (*w).frame_tc / (*w).timescale;
        (*w).cluster = mk_create_context(w, (*w).root, 0x1f43b675 as c_uint);
        if (*w).cluster.is_null() {
            return -1;
        }
        if mk_write_uint(
            (*w).cluster,
            0xe7 as c_uint,
            (*w).cluster_tc_scaled as uint64_t,
        ) < 0 as c_int
        {
            return -1;
        }
        delta = 0 as int64_t;
    }
    fsize = if !(*w).frame.is_null() {
        (*(*w).frame).d_cur
    } else {
        0 as c_uint
    };
    if mk_write_id((*w).cluster, 0xa3 as c_uint) < 0 as c_int {
        return -1;
    }
    if mk_write_size((*w).cluster, fsize.wrapping_add(4 as c_uint)) < 0 as c_int {
        return -1;
    }
    if mk_write_size((*w).cluster, 1 as c_uint) < 0 as c_int {
        return -1;
    }
    c_delta_flags[0] = (delta >> 8 as c_int) as uint8_t;
    c_delta_flags[1] = delta as uint8_t;
    c_delta_flags[2] =
        (((*w).keyframe as c_int) << 7 as c_int | (*w).skippable as c_int) as uint8_t;
    if mk_append_context_data(
        (*w).cluster,
        c_delta_flags.as_mut_ptr() as *const c_void,
        3 as c_uint,
    ) < 0 as c_int
    {
        return -1;
    }
    if !(*w).frame.is_null() {
        if mk_append_context_data((*w).cluster, (*(*w).frame).data, (*(*w).frame).d_cur)
            < 0 as c_int
        {
            return -1;
        }
        (*(*w).frame).d_cur = 0 as c_uint;
    }
    (*w).in_frame = 0 as int8_t;
    if (*(*w).cluster).d_cur > CLSIZE as c_uint {
        if mk_close_cluster(w) < 0 as c_int {
            return -1;
        }
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "456:1"]
unsafe extern "C" fn mk_start_frame(mut w: *mut mk_writer) -> c_int {
    if mk_flush_frame(w) < 0 as c_int {
        return -1;
    }
    (*w).in_frame = 1 as int8_t;
    (*w).keyframe = 0 as int8_t;
    (*w).skippable = 0 as int8_t;
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "468:1"]
unsafe extern "C" fn mk_set_frame_flags(
    mut w: *mut mk_writer,
    mut timestamp: int64_t,
    mut keyframe: c_int,
    mut skippable: c_int,
) -> c_int {
    if (*w).in_frame == 0 {
        return -1;
    }
    (*w).frame_tc = timestamp;
    (*w).keyframe = (keyframe != 0 as c_int) as c_int as int8_t;
    (*w).skippable = (skippable != 0 as c_int) as c_int as int8_t;
    if (*w).max_frame_tc < timestamp {
        (*w).max_frame_tc = timestamp;
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "483:1"]
unsafe extern "C" fn mk_add_frame_data(
    mut w: *mut mk_writer,
    mut data: *const c_void,
    mut size: c_uint,
) -> c_int {
    if (*w).in_frame == 0 {
        return -1;
    }
    if (*w).frame.is_null() {
        (*w).frame = mk_create_context(w, 0 as *mut mk_context, 0 as c_uint);
        if (*w).frame.is_null() {
            return -1;
        }
    }
    return mk_append_context_data((*w).frame, data, size);
}
#[no_mangle]
#[c2rust::src_loc = "495:1"]
unsafe extern "C" fn mk_close(mut w: *mut mk_writer, mut last_delta: int64_t) -> c_int {
    let mut ret: c_int = 0 as c_int;
    if mk_flush_frame(w) < 0 as c_int || mk_close_cluster(w) < 0 as c_int {
        ret = -1;
    }
    if (*w).wrote_header as c_int != 0 && x264_is_regular_file((*w).fp) != 0 {
        let mut last_frametime: int64_t = if (*w).def_duration != 0 {
            (*w).def_duration
        } else {
            last_delta
        };
        let mut total_duration: int64_t = (*w).max_frame_tc + last_frametime;
        if fseeko((*w).fp, (*w).duration_ptr as __off64_t, SEEK_SET) != 0
            || mk_write_float_raw(
                (*w).root,
                (total_duration as c_double / (*w).timescale as c_double) as c_float,
            ) < 0 as c_int
            || mk_flush_context_data((*w).root) < 0 as c_int
        {
            ret = -1;
        }
    }
    mk_destroy_contexts(w);
    fclose((*w).fp);
    free(w as *mut c_void);
    return ret;
}
