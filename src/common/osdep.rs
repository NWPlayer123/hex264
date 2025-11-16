#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = i64;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "169:1"]
    pub type __clockid_t = ::core::ffi::c_int;
    #[c2rust::src_loc = "197:1"]
    pub type __syscall_slong_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:28"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::__int64_t;
}
#[c2rust::header_src = "/usr/include/bits/types/clockid_t.h:28"]
pub mod clockid_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type clockid_t = __clockid_t;
    use super::types_h::__clockid_t;
}
#[c2rust::header_src = "/usr/include/time.h:28"]
pub mod time_h {
    use super::clockid_t_h::clockid_t;
    use super::struct_timespec_h::timespec;
    extern "C" {
        #[c2rust::src_loc = "288:1"]
        pub fn clock_gettime(
            __clock_id: clockid_t,
            __tp: *mut timespec,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/time.h:28"]
pub mod bits_time_h {
    #[c2rust::src_loc = "48:10"]
    pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
}
pub use self::types_h::{__int64_t, __time_t, __clockid_t, __syscall_slong_t};
pub use self::struct_timespec_h::timespec;
pub use self::stdint_intn_h::int64_t;
pub use self::clockid_t_h::clockid_t;
use self::time_h::clock_gettime;
pub use self::bits_time_h::CLOCK_MONOTONIC;
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn x264_mdate() -> int64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    return ts.tv_sec as int64_t * 1000000 as int64_t
        + ts.tv_nsec as int64_t / 1000 as int64_t;
}
