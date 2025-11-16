#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:28"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "118:1"]
        pub fn strtod(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "215:1"]
        pub fn strtol(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
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
        #[c2rust::src_loc = "159:1"]
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "293:1"]
        pub fn strcspn(
            __s: *const ::core::ffi::c_char,
            __reject: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:28"]
pub mod x264_h {
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/strings.h:28"]
pub mod strings_h {
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn strcasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:28"]
pub mod assert_h {
    #[c2rust::src_loc = "137:12"]
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 61] = unsafe {
        ::core::mem::transmute::<[u8; 61], [::core::ffi::c_char; 61]>(
            *b"char **x264_split_options(const char *, const char *const *)\0",
        )
    };
    extern "C" {
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:28"]
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
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:28"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
pub use self::assert_h::{__assert_fail, __ASSERT_FUNCTION};
use self::stdlib_h::{calloc, strtod, strtol};
use self::string_h::{memcpy, strcmp, strcspn, strlen, strncmp};
use self::strings_h::strcasecmp;
pub use self::x264_h::X264_LOG_ERROR;
use self::x264cli_h::x264_cli_log;
#[no_mangle]
#[c2rust::src_loc = "32:1"]
pub unsafe extern "C" fn x264_split_options(
    mut opt_str: *const ::core::ffi::c_char,
    mut options: *const *const ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut opt_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut options_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut found_named: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: size_t = 0 as size_t;
    let mut opt: *const ::core::ffi::c_char = opt_str;
    if opt_str.is_null() {
        return 0 as *mut *mut ::core::ffi::c_char;
    }
    while !(*options.offset(options_count as isize)).is_null() {
        options_count += 1;
    }
    loop {
        let mut length: size_t =
            strcspn(opt, b"=,\0" as *const u8 as *const ::core::ffi::c_char) as size_t;
        if *opt.offset(length as isize) as ::core::ffi::c_int == '=' as i32 {
            let mut option: *const *const ::core::ffi::c_char = options;
            while !(*option).is_null()
                && (strlen(*option) != length || strncmp(opt, *option, length) != 0)
            {
                option = option.offset(1);
            }
            if (*option).is_null() {
                x264_cli_log(
                    b"options\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"Invalid option '%.*s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                    length,
                    opt,
                );
                return 0 as *mut *mut ::core::ffi::c_char;
            }
            found_named = 1 as ::core::ffi::c_int;
            length = (length as ::core::ffi::c_ulong).wrapping_add(strcspn(
                opt.offset(length as isize),
                b",\0" as *const u8 as *const ::core::ffi::c_char,
            )) as size_t as size_t;
        } else {
            if opt_count >= options_count {
                x264_cli_log(
                    b"options\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"Too many options given\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as *mut *mut ::core::ffi::c_char;
            }
            if found_named != 0 {
                x264_cli_log(
                    b"options\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"Ordered option given after named\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return 0 as *mut *mut ::core::ffi::c_char;
            }
            size = size.wrapping_add(
                strlen(*options.offset(opt_count as isize)).wrapping_add(1 as size_t),
            );
        }
        opt_count += 1;
        opt = opt.offset(length as isize);
        let fresh0 = opt;
        opt = opt.offset(1);
        if !(*fresh0 != 0) {
            break;
        }
    }
    let mut offset: size_t = ((2 as ::core::ffi::c_int * (opt_count + 1 as ::core::ffi::c_int))
        as size_t)
        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t);
    size = size.wrapping_add(
        offset.wrapping_add(opt.offset_from(opt_str) as ::core::ffi::c_long as size_t),
    );
    let mut opts: *mut *mut ::core::ffi::c_char =
        calloc(1 as size_t, size) as *mut *mut ::core::ffi::c_char;
    if opts.is_null() {
        x264_cli_log(
            b"options\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as *mut *mut ::core::ffi::c_char;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int * opt_count {
        let mut length_0: size_t =
            strcspn(opt_str, b"=,\0" as *const u8 as *const ::core::ffi::c_char) as size_t;
        if *opt_str.offset(length_0 as isize) as ::core::ffi::c_int == '=' as i32 {
            let fresh1 = i;
            i = i + 1;
            let ref mut fresh2 = *opts.offset(fresh1 as isize);
            *fresh2 = memcpy(
                (opts as *mut ::core::ffi::c_char).offset(offset as isize)
                    as *mut ::core::ffi::c_void,
                opt_str as *const ::core::ffi::c_void,
                length_0,
            ) as *mut ::core::ffi::c_char;
            offset = offset.wrapping_add(length_0.wrapping_add(1 as size_t));
            opt_str = opt_str.offset(length_0.wrapping_add(1 as size_t) as isize);
            length_0 =
                strcspn(opt_str, b",\0" as *const u8 as *const ::core::ffi::c_char) as size_t;
        } else {
            let mut option_0: *const ::core::ffi::c_char =
                *options.offset((i / 2 as ::core::ffi::c_int) as isize);
            let mut option_length: size_t = strlen(option_0);
            let fresh3 = i;
            i = i + 1;
            let ref mut fresh4 = *opts.offset(fresh3 as isize);
            *fresh4 = memcpy(
                (opts as *mut ::core::ffi::c_char).offset(offset as isize)
                    as *mut ::core::ffi::c_void,
                option_0 as *const ::core::ffi::c_void,
                option_length,
            ) as *mut ::core::ffi::c_char;
            offset = offset.wrapping_add(option_length.wrapping_add(1 as size_t));
            option_0 = option_0.offset(option_length.wrapping_add(1 as size_t) as isize);
        }
        let fresh5 = i;
        i = i + 1;
        let ref mut fresh6 = *opts.offset(fresh5 as isize);
        *fresh6 = memcpy(
            (opts as *mut ::core::ffi::c_char).offset(offset as isize) as *mut ::core::ffi::c_void,
            opt_str as *const ::core::ffi::c_void,
            length_0,
        ) as *mut ::core::ffi::c_char;
        offset = offset.wrapping_add(length_0.wrapping_add(1 as size_t));
        opt_str = opt_str.offset(length_0.wrapping_add(1 as size_t) as isize);
    }
    if offset == size {
    } else {
        __assert_fail(
            b"offset == size\0" as *const u8 as *const ::core::ffi::c_char,
            b"filters/filters.c\0" as *const u8 as *const ::core::ffi::c_char,
            96 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    }
    'c_11546: {
        if offset == size {
        } else {
            __assert_fail(
                b"offset == size\0" as *const u8 as *const ::core::ffi::c_char,
                b"filters/filters.c\0" as *const u8 as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    return opts;
}
#[no_mangle]
#[c2rust::src_loc = "100:1"]
pub unsafe extern "C" fn x264_get_option(
    mut name: *const ::core::ffi::c_char,
    mut split_options: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !split_options.is_null() {
        let mut last_i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*split_options.offset(i as isize)).is_null() {
            if strcmp(*split_options.offset(i as isize), name) == 0 {
                last_i = i;
            }
            i += 2 as ::core::ffi::c_int;
        }
        if last_i >= 0 as ::core::ffi::c_int
            && *(*split_options.offset((last_i + 1 as ::core::ffi::c_int) as isize))
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0
        {
            return *split_options.offset((last_i + 1 as ::core::ffi::c_int) as isize);
        }
    }
    return 0 as *mut ::core::ffi::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn x264_otob(
    mut str: *const ::core::ffi::c_char,
    mut def: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !str.is_null() {
        return (strcasecmp(str, b"true\0" as *const u8 as *const ::core::ffi::c_char) == 0
            || strcmp(str, b"1\0" as *const u8 as *const ::core::ffi::c_char) == 0
            || strcasecmp(str, b"yes\0" as *const u8 as *const ::core::ffi::c_char) == 0)
            as ::core::ffi::c_int;
    }
    return def;
}
#[no_mangle]
#[c2rust::src_loc = "121:1"]
pub unsafe extern "C" fn x264_otof(
    mut str: *const ::core::ffi::c_char,
    mut def: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut ret: ::core::ffi::c_double = def;
    if !str.is_null() {
        let mut end: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
        ret = strtod(str, &mut end);
        if end == str as *mut ::core::ffi::c_char || *end as ::core::ffi::c_int != '\0' as i32 {
            ret = def;
        }
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "134:1"]
pub unsafe extern "C" fn x264_otoi(
    mut str: *const ::core::ffi::c_char,
    mut def: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = def;
    if !str.is_null() {
        let mut end: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
        ret = strtol(str, &mut end, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if end == str as *mut ::core::ffi::c_char || *end as ::core::ffi::c_int != '\0' as i32 {
            ret = def;
        }
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn x264_otos(
    mut str: *mut ::core::ffi::c_char,
    mut def: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return if !str.is_null() { str } else { def };
}
