use crate::bits_time_h::CLOCK_MONOTONIC;
use crate::stdint_intn_h::int64_t;
use crate::struct_timespec_h::timespec;
use crate::time_h::clock_gettime;
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn x264_mdate() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    return ts.tv_sec as int64_t * 1000000 as int64_t + ts.tv_nsec as int64_t / 1000 as int64_t;
}
