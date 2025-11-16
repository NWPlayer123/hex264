use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:26"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = i8;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = i64;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "175:1"]
    pub type __blksize_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "180:1"]
    pub type __blkcnt_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "197:1"]
    pub type __syscall_slong_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:26"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "51:8"]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    #[c2rust::src_loc = "45:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t, __uint64_t};
    extern "C" {
        #[c2rust::src_loc = "40:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "39:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "38:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:26"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:26"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:26"]
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{
        __dev_t, __ino_t, __nlink_t, __mode_t, __uid_t, __gid_t, __off_t, __blksize_t,
        __blkcnt_t, __syscall_slong_t,
    };
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:26"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int8_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:26"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    #[c2rust::src_loc = "110:9"]
    pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::FILE_h::FILE;
    use super::__stddef_size_t_h::size_t;
    use super::types_h::__off64_t;
    extern "C" {
        #[c2rust::src_loc = "150:14"]
        pub static mut stdout: *mut FILE;
        #[c2rust::src_loc = "187:1"]
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "279:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "735:1"]
        pub fn fwrite(
            __ptr: *const ::core::ffi::c_void,
            __size: size_t,
            __n: size_t,
            __s: *mut FILE,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "802:1"]
        pub fn fseeko(
            __stream: *mut FILE,
            __off: __off64_t,
            __whence: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "873:1"]
        pub fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:26"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:26"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "683:1"]
        pub fn realloc(
            __ptr: *mut ::core::ffi::c_void,
            __size: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:26"]
pub mod osdep_h {
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn x264_is_regular_file(
        mut filehandle: *mut FILE,
    ) -> ::core::ffi::c_int {
        let mut file_stat: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if fstat(fileno(filehandle), &mut file_stat) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return (file_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            as ::core::ffi::c_int;
    }
    use super::FILE_h::FILE;
    use super::struct_stat_h::stat;
    use super::types_h::{
        __dev_t, __ino_t, __nlink_t, __mode_t, __uid_t, __gid_t, __off_t, __blksize_t,
        __blkcnt_t, __syscall_slong_t, __time_t,
    };
    use super::struct_timespec_h::timespec;
    use super::stat_h::fstat;
    use super::stdio_h::fileno;
    use super::bits_stat_h::__S_IFMT;
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
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:26"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:26"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{
    __int8_t, __uint8_t, __uint32_t, __int64_t, __uint64_t, __dev_t, __uid_t, __gid_t,
    __ino_t, __mode_t, __nlink_t, __off_t, __off64_t, __time_t, __blksize_t, __blkcnt_t,
    __syscall_slong_t,
};
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::struct_timespec_h::timespec;
pub use self::struct_stat_h::stat;
pub use self::stdint_intn_h::{int8_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t, uint64_t};
pub use self::stdio_h::{SEEK_SET, stdout, fclose, fopen, fwrite, fseeko, fileno};
use self::stat_h::fstat;
use self::stdlib_h::{calloc, realloc, free};
pub use self::osdep_h::x264_is_regular_file;
use self::string_h::{memcpy, strcmp, strlen};
pub use self::__stddef_null_h::NULL;
pub use self::bits_stat_h::__S_IFMT;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "48:8"]
pub struct mk_writer {
    pub fp: *mut FILE,
    pub duration_ptr: ::core::ffi::c_uint,
    pub root: *mut mk_context,
    pub cluster: *mut mk_context,
    pub frame: *mut mk_context,
    pub freelist: *mut mk_context,
    pub actlist: *mut mk_context,
    pub def_duration: int64_t,
    pub timescale: int64_t,
    pub cluster_tc_scaled: int64_t,
    pub frame_tc: int64_t,
    pub max_frame_tc: int64_t,
    pub wrote_header: int8_t,
    pub in_frame: int8_t,
    pub keyframe: int8_t,
    pub skippable: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "36:8"]
pub struct mk_context {
    pub next: *mut mk_context,
    pub prev: *mut *mut mk_context,
    pub parent: *mut mk_context,
    pub owner: *mut mk_writer,
    pub id: ::core::ffi::c_uint,
    pub data: *mut ::core::ffi::c_void,
    pub d_cur: ::core::ffi::c_uint,
    pub d_max: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "267:5"]
pub union C2RustUnnamed {
    pub f: ::core::ffi::c_float,
    pub u: uint32_t,
}
#[c2rust::src_loc = "29:9"]
pub const CLSIZE: ::core::ffi::c_int = 1048576 as ::core::ffi::c_int;
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn mk_create_context(
    mut w: *mut mk_writer,
    mut parent: *mut mk_context,
    mut id: ::core::ffi::c_uint,
) -> *mut mk_context {
    let mut c: *mut mk_context = 0 as *mut mk_context;
    if !(*w).freelist.is_null() {
        c = (*w).freelist;
        (*w).freelist = (*(*w).freelist).next as *mut mk_context;
    } else {
        c = calloc(1 as size_t, ::core::mem::size_of::<mk_context>() as size_t)
            as *mut mk_context;
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
    mut data: *const ::core::ffi::c_void,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ns: ::core::ffi::c_uint = (*c).d_cur.wrapping_add(size);
    if ns > (*c).d_max {
        let mut dp: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
        let mut dn: ::core::ffi::c_uint = if (*c).d_max != 0 {
            (*c).d_max << 1 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_uint
        };
        while ns > dn {
            dn <<= 1 as ::core::ffi::c_int;
        }
        dp = realloc((*c).data, dn as size_t);
        if dp.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        (*c).data = dp;
        (*c).d_max = dn;
    }
    memcpy(
        ((*c).data as *mut uint8_t).offset((*c).d_cur as isize)
            as *mut ::core::ffi::c_void,
        data,
        size as size_t,
    );
    (*c).d_cur = ns;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "121:1"]
unsafe extern "C" fn mk_write_id(
    mut c: *mut mk_context,
    mut id: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut c_id: [uint8_t; 4] = [
        (id >> 24 as ::core::ffi::c_int) as uint8_t,
        (id >> 16 as ::core::ffi::c_int) as uint8_t,
        (id >> 8 as ::core::ffi::c_int) as uint8_t,
        id as uint8_t,
    ];
    if c_id[0 as ::core::ffi::c_int as usize] != 0 {
        return mk_append_context_data(
            c,
            c_id.as_mut_ptr() as *const ::core::ffi::c_void,
            4 as ::core::ffi::c_uint,
        );
    }
    if c_id[1 as ::core::ffi::c_int as usize] != 0 {
        return mk_append_context_data(
            c,
            c_id.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            3 as ::core::ffi::c_uint,
        );
    }
    if c_id[2 as ::core::ffi::c_int as usize] != 0 {
        return mk_append_context_data(
            c,
            c_id.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            2 as ::core::ffi::c_uint,
        );
    }
    return mk_append_context_data(
        c,
        c_id.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        1 as ::core::ffi::c_uint,
    );
}
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn mk_write_size(
    mut c: *mut mk_context,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut c_size: [uint8_t; 5] = [
        0x8 as ::core::ffi::c_int as uint8_t,
        (size >> 24 as ::core::ffi::c_int) as uint8_t,
        (size >> 16 as ::core::ffi::c_int) as uint8_t,
        (size >> 8 as ::core::ffi::c_int) as uint8_t,
        size as uint8_t,
    ];
    if size < 0x7f as ::core::ffi::c_uint {
        c_size[4 as ::core::ffi::c_int as usize] = (c_size[4 as ::core::ffi::c_int
            as usize] as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            1 as ::core::ffi::c_uint,
        );
    }
    if size < 0x3fff as ::core::ffi::c_uint {
        c_size[3 as ::core::ffi::c_int as usize] = (c_size[3 as ::core::ffi::c_int
            as usize] as ::core::ffi::c_int | 0x40 as ::core::ffi::c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            2 as ::core::ffi::c_uint,
        );
    }
    if size < 0x1fffff as ::core::ffi::c_uint {
        c_size[2 as ::core::ffi::c_int as usize] = (c_size[2 as ::core::ffi::c_int
            as usize] as ::core::ffi::c_int | 0x20 as ::core::ffi::c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            3 as ::core::ffi::c_uint,
        );
    }
    if size < 0xfffffff as ::core::ffi::c_uint {
        c_size[1 as ::core::ffi::c_int as usize] = (c_size[1 as ::core::ffi::c_int
            as usize] as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            4 as ::core::ffi::c_uint,
        );
    }
    return mk_append_context_data(
        c,
        c_size.as_mut_ptr() as *const ::core::ffi::c_void,
        5 as ::core::ffi::c_uint,
    );
}
#[c2rust::src_loc = "161:1"]
unsafe extern "C" fn mk_flush_context_id(mut c: *mut mk_context) -> ::core::ffi::c_int {
    let mut ff: uint8_t = 0xff as uint8_t;
    if (*c).id == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if mk_write_id((*c).parent as *mut mk_context, (*c).id) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_append_context_data(
        (*c).parent as *mut mk_context,
        &mut ff as *mut uint8_t as *const ::core::ffi::c_void,
        1 as ::core::ffi::c_uint,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*c).id = 0 as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "176:1"]
unsafe extern "C" fn mk_flush_context_data(
    mut c: *mut mk_context,
) -> ::core::ffi::c_int {
    if (*c).d_cur == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !(*c).parent.is_null() {
        if mk_append_context_data((*c).parent as *mut mk_context, (*c).data, (*c).d_cur)
            < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
    } else if fwrite((*c).data, (*c).d_cur as size_t, 1 as size_t, (*(*c).owner).fp)
        != 1 as ::core::ffi::c_ulong
    {
        return -(1 as ::core::ffi::c_int)
    }
    (*c).d_cur = 0 as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "191:1"]
unsafe extern "C" fn mk_close_context(
    mut c: *mut mk_context,
    mut off: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if (*c).id != 0 {
        if mk_write_id((*c).parent as *mut mk_context, (*c).id) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        if mk_write_size((*c).parent as *mut mk_context, (*c).d_cur)
            < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if !(*c).parent.is_null() && !off.is_null() {
        *off = (*off).wrapping_add((*(*c).parent).d_cur);
    }
    if mk_flush_context_data(c) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*c).next.is_null() {
        (*(*c).next).prev = (*c).prev;
    }
    *(*c).prev = (*c).next;
    (*c).next = (*(*c).owner).freelist as *mut mk_context;
    (*(*c).owner).freelist = c;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn mk_destroy_contexts(mut w: *mut mk_writer) {
    let mut next: *mut mk_context = 0 as *mut mk_context;
    let mut cur: *mut mk_context = (*w).freelist;
    while !cur.is_null() {
        next = (*cur).next as *mut mk_context;
        free((*cur).data);
        free(cur as *mut ::core::ffi::c_void);
        cur = next;
    }
    let mut cur_0: *mut mk_context = (*w).actlist;
    while !cur_0.is_null() {
        next = (*cur_0).next as *mut mk_context;
        free((*cur_0).data);
        free(cur_0 as *mut ::core::ffi::c_void);
        cur_0 = next;
    }
    (*w).root = 0 as *mut mk_context;
    (*w).actlist = (*w).root;
    (*w).freelist = (*w).actlist;
}
#[c2rust::src_loc = "234:1"]
unsafe extern "C" fn mk_write_string(
    mut c: *mut mk_context,
    mut id: ::core::ffi::c_uint,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: size_t = strlen(str);
    if mk_write_id(c, id) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_size(c, len as ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_append_context_data(
        c,
        str as *const ::core::ffi::c_void,
        len as ::core::ffi::c_uint,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "244:1"]
unsafe extern "C" fn mk_write_bin(
    mut c: *mut mk_context,
    mut id: ::core::ffi::c_uint,
    mut data: *const ::core::ffi::c_void,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if mk_write_id(c, id) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_size(c, size) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_append_context_data(c, data, size) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "252:1"]
unsafe extern "C" fn mk_write_uint(
    mut c: *mut mk_context,
    mut id: ::core::ffi::c_uint,
    mut ui: uint64_t,
) -> ::core::ffi::c_int {
    let mut c_ui: [uint8_t; 8] = [
        (ui >> 56 as ::core::ffi::c_int) as uint8_t,
        (ui >> 48 as ::core::ffi::c_int) as uint8_t,
        (ui >> 40 as ::core::ffi::c_int) as uint8_t,
        (ui >> 32 as ::core::ffi::c_int) as uint8_t,
        (ui >> 24 as ::core::ffi::c_int) as uint8_t,
        (ui >> 16 as ::core::ffi::c_int) as uint8_t,
        (ui >> 8 as ::core::ffi::c_int) as uint8_t,
        ui as uint8_t,
    ];
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if mk_write_id(c, id) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    while i < 7 as ::core::ffi::c_uint && c_ui[i as usize] == 0 {
        i = i.wrapping_add(1);
    }
    if mk_write_size(c, (8 as ::core::ffi::c_uint).wrapping_sub(i))
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_append_context_data(
        c,
        c_ui.as_mut_ptr().offset(i as isize) as *const ::core::ffi::c_void,
        (8 as ::core::ffi::c_uint).wrapping_sub(i),
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "265:1"]
unsafe extern "C" fn mk_write_float_raw(
    mut c: *mut mk_context,
    mut f: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let mut u: C2RustUnnamed = C2RustUnnamed { f: 0. };
    let mut c_f: [uint8_t; 4] = [0; 4];
    u.f = f;
    c_f[0 as ::core::ffi::c_int as usize] = (u.u >> 24 as ::core::ffi::c_int) as uint8_t;
    c_f[1 as ::core::ffi::c_int as usize] = (u.u >> 16 as ::core::ffi::c_int) as uint8_t;
    c_f[2 as ::core::ffi::c_int as usize] = (u.u >> 8 as ::core::ffi::c_int) as uint8_t;
    c_f[3 as ::core::ffi::c_int as usize] = u.u as uint8_t;
    return mk_append_context_data(
        c,
        c_f.as_mut_ptr() as *const ::core::ffi::c_void,
        4 as ::core::ffi::c_uint,
    );
}
#[c2rust::src_loc = "283:1"]
unsafe extern "C" fn mk_write_float(
    mut c: *mut mk_context,
    mut id: ::core::ffi::c_uint,
    mut f: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    if mk_write_id(c, id) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_size(c, 4 as ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_float_raw(c, f) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "291:1"]
pub unsafe extern "C" fn mk_create_writer(
    mut filename: *const ::core::ffi::c_char,
) -> *mut mk_writer {
    let mut w: *mut mk_writer = calloc(
        1 as size_t,
        ::core::mem::size_of::<mk_writer>() as size_t,
    ) as *mut mk_writer;
    if w.is_null() {
        return 0 as *mut mk_writer;
    }
    (*w).root = mk_create_context(w, 0 as *mut mk_context, 0 as ::core::ffi::c_uint);
    if (*w).root.is_null() {
        free(w as *mut ::core::ffi::c_void);
        return 0 as *mut mk_writer;
    }
    if strcmp(filename, b"-\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*w).fp = stdout;
    } else {
        (*w).fp = fopen(filename, b"wb\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut FILE;
    }
    if (*w).fp.is_null() {
        mk_destroy_contexts(w);
        free(w as *mut ::core::ffi::c_void);
        return 0 as *mut mk_writer;
    }
    (*w).timescale = 1000000 as int64_t;
    return w;
}
#[no_mangle]
#[c2rust::src_loc = "320:1"]
pub unsafe extern "C" fn mk_write_header(
    mut w: *mut mk_writer,
    mut writing_app: *const ::core::ffi::c_char,
    mut codec_id: *const ::core::ffi::c_char,
    mut codec_private: *const ::core::ffi::c_void,
    mut codec_private_size: ::core::ffi::c_uint,
    mut default_frame_duration: int64_t,
    mut timescale: int64_t,
    mut width: ::core::ffi::c_uint,
    mut height: ::core::ffi::c_uint,
    mut d_width: ::core::ffi::c_uint,
    mut d_height: ::core::ffi::c_uint,
    mut display_size_units: ::core::ffi::c_int,
    mut stereo_mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: *mut mk_context = 0 as *mut mk_context;
    let mut ti: *mut mk_context = 0 as *mut mk_context;
    let mut v: *mut mk_context = 0 as *mut mk_context;
    if (*w).wrote_header != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*w).timescale = timescale;
    (*w).def_duration = default_frame_duration;
    c = mk_create_context(w, (*w).root, 0x1a45dfa3 as ::core::ffi::c_uint);
    if c.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(c, 0x4286 as ::core::ffi::c_uint, 1 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(c, 0x42f7 as ::core::ffi::c_uint, 1 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(c, 0x42f2 as ::core::ffi::c_uint, 4 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(c, 0x42f3 as ::core::ffi::c_uint, 8 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_string(
        c,
        0x4282 as ::core::ffi::c_uint,
        b"matroska\0" as *const u8 as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(
        c,
        0x4287 as ::core::ffi::c_uint,
        (if stereo_mode >= 0 as ::core::ffi::c_int {
            3 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }) as uint64_t,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(c, 0x4285 as ::core::ffi::c_uint, 2 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_close_context(c, 0 as *mut ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    c = mk_create_context(w, (*w).root, 0x18538067 as ::core::ffi::c_uint);
    if c.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_flush_context_id(c) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_close_context(c, 0 as *mut ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    c = mk_create_context(w, (*w).root, 0x1549a966 as ::core::ffi::c_uint);
    if c.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_string(
        c,
        0x4d80 as ::core::ffi::c_uint,
        b"Haali Matroska Writer b0\0" as *const u8 as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_string(c, 0x5741 as ::core::ffi::c_uint, writing_app)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(c, 0x2ad7b1 as ::core::ffi::c_uint, (*w).timescale as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_float(
        c,
        0x4489 as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_float,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*w).duration_ptr = (*c).d_cur.wrapping_sub(4 as ::core::ffi::c_uint);
    if mk_close_context(c, &mut (*w).duration_ptr) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    c = mk_create_context(w, (*w).root, 0x1654ae6b as ::core::ffi::c_uint);
    if c.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ti = mk_create_context(w, c, 0xae as ::core::ffi::c_uint);
    if ti.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(ti, 0xd7 as ::core::ffi::c_uint, 1 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(ti, 0x73c5 as ::core::ffi::c_uint, 1 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(ti, 0x83 as ::core::ffi::c_uint, 1 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(ti, 0x9c as ::core::ffi::c_uint, 0 as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_string(ti, 0x86 as ::core::ffi::c_uint, codec_id)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if codec_private_size != 0 {
        if mk_write_bin(
            ti,
            0x63a2 as ::core::ffi::c_uint,
            codec_private,
            codec_private_size,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if default_frame_duration != 0 {
        if mk_write_uint(
            ti,
            0x23e383 as ::core::ffi::c_uint,
            default_frame_duration as uint64_t,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    v = mk_create_context(w, ti, 0xe0 as ::core::ffi::c_uint);
    if v.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(v, 0xb0 as ::core::ffi::c_uint, width as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(v, 0xba as ::core::ffi::c_uint, height as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(v, 0x54b2 as ::core::ffi::c_uint, display_size_units as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(v, 0x54b0 as ::core::ffi::c_uint, d_width as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_uint(v, 0x54ba as ::core::ffi::c_uint, d_height as uint64_t)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if stereo_mode >= 0 as ::core::ffi::c_int {
        if mk_write_uint(v, 0x53b8 as ::core::ffi::c_uint, stereo_mode as uint64_t)
            < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if mk_close_context(v, 0 as *mut ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_close_context(ti, 0 as *mut ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_close_context(c, 0 as *mut ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_flush_context_data((*w).root) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    (*w).wrote_header = 1 as int8_t;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "397:1"]
unsafe extern "C" fn mk_close_cluster(mut w: *mut mk_writer) -> ::core::ffi::c_int {
    if (*w).cluster.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if mk_close_context((*w).cluster, 0 as *mut ::core::ffi::c_uint)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*w).cluster = 0 as *mut mk_context;
    if mk_flush_context_data((*w).root) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "407:1"]
unsafe extern "C" fn mk_flush_frame(mut w: *mut mk_writer) -> ::core::ffi::c_int {
    let mut delta: int64_t = 0;
    let mut fsize: ::core::ffi::c_uint = 0;
    let mut c_delta_flags: [uint8_t; 3] = [0; 3];
    if (*w).in_frame == 0 {
        return 0 as ::core::ffi::c_int;
    }
    delta = (*w).frame_tc / (*w).timescale - (*w).cluster_tc_scaled;
    if delta as ::core::ffi::c_longlong > 32767 as ::core::ffi::c_longlong
        || (delta as ::core::ffi::c_longlong) < -(32768 as ::core::ffi::c_longlong)
    {
        if mk_close_cluster(w) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*w).cluster.is_null() {
        (*w).cluster_tc_scaled = (*w).frame_tc / (*w).timescale;
        (*w).cluster = mk_create_context(
            w,
            (*w).root,
            0x1f43b675 as ::core::ffi::c_uint,
        );
        if (*w).cluster.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if mk_write_uint(
            (*w).cluster,
            0xe7 as ::core::ffi::c_uint,
            (*w).cluster_tc_scaled as uint64_t,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        delta = 0 as int64_t;
    }
    fsize = if !(*w).frame.is_null() {
        (*(*w).frame).d_cur
    } else {
        0 as ::core::ffi::c_uint
    };
    if mk_write_id((*w).cluster, 0xa3 as ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_size((*w).cluster, fsize.wrapping_add(4 as ::core::ffi::c_uint))
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if mk_write_size((*w).cluster, 1 as ::core::ffi::c_uint) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    c_delta_flags[0 as ::core::ffi::c_int as usize] = (delta >> 8 as ::core::ffi::c_int)
        as uint8_t;
    c_delta_flags[1 as ::core::ffi::c_int as usize] = delta as uint8_t;
    c_delta_flags[2 as ::core::ffi::c_int as usize] = (((*w).keyframe
        as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
        | (*w).skippable as ::core::ffi::c_int) as uint8_t;
    if mk_append_context_data(
        (*w).cluster,
        c_delta_flags.as_mut_ptr() as *const ::core::ffi::c_void,
        3 as ::core::ffi::c_uint,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*w).frame.is_null() {
        if mk_append_context_data((*w).cluster, (*(*w).frame).data, (*(*w).frame).d_cur)
            < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        (*(*w).frame).d_cur = 0 as ::core::ffi::c_uint;
    }
    (*w).in_frame = 0 as int8_t;
    if (*(*w).cluster).d_cur > CLSIZE as ::core::ffi::c_uint {
        if mk_close_cluster(w) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "456:1"]
pub unsafe extern "C" fn mk_start_frame(mut w: *mut mk_writer) -> ::core::ffi::c_int {
    if mk_flush_frame(w) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    (*w).in_frame = 1 as int8_t;
    (*w).keyframe = 0 as int8_t;
    (*w).skippable = 0 as int8_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "468:1"]
pub unsafe extern "C" fn mk_set_frame_flags(
    mut w: *mut mk_writer,
    mut timestamp: int64_t,
    mut keyframe: ::core::ffi::c_int,
    mut skippable: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*w).in_frame == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*w).frame_tc = timestamp;
    (*w).keyframe = (keyframe != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as int8_t;
    (*w).skippable = (skippable != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        as int8_t;
    if (*w).max_frame_tc < timestamp {
        (*w).max_frame_tc = timestamp;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "483:1"]
pub unsafe extern "C" fn mk_add_frame_data(
    mut w: *mut mk_writer,
    mut data: *const ::core::ffi::c_void,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if (*w).in_frame == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if (*w).frame.is_null() {
        (*w).frame = mk_create_context(
            w,
            0 as *mut mk_context,
            0 as ::core::ffi::c_uint,
        );
        if (*w).frame.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return mk_append_context_data((*w).frame, data, size);
}
#[no_mangle]
#[c2rust::src_loc = "495:1"]
pub unsafe extern "C" fn mk_close(
    mut w: *mut mk_writer,
    mut last_delta: int64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if mk_flush_frame(w) < 0 as ::core::ffi::c_int
        || mk_close_cluster(w) < 0 as ::core::ffi::c_int
    {
        ret = -(1 as ::core::ffi::c_int);
    }
    if (*w).wrote_header as ::core::ffi::c_int != 0 && x264_is_regular_file((*w).fp) != 0
    {
        let mut last_frametime: int64_t = if (*w).def_duration != 0 {
            (*w).def_duration
        } else {
            last_delta
        };
        let mut total_duration: int64_t = (*w).max_frame_tc + last_frametime;
        if fseeko((*w).fp, (*w).duration_ptr as __off64_t, SEEK_SET) != 0
            || mk_write_float_raw(
                (*w).root,
                (total_duration as ::core::ffi::c_double
                    / (*w).timescale as ::core::ffi::c_double) as ::core::ffi::c_float,
            ) < 0 as ::core::ffi::c_int
            || mk_flush_context_data((*w).root) < 0 as ::core::ffi::c_int
        {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    mk_destroy_contexts(w);
    fclose((*w).fp);
    free(w as *mut ::core::ffi::c_void);
    return ret;
}
