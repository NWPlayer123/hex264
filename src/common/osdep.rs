#[no_mangle]
pub unsafe extern "C" fn x264_mdate() -> crate::stdlib::int64_t {
    unsafe {
        let mut ts = crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        crate::stdlib::clock_gettime(crate::stdlib::CLOCK_MONOTONIC, &raw mut ts);
        return ts.tv_sec * 1000000i64 + ts.tv_nsec / 1000i64;
    }
}
